#region Header
#
# Specs: Mock layer that runs the real Windows provisioning script (copied next to a topology file) on any host with PowerShell 7: every Windows-only cmdlet and executable is a function with the real parameter names that records its call, drive letters are PowerShell drives onto a temporary directory.
# Summary: `pwsh -NoProfile -File <this file> <case.json>`; the case names the script copy, the script parameters and the machine state; the last stdout line is `@@RESULT@@ <json>` with the recorded calls, scheduled tasks, shares, rules, machine environment and the error the script stopped with.
#
#endregion

param([Parameter(Mandatory = $true)][string]$CaseFile)
$ErrorActionPreference = 'Stop'
$case = Get-Content -Raw -LiteralPath $CaseFile | ConvertFrom-Json -AsHashtable
$root = $case.root
$global:Calls = [Collections.Generic.List[string]]::new()
$global:Case = $case
$global:State = @{ Registry = $case.registry; Keys = @($case.keys); Features = $case.features; Tasks = [ordered]@{}; Started = @(); NfsShares = @{}; SmbShares = @{}; Rules = @{}; Users = @($case.users); SmbClient = @{ DirectoryCacheLifetime = 10; FileInfoCacheLifetime = 10; FileNotFoundCacheLifetime = 5 }; TimeStart = $(if ($case.timeStart) { $case.timeStart } else { 'Automatic' }); TimeStatus = $(if ($case.timeStatus) { $case.timeStatus } else { 'Running' }) }
if ($case.smbClient) { foreach ($key in $case.smbClient.Keys) { $global:State.SmbClient[$key] = $case.smbClient[$key] } }
$env:USERDOMAIN = 'DOM'
$env:USERNAME = 'op'
$env:SystemRoot = "$root/Windows"
$env:TEMP = "$root/tmp"
$env:HARNESS_ROOT = $root
$env:LOCALAPPDATA = 'C:\Users\op\AppData\Local'

Add-Type -TypeDefinition @'
using System.Collections.Generic;
public static class MockEnvironment {
    public static Dictionary<string, string> Machine = new Dictionary<string, string>();
    public static Dictionary<string, string> User = new Dictionary<string, string>();
    public static string GetEnvironmentVariable(string name, string scope) { var table = scope == "Machine" ? Machine : User; string value; return table.TryGetValue(name, out value) ? value : null; }
    public static void SetEnvironmentVariable(string name, string value, string scope) { (scope == "Machine" ? Machine : User)[name] = value; }
}
'@
[MockEnvironment]::Machine['Path'] = 'C:\Windows\System32'

foreach ($letter in 'C') { New-Item -ItemType Directory -Force -Path "$root/$letter" | Out-Null; New-PSDrive -Name $letter -PSProvider FileSystem -Root "$root/$letter" -Scope Global | Out-Null }
function global:Mount-Store { if (-not (Get-PSDrive -Name S -ErrorAction SilentlyContinue)) { New-PSDrive -Name S -PSProvider FileSystem -Root "$($global:Case.root)/store" -Scope Global | Out-Null } }
if ($case.mounted) { Mount-Store }

function global:Rec([string]$Name, $Values) { $global:Calls.Add((@($Name) + @($Values | ForEach-Object { "$_" }) -join ' ').Trim()) }
function global:Bound([hashtable]$Parameters, [string[]]$Skip = @()) { $Parameters.Keys | Where-Object { $_ -notin $Skip } | Sort-Object | ForEach-Object { "-$_ $(@($Parameters[$_]) -join ',')" } }
function global:Native([string]$Name, $Arguments, [string]$ExitKey = $Name) {
    Rec $Name $Arguments
    $global:LASTEXITCODE = [int]($global:Case.exit[$ExitKey])
}

