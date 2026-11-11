#region 🧲️Header
#
# Specs: Zero-touch Windows worker provisioning of the shared worktree fleet, from an empty disk. Idempotent: every step probes before it changes anything and prints what it does. Operators run it once per fleet machine from an elevated PowerShell; it changes machine settings (optional features, registry, machine environment, scheduled tasks), so the build never runs it.
#
# Summary: `powershell -NoProfile -ExecutionPolicy Bypass -File "<this file>" <GroupId> <OriginId> [-ArchivePath <directory>] [-FirstMachine | -Server] [-User <DOMAIN\account>]`
# The script needs the topology file next to it (..\🔣️.json): the early steps run before the repo launcher exists and read the worktree path and the NFS export from it; the topology command re-derives the same values later and the script refuses any difference. -User must be the account that runs this elevated shell, because the repository's native bootstrap installs the tools per account.
# Steps: LongPathsEnabled, canonical worktree directory, the attribute and name cache of the store client (SMB: Set-SmbClientConfiguration, NFS: registry values of Client for NFS), Client for NFS and the store mount, the Windows Time service, bootstrap archive (sha256 verified against manifest.json, extracted with the system tar.exe, which is bsdtar and reads the pax UTF-8 names), the standalone `repo` launcher (built from the checkout by the launcher installer, needs neither winget nor a compiler), the remount task, the cache environment (Machine scope), the replica and daily compact scheduled tasks, `bootstrap-settle`, `join`, task start, and only then, as the last step, the repository's native bootstrap (winget packages, the C++ Build Tools, Node.js, Rust, .NET, uv, `repo-mcp:install`, `workspace:setup`).
# Joining the fleet never depends on the native build: a fix for a broken build arrives through the shared worktree itself, so the machine must already be syncing when the native bootstrap runs. The native bootstrap runs last with its output in .cache\f\native-bootstrap.log (ignored local state); the completion marker .cache\f\native-bootstrap.done is written only after it succeeded. When it fails (or winget is missing), the script prints in English the failing command and the log, leaves the marker absent so that the next run retries it, and exits with code 3; the fleet setup stays as it is. `preflight` shows the same state as the warning `native-build`.
# Exit codes: 0 fleet ready and native build complete; 1 error (the script threw, nothing after the failing step ran); 3 fleet ready, native build pending (the machine syncs; run the script again after the cause is fixed or the fix has arrived).
# -ArchivePath is a directory with worktree.tar and manifest.json (default: <drive>\v1\bootstrap of the group store). A worktree that already holds a checkout skips the archive; one that is neither empty nor a checkout is refused. -FirstMachine is for the first machine of the fleet, which has no archive: a checkout is at the canonical path and the store is empty, and `seed` publishes it instead of `join` (refused when the store already has origins; never use it twice).
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
    [string]$User = "$env:USERDOMAIN\$env:USERNAME"
)

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.UTF8Encoding]::new($false)
$OutputEncoding = [Console]::OutputEncoding

$Modules = '🧰️framework\🛍️products\🦑️repo\🔨️modules'
$NativeBootstrap = "$Modules\🔩️native\🥾️bootstrap\🔵️.ps1"
$LauncherInstall = "$Modules\🚀️launcher\📥️install\🔵️.ps1"
$LauncherBin = "$env:LOCALAPPDATA\semio\launcher\bin"
$ExitNativePending = 3
$TopologyRelative = "$Modules\🤝️shared-worktree\🗺️topology"

function Say([string]$Message) { Write-Host "[fleet-provision] $Message" }

$principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { throw 'Run this script from an elevated PowerShell.' }
if ($User -ine "$env:USERDOMAIN\$env:USERNAME") { throw "-User must be the account of this elevated shell ($env:USERDOMAIN\$env:USERNAME): the repository bootstrap installs the tools for it." }

$topologyFiles = @(Get-ChildItem -LiteralPath (Split-Path -Parent $PSScriptRoot) -Filter '*.json' -File)
if ($topologyFiles.Count -ne 1) { throw "The topology file was not found next to this script: $(Split-Path -Parent $PSScriptRoot)" }
$topology = Get-Content -Raw -LiteralPath $topologyFiles[0].FullName | ConvertFrom-Json
$Worktree = $topology.worktree.windows
$Drive = $topology.storeMount.windows.Substring(0, 2)
$group = @($topology.groups | Where-Object { $_.id -eq $GroupId })
if ($group.Count -ne 1 -or -not $Worktree) { throw "The topology has no worktree for windows or no group ${GroupId}." }
$NfsSource = "\\$($group[0].nfs.host)$($group[0].nfs.export.Replace('/', '\'))"
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
if (Test-Path -LiteralPath $machine) {
    $identity = Get-Content -Raw -LiteralPath $machine | ConvertFrom-Json
    if (-not $identity.originId -or -not $identity.groupId) { throw "$machine has no originId and groupId: remove the worktree state first." }
    if ($identity.originId -ne $OriginId) { throw "$machine belongs to origin $($identity.originId), not ${OriginId}: pass that origin, or remove the worktree state first." }
    if ($identity.groupId -ne $GroupId) { throw "$machine belongs to group $($identity.groupId), not ${GroupId}: pass that group, or remove the worktree state first." }
    $joined = $true
}

