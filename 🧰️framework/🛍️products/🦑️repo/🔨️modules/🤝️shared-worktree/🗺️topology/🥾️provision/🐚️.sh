#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖️Header
# Zero-touch macOS/Linux worker provisioning of the shared worktree fleet, from an empty disk. Idempotent: every step probes before it changes anything and prints what it does.
# Operators run it once per fleet machine, as root or through sudo; it changes system settings (fstab or LaunchDaemons, /etc profile files, system services), so it is never run by the build.
# 🧭️Usage: bash "<this file>" <group-id> <origin-id> [--user <account>] [--archive <directory>] [--first-machine | --server]
#   group-id   a group of the topology (g1..g5); origin-id  unique machine name matching ^[a-z0-9][a-z0-9-]{0,39}$;
#   --user     the account that owns the worktree and runs the replica daemon (default: the sudo caller);
#   --archive  a directory with worktree.tar and manifest.json (default: <store>/v1/bootstrap of the group, mounted read-only for the purpose; a new group server passes the bootstrap directory of a peer group mount).
#   --first-machine  the first machine of the fleet: a checkout is at the canonical path and the store is empty; `seed` publishes it instead of `join` (refused when the store already has origins; never use it twice);
#   --server   this machine is the group server, on the platform that the topology names for it (linux or macos; a Windows server uses the PowerShell script): the script writes the exports, mounts the peers read-only, installs the remote cache and relay services next to the replica (the cache tokens file must exist: it is never written here), its daily compaction passes --collect-blobs (the only machine that deletes unreferenced blobs), and, unless --first-machine is also given, one `relay --once` round copies the other groups into its still empty store before it settles and joins; it needs --archive (a peer group's bootstrap directory) when the worktree is empty. On Linux the store is the local export bind-mounted at the store mount (no NFS client entry); on macOS the store root is the exported folder itself (--store);
# The script needs the topology file next to it (../🔣️.json): the early steps run before the repo launcher exists, so they read the worktree path and the NFS export from it with sed; the topology command re-derives the same values later and the script refuses any difference.
# Re-running is safe: when .cache/f/machine.json exists, its origin and group must equal the arguments (otherwise the script stops before it changes anything); `join` is then skipped, while the mounts, limits and services are installed again.
# Linux workers get /etc/sysctl.d/90-semio-fleet.conf with fs.inotify.max_user_watches >= 1048576 and max_user_instances, applied and verified (the tree has ~122k directories).
# Steps: canonical worktree, bootstrap archive (verify its sha256 against manifest.json, extract with the OS tar), the standalone `repo` launcher (built from the checkout by the launcher installer; it needs no compiler and no package), system limits, store mount and NFS client, clock synchronisation check, cache environment, replica and compact services (daily compaction timer or task), `bootstrap-settle`, `join`, service start, and only then, as the last step, the repository's native bootstrap (build tools, Node.js, Rust, .NET, uv, `repo-mcp:install`, `workspace:setup`).
# Joining the fleet never depends on the native build: a fix for a broken build arrives through the shared worktree itself, so the machine must already be syncing when the native bootstrap runs. The native bootstrap runs last with its output in .cache/f/native-bootstrap.log (ignored local state); a completion marker .cache/f/native-bootstrap.done is written only after it succeeded. When it fails (or the build tools are missing and cannot be installed), the script prints in English the failing command and the log, leaves the marker absent so that the next run retries it, and exits with code 3; the fleet setup stays as it is. `preflight` shows the same state as the warning `native-build`.
# Exit codes: 0 fleet ready and native build complete; 1 error (usage errors 2), nothing after the failing step ran; 3 fleet ready, native build pending (the machine syncs; run the script again after the cause is fixed or the fix has arrived).
# A worktree that already holds a checkout skips the archive; a worktree that is neither empty nor a checkout is refused. The first machine of a fleet has no archive: put a checkout at the canonical path and pass --first-machine.
# Tokens of the cache server are not written here (the profile files are world-readable): the runbook says how to give the account SEMIO_REMOTE_CACHE_TOKEN.
# #endregion 🔖️Header
set -euo pipefail

MODULES="🧰️framework/🛍️products/🦑️repo/🔨️modules"
NATIVE_BOOTSTRAP="$MODULES/🔩️native/🥾️bootstrap/🐚️.sh"
LAUNCHER_INSTALL="$MODULES/🚀️launcher/📥️install/🐚️.sh"
EXIT_NATIVE_PENDING=3
TOPOLOGY_RELATIVE="$MODULES/🤝️shared-worktree/🗺️topology"

say() { printf '[fleet-provision] %s\n' "$*"; }
fail() { printf '[fleet-provision] ERROR: %s\n' "$*" >&2; exit 1; }
usage() { printf 'usage: provision <group-id> <origin-id> [--user <account>] [--archive <directory>] [--first-machine | --server]\n' >&2; exit 2; }

ORIGINAL=("$@")
[ $# -ge 2 ] || usage
GROUP_ID=$1; ORIGIN_ID=$2; shift 2
ARCHIVE=""; FIRST_MACHINE=0; SERVER=0; WORKER_USER=${SUDO_USER:-}
while [ $# -gt 0 ]; do
  case "$1" in
    --archive) [ $# -ge 2 ] || usage; ARCHIVE=$2; shift 2 ;;
    --user) [ $# -ge 2 ] || usage; WORKER_USER=$2; shift 2 ;;
    --first-machine) FIRST_MACHINE=1; shift ;;
    --server) SERVER=1; shift ;;
    *) usage ;;
  esac
