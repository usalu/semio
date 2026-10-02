import {openSync,writeSync,closeSync,readFileSync} from "node:fs";
import {spawn} from "node:child_process";
import {terminateOwnedProcessTree} from "../🪓️termination/🟦️.ts";

export type OwnedProcessCaptureResult = { status: number | null; signal: string | null; stdout: string; stderr: string } & { reason?: "exit" | "timeout" | "cancelled" | "output-limit" | "spawn-error" };
export type OwnedProcessCaptureOptions = {
  cwd: string;
  env: Readonly<Record<string, string | undefined>>;
  budgetMs: number;
  maxOutputBytes: number;
  stdoutPath: string;
  stderrPath: string;
  cancelled: () => boolean;
};
/** 📥️ Captures a process into caller-owned evidence files; zero disables its deadline while retaining cancellation and output limits. */
export async function captureOwnedProcess(command: string, args: string[], options: OwnedProcessCaptureOptions): Promise<OwnedProcessCaptureResult> {
  const stdout = openSync(options.stdoutPath, "wx", 0o600);
  let stderr: number;
  try { stderr = openSync(options.stderrPath, "wx", 0o600); }
  catch (error) { closeSync(stdout); throw error; }
  if (options.cancelled()) {
    closeSync(stdout); closeSync(stderr);
    return { status: null, signal: null, reason: "cancelled", stdout: "", stderr: "" };
  }
  return await new Promise((resolveResult) => {
    let reason: OwnedProcessCaptureResult["reason"] = "exit";
    let count = 0;
    let finished = false;
    const child = spawn(command, args, { cwd: options.cwd, env: options.env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
    const terminate = (cause: NonNullable<OwnedProcessCaptureResult["reason"]>): void => {
      if (reason !== "exit") return;
      reason = cause;
      if (child.pid) terminateOwnedProcessTree(child.pid);
    };
    const append = (descriptor: number, bytes: Buffer): void => {
      const remaining = Math.max(0, options.maxOutputBytes - count);
      if (remaining) writeSync(descriptor, bytes.subarray(0, remaining));
      count += bytes.length;
      if (count > options.maxOutputBytes) terminate("output-limit");
    };
    child.stdout?.on("data", (bytes) => append(stdout, bytes));
    child.stderr?.on("data", (bytes) => append(stderr, bytes));
    const timer = options.budgetMs > 0 ? setTimeout(() => terminate("timeout"), options.budgetMs) : undefined;
    const cancel = setInterval(() => {
      if (options.cancelled()) terminate("cancelled");
    }, 100);
    child.on("error", (error) => {
      reason = "spawn-error";
      append(stderr, Buffer.from(error.message));
    });
    child.on("close", (status, signal) => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      clearInterval(cancel);
      closeSync(stdout);
      closeSync(stderr);
      resolveResult({ status, signal, reason, stdout: readFileSync(options.stdoutPath, "utf8"), stderr: readFileSync(options.stderrPath, "utf8") });
    });
  });
}
