#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), Bun, the Node.js
# Nx runs on (`package.json` engines.node), rustup with the `rust-toolchain.toml` pin, Go, .NET, uv, optional Neo4j Desktop,
# then `workspace:setup` (which installs packages).
# 🧭️Invoke from a fresh clone: `bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup` (full
# bootstrap) or `… start` (IDE session / Neo4j-only path); `bun ./📜️script.ts setup native` routes here once Bun exists.
# Every step probes before it installs and never writes user-global tool configuration.
# Native setup is rooted here and does not depend on devcontainer scripts.
# #endregion 🔖️Header
set -euo pipefail

#region 🔖️Dispatch
resolve_repo_root() {
  if [ -n "${COMPOSE_REPO_ROOT:-}" ]; then
    printf '%s' "$(cd "${COMPOSE_REPO_ROOT}" && pwd)"
    return
  fi
  local dir
  dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  local i=0
  while [ "$i" -lt 32 ]; do
    if [ -f "$dir/nx.json" ]; then
      printf '%s' "$dir"
      return
    fi
    local parent
    parent="$(dirname "$dir")"
    [ "$parent" = "$dir" ] && break
    dir="$parent"
    i=$((i + 1))
  done
  printf '%s' "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
}
REPO_ROOT="$(resolve_repo_root)"
case "${1:-}" in
setup)
  shift || true
  export COMPOSE_SESSION_START=0
  ;;
start)
  shift || true
  export COMPOSE_SESSION_START=1
  ;;
*)
  printf '%s\n' "usage: bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh <setup|start>" >&2
  exit 1
  ;;
esac
cd "$REPO_ROOT" || exit 1
#endregion 🔖️Dispatch

#region 🔖️Config
NEO4J_DESKTOP_INSTALLER_VERSION="${NEO4J_DESKTOP_INSTALLER_VERSION:-1.6.3}"
NEO4J_VERSION="${NEO4J_VERSION:-5.26.26}"
APOC_VERSION="${APOC_VERSION:-5.26.4}"
SKIP_NEO4J_DESKTOP="${SKIP_NEO4J_DESKTOP:-0}"
SKIP_REPO_BOOTSTRAP="${SKIP_REPO_BOOTSTRAP:-0}"
DOTNET_CHANNEL="${DOTNET_CHANNEL:-8.0}"
LOCAL_BIN="${HOME}/.local/bin"
COMPOSE_SESSION_START="${COMPOSE_SESSION_START:-0}"
#endregion 🔖️Config

#region 🔖️Logging
log() {
  printf '%s\n' "[compose] $*"
}
#endregion 🔖️Logging

#region 🔖️Neo4jEnv
append_neo4j_env_block() {
  local f="$1"
  [ -n "$f" ] || return 0
  mkdir -p "$(dirname "$f")"
  touch "$f"
  if grep -qF "#region 🔌️Neo4j" "$f" 2>/dev/null; then
    return 0
  fi
  {
    printf '\n'
    printf '%s\n' "#region 🔌️Neo4j"
    printf '%s\n' "export NEO4J_URI=bolt://localhost:7687"
    printf '%s\n' "export NEO4J_USERNAME=neo4j"
    printf '%s\n' "export NEO4J_PASSWORD=password"
    printf '%s\n' "export NEO4J_DATABASE=compose"
    printf '%s\n' "#endregion 🔌️Neo4j"
  } >>"$f"
  log "Appended Neo4j MCP env block to $f"
}

configure_neo4j_shell_env() {
  case "$(uname -s)" in
  Darwin)
    append_neo4j_env_block "${HOME}/.zshrc"
    append_neo4j_env_block "${HOME}/.bashrc"
    ;;
  Linux)
    append_neo4j_env_block "${HOME}/.bashrc"
    append_neo4j_env_block "${HOME}/.profile"
    ;;
  *)
    log "Unsupported OS for shell env; set NEO4J_* manually."
    ;;
  esac
  export NEO4J_URI="bolt://localhost:7687"
  export NEO4J_USERNAME="neo4j"
  export NEO4J_PASSWORD="password"
  export NEO4J_DATABASE="compose"
  export NEO4J_TELEMETRY="false"
}
#endregion 🔖️Neo4jEnv

