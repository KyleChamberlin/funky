# Chocolatey package updater for funky
#
# Called by CI after a GitHub Release is published.
# Downloads the Windows artifact, computes checksums, and injects
# version/URL/checksum into the nuspec and install script.
#
# Required environment variables:
#   VERSION     - Release version without 'v' prefix (e.g. "2026.4.0")
#   GITHUB_TAG  - Full tag name (e.g. "v2026.4.0")

$ErrorActionPreference = 'Stop'

$version = $env:VERSION
$tag = $env:GITHUB_TAG

if (-not $version) {
    Write-Error "VERSION environment variable is required"
    exit 1
}
if (-not $tag) {
    Write-Error "GITHUB_TAG environment variable is required"
    exit 1
}

$repoBase = "https://github.com/KyleChamberlin/funky/releases/download/$tag"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

$url64 = "$repoBase/funky-x86_64-pc-windows-msvc.zip"
$artifact64 = Join-Path $scriptDir "funky-x86_64-pc-windows-msvc.zip"

$maxRetries = 5
$retryDelay = 10
for ($i = 1; $i -le $maxRetries; $i++) {
    try {
        Write-Host "Downloading $url64 (attempt $i/$maxRetries)"
        Invoke-WebRequest -Uri $url64 -OutFile $artifact64
        if ((Get-Item $artifact64).Length -eq 0) {
            throw "Downloaded file is empty"
        }
        break
    } catch {
        if ($i -eq $maxRetries) {
            Write-Error "Failed to download artifact after $maxRetries attempts: $_"
            exit 1
        }
        Write-Host "Download failed: $_. Retrying in ${retryDelay}s..."
        Start-Sleep -Seconds $retryDelay
        $retryDelay *= 2
    }
}

$checksum64 = (Get-FileHash -Algorithm SHA256 -Path $artifact64).Hash
Write-Host "SHA256 (x64): $checksum64"

# Regex-based replacements: match XML tags and PS variable assignments
# so re-runs work even after placeholders are already substituted.
$nuspecPath = Join-Path $scriptDir "funky.nuspec"
$nuspecContent = Get-Content -Path $nuspecPath -Raw
$nuspecContent = $nuspecContent -replace '<version>[^<]+</version>', "<version>$version</version>"
$nuspecContent = $nuspecContent -replace '(releases/tag/v)[^<]+', "`${1}$version"
Set-Content -Path $nuspecPath -Value $nuspecContent -NoNewline

$installPath = Join-Path $scriptDir "tools" "chocolateyInstall.ps1"
$installContent = Get-Content -Path $installPath -Raw
$installContent = $installContent -replace "(Url64bit\s*=\s*)'[^']*'", "`${1}'$url64'"
$installContent = $installContent -replace "(Checksum64\s*=\s*)'[^']*'", "`${1}'$checksum64'"
Set-Content -Path $installPath -Value $installContent -NoNewline

Remove-Item -Path $artifact64 -Force

Write-Host "Package updated for version $version"
Write-Host "  URL (x64):      $url64"
Write-Host "  Checksum (x64): $checksum64"
