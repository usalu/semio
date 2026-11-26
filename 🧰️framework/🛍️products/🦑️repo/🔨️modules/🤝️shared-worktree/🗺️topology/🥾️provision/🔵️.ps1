#region 🧲️Header
#
# Specs: Zero-touch Windows worker provisioning of the shared worktree fleet, from an empty disk. Idempotent: every step probes before it changes anything and prints what it does. Operators run it once per fleet machine from an elevated PowerShell; it changes machine settings (optional features, registry, machine environment, scheduled tasks), so the build never runs it.
#
# Summary: `powershell -NoProfile -ExecutionPolicy Bypass -File "<this file>" <GroupId> <OriginId> [-ArchivePath <directory>] [-FirstMachine | -Server] [-User <DOMAIN\account>] [-Plan] [-Force] [-RelocateFrom <worktree>]`
# The script needs the topology file next to it (..\🔣️.json): the early steps run before the repo launcher exists and read the worktree path and the NFS export from it; the topology command re-derives the same values later and the script refuses any difference. -User must be the account that runs this elevated shell, because the repository's native bootstrap installs the tools per account.
# Steps: LongPathsEnabled, canonical worktree directory, the attribute and name cache of the store client (SMB: Set-SmbClientConfiguration, NFS: registry values of Client for NFS), Client for NFS and the store mount, the Windows Time service, bootstrap archive (sha256 verified against manifest.json, extracted with the system tar.exe, which is bsdtar and reads the pax UTF-8 names), the standalone `repo` launcher (built from the checkout by the launcher installer, needs neither winget nor a compiler), the remount task, the cache environment (Machine scope), the replica and daily compact scheduled tasks, `bootstrap-settle`, `join`, task start, and only then, as the last step, the repository's native bootstrap (winget packages, the C++ Build Tools, Node.js, Rust, .NET, uv, `repo-mcp:install`, `workspace:setup`).
# Joining the fleet never depends on the native build: a fix for a broken build arrives through the shared worktree itself, so the machine must already be syncing when the native bootstrap runs. The native bootstrap runs last with its output in .cache\f\native-bootstrap.log (ignored local state); the completion marker .cache\f\native-bootstrap.done is written only after it succeeded. When it fails (or winget is missing), the script prints in English the failing command and the log, leaves the marker absent so that the next run retries it, and exits with code 3; the fleet setup stays as it is. `preflight` shows the same state as the warning `native-build`.
# Exit codes: 0 fleet ready and native build complete; 1 error (the script threw, nothing after the failing step ran); 3 fleet ready, native build pending (the machine syncs; run the script again after the cause is fixed or the fix has arrived). 5 the fleet code of the checkout is older than this script and the store offers no newer sealed code (a fleet command failed on an unknown option or a missing module; the message names the checkout; nothing after that step ran).
# Fleet code is never older than this script: before its first Invoke-Fleet or Invoke-Replica call on a checkout that has not joined yet (an old checkout to adopt, a tree just extracted from the bootstrap archive, a relocation target) the script reads the digest that the group server attests from the store, verifies the sealed code snapshot of that digest (v1\bootstrap\code\<digest>\sealed.json: size and sha256 of every file) and copies each sealed file into the worktree; every replaced file that differs is kept under .cache\f\lost\provision-code-<stamp>\<path>, nothing is deleted, and the counts replaced / added / identical are printed. -Plan does the same (the plan cannot run on older code). A joined machine, a server and the first machine are skipped: their replica keeps the code current. The kit this script comes from is published by the group server from the same sealed files (v1\bootstrap\kit, receipt published).
# A worktree that already holds the account's own repository is adopted into the fleet, not refused (`join --adopt`: identical files are adopted, missing ones come from the store, a differing one is decided by its content against the fleet's history and the repository's base (an old version loses, local work is published, a change on both sides is a conflict that keeps both versions in .cache\f\lost and in the report) and never by a modification time; ignored files and nested repositories stay). -Plan mounts the store, prints the dry run (`join --adopt --dry-run`, or `relocate --dry-run`) and stops before it registers tasks or changes the worktree, and on an empty worktree before the archive is copied, so it extracts nothing; a tree that this script extracted itself is marked in .cache\f\bootstrap-extract (`started` while tar.exe runs, `complete` after it, removed after `join`) and, until the machine has joined, is settled by a later run and never adopted, or extracted again when the extraction was interrupted; -Force adopts a repository without a common commit; -RelocateFrom (or the working directory of the installed replica task when it differs) moves an already joined machine to C:\semio: the tasks stop, `relocate` takes the identity (the origin id never changes) and adopts the store in the new tree, the tasks start there, `relocate --phase finish` verifies and retires the old tree, which is never deleted.
# -ArchivePath is a directory with manifest.json and the archive it names (worktree-<16 hex>.tar) (default: <drive>\v1\bootstrap of the group store). A worktree that already holds a checkout skips the archive; one that is neither empty nor a checkout is refused. -FirstMachine is for the first machine of the fleet, which has no archive: a checkout is at the canonical path and the store is empty, and `seed` publishes it instead of `join` (refused when the store already has origins; never use it twice).
# Re-running is safe: when .cache\f\machine.json exists, its origin and group must equal the arguments (otherwise the script stops before it changes anything); `join` is then skipped, while the mounts, tasks and services are installed again.
# -Server marks the Windows group server (the topology names the platform and the protocol in its one file): no store mount; the script shares the store folder (Server for NFS `New-NfsShare` on Windows Server, or an SMB share when the topology says `smb`: Windows Server for NFS does not exist on Windows 10/11), opens the firewall for the group network and the peer servers, registers the remote cache and relay tasks next to the replica task, and its daily compaction passes --collect-blobs. It needs -ArchivePath (a peer group's bootstrap directory, a UNC path) when the worktree is empty, and the cache tokens file (never written here).
# Cache server tokens are not written here: the runbook says how to give the account SEMIO_REMOTE_CACHE_TOKEN.
# Saved as UTF-8 with BOM: Windows PowerShell 5.1 decodes a BOM-less script with the ANSI code page and would turn every emoji path literal below into a different path.
#
#endregion 🧲️Header

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidatePattern('^[a-z][a-z0-9]{0,15}$')]
    [string]$GroupId,
    [Parameter(Mandatory = $true, Position = 1)]
    [ValidatePattern('^[a-z0-9][a-z0-9-]{0,39}$')]
    [string]$OriginId,
    [string]$ArchivePath = '',
    [switch]$FirstMachine,
    [switch]$Server,
    [string]$User = [Security.Principal.WindowsIdentity]::GetCurrent().Name,
    [switch]$Plan,
    [switch]$Force,
    [string]$RelocateFrom = ''
)

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$OutputEncoding = [Console]::OutputEncoding