#region 🔖️Neo4jRuntime
is_neo4j_reachable() {
  if command -v nc >/dev/null 2>&1; then
    nc -z 127.0.0.1 7687 >/dev/null 2>&1
    return $?
  fi
  timeout 1 bash -c "echo >/dev/tcp/127.0.0.1/7687" >/dev/null 2>&1
}

run_cypher() {
  local database="$1"
  local cypher="$2"
  local bundled_cypher_shell="$REPO_ROOT/.🧬semio/🦑️repo/⚡️cache/neo4j/neo4j-community-$NEO4J_VERSION/bin/cypher-shell"
  if command -v cypher-shell >/dev/null 2>&1; then
    cypher-shell -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" >/dev/null 2>&1
    return $?
  fi
  if [ -x "$bundled_cypher_shell" ]; then
    "$bundled_cypher_shell" -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" >/dev/null 2>&1
    return $?
  fi
  return 1
}

run_cypher_expect() {
  local database="$1"
  local cypher="$2"
  local pattern="$3"
  local bundled_cypher_shell="$REPO_ROOT/.🧬semio/🦑️repo/⚡️cache/neo4j/neo4j-community-$NEO4J_VERSION/bin/cypher-shell"
  local output
  if command -v cypher-shell >/dev/null 2>&1; then
    output="$(cypher-shell -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" 2>/dev/null)" || return 1
    printf '%s\n' "$output" | grep -Eq "$pattern"
    return $?
  fi
  if [ -x "$bundled_cypher_shell" ]; then
    output="$("$bundled_cypher_shell" -a bolt://localhost:7687 -u neo4j -p password -d "$database" --format plain "$cypher" 2>/dev/null)" || return 1
    printf '%s\n' "$output" | grep -Eq "$pattern"
    return $?
  fi
  return 1
}

neo4j_schema_cypher_uri() {
  local technology="$1"
  printf 'file://%s/.🧬semio/🦑️repo/\\uD83D\\uDEC2/%s.cypher\n' "$REPO_ROOT" "$technology"
}

detect_neo4j_desktop_dbms_home() {
  ps -eo args 2>/dev/null | sed -nE 's/.*--home-dir="?([^" ]*\.Neo4jDesktop2?\/Data\/dbmss\/[^" ]+)"?.*/\1/p' | head -1
}

set_text_setting() {
  local file="$1"
  local key="$2"
  local value="$3"
  mkdir -p "$(dirname "$file")"
  touch "$file"
  if grep -Eq "^[#[:space:]]*${key}=" "$file"; then
    sed -i.bak -E "s|^[#[:space:]]*${key}=.*|${key}=${value}|" "$file"
  else
    printf '%s=%s\n' "$key" "$value" >>"$file"
  fi
}

resolve_native_graph_database() {
  local preferred="${NEO4J_DATABASE:-compose}"
  if run_cypher "$preferred" "RETURN 1;"; then
    printf '%s\n' "$preferred"
    return 0
  fi
  printf '%s\n' "$preferred"
}

