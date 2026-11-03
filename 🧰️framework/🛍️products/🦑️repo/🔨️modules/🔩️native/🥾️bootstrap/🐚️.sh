#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew; pinned Zig when Linux cannot install packages), the Node.js
# the build tool runs on (`package.json` engines.node), rustup with the `rust-toolchain.toml` pin, .NET, and uv (the pinned Go that builds the launcher is fetched by the launcher installer from the Go rows of the same pins, and the repository provisions it into its tools partition afterwards),
# then the `repo` launcher built from the checkout (it provisions the pinned JavaScript runtime itself, so no Bun is installed here)
# and `workspace:setup` (which installs packages). Every downloaded installer is pinned and sha256-verified before it runs,
# from `🧬️schema/🔣️.json` next to this script; the runtime pins live in the launcher registry.
# 🧭️Invoke from a fresh clone: `bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup` (full
# bootstrap); `repo setup native` routes here once the launcher exists.
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
LOCAL_BIN="${HOME}/.local/bin"
PINS_FILE="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
LAUNCHER_INSTALL="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh"
LAUNCHER_REGISTRY="$REPO_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json"
LAUNCHER_BIN="${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin"

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
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$file" | awk '{print $1}')"
  elif command -v python3 >/dev/null 2>&1; then
    actual="$(python3 - "$file" <<'PY'
import hashlib
import sys

digest = hashlib.sha256()
with open(sys.argv[1], "rb") as source:
    for chunk in iter(lambda: source.read(1024 * 1024), b""):
        digest.update(chunk)
print(digest.hexdigest())
PY
)"
  else
    log "No SHA-256 tool is available."
    return 1
  fi
  [ "$actual" = "$expected" ] || {
    log "Checksum mismatch for $file (expected $expected, got $actual)."
    return 1
  }
}

download_verified() {
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fsSL --retry 3 --connect-timeout 30 --max-time 900 -o "$3" "$1" || return 1
  elif command -v python3 >/dev/null 2>&1; then
    python3 - "$1" "$3" <<'PY' || return 1
import ssl
import sys
import urllib.error
import urllib.request

url, dest = sys.argv[1], sys.argv[2]
if not url.startswith("https://"):
    raise SystemExit(1)

class HttpsOnly(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, response, code, message, headers, newurl):
        if not str(newurl).startswith("https://"):
            raise urllib.error.URLError("redirect left https")
        return super().redirect_request(request, response, code, message, headers, newurl)

context = ssl.create_default_context()
context.minimum_version = ssl.TLSVersion.TLSv1_2
opener = urllib.request.build_opener(HttpsOnly, urllib.request.HTTPSHandler(context=context))
temporary = dest + ".partial"
for _attempt in range(3):
    try:
        with opener.open(url, timeout=30) as response, open(temporary, "wb") as output:
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                output.write(chunk)
        import os
        os.replace(temporary, dest)
        raise SystemExit(0)
    except Exception:
        try:
            import os
            os.unlink(temporary)
        except FileNotFoundError:
            pass
raise SystemExit(1)
PY
  else
    return 1
  fi
  verify_sha256 "$3" "$2"
}

fetch_pinned() {
  local row="$1" destination="$2"
  download_verified "$(pin_field "$row" url)" "$(pin_field "$row" sha256)" "$destination"
}

version_at_least() {
  awk -v actual="$1" -v required="$2" 'BEGIN {
    actual_count = split(actual, actual_parts, ".")
    required_count = split(required, required_parts, ".")
    count = actual_count > required_count ? actual_count : required_count
    for (i = 1; i <= count; i++) {
      if (actual_parts[i] + 0 > required_parts[i] + 0) exit 0
      if (actual_parts[i] + 0 < required_parts[i] + 0) exit 1
    }
    exit 0
  }'
}

#endregion 🔖️PinnedDownloads

