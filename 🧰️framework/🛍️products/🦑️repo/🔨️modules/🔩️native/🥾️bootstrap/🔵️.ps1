#region 🧲️Header
#
# 2026 Ueli Saluz <ueli@semio-tech.com>
#
# Specs: Zero-touch Windows-native bootstrap that installs Node.js, uv and rustup from checksum-pinned archives and builds the `repo` launcher (which provisions the pinned JavaScript runtime itself) (Go is provisioned by the repository into its tools partition) (repo pins), installs the remaining machine dependencies with winget when missing, prepares repo-local caches and env vars, applies repo-local git settings (core.longpaths) and syncs workspace dependencies. Needs no elevation: the cache partitions are directories with per-directory NTFS case sensitivity.
#
# Summary: Windows-native bootstrap for the compose monorepo. Invoke from a fresh clone with Windows PowerShell 5.1 or PowerShell 7:
# `powershell -NoProfile -ExecutionPolicy Bypass -File "🧰️framework\🛍️products\🦑️repo\🔨️modules\🔩️native\🥾️bootstrap\🔵️.ps1" setup`
# `repo ./📜️script.ts setup native rust` installs only Rust and its C++ linker.
# Saved as UTF-8 with BOM: Windows PowerShell 5.1
# decodes a BOM-less script with the ANSI code page and would turn every emoji path literal below into a different path.
#
#endregion 🧲️Header

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet("setup", "rust")]
    [string]$Command = "setup",
    [switch]$SkipMachineInstall,
    [switch]$SkipGlobalCliInstall,
    [switch]$SkipPlaywrightInstall,
    [switch]$SkipRepoBootstrap,
    [switch]$StopRepoPython
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)

#region 🎯️Targets
$script:PythonKind = "3.14"
#endregion 🎯️Targets

#region 🔧️Helpers
function Write-Step {
    param([string]$Message)
    Write-Host "[compose] $Message"
}

function Get-RepoRoot {
    param([string]$Start = $PSScriptRoot)
    $explicit = [bool]$env:SEMIO_WORKSPACE_ROOT
    $dir = (Resolve-Path -LiteralPath $(if ($explicit) { $env:SEMIO_WORKSPACE_ROOT } else { $Start })).Path
    for ($i = 0; $i -lt 32; $i++) {
        $project = Join-Path $dir "📋️project.json"
        $package = Join-Path $dir "package.json"
        if ((Test-Path -LiteralPath $project -PathType Leaf) -and (Test-Path -LiteralPath $package -PathType Leaf) -and
            -not ((Get-Item -LiteralPath $project).Attributes -band [IO.FileAttributes]::ReparsePoint) -and
            -not ((Get-Item -LiteralPath $package).Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            $json = Get-Content -LiteralPath $package -Raw
            if ([string]::IsNullOrWhiteSpace($json)) { throw "Invalid workspace package: $package" }
            $manifest = $json | ConvertFrom-Json -ErrorAction Stop
            if ($null -ne $manifest) {
                $property = $manifest.PSObject.Properties | Where-Object { $_.Name -ceq "workspaces" }
                if ($null -ne $property -and ($property.Value -is [Array] -or $property.Value -is [PSCustomObject])) {
                    return $dir
                }
            }
        }
        if ($explicit) { break }
        $parent = Split-Path $dir -Parent
        if (-not $parent -or $parent -eq $dir) { break }
        $dir = $parent
    }
    throw "Workspace root requires paired package.json workspaces and 📋️project.json"
}

function Join-HomePath {
    param([string[]]$Segments)
    $path = $HOME
    foreach ($segment in $Segments) {
        $path = Join-Path $path $segment
    }
    return $path
}

function Ensure-Directory {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Path $Path -Force | Out-Null
    }
}

function Refresh-CurrentProcessPath {
    $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $extraPaths = @(
        (Join-HomePath @(".cargo", "bin")),
        (Join-HomePath @(".local", "bin")),
        (Join-Path $env:LOCALAPPDATA "Microsoft\WinGet\Links")
    ) | Where-Object { $_ -and (Test-Path -LiteralPath $_) }
    $segments = @($machinePath, $userPath) + $extraPaths + @($env:Path)
    $env:Path = (($segments -join ";") -split ";" | Where-Object { $_ } | Select-Object -Unique) -join ";"
}

