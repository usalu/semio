import { join } from "node:path";
import { spawnCapturedSync } from "../../../🏃️process/🔍️probe/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";

/** 🌳️Resolves the git worktree root (never a subdirectory of the monorepo). */
export function gitRepoRoot(start: string): string {
  const r = spawnCapturedSync("git", ["rev-parse", "--show-toplevel"], { cwd: start, env: gitSpawnEnv() });
  if (r.status === 0) {
    const top = r.stdout.toString("utf8").trim();
    if (top) return top;
  }
  return start;
}

export function gitTrackedPaths(root: string): string[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnCapturedSync("git", ["ls-files", "-z"], { cwd: repoRoot, env: gitSpawnEnv() });
  if (r.status !== 0) return [];
  return r.stdout.toString("utf8").split("\0").filter(Boolean);
}

export function gitDir(root: string): string {
  const repoRoot = gitRepoRoot(root);
  const r = spawnCapturedSync("git", ["rev-parse", "--git-dir"], { cwd: repoRoot, env: gitSpawnEnv() });
  if (r.status !== 0) return join(repoRoot, ".git");
  const dir = r.stdout.toString("utf8").trim();
  return dir.startsWith("/") ? dir : join(repoRoot, dir);
}

const GIT_INDEX_LOCK_RE = /Unable to create '.*index\.lock'/;

const GIT_INDEX_LOCK_MAX_ATTEMPTS = 20;

const GIT_INDEX_LOCK_RETRY_DELAY_MS = 300;

/** ⏳️Retries on a concurrent `index.lock` (other agent/IDE git process mid-write) instead of failing the whole commit. */
export function git(root: string, args: string[]): { ok: boolean; out: string } {
  for (let attempt = 1; attempt <= GIT_INDEX_LOCK_MAX_ATTEMPTS; attempt++) {
    const r = spawnCapturedSync("git", args, { cwd: root, env: gitSpawnEnv() });
    if (r.status === 0) return { ok: true, out: r.stdout.toString("utf8").trim() };
    const out = (r.stderr.length > 0 ? r.stderr : r.stdout).toString("utf8").trim();
    if (attempt < GIT_INDEX_LOCK_MAX_ATTEMPTS && GIT_INDEX_LOCK_RE.test(out)) {
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, GIT_INDEX_LOCK_RETRY_DELAY_MS);
      continue;
    }
    return { ok: false, out };
  }
  return { ok: false, out: "git: index.lock retry budget exhausted" };
}

export function gitCachedNames(root: string, extra: string[] = []): string[] {
  const r = spawnCapturedSync("git", ["diff", "--cached", "--name-only", "-z", ...extra], { cwd: root, env: gitSpawnEnv() });
  if (r.status !== 0) return [];
  const raw = r.stdout.toString("utf8");
  if (!raw) return [];
  return raw.split("\0").filter(Boolean);
}

export function gitEmail(root: string): string {
  return git(root, ["config", "user.email"]).out;
}