install_neo4j_desktop_apoc() {
  local dbms_home
  dbms_home="$(detect_neo4j_desktop_dbms_home)"
  if [ -z "$dbms_home" ]; then
    log "APOC auto-install skipped because the reachable DBMS is not a running Neo4j Desktop local DBMS."
    return 1
  fi

  mkdir -p "$dbms_home/plugins"
  local core_jar
  core_jar="$(find "$dbms_home/labs" -maxdepth 1 -type f -name 'apoc-*-core.jar' 2>/dev/null | sort -r | head -1)"
  if [ -n "$core_jar" ]; then
    cp -f "$core_jar" "$dbms_home/plugins/$(basename "$core_jar")"
    local apoc_version
    apoc_version="$(basename "$core_jar" | sed -nE 's/apoc-(.+)-core\.jar/\1/p')"
    if [ -n "$apoc_version" ] && [ ! -f "$dbms_home/plugins/apoc-extended-$apoc_version.jar" ]; then
      curl -fSL --retry 3 --retry-delay 2 -o "$dbms_home/plugins/apoc-extended-$apoc_version.jar" "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-extended/$apoc_version/apoc-extended-$apoc_version.jar"
    fi
  fi

  set_text_setting "$dbms_home/conf/neo4j.conf" "dbms.security.procedures.allowlist" "apoc.*"
  set_text_setting "$dbms_home/conf/neo4j.conf" "dbms.security.procedures.unrestricted" "apoc.*"
  set_text_setting "$dbms_home/conf/neo4j.conf" "server.directories.import" "$REPO_ROOT"
  set_text_setting "$dbms_home/conf/neo4j.conf" "initial.dbms.default_database" "compose"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.export.file.enabled" "true"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.import.file.enabled" "true"
  set_text_setting "$dbms_home/conf/apoc.conf" "apoc.import.file.use_neo4j_config" "false"

  if [ -x "$dbms_home/bin/neo4j" ]; then
    log "Restarting Neo4j Desktop local compose DBMS to load APOC..."
    pkill -f "$dbms_home" >/dev/null 2>&1 || true
    for _ in $(seq 1 30); do
      is_neo4j_reachable || break
      sleep 1
    done
    nohup "$dbms_home/bin/neo4j" console >"$dbms_home/logs/compose-native-console.log" 2>&1 &
    for _ in $(seq 1 45); do
      is_neo4j_reachable && break
      sleep 2
    done
  else
    log "APOC installed into the compose DBMS. Restart it in Neo4j Desktop to load the program."
  fi
}

ensure_native_neo4j_tools() {
  local cache_root="$REPO_ROOT/.🧬semio/🦑️repo/⚡️cache/neo4j"
  local runtime_root="$cache_root/neo4j-community-$NEO4J_VERSION"
  mkdir -p "$cache_root"
  if [ ! -d "$runtime_root" ]; then
    local archive="$cache_root/neo4j-community-$NEO4J_VERSION-unix.tar.gz"
    log "Downloading Neo4j Community $NEO4J_VERSION tools for cypher-shell..."
    curl -fSL --retry 3 --retry-delay 2 -o "$archive" "https://dist.neo4j.org/neo4j-community-$NEO4J_VERSION-unix.tar.gz"
    tar -xzf "$archive" -C "$cache_root"
  fi

  mkdir -p "$runtime_root/plugins"
  if [ ! -f "$runtime_root/plugins/apoc-core-$APOC_VERSION-core.jar" ]; then
    curl -fSL --retry 3 --retry-delay 2 -o "$runtime_root/plugins/apoc-core-$APOC_VERSION-core.jar" "https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-core/$APOC_VERSION/apoc-core-$APOC_VERSION-core.jar"
  fi
  if [ ! -f "$runtime_root/plugins/apoc-$APOC_VERSION-extended.jar" ]; then
    curl -fSL --retry 3 --retry-delay 2 -o "$runtime_root/plugins/apoc-$APOC_VERSION-extended.jar" "https://github.com/neo4j-contrib/neo4j-apoc-procedures/releases/download/$APOC_VERSION/apoc-$APOC_VERSION-extended.jar"
  fi

  local conf="$runtime_root/conf/neo4j.conf"
  mkdir -p "$runtime_root/data" "$runtime_root/logs"
  set_conf_value "$conf" "server.default_listen_address" "127.0.0.1"
  set_conf_value "$conf" "server.bolt.listen_address" ":7687"
  set_conf_value "$conf" "server.http.listen_address" ":7474"
  set_conf_value "$conf" "dbms.usage_report.enabled" "false"
  set_conf_value "$conf" "server.directories.data" "$runtime_root/data"
  set_conf_value "$conf" "server.directories.logs" "$runtime_root/logs"
  set_conf_value "$conf" "server.directories.import" "$REPO_ROOT"
  set_conf_value "$conf" "dbms.security.procedures.allowlist" "apoc.*"
  set_conf_value "$conf" "dbms.security.procedures.unrestricted" "apoc.*"
  set_conf_value "$conf" "initial.dbms.default_database" "compose"
  {
    printf '%s\n' "apoc.export.file.enabled=true"
    printf '%s\n' "apoc.import.file.enabled=true"
    printf '%s\n' "apoc.import.file.use_neo4j_config=false"
  } >"$runtime_root/conf/apoc.conf"
  printf '%s\n' "$runtime_root"
}

