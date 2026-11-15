# Service entry shim (installed as boot.ps1): finds a runtime for the boot program (boot.mjs, then boot.prev.mjs) and keeps trying; everything the boot program prints is also appended to run.log beside it (cut to its last 20000 lines above 8 MiB), because a scheduled task keeps no output; it ends only when the boot program ends with 0 or 75, after 20 s of running, or when the task is stopped.
param([Parameter(ValueFromRemainingArguments = $true)][string[]]$Rest)
$ErrorActionPreference = 'Continue'
$dir = Split-Path -Parent $MyInvocation.MyCommand.Path
$env:SEMIO_FLEET_ENTRY = 'boot'
$log = Join-Path $dir 'run.log'
$script:lines = 0
function Limit-Log { if ((Test-Path -LiteralPath $log) -and ((Get-Item -LiteralPath $log).Length -gt 8MB)) { $tail = Get-Content -LiteralPath $log -Tail 20000; Set-Content -LiteralPath $log -Value $tail } }
function Write-Log([string]$Text) { Add-Content -LiteralPath $log -Value $Text -ErrorAction SilentlyContinue; $script:lines++; if ($script:lines % 500 -eq 0) { Limit-Log } }
Limit-Log
function Write-Note([string]$Text) { $line = "$((Get-Date).ToUniversalTime().ToString('s'))Z $Text"; Write-Output $line; Write-Log $line; Set-Content -Path (Join-Path $dir 'shim.txt') -Value $line }
function Get-CachedBun {
    $roots = @((Join-Path $env:LOCALAPPDATA 'semio\runtimes\bun'), (Join-Path $env:USERPROFILE '.bun\bin'))
    foreach ($root in $roots) {
        $found = Get-ChildItem -Path $root -Filter 'bun.exe' -Recurse -ErrorAction SilentlyContinue | Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if ($found) { return $found.FullName }
    }
    return $null
}
function Invoke-One([string]$Runtime, [string]$Program) {
    $started = Get-Date
    & $Runtime $Program @Rest 2>&1 | ForEach-Object { $text = "$_"; Write-Output $text; Write-Log $text }
    $code = $LASTEXITCODE
    $elapsed = [int]((Get-Date) - $started).TotalSeconds
    if ($code -eq 0 -or $code -eq 75 -or $elapsed -ge 20) { exit $code }
    Write-Note "$Runtime $Program ended with $code after $elapsed s / $Runtime $Program endete mit $code nach $elapsed s"
}
$delay = 2
while ($true) {
    $worktree = (Get-Content -Path (Join-Path $dir 'worktree') -ErrorAction SilentlyContinue | Select-Object -First 1)
    $inside = $worktree -and (Test-Path $worktree) -and $(try { Set-Location $worktree; $true } catch { $false })
    $ran = $false
    foreach ($program in @((Join-Path $dir 'boot.mjs'), (Join-Path $dir 'boot.prev.mjs'))) {
        if (-not (Test-Path $program)) { continue }
        if ($inside -and (Get-Command repo -ErrorAction SilentlyContinue)) { $ran = $true; Invoke-One 'repo' $program }
        $bun = Get-CachedBun
        if ($bun) { $ran = $true; Invoke-One $bun $program }
        if (Get-Command bun -ErrorAction SilentlyContinue) { $ran = $true; Invoke-One 'bun' $program }
    }
    if (-not $ran) { Write-Note 'no boot program or no runtime (repo, cached bun, bun) found / kein Boot-Programm oder keine Laufzeit (repo, bun im Cache, bun) gefunden' }
    Start-Sleep -Seconds $delay
    if ($delay -lt 60) { $delay = $delay * 2 }
}
