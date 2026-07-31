#region 🧲Header
#
# 2026 Ueli Saluz <ueli@semio-tech.com>
#
# Specs: Zero-touch Windows-native bootstrap that upgrades machine dependencies to the current supported baseline with winget, prepares repo-local caches and env vars, syncs workspace dependencies, verifies a user-created native Neo4j Desktop compose DBMS, and installs the local VS Code extension when editor CLIs are available.
#
# Summary: Windows-native bootstrap for the compose monorepo. Invoke `.\⌨️script.ps1 setup` (full) or `.\⌨️script.ps1 start` (IDE session).
#
#endregion 🧲Header

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet("setup", "start")]
    [string]$Command = "setup",
    [switch]$SessionStart,
    [switch]$SkipMachineInstall,
    [switch]$SkipGlobalCliInstall,
    [switch]$SkipEditorInstall,
    [switch]$SkipPlaywrightInstall,
    [switch]$SkipRepoBootstrap
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$sessionStartEffective = $SessionStart.IsPresent -or ($Command -eq "start")

#region 🎯Targets
$script:PythonKind = "3.14"
$script:Neo4jVersion = "5.26.26"
$script:ApocVersion = "5.26.4"
#endregion 🎯Targets

#region 🔧Helpers
function Write-Step {
    param([string]$Message)
    Write-Host "[compose] $Message"
}

