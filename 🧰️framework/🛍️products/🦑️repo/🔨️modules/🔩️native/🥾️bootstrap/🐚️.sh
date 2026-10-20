#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), Bun, the Node.js
# the build tool runs on (`package.json` engines.node), rustup with the `rust-toolchain.toml` pin, Go, .NET, and uv,
# then `workspace:setup` (which installs packages). Every downloaded installer is pinned and sha256-verified before it runs:
# Bun from its tools schema, the rest from `🧬️schema/🔣️.json` next to this script.
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
PINS_FILE="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
BUN_PINS_FILE="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json"

log() {
  printf '[semio] %s\n' "$*" >&2
}
#endregion 🔖️Config

#region 🔖️PinnedDownloads
host_platform() {
  case "$(uname -s)" in
  Darwin) printf darwin ;;
  Linux) printf linux ;;
  *) return 1 ;;
  esac
}

host_arch() {
  case "$(uname -m)" in
  x86_64 | amd64) printf x64 ;;
  arm64 | aarch64) printf arm64 ;;
  *) return 1 ;;
  esac
}

host_is_musl() {
  [ "$(uname -s)" = Linux ] || return 1
  case "$(ldd --version 2>&1 || true)" in
  *[Mm]usl*) return 0 ;;
  esac
  return 1
}

pin_row() {
  { grep -F "{\"tool\":\"$1\",\"version\":\"" "$PINS_FILE" | grep -F "\"platform\":\"$2\",\"arch\":\"$3\"," | sed -n '1p'; } || true
}

pin_field() {
  printf '%s' "$1" | sed -n "s/.*\"$2\":\"\([^\"]*\)\".*/\1/p"
}

host_pin() {
  local row
  row="$(pin_row "$1" "$(host_platform)" "$(host_arch)")" || return 1
  [ -n "$row" ] || {
    log "No pinned $1 for $(uname -s)/$(uname -m) in $PINS_FILE."
    return 1
  }
  printf '%s' "$row"
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

download_verified() {
  curl --proto '=https' --tlsv1.2 -fsSL --retry 3 --connect-timeout 30 --max-time 900 -o "$3" "$1" || return 1
  verify_sha256 "$3" "$2"
}

fetch_pinned() {
  local row="$1" destination="$2"
  download_verified "$(pin_field "$row" url)" "$(pin_field "$row" sha256)" "$destination"
}

version_at_least() {
  [ "$(printf '%s\n%s\n' "$1" "$2" | sort -V | sed -n '1p')" = "$2" ]
}

bun_release_row() {
  awk -v want_platform="$1" -v want_arch="$2" -v want_musl="$3" '
    /"version": "[0-9]+\.[0-9]+\.[0-9]+"/ && version == "" { split($0, part, "\""); version = part[4] }
    /"release": "https:/ { split($0, part, "\""); release = part[4] }
    /"platform": "[a-z0-9]+"/ { split($0, part, "\""); platform = part[4] }
    /"arch": "[a-z0-9]+"/ { split($0, part, "\""); arch = part[4] }
    /"musl": (true|false)/ { musl = index($0, "true") ? "true" : "false" }
    /"archive": "bun-/ { split($0, part, "\""); archive = part[4] }
    /"sha256": "[a-f0-9]+"/ && archive != "" {
      split($0, part, "\"")
      if (platform == want_platform && arch == want_arch && musl == want_musl) { print version, release, archive, part[4]; exit }
      archive = ""
    }
  ' "$BUN_PINS_FILE"
}
#endregion 🔖️PinnedDownloads

