import { prepareCargoWorkspaceInvocation } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { StringDecoder } from "node:string_decoder";
import { spawn } from "node:child_process";
import { buildBudgetMs, terminateOwnedProcessTree } from "../🟦️.ts";

/** ⏱️ Keeps long native queue waits observable until the owning operation completes. */
export function startNativeProgress(label: string, intervalMs = 10_000, output: (line: string) => void = (line) => console.log(line)): () => void {
  const started = Date.now(),
    progress = setInterval(() => output(`[${label}] running elapsedMs=${Date.now() - started}`), intervalMs);
  return () => clearInterval(progress);
}

/** 🏃️ Runs a bounded owned command with progress and process-tree cancellation. Its stdout (unless ignored) and stderr are piped and
 * forwarded, never inherited: an inherited pipe shares this Bun process's `O_NONBLOCK`, and a burst from the command then fails with
 * `EAGAIN` once a slow reader lets the pipe fill ([[cargoStreamingStatus]], ticket 26/09/23 W4). */
export async function runOwnedCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: { stdout?: "inherit" | "ignore"; env?: NodeJS.ProcessEnv; signal?: AbortSignal; onLine?: (line: string) => void } = {}): Promise<void> {
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  if (command === "cargo") prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, cwd);
  const child = spawn(command, args, { cwd, env: options.env ?? process.env, detached: process.platform !== "win32", stdio: ["inherit", options.stdout === "ignore" ? "ignore" : "pipe", "pipe"], windowsHide: true });
  child.stdout?.pipe(process.stdout, { end: false });
  child.stderr!.pipe(process.stderr, { end: false });
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
  const abort = (): void => { stopped ||= "cancelled"; if (child.pid) terminateOwnedProcessTree(child.pid); };
  const observe = (stream: NodeJS.ReadableStream): void => {
    const decoder = new StringDecoder("utf8");
    let pending = "";
    stream.on("data", (bytes: Buffer) => {
      pending += decoder.write(bytes);
      let boundary: number;
      while ((boundary = pending.indexOf("\n")) >= 0) {
        options.onLine!(pending.slice(0, boundary).replace(/\r$/u, ""));
        pending = pending.slice(boundary + 1);
      }
    });
    stream.once("end", () => { pending += decoder.end(); if (pending) options.onLine!(pending); });
  };
  if (options.onLine) { if (child.stdout) observe(child.stdout); observe(child.stderr!); }
  options.signal?.addEventListener("abort", abort, { once: true });
  if (options.signal?.aborted) abort();
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
    options.signal?.removeEventListener("abort", abort);
    stopProgress();
    if (timeout) clearTimeout(timeout);
    if (forceKill) clearTimeout(forceKill);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}
