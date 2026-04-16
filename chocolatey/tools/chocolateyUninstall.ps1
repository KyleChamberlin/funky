$ErrorActionPreference = 'Stop'

$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

Remove-Item -Path (Join-Path $toolsDir "funky.exe") -Force -ErrorAction SilentlyContinue
Get-ChildItem -Directory -Path $toolsDir | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
