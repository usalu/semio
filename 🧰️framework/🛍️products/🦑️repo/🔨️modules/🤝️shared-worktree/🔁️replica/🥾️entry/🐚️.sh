#!/bin/sh
# Service entry shim (installed as boot.sh): on macOS a worktree below Documents, Desktop or Downloads is run by the launcher at its install location (the one identity that holds the file access consent) as soon as the daemon has proved that the launcher holds the consent (marker with the sha-256 of the launcher), never by a runtime of another path afterwards; before that proof it keeps the former order so that no dialog stops the replica; otherwise it finds a runtime for the boot program (boot.mjs, then boot.prev.mjs) and keeps trying; it ends only when the boot program ends with 0 or 75, after 20 s of running, or on a stop signal.
dir=$(cd "$(dirname "$0")" && pwd)
PATH="$HOME/.local/bin:$HOME/.bun/bin:/usr/local/bin:/opt/homebrew/bin:$PATH"
export PATH
SEMIO_FLEET_ENTRY=boot
export SEMIO_FLEET_ENTRY
stopping=
child=
trap 'stopping=1; if [ -n "$child" ]; then kill -TERM "$child" 2>/dev/null; fi' TERM INT HUP
note() { printf '%s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" | tee "$dir/shim.txt"; }
cached_bun() { ls -td "$HOME/Library/Caches/semio/runtimes/bun"/*/bun "${XDG_CACHE_HOME:-$HOME/.cache}/semio/runtimes/bun"/*/bun 2>/dev/null | head -1; }
launcher="${XDG_DATA_HOME:-$HOME/.local/share}/semio/launcher/bin/repo"
protected_worktree() {
  [ "$(uname -s)" = Darwin ] || return 1
  case "$1" in "$HOME/Documents"|"$HOME/Documents/"*|"$HOME/Desktop"|"$HOME/Desktop/"*|"$HOME/Downloads"|"$HOME/Downloads/"*) return 0 ;; esac
  return 1
}
launcher_consented() {
  [ -x "$launcher" ] || return 1
  tag=$(printf %s "$1" | shasum -a 256 | cut -c1-12)
  marker="$HOME/Library/Application Support/semio/access/$tag/launcher.json"
  [ -f "$marker" ] || return 1
  sum=$(shasum -a 256 "$launcher" | cut -d' ' -f1)
  grep -q "\"sha256\":\"$sum\"" "$marker"
}
run_one() {
  started=$(date +%s)
  "$@" &
  child=$!
  wait "$child"
  rc=$?
  while [ -n "$stopping" ] && kill -0 "$child" 2>/dev/null; do wait "$child"; rc=$?; done
  child=
  elapsed=$(( $(date +%s) - started ))
  if [ -n "$stopping" ] || [ "$rc" -eq 0 ] || [ "$rc" -eq 75 ] || [ "$elapsed" -ge 20 ]; then exit "$rc"; fi
  note "$1 $2 ended with $rc after $elapsed s / $1 $2 endete mit $rc nach $elapsed s"
}
delay=2
while :; do
  worktree=$(cat "$dir/worktree" 2>/dev/null)
  inside=
  if [ -n "$worktree" ] && [ -d "$worktree" ] && cd "$worktree" 2>/dev/null; then inside=1; fi
  ran=
  for program in "$dir/boot.mjs" "$dir/boot.prev.mjs"; do
    [ -f "$program" ] || continue
    if protected_worktree "$worktree" && launcher_consented "$worktree"; then ran=1; SEMIO_WORKSPACE_ROOT="$worktree"; export SEMIO_WORKSPACE_ROOT; run_one "$launcher" "$program" "$@"; continue; fi
    if [ -n "$inside" ] && command -v repo >/dev/null 2>&1; then ran=1; run_one repo "$program" "$@"; fi
    bun=$(cached_bun)
    if [ -n "$bun" ]; then ran=1; run_one "$bun" "$program" "$@"; fi
    if command -v bun >/dev/null 2>&1; then ran=1; run_one bun "$program" "$@"; fi
  done
  [ -n "$ran" ] || note "no boot program or no runtime (repo, cached bun, bun) found / kein Boot-Programm oder keine Laufzeit (repo, bun im Cache, bun) gefunden"
  sleep "$delay" &
  child=$!
  wait "$child"
  child=
  [ -n "$stopping" ] && exit 143
  [ "$delay" -ge 60 ] || delay=$(( delay * 2 ))
done