done
[[ $GROUP_ID =~ ^[a-z][a-z0-9]{0,15}$ ]] || fail "group id must match ^[a-z][a-z0-9]{0,15}\$: $GROUP_ID"
[[ $ORIGIN_ID =~ ^[a-z0-9][a-z0-9-]{0,39}$ ]] || fail "origin id must match ^[a-z0-9][a-z0-9-]{0,39}\$: $ORIGIN_ID"

if [ "$(id -u)" -ne 0 ]; then
  say "not root: re-running through sudo"
  exec sudo -E env "PATH=$PATH" bash "${BASH_SOURCE[0]}" "${ORIGINAL[@]}"
fi
[ -n "$WORKER_USER" ] || WORKER_USER=$(logname 2>/dev/null || true)
[ -n "$WORKER_USER" ] || fail "pass --user <account>: no sudo caller found"
id -u "$WORKER_USER" >/dev/null 2>&1 || fail "unknown account: $WORKER_USER"

case "$(uname -s)" in
  Darwin) PLATFORM=macos; WORKER_HOME=$(dscl . -read "/Users/$WORKER_USER" NFSHomeDirectory | awk '{print $2}'); UTF8_LOCALE=en_US.UTF-8 ;;
  Linux) PLATFORM=linux; WORKER_HOME=$(getent passwd "$WORKER_USER" | cut -d: -f6); UTF8_LOCALE=C.UTF-8 ;;
  *) fail "macOS or Linux required (Windows: use the PowerShell script next to this one)" ;;
esac
{ [ -n "$WORKER_HOME" ] && [ -d "$WORKER_HOME" ]; } || fail "the account $WORKER_USER has no home directory"

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TOPOLOGY_JSON=""
for candidate in "$SCRIPT_DIR"/../🔣*.json; do TOPOLOGY_JSON=$candidate; done
[ -f "$TOPOLOGY_JSON" ] || fail "the topology file was not found next to this script: $SCRIPT_DIR/.."

json_object() { printf '%s' "$1" | sed -n "s/.*\"$2\"[[:space:]]*:[[:space:]]*{\([^}]*\)}.*/\1/p"; }
json_string() { printf '%s' "$1" | sed -n "s/.*\"$2\"[[:space:]]*:[[:space:]]*\"\([^\"]*\)\".*/\1/p"; }
TOPOLOGY_TEXT=$(tr -d '\n\r' < "$TOPOLOGY_JSON")
WORKTREE=$(json_string "$(json_object "$TOPOLOGY_TEXT" worktree)" "$PLATFORM")
GROUP_NFS=$(printf '%s' "$TOPOLOGY_TEXT" | sed -n "s/.*{[[:space:]]*\"id\"[[:space:]]*:[[:space:]]*\"$GROUP_ID\"[[:space:]]*,[[:space:]]*\"nfs\"[[:space:]]*:[[:space:]]*{\([^}]*\)}.*/\1/p")
NFS_SOURCE="$(json_string "$GROUP_NFS" host):$(json_string "$GROUP_NFS" export)"
{ [ -n "$WORKTREE" ] && [ "$NFS_SOURCE" != ":" ]; } || fail "the topology has no worktree for $PLATFORM or no group $GROUP_ID: $TOPOLOGY_JSON"
GROUP_SERVER=$(printf '%s' "$TOPOLOGY_TEXT" | sed -n "s/.*{[[:space:]]*\"id\"[[:space:]]*:[[:space:]]*\"$GROUP_ID\"[[:space:]]*,[[:space:]]*\"nfs\"[[:space:]]*:[[:space:]]*{[^}]*}[[:space:]]*,[[:space:]]*\"server\"[[:space:]]*:[[:space:]]*{\([^}]*\)}.*/\1/p")
SERVER_PLATFORM=$(json_string "$GROUP_SERVER" platform)
SERVER_PROTOCOL=$(json_string "$GROUP_SERVER" protocol)
{ [ -n "$SERVER_PLATFORM" ] && [ -n "$SERVER_PROTOCOL" ]; } || fail "the topology has no server for group $GROUP_ID: $TOPOLOGY_JSON"
[ "$SERVER_PROTOCOL" != smb ] || fail "group $GROUP_ID shares its store over SMB, which only Windows workers mount: use the PowerShell script on a Windows machine"
[ "$SERVER" = 0 ] || [ "$SERVER_PLATFORM" = "$PLATFORM" ] || fail "the topology puts the server of $GROUP_ID on $SERVER_PLATFORM, this machine is $PLATFORM: provision it without --server, or change the one topology file"
JOINED=0
MACHINE=$WORKTREE/.cache/f/machine.json
if [ -f "$MACHINE" ]; then
  MACHINE_TEXT=$(tr -d '\n\r' < "$MACHINE")
  MACHINE_ORIGIN=$(json_string "$MACHINE_TEXT" originId); MACHINE_GROUP=$(json_string "$MACHINE_TEXT" groupId)
  { [ -n "$MACHINE_ORIGIN" ] && [ -n "$MACHINE_GROUP" ]; } || fail "$MACHINE has no originId and groupId: remove the worktree state first"
  [ "$MACHINE_ORIGIN" = "$ORIGIN_ID" ] || fail "$MACHINE belongs to origin $MACHINE_ORIGIN, not $ORIGIN_ID: pass that origin, or remove the worktree state first"
  [ "$MACHINE_GROUP" = "$GROUP_ID" ] || fail "$MACHINE belongs to group $MACHINE_GROUP, not $GROUP_ID: pass that group, or remove the worktree state first"
  JOINED=1
fi