$Modules = '🧰️framework\🛍️products\🦑️repo\🔨️modules'
$NativeBootstrap = "$Modules\🔩️native\🥾️bootstrap\🔵️.ps1"
$LauncherInstall = "$Modules\🚀️launcher\📥️install\🔵️.ps1"
$LauncherBin = "$env:LOCALAPPDATA\semio\launcher\bin"
$ExitNativePending = 3
$ExitFleetCodeOld = 5
$TopologyRelative = "$Modules\🤝️shared-worktree\🗺️topology"

function Say([string]$Message) { Write-Host "[fleet-provision] $Message" }

if (-not [Environment]::Is64BitProcess) { throw 'Run this script from a 64-bit PowerShell: the 32-bit one sees System32 as SysWOW64, which has no tar.exe.' }
$principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
$CurrentUser = [Security.Principal.WindowsIdentity]::GetCurrent().Name
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'Run this script from an elevated PowerShell.' }
if ($User -ine $CurrentUser) { throw "-User must be the account of this elevated shell ($CurrentUser): the repository bootstrap installs the tools for it." }

$topologyFiles = @(Get-ChildItem -LiteralPath (Split-Path -Parent $PSScriptRoot) -Filter '*.json' -File)
if ($topologyFiles.Count -ne 1) { throw "The topology file was not found next to this script: $(Split-Path -Parent $PSScriptRoot)" }
$topology = Get-Content -Raw -Encoding UTF8 -LiteralPath $topologyFiles[0].FullName | ConvertFrom-Json
$Worktree = $topology.worktree.windows
$Drive = $topology.storeMount.windows.Substring(0, 2)
$group = @($topology.groups | Where-Object { $_.id -eq $GroupId })
if ($group.Count -ne 1 -or -not $Worktree) { throw "The topology has no worktree for windows or no group ${GroupId}." }
$StoreOrigin = $group[0].roles.store
function Find-StoreAddress([string]$Origin) {
    # Bonjour (dns-sd.exe): the instance of the service _semio-fleet._tcp is the origin; nothing is written to the machine.
    $tool = Get-Command dns-sd.exe -ErrorAction SilentlyContinue
    if (-not $tool) { return $null }
    $run = {
        param([string[]]$Arguments)
        $out = Join-Path ([IO.Path]::GetTempPath()) ("semio-dnssd-" + [guid]::NewGuid().ToString('N') + '.txt')
        $process = Start-Process -FilePath $tool.Source -ArgumentList $Arguments -NoNewWindow -PassThru -RedirectStandardOutput $out
        if (-not $process.WaitForExit(2500)) { try { $process.Kill() } catch {} }
        $text = if (Test-Path -LiteralPath $out) { Get-Content -Raw -LiteralPath $out } else { '' }
        Remove-Item -LiteralPath $out -Force -ErrorAction SilentlyContinue
        $text
    }
    $lookup = & $run @('-L', $Origin, '_semio-fleet._tcp', 'local')
    if ($lookup -notmatch 'can be reached at (\S+?)\.?:\d+') { return $null }
    $answer = & $run @('-G', 'v4', $Matches[1])
    foreach ($line in ($answer -split "`n")) { $cells = @($line.Trim() -split '\s+'); if ($cells.Count -ge 6 -and $cells[1] -eq 'Add' -and $cells[5] -notlike '169.254.*') { return $cells[5] } }
    return $null
}
$NfsHost = if ($env:SEMIO_FLEET_GROUP -eq $GroupId -and $env:SEMIO_FLEET_STORE_ADDRESS) { $env:SEMIO_FLEET_STORE_ADDRESS } else { Find-StoreAddress $StoreOrigin }
if (-not $NfsHost) { throw "The address of the store ($StoreOrigin) could not be found: no Bonjour answer. Run the script again with `$env:SEMIO_FLEET_GROUP='$GroupId'; `$env:SEMIO_FLEET_STORE_ADDRESS='<address of $StoreOrigin>' once (it is never stored) / Die Adresse des Speichers ($StoreOrigin) wurde nicht gefunden." }
$NfsSource = "\\$NfsHost$($group[0].nfs.export.Replace('/', '\'))"
$ServerPlatform = $group[0].server.platform
$ServerProtocol = $group[0].server.protocol
$Smb = $ServerProtocol -eq 'smb'
$MountOptions = if ($Smb) { '' } else { 'anon,nolock,mtype=hard' }
if ($Server -and $ServerPlatform -ne 'windows') { throw "The topology puts the server of ${GroupId} on ${ServerPlatform}, this machine is windows: provision it without -Server, or change the one topology file." }
if ($Worktree.Length -gt 15) { throw "The worktree root $Worktree is not short: deep emoji paths need it at C:\semio length." }
$StoreRoot = if ($Server) { $group[0].server.directory } else { "$Drive\" }
$SmbCache = [ordered]@{ DirectoryCacheLifetime = 0; FileInfoCacheLifetime = 0; FileNotFoundCacheLifetime = 0 }
$NfsCacheKey = 'HKLM:\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache'
$NfsCache = [ordered]@{ RemoteWriteCache = 0; FileAttributeCache = 0 }
$peers = @($topology.groups | Where-Object { $_.id -ne $GroupId })
$NeedSmb = if ($Server) { @($peers | Where-Object { $_.server.protocol -eq 'smb' }).Count -gt 0 } else { $Smb }
$NeedNfs = if ($Server) { @($peers | Where-Object { $_.server.protocol -eq 'nfs' }).Count -gt 0 } else { -not $Smb }
$BootstrapMarker = Join-Path $Worktree '.cache\f\native-bootstrap.done'
$BootstrapLog = Join-Path $Worktree '.cache\f\native-bootstrap.log'
$machine = Join-Path $Worktree '.cache\f\machine.json'
$joined = $false
$relocate = $false
if (Test-Path -LiteralPath $machine) {
    $identity = Get-Content -Raw -Encoding UTF8 -LiteralPath $machine | ConvertFrom-Json
    if (-not $identity.originId -or -not $identity.groupId) { throw "$machine has no originId and groupId: remove the worktree state first." }
    if ($identity.originId -ne $OriginId) { throw "$machine belongs to origin $($identity.originId), not ${OriginId}: pass that origin, or remove the worktree state first." }
    if ($identity.groupId -ne $GroupId) { throw "$machine belongs to group $($identity.groupId), not ${GroupId}: pass that group, or remove the worktree state first." }
    $joined = $true
}

function Sync-ProcessPath {
    $env:Path = (@([Environment]::GetEnvironmentVariable('Path', 'Machine'), [Environment]::GetEnvironmentVariable('Path', 'User')) | Where-Object { $_ }) -join ';'
}

function Test-BootstrapNeeded { return (-not (Test-Path -LiteralPath $BootstrapMarker)) -or (-not (Get-Command repo -ErrorAction SilentlyContinue)) }

function Get-InstalledWorktree {
    $installed = try { Get-ScheduledTask -TaskName 'semio-fleet-replica' -ErrorAction Stop } catch { $null }
    if (-not $installed) { return '' }
    $directory = [string]$installed.Actions[0].WorkingDirectory
    $suffix = "\$Modules\🤝️shared-worktree\🔁️replica"
    if ($directory.EndsWith($suffix)) { return $directory.Substring(0, $directory.Length - $suffix.Length) }
    return ''
}
if (-not $RelocateFrom -and -not $joined) { $RelocateFrom = Get-InstalledWorktree }
if ($RelocateFrom -and $RelocateFrom -ine $Worktree -and -not $joined -and (Test-Path -LiteralPath (Join-Path $RelocateFrom '.cache\f\machine.json'))) {
    $oldMachine = Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-Path $RelocateFrom '.cache\f\machine.json') | ConvertFrom-Json
    if ($oldMachine.originId -ne $OriginId) { throw "$RelocateFrom belongs to origin $($oldMachine.originId), not ${OriginId}: origin ids never change and are never reused, pass that origin." }
    if ($oldMachine.groupId -ne $GroupId) { throw "$RelocateFrom belongs to group $($oldMachine.groupId), not ${GroupId}: pass that group." }
    $relocate = $true
}