#region 🔖️EnsureUv
ensure_uv() {
  export PATH="$LOCAL_BIN:$PATH"
  local row version
  row="$(host_pin uv)" || return 1
  version="$(pin_field "$row" version)"
  if command -v uv >/dev/null 2>&1 && command -v uvx >/dev/null 2>&1 && version_at_least "$(uv --version | awk '{print $2}')" "$version"; then
    local name found
    for name in uv uvx; do
      found="$(command -v "$name")"
      case "$found" in
      /*) ;;
      *) continue ;;
      esac
      if [ "$found" != "$LOCAL_BIN/$name" ] && [ -x "$found" ]; then
        mkdir -p "$LOCAL_BIN"
        ln -sfn "$found" "$LOCAL_BIN/$name"
      fi
    done
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
  if ! tar -xzf "$download/uv.tar.gz" -C "$download/unpacked" --strip-components=1; then
    rm -rf "$download/unpacked"
    mkdir -p "$download/unpacked"
    command -v python3 >/dev/null 2>&1 || {
      rm -rf "$download"
      return 1
    }
    if ! python3 - "$download/uv.tar.gz" "$download/unpacked" <<'PY'
import gzip
import os
import sys
import tarfile

archive, dest = sys.argv[1], os.path.realpath(sys.argv[2])

def inside(path):
    path = os.path.normpath(path)
    if path != dest and not path.startswith(dest + os.sep):
        raise SystemExit(1)
    return path

with gzip.open(archive, "rb") as raw, tarfile.open(fileobj=raw, mode="r|") as tar:
    for member in tar:
        parts = member.name.split("/")
        if len(parts) <= 1:
            continue
        relative = parts[1:]
        if any(part in ("", ".", "..") for part in relative):
            raise SystemExit(1)
        target = inside(os.path.join(dest, *relative))
        if member.isdir():
            os.makedirs(target, exist_ok=True)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if not member.isreg():
            continue
        source = tar.extractfile(member)
        if source is None:
            continue
        with open(target, "wb") as output:
            output.write(source.read())
        os.chmod(target, member.mode & 0o777)
PY
    then
      rm -rf "$download"
      return 1
    fi
  fi
  cp "$download/unpacked/uv" "$download/unpacked/uvx" "$LOCAL_BIN/" && chmod 755 "$LOCAL_BIN/uv" "$LOCAL_BIN/uvx"
  rm -rf "$download"
}
#endregion 🔖️EnsureUv

#region 🔖️NativeToolchain
run_privileged() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1 && sudo -n true >/dev/null 2>&1; then
    sudo -n "$@"
  else
    log "Root privileges are not available for: $*"
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
  apt-get) printf '%s\n' build-essential pkg-config libpango1.0-dev libcairo2-dev fonts-liberation fonts-dejavu-extra fonts-noto fonts-noto-mono fonts-noto-color-emoji uuid-dev curl ca-certificates git unzip xz-utils libicu-dev procps lldb python3-lldb lsof xdg-utils libvulkan1 mesa-vulkan-drivers ;;
  dnf) printf '%s\n' gcc gcc-c++ make pkgconf-pkg-config pango-devel cairo-devel liberation-serif-fonts dejavu-serif-fonts google-noto-sans-fonts google-noto-serif-fonts google-noto-sans-mono-fonts google-noto-color-emoji-fonts libuuid-devel curl ca-certificates git unzip xz tar libicu-devel procps-ng findutils lldb python3-lldb lsof xdg-utils vulkan-loader mesa-vulkan-drivers ;;
  pacman) printf '%s\n' base-devel pkgconf pango cairo ttf-liberation ttf-dejavu noto-fonts noto-fonts-emoji util-linux-libs curl ca-certificates git unzip xz icu procps-ng lldb lsof xdg-utils vulkan-icd-loader vulkan-swrast ;;
  zypper) printf '%s\n' gcc gcc-c++ make pkg-config pango-devel cairo-devel liberation-fonts dejavu-fonts libuuid-devel curl ca-certificates git unzip xz tar libicu-devel procps lldb python3-lldb lsof xdg-utils libvulkan1 libvulkan_lvp ;;
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


ensure_user_cc() {
  export PATH="$LOCAL_BIN:$PATH"
  local pin_row="" pin_version="" pin_zig=""
  if pin_row="$(host_pin zig)"; then
    pin_version="$(pin_field "$pin_row" version)"
    pin_zig="${XDG_DATA_HOME:-$HOME/.local/share}/semio/zig/v$pin_version/zig"
    if [ -x "$pin_zig" ]; then
      mkdir -p "$LOCAL_BIN"
      ln -sfn "$pin_zig" "$LOCAL_BIN/zig"
    fi
  fi
  if command -v cc >/dev/null 2>&1 && command -v c++ >/dev/null 2>&1; then
    local cc_path cxx_path first="" line=""
    cc_path=$(command -v cc)
    cxx_path=$(command -v c++)
    case "$cc_path:$cxx_path" in
    "$LOCAL_BIN"/*:"$LOCAL_BIN"/*)
      local n=0
      while IFS= read -r line && [ "$n" -lt 40 ]; do first="$first$line"; n=$((n+1)); done < "$cc_path"
      n=0
      while IFS= read -r line && [ "$n" -lt 40 ]; do first="$first$line"; n=$((n+1)); done < "$cxx_path"
      case "$first" in
      '#!'*'-target'*)
        local expected=""
        case "$(uname -s):$(uname -m)" in
        Linux:x86_64|Linux:amd64) expected=x86_64-linux-gnu ;;
        Linux:aarch64|Linux:arm64) expected=aarch64-linux-gnu ;;
        Darwin:x86_64|Darwin:amd64) expected=x86_64-macos ;;
        Darwin:aarch64|Darwin:arm64) expected=aarch64-macos ;;
        esac
        if [ -z "$expected" ] || case "$first" in *"$expected"*) true ;; *) false ;; esac; then
          return 0
        fi
        ;;
      '#!'*) ;;
      *) return 0 ;;
      esac
      ;;
    *) return 0 ;;
    esac
  fi
  local row version root archive download zig name mode
  row="$(host_pin zig)" || return 1
  version="$(pin_field "$row" version)"
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/zig/v$version"
  zig="$root/zig"
  if [ ! -x "$zig" ]; then
    log "Installing Zig $version as the user-local C and C++ compiler…"
    download="$(mktemp -d)"
    archive="$(pin_field "$row" url)"
    archive="${archive##*/}"
    fetch_pinned "$row" "$download/$archive" || {
      rm -rf "$download"
      return 1
    }
    unpack_tar_xz "$download/$archive" "$root" || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  for name in cc gcc c++ g++; do
    case "$name" in
    c++ | g++) mode=c++ ;;
    *) mode=cc ;;
    esac
    cat > "$LOCAL_BIN/$name" <<EOF
