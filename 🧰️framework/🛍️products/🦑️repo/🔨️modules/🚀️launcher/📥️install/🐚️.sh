#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Builds and installs the `repo` launcher on macOS, Linux and the devcontainer: the one small native program that provisions the pinned JavaScript runtime
# itself and starts the repository implementation, so that neither `bun` nor a Go toolchain has to be installed by hand.
# Stage 0 of a bare machine: the pinned Go toolchain is found (PATH, the repository tools partition, the user cache) or fetched from the pins in
# `🔩️native/🥾️bootstrap/🧬️schema/🔣️.json` (https only, SHA-256 checked before it is unpacked, one atomic rename, progress, Ctrl+C leaves nothing); curl, tar and a SHA-256 tool are used when they exist; Python 3 covers a missing downloader, a missing checksum tool and a tar that cannot read gzip.
# Usage: bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh" [destination-directory]
# The default destination is ${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin; put it on PATH. Run it again after pulling to rebuild.
# Environment: SEMIO_GO_ROOT (user cache of the toolchain, default ${XDG_DATA_HOME:-$HOME/.local/share}/semio/go), SEMIO_GO_MIRROR (https or loopback http directory that holds the pinned archive).
# Der Launcher baut sich aus dem Checkout; das gepinnte Go holt sich das Skript bei Bedarf selbst (nur https, SHA-256 vor dem Entpacken). Zielverzeichnis optional, Standard siehe oben.
# #endregion 🔖️Header
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
module="$here/../📦️packages/🐹️go"
root="$(cd "$here/../../../../../.." && pwd)"
pins="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
destination="${1:-${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin}"
go_home="${SEMIO_GO_ROOT:-${XDG_DATA_HOME:-$HOME/.local/share}/semio/go}"

say() {
  printf '[semio] %s\n[semio] %s\n' "$1" "$2" >&2
}

fail() {
  say "$1" "$2"
  exit 1
}

pin_row() {
  { grep -F "{\"tool\":\"go\",\"version\":\"" "$pins" | grep -F "\"platform\":\"$1\",\"arch\":\"$2\"," | sed -n '1p'; } || true
}

pin_field() {
  printf '%s' "$1" | sed -n "s/.*\"$2\":\"\([^\"]*\)\".*/\1/p"
}

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v python3 >/dev/null 2>&1; then
    python3 - "$1" <<'PY'
import hashlib
import sys

digest = hashlib.sha256()
with open(sys.argv[1], "rb") as source:
    for chunk in iter(lambda: source.read(1024 * 1024), b""):
        digest.update(chunk)
print(digest.hexdigest())
PY
  else
    fail "A SHA-256 tool (sha256sum, shasum, or Python 3) is required to verify the Go toolchain." "Ein SHA-256-Werkzeug (sha256sum, shasum oder Python 3) ist zum Prüfen der Go-Toolchain nötig."
  fi
}

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

is_pinned_go() {
  [ -x "$1" ] && [ "$("$1" version 2>/dev/null | awk '{print $3}')" = "go$2" ]
}

work=""
partial=""
lock=""
cleanup() {
  [ -z "$work" ] || rm -rf "$work"
  [ -z "$partial" ] || rm -rf "$partial"
  [ -z "$lock" ] || rm -rf "$lock"
}
cancelled() {
  say "Cancelled; nothing was installed." "Abgebrochen; nichts wurde installiert."
  exit 130
}
trap cleanup EXIT
trap cancelled INT TERM

wait_for_lock() {
  local attempts=0 owner
  mkdir -p "$(dirname "$1")"
  until mkdir "$1" 2>/dev/null; do
    owner="$(cat "$1/pid" 2>/dev/null || true)"
    if [ -n "$owner" ] && ! kill -0 "$owner" 2>/dev/null; then
      rm -rf "$1"
      continue
    fi
    attempts=$((attempts + 1))
    [ "$attempts" -le 900 ] || fail "Another process has been installing the Go toolchain for 15 minutes: $1" "Ein anderer Prozess installiert seit 15 Minuten die Go-Toolchain: $1"
    [ "$attempts" -ne 1 ] || say "Waiting for another process that installs the Go toolchain…" "Warte auf einen anderen Prozess, der die Go-Toolchain installiert…"
    sleep 1
  done
  printf '%s' "$$" >"$1/pid"
  lock="$1"
}