function ConvertTo-List($Table) { return (@($Table.Keys | ForEach-Object { "$_=$($Table[$_])" })) -join ',' }

function Set-ClientCache {
    if ($NeedSmb) {
        $current = Get-SmbClientConfiguration
        if (@($SmbCache.Keys | Where-Object { $current.$_ -ne $SmbCache[$_] }).Count -eq 0) { Say 'unchanged SMB client cache lifetimes' } else {
            Set-SmbClientConfiguration @SmbCache -Force
            Say 'set the SMB client cache lifetimes (directory listing, file information, file not found) to 0'
        }
    }
    if ($NeedNfs) {
        $changed = $false
        if (-not (Test-Path -LiteralPath $NfsCacheKey)) { New-Item -Path $NfsCacheKey -Force | Out-Null; $changed = $true; Say "created $NfsCacheKey" }
        foreach ($name in $NfsCache.Keys) {
            $now = (Get-ItemProperty -Path $NfsCacheKey -Name $name -ErrorAction SilentlyContinue).$name
            if ($now -eq $NfsCache[$name]) { Say "unchanged $name" } else {
                New-ItemProperty -Path $NfsCacheKey -Name $name -Value $NfsCache[$name] -PropertyType DWord -Force | Out-Null
                $changed = $true
                Say "set $name = $($NfsCache[$name])"
            }
        }
        if ($changed) {
            foreach ($action in 'stop', 'start') {
                & nfsadmin client $action | Out-Null
                if ($LASTEXITCODE -ne 0) { throw "nfsadmin client $action failed with exit code $LASTEXITCODE" }
            }
            Say 'restarted the Client for NFS service so that the cache values apply to the mount'
        }
    }
}

function Stop-FleetCodeOld {
    [Console]::Error.WriteLine("[fleet-provision] ERROR: the checkout's fleet code is older than this script and the store offers no newer sealed code: $Worktree / der Fleet-Code des Checkouts ist älter als dieses Skript und der Store bietet keinen neueren versiegelten Code: $Worktree")
    [Console]::Error.WriteLine("[fleet-provision] exit code ${ExitFleetCodeOld}: update the checkout to the current fleet revision, or run this script again once the group server has sealed the current code / Checkout auf die aktuelle Fleet-Revision bringen oder das Skript erneut starten, sobald der Gruppenserver den aktuellen Code versiegelt hat")
    exit $ExitFleetCodeOld
}

function Invoke-Guarded([string]$Script, [string[]]$Arguments) {
    $ErrorActionPreference = 'Continue'
    $errorText = New-Object System.Collections.Generic.List[string]
    $output = & repo $Script @Arguments 2>&1 | ForEach-Object {
        if ($_ -is [System.Management.Automation.ErrorRecord]) { $errorText.Add($_.ToString()); [Console]::Error.WriteLine($_.ToString()) } else { $_ }
    }
    $code = $LASTEXITCODE
    if ($code -ne 0 -and ($errorText -join "`n") -match 'Unknown option|unknown option|Cannot find module|Cannot find package|Module not found|ERR_MODULE_NOT_FOUND|Could not resolve') { Stop-FleetCodeOld }
    if ($code -ne 0) { throw "repo $Script $($Arguments -join ' ') failed with exit code $code" }
    return $output
}

function Invoke-Fleet([string[]]$Arguments) {
    $env:SEMIO_WORKSPACE_ROOT = $Worktree
    $script = Join-Path (Join-Path $Worktree $TopologyRelative) '📜️script.ts'
    Push-Location $Worktree
    try { return Invoke-Guarded $script $Arguments } finally { Pop-Location }
}