set_conf_value() {
  local conf="$1"
  local key="$2"
  local value="$3"
  if grep -Eq "^[#[:space:]]*${key}=" "$conf"; then
    sed -i.bak -E "s|^[#[:space:]]*${key}=.*|${key}=${value}|" "$conf"
  else
    printf '%s=%s\n' "$key" "$value" >>"$conf"
  fi
}

extra_neo4j_graph_names_from_env() {
  [ -z "${NEO4J_EXTRA_GRAPH_DATABASES:-}" ] && return 0
  local _ifs=$IFS
  IFS=,
  local s n
  for s in $NEO4J_EXTRA_GRAPH_DATABASES; do
    n="$(echo "$s" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
    [ -n "$n" ] && printf '%s\n' "$n"
  done
  IFS=$_ifs
}

neo4j_quote_db_name() {
  local n="$1"
  if [[ "$n" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]]; then
    printf '%s' "$n"
  else
    local e="${n//\`/``}"
    printf '`%s`' "$e"
  fi
}

ensure_native_neo4j() {
  local runtime_root
  local graph_db
  if is_neo4j_reachable; then
    log "Neo4j is reachable at bolt://localhost:7687."
  else
    log "Neo4j is not reachable. Create and start a native Neo4j Desktop local DBMS named compose on Bolt port 7687, password password, then run this setup again."
    return 0
  fi

  runtime_root="$(ensure_native_neo4j_tools)"
  graph_db="$(resolve_native_graph_database)"
  log "Neo4j graph database for imports: ${graph_db}"

  if ! run_cypher_expect "$graph_db" "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" "\\b2\\b"; then
    install_neo4j_desktop_apoc || true
    graph_db="$(resolve_native_graph_database)"
    if ! run_cypher_expect "$graph_db" "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" "\\b2\\b"; then
      log "Neo4j is reachable, but APOC is not ready. In Neo4j Desktop, install/enable APOC for the local compose DBMS and restart it."
      return 0
    fi
  fi

  run_cypher system "CREATE DATABASE compose IF NOT EXISTS;" >/dev/null 2>&1 || true
  while IFS= read -r db; do
    [ -z "$db" ] && continue
    q="$(neo4j_quote_db_name "$db")"
    run_cypher system "CREATE DATABASE $q IF NOT EXISTS;" >/dev/null 2>&1 || true
  done < <(extra_neo4j_graph_names_from_env)
  #region 🔥️Neo4jEnterpriseDropStockDb
  # Enterprise Desktop: schema often lands in the stock `neo4j` DB. Ensure `compose` is default, then drop `neo4j`.
  # Community (single user DB): these calls fail harmlessly and are skipped via `|| true`.
  run_cypher system "START DATABASE compose WAIT;" >/dev/null 2>&1 || true
  run_cypher system "CALL dbms.setDefaultDatabase('compose');" >/dev/null 2>&1 || true
  run_cypher system "DROP DATABASE neo4j IF EXISTS CASCADE ALIASES WAIT;" >/dev/null 2>&1 || true
  #endregion 🔥️Neo4jEnterpriseDropStockDb
  graph_db="$(resolve_native_graph_database)"
  log "Neo4j graph database for imports (after optional CREATE DATABASE compose): ${graph_db}"

  log "Neo4j: clearing graph in ${graph_db}, then loading generated .🧬semio/🦑️repo/🛂️manifest/*.cypher (from bun run generate) …"
  run_cypher "$graph_db" "MATCH (n) DETACH DELETE n" || log "Neo4j wipe skipped (failed)."

  local technologies=(compose elements coda reuse)
  local ex
  while IFS= read -r ex; do
    [ -n "$ex" ] && technologies+=("$ex")
  done < <(extra_neo4j_graph_names_from_env)
  for technology in "${technologies[@]}"; do
    local schema_file="$REPO_ROOT/.🧬semio/🦑️repo/🛂️manifest/${technology}.cypher"
    if [ -f "$schema_file" ] && grep -Eqv '^[[:space:]]*(//|:|$)' "$schema_file"; then
      local schema_uri
      schema_uri="$(neo4j_schema_cypher_uri "$technology")"
      run_cypher "$graph_db" "CALL apoc.cypher.runFile('$schema_uri') YIELD row RETURN count(row) AS rows;" || true
      log "Neo4j schema imported into ${graph_db}: ${technology}."
    fi
  done
  if command -v bun >/dev/null 2>&1; then
    (cd "$REPO_ROOT" && NEO4J_DATABASE="$graph_db" bun ./📜️script.ts purge neo4j) || log "Neo4j legacy-property prune skipped (bun or cypher-shell failed)."
  fi
}
#endregion 🔖️Neo4jRuntime