function Set-UserPathPriority {
    param([string[]]$PreferredEntries)

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $segments = @()
    if ($userPath) {
        $segments = ($userPath -split ";") | Where-Object { $_ }
    }

    $normalizedPreferred = $PreferredEntries | Where-Object { $_ } | Select-Object -Unique
    $remaining = $segments | Where-Object { $normalizedPreferred -notcontains $_ }
    $ordered = @($normalizedPreferred + $remaining | Select-Object -Unique)
    $updated = $ordered -join ";"
    [Environment]::SetEnvironmentVariable("Path", $updated, "User")
    $env:Path = (($ordered + (($env:Path -split ";") | Where-Object { $_ })) | Select-Object -Unique) -join ";"
}

function Test-WingetPackageInstalled {
    param([string]$Id)
    $output = & winget list --exact --id $Id --accept-source-agreements --disable-interactivity 2>&1
    return $LASTEXITCODE -eq 0 -and (($output | Out-String) -match [Regex]::Escape($Id))
}

function Sync-WingetPackage {
    param(
        [string]$Id,
        [string]$Label,
        [string[]]$AdditionalArguments = @()
    )

    if (Test-WingetPackageInstalled -Id $Id) {
        Write-Step "$Label ($Id) is already installed."
        return
    }

    $baseArguments = @(
        "--exact",
        "--id",
        $Id,
        "--accept-package-agreements",
        "--accept-source-agreements",
        "--disable-interactivity",
        "--silent"
    ) + $AdditionalArguments

    Write-Step "Installing $Label ($Id)..."
    & winget install @baseArguments
    if ($LASTEXITCODE -ne 0) {
        throw "winget install failed for $Id"
    }
}

function Set-UserEnvironmentVariable {
    param(
        [string]$Name,
        [string]$Value
    )

    [Environment]::SetEnvironmentVariable($Name, $Value, "User")
    Set-Item -Path ("Env:{0}" -f $Name) -Value $Value
}

function Invoke-RepoCommand {
    param(
        [string]$FilePath,
        [string[]]$ArgumentList,
        [string]$WorkingDirectory,
        [int[]]$SuccessCodes = @(0)
    )

    Write-Step ("Running: {0} {1}" -f $FilePath, ($ArgumentList -join " "))
    Push-Location $WorkingDirectory
    try {
        & $FilePath @ArgumentList
        if ($SuccessCodes -notcontains $LASTEXITCODE) {
            throw "Command failed: $FilePath $($ArgumentList -join ' ')"
        }
    } finally {
        Pop-Location
    }
}

function Get-FirstCommandPath {
    param([string[]]$Candidates)

    foreach ($candidate in $Candidates) {
        if (Test-Path -LiteralPath $candidate) {
            return (Resolve-Path $candidate).Path
        }

        $command = Get-Command $candidate -ErrorAction SilentlyContinue
        if ($null -ne $command) {
            return $command.Source
        }
    }

    return $null
}

function Get-CommandPathOrThrow {
    param(
        [string]$Label,
        [string[]]$Candidates
    )

    $path = Get-FirstCommandPath -Candidates $Candidates
    if (-not $path) {
        throw "$Label was not found on PATH after bootstrap."
    }

    return $path
}

