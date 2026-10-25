/** 🛠️ Spawned jobs: placement, the framework reserved job kind, the drive limits, step and completion shapes and the drive error.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region InvocationResponse
//#region 🧵️SpawnedJobDrive
/** 🚦 Where a spawned job runs — the WIT `enum job-placement`, which jco lowers to a BARE string
 * rather than a `{tag}` record. TS twin of Rust `kernel::JobPlacement`. */
export type JobPlacement = "inline" | "isolated" | "exclusive";

/** 🗺️ The whole placement vocabulary, in WIT declaration order — twin of Rust `JobPlacement::ALL`. */
export const JOB_PLACEMENTS: readonly JobPlacement[] = Object.freeze(["inline", "isolated", "exclusive"] as const);

/** 🔎️ Resolves a wire placement name, or `undefined`. Never defaults: guessing `inline` for an
 * unrecognised spelling would run a pooled job inside the spawning instance's own turn budget.
 * Twin of Rust `JobPlacement::from_wire_name`. */
export function jobPlacementFromWireName(name: unknown): JobPlacement | undefined {
  return typeof name === "string" && (JOB_PLACEMENTS as readonly string[]).includes(name) ? (name as JobPlacement) : undefined;
}

/** 🧰️ The `spawn-job` kind of every framework reserved tool verb (undo, redo, copy/paste, selection and
 * interaction verbs): a live job each host starts, steps to its end and completes on the spawning
 * instance, never a replayable product job. Twin of Rust `kernel::FRAMEWORK_RESERVED_JOB_KIND`. */
export const FRAMEWORK_RESERVED_JOB_KIND = "framework.reserved.tool";

/** 🧵 How many `step-job` observations ONE host admission may take. Twin of Rust
 * `kernel::SPAWNED_JOB_STEP_CEILING`. */
export const SPAWNED_JOB_STEP_CEILING = 32;

/** ⛽️ The fuel one `step-job` is granted — a WIT `u64`, therefore a `bigint`. Twin of Rust
 * `kernel::SPAWNED_JOB_FUEL`. */
export const SPAWNED_JOB_FUEL = 50_000_000n;

/** ⏰️ The wall deadline one `step-job` is granted, in milliseconds. Twin of Rust
 * `kernel::SPAWNED_JOB_DEADLINE_MS`. */
export const SPAWNED_JOB_DEADLINE_MS = 100;

/** 👣️ One observation of the guest's `jobs::step-job` export, in the WIT `job-step` vocabulary. */
export type SpawnedJobStep = { readonly status: "running" } | { readonly status: "done"; readonly value: Uint8Array } | { readonly status: "failed"; readonly value: Uint8Array };

/** 🔂️ What the host owes the guest once a spawned job reached a terminal step. `outcome` keeps the WIT
 * `completion-result` arm names (`ok`/`fault`), the shape the wire carries. */
export type SpawnedJobCompletion = { readonly steps: number; readonly outcome: { readonly ok: Uint8Array } | { readonly fault: Uint8Array } };

/** ❌️ Why a spawned job produced no completion — TS twin of Rust `kernel::SpawnedJobDriveError`. */
export class SpawnedJobDriveError extends Error {
  readonly reason: "stalled" | "overrun";
  readonly steps: number;
  constructor(reason: "stalled" | "overrun", steps: number) {
    super(reason === "stalled" ? `spawned job did not reach a terminal step within ${steps} host steps` : `spawned job drive took ${steps} steps, past the ${SPAWNED_JOB_STEP_CEILING}-step ceiling`);
    this.name = "SpawnedJobDriveError";
    this.reason = reason;
    this.steps = steps;
  }
}

/** 🏁️ The ONE rule that turns a transcript of `step-job` observations into the `Event::JobCompleted`
 * the host owes the guest. Twin of Rust `kernel::spawned_job_completion`; both drive
 * `🧫️fixtures/🧵️spawned-job-drive/🔣️.json`.
 *
 * Observations after the first terminal step are ignored, so a driver that stops the instant it sees
 * `done` and one that read a batch hand over the same completion. */
export function spawnedJobCompletion(steps: readonly SpawnedJobStep[]): SpawnedJobCompletion {
  if (steps.length > SPAWNED_JOB_STEP_CEILING) throw new SpawnedJobDriveError("overrun", steps.length);
  for (let index = 0; index < steps.length; index += 1) {
    const step = steps[index]!;
    if (step.status === "running") continue;
    return { steps: index + 1, outcome: step.status === "done" ? { ok: step.value } : { fault: step.value } };
  }
  throw new SpawnedJobDriveError("stalled", steps.length);
}
//#endregion 🧵️SpawnedJobDrive
//#endregion InvocationResponse
// #endregion 🎠️Kernel