#!/bin/sh
target=
case "\$(uname -s):\$(uname -m)" in
Linux:x86_64|Linux:amd64) target=x86_64-linux-gnu ;;
Linux:aarch64|Linux:arm64) target=aarch64-linux-gnu ;;
Darwin:x86_64|Darwin:amd64) target=x86_64-macos ;;
Darwin:aarch64|Darwin:arm64) target=aarch64-macos ;;
*) exec $(printf '%q' "$zig") $mode "\$@" ;;
esac
for arg in "\$@"; do
  case "\$arg" in
  -target|-target=*|--target|--target=*) exec $(printf '%q' "$zig") $mode "\$@" ;;
  esac
done
exec $(printf '%q' "$zig") $mode -target "\$target" "\$@"
EOF
    chmod 755 "$LOCAL_BIN/$name"
  done
  if [ -x "$zig" ]; then
    mkdir -p "$LOCAL_BIN"
    ln -sfn "$zig" "$LOCAL_BIN/zig"
  fi
}

ensure_user_pkg_config() {
  export PATH="$LOCAL_BIN:$PATH"
  local pkg_path
  if command -v pkg-config >/dev/null 2>&1; then
    pkg_path=$(command -v pkg-config)
    case "$pkg_path" in
    "$LOCAL_BIN"/*)
      if [ "$(pkg-config --version 2>/dev/null || true)" = "0.7.0" ]; then
        return 0
      fi
      ;;
    *) return 0 ;;
    esac
  fi
  mkdir -p "$LOCAL_BIN"
  cat > "$LOCAL_BIN/pkg-config" <<'EOF'
#!/bin/sh
set -eu
tool_version=0.7.0
if [ "${1:-}" = "--version" ]; then
  printf '%s\n' "$tool_version"
  exit 0
fi
mode=""
static=0
packages=""
defines=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --exists) mode=exists ;;
    --cflags) mode=cflags ;;
    --cflags-only-I) mode=cflags-only-I ;;
    --cflags-only-other) mode=cflags-only-other ;;
    --libs) mode=libs ;;
    --libs-only-l) mode=libs-only-l ;;
    --libs-only-L) mode=libs-only-L ;;
    --libs-only-other) mode=libs-only-other ;;
    --modversion) mode=modversion ;;
    --static) static=1 ;;
    --print-errors|--short-errors|--silence-errors) ;;
    --define-variable=*) defines="${defines}
${1#--define-variable=}" ;;
    --variable=*) mode="variable:${1#--variable=}" ;;
    -*) printf 'pkg-config: unsupported option %s\n' "$1" >&2; exit 1 ;;
    *) packages="${packages} $1" ;;
  esac
  shift
done
[ -n "$mode" ] || exit 1
[ -n "${packages# }" ] || exit 1
if [ -n "${PKG_CONFIG_LIBDIR+x}" ]; then
  search="${PKG_CONFIG_PATH:+$PKG_CONFIG_PATH:}${PKG_CONFIG_LIBDIR}"
else
  multiarch=
  case "$(uname -m)" in
  x86_64|amd64) multiarch=/usr/lib/x86_64-linux-gnu/pkgconfig ;;
  aarch64|arm64) multiarch=/usr/lib/aarch64-linux-gnu/pkgconfig ;;
  esac
  homebrew=
  case "$(uname -s)" in
  Darwin) homebrew=/opt/homebrew/lib/pkgconfig:/opt/homebrew/share/pkgconfig ;;
  esac
  search="${PKG_CONFIG_PATH:+$PKG_CONFIG_PATH:}${HOME}/.local/lib/pkgconfig:${homebrew:+$homebrew:}/usr/local/lib/pkgconfig:/usr/local/share/pkgconfig:/usr/lib/pkgconfig:${multiarch:+$multiarch:}/usr/lib64/pkgconfig:/usr/share/pkgconfig"
fi
SEMIO_PKG_CONFIG_MODE="$mode" SEMIO_PKG_CONFIG_STATIC="$static" SEMIO_PKG_CONFIG_PACKAGES="$packages" SEMIO_PKG_CONFIG_SEARCH="$search" SEMIO_PKG_CONFIG_DEFINES="$defines" awk '
function trim(value) { sub(/^[ \t]+/, "", value); sub(/[ \t]+$/, "", value); return value }
function expand(value, name, out) {
  out = ""
  while (match(value, /\$\{[A-Za-z_][A-Za-z0-9_]*\}/)) {
    name = substr(value, RSTART + 2, RLENGTH - 3)
    out = out substr(value, 1, RSTART - 1) ((name in vars) ? vars[name] : "")
    value = substr(value, RSTART + RLENGTH)
  }
  return out value
}
function locate(name, parts, count, slot, candidate) {
  if (name in located) return located[name]
  count = split(search, parts, ":")
  for (slot = 1; slot <= count; slot++) {
    if (parts[slot] == "") continue
    candidate = parts[slot] "/" name ".pc"
    if ((getline probe < candidate) >= 0) { close(candidate); located[name] = candidate; return candidate }
  }
  return ""
}
function version_ok(actual, op, required, left, right, position, count, actual_part, required_part) {
  count = split(actual, left, ".")
  if (split(required, right, ".") > count) count = split(required, right, ".")
  for (position = 1; position <= count; position++) {
    actual_part = left[position] + 0
    required_part = right[position] + 0
    if (actual_part == required_part) continue
    if (op == "=" || op == "==") return 0
    if (op == "!=") return 1
    if (op == ">" || op == ">=") return actual_part > required_part
    if (op == "<" || op == "<=") return actual_part < required_part
    return 0
  }
  if (op == "!=" || op == ">" || op == "<") return 0
  return 1
}
function load(name, path, raw, line, key, value, separator, requires, partcount, part, slot, dep, op, required, target) {
  if (name in loaded) return
  path = locate(name)
  if (path == "") { missing = missing " " name; return }
  requires = ""
  while ((getline raw < path) > 0) {
    line = raw
    sub(/[ \t]*#.*/, "", line)
    if (line ~ /^[ \t]*$/) continue
    if (line ~ /^[A-Za-z0-9_.]+[ \t]*:/) separator = index(line, ":")
    else if (index(line, "=") > 0) separator = index(line, "=")
    else continue
    key = trim(substr(line, 1, separator - 1))
    value = expand(trim(substr(line, separator + 1)))
    if (!(key in defined)) vars[key] = value
    packagevars[name, key] = (key in defined) ? defined[key] : value
    if (key == "Requires" || (staticmode && key == "Requires.private")) requires = requires " " value
    if (key == "Version") fields[name, key] = value
    else if (key == "Cflags" || key == "Libs" || (staticmode && (key == "Cflags.private" || key == "Libs.private"))) {
      target = key
      sub(/\.private$/, "", target)
      fields[name, target] = fields[name, target] (fields[name, target] == "" ? "" : " ") value
    }
  }
  close(path)
  loaded[name] = 1
  partcount = split(requires, parts, /[ \t,]+/)
  slot = 1
  while (slot <= partcount) {
    if (parts[slot] == "" || parts[slot] ~ /[<>=!]/) { slot++; continue }
    dep = parts[slot]
    op = ""
    required = ""
    if (slot + 2 <= partcount && parts[slot + 1] ~ /^(>=|<=|!=|==|=|>|<)$/) {
      op = parts[slot + 1]
      required = parts[slot + 2]
      slot += 3
    } else slot++
    load(dep)
    if (!(dep in loaded)) continue
    if (op != "" && !version_ok(fields[dep, "Version"], op, required)) { missing = missing " " dep; continue }
    deps[name] = deps[name] " " dep
  }
}
function wanted(token, mode) {
  if (mode == "cflags" || mode == "libs") return 1
  if (mode == "cflags-only-I") return token ~ /^(-I|-isystem)/
  if (mode == "cflags-only-other") return token !~ /^(-I|-isystem)/
  if (mode == "libs-only-l") return token ~ /^-l/
  if (mode == "libs-only-L") return token ~ /^-L/
  if (mode == "libs-only-other") return token !~ /^(-l|-L)/
  return 0
}
function emit(name, kind, slot, partcount, part) {
  if (name in emitted) return
  emitted[name] = 1
  partcount = split(deps[name], parts, /[ \t]+/)
  for (slot = 1; slot <= partcount; slot++) if (parts[slot] != "") emit(parts[slot], kind)
  if (fields[name, kind] != "") pieces[++piececount] = fields[name, kind]
}
BEGIN {
  staticmode = ENVIRON["SEMIO_PKG_CONFIG_STATIC"] == "1"
  search = ENVIRON["SEMIO_PKG_CONFIG_SEARCH"]
  defcount = split(ENVIRON["SEMIO_PKG_CONFIG_DEFINES"], deflines, "\n")
  for (slot = 1; slot <= defcount; slot++) {
    if (deflines[slot] == "") continue
    sep = index(deflines[slot], "=")
    if (sep < 2) continue
    name = substr(deflines[slot], 1, sep - 1)
    defined[name] = substr(deflines[slot], sep + 1)
    vars[name] = defined[name]
  }
  count = split(ENVIRON["SEMIO_PKG_CONFIG_PACKAGES"], names, /[ \t]+/)
  for (slot = 1; slot <= count; slot++) if (names[slot] != "") load(names[slot])
  if (missing != "") exit 1
  mode = ENVIRON["SEMIO_PKG_CONFIG_MODE"]
  if (mode == "exists") exit 0
  if (mode == "modversion") {
    for (slot = 1; slot <= count; slot++) if (names[slot] != "") print fields[names[slot], "Version"]
    exit 0
  }
  if (index(mode, "variable:") == 1) {
    key = substr(mode, 10)
    for (slot = 1; slot <= count; slot++) if (names[slot] != "") { print packagevars[names[slot], key]; exit 0 }
    exit 0
  }
  kind = (mode == "cflags" || index(mode, "cflags-only-") == 1) ? "Cflags" : "Libs"
  for (slot = 1; slot <= count; slot++) if (names[slot] != "") emit(names[slot], kind)
  out = ""
  for (slot = 1; slot <= piececount; slot++) {
    tokencount = split(pieces[slot], tokens, /[ \t]+/)
    for (token = 1; token <= tokencount; token++) if (tokens[token] != "" && wanted(tokens[token], mode)) out = out (out == "" ? "" : " ") tokens[token]
  }
  if (out != "") print out
}
'
EOF
  chmod 755 "$LOCAL_BIN/pkg-config"
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
      brew install pkg-config cairo pango || log "Homebrew could not install pkg-config, Cairo and Pango."
      brew install --cask font-dejavu || log "Homebrew could not install DejaVu mathematical fonts."
    else
      log "Homebrew not available; install pkg-config, Cairo, Pango and DejaVu fonts before running native C++ builds (CMake and Ninja are provisioned by the repository)."
    fi
    export PATH="$LOCAL_BIN:$PATH"
    ensure_user_cc || log "User-local C compiler provisioning failed; install a C compiler and rerun setup."
    ensure_user_pkg_config || log "User-local pkg-config provisioning failed; install pkg-config and rerun setup."
    export CMAKE_PRESET="${CMAKE_PRESET:-macos-prototype}"
    ;;
  Linux)
    local manager packages=() package
    if manager="$(linux_package_manager)"; then
      while IFS= read -r package; do packages+=("$package"); done < <(linux_toolchain_packages "$manager")
      log "Installing the native toolchain with $manager: ${packages[*]}"
      install_linux_packages "$manager" "${packages[@]}" || log "The native toolchain install through $manager failed (package index, lock or privileges); setup continues and reports what is missing."
    else
      log "No supported package manager (apt-get, dnf, pacman, zypper) found; install a C/C++ compiler, make, pkg-config, libuuid, ICU, curl, git and unzip (CMake and Ninja are provisioned by the repository)."
    fi
    export PATH="$LOCAL_BIN:$PATH"
    ensure_user_cc || log "User-local C compiler provisioning failed; install a C compiler and rerun setup."
    ensure_user_pkg_config || log "User-local pkg-config provisioning failed; install pkg-config and rerun setup."
    export CMAKE_PRESET="${CMAKE_PRESET:-linux-prototype}"
    ;;
  esac
  report_missing_tools cc c++ make pkg-config curl git unzip
}