function Invoke-Replica([string[]]$Arguments) {
    $env:SEMIO_WORKSPACE_ROOT = $Worktree
    $script = Join-Path $FLEET_REPLICA_DIRECTORY '📜️script.ts'
    Push-Location $Worktree
    try { Invoke-Guarded $script $Arguments } finally { Pop-Location }
}

function Test-OverlayPath([string]$Path) {
    if ($Path -eq '' -or $Path.StartsWith('/') -or $Path.Contains('\')) { return $false }
    foreach ($part in ($Path -split '/')) { if ($part -eq '' -or $part -eq '.' -or $part -eq '..') { return $false } }
    return $true
}

function Set-OverlayBit([string]$Path, [bool]$Executable) {
    $unix = (Get-Variable -Name IsWindows -ValueOnly -ErrorAction SilentlyContinue) -eq $false
    if (-not $unix) { return }
    & chmod $(if ($Executable) { '755' } else { '644' }) -- $Path
    if ($LASTEXITCODE -ne 0) { throw "chmod failed for $Path / chmod fehlgeschlagen für $Path" }
}

function Get-OverlayDigest([string]$V1) {
    $attest = Join-Path $V1 "origins\$GroupId-server\attest.json"
    if (Test-Path -LiteralPath $attest) {
        $attested = try { [string](Get-Content -Raw -Encoding UTF8 -LiteralPath $attest | ConvertFrom-Json).code } catch { '' }
        if ($attested -match '^[0-9a-f]{64}$' -and (Test-Path -LiteralPath "$V1\bootstrap\code\$attested\sealed.json")) { return $attested }
    }
    $best = ''
    $newest = [int64]-1
    foreach ($directory in @(Get-ChildItem -LiteralPath "$V1\bootstrap\code" -Directory -ErrorAction SilentlyContinue)) {
        if ($directory.Name -notmatch '^[0-9a-f]{64}$') { continue }
        $seal = try { Get-Content -Raw -Encoding UTF8 -LiteralPath (Join-Path $directory.FullName 'sealed.json') | ConvertFrom-Json } catch { $null }
        if ($seal -and $seal.digest -eq $directory.Name -and [int64]$seal.createdMs -gt $newest) { $newest = [int64]$seal.createdMs; $best = $directory.Name }
    }
    return $best
}

function Invoke-FleetCodeOverlay {
    if ($joined -or $Server -or $FirstMachine) {
        Say 'fleet code: not overlaid, the replica of this machine keeps it current (joined machine, group server or first machine) / Fleet-Code: nicht überlagert, das Replikat dieser Maschine hält ihn aktuell (beigetretene Maschine, Gruppenserver oder erste Maschine)'
        return
    }
    $planNote = if ($Plan) { ' (also with -Plan: the plan cannot run on older code) / (auch mit -Plan: der Plan läuft nicht auf älterem Code)' } else { '' }
    Say "step 4b: fleet code of the checkout brought to the sealed code of the fleet / Fleet-Code des Checkouts auf den versiegelten Code der Flotte gebracht$planNote"
    $v1 = if ($ArchivePath -and (Test-Path -LiteralPath "$ArchivePath\code")) { Split-Path -Parent $ArchivePath } else { Join-Path $StoreRoot 'v1' }
    $digest = try { Get-OverlayDigest $v1 } catch { '' }
    if (-not $digest) { Say 'WARNING: the store holds no readable sealed code, the fleet code of the checkout stays as it is / Der Store enthält keinen lesbaren versiegelten Code, der Fleet-Code des Checkouts bleibt unverändert'; return }
    $code = "$v1\bootstrap\code\$digest"
    $problem = ''
    $files = @()
    try {
        $seal = Get-Content -Raw -Encoding UTF8 -LiteralPath "$code\sealed.json" | ConvertFrom-Json
        if ($seal.version -ne 1 -or $seal.digest -ne $digest -or @($seal.files).Count -eq 0) { throw 'the seal is not a version 1 seal of this digest / das Siegel ist kein Version-1-Siegel dieses Digests' }
        foreach ($file in @($seal.files)) {
            $path = [string]$file.path
            if (-not (Test-OverlayPath $path)) { throw "unsafe path in the seal: $path / unsicherer Pfad im Siegel: $path" }
            $source = Join-Path $code ('files\' + $path.Replace('/', '\'))
            if (-not (Test-Path -LiteralPath $source)) { throw "missing $path / fehlt $path" }
            if ((Get-Item -LiteralPath $source).Length -ne [int64]$file.bytes -or (Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash.ToLowerInvariant() -ne [string]$file.sha256) { throw "changed $path / geändert $path" }
            if ($file.executable -isnot [bool]) { throw "the seal lacks the execute bit of ${path}: seal it again on the group server / dem Siegel fehlt das Ausführungsbit von ${path}: auf dem Gruppenserver erneut versiegeln" }
            $files += [pscustomobject]@{ Path = $path.Replace('/', '\'); Source = $source; Hash = [string]$file.sha256; Executable = [bool]$file.executable }
        }
    } catch { $problem = $_.Exception.Message }
    if ($problem) { Say "WARNING: the sealed code $digest is not intact ($problem), the fleet code of the checkout stays as it is / Der versiegelte Code $digest ist nicht intakt, der Fleet-Code des Checkouts bleibt unverändert"; return }
    $keep = Join-Path $Worktree ('.cache\f\lost\provision-code-' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
    $replaced = 0; $added = 0; $identical = 0
    foreach ($item in $files) {
        $target = Join-Path $Worktree $item.Path
        if ((Test-Path -LiteralPath $target -PathType Leaf) -and (Get-FileHash -Algorithm SHA256 -LiteralPath $target).Hash.ToLowerInvariant() -eq $item.Hash) { Set-OverlayBit $target $item.Executable; $identical++; continue }
        New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
        if (Test-Path -LiteralPath $target) {
            $kept = Join-Path $keep $item.Path
            New-Item -ItemType Directory -Force -Path (Split-Path -Parent $kept) | Out-Null
            Copy-Item -LiteralPath $target -Destination $kept -Force
            $replaced++
        } else { $added++ }
        $temporary = "$target.provision-code.tmp"
        Copy-Item -LiteralPath $item.Source -Destination $temporary -Force
        Set-OverlayBit $temporary $item.Executable
        Move-Item -LiteralPath $temporary -Destination $target -Force
    }
    Say "fleet code ${digest}: replaced $replaced, added $added, identical $identical / Fleet-Code ${digest}: ersetzt $replaced, hinzugefügt $added, identisch $identical"
    if ($replaced -gt 0) { Say "the replaced files are kept in $keep / die ersetzten Dateien liegen in $keep" }
}

Say "group $GroupId, origin $OriginId, account $User, platform windows"

Sync-ProcessPath
$nativeNeeded = Test-BootstrapNeeded

Say 'step 1: long paths'
$filesystem = 'HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem'
$current = (Get-ItemProperty -Path $filesystem -Name LongPathsEnabled -ErrorAction SilentlyContinue).LongPathsEnabled
if ($current -eq 1) { Say 'unchanged LongPathsEnabled' } else {
    New-ItemProperty -Path $filesystem -Name LongPathsEnabled -Value 1 -PropertyType DWord -Force | Out-Null
    Say 'set LongPathsEnabled = 1 (applies to processes started from now on; the repository paths are deep emoji paths)'
}
if ((Get-ItemProperty -Path $filesystem -Name LongPathsEnabled).LongPathsEnabled -ne 1) { throw 'LongPathsEnabled could not be set to 1.' }

Say "step 2: canonical worktree directory $Worktree"
New-Item -ItemType Directory -Force -Path $Worktree | Out-Null
& icacls $Worktree /grant "${User}:(OI)(CI)M" | Out-Null
if ($LASTEXITCODE -ne 0) { throw "icacls failed for $Worktree" }

Say 'step 3: store client cache and store mount'
Set-ClientCache
if ($Server) {
    New-Item -ItemType Directory -Force -Path $StoreRoot | Out-Null
    & icacls $StoreRoot /grant "${User}:(OI)(CI)M" | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "icacls failed for $StoreRoot" }
    Say "group server: the store is the local folder $StoreRoot (shared in step 8b), no mount"
} elseif ($Smb) {
    if (Test-Path -LiteralPath "$Drive\") { Say "already mounted $Drive" } else {
        & net.exe use $Drive $NfsSource /persistent:yes
        if ($LASTEXITCODE -ne 0) { throw "net use failed with exit code $LASTEXITCODE. The store of ${GroupId} is an SMB share: store the credentials of the fleet account first with: cmdkey /add:$($group[0].nfs.host) /user:<server name>\semio-fleet /pass:<password>" }
        Say "mounted $Drive over SMB"
    }
} else {
    foreach ($feature in 'ServicesForNFS-ClientOnly', 'ClientForNFS-Infrastructure') {
        if ((Get-WindowsOptionalFeature -Online -FeatureName $feature).State -eq 'Enabled') { Say "unchanged feature $feature" } else {
            Enable-WindowsOptionalFeature -Online -FeatureName $feature -All -NoRestart | Out-Null
            Say "enabled feature $feature"
        }
    }
    if (Test-Path -LiteralPath "$Drive\") {
        Say "already mounted $Drive"
        $mountedSource = @(& mount.exe 2>$null | Where-Object { $_ -match "^\s*$([regex]::Escape($Drive.TrimEnd('\')))" }) -join ' '
        if ($mountedSource -and $mountedSource.ToLowerInvariant() -notlike "*$($NfsSource.ToLowerInvariant())*") {
            Say "WARNING: $Drive is mapped from another source ($mountedSource), the topology names $NfsSource (a host name that follows the server to another network): stop the replica service, then run: mount.exe -u $Drive; mount.exe $($MountOptions -split ',' | ForEach-Object { "-o $_" }) $NfsSource $Drive / WARNUNG: $Drive ist von einer anderen Quelle verbunden, die Topologie nennt $NfsSource"
        }
    } else {
        $mountFlags = @($MountOptions -split ',' | ForEach-Object { '-o'; $_ })
        & mount.exe @mountFlags $NfsSource $Drive
        if ($LASTEXITCODE -ne 0) { throw "mount.exe failed with exit code $LASTEXITCODE" }
        Say "mounted $Drive"
    }
}

Say 'step 3b: clock'
$time = Get-Service -Name w32time -ErrorAction SilentlyContinue
if (-not $time) { Say 'WARNING: the Windows Time service (w32time) is missing: a clock more than 60 s off holds back the events of other machines' } else {
    if ($time.StartType -ne 'Automatic') { Set-Service -Name w32time -StartupType Automatic; Say 'set w32time to start automatically' }
    if ($time.Status -ne 'Running') { Start-Service -Name w32time; Say 'started w32time' } else { Say 'w32time is running' }
    Say "time source: $((& w32tm.exe /query /source) -join ' ')"
}

Say 'step 4: bootstrap archive into the empty worktree'
$nativeScript = Join-Path $Worktree $NativeBootstrap
$adopt = $false
$extract = $false
$extractMarker = Join-Path $Worktree '.cache\f\bootstrap-extract'
$extractState = if (-not $joined -and (Test-Path -LiteralPath $extractMarker)) { "$(Get-Content -Raw -LiteralPath $extractMarker)".Trim() } else { '' }
if ($extractState -eq 'complete') {
    Say 'the worktree holds the tree that an earlier run of this script extracted: it is settled, not adopted, the archive is skipped'
} elseif ($extractState) {
    $extract = $true
    Say 'an earlier run stopped while it extracted the bootstrap archive: the archive is extracted again over it'
} elseif (Test-Path -LiteralPath $nativeScript) {
    if (-not $joined -and -not $relocate -and @(Get-ChildItem -LiteralPath $Worktree -Force | Where-Object { $_.Name -ne '.cache' }).Count -gt 0) {
        $adopt = $true
        Say 'the worktree already holds a repository: it is adopted into the fleet (nothing is refused, every difference is kept in .cache\f\lost and reported), the archive is skipped'
    } else { Say 'the worktree already holds a checkout: the archive is skipped' }
} elseif (@(Get-ChildItem -LiteralPath $Worktree -Force | Where-Object { $_.Name -ne '.git' }).Count -gt 0) {
    throw "$Worktree holds files but no fleet module ($NativeBootstrap is missing): update that repository to a revision that contains the fleet module, or move it away. The bootstrap archive is only for a machine without any checkout."
} elseif ($FirstMachine) {
    throw "-FirstMachine needs a complete checkout at ${Worktree}: the first machine has no archive to extract."
} else { $extract = $true }
if ($extract -and $Plan) {
    if ($extractState) { Say 'plan: an earlier run stopped while it extracted the bootstrap archive: nothing was extracted now, a plain provisioning extracts it again, settles the tree and joins' }
    else { Say 'plan: the worktree is empty: nothing was extracted, a plain provisioning extracts the bootstrap archive, settles the tree and joins' }
    exit 0
}
if ($extract) {
    if ($Server -and -not $ArchivePath) { throw 'A group server takes the archive of a peer group: pass -ArchivePath <peer share>\v1\bootstrap.' }
    $source = if ($ArchivePath) { $ArchivePath } else { "$Drive\v1\bootstrap" }
    $tar = Join-Path $env:SystemRoot 'System32\tar.exe'
    if (-not (Test-Path -LiteralPath $tar)) { throw "tar.exe was not found at ${tar}: Windows 10 version 1803 or later is required." }
    $work = Join-Path $env:TEMP ("semio-bootstrap-" + [Guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force -Path $work | Out-Null
    try {
        $local = Join-Path $work 'worktree.tar'
        $verified = $false
        foreach ($attempt in 1..5) {
            if (-not (Test-Path -LiteralPath "$source\manifest.json")) { throw "No bootstrap archive in ${source}: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            $manifest = Get-Content -Raw -Encoding UTF8 -LiteralPath "$source\manifest.json" | ConvertFrom-Json
            if (($manifest.archive -notmatch '^worktree-[0-9a-f]{16}\.tar$') -or -not (Test-Path -LiteralPath "$source\$($manifest.archive)")) { throw "The bootstrap manifest in ${source} names no archive that exists: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            Copy-Item -LiteralPath "$source\$($manifest.archive)" -Destination $local -Force
            $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $local).Hash.ToLowerInvariant()
            if ($actual -eq $manifest.sha256) { $verified = $true; Say "archive verified: sha256 $actual, $($manifest.files) files"; break }
            Say "the archive differs from manifest.json (refreshed while it was copied?), attempt $attempt of 5"
            Start-Sleep -Seconds 20
        }
        if (-not $verified) { throw "The archive in $source never matched its manifest.json." }
        $entries = (& $tar -tf $local | Measure-Object -Line).Lines
        if ($LASTEXITCODE -ne 0 -or $entries -ne $manifest.files) { throw "The archive lists $entries entries, manifest.json says $($manifest.files)." }
        Say "extracting with tar.exe into $Worktree"
        New-Item -ItemType Directory -Force -Path (Split-Path -Parent $extractMarker) | Out-Null
        Set-Content -LiteralPath $extractMarker -Value 'started'
        & $tar -xf $local -C $Worktree
        if ($LASTEXITCODE -ne 0) { throw "tar.exe failed with exit code $LASTEXITCODE" }
        Set-Content -LiteralPath $extractMarker -Value 'complete'
        $nativeNeeded = $true
    } finally { Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue }
}

Invoke-FleetCodeOverlay

Say 'step 5: standalone repo launcher (built from the checkout; the native bootstrap comes last)'
Sync-ProcessPath
if (Get-Command repo -ErrorAction SilentlyContinue) { Say 'repo launcher present' } else {
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File (Join-Path $Worktree $LauncherInstall) -Destination $LauncherBin
    if ($LASTEXITCODE -ne 0) { throw "The repo launcher could not be built (exit code ${LASTEXITCODE}): the machine cannot join without it, fix the cause and run again." }
    $rawPath = @(& reg.exe query 'HKCU\Environment' /v Path 2>$null | Where-Object { $_ -match 'REG_(EXPAND_)?SZ' } | ForEach-Object { ($_ -split 'REG_(?:EXPAND_)?SZ\s*', 2)[1].Trim() })
    $userPath = @(($rawPath | Select-Object -First 1) -split ';' | Where-Object { $_ })
    if (@($userPath | ForEach-Object { [Environment]::ExpandEnvironmentVariables($_) }) -notcontains $LauncherBin) {
        & reg.exe add 'HKCU\Environment' /v Path /t REG_EXPAND_SZ /d ((@($LauncherBin) + $userPath) -join ';') /f | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "reg.exe could not put $LauncherBin on the user PATH (exit code $LASTEXITCODE): the user PATH is written as REG_EXPAND_SZ so that its %VARIABLE% entries keep expanding." }
    }
    Sync-ProcessPath
    if (-not (Get-Command repo -ErrorAction SilentlyContinue)) { throw "The launcher installer did not put repo on PATH ($LauncherBin)." }
    Say "installed the launcher in $LauncherBin and put it on the user PATH"
}

$facts = @(Invoke-Fleet @('facts', '--group', $GroupId, '--platform', 'windows', '--shell', 'powershell', '--addresses', "$StoreOrigin=$NfsHost") | Where-Object { $_ -like '$FLEET_*' })
if ($facts.Count -eq 0) { throw 'The topology command printed no facts.' }
foreach ($fact in $facts) {
    $match = [regex]::Match($fact, '^\$(FLEET_[A-Z0-9_]+)=''((?:[^'']|'''')*)''$')
    if (-not $match.Success) { throw "Unexpected line in the topology facts: $fact" }
    Set-Variable -Name $match.Groups[1].Value -Value $match.Groups[2].Value.Replace("''", "'")
}
if ($FLEET_WORKTREE -ne $Worktree -or $FLEET_NFS_SOURCE -ne $NfsSource -or $FLEET_DRIVE -ne $Drive -or $FLEET_MOUNT_OPTIONS -ne $MountOptions -or $FLEET_SERVER_PLATFORM -ne $ServerPlatform -or $FLEET_SERVER_PROTOCOL -ne $ServerProtocol -or $FLEET_SMB_CACHE -ne (ConvertTo-List $SmbCache) -or $FLEET_NFS_CACHE -ne (ConvertTo-List $NfsCache) -or $FLEET_NFS_CACHE_KEY -ne $NfsCacheKey) { throw "The topology file next to this script differs from the one in the worktree ($FLEET_WORKTREE, $FLEET_NFS_SOURCE, $FLEET_DRIVE, $FLEET_MOUNT_OPTIONS, $FLEET_SERVER_PLATFORM, $FLEET_SERVER_PROTOCOL): copy the topology directory from the worktree and run again." }
Say "canonical worktree $FLEET_WORKTREE, store $FLEET_NFS_SOURCE (role of $StoreOrigin) at $FLEET_DRIVE"

if ($Plan) {
    Say 'plan: nothing below this line is registered or changed'
    if ($relocate) { Invoke-Replica @('relocate', '--dry-run', '--worktree', $RelocateFrom, '--to', $Worktree) }
    elseif ($adopt) { Invoke-Replica @('join', '--adopt', '--dry-run', '--group', $GroupId, '--origin', $OriginId, '--store', $StoreRoot) }
    else { Say 'the worktree holds no repository to adopt: a plain provisioning extracts the bootstrap archive and joins, or refreshes the tasks of the joined machine' }
    exit 0
}
Say 'step 6: store mount task'
if ($Server) { Say 'group server: no remount task' } else {
    $mountName = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'mount-target') | Select-Object -First 1)
    $mountDefinition = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'mount-definition')) -join "`n"
    Register-ScheduledTask -TaskName $mountName -Xml $mountDefinition -User $User -Force | Out-Null
    Say "registered task $mountName (maps $FLEET_DRIVE in the logon session at every logon and again every minute while the drive is missing)"
}

Say 'step 7b: the boot program outside the worktree (the service entry never depends on replicated code)'
$accountName = ($User -split '\\')[-1]
$accountHome = (Get-CimInstance Win32_UserProfile -ErrorAction SilentlyContinue | Where-Object { $_.LocalPath -like "*\$accountName" } | Select-Object -First 1).LocalPath
if (-not $accountHome) { $accountHome = $env:USERPROFILE }
$entryDirectory = Join-Path $accountHome 'AppData\Local\semio\fleet\entry'
$entryProgram = (Get-ChildItem -LiteralPath (Join-Path $FLEET_REPLICA_DIRECTORY '🥾️entry') -Filter '*.mjs' -ErrorAction SilentlyContinue | Select-Object -First 1).FullName
if (-not $entryProgram) { Stop-FleetCodeOld }
$env:SEMIO_WORKSPACE_ROOT = $Worktree
$env:SEMIO_FLEET_ENTRY_DIR = $entryDirectory
Push-Location $Worktree
try { $entryResult = (& repo $entryProgram install --worktree $Worktree --store $StoreRoot --take-over | Select-Object -Last 1) } finally { Pop-Location; Remove-Item Env:SEMIO_FLEET_ENTRY_DIR -ErrorAction SilentlyContinue }
Say "boot program: $entryResult"
if (-not (Test-Path -LiteralPath (Join-Path $entryDirectory 'entry.json'))) { throw "The boot program could not be installed into ${entryDirectory}: $entryResult" }

Say 'step 8: replica and compact tasks (registered, started after join)'
$taskName = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'target') | Select-Object -First 1)
$definition = ((Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows')) -join "`n").Replace('__WORKER_HOME__', $accountHome)
Register-ScheduledTask -TaskName $taskName -Xml $definition -User $User -Force | Out-Null
Say "registered task $taskName"
$compactName = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'compact-target') | Select-Object -First 1)
$role = if ($Server) { 'server' } else { 'worker' }
$compactDefinition = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'compact-definition', '--role', $role)) -join "`n"
Register-ScheduledTask -TaskName $compactName -Xml $compactDefinition -User $User -Force | Out-Null
Say "registered task $compactName (daily)"

if ($Server) {
    Say 'step 8b: group server share, firewall, remote cache and relay'
    function Get-ServerPart([string]$Part) { return (Invoke-Fleet @('server-plan', '--group', $GroupId, '--part', $Part)) -join "`n" }
    $tokens = (Get-ServerPart 'cache-environment-target').Trim()
    if (-not (Test-Path -LiteralPath $tokens)) { throw "$tokens does not exist: create it with the real tokens (lines of: server-plan --group $GroupId --part cache-environment), then run again." }
    $tokenText = Get-Content -Raw -LiteralPath $tokens
    foreach ($line in (Get-ServerPart 'cache-environment') -split "`n") {
        $tokenName = $line.Split('=')[0]
        if ($tokenText -notmatch "(?m)^$tokenName=[^<\r\n]") { throw "$tokens has no real value for $tokenName." }
    }
    & icacls $tokens /inheritance:r /grant:r "${User}:(R)" | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "icacls failed for $tokens" }
    New-Item -ItemType Directory -Force -Path $FLEET_SERVER_CACHE_DIRECTORY | Out-Null
    & icacls $FLEET_SERVER_CACHE_DIRECTORY /grant "${User}:(OI)(CI)M" | Out-Null
    if ($Smb) {
        foreach ($account in 'semio-fleet', 'semio-fleet-peer') {
            if (-not (Get-LocalUser -Name $account -ErrorAction SilentlyContinue)) { throw "The local account $account does not exist: create it first (prepare part of the server plan)." }
        }
    } elseif (-not (Get-Command Install-WindowsFeature -ErrorAction SilentlyContinue)) {
        throw 'Server for NFS needs Windows Server (Install-WindowsFeature FS-NFS-Service). On Windows 10/11 set the group server protocol to "smb" in the topology file.'
    }
    foreach ($part in 'feature', 'share', 'firewall') {
        Invoke-Expression (Get-ServerPart $part)
        Say "applied server part $part"
    }
    foreach ($part in 'cache', 'relay') {
        $serverTask = (Get-ServerPart "$part-target").Trim()
        Register-ScheduledTask -TaskName $serverTask -Xml (Get-ServerPart "$part-unit") -User $User -Force | Out-Null
        Say "registered task $serverTask"
    }
}

Say 'step 9: settle the extracted tree and join'
if ($joined) {
    Say "already joined as $OriginId in group ${GroupId}: join is skipped, the mounts and tasks above were refreshed"
} elseif ($FirstMachine) {
    if (@(Get-ChildItem -LiteralPath (Join-Path $StoreRoot 'v1\origins') -ErrorAction SilentlyContinue).Count -gt 0) { throw 'The store already has origins: this is not the first machine of the fleet, run without -FirstMachine.' }
    Invoke-Replica @('seed', '--group', $GroupId, '--origin', $OriginId, '--store', $StoreRoot)
} elseif ($relocate) {
    Say "relocating $OriginId from $RelocateFrom to ${Worktree}: the tasks stop, the identity moves (the origin id stays), the store is adopted in the new tree"
    foreach ($stopTask in @('semio-fleet-replica', 'semio-fleet-compact', 'semio-fleet-relay', 'semio-fleet-cache')) { Stop-ScheduledTask -TaskName $stopTask -ErrorAction SilentlyContinue }
    $forceArgument = if ($Force) { @('--force') } else { @() }
    Invoke-Replica @('relocate', '--worktree', $RelocateFrom, '--to', $Worktree, '--phase', 'prepare')
    Invoke-Replica (@('relocate', '--worktree', $RelocateFrom, '--to', $Worktree, '--phase', 'move', '--wait', '60') + $forceArgument)
} elseif ($adopt) {
    $forceArgument = if ($Force) { @('--force') } else { @() }
    Invoke-Replica (@('join', '--adopt', '--group', $GroupId, '--origin', $OriginId, '--store', $StoreRoot) + $forceArgument)
} else {
    if ($Server) {
        $relayPeers = (Get-ServerPart 'relay-peers').Trim()
        if ($relayPeers) { Invoke-Replica @('relay', '--once', '--group', $GroupId, '--store', $StoreRoot, '--peers', $relayPeers) }
    }
    Invoke-Replica @('bootstrap-settle', '--store', $StoreRoot)
    Invoke-Replica @('join', '--group', $GroupId, '--origin', $OriginId, '--store', $StoreRoot)
    Remove-Item -LiteralPath $extractMarker -Force -ErrorAction SilentlyContinue
}

Say 'step 10: start the tasks'
$startTasks = @($taskName)
if (-not $Server) { $startTasks = @($mountName) + $startTasks }
if ($Server) { $startTasks += 'semio-fleet-cache'; $startTasks += 'semio-fleet-relay' }
foreach ($startTask in $startTasks) {
    if ((Get-ScheduledTask -TaskName $startTask).State -eq 'Running') { Say "already running $startTask" } else {
        Start-ScheduledTask -TaskName $startTask
        Say "started $startTask"
    }
}
if ($relocate) {
    Invoke-Replica @('relocate', '--worktree', $RelocateFrom, '--to', $Worktree, '--phase', 'finish', '--wait', '120')
    Say "the old worktree $RelocateFrom is retired (marker .cache\f\relocated.json) and not deleted: delete it yourself when you are sure"
}
Say "fleet ready: $OriginId is a member of group $GroupId and syncs"

function Exit-NativePending([string]$Command, [string]$Reason) {
    Say "NATIVE BUILD PENDING: $OriginId is a member of group $GroupId and syncs, but its native build is not ready"
    Say "failing command: $Command"
    Say "reason: $Reason"
    if (Test-Path -LiteralPath $BootstrapLog) {
        Say "log: $BootstrapLog (last lines below)"
        Get-Content -LiteralPath $BootstrapLog -Tail 15 | ForEach-Object { Say "  | $_" }
    }
    Say "the completion marker $BootstrapMarker was not written: run this script again to retry the native bootstrap once the cause is fixed (a fix for a broken build arrives through the shared worktree itself); preflight shows it as the warning native-build"
    Say "exit code ${ExitNativePending}: fleet ready, native build pending"
    exit $ExitNativePending
}

if (-not $nativeNeeded) { Say "native bootstrap complete ($BootstrapMarker)" } else {
    Say 'step 11: native bootstrap, the last step (the machine already syncs: a failure here leaves the fleet setup as it is)'
    $command = "powershell.exe -NoProfile -ExecutionPolicy Bypass -File $nativeScript setup"
    Remove-Item -LiteralPath $BootstrapMarker -Force -ErrorAction SilentlyContinue
    if (-not (Get-Command winget -ErrorAction SilentlyContinue)) { Exit-NativePending "the tool check before $command" 'winget is missing: the native bootstrap installs Git, GitHub CLI, ripgrep, jq, SQLite, Python, LLVM and the .NET SDKs through it. Install App Installer (Microsoft.DesktopAppInstaller) from the Microsoft Store, then run again.' }
    $vswhere = if (${env:ProgramFiles(x86)}) { Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe' }
    if (-not $vswhere -or -not (Test-Path -LiteralPath $vswhere)) { Say 'the Visual Studio C++ Build Tools are missing: the native bootstrap installs them (several GB); to install them first run: winget install --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"' }
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $BootstrapLog) | Out-Null
    Remove-Item -LiteralPath $BootstrapLog -Force -ErrorAction SilentlyContinue
    $env:SEMIO_WORKSPACE_ROOT = $Worktree
    $errorPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $nativeScript setup 2>&1 | ForEach-Object { "$_" | Out-Host; Add-Content -LiteralPath $BootstrapLog -Value "$_" -Encoding UTF8 }
        $status = $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $errorPreference
        Remove-Item Env:SEMIO_WORKSPACE_ROOT -ErrorAction SilentlyContinue
    }
    if ($status -ne 0) { Exit-NativePending $command "it exited with code $status" }
    Sync-ProcessPath
    if (-not (Get-Command repo -ErrorAction SilentlyContinue)) { Exit-NativePending $command 'it did not put the repo command on PATH' }
    Set-Content -LiteralPath $BootstrapMarker -Value (Get-Date).ToUniversalTime().ToString('s') -Encoding ascii
    Say "recorded the completed bootstrap in $BootstrapMarker"
}
Say "done: $OriginId is a member of group $GroupId and its native build is complete"
exit 0