function Get-RepoRoot {
    if ($env:COMPOSE_REPO_ROOT) {
        return (Resolve-Path $env:COMPOSE_REPO_ROOT).Path
    }
    $dir = $PSScriptRoot
    for ($i = 0; $i -lt 32; $i++) {
        if (Test-Path -LiteralPath (Join-Path $dir "nx.json")) {
            return (Resolve-Path $dir).Path
        }
        $parent = Split-Path $dir -Parent
        if ($parent -eq $dir) { break }
        $dir = $parent
    }
    return (Resolve-Path $PSScriptRoot).Path
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

function Set-TextFileUtf8NoBom {
    param(
        [string]$Path,
        [string[]]$Value
    )

    $encoding = [System.Text.UTF8Encoding]::new($false)
    [System.IO.File]::WriteAllLines($Path, $Value, $encoding)
}

function Refresh-CurrentProcessPath {
    $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $extraPaths = @(
        (Join-HomePath @(".cargo", "bin")),
        (Join-HomePath @(".local", "bin")),
        (Join-Path $env:LOCALAPPDATA "GitKrakenCLI"),
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

    $baseArguments = @(
        "--exact",
        "--id",
        $Id,
        "--accept-package-agreements",
        "--accept-source-agreements",
        "--disable-interactivity",
        "--silent"
    ) + $AdditionalArguments

    if (Test-WingetPackageInstalled -Id $Id) {
        Write-Step "Upgrading $Label ($Id) to the latest stable release..."
        & winget upgrade @baseArguments
        if ($LASTEXITCODE -eq 0) {
            return
        }

        $upgradeOutput = (& winget upgrade --exact --id $Id --accept-source-agreements --disable-interactivity 2>&1 | Out-String)
        if ($upgradeOutput -match "No available upgrade found" -or $upgradeOutput -match "No newer package versions are available") {
            Write-Step "$Label already on the latest stable release."
            return
        }
    }

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

function Install-EditorExtensions {
    param(
        [string]$RepoRoot,
        [string[]]$EditorCliPaths
    )

    if ($EditorCliPaths.Count -eq 0) {
        Write-Step "No editor CLI detected; skipping extension install."
        return
    }

    $extensionsPath = Join-Path $RepoRoot ".vscode\extensions.json"
    $recommendations = (Get-Content $extensionsPath -Raw | ConvertFrom-Json).recommendations
    $vsixPath = Join-Path $RepoRoot "repo\vscode\🧩repo.vsix"

    $bunPathLocal = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")
    Invoke-RepoCommand -FilePath $bunPathLocal -ArgumentList @("nx", "run", "repo:build") -WorkingDirectory $RepoRoot
    Invoke-RepoCommand -FilePath $bunPathLocal -ArgumentList @("nx", "run", "repo:build-vsix") -WorkingDirectory $RepoRoot

    foreach ($editorCli in $EditorCliPaths) {
        foreach ($extension in $recommendations) {
            & $editorCli --install-extension $extension --force | Out-Null
        }
        & $editorCli --install-extension $vsixPath --force | Out-Null
        Write-Step "Installed workspace extensions via $editorCli."
    }
}

function Configure-GitSafeDirectories {
    param([string]$RepoRoot)

    & git config --global --add safe.directory $RepoRoot | Out-Null
    $gitmodulesPath = Join-Path $RepoRoot ".gitmodules"
    if (-not (Test-Path -LiteralPath $gitmodulesPath)) {
        return
    }

    $submodulePaths = & git config -f $gitmodulesPath --get-regexp '^submodule\..*\.path$' 2>$null
    foreach ($line in $submodulePaths) {
        $parts = $line -split "\s+", 2
        if ($parts.Count -eq 2 -and $parts[1]) {
            & git config --global --add safe.directory (Join-Path $RepoRoot $parts[1]) | Out-Null
        }
    }
}

function Configure-GitKrakenWorkspace {
    param([string]$RepoRoot)

    $gkPath = Get-FirstCommandPath @((Join-Path $env:LOCALAPPDATA "GitKrakenCLI\gk.exe"), "gk.exe", "gk")
    if (-not $gkPath) {
        Write-Step "GitKraken CLI not on PATH yet; skipping workspace bootstrap."
        return
    }

    & $gkPath auth status *> $null
    if ($LASTEXITCODE -ne 0) {
        Write-Step "GitKraken CLI is not authenticated; skipping workspace bootstrap."
        return
    }

    $workspaceName = $env:SEMIO_GITKRAKEN_WORKSPACE_NAME
    $repos = [System.Collections.Generic.List[string]]::new()
    $repos.Add($RepoRoot)

    $gitmodulesPath = Join-Path $RepoRoot ".gitmodules"
    if (Test-Path -LiteralPath $gitmodulesPath) {
        $submodulePaths = & git config -f $gitmodulesPath --get-regexp '^submodule\..*\.path$' 2>$null
        foreach ($line in $submodulePaths) {
            $parts = $line -split "\s+", 2
            if ($parts.Count -eq 2 -and $parts[1]) {
                $repos.Add((Join-Path $RepoRoot $parts[1]))
            }
        }
    }

    $repoCsv = ($repos | Select-Object -Unique) -join ","
    $infoOutput = & $gkPath ws info $workspaceName 2>$null | Out-String
    if ($LASTEXITCODE -eq 0 -and $infoOutput -and -not ($infoOutput -match "no workspace with name")) {
        $missing = @($repos | Where-Object { $infoOutput -notmatch [Regex]::Escape($_) })
        if ($missing.Count -gt 0) {
            & $gkPath ws update $workspaceName --add-repos (($missing | Select-Object -Unique) -join ",") | Out-Null
            & $gkPath ws refresh $workspaceName | Out-Null
        }
    } else {
        & $gkPath ws create $workspaceName --add-repos $repoCsv | Out-Null
        & $gkPath ws refresh $workspaceName | Out-Null
    }
    & $gkPath ws set $workspaceName | Out-Null
    Write-Step "GitKraken workspace ready: $workspaceName."
}

function Stop-RepoPythonProcesses {
    param([string]$RepoRoot)

    $repoVenvRoots = @(
        (Join-Path $RepoRoot ".venv"),
        (Join-Path $RepoRoot "coda\assistant\.venv")
    ) | Where-Object { Test-Path -LiteralPath $_ } | ForEach-Object { (Resolve-Path $_).Path }

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
            if ($executablePath.StartsWith($venvRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
                Write-Step "Stopping repo-local Python process $($process.ProcessId) from $executablePath to refresh the virtual environment."
                Stop-Process -Id $process.ProcessId -Force -ErrorAction Stop
                break
            }
        }
    }
}

function Test-TcpPort {
    param(
        [string]$HostName,
        [int]$Port
    )

    $client = [System.Net.Sockets.TcpClient]::new()
    try {
        $async = $client.BeginConnect($HostName, $Port, $null, $null)
        if (-not $async.AsyncWaitHandle.WaitOne(1000)) {
            return $false
        }
        $client.EndConnect($async)
        return $true
    } catch {
        return $false
    } finally {
        $client.Close()
    }
}

function Invoke-Neo4jCypher {
    param(
        [string]$RepoRoot,
        [string]$Cypher,
        [string]$Database = "compose",
        [string]$RequiredOutputPattern = ""
    )

    Use-MicrosoftOpenJdk21
    $runtimeCypherShell = Join-Path $RepoRoot ".repo\cache\neo4j\neo4j-community-$($script:Neo4jVersion)\bin\cypher-shell.bat"
    $cypherShell = Get-FirstCommandPath @($runtimeCypherShell, "cypher-shell.cmd", "cypher-shell.exe", "cypher-shell")
    if ($cypherShell) {
        $logRoot = Join-Path $RepoRoot ".repo\cache\neo4j"
        Ensure-Directory -Path $logRoot
        $logId = "{0}-{1}" -f $PID, ([Guid]::NewGuid().ToString("N"))
        $stdout = Join-Path $logRoot "cypher-shell.$logId.stdout.log"
        $stderr = Join-Path $logRoot "cypher-shell.$logId.stderr.log"
        $arguments = @(
            "-a", "bolt://localhost:7687",
            "-u", "neo4j",
            "-p", "password",
            "-d", $Database,
            "--format", "plain",
            $Cypher
        )
        $previousErrorActionPreference = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        try {
            & $cypherShell @arguments > $stdout 2> $stderr
            if ($LASTEXITCODE -ne 0) {
                return $false
            }
            if ($RequiredOutputPattern) {
                $output = Get-Content -LiteralPath $stdout -Raw -ErrorAction SilentlyContinue
                return $output -match $RequiredOutputPattern
            }
            return $true
        } finally {
            $ErrorActionPreference = $previousErrorActionPreference
        }
    }

    return $false
}

function Get-Neo4jSchemaUri {
    param(
        [string]$RepoRoot,
        [string]$Technology
    )

    $rootUri = ($RepoRoot -replace "\\", "/")
    return "file:///$rootUri/.🦑repo/\uD83D\uDEC2/$Technology.cypher"
}

function Get-RunningNeo4jDesktopDbmsHome {
    $processes = Get-CimInstance Win32_Process | Where-Object {
        $_.CommandLine -and
        $_.CommandLine -match "--home-dir=" -and
        $_.CommandLine -match "\\.Neo4jDesktop2?\\Data\\dbmss\\"
    }
    foreach ($process in $processes) {
        if ($process.CommandLine -match '--home-dir="([^"]+)"') {
            return $Matches[1]
        }
        if ($process.CommandLine -match '--home-dir=([^\s]+)') {
            return $Matches[1]
        }
    }
    return $null
}

function Set-TextSetting {
    param(
        [string]$Path,
        [string]$Key,
        [string]$Value
    )

    $lines = @()
    if (Test-Path -LiteralPath $Path) {
        $lines = Get-Content -LiteralPath $Path
    }
    $pattern = "^[#\s]*$([Regex]::Escape($Key))="
    $matched = $false
    $lines = @($lines | ForEach-Object {
        if ($_ -match $pattern) {
            $matched = $true
            "$Key=$Value"
        } else {
            $_
        }
    })
    if (-not $matched) {
        $lines += "$Key=$Value"
    }
    Set-TextFileUtf8NoBom -Path $Path -Value $lines
}

function Install-Neo4jDesktopApoc {
    param([string]$RepoRoot)

    $dbmsHome = Get-RunningNeo4jDesktopDbmsHome
    if (-not $dbmsHome) {
        Write-Step "APOC auto-install skipped because the reachable DBMS is not a running Neo4j Desktop local DBMS."
        return $false
    }

    $plugins = Join-Path $dbmsHome "plugins"
    $labs = Join-Path $dbmsHome "labs"
    $conf = Join-Path $dbmsHome "conf"
    Ensure-Directory -Path $plugins
    $coreJar = Get-ChildItem -LiteralPath $labs -Filter "apoc-*-core.jar" -ErrorAction SilentlyContinue | Sort-Object Name -Descending | Select-Object -First 1
    if ($coreJar) {
        Copy-Item -LiteralPath $coreJar.FullName -Destination (Join-Path $plugins $coreJar.Name) -Force
        if ($coreJar.Name -match "apoc-(.+)-core\.jar") {
            $apocVersion = $Matches[1]
            $extendedJar = Join-Path $plugins ("apoc-extended-{0}.jar" -f $apocVersion)
            if (-not (Test-Path -LiteralPath $extendedJar)) {
                Invoke-WebRequest -Uri "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-extended/$apocVersion/apoc-extended-$apocVersion.jar" -OutFile $extendedJar
            }
        }
    }

    Set-TextSetting -Path (Join-Path $conf "neo4j.conf") -Key "dbms.security.procedures.allowlist" -Value "apoc.*"
    Set-TextSetting -Path (Join-Path $conf "neo4j.conf") -Key "dbms.security.procedures.unrestricted" -Value "apoc.*"
    Set-TextSetting -Path (Join-Path $conf "neo4j.conf") -Key "server.directories.import" -Value (($RepoRoot -replace "\\", "/"))
    Set-TextSetting -Path (Join-Path $conf "neo4j.conf") -Key "initial.dbms.default_database" -Value "compose"
    Set-TextSetting -Path (Join-Path $conf "apoc.conf") -Key "apoc.export.file.enabled" -Value "true"
    Set-TextSetting -Path (Join-Path $conf "apoc.conf") -Key "apoc.import.file.enabled" -Value "true"
    Set-TextSetting -Path (Join-Path $conf "apoc.conf") -Key "apoc.import.file.use_neo4j_config" -Value "false"

    $neo4jBat = Join-Path $dbmsHome "bin\neo4j.bat"
    if (Test-Path -LiteralPath $neo4jBat) {
        Write-Step "Restarting Neo4j Desktop local compose DBMS to load APOC..."
        Use-MicrosoftOpenJdk21
        $escapedHome = [Regex]::Escape($dbmsHome)
        $processes = Get-CimInstance Win32_Process | Where-Object {
            $_.Name -match "^java" -and $_.CommandLine -match $escapedHome
        }
        foreach ($process in $processes) {
            Stop-Process -Id $process.ProcessId -Force
        }
        for ($i = 0; $i -lt 30 -and (Test-TcpPort -HostName "127.0.0.1" -Port 7687); $i++) {
            Start-Sleep -Seconds 1
        }
        Start-Process -FilePath $neo4jBat -ArgumentList @("console") -WorkingDirectory $dbmsHome -WindowStyle Hidden | Out-Null
        for ($i = 0; $i -lt 45 -and -not (Test-TcpPort -HostName "127.0.0.1" -Port 7687); $i++) {
            Start-Sleep -Seconds 2
        }
    } else {
        Write-Step "APOC installed into the compose DBMS. Restart it in Neo4j Desktop to load the program."
    }

    return $true
}

function Resolve-NativeNeo4jGraphDatabase {
    param([string]$RepoRoot)

    $preferred = if ($env:NEO4J_DATABASE) { $env:NEO4J_DATABASE } else { "compose" }
    if (Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database $preferred -Cypher "RETURN 1 AS ok;") {
        return $preferred
    }
    return $preferred
}

function Get-Neo4jExtraBoltGraphNamesFromEnv {
    if ([string]::IsNullOrWhiteSpace($env:NEO4J_EXTRA_GRAPH_DATABASES)) { return @() }
    $env:NEO4J_EXTRA_GRAPH_DATABASES.Split(',', [StringSplitOptions]::RemoveEmptyEntries) | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }
}

function Format-Neo4jDatabaseNameForCypher {
    param([string]$Name)
    if ($Name -match '^[a-zA-Z_][a-zA-Z0-9_]*$') { return $Name }
    return ('`' + ($Name -replace '`', '``') + '`')
}

function Get-JavaMajorVersion {
    $java = Get-FirstCommandPath @(
        "C:\Program Files\Microsoft\jdk-21.0.11.10-hotspot\bin\java.exe",
        "java.exe",
        "java"
    )
    if (-not $java) {
        return 0
    }
    $logRoot = Join-Path (Get-RepoRoot) ".repo\cache\neo4j"
    Ensure-Directory -Path $logRoot
    $stdout = Join-Path $logRoot "java-version.stdout.log"
    $stderr = Join-Path $logRoot "java-version.stderr.log"
    Start-Process -FilePath $java -ArgumentList @("-version") -NoNewWindow -Wait -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr | Out-Null
    $versionText = ((Get-Content -LiteralPath $stdout -ErrorAction SilentlyContinue) + (Get-Content -LiteralPath $stderr -ErrorAction SilentlyContinue)) -join "`n"
    if ($versionText -match 'version\s+"(\d+)') {
        return [int]$Matches[1]
    }
    if ($versionText -match '(\d+)\.\d+\.\d+') {
        return [int]$Matches[1]
    }
    return 0
}

function Use-MicrosoftOpenJdk21 {
    $javaHome = "C:\Program Files\Microsoft\jdk-21.0.11.10-hotspot"
    if (Test-Path -LiteralPath (Join-Path $javaHome "bin\java.exe")) {
        $env:JAVA_HOME = $javaHome
        $env:Path = (Join-Path $javaHome "bin") + ";" + $env:Path
    }
}

function Ensure-NativeNeo4jTools {
    param([string]$RepoRoot)

    $cacheRoot = Join-Path $RepoRoot ".repo\cache\neo4j"
    $runtimeRoot = Join-Path $cacheRoot ("neo4j-community-{0}" -f $script:Neo4jVersion)
    $zipPath = Join-Path $cacheRoot ("neo4j-community-{0}-windows.zip" -f $script:Neo4jVersion)
    Ensure-Directory -Path $cacheRoot

    if (-not (Test-Path -LiteralPath $runtimeRoot)) {
        $url = "https://dist.neo4j.org/neo4j-community-$($script:Neo4jVersion)-windows.zip"
        Write-Step "Downloading Neo4j Community $($script:Neo4jVersion) tools for cypher-shell..."
        Invoke-WebRequest -Uri $url -OutFile $zipPath
        Write-Step "Extracting Neo4j tools..."
        Expand-Archive -LiteralPath $zipPath -DestinationPath $cacheRoot -Force
    }

    $plugins = Join-Path $runtimeRoot "plugins"
    Ensure-Directory -Path $plugins
    $apocCore = Join-Path $plugins ("apoc-core-{0}-core.jar" -f $script:ApocVersion)
    $apocExtended = Join-Path $plugins ("apoc-{0}-extended.jar" -f $script:ApocVersion)
    if (-not (Test-Path -LiteralPath $apocCore)) {
        Invoke-WebRequest -Uri "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-core/$($script:ApocVersion)/apoc-core-$($script:ApocVersion)-core.jar" -OutFile $apocCore
    }
    if (-not (Test-Path -LiteralPath $apocExtended)) {
        Invoke-WebRequest -Uri "https://github.com/neo4j-contrib/neo4j-apoc-procedures/releases/download/$($script:ApocVersion)/apoc-$($script:ApocVersion)-extended.jar" -OutFile $apocExtended
    }

    $conf = Join-Path $runtimeRoot "conf\neo4j.conf"
    Use-MicrosoftOpenJdk21
    $dataDir = (Join-Path $runtimeRoot "data") -replace "\\", "/"
    $logsDir = (Join-Path $runtimeRoot "logs") -replace "\\", "/"
    $importDir = $RepoRoot -replace "\\", "/"
    $settings = [ordered]@{
        "server.default_listen_address" = "127.0.0.1"
        "server.bolt.listen_address" = ":7687"
        "server.http.listen_address" = ":7474"
        "dbms.usage_report.enabled" = "false"
        "server.directories.data" = $dataDir
        "server.directories.logs" = $logsDir
        "server.directories.import" = $importDir
        "dbms.security.procedures.allowlist" = "apoc.*"
        "dbms.security.procedures.unrestricted" = "apoc.*"
    }
    $lines = @()
    if (Test-Path -LiteralPath $conf) {
        $lines = Get-Content -LiteralPath $conf
    }
    foreach ($key in $settings.Keys) {
        $value = $settings[$key]
        $pattern = "^[#\s]*$([Regex]::Escape($key))="
        $matched = $false
        $lines = @($lines | ForEach-Object {
            if ($_ -match $pattern) {
                $matched = $true
                "$key=$value"
            } else {
                $_
            }
        })
        if (-not $matched) {
            $lines += "$key=$value"
        }
    }
    Set-TextFileUtf8NoBom -Path $conf -Value $lines

    $apocConf = Join-Path $runtimeRoot "conf\apoc.conf"
    Set-TextFileUtf8NoBom -Path $apocConf -Value @(
        "apoc.export.file.enabled=true",
        "apoc.import.file.enabled=true",
        "apoc.import.file.use_neo4j_config=false"
    )

    return $runtimeRoot
}

function Ensure-NativeNeo4j {
    param([string]$RepoRoot)

    $technologies = @("compose", "elements", "coda", "reuse") + @(Get-Neo4jExtraBoltGraphNamesFromEnv)
    if (Test-TcpPort -HostName "127.0.0.1" -Port 7687) {
        Write-Step "Neo4j is reachable at bolt://localhost:7687."
    } else {
        Write-Step "Neo4j is not reachable. Create and start a native Neo4j Desktop local DBMS named compose on Bolt port 7687, password password, then run this setup again."
        return
    }

    Ensure-NativeNeo4jTools -RepoRoot $RepoRoot | Out-Null
    $graphDb = Resolve-NativeNeo4jGraphDatabase -RepoRoot $RepoRoot
    Write-Step "Neo4j graph database for imports: $graphDb"

    $apocReady = Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database $graphDb -Cypher "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" -RequiredOutputPattern "\b2\b"
    if (-not $apocReady) {
        if (Install-Neo4jDesktopApoc -RepoRoot $RepoRoot) {
            $graphDb = Resolve-NativeNeo4jGraphDatabase -RepoRoot $RepoRoot
            $apocReady = Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database $graphDb -Cypher "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" -RequiredOutputPattern "\b2\b"
        }
        if (-not $apocReady) {
            Write-Step "Neo4j is reachable, but APOC is not ready. In Neo4j Desktop, install/enable APOC for the local compose DBMS and restart it."
            return
        }
    }

    Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database "system" -Cypher "CREATE DATABASE compose IF NOT EXISTS;" | Out-Null
    foreach ($extraDb in (Get-Neo4jExtraBoltGraphNamesFromEnv)) {
        $q = Format-Neo4jDatabaseNameForCypher -Name $extraDb
        Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database "system" -Cypher "CREATE DATABASE $q IF NOT EXISTS;" | Out-Null
    }
    #region 🔥Neo4jEnterpriseDropStockDb
    # Enterprise Desktop: schema often lands in the stock `neo4j` DB. Ensure `compose` is default, then drop `neo4j`.
    # Community (single user DB): these calls fail harmlessly.
    Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database "system" -Cypher "START DATABASE compose WAIT;" | Out-Null
    Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database "system" -Cypher "CALL dbms.setDefaultDatabase('compose');" | Out-Null
    Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database "system" -Cypher "DROP DATABASE neo4j IF EXISTS CASCADE ALIASES WAIT;" | Out-Null
    #endregion 🔥Neo4jEnterpriseDropStockDb
    $graphDb = Resolve-NativeNeo4jGraphDatabase -RepoRoot $RepoRoot
    Write-Step "Neo4j graph database for imports (after optional CREATE DATABASE compose): $graphDb"

    Write-Step "Neo4j: clearing graph in $graphDb, then loading generated .🦑repo/🛂manifest/*.cypher (from `bun run generate`) …"
    Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database $graphDb -Cypher "MATCH (n) DETACH DELETE n;" | Out-Null

    foreach ($technology in $technologies) {
        $schemaFile = Join-Path $RepoRoot ".repo\🛂\$technology.cypher"
        if (Test-Path -LiteralPath $schemaFile) {
            $meaningfulLine = Get-Content -LiteralPath $schemaFile | Where-Object { $_ -notmatch '^\s*(//|:|$)' } | Select-Object -First 1
            if ($meaningfulLine) {
                $schemaUri = Get-Neo4jSchemaUri -RepoRoot $RepoRoot -Technology $technology
                Invoke-Neo4jCypher -RepoRoot $RepoRoot -Database $graphDb -Cypher "CALL apoc.cypher.runFile('$schemaUri') YIELD row RETURN count(row) AS rows;" | Out-Null
                Write-Step "Neo4j schema imported into ${graphDb}: $technology."
            }
        }
    }
    if (Get-Command bun -ErrorAction SilentlyContinue) {
        $savedDb = $env:NEO4J_DATABASE
        $env:NEO4J_DATABASE = $graphDb
        Push-Location $RepoRoot
        try {
            & bun "./📜script.ts" purge neo4j | Out-Null
            if ($LASTEXITCODE -ne 0) {
                Write-Step "Neo4j legacy-property prune exited with code $LASTEXITCODE (cypher-shell may be missing)."
            }
        } finally {
            Pop-Location
            if ($null -ne $savedDb) {
                $env:NEO4J_DATABASE = $savedDb
            } else {
                Remove-Item Env:\NEO4J_DATABASE -ErrorAction SilentlyContinue
            }
        }
    }
}
#endregion 🔧Helpers

if ($sessionStartEffective) {
    $SkipMachineInstall = $true
    $SkipGlobalCliInstall = $true
    $SkipEditorInstall = $true
    $SkipPlaywrightInstall = $true
    $SkipRepoBootstrap = $true
}

$repoRoot = Get-RepoRoot
Set-Location $repoRoot
$nxWorkspaceDataTerminal = Join-Path $repoRoot ".nx\workspace-data-terminal"
Ensure-Directory -Path $nxWorkspaceDataTerminal
$env:NX_WORKSPACE_DATA_DIRECTORY = $nxWorkspaceDataTerminal
Refresh-CurrentProcessPath

#region 🧰MachineInstall
if (-not $SkipMachineInstall) {
    Sync-WingetPackage -Id "Git.Git" -Label "Git"
    Sync-WingetPackage -Id "GitHub.GitLFS" -Label "Git LFS"
    Sync-WingetPackage -Id "GitHub.cli" -Label "GitHub CLI"
    Sync-WingetPackage -Id "BurntSushi.ripgrep.MSVC" -Label "ripgrep"
    Sync-WingetPackage -Id "jqlang.jq" -Label "jq"
    Sync-WingetPackage -Id "SQLite.SQLite" -Label "SQLite"
    Sync-WingetPackage -Id "Oven-sh.Bun" -Label "Bun"
    Sync-WingetPackage -Id "Kitware.CMake" -Label "CMake"
    Sync-WingetPackage -Id "Ninja-build.Ninja" -Label "Ninja"
    Sync-WingetPackage -Id "Microsoft.OpenJDK.21" -Label "Microsoft OpenJDK 21"
    Sync-WingetPackage -Id "GoLang.Go" -Label "Go"
    Sync-WingetPackage -Id "Python.Python.3.14" -Label "Python 3.14"
    Sync-WingetPackage -Id "astral-sh.uv" -Label "uv"
    Sync-WingetPackage -Id "Rustlang.Rustup" -Label "rustup"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.8" -Label ".NET SDK 8.0"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.9" -Label ".NET SDK 9.0"
    Sync-WingetPackage -Id "Microsoft.DotNet.SDK.10" -Label ".NET SDK 10.0"
    $vs2026CppOverride = @("--override", "--wait --quiet --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended")
    if (Test-WingetPackageInstalled -Id "Microsoft.VisualStudio.Community") {
        Sync-WingetPackage -Id "Microsoft.VisualStudio.Community" -Label "Visual Studio 2026 Community" -AdditionalArguments $vs2026CppOverride
    } else {
        Sync-WingetPackage -Id "Microsoft.VisualStudio.BuildTools" -Label "Visual Studio 2026 Build Tools" -AdditionalArguments $vs2026CppOverride
    }
    Sync-WingetPackage -Id "Axosoft.GitKraken" -Label "GitKraken Desktop"
    Sync-WingetPackage -Id "GitKraken.cli" -Label "GitKraken CLI"
    Sync-WingetPackage -Id "f3d-app.f3d" -Label "F3D"
    Sync-WingetPackage -Id "Microsoft.VisualStudioCode" -Label "VS Code"
    Sync-WingetPackage -Id "Microsoft.VisualStudioCode.CLI" -Label "VS Code CLI"
    Sync-WingetPackage -Id "Neo4j.Neo4jDesktop" -Label "Neo4j Desktop"
    Set-UserPathPriority -PreferredEntries @(
        "C:\Program Files\Microsoft\jdk-21.0.11.10-hotspot\bin",
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Python314\Scripts"),
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Python314"),
        (Join-Path $env:LOCALAPPDATA "Programs\Python\Launcher"),
        (Join-Path $env:LOCALAPPDATA "GitKrakenCLI"),
        (Join-Path $env:LOCALAPPDATA "Microsoft\WinGet\Links"),
        (Join-HomePath @(".local", "bin")),
        (Join-HomePath @(".cargo", "bin"))
    )
    Refresh-CurrentProcessPath
}
#endregion 🧰MachineInstall

#region 🗂️UserState
@(
    (Join-HomePath @(".claude")),
    (Join-HomePath @(".codex")),
    (Join-HomePath @(".config", "gh")),
    (Join-HomePath @(".config", "cursor")),
    (Join-HomePath @(".config", "antigravity")),
    (Join-HomePath @(".config", "openai")),
    (Join-HomePath @(".codeium", "windsurf")),
    (Join-HomePath @(".gitkraken")),
    (Join-HomePath @(".local", "share", "GitKrakenCLI")),
    (Join-HomePath @(".local", "share", "gk")),
    (Join-HomePath @(".cache", "ms-playwright")),
    (Join-HomePath @(".cargo")),
    (Join-HomePath @(".config", "F3D"))
) | ForEach-Object { Ensure-Directory -Path $_ }

$playwrightPath = Join-Path $repoRoot "node_modules\.cache\ms-playwright"
Ensure-Directory -Path $playwrightPath
Set-UserEnvironmentVariable -Name "DEVCONTAINER" -Value "false"
Set-UserEnvironmentVariable -Name "VCPKG_ROOT" -Value (Join-Path $repoRoot ".repo\cache\vcpkg")
Set-UserEnvironmentVariable -Name "CMAKE_PRESET" -Value "windows"
Set-UserEnvironmentVariable -Name "DOTNET_CLI_TELEMETRY_OPTOUT" -Value "1"
Set-UserEnvironmentVariable -Name "PLAYWRIGHT_BROWSERS_PATH" -Value $playwrightPath
Set-UserEnvironmentVariable -Name "SEMIO_GITKRAKEN_WORKSPACE_NAME" -Value "compose"
Set-UserEnvironmentVariable -Name "SEMIO_GITKRAKEN_AUTO_START" -Value "false"
Set-UserEnvironmentVariable -Name "SEMIO_F3D_AUTO_START" -Value "true"
Set-UserEnvironmentVariable -Name "SEMIO_POST_ATTACH_SKIP_EXTENSION_INSTALL" -Value ""
Set-UserEnvironmentVariable -Name "EDITOR" -Value "code --wait"
Set-UserEnvironmentVariable -Name "NEO4J_URI" -Value "bolt://localhost:7687"
Set-UserEnvironmentVariable -Name "NEO4J_USERNAME" -Value "neo4j"
Set-UserEnvironmentVariable -Name "NEO4J_PASSWORD" -Value "password"
    Set-UserEnvironmentVariable -Name "NEO4J_TELEMETRY" -Value "false"
    Set-UserEnvironmentVariable -Name "NEO4J_DATABASE" -Value "compose"
#endregion 🗂️UserState

#region 🗄️Neo4jRuntime
Ensure-NativeNeo4j -RepoRoot $repoRoot
if ($sessionStartEffective) {
    Write-Step "Native Windows IDE session setup complete."
    exit 0
}
#endregion 🗄️Neo4jRuntime

#region 🌐GlobalCliInstall
if (-not $SkipGlobalCliInstall) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")
    $rustupPath = Get-CommandPathOrThrow -Label "rustup" -Candidates @("rustup.exe")
    $uvPath = Get-CommandPathOrThrow -Label "uv" -Candidates @("uv.exe")

    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("add", "--global", "@google/gemini-cli", "typescript-language-server", "typescript", "pyright") -WorkingDirectory $repoRoot
    Install-UvTool -ToolName "ruff" -UvPath $uvPath
    Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("target", "add", "wasm32-unknown-unknown") -WorkingDirectory $repoRoot
    $cargoConfigPath = Join-HomePath @(".cargo", "config.toml")
    @"
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
"@ | Set-Content -Path $cargoConfigPath -Encoding UTF8
}
#endregion 🌐GlobalCliInstall