ensure_rust_toolchain() {
  export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
  export PATH="$CARGO_HOME/bin:$PATH"
  if ! command -v rustup >/dev/null 2>&1; then
    local row download
    row="$(host_pin rustup)" || return 1
    log "Installing rustup $(pin_field "$row" version)…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/rustup-init" && chmod 755 "$download/rustup-init" && RUSTUP_INIT_SKIP_PATH_CHECK=yes "$download/rustup-init" -y --no-modify-path --profile minimal --default-toolchain none || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  log "Installing the rust-toolchain.toml toolchain…"
  local previous_rustup_toolchain=""
  if [ -n "${RUSTUP_TOOLCHAIN+x}" ]; then
    previous_rustup_toolchain="$RUSTUP_TOOLCHAIN"
  fi
  unset RUSTUP_TOOLCHAIN
  if ! (cd "$REPO_ROOT" && rustup toolchain install --no-self-update); then
    if [ -n "$previous_rustup_toolchain" ]; then
      export RUSTUP_TOOLCHAIN="$previous_rustup_toolchain"
    fi
    return 1
  fi
  if [ -n "$previous_rustup_toolchain" ]; then
    export RUSTUP_TOOLCHAIN="$previous_rustup_toolchain"
  fi
}

unpack_tar_xz() {
  local archive="$1" dest="$2"
  mkdir -p "$dest"
  if tar -xJf "$archive" -C "$dest" --strip-components=1; then
    return 0
  fi
  rm -rf "$dest"
  mkdir -p "$dest"
  command -v python3 >/dev/null 2>&1 || return 1
  if ! python3 - "$archive" "$dest" <<'PY'
import lzma
import os
import sys
import tarfile

archive, dest = sys.argv[1], os.path.realpath(sys.argv[2])

def inside(path):
    path = os.path.normpath(path)
    if path != dest and not path.startswith(dest + os.sep):
        raise SystemExit(1)
    return path

with lzma.open(archive) as raw, tarfile.open(fileobj=raw, mode="r|") as tar:
    for member in tar:
        parts = member.name.split("/")
        if len(parts) <= 1:
            continue
        relative = parts[1:]
        if any(part in ("", ".", "..") for part in relative):
            raise SystemExit(1)
        target = inside(os.path.join(dest, *relative))
        if member.isdir():
            os.makedirs(target, exist_ok=True)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if member.issym():
            link = member.linkname
            if link.startswith("/") or os.path.isabs(link):
                raise SystemExit(1)
            inside(os.path.join(os.path.dirname(target), link))
            if os.path.lexists(target):
                os.unlink(target)
            os.symlink(link, target)
            continue
        if not member.isreg():
            continue
        source = tar.extractfile(member)
        if source is None:
            continue
        with open(target, "wb") as output:
            output.write(source.read())
        os.chmod(target, member.mode & 0o777)
PY
  then
    rm -rf "$dest"
    return 1
  fi
}