#region 🔖️EnsureUv
ensure_uv() {
  if command -v uv >/dev/null 2>&1; then
    return 0
  fi
  log "Installing uv…"
  curl -LsSf https://astral.sh/uv/install.sh | sh
  export PATH="${HOME}/.local/bin:${PATH}"
}
#endregion 🔖️EnsureUv

#region 🔖️NativeToolchain
run_privileged() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    log "Root privileges are required for: $*"
    return 1
  fi
}

linux_package_manager() {
  local manager
  for manager in apt-get dnf pacman zypper; do
    if command -v "$manager" >/dev/null 2>&1; then
      printf '%s' "$manager"
      return 0
    fi
  done
  return 1
}

linux_toolchain_packages() {
  case "$1" in
  apt-get) printf '%s\n' build-essential cmake ninja-build pkg-config uuid-dev curl ca-certificates git unzip xz-utils libicu-dev procps ;;
  dnf) printf '%s\n' gcc gcc-c++ make cmake ninja-build pkgconf-pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu procps-ng findutils ;;
  pacman) printf '%s\n' base-devel cmake ninja pkgconf util-linux-libs curl ca-certificates git unzip xz icu procps-ng ;;
  zypper) printf '%s\n' gcc gcc-c++ make cmake ninja pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu-devel procps ;;
  esac
}

install_linux_packages() {
  local manager="$1"
  shift
  case "$manager" in
  apt-get) run_privileged env DEBIAN_FRONTEND=noninteractive apt-get update -qq && run_privileged env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "$@" ;;
  dnf) run_privileged dnf install -y "$@" ;;
  pacman) run_privileged pacman -Sy --needed --noconfirm "$@" ;;
  zypper) run_privileged zypper --non-interactive install --no-recommends "$@" ;;
  *) return 1 ;;
  esac
}

report_missing_tools() {
  local missing=() tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null 2>&1 || missing+=("$tool")
  done
  if [ "${#missing[@]}" -gt 0 ]; then
    log "Native toolchain incomplete, missing: ${missing[*]}. Install them with your system package manager and rerun setup."
  fi
}

ensure_native_toolchain() {
  case "$(uname -s)" in
  Darwin)
    xcode-select -p >/dev/null 2>&1 || log "Xcode Command Line Tools are missing; run xcode-select --install and rerun setup."
    if command -v brew >/dev/null 2>&1; then
      brew install cmake ninja pkg-config || log "Homebrew could not install cmake, ninja and pkg-config."
    else
      log "Homebrew not available; install CMake, Ninja and pkg-config before running native C++ builds."
    fi
    export CMAKE_PRESET="${CMAKE_PRESET:-macos}"
    ;;
  Linux)
    local manager packages=() package
    if manager="$(linux_package_manager)"; then
      while IFS= read -r package; do packages+=("$package"); done < <(linux_toolchain_packages "$manager")
      log "Installing the native toolchain with $manager: ${packages[*]}"
      install_linux_packages "$manager" "${packages[@]}" || log "The native toolchain install through $manager failed (package index, lock or privileges); setup continues and reports what is missing."
    else
      log "No supported package manager (apt-get, dnf, pacman, zypper) found; install a C/C++ compiler, make, CMake, Ninja, pkg-config, libuuid, ICU, curl, git and unzip."
    fi
    export CMAKE_PRESET="${CMAKE_PRESET:-linux}"
    ;;
  esac
  report_missing_tools cc c++ make cmake ninja pkg-config curl git unzip
}