download_go() {
  local row="$1" version="$2" target="$3" url sha base archive actual tty=0 protocols="=https"
  url="$(pin_field "$row" url)"
  sha="$(pin_field "$row" sha256)"
  archive="${url##*/}"
  if [ -n "${SEMIO_GO_MIRROR:-}" ]; then
    case "$SEMIO_GO_MIRROR" in
    https://*) base="${SEMIO_GO_MIRROR%/}" ;;
    http://127.0.0.1[:/]* | http://localhost[:/]* | http://\[::1\][:/]*) base="${SEMIO_GO_MIRROR%/}" protocols="=https,http" ;;
    *) fail "SEMIO_GO_MIRROR must be an https URL or a loopback http URL." "SEMIO_GO_MIRROR muss eine https-URL oder eine Loopback-http-URL sein." ;;
    esac
    url="$base/$archive"
  fi
  command -v curl >/dev/null 2>&1 || command -v python3 >/dev/null 2>&1 || fail "curl or Python 3 is required to download the Go toolchain." "curl oder Python 3 ist zum Herunterladen der Go-Toolchain nötig."
  command -v tar >/dev/null 2>&1 || command -v python3 >/dev/null 2>&1 || fail "tar or Python 3 is required to unpack the Go toolchain." "tar oder Python 3 ist zum Entpacken der Go-Toolchain nötig."
  mkdir -p "$(dirname "$target")"
  wait_for_lock "$target.lock"
  if is_pinned_go "$target/bin/go" "$version"; then
    return 0
  fi
  partial="$(mktemp -d "$(dirname "$target")/.partial.XXXXXX")"
  say "Downloading Go $version (pinned, SHA-256 checked): $url" "Lade Go $version herunter (gepinnt, SHA-256 geprüft): $url"
  [ -t 2 ] && tty=1
  schemes="${protocols//=}"
  if command -v curl >/dev/null 2>&1; then
    if [ "$tty" -eq 1 ]; then
      curl --proto "$protocols" --proto-redir "=https" --tlsv1.2 -fL --progress-bar --retry 3 --connect-timeout 30 --max-time 1800 --max-filesize 400000000 -o "$partial/$archive" "$url" || fail "The Go download failed: $url" "Der Go-Download ist fehlgeschlagen: $url"
    else
      curl --proto "$protocols" --proto-redir "=https" --tlsv1.2 -fsSL --retry 3 --connect-timeout 30 --max-time 1800 --max-filesize 400000000 -o "$partial/$archive" "$url" || fail "The Go download failed: $url" "Der Go-Download ist fehlgeschlagen: $url"
    fi
  else
    python3 - "$url" "$partial/$archive" "$schemes" <<'PY' || fail "The Go download failed: $url" "Der Go-Download ist fehlgeschlagen: $url"
import ssl
import sys
import time
import urllib.error
import urllib.request

url, dest, schemes = sys.argv[1], sys.argv[2], set(sys.argv[3].split(","))
limit = 400000000
deadline = time.monotonic() + 1800

def scheme_of(value):
    return str(value).split(":", 1)[0]

if scheme_of(url) not in schemes:
    raise SystemExit(1)

class HttpsRedirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, response, code, message, headers, newurl):
        if scheme_of(newurl) != "https":
            raise urllib.error.URLError("redirect left https")
        return super().redirect_request(request, response, code, message, headers, newurl)

context = ssl.create_default_context()
context.minimum_version = ssl.TLSVersion.TLSv1_2
opener = urllib.request.build_opener(HttpsRedirects, urllib.request.HTTPSHandler(context=context))
temporary = dest + ".partial"
for _attempt in range(3):
    try:
        with opener.open(url, timeout=30) as response, open(temporary, "wb") as output:
            total = 0
            while True:
                if time.monotonic() > deadline:
                    raise TimeoutError("download exceeded 1800s")
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                total += len(chunk)
                if total > limit:
                    raise OSError("download exceeded 400000000 bytes")
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
  fi
  actual="$(sha256_of "$partial/$archive")"
  [ "$actual" = "$sha" ] || fail "Checksum mismatch for $archive (pinned $sha, received $actual); nothing was unpacked." "Prüfsumme von $archive stimmt nicht (gepinnt $sha, erhalten $actual); nichts wurde entpackt."
  say "Unpacking Go $version…" "Entpacke Go $version…"
  mkdir "$partial/unpacked"
  if command -v tar >/dev/null 2>&1 && tar -xzf "$partial/$archive" -C "$partial/unpacked"; then
    :
  elif command -v python3 >/dev/null 2>&1; then
    python3 - "$partial/$archive" "$partial/unpacked" <<'PY' || fail "The Go archive could not be unpacked." "Das Go-Archiv konnte nicht entpackt werden."
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
        parts = [part for part in member.name.split("/") if part != ""]
        if not parts or any(part in (".", "..") for part in parts):
            raise SystemExit(1)
        target = inside(os.path.join(dest, *parts))
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
  else
    fail "The Go archive could not be unpacked." "Das Go-Archiv konnte nicht entpackt werden."
  fi
  is_pinned_go "$partial/unpacked/go/bin/go" "$version" || fail "The unpacked toolchain does not report Go $version." "Die entpackte Toolchain meldet nicht Go $version."
  rm -rf "$target"
  mv "$partial/unpacked/go" "$target"
  say "Installed Go $version: $target" "Go $version installiert: $target"
}