CLEANUP_MOUNT=""; CLEANUP_DIRECTORY=""
cleanup() {
  [ -z "$CLEANUP_MOUNT" ] || umount "$CLEANUP_MOUNT" 2>/dev/null || true
  [ -z "$CLEANUP_MOUNT" ] || rmdir "$CLEANUP_MOUNT" 2>/dev/null || true
  [ -z "$CLEANUP_DIRECTORY" ] || rm -rf "$CLEANUP_DIRECTORY"
}
trap cleanup EXIT

LANGUAGE_CHOICE=${SEMIO_LANGUAGE:-en}
run_as() {
  if [ "$WORKER_USER" = root ]; then env "PATH=$WORKER_HOME/.local/bin:$PATH" "SEMIO_LANGUAGE=$LANGUAGE_CHOICE" "$@"; else sudo -u "$WORKER_USER" -H env "PATH=$WORKER_HOME/.local/bin:$PATH" "SEMIO_LANGUAGE=$LANGUAGE_CHOICE" "$@"; fi
}
fleet() { (cd "$WORKTREE/$TOPOLOGY_RELATIVE" && run_as repo ./📜️script.ts "$@"); }
replica() { local operation=$1; shift; (cd "$FLEET_REPLICA_DIRECTORY" && run_as repo ./📜️script.ts "$operation" "$@"); }
fill() { sed -e "s|__WORKER_USER__|$WORKER_USER|" -e "s|__WORKER_HOME__|$WORKER_HOME|"; }
server_part() { fleet server-plan --group "$GROUP_ID" --part "$1"; }
sha256_of() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }
CHANGED=0; UNITS_CHANGED=0
install_text() {
  local path=$1 mode=$2 content=$3 tmp
  tmp=$(mktemp)
  printf '%s\n' "$content" > "$tmp"
  if [ -f "$path" ] && cmp -s "$tmp" "$path"; then
    say "unchanged $path"; CHANGED=0
  else
    mkdir -p "$(dirname "$path")"; install -m "$mode" "$tmp" "$path"; say "wrote $path"; CHANGED=1; UNITS_CHANGED=1
  fi
  rm -f "$tmp"
}
write_block() {
  local file=$1 content=$2 begin="# >>> semio fleet" end="# <<< semio fleet" tmp
  tmp=$(mktemp); [ -f "$file" ] || : > "$file"
  awk -v b="$begin" -v e="$end" '$0==b{skip=1;next} $0==e{skip=0;next} !skip' "$file" > "$tmp"
  printf '%s\n%s\n%s\n' "$begin" "$content" "$end" >> "$tmp"
  if cmp -s "$tmp" "$file"; then say "unchanged $file"; CHANGED=0; else cat "$tmp" > "$file"; say "updated $file"; CHANGED=1; fi
  rm -f "$tmp"
}
install_daemon() {
  local target=$1 label
  label=$(basename "$target" .plist)
  install_text "$target" 644 "$2"
  chown root:wheel "$target"
  if [ "$CHANGED" = 1 ]; then launchctl bootout "system/$label" 2>/dev/null || true; fi
}
load_daemon() {
  local label
  label=$(basename "$1" .plist)
  if launchctl print "system/$label" >/dev/null 2>&1; then say "already loaded $label"; else launchctl bootstrap system "$1"; say "loaded $label"; fi
}
install_nfs_server() {
  command -v exportfs >/dev/null 2>&1 && return 0
  say "installing the NFS server"
  if command -v apt-get >/dev/null 2>&1; then apt-get install -y nfs-kernel-server
  elif command -v dnf >/dev/null 2>&1; then dnf install -y nfs-utils
  elif command -v zypper >/dev/null 2>&1; then zypper --non-interactive install nfs-kernel-server
  elif command -v pacman >/dev/null 2>&1; then pacman -S --noconfirm nfs-utils
  else fail "install an NFS server (exportfs) first"; fi
}
install_nfs_client() {
  command -v mount.nfs >/dev/null 2>&1 && return 0
  say "installing the NFS client"
  if command -v apt-get >/dev/null 2>&1; then apt-get install -y nfs-common
  elif command -v dnf >/dev/null 2>&1; then dnf install -y nfs-utils
  elif command -v zypper >/dev/null 2>&1; then zypper --non-interactive install nfs-client
  elif command -v pacman >/dev/null 2>&1; then pacman -S --noconfirm nfs-utils
  else fail "install an NFS client (mount.nfs) first"; fi
}
fetch_archive() {
  local source=$1 target=$2 attempt expected actual
  for attempt in 1 2 3 4 5; do
    { [ -f "$source/manifest.json" ] && [ -f "$source/worktree.tar" ]; } || fail "no bootstrap archive in $source: run bootstrap-archive on the group server (its relay refreshes it on a schedule) or pass --archive <directory>"
    expected=$(sed -n 's/.*"sha256"[[:space:]]*:[[:space:]]*"\([0-9a-f]\{64\}\)".*/\1/p' "$source/manifest.json")
    EXPECTED_FILES=$(sed -n 's/.*"files"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$source/manifest.json")
    { [ -n "$expected" ] && [ -n "$EXPECTED_FILES" ]; } || fail "unreadable $source/manifest.json"
    cp "$source/worktree.tar" "$target"
    actual=$(sha256_of "$target")
    if [ "$actual" = "$expected" ]; then say "archive verified: sha256 $expected, $EXPECTED_FILES files"; return 0; fi
    say "the archive differs from manifest.json (refreshed while it was copied?), attempt $attempt of 5"; sleep 20
  done
  fail "the archive in $source never matched its manifest.json"
}

BOOTSTRAP_MARKER=$WORKTREE/.cache/f/native-bootstrap.done
BOOTSTRAP_LOG=$WORKTREE/.cache/f/native-bootstrap.log
LAUNCHER_BIN=$WORKER_HOME/.local/share/semio/launcher/bin
TOOLCHAIN_PROBLEM=""
bootstrap_needed() { [ ! -f "$BOOTSTRAP_MARKER" ] || [ ! -x "$WORKER_HOME/.local/bin/repo" ]; }
toolchain_packages() {
  case "$1" in
    apt-get) printf 'apt-get install -y build-essential pkg-config curl git unzip' ;;
    dnf) printf 'dnf install -y gcc gcc-c++ make pkgconf-pkg-config curl git unzip' ;;
    zypper) printf 'zypper install -y gcc gcc-c++ make pkg-config curl git unzip' ;;
    pacman) printf 'pacman -S --noconfirm base-devel pkgconf curl git unzip' ;;
    *) printf 'the C/C++ toolchain (cc, c++, make), pkg-config, curl, git and unzip of your distribution' ;;
  esac
}
check_toolchain() {
  local tool missing="" manager=""
  if [ "$PLATFORM" = macos ]; then
    for tool in cc c++ make git curl; do command -v "$tool" >/dev/null 2>&1 || missing="$missing $tool"; done
    xcode-select -p >/dev/null 2>&1 || missing="$missing xcode-command-line-tools"
    [ -z "$missing" ] || { TOOLCHAIN_PROBLEM="missing build tools:$missing: install the Xcode Command Line Tools with 'xcode-select --install' (cc, c++, make, git, curl), then run again"; return 1; }
    command -v pkg-config >/dev/null 2>&1 || say "WARNING: pkg-config is missing: the bootstrap installs it with Homebrew (brew install pkg-config cairo pango), install Homebrew first if the machine has none"
    return 0
  fi
  for tool in cc c++ make pkg-config curl git unzip; do command -v "$tool" >/dev/null 2>&1 || missing="$missing $tool"; done
  [ -n "$missing" ] || return 0
  for tool in apt-get dnf zypper pacman; do if command -v "$tool" >/dev/null 2>&1; then manager=$tool; break; fi; done
  if [ "$WORKER_USER" = root ] || run_as sudo -n true >/dev/null 2>&1; then
    say "missing build tools:$missing: the native bootstrap installs them ($(toolchain_packages "$manager"))"
  else
    TOOLCHAIN_PROBLEM="$WORKER_USER has no passwordless sudo, so the bootstrap cannot install packages, and these tools are missing:$missing: install them as root first ($(toolchain_packages "$manager")), or give the account passwordless sudo for the run"
    return 1
  fi
}
native_pending() {
  local command=$1 reason=$2
  say "NATIVE BUILD PENDING: $ORIGIN_ID is a member of group $GROUP_ID and syncs, but its native build is not ready"
  say "failing command: $command"
  say "reason: $reason"
  if [ -f "$BOOTSTRAP_LOG" ]; then
    say "log: $BOOTSTRAP_LOG (last lines below)"
    tail -n 15 "$BOOTSTRAP_LOG" | sed 's/^/[fleet-provision]   | /'
  fi
  say "the completion marker $BOOTSTRAP_MARKER was not written: run this script again to retry the native bootstrap once the cause is fixed (a fix for a broken build arrives through the shared worktree itself); preflight shows it as the warning native-build"
  say "exit code $EXIT_NATIVE_PENDING: fleet ready, native build pending"
  exit "$EXIT_NATIVE_PENDING"
}
# A cold graph build needs a 7 to 8 GB heap (README of the repo tool). Linux workers get swap up to this much memory in total, so a small machine becomes slow instead of having its graph worker killed for lack of memory; normally the graph comes from the remote cache of the group and never builds cold.
COLD_GRAPH_MEMORY_MB=16384
SWAP_FILE=/var/lib/semio-swap
swap_shortfall_mb() { awk -v want="$COLD_GRAPH_MEMORY_MB" '/^MemTotal:/ {ram=$2/1024} /^SwapTotal:/ {swap=$2/1024} END {need=want-ram-swap; print (need > 0 ? int(need) + 1 : 0)}' "${1:-/proc/meminfo}"; }
ensure_swap() {
  local need
  need=$(swap_shortfall_mb)
  if [ "$need" = 0 ]; then say "memory: RAM and swap together cover ${COLD_GRAPH_MEMORY_MB} MB, the heap of a cold graph build"; return 0; fi
  if [ ! -f "$SWAP_FILE" ]; then
    { fallocate -l "${need}M" "$SWAP_FILE" 2>/dev/null || dd if=/dev/zero of="$SWAP_FILE" bs=1M count="$need" status=none; } || { say "WARNING: $SWAP_FILE of $need MB could not be created: a cold graph build may be killed for lack of memory"; rm -f "$SWAP_FILE"; return 0; }
    chmod 600 "$SWAP_FILE"; mkswap "$SWAP_FILE" >/dev/null
  fi
  swapon "$SWAP_FILE" 2>/dev/null || { say "WARNING: swapon $SWAP_FILE failed (a container shares the swap of its host): a cold graph build may be killed for lack of memory"; return 0; }
  grep -q "^$SWAP_FILE " /etc/fstab || printf '%s none swap sw 0 0\n' "$SWAP_FILE" >> /etc/fstab
  say "swap: $SWAP_FILE of $(( $(stat -c %s "$SWAP_FILE") / 1048576 )) MB in use, RAM and swap together cover ${COLD_GRAPH_MEMORY_MB} MB"
}
# The build tools resolve their remote cache from SEMIO_REMOTE_CACHE_*: the profile files that step 5 writes belong to login shells, and the native bootstrap runs through sudo without one, so it gets the same variables and the write token of the account through a file that only the account can read.
native_environment() {
  local line token="${SEMIO_REMOTE_CACHE_TOKEN:-}" file
  while IFS= read -r line; do
    case $line in "export SEMIO_REMOTE_CACHE_"*=*) line=${line#export }; printf "%s='%s'\n" "${line%%=*}" "${line#*=}";; esac
  done <<< "${ENVIRONMENT:-}"
  for file in "$WORKER_HOME/.profile" "$WORKER_HOME/.zprofile"; do
    [ -n "$token" ] || [ ! -f "$file" ] || token=$(sed -n "s/^export SEMIO_REMOTE_CACHE_TOKEN=//p" "$file" | tail -n 1 | tr -d "'\"")
  done
  [ -z "$token" ] || printf "SEMIO_REMOTE_CACHE_TOKEN='%s'\n" "$token"
}
native_bootstrap() {
  local command="bash $WORKTREE/$NATIVE_BOOTSTRAP setup" status environment="$WORKTREE/.cache/f/native-environment"
  if [ "$NEEDS_NATIVE" = 0 ]; then say "native bootstrap complete ($BOOTSTRAP_MARKER)"; return 0; fi
  say "step 9: native bootstrap, the last step (the machine already syncs: a failure here leaves the fleet setup as it is)"
  run_as rm -f "$BOOTSTRAP_MARKER"
  check_toolchain || native_pending "the build tool check before $command" "$TOOLCHAIN_PROBLEM"
  run_as mkdir -p "$(dirname "$BOOTSTRAP_LOG")"
  # shellcheck disable=SC2016
  native_environment | run_as sh -c 'umask 077; cat > "$1"' sh "$environment"
  # shellcheck disable=SC2016
  if run_as env "SEMIO_WORKSPACE_ROOT=$WORKTREE" sh -c 'set -a; . "$1"; set +a; exec bash "$2" setup' sh "$environment" "$WORKTREE/$NATIVE_BOOTSTRAP" 2>&1 | run_as tee "$BOOTSTRAP_LOG"; then status=0; else status=${PIPESTATUS[0]}; fi
  run_as rm -f "$environment"
  [ "$status" = 0 ] || native_pending "$command" "it exited with code $status"
  [ -x "$WORKER_HOME/.local/bin/repo" ] || native_pending "$command" "it did not install $WORKER_HOME/.local/bin/repo"
  date -u +%Y-%m-%dT%H:%M:%SZ | run_as tee "$BOOTSTRAP_MARKER" >/dev/null
  say "recorded the completed bootstrap in $BOOTSTRAP_MARKER"
}
check_time_sync() {
  if [ "$PLATFORM" = macos ]; then
    case "$(systemsetup -getusingnetworktime 2>/dev/null || true)" in
      *": On"*) say "network time is on" ;;
      *) if systemsetup -setusingnetworktime on >/dev/null 2>&1; then say "switched network time on"; else say "WARNING: network time could not be read or switched on: turn on System Settings > General > Date & Time > Set time and date automatically"; fi ;;
    esac
    return 0
  fi
  if ! command -v timedatectl >/dev/null 2>&1; then say "WARNING: timedatectl is missing: install and enable chrony (apt-get install chrony, dnf install chrony, zypper install chrony, pacman -S chrony): a clock more than 60 s off holds back the events of other machines"; return 0; fi
  if [ "$(timedatectl show -p NTPSynchronized --value 2>/dev/null)" != yes ]; then timedatectl set-ntp true >/dev/null 2>&1 || true; sleep 5; fi
  if [ "$(timedatectl show -p NTPSynchronized --value 2>/dev/null)" = yes ]; then say "clock synchronized (NTPSynchronized=yes)"
  else say "WARNING: the clock is not synchronized: install and enable chrony (apt-get install chrony, dnf install chrony, zypper install chrony, pacman -S chrony), a clock more than 60 s off holds back the events of other machines"; fi
}

