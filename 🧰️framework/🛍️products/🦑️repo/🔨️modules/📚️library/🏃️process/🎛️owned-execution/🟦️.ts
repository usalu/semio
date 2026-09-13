import { spawn } from "node:child_process";
import { buildBudgetMs } from "../🟦️.ts";

/** ⏱️ Keeps long native queue waits observable until the owning operation completes. */
export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => console.log(line)): () => void {
  const started = Date.now(),
    progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. */
export async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: { stdout?: "inherit" | "ignore"; env?: NodeJS.ProcessEnv } = {}): Promise<void> {
  const child = spawn(command, args, { cwd, env: options.env ?? process.env, detached: process.platform !== "win32", stdio: ["inherit", options.stdout ?? "inherit", "inherit"], windowsHide: true });
  let stopped = "",
    forceKill: ReturnType<typeof setTimeout> | undefined;
  const terminate = (reason: string): void => {
    stopped ||= reason;
    if (!child.pid) return;
    if (process.platform === "win32") (globalThis as any).Bun.spawnSync(["taskkill", "/pid", String(child.pid), "/t", "/f"], { stdout: "ignore", stderr: "ignore" });
    else {
      try {
        process.kill(-child.pid, "SIGTERM");
      } catch {}
      forceKill ??= setTimeout(() => {
        try {
          process.kill(-child.pid!, "SIGKILL");
        } catch {}
      }, 2_000);
      forceKill.unref();
    }
  };
  const interrupt = (): void => terminate("SIGINT");
  const stop = (): void => terminate("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", stop);
  const stopProgress = startNativeProgress(label);
  const timeout = timeoutMs > 0 ? setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs) : undefined;
  try {
    const status = await new Promise<number>((accept) => {
      child.once("error", (error) => {
        console.error(error.message);
        accept(1);
      });
      child.once("close", (code) => accept(code ?? 1));
    });
    if (stopped) throw new Error(`${label} stopped: ${stopped}`);
    if (status !== 0) throw new Error(`${label} failed (${status})`);
  } finally {
    stopProgress();
    if (timeout) clearTimeout(timeout);
    if (forceKill) clearTimeout(forceKill);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}