ensure_node() {
  export PATH="$LOCAL_BIN:$PATH"
  local version row pinned archive root download name
  version="$(awk -F'"' '/"engines"/ { engines = 1 } engines && /"node"[[:space:]]*:/ { print $4; exit }' "$REPO_ROOT/package.json")"
  [ -n "$version" ] || return 1
  root="${XDG_DATA_HOME:-$HOME/.local/share}/semio/node/v$version"
  if [ "$(node --version 2>/dev/null)" = "v$version" ] && command -v npm >/dev/null 2>&1; then
    local name found
    for name in node npm npx corepack; do
      command -v "$name" >/dev/null 2>&1 || continue
      found="$(command -v "$name")"
      case "$found" in
      /*) ;;
      *) continue ;;
      esac
      if [ "$found" != "$LOCAL_BIN/$name" ] && [ -x "$found" ]; then
        mkdir -p "$LOCAL_BIN"
        ln -sfn "$found" "$LOCAL_BIN/$name"
      fi
    done
    return 0
  fi
  if [ ! -x "$root/bin/node" ] || [ ! -x "$root/bin/npm" ]; then
    rm -rf "$root"
    row="$(host_pin node)" || return 1
    pinned="$(pin_field "$row" version)"
    [ "$pinned" = "$version" ] || {
      log "The Node.js pin $pinned in $PINS_FILE differs from package.json engines.node $version; update the pin."
      return 1
    }
    archive="$(pin_field "$row" url)"
    archive="${archive##*/}"
    log "Installing Node.js $version into $root…"
    download="$(mktemp -d)"
    fetch_pinned "$row" "$download/$archive" || {
      rm -rf "$download"
      return 1
    }
    unpack_tar_xz "$download/$archive" "$root" || {
      rm -rf "$download"
      return 1
    }
    rm -rf "$download"
  fi
  mkdir -p "$LOCAL_BIN"
  for name in node npm npx corepack; do
    if [ -x "$root/bin/$name" ]; then
      ln -sfn "$root/bin/$name" "$LOCAL_BIN/$name"
    fi
  done
}

