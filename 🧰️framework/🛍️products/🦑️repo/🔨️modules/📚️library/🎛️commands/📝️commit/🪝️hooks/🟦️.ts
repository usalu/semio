import { join } from "node:path";
import { chmodSync, existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { GK_COMMIT_TEMPLATE_FILE } from "../📄️prepared-message/🟦️.ts";

const MICRO_COMMIT_POST_WIPE_HOOKS = ["post-commit", "post-checkout", "post-merge", "post-rewrite"] as const;

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
  NATIVE_PIN="$ROOT/.🧬semio/🦑️repo/native-repo-path"
  if [ -f "$NATIVE_PIN" ]; then
    REPO_CLI=$(sed -n '1p' "$NATIVE_PIN" 2>/dev/null)
    if [ -n "$REPO_CLI" ] && [ -x "$REPO_CLI" ]; then
      echo "$REPO_CLI"
      return
    fi
  fi
  if [ -n "$HOME" ] && [ -x "$HOME/.local/bin/repo" ]; then
    echo "$HOME/.local/bin/repo"
    return
  fi
  REPO_CLI=$(command -v repo 2>/dev/null) || return
  if [ -n "$REPO_CLI" ] && [ -x "$REPO_CLI" ]; then
    echo "$REPO_CLI"
  fi
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
  ];
  if (isPostWipe) {
    lines.push('CLI=$(compose_resolve_repo_cli "$ROOT")', '[ -n "$CLI" ] && "$CLI" micro-commit reset 2>/dev/null || true', "compose_micro_commit_wipe", "exit 0");
  } else {
    lines.push(
      '[ ! -f "$GIT_DIR/compose-micro-commit-active" ] && {',
      "  compose_micro_commit_seed_empty_gk",
      "  exit 0",
      "}",
      'CLI=$(compose_resolve_repo_cli "$ROOT")',
      '[ -z "$CLI" ] && { compose_micro_commit_disable_prepare; exit 0; }',
      '"$CLI" micro-commit prepare-commit-msg "$1" "$2" 2>/dev/null || compose_micro_commit_disable_prepare',
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