function Sync-ProcessPath {
    $env:Path = (@([Environment]::GetEnvironmentVariable('Path', 'Machine'), [Environment]::GetEnvironmentVariable('Path', 'User')) | Where-Object { $_ }) -join ';'
}

function Test-BootstrapNeeded { return (-not (Test-Path -LiteralPath $BootstrapMarker)) -or (-not (Get-Command repo -ErrorAction SilentlyContinue)) }

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

function Invoke-Fleet([string[]]$Arguments) {
    $env:SEMIO_WORKSPACE_ROOT = $Worktree
    $script = Join-Path (Join-Path $Worktree $TopologyRelative) '📜️script.ts'
    Push-Location $Worktree
    try {
        $lines = & repo $script @Arguments
        if ($LASTEXITCODE -ne 0) { throw "repo $script $($Arguments -join ' ') failed with exit code $LASTEXITCODE" }
        return $lines
    } finally { Pop-Location }
}

function Invoke-Replica([string[]]$Arguments) {
    $env:SEMIO_WORKSPACE_ROOT = $Worktree
    $script = Join-Path $FLEET_REPLICA_DIRECTORY '📜️script.ts'
    Push-Location $Worktree
    try {
        & repo $script @Arguments
        if ($LASTEXITCODE -ne 0) { throw "repo $script $($Arguments -join ' ') failed with exit code $LASTEXITCODE" }
    } finally { Pop-Location }
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
    if (Test-Path -LiteralPath "$Drive\") { Say "already mounted $Drive" } else {
        & mount.exe -o $MountOptions $NfsSource $Drive
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
if (Test-Path -LiteralPath $nativeScript) {
    Say 'the worktree already holds a checkout: the archive is skipped'
} elseif (@(Get-ChildItem -LiteralPath $Worktree -Force).Count -gt 0) {
    throw "$Worktree is neither empty nor a checkout: empty it, or put a complete checkout there."
} elseif ($FirstMachine) {
    throw "-FirstMachine needs a complete checkout at ${Worktree}: the first machine has no archive to extract."
} else {
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
            if (-not ((Test-Path -LiteralPath "$source\manifest.json") -and (Test-Path -LiteralPath "$source\worktree.tar"))) { throw "No bootstrap archive in ${source}: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass -ArchivePath <directory>." }
            $manifest = Get-Content -Raw -LiteralPath "$source\manifest.json" | ConvertFrom-Json
            Copy-Item -LiteralPath "$source\worktree.tar" -Destination $local -Force
            $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $local).Hash.ToLowerInvariant()
            if ($actual -eq $manifest.sha256) { $verified = $true; Say "archive verified: sha256 $actual, $($manifest.files) files"; break }
            Say "the archive differs from manifest.json (refreshed while it was copied?), attempt $attempt of 5"
            Start-Sleep -Seconds 20
        }
        if (-not $verified) { throw "The archive in $source never matched its manifest.json." }
        $entries = (& $tar -tf $local | Measure-Object -Line).Lines
        if ($LASTEXITCODE -ne 0 -or $entries -ne $manifest.files) { throw "The archive lists $entries entries, manifest.json says $($manifest.files)." }
        Say "extracting with tar.exe into $Worktree"
        & $tar -xf $local -C $Worktree
        if ($LASTEXITCODE -ne 0) { throw "tar.exe failed with exit code $LASTEXITCODE" }
        $nativeNeeded = $true
    } finally { Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue }
}

Say 'step 5: standalone repo launcher (built from the checkout; the native bootstrap comes last)'
Sync-ProcessPath
if (Get-Command repo -ErrorAction SilentlyContinue) { Say 'repo launcher present' } else {
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File (Join-Path $Worktree $LauncherInstall) -Destination $LauncherBin
    if ($LASTEXITCODE -ne 0) { throw "The repo launcher could not be built (exit code ${LASTEXITCODE}): the machine cannot join without it, fix the cause and run again." }
    $userPath = @(([Environment]::GetEnvironmentVariable('Path', 'User')) -split ';' | Where-Object { $_ })
    if ($userPath -notcontains $LauncherBin) { [Environment]::SetEnvironmentVariable('Path', (@($LauncherBin) + $userPath) -join ';', 'User') }
    Sync-ProcessPath
    if (-not (Get-Command repo -ErrorAction SilentlyContinue)) { throw "The launcher installer did not put repo on PATH ($LauncherBin)." }
    Say "installed the launcher in $LauncherBin and put it on the user PATH"
}