verify_sha256() {
  local file="$1" expected="$2" actual
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$file" | awk '{print $1}')"
  else
    actual="$(shasum -a 256 "$file" | awk '{print $1}')"
  fi
  [ "$actual" = "$expected" ] || {
    log "Checksum mismatch for $file (expected $expected, got $actual)."
    return 1
  }
}

ensure_rust_toolchain() {
  export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
  export PATH="$CARGO_HOME/bin:$PATH"
  if ! command -v rustup >/dev/null 2>&1; then
    log "Installing rustup…"
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh -s -- -y --profile minimal --default-toolchain none || return 1
  fi
  log "Installing the rust-toolchain.toml toolchain…"
  (cd "$REPO_ROOT" && rustup toolchain install) || return 1
}

ensure_node() {
  export PATH="$LOCAL_BIN:$PATH"
  local version os arch archive root download expected
  version="$(cd "$REPO_ROOT" && bun -e 'process.stdout.write(require("./package.json").engines.node)')" || return 1
  if [ "$(node --version 2>/dev/null)" = "v$version" ]; then
    return 0
  fi
  case "$(uname -s)" in
  Darwin) os=darwin ;;
  Linux) os=linux ;;
  *) return 1 ;;
  esac
  case "$(uname -m)" in
  x86_64 | amd64) arch=x64 ;;
  arm64 | aarch64) arch=arm64 ;;
  *) return 1 ;;
  esac
  archive="node-v$version-$os-$arch.tar.xz"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/node/v$version"
  if [ ! -x "$root/bin/node" ]; then
    log "Installing Node.js $version into $root…"
    download="$(mktemp -d)"
    curl -fsSL --retry 3 -o "$download/$archive" "https://nodejs.org/dist/v$version/$archive" || return 1
    expected="$(curl -fsSL --retry 3 "https://nodejs.org/dist/v$version/SHASUMS256.txt" | awk -v archive="$archive" '$2 == archive { print $1 }')" || return 1
    verify_sha256 "$download/$archive" "$expected" || return 1
    mkdir -p "$root" && tar -xJf "$download/$archive" -C "$root" --strip-components=1 || return 1
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sf "$root/bin/node" "$LOCAL_BIN/node"
}

go_minor_version() {
  go version 2>/dev/null | sed -nE 's/^go version go1\.([0-9]+).*/\1/p'
}

ensure_go() {
  export PATH="$LOCAL_BIN:$PATH"
  local minor
  minor="$(go_minor_version)"
  if [ -n "$minor" ] && [ "$minor" -ge 21 ]; then
    return 0
  fi
  local os arch version archive root download
  case "$(uname -s)" in
  Darwin) os=darwin ;;
  Linux) os=linux ;;
  *) return 1 ;;
  esac
  case "$(uname -m)" in
  x86_64 | amd64) arch=amd64 ;;
  arm64 | aarch64) arch=arm64 ;;
  *) return 1 ;;
  esac
  version="$(curl -fsSL --retry 3 'https://go.dev/VERSION?m=text' | sed -n 1p)" || return 1
  archive="$version.$os-$arch.tar.gz"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/go/$version"
  if [ ! -x "$root/go/bin/go" ]; then
    log "Installing Go $version into $root…"
    download="$(mktemp -d)"
    curl -fsSL --retry 3 -o "$download/$archive" "https://dl.google.com/go/$archive" || return 1
    verify_sha256 "$download/$archive" "$(curl -fsSL --retry 3 "https://dl.google.com/go/$archive.sha256")" || return 1
    mkdir -p "$root" && tar -xzf "$download/$archive" -C "$root" || return 1
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sf "$root/go/bin/go" "$LOCAL_BIN/go"
  ln -sf "$root/go/bin/gofmt" "$LOCAL_BIN/gofmt"
}