function Test-ElevatedSession {
    $principal = [Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-HostArchitecture {
    $native = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
    if ($native -eq "ARM64") { return "arm64" }
    return "x64"
}

function Get-PinnedAsset {
    param([string]$Tool)

    $architecture = Get-HostArchitecture
    $assets = (Get-Content -LiteralPath $script:PinsFile -Raw -Encoding UTF8 | ConvertFrom-Json).definitions.Pins.const.assets
    $asset = $assets | Where-Object { $_.tool -eq $Tool -and $_.platform -eq "win32" -and $_.arch -eq $architecture } | Select-Object -First 1
    if ($null -eq $asset) {
        throw "No pinned $Tool for win32/$architecture in $script:PinsFile."
    }
    return $asset
}

function Get-PinnedGoDirectory {
    param([string]$RepoRoot)

    $asset = Get-PinnedAsset -Tool "go"
    $share = Join-Path $env:LOCALAPPDATA "semio\go\v$($asset.version)\bin\go.exe"
    if (Test-Path -LiteralPath $share -PathType Leaf) { return (Split-Path -Parent $share) }
    $prefix = $asset.sha256.Substring(0, 16)
    $partition = Join-Path $RepoRoot ".cache\b\t\m\native\go\$($asset.version)\win32-$($asset.arch)-$prefix\bin\go.exe"
    if (Test-Path -LiteralPath $partition -PathType Leaf) { return (Split-Path -Parent $partition) }
    $found = Get-FirstCommandPath @("go.exe")
    if ($found) { return (Split-Path -Parent $found) }
    return $null
}

function Get-InstalledVersion {
    param([string]$Command)

    $path = Get-FirstCommandPath @($Command)
    if (-not $path) { return $null }
    $output = (& $path "--version" 2>$null | Out-String)
    if ($LASTEXITCODE -ne 0) { $output = (& $path "version" 2>$null | Out-String) }
    $match = [Regex]::Match($output, "\d+\.\d+(\.\d+)?")
    if ($match.Success) { return $match.Value }
    return $null
}

function Save-VerifiedDownload {
    param([string]$Url, [string]$Sha256, [string]$Destination)

    $ProgressPreference = "SilentlyContinue"
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $Url -OutFile $Destination -UseBasicParsing
    $algorithm = [Security.Cryptography.SHA256]::Create()
    $stream = [IO.File]::OpenRead($Destination)
    try {
        $actual = ([BitConverter]::ToString($algorithm.ComputeHash($stream))).Replace("-", "").ToLowerInvariant()
    } finally {
        $stream.Dispose()
        $algorithm.Dispose()
    }
    if ($actual -ne $Sha256) {
        Remove-Item -LiteralPath $Destination -Force
        throw "Checksum mismatch for $Url (expected $Sha256, got $actual)."
    }
}

function New-ScratchDirectory {
    $path = Join-Path ([IO.Path]::GetTempPath()) ("semio-" + [Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Path $path | Out-Null
    return $path
}

function Remove-ScratchDirectory {
    param([string]$Path)
    $resolved = [IO.Path]::GetFullPath($Path)
    $temporaryRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if (-not $resolved.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase) -or [IO.Path]::GetFileName($resolved) -notmatch '^semio-[0-9a-f]{32}$') { throw "Owned bootstrap scratch directory required" }
    Remove-Item -LiteralPath $resolved -Recurse -Force -ErrorAction SilentlyContinue
}

function Install-PinnedDirectory {
    param($Asset, [string]$Destination, [string]$Child)

    if (Test-Path -LiteralPath $Destination) { return }
    $scratch = New-ScratchDirectory
    try {
        $archive = Join-Path $scratch "archive.zip"
        Save-VerifiedDownload -Url $Asset.url -Sha256 $Asset.sha256 -Destination $archive
        $staging = Join-Path $scratch "staging"
        Expand-Archive -LiteralPath $archive -DestinationPath $staging -Force
        Ensure-Directory -Path (Split-Path $Destination -Parent)
        Move-Item -LiteralPath (Join-Path $staging $Child) -Destination $Destination
    } finally {
        Remove-Item -LiteralPath $scratch -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Install-PinnedExecutables {
    param($Asset, [string]$Destination, [string[]]$FileNames)

    $scratch = New-ScratchDirectory
    try {
        $archive = Join-Path $scratch "archive.zip"
        Save-VerifiedDownload -Url $Asset.url -Sha256 $Asset.sha256 -Destination $archive
        $staging = Join-Path $scratch "staging"
        Expand-Archive -LiteralPath $archive -DestinationPath $staging -Force
        Ensure-Directory -Path $Destination
        foreach ($name in $FileNames) {
            $found = Get-ChildItem -LiteralPath $staging -Recurse -Filter $name | Select-Object -First 1
            if ($null -eq $found) { throw "The pinned archive $($Asset.url) holds no $name." }
            $target = Join-Path $Destination $name
            Copy-Item -LiteralPath $found.FullName -Destination "$target.partial" -Force
            Move-Item -LiteralPath "$target.partial" -Destination $target -Force
        }
    } finally {
        Remove-Item -LiteralPath $scratch -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Test-DotnetSdkInstalled {
    param([string]$Version)

    $dotnet = Get-FirstCommandPath @("dotnet.exe")
    if (-not $dotnet) { return $false }
    $sdks = & $dotnet "--list-sdks" 2>$null
    return [bool]($sdks | Where-Object { $_ -match ("^" + [Regex]::Escape($Version) + " ") })
}

function Install-PinnedDotnetSdk {
    param([string]$RepoRoot)

    $sdk = (Get-Content -LiteralPath (Join-Path $RepoRoot "global.json") -Raw -Encoding UTF8 | ConvertFrom-Json).sdk.version
    $dotnetRoot = Join-Path $env:LOCALAPPDATA "Microsoft\dotnet"
    if (-not (Test-DotnetSdkInstalled -Version $sdk)) {
        $asset = Get-PinnedAsset -Tool "dotnet-install-powershell"
        $scratch = New-ScratchDirectory
        try {
            $installer = Join-Path $scratch "dotnet-install.ps1"
            Save-VerifiedDownload -Url $asset.url -Sha256 $asset.sha256 -Destination $installer
            Write-Step "Installing .NET SDK $sdk..."
            & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $installer -Version $sdk -InstallDir $dotnetRoot -NoPath | Out-Host
            if ($LASTEXITCODE -ne 0) { throw "dotnet-install failed with exit code $LASTEXITCODE" }
        } finally {
            Remove-Item -LiteralPath $scratch -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    return $dotnetRoot
}

function Ensure-Rustup {
    if (Get-FirstCommandPath @((Join-HomePath @(".cargo", "bin", "rustup.exe")), "rustup.exe")) { return }
    $rustup = Get-PinnedAsset -Tool "rustup"
    $scratch = New-ScratchDirectory
    try {
        $installer = Join-Path $scratch "rustup-init.exe"
        Save-VerifiedDownload -Url $rustup.url -Sha256 $rustup.sha256 -Destination $installer
        Write-Step "Installing rustup $($rustup.version)..."
        & $installer -y --no-modify-path --profile minimal --default-toolchain none | Out-Host
        if ($LASTEXITCODE -ne 0) { throw "rustup-init failed with exit code $LASTEXITCODE" }
    } finally {
        Remove-ScratchDirectory -Path $scratch
    }
}

function Ensure-RustToolchain {
    param([string]$repoRoot)
    $rustupPath = Get-CommandPathOrThrow -Label "rustup" -Candidates @((Join-HomePath @(".cargo", "bin", "rustup.exe")), "rustup.exe")
    $previousRustupToolchain = $env:RUSTUP_TOOLCHAIN
    Remove-Item Env:RUSTUP_TOOLCHAIN -ErrorAction SilentlyContinue
    try {
        Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("toolchain", "install", "--no-self-update") -WorkingDirectory $repoRoot
    } finally {
        if ($null -ne $previousRustupToolchain) { $env:RUSTUP_TOOLCHAIN = $previousRustupToolchain }
    }
}

function Get-MsvcInstallation {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path -LiteralPath $vswhere -PathType Leaf)) { return $null }
    return (& $vswhere -latest -products '*' -property installationPath | Select-Object -First 1)
}

function Test-MsvcLinker {
    param([string]$Path)
    if (-not $Path) { return $false }
    $tools = Join-Path $Path "VC\Tools\MSVC"
    $architecture = Get-HostArchitecture
    foreach ($version in (Get-ChildItem -LiteralPath $tools -Directory -ErrorAction SilentlyContinue)) {
        if (Test-Path -LiteralPath (Join-Path $version.FullName "bin\Host$architecture\$architecture\link.exe") -PathType Leaf) { return $true }
    }
    return $false
}

function Get-CppBootstrap {
    return (Get-Content -LiteralPath $script:PinsFile -Raw -Encoding UTF8 | ConvertFrom-Json).definitions.MsvcBootstrap.const
}

function Install-CppBuildTools {
    $plan = Get-CppBootstrap
    $scratch = New-ScratchDirectory
    try {
        $installer = Join-Path $scratch $plan.file
        Save-VerifiedDownload -Url $plan.url -Sha256 $plan.sha256 -Destination $installer
        Write-Step "Installing C++ Build Tools bootstrapper release $($plan.release)..."
        $result = Start-Process -FilePath $installer -ArgumentList @("--wait", "--quiet", "--norestart", "--add", $plan.workload, "--includeRecommended") -WorkingDirectory $repoRoot -Wait -PassThru -WindowStyle Hidden
        if ($result.ExitCode -ne 0 -and $result.ExitCode -ne 3010) { throw "C++ Build Tools bootstrapper failed with exit code $($result.ExitCode)" }
    } finally {
        Remove-ScratchDirectory -Path $scratch
    }
}

function Ensure-CppBuildTools {
    $installation = Get-MsvcInstallation
    if ($installation -and (Test-MsvcLinker -Path $installation)) { return }
    $plan = Get-CppBootstrap
    if ($installation) {
        $installer = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\setup.exe"
        if (-not (Test-Path -LiteralPath $installer -PathType Leaf)) { throw "Visual Studio Installer is missing for $installation" }
        $result = Start-Process -FilePath $installer -ArgumentList @("modify", "--installPath", ('"' + $installation + '"'), "--add", $plan.workload, "--includeRecommended", "--quiet", "--norestart") -Wait -PassThru -WindowStyle Hidden
        if ($result.ExitCode -ne 0 -and $result.ExitCode -ne 3010) { throw "C++ Build Tools workload installation failed with exit code $($result.ExitCode)" }
    } else {
        Install-CppBuildTools
    }
    if (-not (Test-MsvcLinker -Path (Get-MsvcInstallation))) { throw "The required native MSVC linker is missing after C++ Build Tools setup" }
}

function Install-RustProfile {
    param([string]$repoRoot)
    Ensure-CppBuildTools
    Ensure-Rustup
    Ensure-RustToolchain -repoRoot $repoRoot
}

function Install-PinnedToolchain {
    param([string]$RepoRoot)

    $manifest = Get-Content -LiteralPath (Join-Path $RepoRoot "package.json") -Raw -Encoding UTF8 | ConvertFrom-Json
    $toolsRoot = Join-Path $env:LOCALAPPDATA "semio"
    $paths = @()

    $node = Get-PinnedAsset -Tool "node"
    if ($manifest.engines.node -ne $node.version) {
        throw "The Node.js pin $($node.version) in $script:PinsFile differs from package.json engines.node $($manifest.engines.node); update the pin."
    }
    $nodeRoot = Join-Path $toolsRoot "node\v$($node.version)"
    if ((Get-InstalledVersion "node.exe") -ne $node.version -or -not (Get-FirstCommandPath @("npm.cmd", "npm.exe"))) {
        $npmInRoot = Join-Path $nodeRoot "npm.cmd"
        if ((Test-Path -LiteralPath $nodeRoot) -and -not (Test-Path -LiteralPath $npmInRoot)) {
            Remove-Item -LiteralPath $nodeRoot -Recurse -Force
        }
        Write-Step "Installing Node.js $($node.version)..."
        Install-PinnedDirectory -Asset $node -Destination $nodeRoot -Child "node-v$($node.version)-win-$(Get-HostArchitecture)"
    }
    if (Test-Path -LiteralPath $nodeRoot) { $paths += $nodeRoot }
    else {
        $nodeFound = Get-FirstCommandPath @("node.exe")
        if ($nodeFound -and (Get-InstalledVersion "node.exe") -eq $node.version) { $paths += (Split-Path -Parent $nodeFound) }
    }

    $uv = Get-PinnedAsset -Tool "uv"
    $installedUv = Get-InstalledVersion "uv.exe"
    $uvBin = Join-HomePath @(".local", "bin")
    if ($null -eq $installedUv -or [version]$installedUv -lt [version]$uv.version -or -not (Get-FirstCommandPath @("uvx.exe"))) {
        Write-Step "Installing uv $($uv.version)..."
        Install-PinnedExecutables -Asset $uv -Destination $uvBin -FileNames @("uv.exe", "uvx.exe")
    }
    if (-not (Test-Path -LiteralPath (Join-Path $uvBin "uv.exe")) -or -not (Test-Path -LiteralPath (Join-Path $uvBin "uvx.exe"))) {
        foreach ($uvName in @("uv.exe", "uvx.exe")) {
            $uvFound = Get-FirstCommandPath @($uvName)
            if ($uvFound) { $paths += (Split-Path -Parent $uvFound) }
        }
    }

    Ensure-Rustup
    $paths += (Install-PinnedDotnetSdk -RepoRoot $RepoRoot)
    return $paths
}

function Install-UvTool {
    param(
        [string]$ToolName,
        [string]$UvPath
    )

    $toolPath = Join-HomePath @(".local", "bin", "$ToolName.exe")
    $arguments = @("tool", "install", "--upgrade", $ToolName)
    if (Test-Path -LiteralPath $toolPath) {
        $arguments = @("tool", "upgrade", $ToolName)
    }

    Invoke-RepoCommand -FilePath $UvPath -ArgumentList $arguments -WorkingDirectory (Get-RepoRoot)
}

function Test-WindowsLongPathsEnabled {
    $setting = Get-ItemProperty -LiteralPath "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -ErrorAction SilentlyContinue
    return ($null -ne $setting) -and ($setting.LongPathsEnabled -eq 1)
}

function Write-WindowsLongPathsGuidance {
    if (Test-WindowsLongPathsEnabled) {
        Write-Step "Windows long paths (LongPathsEnabled) are enabled."
        return
    }
    $enable = 'New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force'
    Write-Warning "[en] Windows long paths are disabled. This repository's fixtures and Rust build outputs reach 240+ characters below the clone root, and Python, CMake and every other tool that relies on this switch fail past 260. Run once in an elevated PowerShell, then open a new terminal: $enable"
    Write-Warning "[de] Lange Windows-Pfade sind deaktiviert. Fixtures und Rust-Build-Ausgaben dieses Repositorys erreichen unterhalb des Klon-Verzeichnisses 240+ Zeichen, und Python, CMake und jedes andere Werkzeug, das auf diesen Schalter angewiesen ist, scheitern ab 260. Einmal in einer PowerShell mit Administratorrechten ausführen und danach ein neues Terminal öffnen: $enable"
}

function Set-RepoLocalGitConfig {
    param([string]$RepoRoot)

    $gitPath = Get-FirstCommandPath @("git.exe", "git")
    if (-not $gitPath) {
        Write-Step "git is not on PATH yet; workspace:setup applies the repo-local git settings."
        return
    }
    foreach ($setting in @(@("core.symlinks", "true"), @("core.longpaths", "true"))) {
        & $gitPath -C $RepoRoot config --local $setting[0] $setting[1]
        if ($LASTEXITCODE -ne 0) { throw "git config --local $($setting[0]) failed" }
    }
}

function Write-WindowsReadinessFacts {
    param([string]$RepoRoot)

    $gitPath = Get-FirstCommandPath @("git.exe", "git")
    $gitLongPaths = if ($gitPath) { (& $gitPath -C $RepoRoot config --local --get core.longpaths 2>$null | Out-String).Trim() } else { "" }
    Write-Step ("[readiness] LongPathsEnabled={0} core.longpaths={1} elevated={2} rootLength={3}" -f (Test-WindowsLongPathsEnabled), $gitLongPaths, (Test-ElevatedSession), $RepoRoot.Length)
}

function Stop-RepoPythonProcesses {
    param([string]$RepoRoot, [switch]$Consent)

    $repoVenvRoots = @(
        (Join-Path $RepoRoot ".venv"),
        (Join-Path $RepoRoot "coda\assistant\.venv")
    ) | Where-Object { Test-Path -LiteralPath $_ } | ForEach-Object { (Resolve-Path $_).Path.TrimEnd("\") + "\" }

    if ($repoVenvRoots.Count -eq 0) {
        return
    }

    $pythonProcesses = Get-CimInstance Win32_Process -Filter "Name = 'python.exe' OR Name = 'pythonw.exe'" -ErrorAction SilentlyContinue
    foreach ($process in $pythonProcesses) {
        $executablePath = $process.ExecutablePath
        if (-not $executablePath) {
            continue
        }

        foreach ($venvRoot in $repoVenvRoots) {
            if (-not $executablePath.StartsWith($venvRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
                continue
            }
            if (-not $Consent) {
                Write-Warning "Repo-local Python process $($process.ProcessId) ($executablePath) locks the virtual environment. Close it, or rerun the bootstrap with -StopRepoPython to stop exactly the processes running from $venvRoot."
                break
            }
            Write-Step "Stopping repo-local Python process $($process.ProcessId) from $executablePath to refresh the virtual environment."
            Stop-Process -Id $process.ProcessId -ErrorAction Stop
            break
        }
    }
}

#endregion 🔧️Helpers

$repoRoot = Get-RepoRoot
Set-Location $repoRoot
$script:PinsFile = Join-Path $repoRoot "🧰️framework\🛍️products\🦑️repo\🔨️modules\🔩️native\🥾️bootstrap\🧬️schema\🔣️.json"
$script:LauncherInstall = Join-Path $repoRoot "🧰️framework\🛍️products\🦑️repo\🔨️modules\🚀️launcher\📥️install\🔵️.ps1"
$script:LauncherBin = Join-Path $env:LOCALAPPDATA "semio\launcher\bin"
if ($Command -eq "rust") {
    $env:SEMIO_WORKSPACE_ROOT = $repoRoot
    Refresh-CurrentProcessPath
    Install-RustProfile -repoRoot $repoRoot
    Set-UserPathPriority -PreferredEntries @((Join-HomePath @(".cargo", "bin")))
    Write-Step "Pinned Rust and the native C++ toolchain are ready."
    exit 0
}
Set-RepoLocalGitConfig -RepoRoot $repoRoot
Write-WindowsLongPathsGuidance
Write-WindowsReadinessFacts -RepoRoot $repoRoot
$env:SEMIO_WORKSPACE_ROOT = $repoRoot
Refresh-CurrentProcessPath

#region 🧰️MachineInstall
if (-not $SkipMachineInstall) {
    Sync-WingetPackage -Id "Git.Git" -Label "Git"
    Sync-WingetPackage -Id "GitHub.GitLFS" -Label "Git LFS"
    Sync-WingetPackage -Id "GitHub.cli" -Label "GitHub CLI"
    Sync-WingetPackage -Id "BurntSushi.ripgrep.MSVC" -Label "ripgrep"
    Sync-WingetPackage -Id "jqlang.jq" -Label "jq"
    Sync-WingetPackage -Id "SQLite.SQLite" -Label "SQLite"
    Sync-WingetPackage -Id "Python.Python.3.14" -Label "Python 3.14"
    $debuggerPolicyPath = Join-Path $repoRoot "🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\🛠️developer-tools\🧬️schema\🔣️.json"
    $debuggerPolicy = (Get-Content -LiteralPath $debuggerPolicyPath -Raw | ConvertFrom-Json).'$defs'.NativeDebuggerSetupPolicy.const.windows
    Sync-WingetPackage -Id $debuggerPolicy.pythonPackage -Label "LLDB Python Runtime" -AdditionalArguments @("--version", $debuggerPolicy.pythonVersion, "--scope", "user", "--location", (Join-Path $env:LOCALAPPDATA $debuggerPolicy.pythonDirectory), "--override", $debuggerPolicy.pythonOptions.Replace("{pythonRoot}", (Join-Path $env:LOCALAPPDATA $debuggerPolicy.pythonDirectory)))
    Sync-WingetPackage -Id $debuggerPolicy.llvmPackage -Label "LLVM Native Debugger" -AdditionalArguments @("--version", $debuggerPolicy.llvmVersion, "--scope", "machine", "--location", (Join-Path $env:ProgramFiles $debuggerPolicy.llvmDirectory))
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.9" -Label ".NET SDK 9.0"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.10" -Label ".NET SDK 10.0"
    Ensure-CppBuildTools
    Sync-WingetPackage -Id "f3d-app.f3d" -Label "F3D"
    $pinnedPaths = Install-PinnedToolchain -RepoRoot $repoRoot
    Set-UserPathPriority -PreferredEntries (@($pinnedPaths) + @(
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Python314\Scripts"),
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Python314"),
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Launcher"),
        (Join-Path $env:LOCALAPPDATA "Microsoft\WinGet\Links"),
        (Join-HomePath @(".local", "bin")),
        (Join-HomePath @(".cargo", "bin"))
    ))
    Refresh-CurrentProcessPath
    Set-RepoLocalGitConfig -RepoRoot $repoRoot
}
#endregion 🧰️MachineInstall

#region 🗂️UserState
@(
    (Join-HomePath @(".claude")),
    (Join-HomePath @(".codex")),
    (Join-HomePath @(".config", "gh")),
    (Join-HomePath @(".config", "openai")),
    (Join-HomePath @(".cargo")),
    (Join-HomePath @(".config", "F3D"))
) | ForEach-Object { Ensure-Directory -Path $_ }

$playwrightPath = Join-Path $repoRoot ".🧬semio\🦑️repo\⚡️cache\tools\ms-playwright"
Ensure-Directory -Path $playwrightPath
Set-UserEnvironmentVariable -Name "DEVCONTAINER" -Value "false"
Set-UserEnvironmentVariable -Name "VCPKG_ROOT" -Value (Join-Path $repoRoot ".🧬semio\🦑️repo\⚡️cache\vcpkg")
Set-UserEnvironmentVariable -Name "CMAKE_PRESET" -Value "windows-prototype"
Set-UserEnvironmentVariable -Name "DOTNET_CLI_TELEMETRY_OPTOUT" -Value "1"
Set-UserEnvironmentVariable -Name "PLAYWRIGHT_BROWSERS_PATH" -Value $playwrightPath
#endregion 🗂️UserState

#region 🌐️GlobalCliInstall
if (-not $SkipGlobalCliInstall) {
    Refresh-CurrentProcessPath
    $uvPath = Get-CommandPathOrThrow -Label "uv" -Candidates @("uv.exe")

    Install-UvTool -ToolName "ruff" -UvPath $uvPath
    Ensure-RustToolchain -repoRoot $repoRoot
}
#endregion 🌐️GlobalCliInstall

#region 🧱️RepoBootstrap
if (-not $SkipRepoBootstrap) {
    Refresh-CurrentProcessPath

    Stop-RepoPythonProcesses -RepoRoot $repoRoot -Consent:$StopRepoPython
    Invoke-RepoCommand -FilePath (Get-Command "powershell.exe").Source -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $script:LauncherInstall, "-Destination", $script:LauncherBin) -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath (Join-Path $script:LauncherBin "repo.exe") -ArgumentList @("run", "repo-mcp:install") -WorkingDirectory $repoRoot
    $repoReceipt = Get-Content -Raw -LiteralPath (Join-Path $env:LOCALAPPDATA "Semio\repo\installed.json") | ConvertFrom-Json
    if ($repoReceipt.version -ne 1 -or $repoReceipt.platform -ne "win32" -or -not (Test-Path -LiteralPath $repoReceipt.executable -PathType Leaf)) { throw "Verified native repo installation receipt required." }
    $goDirectory = Get-PinnedGoDirectory -RepoRoot $repoRoot
    Set-UserPathPriority -PreferredEntries @($repoReceipt.directory, $goDirectory)
    Refresh-CurrentProcessPath
    $repoPath = Get-CommandPathOrThrow -Label "repo" -Candidates @("repo.exe")
    if ([IO.Path]::GetFullPath($repoPath) -ine [IO.Path]::GetFullPath($repoReceipt.executable)) { throw "Current process PATH must select the installed native repository command." }
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("--help") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("dependencies", "sync", "--all") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("run", "workspace:setup") -WorkingDirectory $repoRoot
    try {
        Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("run", "@semio-tech/repo-developer-tools:setup-debugger") -WorkingDirectory $repoRoot
    } catch {
        Write-Step "Native debugger setup needs noninteractive install privileges or a working LLDB already on PATH. Setup continues."
    }
}
#endregion 🧱️RepoBootstrap



#region 🎬️Playwright
if (-not $SkipPlaywrightInstall) {
    Refresh-CurrentProcessPath
    $playwrightCli = Join-Path $repoRoot "node_modules\playwright\cli.js"
    if (Test-Path -LiteralPath $playwrightCli -PathType Leaf) {
        $nodePath = Get-CommandPathOrThrow -Label "node" -Candidates @("node.exe")
        Invoke-RepoCommand -FilePath $nodePath -ArgumentList @($playwrightCli, "install", "chromium") -WorkingDirectory $repoRoot
    } else {
        Write-Step "Playwright is not installed; skipping the Chromium browser."
    }
}
#endregion 🎬️Playwright

Write-WindowsLongPathsGuidance
Write-Step "Native bootstrap complete. Open a new shell to pick up the persisted PATH/env vars."