dotnet_sdk_pin() {
  sed -n 's/^[[:space:]]*"version"[[:space:]]*:[[:space:]]*"\([0-9][0-9.]*\)".*/\1/p' "$REPO_ROOT/global.json" | sed -n '1p'
}

provide_tar_gzip() {
  command -v python3 >/dev/null 2>&1 || return 0
  local probe archive dest real dir
  probe="$(mktemp -d)"
  archive="$probe/a.tar.gz"
  dest="$probe/out"
  mkdir -p "$dest"
  if ! python3 - "$archive" <<'PY'
import gzip
import io
import sys
import tarfile

buffer = io.BytesIO()
with tarfile.open(fileobj=buffer, mode="w") as tar:
    payload = b"ok"
    info = tarfile.TarInfo("f")
    info.size = len(payload)
    tar.addfile(info, io.BytesIO(payload))
with gzip.open(sys.argv[1], "wb") as output:
    output.write(buffer.getvalue())
PY
  then
    rm -rf "$probe"
    return 1
  fi
  if tar -xzf "$archive" -C "$dest" >/dev/null 2>&1 && [ -f "$dest/f" ]; then
    rm -rf "$probe"
    return 0
  fi
  rm -rf "$probe"
  real="$(command -v tar 2>/dev/null || true)"
  dir="$(mktemp -d)"
  cat > "$dir/tar" <<'EOF'
#!/bin/sh
if [ "$1" = "-xzf" ] && [ "$3" = "-C" ] && [ -n "$4" ]; then
  if [ -n "${SEMIO_REAL_TAR:-}" ] && "$SEMIO_REAL_TAR" "$@"; then
    exit 0
  fi
  exec python3 - "$2" "$4" <<'PY'
import gzip
import os
import sys
import tarfile

archive, dest = sys.argv[1], os.path.realpath(sys.argv[2])
os.makedirs(dest, exist_ok=True)

def inside(path):
    path = os.path.normpath(path)
    if path != dest and not path.startswith(dest + os.sep):
        raise SystemExit(1)
    return path

with gzip.open(archive, "rb") as raw, tarfile.open(fileobj=raw, mode="r|") as tar:
    for member in tar:
        parts = [part for part in member.name.split("/") if part != ""]
        if not parts or any(part in (".", "..") for part in parts):
            raise SystemExit(1)
        target = inside(os.path.join(dest, *parts))
        if member.isdir():
            os.makedirs(target, exist_ok=True)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if member.issym():
            link = member.linkname
            if link.startswith("/") or os.path.isabs(link):
                raise SystemExit(1)
            inside(os.path.join(os.path.dirname(target), link))
            if os.path.lexists(target):
                os.unlink(target)
            os.symlink(link, target)
            continue
        if not member.isreg():
            continue
        source = tar.extractfile(member)
        if source is None:
            continue
        with open(target, "wb") as output:
            output.write(source.read())
        os.chmod(target, member.mode & 0o777)
PY
fi
if [ -n "${SEMIO_REAL_TAR:-}" ]; then
  exec "$SEMIO_REAL_TAR" "$@"
fi
printf '%s\n' 'tar: cannot read gzip' >&2
exit 1
EOF
  chmod 755 "$dir/tar"
  export SEMIO_REAL_TAR="$real"
  export PATH="$dir:$PATH"
}