ensure_dotnet() {
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}" DOTNET_CLI_TELEMETRY_OPTOUT=1
  export PATH="$LOCAL_BIN:$PATH"
  if command -v dotnet >/dev/null 2>&1 && dotnet --list-runtimes 2>/dev/null | grep -q "^Microsoft.NETCore.App ${DOTNET_CHANNEL}\."; then
    return 0
  fi
  log "Installing .NET SDK $DOTNET_CHANNEL into $DOTNET_ROOT…"
  local installer
  installer="$(mktemp)"
  curl -fsSL --retry 3 -o "$installer" https://dot.net/v1/dotnet-install.sh || return 1
  bash "$installer" --channel "$DOTNET_CHANNEL" --install-dir "$DOTNET_ROOT" --no-path || return 1
  rm -f "$installer"
  mkdir -p "$LOCAL_BIN"
  ln -sf "$DOTNET_ROOT/dotnet" "$LOCAL_BIN/dotnet"
}
#endregion 🔖️NativeToolchain

#region 🔖️EnsureBun
ensure_bun() {
  local package_manager
  package_manager="$(awk -F'"' '/"packageManager"[[:space:]]*:/ { print $4; exit }' "$REPO_ROOT/package.json")"
  if ! printf '%s\n' "$package_manager" | grep -Eq '^bun@[0-9]+\.[0-9]+\.[0-9]+$'; then
    log "package.json must declare an exact packageManager bun@x.y.z before native setup."
    return 1
  fi
  local required_version="${package_manager#bun@}"
  local actual_version
  actual_version="$(bun --version 2>/dev/null || true)"
  export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
  export PATH="${BUN_INSTALL}/bin:${PATH}"
  if [ "$actual_version" != "$required_version" ]; then
    log "Installing repository-pinned Bun $required_version…"
    curl -fsSL https://bun.sh/install | bash -s "bun-v$required_version"
    hash -r
    actual_version="$(bun --version 2>/dev/null || true)"
  fi
  if [ "$actual_version" != "$required_version" ]; then
    log "Bun toolchain mismatch: package.json requires $required_version, but native setup resolved ${actual_version:-missing}. Install bun-v$required_version and rerun setup."
    return 1
  fi
}
#endregion 🔖️EnsureBun

#region 🔖️InstallNeo4jDesktop
install_linux_fuse_deps() {
  if [ "${SKIP_LINUX_APT:-0}" = "1" ]; then
    return 0
  fi
  if ! command -v apt-get >/dev/null 2>&1; then
    return 0
  fi
  log "Installing fuse dependencies for Neo4j Desktop AppImage (apt)…"
  install_linux_packages apt-get wget curl ca-certificates libfuse2 fuse3 libglib2.0-0 libgtk-3-0 libnotify4 libnss3 libxss1 libxtst6 xdg-utils \
    || log "Optional Neo4j Desktop AppImage packages skipped (non-fatal)."
}

