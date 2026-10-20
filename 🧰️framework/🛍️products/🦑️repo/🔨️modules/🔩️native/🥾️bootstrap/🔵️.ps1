#region 🧲️Header
#
# 2026 Ueli Saluz <ueli@semio-tech.com>
#
# Specs: Zero-touch Windows-native bootstrap that installs Bun, Node.js, Go, uv and rustup from checksum-pinned archives (repo pins), installs the remaining machine dependencies with winget when missing, prepares repo-local caches and env vars, applies repo-local git settings (core.longpaths) and syncs workspace dependencies. Needs no elevation: the cache partitions are directories with per-directory NTFS case sensitivity.
#
# Summary: Windows-native bootstrap for the compose monorepo. Invoke from a fresh clone with Windows PowerShell 5.1 or PowerShell 7:
# `powershell -NoProfile -ExecutionPolicy Bypass -File "🧰️framework\🛍️products\🦑️repo\🔨️modules\🔩️native\🥾️bootstrap\🔵️.ps1" setup`
# `bun ./📜️script.ts setup native` routes here once Bun exists. Saved as UTF-8 with BOM: Windows PowerShell 5.1
# decodes a BOM-less script with the ANSI code page and would turn every emoji path literal below into a different path.
#
#endregion 🧲️Header

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet("setup")]
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
        [string]$WorkingDirectory
    )

    Write-Step ("Running: {0} {1}" -f $FilePath, ($ArgumentList -join " "))
    Push-Location $WorkingDirectory
    try {
        & $FilePath @ArgumentList
        if ($LASTEXITCODE -ne 0) {
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

function Get-PinnedBunAsset {
    $release = (Get-Content -LiteralPath $script:BunPinsFile -Raw -Encoding UTF8 | ConvertFrom-Json).definitions.Release.const
    $architecture = Get-HostArchitecture
    $row = $release.distributions | Where-Object { $_.platform -eq "win32" -and $_.arch -eq $architecture -and -not $_.musl } | Select-Object -First 1
    if ($null -eq $row) {
        throw "No pinned Bun for win32/$architecture in $script:BunPinsFile."
    }
    return [pscustomobject]@{ version = $release.version; url = "$($release.release)/$($row.archive)"; sha256 = $row.sha256 }
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
    $actual = (Get-FileHash -LiteralPath $Destination -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne $Sha256) {
        Remove-Item -LiteralPath $Destination -Force
        throw "Checksum mismatch for $Url (expected $Sha256, got $actual)."
    }
}

function New-ScratchDirectory {
    $path = Join-Path ([IO.Path]::GetTempPath()) ("semio-" + [Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Path $path -Force | Out-Null
    return $path
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

function Install-PinnedToolchain {
    param([string]$RepoRoot)

    $manifest = Get-Content -LiteralPath (Join-Path $RepoRoot "package.json") -Raw -Encoding UTF8 | ConvertFrom-Json
    $toolsRoot = Join-Path $env:LOCALAPPDATA "semio"
    $paths = @()

    $bun = Get-PinnedBunAsset
    if ($manifest.packageManager -ne "bun@$($bun.version)") {
        throw "The Bun pin $($bun.version) in $script:BunPinsFile differs from package.json packageManager $($manifest.packageManager); update both."
    }
    $bunBin = Join-HomePath @(".bun", "bin")
    if ((Get-InstalledVersion "bun.exe") -ne $bun.version) {
        Write-Step "Installing Bun $($bun.version)..."
        Install-PinnedExecutables -Asset $bun -Destination $bunBin -FileNames @("bun.exe")
    }
    $paths += $bunBin

    $node = Get-PinnedAsset -Tool "node"
    if ($manifest.engines.node -ne $node.version) {
        throw "The Node.js pin $($node.version) in $script:PinsFile differs from package.json engines.node $($manifest.engines.node); update the pin."
    }
    $nodeRoot = Join-Path $toolsRoot "node\v$($node.version)"
    if ((Get-InstalledVersion "node.exe") -ne $node.version) {
        Write-Step "Installing Node.js $($node.version)..."
        Install-PinnedDirectory -Asset $node -Destination $nodeRoot -Child "node-v$($node.version)-win-$(Get-HostArchitecture)"
    }
    if (Test-Path -LiteralPath $nodeRoot) { $paths += $nodeRoot }

    $go = Get-PinnedAsset -Tool "go"
    $goRoot = Join-Path $toolsRoot "go\go$($go.version)"
    $installedGo = Get-InstalledVersion "go.exe"
    if ($null -eq $installedGo -or [version]$installedGo -lt [version]$go.version) {
        Write-Step "Installing Go $($go.version)..."
        Install-PinnedDirectory -Asset $go -Destination $goRoot -Child "go"
    }
    if (Test-Path -LiteralPath $goRoot) { $paths += (Join-Path $goRoot "bin") }

    $uv = Get-PinnedAsset -Tool "uv"
    $installedUv = Get-InstalledVersion "uv.exe"
    $uvBin = Join-HomePath @(".local", "bin")
    if ($null -eq $installedUv -or [version]$installedUv -lt [version]$uv.version) {
        Write-Step "Installing uv $($uv.version)..."
        Install-PinnedExecutables -Asset $uv -Destination $uvBin -FileNames @("uv.exe", "uvx.exe")
    }

    if (-not (Get-FirstCommandPath @("rustup.exe"))) {
        $rustup = Get-PinnedAsset -Tool "rustup"
        $scratch = New-ScratchDirectory
        try {
            $installer = Join-Path $scratch "rustup-init.exe"
            Save-VerifiedDownload -Url $rustup.url -Sha256 $rustup.sha256 -Destination $installer
            Write-Step "Installing rustup $($rustup.version)..."
            & $installer -y --profile minimal --default-toolchain none | Out-Host
            if ($LASTEXITCODE -ne 0) { throw "rustup-init failed with exit code $LASTEXITCODE" }
        } finally {
            Remove-Item -LiteralPath $scratch -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
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
$script:BunPinsFile = Join-Path $repoRoot "🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\⚡️caching\🚀️bootstrap\🛠️tools\🟦️bun\🧬️schema\🔣️.json"
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
    Sync-WingetPackage -Id "Kitware.CMake" -Label "CMake"
    Sync-WingetPackage -Id "Ninja-build.Ninja" -Label "Ninja"
    Sync-WingetPackage -Id "Python.Python.3.14" -Label "Python 3.14"
    $debuggerPolicyPath = Join-Path $repoRoot "🧰️framework\🛍️products\🦑️repo\🔨️modules\💻️client\🛠️developer-tools\🧬️schema\🔣️.json"
    $debuggerPolicy = (Get-Content -LiteralPath $debuggerPolicyPath -Raw | ConvertFrom-Json).'$defs'.NativeDebuggerSetupPolicy.const.windows
    Sync-WingetPackage -Id $debuggerPolicy.pythonPackage -Label "LLDB Python Runtime" -AdditionalArguments @("--version", $debuggerPolicy.pythonVersion, "--scope", "user", "--location", (Join-Path $env:LOCALAPPDATA $debuggerPolicy.pythonDirectory), "--override", $debuggerPolicy.pythonOptions.Replace("{pythonRoot}", (Join-Path $env:LOCALAPPDATA $debuggerPolicy.pythonDirectory)))
    Sync-WingetPackage -Id $debuggerPolicy.llvmPackage -Label "LLVM Native Debugger" -AdditionalArguments @("--version", $debuggerPolicy.llvmVersion, "--scope", "machine", "--location", (Join-Path $env:ProgramFiles $debuggerPolicy.llvmDirectory))
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.8" -Label ".NET SDK 8.0"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.9" -Label ".NET SDK 9.0"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.10" -Label ".NET SDK 10.0"
    $msvcCppOverride = @("--override", "--wait --quiet --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended")
    Sync-WingetPackage -Id "Microsoft.VisualStudio.BuildTools" -Label "Microsoft C++ Build Tools" -AdditionalArguments $msvcCppOverride
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
Set-UserEnvironmentVariable -Name "CMAKE_PRESET" -Value "windows"
Set-UserEnvironmentVariable -Name "DOTNET_CLI_TELEMETRY_OPTOUT" -Value "1"
Set-UserEnvironmentVariable -Name "PLAYWRIGHT_BROWSERS_PATH" -Value $playwrightPath
#endregion 🗂️UserState

#region 🌐️GlobalCliInstall
if (-not $SkipGlobalCliInstall) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")
    $rustupPath = Get-CommandPathOrThrow -Label "rustup" -Candidates @("rustup.exe")
    $uvPath = Get-CommandPathOrThrow -Label "uv" -Candidates @("uv.exe")

    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("add", "--global", "@google/gemini-cli", "typescript-language-server", "typescript", "pyright") -WorkingDirectory $repoRoot
    Install-UvTool -ToolName "ruff" -UvPath $uvPath
    Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("toolchain", "install") -WorkingDirectory $repoRoot
}
#endregion 🌐️GlobalCliInstall

#region 🧱️RepoBootstrap
if (-not $SkipRepoBootstrap) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")

    Stop-RepoPythonProcesses -RepoRoot $repoRoot -Consent:$StopRepoPython
    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("repo", "run", "repo-mcp:install") -WorkingDirectory $repoRoot
    $repoReceipt = Get-Content -Raw -LiteralPath (Join-Path $env:LOCALAPPDATA "Semio\repo\installed.json") | ConvertFrom-Json
    if ($repoReceipt.version -ne 1 -or $repoReceipt.platform -ne "win32" -or -not (Test-Path -LiteralPath $repoReceipt.executable -PathType Leaf)) { throw "Verified native repo installation receipt required." }
    Set-UserPathPriority -PreferredEntries @($repoReceipt.directory)
    Refresh-CurrentProcessPath
    $repoPath = Get-CommandPathOrThrow -Label "repo" -Candidates @("repo.exe")
    if ([IO.Path]::GetFullPath($repoPath) -ine [IO.Path]::GetFullPath($repoReceipt.executable)) { throw "Current process PATH must select the installed native repository command." }
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("--help") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("run", "workspace:setup") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $repoPath -ArgumentList @("run", "@semio-tech/repo-developer-tools:setup-debugger") -WorkingDirectory $repoRoot
}
#endregion 🧱️RepoBootstrap



#region 🎬️Playwright
if (-not $SkipPlaywrightInstall) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")
    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("x", "playwright", "install", "chromium") -WorkingDirectory $repoRoot
}
#endregion 🎬️Playwright

Write-WindowsLongPathsGuidance
Write-Step "Native bootstrap complete. Open a new shell to pick up the persisted PATH/env vars."
