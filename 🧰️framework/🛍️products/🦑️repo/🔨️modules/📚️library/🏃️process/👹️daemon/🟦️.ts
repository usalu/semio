import { type ChildProcess, spawn } from "node:child_process";
import { terminateOwnedProcessTree } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { daemonBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

/**
 * ⏱️SIGKILLs `pid`'s whole process tree, not just `pid` — a timed-out `spawnSync`/`execFileSync` only signals the
 * direct child, leaking forked worker pools (e.g. vitest's `workers/forks.js`) that keep burning CPU indefinitely.
 * Requires the child to have been spawned with `detached: true` on POSIX so `pid` is its own process-group leader.
 */
function killBudgetTree(pid: number): void {
  terminateOwnedProcessTree(pid);
}

export interface SpawnDaemonHandle {
  child: ChildProcess;
  kill: () => void;
}

/** 🌙️Spawns a long-lived daemon under [[daemonBudgetMs]]; [[SpawnDaemonHandle.kill]] tears down the whole process tree. */
export function spawnDaemon(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv; stdio?: "inherit" | "pipe" | "ignore" } = {}): SpawnDaemonHandle {
  const child = spawn(cmd, args, {
    cwd: opts.cwd,
    env: opts.env ?? process.env,
    stdio: opts.stdio ?? "inherit",
    detached: process.platform !== "win32",
  });
  let killed = false;
  const budgetMs = daemonBudgetMs();
  const timer = budgetMs > 0 ? setTimeout(() => {
    killed = true;
    if (child.pid) killBudgetTree(child.pid);
  }, budgetMs) : undefined;
  const kill = () => {
    if (timer !== undefined) clearTimeout(timer);
    if (!killed && child.pid) killBudgetTree(child.pid);
  };
  child.on("exit", () => { if (timer !== undefined) clearTimeout(timer); });
  return { child, kill };
}

/** ⏳️Polls `url` with `fetch` every 500ms until it responds `ok`, throwing once `timeoutMs` elapses —
 * the shared readiness gate every `spawnDaemon`-fronted dev/static server (Storybook's own Playwright
 * pipeline, the demonstrator's) waits on before handing off to Playwright. */
export async function waitForHttpUrl(url: string, timeoutMs: number): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      /* retry */
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Timed out waiting for ${url}`);
}