#region 🔖️EnsureUv
ensure_uv() {
  export PATH="$LOCAL_BIN:$PATH"
  local row version
  row="$(host_pin uv)" || return 1
  version="$(pin_field "$row" version)"
  if command -v uv >/dev/null 2>&1 && version_at_least "$(uv --version | awk '{print $2}')" "$version"; then
    return 0
  fi
  log "Installing uv $version…"
  local download
  download="$(mktemp -d)"
  fetch_pinned "$row" "$download/uv.tar.gz" || {
    rm -rf "$download"
    return 1
  }
  mkdir -p "$download/unpacked" "$LOCAL_BIN"
  tar -xzf "$download/uv.tar.gz" -C "$download/unpacked" --strip-components=1 || {
    rm -rf "$download"
    return 1
  }
  cp "$download/unpacked/uv" "$download/unpacked/uvx" "$LOCAL_BIN/" && chmod 755 "$LOCAL_BIN/uv" "$LOCAL_BIN/uvx"
  rm -rf "$download"
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
  apt-get) printf '%s\n' build-essential cmake ninja-build pkg-config uuid-dev curl ca-certificates git unzip xz-utils libicu-dev procps lldb ;;
  dnf) printf '%s\n' gcc gcc-c++ make cmake ninja-build pkgconf-pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu procps-ng findutils lldb ;;
  pacman) printf '%s\n' base-devel cmake ninja pkgconf util-linux-libs curl ca-certificates git unzip xz icu procps-ng lldb ;;
  zypper) printf '%s\n' gcc gcc-c++ make cmake ninja pkg-config libuuid-devel curl ca-certificates git unzip xz tar libicu-devel procps lldb python3-lldb ;;
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

ensure_rust_toolchain() {
  export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
  export PATH="$CARGO_HOME/bin:$PATH"
  if ! command -v rustup >/dev/null 2>&1; then
    local row download
    row="$(host_pin rustup)" || return 1
    log "Installing rustup $(pin_field "$row" version)…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/rustup-init" && chmod 755 "$download/rustup-init" && RUSTUP_INIT_SKIP_PATH_CHECK=yes "$download/rustup-init" -y --profile minimal --default-toolchain none || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  log "Installing the rust-toolchain.toml toolchain…"
  (cd "$REPO_ROOT" && rustup toolchain install) || return 1
}

ensure_node() {
  export PATH="$LOCAL_BIN:$PATH"
  local version row pinned archive root download
  version="$(cd "$REPO_ROOT" && bun -e 'process.stdout.write(require("./package.json").engines.node)')" || return 1
  if [ "$(node --version 2>/dev/null)" = "v$version" ]; then
    return 0
  fi
  row="$(host_pin node)" || return 1
  pinned="$(pin_field "$row" version)"
  [ "$pinned" = "$version" ] || {
    log "The Node.js pin $pinned in $PINS_FILE differs from package.json engines.node $version; update the pin."
    return 1
  }
  archive="$(pin_field "$row" url)"
  archive="${archive##*/}"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/node/v$version"
  if [ ! -x "$root/bin/node" ]; then
    log "Installing Node.js $version into $root…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/$archive" || {
      rm -rf "$download"
      return 1
    }
    mkdir -p "$root" && tar -xJf "$download/$archive" -C "$root" --strip-components=1 || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sf "$root/bin/node" "$LOCAL_BIN/node"
}

go_version() {
  go version 2>/dev/null | sed -nE 's/^go version go([0-9]+\.[0-9]+(\.[0-9]+)?).*/\1/p'
}

ensure_go() {
  export PATH="$LOCAL_BIN:$PATH"
  local row version installed archive root download
  row="$(host_pin go)" || return 1
  version="$(pin_field "$row" version)"
  installed="$(go_version)"
  if [ -n "$installed" ] && version_at_least "$installed" "$version"; then
    return 0
  fi
  archive="$(pin_field "$row" url)"
  archive="${archive##*/}"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/go/go$version"
  if [ ! -x "$root/go/bin/go" ]; then
    log "Installing Go $version into $root…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/$archive" || {
      rm -rf "$download"
      return 1
    }
    mkdir -p "$root" && tar -xzf "$download/$archive" -C "$root" || {
      rm -rf "$download"
      return 1
    }
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
  local row installer
  row="$(pin_row dotnet-install any any)"
  [ -n "$row" ] || {
    log "No pinned dotnet-install script in $PINS_FILE."
    return 1
  }
  log "Installing .NET SDK $DOTNET_CHANNEL into $DOTNET_ROOT…"
  installer="$(mktemp)"
  fetch_pinned "$row" "$installer" || {
    rm -f "$installer"
    return 1
  }
  bash "$installer" --channel "$DOTNET_CHANNEL" --install-dir "$DOTNET_ROOT" --no-path || {
    rm -f "$installer"
    return 1
  }
  rm -f "$installer"
  mkdir -p "$LOCAL_BIN"
  ln -sf "$DOTNET_ROOT/dotnet" "$LOCAL_BIN/dotnet"
}