say "group $GROUP_ID, origin $ORIGIN_ID, platform $PLATFORM, account $WORKER_USER"

NEEDS_NATIVE=0
if bootstrap_needed; then NEEDS_NATIVE=1; fi

say "step 1: canonical worktree $WORKTREE"
mkdir -p "$WORKTREE"
chown "$WORKER_USER" "$WORKTREE"
if [ -f "$WORKTREE/$NATIVE_BOOTSTRAP" ]; then
  say "the worktree already holds a checkout: the archive is skipped"
elif [ -n "$(ls -A "$WORKTREE")" ]; then
  fail "$WORKTREE is neither empty nor a checkout: empty it, or put a complete checkout there"
elif [ "$FIRST_MACHINE" = 1 ]; then
  fail "--first-machine needs a complete checkout at $WORKTREE: the first machine has no archive to extract"
else
  say "step 2: bootstrap archive into the empty worktree"
  [ "$PLATFORM" = macos ] || install_nfs_client
  SOURCE=$ARCHIVE
  [ -n "$SOURCE" ] || [ "$SERVER" = 0 ] || fail "a group server takes the archive of a peer group: pass --archive <peer mount>/v1/bootstrap"
  if [ -z "$SOURCE" ]; then
    CLEANUP_MOUNT=$(mktemp -d /var/tmp/semio-store.XXXXXX)
    if [ "$PLATFORM" = linux ] && [ "$SERVER_PLATFORM" = linux ]; then mount -t nfs4 -o ro,vers=4.2 "$NFS_SOURCE" "$CLEANUP_MOUNT"
    elif [ "$PLATFORM" = linux ]; then mount -t nfs -o ro,vers=3,nolock "$NFS_SOURCE" "$CLEANUP_MOUNT"
    elif [ "$SERVER_PLATFORM" = linux ]; then mount_nfs -o ro,vers=4 "$NFS_SOURCE" "$CLEANUP_MOUNT"
    else mount_nfs -o ro,vers=3,nolocks "$NFS_SOURCE" "$CLEANUP_MOUNT"; fi
    SOURCE=$CLEANUP_MOUNT/v1/bootstrap
  fi
  CLEANUP_DIRECTORY=$(mktemp -d /var/tmp/semio-bootstrap.XXXXXX)
  chmod 755 "$CLEANUP_DIRECTORY"
  fetch_archive "$SOURCE" "$CLEANUP_DIRECTORY/worktree.tar"
  chmod 644 "$CLEANUP_DIRECTORY/worktree.tar"
  say "extracting with tar into $WORKTREE"
  run_as env "LC_ALL=$UTF8_LOCALE" tar -xf "$CLEANUP_DIRECTORY/worktree.tar" -C "$WORKTREE"
  FOUND=$(find "$WORKTREE" -type f | wc -l | tr -d ' ')
  [ "$FOUND" = "$EXPECTED_FILES" ] || fail "extracted $FOUND files, manifest.json says $EXPECTED_FILES: empty $WORKTREE and run again"
  NEEDS_NATIVE=1