function global:icacls { Native 'icacls' $args }
function global:net.exe { Native 'net.exe' $args; if ($global:LASTEXITCODE -eq 0) { Mount-Store } }
function global:mount.exe { Native 'mount.exe' $args; if ($global:LASTEXITCODE -eq 0) { Mount-Store } }
function global:powershell.exe {
    if ("$args" -like '*launcher*') {
        Native 'powershell.exe' $args 'powershell.exe:launcher'
        if ($global:LASTEXITCODE -eq 0) { $global:State.Launcher = $true; Set-Item Function:global:repo $global:RepoBlock }
        return
    }
    Native 'powershell.exe' $args 'powershell.exe:native'
    Rec 'powershell.exe(env)' "SEMIO_WORKSPACE_ROOT=$env:SEMIO_WORKSPACE_ROOT"
    if ($global:LASTEXITCODE -eq 0) { $global:State.Bootstrapped = $true }
    if ($global:Case.nativeOutput) { $global:Case.nativeOutput -split "\r?\n" }
}
$repoBlock = {
    $operation = $args[1]
    Rec 'repo' (@("[$((Get-Location).Path -replace [regex]::Escape($global:Case.root), '<root>')]") + $args[1..($args.Count - 1)])
    $global:LASTEXITCODE = [int]($global:Case.exit["repo:$operation"])
    $key = $args[1..($args.Count - 1)] -join ' '
    if ($global:Case.plans.ContainsKey($key)) { $global:Case.plans[$key] -split "\r?\n" } elseif ($operation -in 'facts', 'env', 'worker-service', 'server-plan') { throw "unexpected topology call: $key" }
}
if ($case.repo) { Set-Item Function:global:repo $repoBlock }
$global:RepoBlock = $repoBlock