ensure_browser_libraries() {
  [ "$(uname -s)" = Linux ] || return 0
  local cli="$REPO_ROOT/node_modules/playwright/cli.js"
  [ -f "$cli" ] || {
    log "Playwright is not installed; skipping the Chromium system libraries."
    return 0
  }
  if [ "$(linux_package_manager || true)" != apt-get ]; then
    log "Playwright installs the Chromium system libraries only through apt-get; install them with your package manager (https://playwright.dev/docs/browsers) before running browser tests."
    return 0
  fi
  run_privileged env "PATH=$PATH" node "$cli" install-deps chromium
}
#endregion 🔖️NativeToolchain

#region 🔖️EnsureBun
install_pinned_bun() {
  local version="$1" platform arch musl=false row pinned release archive sha download
  platform="$(host_platform)" || return 1
  arch="$(host_arch)" || return 1
  if host_is_musl; then musl=true; fi
  row="$(bun_release_row "$platform" "$arch" "$musl")"
  if [ -z "$row" ]; then
    log "No pinned Bun distribution for $platform/$arch (musl=$musl) in $BUN_PINS_FILE."
    return 1
  fi
  read -r pinned release archive sha <<<"$row"
  [ "$pinned" = "$version" ] || {
    log "The Bun pin $pinned in $BUN_PINS_FILE differs from package.json packageManager bun@$version; update both."
    return 1
  }
  download="$(mktemp -d)"
  download_verified "$release/$archive" "$sha" "$download/$archive" || {
    rm -rf "$download"
    return 1
  }
  mkdir -p "$BUN_INSTALL/bin"
  unzip -p "$download/$archive" "${archive%.zip}/bun" >"$download/bun" && chmod 755 "$download/bun" && mv -f "$download/bun" "$BUN_INSTALL/bin/bun" || {
    rm -rf "$download"
    return 1
  }
  rm -rf "$download"
}

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
    install_pinned_bun "$required_version" || return 1
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
  log "Installing the native repository command…"
  bun repo run repo-mcp:install || return 1
  export PATH="$LOCAL_BIN:$PATH"
  hash -r
  repo --help >/dev/null || return 1
  log "workspace:setup…"
  repo run workspace:setup || return 1
  repo run @semio-tech/repo-developer-tools:setup-debugger || return 1
  ensure_browser_libraries || log "The Chromium system libraries could not be installed; use the owned browser setup control with root privileges."

}
#endregion 🔖️RepoBootstrap

#region 🔖️Main
cd "$REPO_ROOT"
[ -f "$PINS_FILE" ] && [ -f "$BUN_PINS_FILE" ] || {
  log "Pinned toolchain manifests are missing; expected $PINS_FILE and $BUN_PINS_FILE."
  exit 1
}
ensure_native_toolchain
ensure_bun
ensure_node || log "Node.js provisioning failed; install the Node.js version pinned in package.json engines.node and rerun setup."
ensure_rust_toolchain || log "Rust toolchain provisioning failed; install rustup from https://rustup.rs and rerun setup."
ensure_go || log "Go provisioning failed; install the Go version pinned in $PINS_FILE from https://go.dev/dl and rerun setup."
ensure_dotnet || log ".NET SDK ${DOTNET_CHANNEL} provisioning failed; install it from https://dot.net and rerun setup."
ensure_uv || log "uv provisioning failed; install the uv version pinned in $PINS_FILE from https://docs.astral.sh/uv and rerun setup."
repo_bootstrap
log "Native (Unix) bootstrap complete. Open a new shell to load the updated PATH."
#endregion 🔖️Main