provide_curl() {
  command -v curl >/dev/null 2>&1 && return 0
  command -v python3 >/dev/null 2>&1 || return 1
  local dir
  dir="$(mktemp -d)"
  cat > "$dir/curl" <<'EOF'
#!/bin/sh
exec python3 - "$@" <<'PY'
import os
import ssl
import sys
import urllib.error
import urllib.request

args = sys.argv[1:]
silent = False
show_error = False
fail = False
head = False
follow = False
output = None
create_dirs = False
retries = 0
timeout = 30
url = None
index = 0
while index < len(args):
    arg = args[index]
    if arg == "--":
        index += 1
        if index < len(args):
            url = args[index]
        break
    if arg.startswith("--"):
        name, separator, value = arg.partition("=")
        def take():
            global index
            if separator:
                return value
            index += 1
            return args[index]
        if name == "--fail":
            fail = True
        elif name == "--location":
            follow = True
        elif name == "--silent":
            silent = True
        elif name == "--show-error":
            show_error = True
        elif name == "--head":
            head = True
        elif name == "--create-dirs":
            create_dirs = True
        elif name == "--retry":
            retries = int(take())
        elif name in {"--retry-delay", "--max-time"}:
            take()
        elif name == "--connect-timeout":
            timeout = int(float(take()))
        elif name == "--output":
            output = take()
        elif name in {"--proto", "--proto-redir"}:
            take()
        index += 1
        continue
    if arg.startswith("-") and len(arg) > 1:
        flag = 1
        while flag < len(arg):
            letter = arg[flag]
            if letter == "o":
                rest = arg[flag + 1 :]
                output = rest if rest else args[index + 1]
                if not rest:
                    index += 1
                break
            if letter == "s":
                silent = True
            elif letter == "S":
                show_error = True
            elif letter == "f":
                fail = True
            elif letter == "L":
                follow = True
            elif letter == "I":
                head = True
            else:
                raise SystemExit(2)
            flag += 1
        index += 1
        continue
    url = arg
    index += 1

if not url or not url.startswith("https://"):
    raise SystemExit(2)

class HttpsRedirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, response, code, message, headers, newurl):
        if not follow or not str(newurl).startswith("https://"):
            raise urllib.error.URLError("redirect left https")
        return super().redirect_request(request, response, code, message, headers, newurl)

context = ssl.create_default_context()
context.minimum_version = ssl.TLSVersion.TLSv1_2
opener = urllib.request.build_opener(HttpsRedirects, urllib.request.HTTPSHandler(context=context))
status = 1
for _attempt in range(retries + 1):
    try:
        request = urllib.request.Request(url, method="HEAD" if head else "GET")
        with opener.open(request, timeout=timeout) as response:
            if head:
                sys.stdout.write(f"HTTP/1.1 {response.status} OK\n")
                for key, value in response.headers.items():
                    sys.stdout.write(f"{key}: {value}\n")
            else:
                body = response.read()
                if output:
                    if create_dirs:
                        os.makedirs(os.path.dirname(output) or ".", exist_ok=True)
                    with open(output, "wb") as handle:
                        handle.write(body)
                else:
                    sys.stdout.buffer.write(body)
        status = 0
        break
    except urllib.error.HTTPError as error:
        status = 22 if fail else 0
        if status == 0 and not head:
            body = error.read()
            if output:
                with open(output, "wb") as handle:
                    handle.write(body)
            else:
                sys.stdout.buffer.write(body)
        if status == 0:
            break
    except TimeoutError:
        status = 28
    except urllib.error.URLError as error:
        reason = getattr(error, "reason", error)
        status = 28 if isinstance(reason, TimeoutError) else 7
    except Exception:
        status = 1
if status != 0 and (not silent or show_error):
    sys.stderr.write(f"curl: ({status}) download failed\n")
raise SystemExit(status)
PY
EOF
  chmod 755 "$dir/curl"
  export PATH="$dir:$PATH"
}

ensure_dotnet() {
  export DOTNET_ROOT="${DOTNET_ROOT:-$HOME/.dotnet}" DOTNET_CLI_TELEMETRY_OPTOUT=1
  export PATH="$LOCAL_BIN:$PATH"
  local row installer sdk
  sdk="$(dotnet_sdk_pin)"
  [ -n "$sdk" ] || {
    log "No SDK version pinned in $REPO_ROOT/global.json."
    return 1
  }
  if command -v dotnet >/dev/null 2>&1 && dotnet --list-sdks 2>/dev/null | grep -q "^${sdk} "; then
    mkdir -p "$LOCAL_BIN"
    if [ "$(command -v dotnet)" != "$LOCAL_BIN/dotnet" ]; then
      ln -sfn "$(command -v dotnet)" "$LOCAL_BIN/dotnet"
    fi
    return 0
  fi
  row="$(pin_row dotnet-install any any)"
  [ -n "$row" ] || {
    log "No pinned dotnet-install script in $PINS_FILE."
    return 1
  }
  log "Installing .NET SDK $sdk into $DOTNET_ROOT…"
  installer="$(mktemp)"
  fetch_pinned "$row" "$installer" || {
    rm -f "$installer"
    return 1
  }
  provide_curl || {
    rm -f "$installer"
    return 1
  }
  provide_tar_gzip || {
    rm -f "$installer"
    return 1
  }
  bash "$installer" --version "$sdk" --install-dir "$DOTNET_ROOT" --no-path || {
    rm -f "$installer"
    return 1
  }
  rm -f "$installer"
  mkdir -p "$LOCAL_BIN"
  ln -sf "$DOTNET_ROOT/dotnet" "$LOCAL_BIN/dotnet"
}

#endregion 🔖️NativeToolchain

#region 🔖️EnsureLauncher
link_user_tool() {
  local source="$1" name="$2"
  if [ ! -x "$source" ]; then
    return 0
  fi
  mkdir -p "$LOCAL_BIN"
  ln -sfn "$source" "$LOCAL_BIN/$name"
}

