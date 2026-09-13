import assert from "node:assert/strict";
import { spawn } from "node:child_process";

/** 🏃️ Captures a bounded subprocess while retaining progress and cancellation. */
export async function captureArtifactContract(command: string, args: string[], cwd: string, timeoutMs: number): Promise<string> {
  const child = spawn(command, args, { cwd, env: process.env, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  let stdout = "", stderr = "", stopped = "", forceKill: ReturnType<typeof setTimeout> | undefined;
  const terminate = (reason: string): void => {
    stopped ||= reason;
    if (!child.pid) return;
    if (process.platform === "win32") child.kill("SIGTERM");
    else {
      try { process.kill(-child.pid, "SIGTERM"); } catch {}
      forceKill ??= setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch {} }, 2000);
      forceKill.unref();
    }
  };
  child.stdout.on("data", (chunk) => { stdout += chunk; if (stdout.length > 64 * 1024 * 1024) terminate("output limit"); });
  child.stderr.on("data", (chunk) => { stderr += chunk; if (stderr.length > 64 * 1024 * 1024) terminate("output limit"); });
  const interrupt = (): void => terminate("SIGINT");
  const stop = (): void => terminate("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", stop);
  const started = Date.now();
  const progress = setInterval(() => console.log(`[artifact-package-contract] ${command} running elapsedMs=${Date.now() - started}`), 10_000);
  const timeout = setTimeout(() => terminate(`timeout ${timeoutMs}ms`), timeoutMs);
  try {
    const status = await new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code) => accept(code ?? 1)); });
    assert.equal(stopped, "", `${command} stopped: ${stopped}\n${stderr}`);
    assert.equal(status, 0, stderr || `${command} exited with status ${status}`);
    return stdout;
  } finally {
    clearInterval(progress);
    clearTimeout(timeout);
    if (forceKill) clearTimeout(forceKill);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", stop);
  }
}

