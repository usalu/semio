import schema from "./🧬️schema/🔣️.json";
import { requireExactKeys, requireRecord, validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";

/** 🧬️ Names the original caller's finite or ordinary no-deadline command authority. */
export type ScriptPolicy = Readonly<{ version: 1; owner: string; maximumElapsedMilliseconds: number }>;
/** 📣️ Identifies actual neutral command ownership transitions. */
export type ScriptProgress = Readonly<{ owner: string; command: string; stage: "contribution" | "loading" | "running" | "complete" }>;
/** 🎛️ Receives the caller's existing cancellation, time, progress and continuation ports. */
export interface ScriptControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: ScriptProgress): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}
/** 📥️ Carries exact incoming control and application capabilities without creating authority. */
export interface ScriptInvocation<Capabilities extends object = object> {
  readonly policy: ScriptPolicy;
  readonly control: ScriptControl;
  readonly capabilities: Capabilities;
}

/** 🔐️ Admits a complete authored wire policy including explicit zero for no deadline. */
export function readScriptPolicy(value: unknown): ScriptPolicy {
  if (validateJsonSchemaSubset(schema, value).length) throw Error("Complete Script policy required");
  return value as ScriptPolicy;
}

/** ⏳️ Checks original incoming authority before acquiring or executing an owner. */
export function checkScriptInvocation<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>): void {
  const row = requireRecord(invocation, "Script invocation");
  requireExactKeys(row, ["policy", "control", "capabilities"], "Script invocation");
  const policy = readScriptPolicy(row.policy), control = requireRecord(row.control, "Script control");
  requireExactKeys(control, ["signal", "remainingMilliseconds", "publish", "yieldContinuation"], "Script control");
  requireRecord(row.capabilities, "Script capabilities");
  if (!(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Original Script control ports required");
  invocation.control.signal.throwIfAborted();
  const remaining = invocation.control.remainingMilliseconds();
  if (policy.maximumElapsedMilliseconds === 0 ? remaining !== null : remaining === null || !Number.isFinite(remaining) || remaining <= 0 || remaining > policy.maximumElapsedMilliseconds) throw Error("Original Script deadline refused");
}

/** 🔄️ Publishes and yields through the same caller-owned operation. */
export async function advanceScriptInvocation<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>, command: string, stage: ScriptProgress["stage"]): Promise<void> {
  checkScriptInvocation(invocation);
  await invocation.control.publish({ owner: invocation.policy.owner, command, stage });
  checkScriptInvocation(invocation);
  await invocation.control.yieldContinuation();
  checkScriptInvocation(invocation);
}

/** ⏱️ Intersects a supplied child budget with the same original operation authority. */
export function scriptInvocationBudget<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>, requestedMilliseconds: number): number {
  checkScriptInvocation(invocation);
  if (!Number.isSafeInteger(requestedMilliseconds) || requestedMilliseconds < 0) throw Error("Original child process budget refused");
  const remaining = invocation.control.remainingMilliseconds();
  if (remaining === null) return requestedMilliseconds;
  const budget = Math.floor(requestedMilliseconds === 0 ? remaining : Math.min(requestedMilliseconds, remaining));
  if (budget < 1) throw Error("Original child process deadline exhausted");
  return budget;
}