$facts = @(Invoke-Fleet @('facts', '--group', $GroupId, '--platform', 'windows', '--shell', 'powershell') | Where-Object { $_ -like '$FLEET_*' })
if ($facts.Count -eq 0) { throw 'The topology command printed no facts.' }
foreach ($fact in $facts) {
    $match = [regex]::Match($fact, '^\$(FLEET_[A-Z0-9_]+)=''((?:[^'']|'''')*)''$')
    if (-not $match.Success) { throw "Unexpected line in the topology facts: $fact" }
    Set-Variable -Name $match.Groups[1].Value -Value $match.Groups[2].Value.Replace("''", "'")
}
if ($FLEET_WORKTREE -ne $Worktree -or $FLEET_NFS_SOURCE -ne $NfsSource -or $FLEET_DRIVE -ne $Drive -or $FLEET_MOUNT_OPTIONS -ne $MountOptions -or $FLEET_SERVER_PLATFORM -ne $ServerPlatform -or $FLEET_SERVER_PROTOCOL -ne $ServerProtocol -or $FLEET_SMB_CACHE -ne (ConvertTo-List $SmbCache) -or $FLEET_NFS_CACHE -ne (ConvertTo-List $NfsCache) -or $FLEET_NFS_CACHE_KEY -ne $NfsCacheKey) { throw "The topology file next to this script differs from the one in the worktree ($FLEET_WORKTREE, $FLEET_NFS_SOURCE, $FLEET_DRIVE, $FLEET_MOUNT_OPTIONS, $FLEET_SERVER_PLATFORM, $FLEET_SERVER_PROTOCOL): copy the topology directory from the worktree and run again." }
Say "canonical worktree $FLEET_WORKTREE, store $FLEET_NFS_SOURCE at $FLEET_DRIVE, cache $FLEET_CACHE_ENDPOINT"

Say 'step 6: remount task'
if ($Server) { Say 'group server: no remount task' } else {
    $mountAction = if ($Smb) { New-ScheduledTaskAction -Execute 'net.exe' -Argument "use $FLEET_DRIVE $FLEET_NFS_SOURCE /persistent:yes" } else { New-ScheduledTaskAction -Execute 'mount.exe' -Argument "-o $FLEET_MOUNT_OPTIONS $FLEET_NFS_SOURCE $FLEET_DRIVE" }
    Register-ScheduledTask -TaskName 'semio-fleet-store-mount' -Action $mountAction -Trigger (New-ScheduledTaskTrigger -AtLogOn -User $User) -User $User -Force | Out-Null
    Say 'registered task semio-fleet-store-mount (remounts at logon)'
}

Say 'step 7: cache environment'
$environment = @(Invoke-Fleet @('env', '--group', $GroupId, '--shell', 'powershell') | Where-Object { $_ -match '^\$env:[A-Z0-9_]+=".*"$' })
if ($environment.Count -eq 0) { throw 'The topology command printed no environment.' }
foreach ($line in $environment) {
    $match = [regex]::Match($line, '^\$env:([A-Z0-9_]+)="(.*)"$')
    $name = $match.Groups[1].Value
    $value = $match.Groups[2].Value
    if ([Environment]::GetEnvironmentVariable($name, 'Machine') -eq $value) { Say "unchanged $name" } else {
        [Environment]::SetEnvironmentVariable($name, $value, 'Machine')
        Say "set $name (Machine)"
    }
    Set-Item -Path "Env:$name" -Value $value
}

Say 'step 8: replica and compact tasks (registered, started after join)'
$taskName = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows', '--part', 'target') | Select-Object -First 1)
$definition = (Invoke-Fleet @('worker-service', '--group', $GroupId, '--platform', 'windows')) -join "`n"
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
} else {
    if ($Server) {
        $relayPeers = (Get-ServerPart 'relay-peers').Trim()
        if ($relayPeers) { Invoke-Replica @('relay', '--once', '--group', $GroupId, '--store', $StoreRoot, '--peers', $relayPeers) }
    }
    Invoke-Replica @('bootstrap-settle', '--store', $StoreRoot)
    Invoke-Replica @('join', '--group', $GroupId, '--origin', $OriginId, '--store', $StoreRoot)
}

Say 'step 10: start the tasks'
$startTasks = @($taskName)
if (-not $Server) { $startTasks = @('semio-fleet-store-mount') + $startTasks }
if ($Server) { $startTasks += 'semio-fleet-cache'; $startTasks += 'semio-fleet-relay' }
foreach ($startTask in $startTasks) {
    if ((Get-ScheduledTask -TaskName $startTask).State -eq 'Running') { Say "already running $startTask" } else {
        Start-ScheduledTask -TaskName $startTask
        Say "started $startTask"
    }
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
