import { join } from "node:path";
import { chmodSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { GK_COMMIT_TEMPLATE_FILE } from "../📄️prepared-message/🟦️.ts";
import { getRepoMetaDir } from "../../../🗂️workspaces/🧬️metadata/🟦️.ts";

const MICRO_COMMIT_POST_WIPE_HOOKS = ["post-commit", "post-checkout", "post-merge", "post-rewrite"] as const;

const MICRO_COMMIT_BUN_PIN = "compose-micro-commit-bun";

/** 🥖️Resolves the Bun executable for git hooks (GUI git often has a minimal PATH). */
export function resolveMicroCommitBunBin(root: string): string {
  const fromEnv = process.env.COMPOSE_BUN?.trim();
  if (fromEnv) return fromEnv;
  const argv0 = process.argv[0] ?? "";
  if (/bun(\.exe)?$/i.test(argv0)) return argv0;
  const win = process.platform === "win32";
  const home = process.env.HOME ?? process.env.USERPROFILE ?? "";
  const bunInstall = process.env.BUN_INSTALL ?? join(home, ".bun");
  const candidates = [join(root, "node_modules", ".bin", win ? "bun.cmd" : "bun"), join(root, "node_modules", ".bin", "bun.exe"), join(bunInstall, "bin", win ? "bun.exe" : "bun"), join(bunInstall, "bin", "bun")];
  for (const c of candidates) {
    if (c && existsSync(c)) return c;
  }
  const which = spawnSync(win ? "where" : "which", ["bun"], { encoding: "utf8", shell: win });
  if (which.status === 0) {
    const first = (which.stdout ?? "")
      .split(/\r?\n/)
      .map((l) => l.trim())
      .find(Boolean);
    if (first && existsSync(first)) return first;
  }
  return win ? "bun.exe" : "bun";
}

const MICRO_COMMIT_SEED_EMPTY_GK_SH = `compose_micro_commit_seed_empty_gk() {
  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0
  GK_TEMPLATE="$GIT_DIR/${GK_COMMIT_TEMPLATE_FILE}"
  if [ -d "$GIT_DIR" ]; then
    for f in "$GIT_DIR"/gkcommittemplate*; do
      [ -e "$f" ] || continue
      rm -f "$f" 2>/dev/null || true
    done
  fi
  : >"$GK_TEMPLATE" 2>/dev/null || true
  git config --local commit.template "$GK_TEMPLATE" 2>/dev/null || true
}`;

const MICRO_COMMIT_DISABLE_PREPARE_SH = `${MICRO_COMMIT_SEED_EMPTY_GK_SH}
compose_micro_commit_disable_prepare() {
  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0
  for f in "$GIT_DIR"/compose-micro-commit-*; do
    [ -e "$f" ] || continue
    rm -f "$f" 2>/dev/null || true
  done
  compose_micro_commit_seed_empty_gk
}`;

const MICRO_COMMIT_WIPE_FULL_SH = `${MICRO_COMMIT_SEED_EMPTY_GK_SH}
compose_micro_commit_wipe() {
  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0
  compose_micro_commit_seed_empty_gk
  for msg in COMMIT_EDITMSG MERGE_MSG SQUASH_MSG; do
    if [ -f "$GIT_DIR/$msg" ]; then
      : >"$GIT_DIR/$msg" 2>/dev/null || true
    fi
  done
  if [ -d "$GIT_DIR" ]; then
    for f in "$GIT_DIR"/compose-micro-commit-*; do
      [ -e "$f" ] || continue
      rm -f "$f" 2>/dev/null || true
    done
  fi
}`;

const MICRO_COMMIT_RESOLVE_REPO_CLI_SH = `compose_resolve_repo_cli() {
  ROOT="$1"
  if [ -n "$REPO_CLI_BIN" ] && [ -x "$REPO_CLI_BIN" ]; then
    echo "$REPO_CLI_BIN"
    return
  fi
  if [ -x "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client" ]; then
    echo "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client"
    return
  fi
  if [ -x "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client.exe" ]; then
    echo "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/client.exe"
    return
  fi
}`;

const MICRO_COMMIT_RESOLVE_BUN_SH = `compose_resolve_bun() {
  ROOT="$1"
  if [ -n "$COMPOSE_BUN" ] && [ -x "$COMPOSE_BUN" ]; then
    echo "$COMPOSE_BUN"
    return
  fi
  BUN_PIN="$ROOT/.🧬semio/🦑️repo/${MICRO_COMMIT_BUN_PIN}"
  if [ -f "$BUN_PIN" ]; then
    BUN=$(sed -n '1p' "$BUN_PIN" 2>/dev/null)
    if [ -n "$BUN" ] && [ -x "$BUN" ]; then
      echo "$BUN"
      return
    fi
  fi
  command -v bun 2>/dev/null || true
}`;

/** 🪝️Renders a portable `sh` git hook (LF, inline wipe; repo client binary for micro-commit). */
export function renderMicroCommitGitHook(name: "prepare-commit-msg" | (typeof MICRO_COMMIT_POST_WIPE_HOOKS)[number]): string {
  const isPostWipe = (MICRO_COMMIT_POST_WIPE_HOOKS as readonly string[]).includes(name);
  const lines = [
    "#!/usr/bin/env sh",
    isPostWipe ? MICRO_COMMIT_WIPE_FULL_SH : MICRO_COMMIT_DISABLE_PREPARE_SH,
    "ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0",
    'cd "$ROOT" || exit 0',
    "GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || exit 0",
    MICRO_COMMIT_RESOLVE_REPO_CLI_SH,
    MICRO_COMMIT_RESOLVE_BUN_SH,
  ];
  if (isPostWipe) {
    lines.push('CLI=$(compose_resolve_repo_cli "$ROOT")', '[ -n "$CLI" ] && "$CLI" micro-commit reset 2>/dev/null || true', "compose_micro_commit_wipe", "exit 0");
  } else {
    lines.push(
      '[ ! -f "$GIT_DIR/compose-micro-commit-active" ] && {',
      "  compose_micro_commit_seed_empty_gk",
      "  exit 0",
      "}",
      'BUN=$(compose_resolve_bun "$ROOT")',
      '[ -z "$BUN" ] && { compose_micro_commit_disable_prepare; exit 0; }',
      '"$BUN" "$ROOT/📜️script.ts" micro-commit prepare-commit-msg "$1" "$2" 2>/dev/null || compose_micro_commit_disable_prepare',
      "exit 0",
    );
  }
  return `${lines.join("\n")}\n`;
}

function writeMicroCommitHookFile(path: string, body: string): void {
  writeFileSync(path, body.replace(/\r\n/g, "\n"), "utf8");
  try {
    chmodSync(path, 0o755);
  } catch {
    /* windows */
  }
}

export function installMicroCommitGitHooks(root: string): void {
  const bunBin = resolveMicroCommitBunBin(root).replace(/\r/g, "");
  mkdirSync(getRepoMetaDir(root), { recursive: true });
  writeFileSync(join(getRepoMetaDir(root), MICRO_COMMIT_BUN_PIN), `${bunBin}\n`, "utf8");
  const hooksDir = join(root, ".git", "hooks");
  const repoHooksDir = join(root, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks");
  mkdirSync(hooksDir, { recursive: true });
  mkdirSync(repoHooksDir, { recursive: true });
  for (const name of [...MICRO_COMMIT_POST_WIPE_HOOKS, "prepare-commit-msg"] as const) {
    const body = renderMicroCommitGitHook(name);
    writeMicroCommitHookFile(join(repoHooksDir, name), body);
    writeMicroCommitHookFile(join(hooksDir, name), body);
  }
  const stalePreCommit = join(hooksDir, "pre-commit");
  if (existsSync(stalePreCommit)) rmSync(stalePreCommit, { force: true });
  const repoPreCommit = join(repoHooksDir, "pre-commit");
  if (existsSync(repoPreCommit)) rmSync(repoPreCommit, { force: true });
}