function global:Get-WindowsOptionalFeature { [CmdletBinding()]param([switch]$Online, [string[]]$FeatureName) foreach ($name in $FeatureName) { [pscustomobject]@{ FeatureName = $name; State = $(if ($global:State.Features[$name]) { 'Enabled' } else { 'Disabled' }) } } }
function global:Enable-WindowsOptionalFeature { [CmdletBinding()]param([switch]$Online, [string[]]$FeatureName, [switch]$All, [switch]$NoRestart) Rec 'Enable-WindowsOptionalFeature' (Bound $PSBoundParameters); foreach ($name in $FeatureName) { $global:State.Features[$name] = $true }; [pscustomobject]@{ RestartNeeded = $false } }
function global:Test-Path { $key = @($args | Where-Object { "$_" -like 'HKLM:*' }); if ($key.Count -gt 0) { $key[0] -in $global:State.Keys } else { Microsoft.PowerShell.Management\Test-Path @args } }
function global:New-Item { $key = @($args | Where-Object { "$_" -like 'HKLM:*' }); if ($key.Count -gt 0) { Rec 'New-Item' @('-Path', $key[0]); $global:State.Keys += $key[0] } else { Microsoft.PowerShell.Management\New-Item @args } }
function global:nfsadmin { Native 'nfsadmin' $args }
function global:Get-ItemProperty { [CmdletBinding()]param([string]$Path, [string]$Name) if ($global:State.Registry.ContainsKey($Name)) { [pscustomobject]@{ $Name = $global:State.Registry[$Name] } } else { Write-Error 'missing' } }
function global:New-ItemProperty { [CmdletBinding()]param([string]$Path, [string]$Name, $Value, [string]$PropertyType, [switch]$Force) Rec 'New-ItemProperty' (Bound $PSBoundParameters); $global:State.Registry[$Name] = $Value }
function global:Get-LocalUser { [CmdletBinding()]param([string]$Name) if ($Name -in $global:State.Users) { [pscustomobject]@{ Name = $Name } } else { Write-Error 'missing' } }
if (-not $case.windowsClient) { function global:Install-WindowsFeature { [CmdletBinding()]param([Parameter(Position = 0)][string[]]$Name, [switch]$IncludeManagementTools) Rec 'Install-WindowsFeature' (Bound $PSBoundParameters) } }
function global:Get-NfsShare { [CmdletBinding()]param([string]$Name) if ($global:State.NfsShares.ContainsKey($Name)) { $global:State.NfsShares[$Name] } else { Write-Error 'missing' } }
function global:New-NfsShare {
    [CmdletBinding()]param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$Path, [ValidateSet('sys', 'krb5', 'krb5i', 'krb5p', 'all')][string[]]$Authentication, [bool]$AllowRootAccess, [bool]$EnableAnonymousAccess, [bool]$EnableUnmappedAccess, [int]$AnonymousGid, [int]$AnonymousUid, [ValidateSet('readonly', 'readwrite', 'no-access')][string]$Permission)
    Rec 'New-NfsShare' (Bound $PSBoundParameters)
    $global:State.NfsShares[$Name] = [pscustomobject]@{ Name = $Name; Path = $Path; Grants = @() }
}
function global:Grant-NfsSharePermission {
    [CmdletBinding()]param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$ClientName, [ValidateSet('host', 'netgroup', 'ipv4subnet', 'ipv6subnet', 'builtin')][string]$ClientType, [ValidateSet('readonly', 'readwrite', 'no-access')][string]$Permission, [bool]$AllowRootAccess, [switch]$LanguageEncoding)
    Rec 'Grant-NfsSharePermission' (Bound $PSBoundParameters)
    if (-not $global:State.NfsShares.ContainsKey($Name)) { throw "no NFS share $Name" }
}
function global:Get-SmbShare { [CmdletBinding()]param([string]$Name) if ($global:State.SmbShares.ContainsKey($Name)) { $global:State.SmbShares[$Name] } else { Write-Error 'missing' } }
function global:New-SmbShare { [CmdletBinding()]param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$Path, [string[]]$ChangeAccess, [string[]]$ReadAccess, [string[]]$FullAccess) Rec 'New-SmbShare' (Bound $PSBoundParameters); $global:State.SmbShares[$Name] = [pscustomobject]@{ Name = $Name } }
function global:Set-SmbServerConfiguration { [CmdletBinding()]param([bool]$EnableSMB2Protocol, [switch]$Force) Rec 'Set-SmbServerConfiguration' (Bound $PSBoundParameters) }
function global:Get-NetFirewallRule { [CmdletBinding()]param([string]$DisplayName) if ($global:State.Rules.ContainsKey($DisplayName)) { $global:State.Rules[$DisplayName] } else { Write-Error 'missing' } }
function global:Remove-NetFirewallRule { [CmdletBinding()]param([Parameter(ValueFromPipeline = $true)]$InputObject) process { Rec 'Remove-NetFirewallRule' $InputObject.DisplayName; $global:State.Rules.Remove($InputObject.DisplayName) } }
function global:New-NetFirewallRule {
    [CmdletBinding()]param([Parameter(Mandatory = $true)][string]$DisplayName, [ValidateSet('Inbound', 'Outbound')][string]$Direction, [ValidateSet('Allow', 'Block')][string]$Action, [string]$Protocol, [string[]]$LocalPort, [string[]]$RemoteAddress)
    Rec 'New-NetFirewallRule' (Bound $PSBoundParameters)
    $global:State.Rules[$DisplayName] = [pscustomobject]@{ DisplayName = $DisplayName }
}
function global:Resolve-DnsName { [CmdletBinding()]param([string]$Name, [string]$Type) Rec 'Resolve-DnsName' (Bound $PSBoundParameters); [pscustomobject]@{ IPAddress = "192.0.2.$($Name.Length)" } }
function global:New-ScheduledTaskAction { [CmdletBinding()]param([string]$Execute, [string]$Argument) [pscustomobject]@{ Execute = $Execute; Argument = $Argument } }
function global:New-ScheduledTaskTrigger { [CmdletBinding()]param([switch]$AtLogOn, [string]$User) [pscustomobject]@{ AtLogOn = $AtLogOn.IsPresent; User = $User } }
function global:Register-ScheduledTask {
    [CmdletBinding()]param([Parameter(Mandatory = $true)][string]$TaskName, $Action, $Trigger, [string]$Xml, [string]$User, [switch]$Force)
    Rec 'Register-ScheduledTask' ((Bound $PSBoundParameters 'Xml', 'Action', 'Trigger') + $(if ($Xml) { '-Xml' }) + $(if ($Action) { "-Action $($Action.Execute) $($Action.Argument)" }) + $(if ($Trigger) { "-Trigger AtLogOn=$($Trigger.AtLogOn) User=$($Trigger.User)" }))
    if ($Xml) { [void][xml]$Xml }
    $global:State.Tasks[$TaskName] = @{ Xml = $Xml; User = $User; Exe = $Action.Execute; Arguments = $Action.Argument }
}
function global:Get-ScheduledTask { [CmdletBinding()]param([string]$TaskName) if (-not $global:State.Tasks.Contains($TaskName)) { throw "no task $TaskName" }; [pscustomobject]@{ TaskName = $TaskName; State = $(if ($TaskName -in $global:State.Started -or $TaskName -in $global:Case.running) { 'Running' } else { 'Ready' }) } }
function global:Start-ScheduledTask { [CmdletBinding()]param([string]$TaskName) Rec 'Start-ScheduledTask' $TaskName; $global:State.Started += $TaskName }
function global:Get-SmbClientConfiguration { [CmdletBinding()]param() [pscustomobject]$global:State.SmbClient }
function global:Set-SmbClientConfiguration { [CmdletBinding()]param([int]$DirectoryCacheLifetime, [int]$FileInfoCacheLifetime, [int]$FileNotFoundCacheLifetime, [switch]$Force) Rec 'Set-SmbClientConfiguration' (Bound $PSBoundParameters); foreach ($key in @($PSBoundParameters.Keys | Where-Object { $_ -ne 'Force' })) { $global:State.SmbClient[$key] = $PSBoundParameters[$key] } }
function global:Get-Service { [CmdletBinding()]param([string]$Name) if ($global:Case.noTimeService) { Write-Error 'missing' } else { [pscustomobject]@{ Name = $Name; StartType = $global:State.TimeStart; Status = $global:State.TimeStatus } } }
function global:Set-Service { [CmdletBinding()]param([string]$Name, [string]$StartupType) Rec 'Set-Service' (Bound $PSBoundParameters); $global:State.TimeStart = $StartupType }
function global:Start-Service { [CmdletBinding()]param([string]$Name) Rec 'Start-Service' (Bound $PSBoundParameters); $global:State.TimeStatus = 'Running' }
function global:w32tm.exe { 'time.windows.com' }
if (-not $case.noWinget) { function global:winget { Native 'winget' $args } }
function global:Start-Sleep { [CmdletBinding()]param([int]$Seconds) Rec 'Start-Sleep' $Seconds }

