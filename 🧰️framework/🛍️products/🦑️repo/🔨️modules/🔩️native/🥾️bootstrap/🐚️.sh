#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), Bun, the Node.js
# the build tool runs on (`package.json` engines.node), rustup with the `rust-toolchain.toml` pin, Go, .NET, and uv,
# then `workspace:setup` (which installs packages).
# 🧭️Invoke from a fresh clone: `bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup` (full
# bootstrap); `bun ./📜️script.ts setup native` routes here once Bun exists.
# Every step probes before it installs and never writes user-global tool configuration.
# Native setup is rooted here and does not depend on devcontainer scripts.
# #endregion 🔖️Header
set -euo pipefail

#region 🔖️Dispatch
package_declares_workspace() {
  LC_ALL=C awk '
    function whitespace() { while (substr(document, position, 1) ~ /^[ \t\r\n]$/) position++ }
    function string_value(    character, escape, digits, offset, code, digit) {
      if (substr(document, position++, 1) != "\"") { invalid = 1; return }
      token = ""
      while (position <= length(document)) {
        character = substr(document, position++, 1)
        if (character == "\"") return
        if (character ~ /[[:cntrl:]]/) { invalid = 1; return }
        if (character == "\\") {
          escape = substr(document, position++, 1)
          if (escape == "u") {
            digits = substr(document, position, 4)
            if (length(digits) != 4 || digits ~ /[^0-9a-fA-F]/) { invalid = 1; return }
            code = 0
            for (offset = 1; offset <= 4; offset++) {
              digit = index("0123456789abcdef", tolower(substr(digits, offset, 1))) - 1
              code = code * 16 + digit
            }
            token = token (code > 0 && code < 128 ? sprintf("%c", code) : "#")
            position += 4
          } else if (escape ~ /^["\\\/bfnrt]$/) token = token (escape ~ /^["\\\/]$/ ? escape : "#")
          else { invalid = 1; return }
        } else token = token character
      }
      invalid = 1
    }
    function value(depth,    character, key, kind, part) {
      whitespace()
      character = substr(document, position, 1)
      if (character == "{") {
        position++; whitespace()
        if (substr(document, position, 1) == "}") { position++; return 1 }
        while (!invalid) {
          whitespace(); string_value(); key = token; whitespace()
          if (invalid || substr(document, position++, 1) != ":") { invalid = 1; return }
          kind = value(depth + 1)
          if (depth == 0 && key == "workspaces") workspace = kind == 1 || kind == 2
          whitespace(); character = substr(document, position++, 1)
          if (character == "}") return 1
          if (character != ",") { invalid = 1; return }
        }
      } else if (character == "[") {
        position++; whitespace()
        if (substr(document, position, 1) == "]") { position++; return 2 }
        while (!invalid) {
          value(depth + 1); whitespace(); character = substr(document, position++, 1)
          if (character == "]") return 2
          if (character != ",") { invalid = 1; return }
        }
      } else if (character == "\"") { string_value(); return 3 }
      else {
        part = substr(document, position)
        if (match(part, /^(true|false|null)/) || match(part, /^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?/)) { position += RLENGTH; return 3 }
        invalid = 1
      }
    }
    { document = document $0 "\n" }
    END { position = 1; value(0); whitespace(); if (invalid || position <= length(document)) exit 2; exit workspace ? 0 : 1 }
  ' "$1"
}

resolve_repo_root() {
  local dir explicit=0 status i=0 parent
  if [ -n "${SEMIO_WORKSPACE_ROOT:-}" ]; then
    dir="$(cd "$SEMIO_WORKSPACE_ROOT" && pwd)" || return 1
    explicit=1
  else
    dir="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")}" && pwd)" || return 1
  fi
  while [ "$i" -lt 32 ]; do
    if [ -f "$dir/📋️project.json" ] && [ ! -L "$dir/📋️project.json" ] && [ -f "$dir/package.json" ] && [ ! -L "$dir/package.json" ]; then
      if package_declares_workspace "$dir/package.json"; then
        printf '%s' "$dir"
        return
      else
        status=$?
        if [ "$status" -gt 1 ]; then
          printf '%s\n' "Invalid workspace package: $dir/package.json" >&2
          return 1
        fi
      fi
    fi
    [ "$explicit" -eq 1 ] && break
    parent="$(dirname "$dir")"
    [ "$parent" = "$dir" ] && break
    dir="$parent"
    i=$((i + 1))
  done
  printf '%s\n' "Workspace root requires paired package.json workspaces and 📋️project.json" >&2
  return 1
}
REPO_ROOT="$(resolve_repo_root)"
case "${1:-}" in
setup)
  shift || true
  ;;
*)
  printf '%s\n' "usage: bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh setup" >&2
  exit 1
  ;;
esac
cd "$REPO_ROOT" || exit 1
#endregion 🔖️Dispatch

#region 🔖️Config
SKIP_REPO_BOOTSTRAP="${SKIP_REPO_BOOTSTRAP:-0}"
DOTNET_CHANNEL="${DOTNET_CHANNEL:-8.0}"
LOCAL_BIN="${HOME}/.local/bin"
#endregion 🔖️Config

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

#region 🔖️RepoBootstrap
repo_bootstrap() {
  if [ "$SKIP_REPO_BOOTSTRAP" = "1" ]; then
    log "Skipping repo bootstrap (SKIP_REPO_BOOTSTRAP=1)."
    return 0
  fi
  cd "$REPO_ROOT"
  export SEMIO_WORKSPACE_ROOT="$REPO_ROOT"
  log "workspace:setup…"
  bun repo run workspace:setup
  log "Building repo client binary…"
  bun repo run @semio-tech/repo-client:build || log "repo client build failed; install Go 1.21+ and rerun setup."
}
#endregion 🔖️RepoBootstrap

#region 🔖️Main
cd "$REPO_ROOT"
ensure_native_toolchain
ensure_bun
ensure_node || log "Node.js provisioning failed; install the Node.js version pinned in package.json engines.node and rerun setup."
ensure_rust_toolchain || log "Rust toolchain provisioning failed; install rustup from https://rustup.rs and rerun setup."
ensure_go || log "Go provisioning failed; install Go 1.21+ from https://go.dev/dl and rerun setup."
ensure_dotnet || log ".NET SDK ${DOTNET_CHANNEL} provisioning failed; install it from https://dot.net and rerun setup."
ensure_uv || log "uv provisioning failed; install it from https://docs.astral.sh/uv and rerun setup."
repo_bootstrap
log "Native (Unix) bootstrap complete. Open a new shell to load the updated PATH."
#endregion 🔖️Main