fi

say "step 3: standalone repo launcher (built from the checkout; the native bootstrap comes last)"
if [ -x "$WORKER_HOME/.local/bin/repo" ]; then
  say "repo launcher present for $WORKER_USER"
else
  run_as env "XDG_DATA_HOME=$WORKER_HOME/.local/share" bash "$WORKTREE/$LAUNCHER_INSTALL" "$LAUNCHER_BIN" || fail "the repo launcher could not be built by $WORKTREE/$LAUNCHER_INSTALL: the machine cannot join without it, fix the cause and run again"
  [ -x "$LAUNCHER_BIN/repo" ] || fail "the launcher installer did not install $LAUNCHER_BIN/repo"
  run_as mkdir -p "$WORKER_HOME/.local/bin"
  run_as ln -sfn "$LAUNCHER_BIN/repo" "$WORKER_HOME/.local/bin/repo"
  say "installed the launcher $LAUNCHER_BIN/repo and linked it as $WORKER_HOME/.local/bin/repo"
fi
mkdir -p /usr/local/bin
ln -sfn "$WORKER_HOME/.local/bin/repo" /usr/local/bin/repo
say "linked /usr/local/bin/repo for the service definitions"

FACTS=$(fleet facts --group "$GROUP_ID" --platform "$PLATFORM" --shell sh | grep '^FLEET_')
eval "$FACTS"
{ [ "$FLEET_WORKTREE" = "$WORKTREE" ] && [ "$FLEET_NFS_SOURCE" = "$NFS_SOURCE" ]; } || fail "the topology file next to this script differs from the one in the worktree ($FLEET_WORKTREE, $FLEET_NFS_SOURCE): copy the topology directory from the worktree and run again"
{ [ "$FLEET_SERVER_PLATFORM" = "$SERVER_PLATFORM" ] && [ "$FLEET_SERVER_PROTOCOL" = "$SERVER_PROTOCOL" ]; } || fail "the topology file next to this script differs from the one in the worktree (server $FLEET_SERVER_PLATFORM/$FLEET_SERVER_PROTOCOL): copy the topology directory from the worktree and run again"
STORE_ROOT=$FLEET_STORE_MOUNT
if [ "$SERVER" = 1 ] && [ "$PLATFORM" != linux ]; then STORE_ROOT=$FLEET_SERVER_DIRECTORY; fi
say "canonical worktree $FLEET_WORKTREE, store $FLEET_NFS_SOURCE at $FLEET_STORE_MOUNT, cache $FLEET_CACHE_ENDPOINT"