ensure_go() {
  local platform arch row version sha partition
  [ -f "$pins" ] || fail "The pinned toolchain table is missing: $pins" "Die Tabelle der gepinnten Toolchains fehlt: $pins"
  platform="$(host_platform)" || fail "Unsupported operating system $(uname -s); on Windows use the PowerShell installer." "Nicht unterstütztes Betriebssystem $(uname -s); unter Windows den PowerShell-Installer verwenden."
  arch="$(host_arch)" || fail "Unsupported architecture $(uname -m)." "Nicht unterstützte Architektur $(uname -m)."
  row="$(pin_row "$platform" "$arch")"
  [ -n "$row" ] || fail "No pinned Go for $platform/$arch in $pins." "Kein gepinntes Go für $platform/$arch in $pins."
  version="$(pin_field "$row" version)"
  sha="$(pin_field "$row" sha256)"
  if command -v go >/dev/null 2>&1 && is_pinned_go "$(command -v go)" "$version"; then
    GO="$(command -v go)"
    return 0
  fi
  partition="$root/.cache/b/t/m/native/go/$version/$platform-$arch-${sha:0:16}/bin/go"
  if is_pinned_go "$partition" "$version"; then
    GO="$partition"
    return 0
  fi
  GO="$go_home/v$version/bin/go"
  is_pinned_go "$GO" "$version" || download_go "$row" "$version" "$go_home/v$version"
  is_pinned_go "$GO" "$version" || fail "The Go toolchain $version is not usable at $GO." "Die Go-Toolchain $version ist unter $GO nicht benutzbar."
}

GO=""
ensure_go

work="$(mktemp -d)"
mkdir -p "$work/cmd" "$destination"
printf 'module github.com/usalu/semio/repo/launcher\n\ngo 1.25\n' >"$work/go.mod"
index=0
for source in "$module/🐹️.go" "$module"/*/🐹️.go; do
  [ "$(basename "$(dirname "$source")")" = "📦️main" ] && continue
  index=$((index + 1))
  cp "$source" "$work/s$index.go"
done
cp "$module/📦️main/🐹️.go" "$work/cmd/main.go"

(cd "$work" && GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 "$GO" build -trimpath -ldflags "-s -w" -o "$work/repo" ./cmd) || fail "Building the launcher failed." "Das Bauen des Launchers ist fehlgeschlagen."
rm -f "$destination/repo.new"
cp "$work/repo" "$destination/repo.new"
chmod 755 "$destination/repo.new"
mv -f "$destination/repo.new" "$destination/repo"
printf '[semio] Installed the repo launcher: %s/repo\n[semio] Repo-Launcher installiert: %s/repo\n' "$destination" "$destination" >&2
case ":$PATH:" in
*":$destination:"*) ;;
*) printf '[semio] Add to PATH: %s\n[semio] Zum PATH hinzufügen: %s\n' "$destination" "$destination" >&2 ;;
esac
physical() {
  local path="$1" target
  while [ -L "$path" ]; do
    target="$(readlink "$path")"
    case "$target" in
    /*) path="$target" ;;
    *) path="$(dirname "$path")/$target" ;;
    esac
  done
  printf '%s/%s' "$(cd "$(dirname "$path")" && pwd -P)" "$(basename "$path")"
}

installed="$(physical "$destination/repo")"
IFS=: read -r -a path_entries <<<"$PATH"
seen=""
for entry in "${path_entries[@]}"; do
  [ -n "$entry" ] && [ -x "$entry/repo" ] || continue
  found="$(physical "$entry/repo")"
  [ "$found" = "$installed" ] && continue
  case "
$seen
" in
  *"
$found
"*) continue ;;
  esac
  seen="${seen}
${found}"
  printf '[semio] Another repository frontend is on PATH: %s/repo. Installed launcher: %s\n[semio] Ein weiteres Repository-Frontend liegt im PATH: %s/repo. Installierter Launcher: %s\n' "$entry" "$installed" "$entry" "$installed" >&2
done
