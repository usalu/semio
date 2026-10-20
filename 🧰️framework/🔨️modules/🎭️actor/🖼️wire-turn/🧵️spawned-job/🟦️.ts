// #region 🧲️Header
/** 🧵️ The spawned-job drive: a `spawn-job` recovered off the boundary and pumped step by step until the kernel's rule completes it. */
// #endregion 🧲️Header

// #region 🔌️Imports
// 🧵️ The spawned-job drive RULE is the kernel's (Rust `kernel::spawned_job_completion` and this
// twin), not this module's: the wire layer recovers a `spawn-job` off the boundary and pumps it, the
// kernel alone decides what a transcript of `step-job` observations means. A value import, unlike
// `decodePackValue`'s injected codec, because nothing in `🎠️kernel`'s own import graph reaches
// back here — verified, not assumed.
import type { Effect } from "@semio-tech/framework-effect";
import type { SpawnedJobCompletion, SpawnedJobStep } from "@semio-tech/framework-effect/spawned-job";
import { spawnedJobCompletion, SPAWNED_JOB_DEADLINE_MS, SPAWNED_JOB_FUEL, SPAWNED_JOB_STEP_CEILING } from "@semio-tech/framework-effect/spawned-job";
import { type WireVariant, coerceWireBytes } from "../🔖️turn-result/🟦️.ts";
import { wireOptionValue, wireSpawnJob } from "../🟦️.ts";
// #endregion 🔌️Imports

//#region 🧵️SpawnedJob
/** 🧵️ The two guest-job doors a host must own to honour a `spawn-job` effect — `jobs::start-job` and
 * `jobs::step-job` on the instance the job was placed on (`🔌️plugin/🧬️schema/📜️.wit`'s `interface
 * jobs`). Injected rather than imported so this module keeps no dependency on a particular transport:
 * the wgpu bridge hands over its `ShardClient` bound to one actor, and any other host its own. */
export type SpawnedJobPort = {
  readonly startJob: (job: bigint, kind: string, input: Uint8Array) => Promise<void>;
  readonly stepJob: (job: bigint, budget: { readonly fuel: bigint; readonly deadlineMs: number }) => Promise<unknown>;
};

/** 🧵️ One spawned job, as this side of the ABI states it. `stepCeiling` exists for a law that wants
 * to drive the stall arm without 32 real round trips; production leaves it alone. */
export type SpawnedJobDrive = {
  readonly job: bigint;
  readonly kind: string;
  readonly input: Uint8Array;
  readonly port: SpawnedJobPort;
  readonly stepCeiling?: number;
  readonly signal?: AbortSignal;
  readonly onStep?: (step: { readonly job: bigint; readonly index: number; readonly status: SpawnedJobStep["status"] }) => void;
};

/** 🧵️ Normalizes one `jobs::job-step` observation. jco hands the WIT variant as `{tag, val}`; the
 * shard worker already normalizes it to `{status, value}` for `ShardClient.stepJob`, and a
 * `running` carries an OPTIONAL progress pack. Both shapes are real on the same door, so reading only
 * one of them turns a finished job into a stall — the same class of defect that lost
 * `download-media-export.encoding` (this module's `wireOptionValue`). */
export function wireJobStep(raw: unknown): SpawnedJobStep {
  const record = (raw ?? {}) as { readonly tag?: unknown; readonly val?: unknown; readonly status?: unknown; readonly value?: unknown; readonly progress?: unknown };
  const status = typeof record.status === "string" ? record.status : typeof record.tag === "string" ? record.tag : "";
  const payload = record.value !== undefined ? record.value : record.val;
  if (status === "running") return { status: "running" };
  if (status === "done" || status === "failed") return { status, value: coerceWireBytes(wireOptionValue(payload) ?? []) };
  throw new Error(`job-step.status-invalid ${JSON.stringify(status).slice(0, 60)}`);
}

/**
 * 🧵️ Drives ONE `Effect::SpawnJob` to a terminal `job-step` — the host half of the ABI's job
 * seam, shared verbatim by every renderer door so the two targets can never drift into two
 * protocols (this module's own header: the "third divergent copy" hazard).
 *
 * The job is started once, then stepped with the kernel's STATIC admission budget until it reports a
 * terminal step or the ceiling is reached. The transcript — never this loop — decides what happened:
 * `kernel::spawnedJobCompletion` owns that rule and its Rust twin answers identically.
 *
 * A host that skips this pump is not merely slow, it is silently broken: every framework reserved
 * tool verb (`interactionSelect`/`interactionHover`/`clearSelection`) is delivered as this effect and
 * nothing else, so the actor publishes a perfect interaction message and never applies it — measured
 * on 6118, 18 dropped `spawn-job` effects per run
 * (`📓️wgpu-world3d-interaction-2026-09-13.md` §7).
 */
export async function driveSpawnedJob(drive: SpawnedJobDrive): Promise<SpawnedJobCompletion> {
  const ceiling = Math.max(1, Math.min(drive.stepCeiling ?? SPAWNED_JOB_STEP_CEILING, SPAWNED_JOB_STEP_CEILING));
  const budget = { fuel: SPAWNED_JOB_FUEL, deadlineMs: SPAWNED_JOB_DEADLINE_MS } as const;
  await drive.port.startJob(drive.job, drive.kind, drive.input);
  const steps: SpawnedJobStep[] = [];
  for (let index = 0; index < ceiling; index += 1) {
    if (drive.signal?.aborted === true) break;
    const step = wireJobStep(await drive.port.stepJob(drive.job, budget));
    steps.push(step);
    drive.onStep?.({ job: drive.job, index, status: step.status });
    if (step.status !== "running") break;
  }
  return spawnedJobCompletion(steps);
}

/** 🧵️ The actor-boundary event a completed spawned job owes its guest — the WIT `job-completed-event`
 * (`{job: u64, outcome: completion-result}`) in the same `{kind, payload}` envelope every other
 * host-submitted event uses. The guest correlates on `job` ALONE: the job id IS the parked request id
 * (`⚛️reactor/🔄️turn/🦀️.rs`), so no host-side request table exists or is needed. */
export function spawnedJobCompletedEvent(job: bigint, completion: SpawnedJobCompletion): { readonly kind: "job-completed"; readonly payload: { readonly job: bigint; readonly outcome: { readonly tag: "ok" | "fault"; readonly val: readonly number[] } } } {
  const bytes = "ok" in completion.outcome ? completion.outcome.ok : completion.outcome.fault;
  return { kind: "job-completed", payload: { job, outcome: { tag: "ok" in completion.outcome ? "ok" : "fault", val: Array.from(bytes) } } };
}

/** 🧵️ Every `spawn-job` a turn handed back, decoded once. Kept here rather than at each door so a
 * renderer never re-implements "which of these effects is a job". */
export function wireSpawnedJobs(effects: readonly WireVariant[]): readonly Extract<Effect, { readonly spawnJob: unknown }>["spawnJob"][] {
  return effects.filter((effect) => effect.tag === "spawn-job").map((effect) => wireSpawnJob(effect).spawnJob);
}
//#endregion 🧵️SpawnedJob