say "step 4: system limits, store and NFS client"
if [ "$PLATFORM" = linux ]; then
  command -v systemctl >/dev/null 2>&1 || fail "systemd is required"
  SYSCTL_TARGET=$(fleet mount-plan --group "$GROUP_ID" --platform linux --part sysctl-path)
  install_text "$SYSCTL_TARGET" 644 "$(fleet mount-plan --group "$GROUP_ID" --platform linux --part sysctl)"
  sysctl -p "$SYSCTL_TARGET" >/dev/null || fail "sysctl could not apply $SYSCTL_TARGET (inside a container the host must raise these limits)"
  while IFS='=' read -r key wanted; do
    [ -n "$key" ] || continue
    [ "$(sysctl -n "$key")" -ge "$wanted" ] || fail "$key is $(sysctl -n "$key"), below the $wanted that the tree of ~122k directories needs"
    say "$key = $(sysctl -n "$key")"
  done < "$SYSCTL_TARGET"
  ensure_swap
  install_nfs_client
  mkdir -p "$FLEET_STORE_MOUNT"
  if [ "$SERVER" = 1 ]; then
    install_nfs_server
    install -d -o "$WORKER_USER" "$FLEET_SERVER_DIRECTORY" "$FLEET_SERVER_CACHE_DIRECTORY"
    install -d -m 0750 -o root -g "$(id -gn "$WORKER_USER")" /etc/semio
    install_text "$(server_part exports-target)" 644 "$(server_part exports)"
    SERVER_SYSCTL_TARGET=$(server_part sysctl-target)
    install_text "$SERVER_SYSCTL_TARGET" 644 "$(server_part sysctl)"
    sysctl -p "$SERVER_SYSCTL_TARGET" >/dev/null || fail "sysctl could not apply $SERVER_SYSCTL_TARGET"
    while IFS='=' read -r key wanted; do
      [ -n "$key" ] || continue
      [ "$(sysctl -n "$key")" = "$wanted" ] || fail "$key is $(sysctl -n "$key"), not the $wanted that stops the NFS server from granting delegations (a macOS client stalls 4 s on every head that another client holds a delegation on)"
      say "$key = $wanted"
    done < "$SERVER_SYSCTL_TARGET"
    write_block /etc/fstab "$(server_part fstab)"
    PEER_MOUNTS=$(server_part fstab | awk '$3!="none"{print $2}')
    PEER_STRICT=$(server_part fstab | awk '/lookupcache=none/{print $2}')
    for path in $PEER_MOUNTS; do mkdir -p "$path"; done
    systemctl daemon-reload
    systemctl enable --now nfs-server || systemctl enable --now nfs-kernel-server
    exportfs -ra
    if mountpoint -q "$FLEET_STORE_MOUNT"; then say "already mounted $FLEET_STORE_MOUNT"; else mount "$FLEET_STORE_MOUNT"; say "bind-mounted $FLEET_SERVER_DIRECTORY at $FLEET_STORE_MOUNT"; fi
    for path in $PEER_MOUNTS; do
      if mountpoint -q "$path"; then
        say "already mounted $path"
        if grep -qx "$path" <<< "$PEER_STRICT" && ! findmnt -no OPTIONS "$path" | grep -q 'lookupcache=none'; then say "WARNING: $path is mounted without lookupcache=none of the mount plan: the relay sees a replaced head up to a minute late (measured: 3 s at the 95th percentile and half of the edits of the other group skipped); stop the relay, unmount it and run this script again"; fi
      elif mount "$path"; then say "mounted $path"; else say "WARNING: $path is not mounted: is the server of that group up? It is retried at boot (nofail) and by this script"; fi
    done
  else
    FSTAB_FOREIGN=$(awk -v mp="$FLEET_STORE_MOUNT" -v src="$FLEET_NFS_SOURCE" '$1 !~ /^#/ && $2==mp && $1!=src' /etc/fstab)
    [ -z "$FSTAB_FOREIGN" ] || fail "/etc/fstab already mounts another source at $FLEET_STORE_MOUNT: $FSTAB_FOREIGN"
    FSTAB_CHANGED=0
    if grep -qxF "$FLEET_FSTAB_LINE" /etc/fstab; then say "unchanged /etc/fstab"; else
      FSTAB_TMP=$(mktemp)
      awk -v mp="$FLEET_STORE_MOUNT" -v src="$FLEET_NFS_SOURCE" '!($1 !~ /^#/ && $2==mp && $1==src)' /etc/fstab > "$FSTAB_TMP"
      printf '%s\n' "$FLEET_FSTAB_LINE" >> "$FSTAB_TMP"
      cat "$FSTAB_TMP" > /etc/fstab; rm -f "$FSTAB_TMP"
      FSTAB_CHANGED=1; say "set in /etc/fstab: $FLEET_FSTAB_LINE"
    fi
    systemctl daemon-reload
    if mountpoint -q "$FLEET_STORE_MOUNT"; then
      say "already mounted $FLEET_STORE_MOUNT"
      [ "$FSTAB_CHANGED" = 0 ] || say "WARNING: the mount plan changed; $FLEET_STORE_MOUNT keeps its old options until it is unmounted and mounted again (stop the replica services first)"
    else mount "$FLEET_STORE_MOUNT"; say "mounted $FLEET_STORE_MOUNT"; fi
  fi
