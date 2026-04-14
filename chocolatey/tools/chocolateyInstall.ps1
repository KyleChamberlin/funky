$ErrorActionPreference = 'Stop'

$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
    PackageName    = $env:ChocolateyPackageName
    FileType       = 'zip'
    Url64bit       = 'REPLACE_URL_64'
    Checksum64     = 'REPLACE_CHECKSUM_64'
    ChecksumType64 = 'sha256'
    UnzipLocation  = $toolsDir
}

Install-ChocolateyZipPackage @packageArgs

# cargo-dist archives extract into a subdirectory (e.g. funky-x86_64-pc-windows-msvc/).
# Chocolatey only auto-shims executables directly under tools/, so move it up.
$exe = Get-ChildItem -Path $toolsDir -Recurse -Filter "funky.exe" | Select-Object -First 1
if ($exe) {
    Move-Item -Path $exe.FullName -Destination $toolsDir -Force
}
Get-ChildItem -Directory -Path $toolsDir | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
