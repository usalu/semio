import {openSync,writeSync,closeSync,readFileSync} from "node:fs";
import {spawn} from "node:child_process";
import {terminateOwnedProcessTree} from "../🪓️termination/🟦️.ts";
import { advanceScriptInvocation, checkScriptInvocation, type ScriptInvocation, type ScriptProgress } from "../🧭️routing/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../🧬️schema/✅️validator/🟦️.ts";
import policySchema from "./📋️policy/🧬️schema/🔣️.json";

export type OwnedProcessCaptureResult = { status: number | null; signal: string | null; stdout: string; stderr: string } & { reason?: "exit" | "timeout" | "cancelled" | "output-limit" | "spawn-error" };
export type OwnedProcessCaptureOptions = {
  invocation: ScriptInvocation;
  cwd: string;
  env: Readonly<Record<string, string | undefined>>;
  budgetMs: number;
  maxOutputBytes: number;
  stdoutPath: string;
  stderrPath: string;
};
/** 📥️ Captures evidence under the original caller's signal, decreasing time and awaited publication. */
export async function captureOwnedProcess(command: string, args: string[], options: OwnedProcessCaptureOptions): Promise<OwnedProcessCaptureResult> {
  const started = performance.now(), invocation = options.invocation;
  if (validateJsonSchemaSubset(policySchema, { budgetMs: options.budgetMs, maxOutputBytes: options.maxOutputBytes }).length) throw Error("Complete original capture policy required");
  try { checkScriptInvocation(invocation); }
  catch (error) {
    if (invocation?.control?.signal instanceof AbortSignal && invocation.control.signal.aborted && error === invocation.control.signal.reason) return { status: null, signal: null, reason: "cancelled", stdout: "", stderr: "" };
    throw error;
  }
  const remaining = (): number | null => {
    const parent = invocation.control.remainingMilliseconds(), phase = options.budgetMs === 0 ? null : options.budgetMs - (performance.now() - started);
    return parent === null ? phase : phase === null ? parent : Math.min(parent, phase);
  };
  const publish = async (stage: ScriptProgress["stage"]): Promise<void> => {
    const budget = remaining();
    if (budget !== null && budget <= 0) throw new CaptureInterrupted("timeout");
    let timer: ReturnType<typeof setTimeout> | undefined, abort: (() => void) | undefined;
    const interrupted = new Promise<never>((_, reject) => {
      abort = () => reject(new CaptureInterrupted("cancelled"));
      invocation.control.signal.addEventListener("abort", abort, { once: true });
      if (invocation.control.signal.aborted) abort();
      if (budget !== null) timer = setTimeout(() => reject(new CaptureInterrupted("timeout")), budget);
    });
    try { await Promise.race([advanceScriptInvocation(invocation, command, stage), interrupted]); }
    finally { clearTimeout(timer); if (abort) invocation.control.signal.removeEventListener("abort", abort); }
  };
  try { await publish("running"); }
  catch (error) { if (error instanceof CaptureInterrupted) return { status: null, signal: null, reason: error.reason, stdout: "", stderr: "" }; throw error; }
  const admittedBudget = remaining();
  if (admittedBudget !== null && admittedBudget <= 0) return { status: null, signal: null, reason: "timeout", stdout: "", stderr: "" };
  const stdout = openSync(options.stdoutPath, "wx", 0o600);
  let stderr: number;
  try { stderr = openSync(options.stderrPath, "wx", 0o600); }
  catch (error) { closeSync(stdout); throw error; }
  if (invocation.control.signal.aborted) {
    closeSync(stdout); closeSync(stderr);
    return { status: null, signal: null, reason: "cancelled", stdout: "", stderr: "" };
  }
  const result = await new Promise<OwnedProcessCaptureResult>((resolveResult, rejectResult) => {
    let reason: OwnedProcessCaptureResult["reason"] = "exit";
    let count = 0;
    let finished = false;
    const child = spawn(command, args, { cwd: options.cwd, env: options.env, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
    const terminate = (cause: NonNullable<OwnedProcessCaptureResult["reason"]>): void => {
      if (reason !== "exit") return;
      reason = cause;
      if (child.pid) terminateOwnedProcessTree(child.pid);
    };
    const originalAbort = () => terminate("cancelled");
    invocation.control.signal.addEventListener("abort", originalAbort, { once: true });
    if (invocation.control.signal.aborted) originalAbort();
    const append = (descriptor: number, bytes: Buffer): void => {
      const remaining = Math.max(0, options.maxOutputBytes - count);
      if (remaining) writeSync(descriptor, bytes.subarray(0, remaining));
      count += bytes.length;
      if (count > options.maxOutputBytes) terminate("output-limit");
    };
    child.stdout?.on("data", (bytes) => append(stdout, bytes));
    child.stderr?.on("data", (bytes) => append(stderr, bytes));
    const timer = admittedBudget !== null ? setTimeout(() => terminate("timeout"), admittedBudget) : undefined;
    let publication: Promise<void> | undefined, publicationFailure: unknown;
    const progress = setInterval(() => {
      if (publication || finished) return;
      publication = publish("running").catch(error => { publicationFailure = error; terminate(error instanceof CaptureInterrupted ? error.reason : "cancelled"); }).finally(() => { publication = undefined; });
    }, 10000);
    child.on("error", (error) => {
      reason = "spawn-error";
      append(stderr, Buffer.from(error.message));
    });
    child.on("close", (status, signal) => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      clearInterval(progress);
      invocation.control.signal.removeEventListener("abort", originalAbort);
      closeSync(stdout);
      closeSync(stderr);
      const evidence = { status, signal, reason, stdout: readFileSync(options.stdoutPath, "utf8"), stderr: readFileSync(options.stderrPath, "utf8") };
      Promise.resolve(publication).then(() => publicationFailure && !(publicationFailure instanceof CaptureInterrupted) ? rejectResult(publicationFailure) : resolveResult(evidence), rejectResult);
    });
  });
  if (result.reason === "exit" || result.reason === "spawn-error") await publish("complete");
  return result;
}

/** 🛑️ Reports interruption from the same original signal or decreasing deadline. */
class CaptureInterrupted extends Error {
  constructor(readonly reason: "cancelled" | "timeout") { super("Owned capture stopped: " + reason); }
}