elif [ "$SERVER" = 1 ]; then
  install -d -o "$WORKER_USER" "$FLEET_SERVER_DIRECTORY" "$FLEET_SERVER_CACHE_DIRECTORY"
  install -d -m 0750 -o "$WORKER_USER" /etc/semio
  write_block /etc/exports "$(server_part exports | fill)"
  write_block /etc/nfs.conf "$(server_part nfs-conf)"
  NFS_CONF_CHANGED=$CHANGED
  nfsd checkexports || fail "/etc/exports is not valid: nothing was exported"
  nfsd enable
  if [ "$NFS_CONF_CHANGED" = 1 ]; then nfsd restart; else nfsd update || true; fi
  say "exporting $FLEET_SERVER_DIRECTORY through nfsd (the server's own store root, no mount)"
  PEER_TARGET=$(server_part peer-mount-target)
  install_daemon "$PEER_TARGET" "$(server_part peer-mount-unit)"
  load_daemon "$PEER_TARGET"
else
  MOUNT_PLIST=$(fleet mount-plan --group "$GROUP_ID" --platform macos --part plist-path)
  install_daemon "$MOUNT_PLIST" "$(fleet mount-plan --group "$GROUP_ID" --platform macos --part plist)"
  MOUNT_PLAN_CHANGED=$CHANGED
  load_daemon "$MOUNT_PLIST"
  if grep -q " on $FLEET_STORE_MOUNT " <<< "$(mount)"; then
    say "already mounted $FLEET_STORE_MOUNT"
    [ "$MOUNT_PLAN_CHANGED" = 0 ] || say "WARNING: the mount plan changed; $FLEET_STORE_MOUNT keeps its old options until it is unmounted and mounted again (stop the replica service first, then sudo umount $FLEET_STORE_MOUNT)"
  else mkdir -p "$FLEET_STORE_MOUNT"; mount_nfs -o "$FLEET_MOUNT_OPTIONS" "$FLEET_NFS_SOURCE" "$FLEET_STORE_MOUNT"; say "mounted $FLEET_STORE_MOUNT"; fi
fi

say "step 4b: clock"
check_time_sync