$text = [IO.File]::ReadAllText($case.script)
$patches = [ordered]@{
    '[Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()' = '$global:Principal'
    '[Environment]::GetEnvironmentVariable' = '[MockEnvironment]::GetEnvironmentVariable'
    '[Environment]::SetEnvironmentVariable' = '[MockEnvironment]::SetEnvironmentVariable'
}
foreach ($patch in $patches.GetEnumerator()) {
    if (-not $text.Contains($patch.Key)) { throw "the script no longer contains: $($patch.Key)" }
    $text = $text.Replace($patch.Key, $patch.Value)
}
[IO.File]::WriteAllText($case.script, $text, [Text.UTF8Encoding]::new($true))
$global:Principal = [pscustomobject]@{}
$global:Principal | Add-Member -MemberType ScriptMethod -Name IsInRole -Value { param($role) [bool]$global:Case.admin }

$errorMessage = $null
$scriptArguments = $case.arguments
$global:LASTEXITCODE = 0
try { & $case.script @scriptArguments } catch { $errorMessage = $_.Exception.Message }
[Console]::Out.Flush()
$tasks = [ordered]@{}
foreach ($task in $global:State.Tasks.GetEnumerator()) { $tasks[$task.Key] = $task.Value }
$result = [ordered]@{ error = $errorMessage; calls = $global:Calls; tasks = $tasks; started = $global:State.Started; registry = $global:State.Registry; machine = [MockEnvironment]::Machine; rules = @($global:State.Rules.Keys); nfsShares = @($global:State.NfsShares.Keys); smbShares = @($global:State.SmbShares.Keys); features = $global:State.Features; bootstrapped = [bool]$global:State.Bootstrapped; launcher = [bool]$global:State.Launcher; user = [MockEnvironment]::User; exitCode = $(if ($errorMessage) { 1 } else { [int]$global:LASTEXITCODE }) }
"@@RESULT@@ $($result | ConvertTo-Json -Depth 6 -Compress)"
