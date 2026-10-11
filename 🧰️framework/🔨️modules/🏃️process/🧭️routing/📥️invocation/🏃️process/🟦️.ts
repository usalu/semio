import schema from "./🧬️schema/🔣️.json";
import policySchema from "../🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import { checkScriptInvocation, readScriptPolicy, type ScriptInvocation, type ScriptPolicy } from "../🟦️.ts";

/** 📨️ Transfers the original process deadline without rebasing child authority. */
export type ScriptProcessEnvelope = Readonly<{ version: 1; policy: ScriptPolicy; deadlineEpochMilliseconds: number | null; capabilities: object }>;
export const SCRIPT_PROCESS_INVOCATION_ENV = "SEMIO_SCRIPT_PROCESS_INVOCATION";

/** 🔎️ Validates the authored process envelope and its remaining original authority. */
export function readScriptProcessEnvelope(value: unknown, now: number): ScriptProcessEnvelope {
  if (schema.properties.policy.$ref !== policySchema.$id) throw Error("Original Script policy schema owner required");
  const admittedSchema = { ...schema, properties: { ...schema.properties, policy: policySchema } };
  if (validateJsonSchemaSubset(admittedSchema, value).length) throw Error("Complete original Script process envelope required");
  const envelope = value as ScriptProcessEnvelope;
  if (!Number.isFinite(now)) throw Error("Original Script process clock refused");
  if (envelope.deadlineEpochMilliseconds !== null) {
    const remaining = envelope.deadlineEpochMilliseconds - now;
    if (remaining <= 0 || remaining > envelope.policy.maximumElapsedMilliseconds) throw Error("Original Script process deadline refused");
  }
  return envelope;
}

/** 🌱️ Creates the original process deadline from its explicitly supplied authored policy. */
export function createScriptProcessEnvelope(policy: ScriptPolicy, capabilities: object, now: number): ScriptProcessEnvelope {
  const original = readScriptPolicy(policy);
  return readScriptProcessEnvelope({ version: 1, policy: original, deadlineEpochMilliseconds: original.maximumElapsedMilliseconds === 0 ? null : now + original.maximumElapsedMilliseconds, capabilities }, now);
}

/** 🎛️ Owns actual process cancellation, progress and deadline until dispatch returns. */
export async function withScriptProcessEnvelope<T>(envelope: ScriptProcessEnvelope, run: (invocation: ScriptInvocation) => Promise<T>): Promise<T> {
  const admitted = readScriptProcessEnvelope(envelope, Date.now()), started = performance.now(), deadline = admitted.deadlineEpochMilliseconds, initial = deadline === null ? null : deadline - Date.now();
  const controller = new AbortController(), interrupt = (): void => controller.abort(Error("Original Script process interrupted")), terminate = (): void => controller.abort(Error("Original Script process terminated"));
  const timeout = initial === null ? undefined : setTimeout(() => controller.abort(Error("Original Script process deadline exhausted")), initial);
  timeout?.unref();
  process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
  const invocation: ScriptInvocation = {
    policy: admitted.policy,
    control: {
      signal: controller.signal,
      remainingMilliseconds: () => deadline === null || initial === null ? null : Math.max(0, Math.floor(Math.min(deadline - Date.now(), initial - (performance.now() - started)))),
      publish: progress => { process.stderr.write(`[script] ${JSON.stringify(progress)}\n`); },
      yieldContinuation: () => new Promise<void>(done => setImmediate(done)),
    },
    capabilities: admitted.capabilities,
  };
  try { checkScriptInvocation(invocation); const result = await run(invocation); checkScriptInvocation(invocation); return result; }
  finally { if (timeout) clearTimeout(timeout); process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
}

/** 📥️ Receives a mandatory original parent envelope from the actual process environment. */
export async function receiveScriptProcessInvocation<T>(environment: Readonly<Record<string, string | undefined>>, run: (invocation: ScriptInvocation) => Promise<T>): Promise<T> {
  const wire = environment[SCRIPT_PROCESS_INVOCATION_ENV];
  if (wire === undefined) throw Error("Original parent Script process invocation required");
  return withScriptProcessEnvelope(readScriptProcessEnvelope(JSON.parse(wire), Date.now()), run);
}

/** 📤️ Carries the same admitted deadline into an owned child process environment. */
export function scriptProcessEnvironment(envelope: ScriptProcessEnvelope, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  return { ...environment, [SCRIPT_PROCESS_INVOCATION_ENV]: JSON.stringify(readScriptProcessEnvelope(envelope, Date.now())) };
}

/** 📤️ Hands the caller's current original invocation, with its remaining deadline, to an owned child process environment. */
export function scriptInvocationEnvironment(invocation: ScriptInvocation, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  checkScriptInvocation(invocation);
  const now = Date.now(), remaining = invocation.control.remainingMilliseconds();
  return scriptProcessEnvironment(readScriptProcessEnvelope({ version: 1, policy: invocation.policy, deadlineEpochMilliseconds: remaining === null ? null : now + remaining, capabilities: invocation.capabilities }, now), environment);
}