say "step 5: cache environment"
ENVIRONMENT=$(fleet env --group "$GROUP_ID" --shell sh | grep '^export ')
if [ "$PLATFORM" = linux ]; then
  install_text /etc/profile.d/semio-fleet.sh 644 "$ENVIRONMENT"
else
  while IFS= read -r line; do
    assignment=${line#export }
    launchctl setenv "${assignment%%=*}" "${assignment#*=}"
    say "launchctl setenv ${assignment%%=*}"
  done <<< "$ENVIRONMENT"
  write_block /etc/zshenv "$ENVIRONMENT"
  write_block /etc/profile "$ENVIRONMENT"
fi

say "step 6: replica and compact services (installed, started after join)"
UNITS_CHANGED=0
SERVICE_TARGET=$(fleet worker-service --group "$GROUP_ID" --platform "$PLATFORM" --part target)
COMPACT_TARGET=$(fleet worker-service --group "$GROUP_ID" --platform "$PLATFORM" --part compact-target)
ROLE=worker; [ "$SERVER" = 0 ] || ROLE=server
if [ "$PLATFORM" = linux ]; then
  install_text "$SERVICE_TARGET" 644 "$(fleet worker-service --group "$GROUP_ID" --platform linux)"
  install_text "$COMPACT_TARGET" 644 "$(fleet worker-service --group "$GROUP_ID" --platform linux --part compact-definition --role "$ROLE")"
  install_text "$(fleet worker-service --group "$GROUP_ID" --platform linux --part timer-target)" 644 "$(fleet worker-service --group "$GROUP_ID" --platform linux --part timer-definition)"
  UNIT=$(basename "$SERVICE_TARGET")
  INSTANCE=${UNIT/@/@$WORKER_USER}
  TIMER=semio-fleet-compact@$WORKER_USER.timer
  systemctl daemon-reload
  systemctl enable "$INSTANCE"
else
  install_daemon "$SERVICE_TARGET" "$(fleet worker-service --group "$GROUP_ID" --platform macos | fill)"
  install_daemon "$COMPACT_TARGET" "$(fleet worker-service --group "$GROUP_ID" --platform macos --part compact-definition --role "$ROLE" | fill)"
fi

if [ "$SERVER" = 1 ]; then
  say "step 6b: group server services (remote cache and relay)"
  TOKENS=$(server_part cache-environment-target)
  [ -f "$TOKENS" ] || fail "$TOKENS does not exist: create it with the real tokens (lines of: server-plan --group $GROUP_ID --part cache-environment), then run again"
  while IFS= read -r line; do
    token=${line%%=*}
    grep -q "^$token=[^<]" "$TOKENS" || fail "$TOKENS has no real value for $token"
  done <<< "$(server_part cache-environment)"
  if [ "$PLATFORM" = linux ]; then
    chown "root:$(id -gn "$WORKER_USER")" "$TOKENS"; chmod 640 "$TOKENS"
    install_text "$(server_part cache-target)" 644 "$(server_part cache-unit)"
    install_text "$(server_part relay-target)" 644 "$(server_part relay-unit)"
    CACHE_UNIT=$(basename "$(server_part cache-target)"); RELAY_UNIT=$(basename "$(server_part relay-target)")
    systemctl daemon-reload
    systemctl enable "$CACHE_UNIT" "$RELAY_UNIT"
  else
    chown "$WORKER_USER" "$TOKENS"; chmod 600 "$TOKENS"
    CACHE_TARGET=$(server_part cache-target); RELAY_TARGET=$(server_part relay-target)
    install_daemon "$CACHE_TARGET" "$(server_part cache-unit | fill)"
    install_daemon "$RELAY_TARGET" "$(server_part relay-unit | fill)"
  fi
fi

say "step 7: settle the extracted tree and join (first machine: seed)"
if [ "$JOINED" = 1 ]; then
  say "already joined as $ORIGIN_ID in group $GROUP_ID: join is skipped, the mounts and services above were refreshed"
elif [ "$FIRST_MACHINE" = 1 ]; then
  [ -z "$(ls -A "$STORE_ROOT/v1/origins" 2>/dev/null)" ] || fail "the store already has origins: this is not the first machine of the fleet, run without --first-machine"
  replica seed --group "$GROUP_ID" --origin "$ORIGIN_ID" --store "$STORE_ROOT"
else
  RELAY_PEERS=$(server_part relay-peers 2>/dev/null || true)
  if [ "$SERVER" = 1 ] && [ -n "$RELAY_PEERS" ]; then replica relay --once --group "$GROUP_ID" --store "$STORE_ROOT" --peers "$RELAY_PEERS"; fi
  replica bootstrap-settle --store "$STORE_ROOT"
  replica join --group "$GROUP_ID" --origin "$ORIGIN_ID" --store "$STORE_ROOT"
fi

say "step 8: start the services"
if [ "$PLATFORM" = linux ]; then
  STARTED=("$INSTANCE"); [ "$SERVER" = 0 ] || STARTED+=("$CACHE_UNIT" "$RELAY_UNIT")
  [ "$UNITS_CHANGED" = 0 ] || systemctl try-restart "${STARTED[@]}"
  systemctl enable --now "${STARTED[@]}" "$TIMER"
  say "started ${STARTED[*]} and $TIMER"
else
  for target in "$SERVICE_TARGET" "$COMPACT_TARGET"; do load_daemon "$target"; done
  if [ "$SERVER" = 1 ]; then for target in "$CACHE_TARGET" "$RELAY_TARGET"; do load_daemon "$target"; done; fi
fi
say "fleet ready: $ORIGIN_ID is a member of group $GROUP_ID and syncs"
native_bootstrap
say "done: $ORIGIN_ID is a member of group $GROUP_ID and its native build is complete"