install_neo4j_desktop_linux_appimage() {
  local arch
  arch="$(uname -m)"
  if [ "$arch" != "x86_64" ]; then
    log "Neo4j Desktop Linux AppImage is published for x86_64 only; install manually from https://neo4j.com/deployment-center/?desktop-gdb (arch=${arch})."
    return 0
  fi
  install_linux_fuse_deps
  local ver="$NEO4J_DESKTOP_INSTALLER_VERSION"
  local url="https://dist.neo4j.org/neo4j-desktop/linux/neo4j-desktop-${ver}-x86_64.AppImage"
  local dest_dir="${HOME}/.local/share/semio/neo4j-desktop"
  local dest="${dest_dir}/neo4j-desktop-${ver}-x86_64.AppImage"
  mkdir -p "$dest_dir"
  if [ -f "$dest" ]; then
    log "Neo4j Desktop AppImage already present: $dest"
  else
    log "Downloading Neo4j Desktop ${ver} AppImage…"
    curl -fSL --retry 3 --retry-delay 2 -o "$dest.partial" "$url"
    mv "$dest.partial" "$dest"
  fi
  chmod a+x "$dest"
  local link="${HOME}/.local/bin/neo4j-desktop"
  mkdir -p "${HOME}/.local/bin"
  ln -sf "$dest" "$link"
  log "Neo4j Desktop ready: $link"
}

install_neo4j_desktop_macos() {
  if ! command -v brew >/dev/null 2>&1; then
    log "Installing Homebrew (non-interactive)…"
    NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" || true
  fi
  for b in /opt/homebrew/bin/brew /usr/local/bin/brew; do
    if [ -x "$b" ]; then
      eval "$("$b" shellenv)"
      break
    fi
  done
  if ! command -v brew >/dev/null 2>&1; then
    log "Homebrew not available; install Neo4j Desktop from https://neo4j.com/deployment-center/?desktop-gdb"
    return 0
  fi
  log "Installing Neo4j Desktop (Homebrew cask neo4j-desktop)…"
  if brew list --cask neo4j-desktop >/dev/null 2>&1; then
    brew upgrade --cask neo4j-desktop || true
  else
    brew install --cask neo4j-desktop
  fi
}

install_neo4j_desktop() {
  if [ "$SKIP_NEO4J_DESKTOP" = "1" ]; then
    log "Skipping Neo4j Desktop install (SKIP_NEO4J_DESKTOP=1)."
    return 0
  fi
  case "$(uname -s)" in
  Darwin) install_neo4j_desktop_macos ;;
  Linux) install_neo4j_desktop_linux_appimage ;;
  *) log "Skipping Neo4j Desktop auto-install on this OS." ;;
  esac
}
#endregion 🔖️InstallNeo4jDesktop

#region 🔖️RepoBootstrap
repo_bootstrap() {
  if [ "$SKIP_REPO_BOOTSTRAP" = "1" ]; then
    log "Skipping repo bootstrap (SKIP_REPO_BOOTSTRAP=1)."
    return 0
  fi
  cd "$REPO_ROOT"
  export NX_WORKSPACE_DATA_DIRECTORY="${REPO_ROOT}/.nx/workspace-data"
  mkdir -p "$NX_WORKSPACE_DATA_DIRECTORY"
  log "workspace:setup…"
  bun nx run workspace:setup
  log "Building repo client binary…"
  bun nx run @semio-tech/repo-client:build || log "repo client build failed; install Go 1.21+ and rerun setup."
}
#endregion 🔖️RepoBootstrap

#region 🔖️Main
cd "$REPO_ROOT"
configure_neo4j_shell_env
if [ "$COMPOSE_SESSION_START" = "1" ]; then
  ensure_native_neo4j
  log "Native IDE session setup complete."
  exit 0
fi
ensure_native_toolchain
ensure_bun
ensure_node || log "Node.js provisioning failed; install the Node.js version pinned in package.json engines.node and rerun setup."
ensure_rust_toolchain || log "Rust toolchain provisioning failed; install rustup from https://rustup.rs and rerun setup."
ensure_go || log "Go provisioning failed; install Go 1.21+ from https://go.dev/dl and rerun setup."
ensure_dotnet || log ".NET SDK ${DOTNET_CHANNEL} provisioning failed; install it from https://dot.net and rerun setup."
ensure_uv || log "uv provisioning failed; install it from https://docs.astral.sh/uv and rerun setup."
install_neo4j_desktop
ensure_native_neo4j
repo_bootstrap
log "Native (Unix) bootstrap complete. Open a new shell to load NEO4J_* from your profile, or run: export NEO4J_URI=bolt://localhost:7687 …"
#endregion 🔖️Main