link_pinned_go() {
  local row version sha platform arch share partition source
  row="$(host_pin go)" || return 0
  version="$(pin_field "$row" version)"
  sha="$(pin_field "$row" sha256)"
  platform="$(host_platform)" || return 0
  arch="$(host_arch)" || return 0
  share="${XDG_DATA_HOME:-$HOME/.local/share}/semio/go/v$version/bin/go"
  partition="$REPO_ROOT/.cache/b/t/m/native/go/$version/$platform-$arch-${sha:0:16}/bin/go"
  if [ -x "$share" ]; then
    source="$share"
  elif [ -x "$partition" ]; then
    source="$partition"
  elif command -v go >/dev/null 2>&1; then
    source="$(command -v go)"
    if [ "$source" = "$LOCAL_BIN/go" ]; then
      return 0
    fi
  else
    return 0
  fi
  link_user_tool "$source" go
  link_user_tool "$(dirname "$source")/gofmt" gofmt
}

link_installed_rust() {
  local name found
  link_user_tool "$CARGO_HOME/bin/rustup" rustup
  link_user_tool "$CARGO_HOME/bin/cargo" cargo
  link_user_tool "$CARGO_HOME/bin/rustc" rustc
  for name in rustup cargo rustc; do
    if [ -x "$LOCAL_BIN/$name" ]; then
      continue
    fi
    command -v "$name" >/dev/null 2>&1 || continue
    found="$(command -v "$name")"
    if [ "$found" != "$LOCAL_BIN/$name" ]; then
      link_user_tool "$found" "$name"
    fi
  done
}

ensure_launcher() {
  export PATH="$LOCAL_BIN:$PATH"
  log "Building the repo launcher (it provisions the pinned JavaScript runtime itself)…"
  bash "$LAUNCHER_INSTALL" "$LAUNCHER_BIN" || return 1
  export PATH="$LAUNCHER_BIN:$PATH"
  hash -r
  [ -x "$LAUNCHER_BIN/repo" ] || {
    log "The repo launcher was not installed into $LAUNCHER_BIN."
    return 1
  }
  link_user_tool "$LAUNCHER_BIN/repo" repo
}
#endregion 🔖️EnsureLauncher

#region 🔖️RepoBootstrap
repo_bootstrap() {
  if [ "$SKIP_REPO_BOOTSTRAP" = "1" ]; then
    log "Skipping repo bootstrap (SKIP_REPO_BOOTSTRAP=1)."
    return 0
  fi
  cd "$REPO_ROOT"
  export SEMIO_WORKSPACE_ROOT="$REPO_ROOT"
  log "Installing the native repository command…"
  repo run repo-mcp:install || return 1
  export PATH="$LOCAL_BIN:$PATH"
  hash -r
  repo --help >/dev/null || return 1
  log "workspace:setup…"
  repo run workspace:setup || return 1
  repo run @semio-tech/repo-developer-tools:setup-debugger || log "Native debugger setup needs noninteractive install privileges or a working LLDB already on PATH. Setup continues."

}
#endregion 🔖️RepoBootstrap

#region 🔖️Main
cd "$REPO_ROOT"
[ -f "$PINS_FILE" ] && [ -f "$LAUNCHER_REGISTRY" ] && [ -f "$LAUNCHER_INSTALL" ] || {
  log "Pinned toolchain manifests are missing; expected $PINS_FILE, $LAUNCHER_REGISTRY and $LAUNCHER_INSTALL."
  exit 1
}
persist_linux_session() {
  [ "$(uname -s)" = Linux ] || return 0
  local dir file playwright
  playwright="$REPO_ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
  mkdir -p "$playwright"
  export CMAKE_PRESET="${CMAKE_PRESET:-linux-prototype}"
  export PLAYWRIGHT_BROWSERS_PATH="$playwright"
  export DOTNET_CLI_TELEMETRY_OPTOUT=1
  dir="${XDG_CONFIG_HOME:-$HOME/.config}/environment.d"
  mkdir -p "$dir"
  file="$dir/semio.conf"
  cat > "$file" <<EOF
CMAKE_PRESET=${CMAKE_PRESET}
PLAYWRIGHT_BROWSERS_PATH="${playwright}"
DOTNET_CLI_TELEMETRY_OPTOUT=1
EOF
}

if host_is_musl; then
  log "This Linux uses musl. The pinned Node.js, uv, rustup and Go archives are glibc builds and are not installed."
  exit 1
fi
persist_linux_session
ensure_native_toolchain
ensure_launcher || exit 1
link_pinned_go
ensure_node || {
  log "Node.js provisioning failed; install the Node.js version pinned in package.json engines.node and rerun setup."
  exit 1
}
ensure_rust_toolchain || {
  log "Rust toolchain provisioning failed; install rustup from https://rustup.rs and rerun setup."
  exit 1
}
link_installed_rust
ensure_dotnet || {
  log ".NET SDK provisioning failed; install the SDK version pinned in $REPO_ROOT/global.json from https://dot.net and rerun setup."
  exit 1
}
ensure_uv || {
  log "uv provisioning failed; install the uv version pinned in $PINS_FILE from https://docs.astral.sh/uv and rerun setup."
  exit 1
}
repo_bootstrap
log "Native (Unix) bootstrap complete. Commands are linked into $LOCAL_BIN."
#endregion 🔖️Main