#region 🧱RepoBootstrap
if (-not $SkipRepoBootstrap) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")

    Configure-GitSafeDirectories -RepoRoot $repoRoot
    Stop-RepoPythonProcesses -RepoRoot $repoRoot
    $goPath = Get-CommandPathOrThrow -Label "go" -Candidates @("go.exe", "go")
    Write-Step "Building repo client binary…"
    Invoke-RepoCommand -FilePath $goPath -ArgumentList @("build", "-o", (Join-Path $repoRoot "repo/client/client.exe"), "./repo/client/mcp/go") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("install") -WorkingDirectory $repoRoot
    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("nx", "run", "workspace:setup") -WorkingDirectory $repoRoot
    Configure-GitKrakenWorkspace -RepoRoot $repoRoot
}
#endregion 🧱RepoBootstrap

#region 🎭Editors
if (-not $SkipEditorInstall) {
    Refresh-CurrentProcessPath
    $editorCliPaths = @(
        (Get-FirstCommandPath @((Join-Path $env:LOCALAPPDATA "Programs\Microsoft VS Code\bin\code.cmd"), "code.cmd", "code")),
        (Get-FirstCommandPath @((Join-Path $env:LOCALAPPDATA "Programs\cursor\resources\app\bin\cursor.cmd"), "cursor.cmd", "cursor")),
        (Get-FirstCommandPath @((Join-Path $env:LOCALAPPDATA "Programs\Windsurf\bin\windsurf.cmd"), "windsurf.cmd", "windsurf")),
        (Get-FirstCommandPath @((Join-Path $env:LOCALAPPDATA "Programs\Antigravity\bin\antigravity.cmd"), "antigravity.cmd", "antigravity"))
    ) | Where-Object { $_ } | Select-Object -Unique
    Install-EditorExtensions -RepoRoot $repoRoot -EditorCliPaths $editorCliPaths
}
#endregion 🎭Editors

#region 🎬Playwright
if (-not $SkipPlaywrightInstall) {
    Refresh-CurrentProcessPath
    $bunPath = Get-CommandPathOrThrow -Label "bun" -Candidates @("bun.exe", "bun")
    Invoke-RepoCommand -FilePath $bunPath -ArgumentList @("x", "playwright", "install", "chromium") -WorkingDirectory $repoRoot
}
#endregion 🎬Playwright

Write-Step "Native bootstrap complete. Open a new shell to pick up the persisted PATH/env vars."
