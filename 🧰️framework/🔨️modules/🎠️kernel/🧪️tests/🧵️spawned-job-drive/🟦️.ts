import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import {
  jobPlacementFromWireName,
  JOB_PLACEMENTS,
  spawnedJobCompletion,
  SpawnedJobDriveError,
  SPAWNED_JOB_DEADLINE_MS,
  SPAWNED_JOB_FUEL,
  SPAWNED_JOB_STEP_CEILING,
  type SpawnedJobCompletion,
  type SpawnedJobStep,
} from "../../🟦️.ts";
import { driveSpawnedJob, spawnedJobCompletedEvent, wireEffectToFriendly, wireJobStep, wireSpawnedJobs, type SpawnedJobPort, type WireVariant } from "../../../🎭️actor/🖼️wire-turn/🟦️.ts";

/** 🧵 TypeScript twin of `🧪️tests/🧵️spawned-job-drive/🦀️.rs`, driven from the SAME fixture
 * (`🧫️fixtures/🧵️spawned-job-drive/🔣️.json`): a `spawn-job` effect obliges its host to start the job,
 * step it to a terminal `job-step`, and answer the guest with `Event::JobCompleted`.
 *
 * 🐛️ The defect this pins: the browser hosts had NO `spawn-job` case at all. Every framework reserved
 * tool verb — `interactionSelect`, `interactionHover`, `clearSelection` — is delivered as this effect
 * and nothing else, so on the wgpu target every gesture published exactly the right wire message and
 * was applied never: 18 `unmapped effect "spawn-job" dropped` lines per run on 6118
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md` §7).
 *
 * 🔍️ Independent where it counts: the wire event's shape is validated by `ajv` against a JSON Schema
 * the fixture declares — a third-party validator that shares no line with the builder — and the
 * drive loop is driven through a SCRIPTED port whose transcripts are the fixture's, so the loop and
 * the rule are checked separately rather than by each other.
 */

interface StepFixture {
  readonly status: "running" | "done" | "failed";
  readonly value?: readonly number[];
}

interface OutcomeFixture {
  readonly ok?: readonly number[];
  readonly fault?: readonly number[];
}

interface CompletionFixture {
  readonly steps: number;
  readonly outcome: OutcomeFixture;
}

interface TranscriptFixture {
  readonly id: string;
  readonly runningPrefix?: number;
  readonly steps: readonly StepFixture[];
  readonly completion: CompletionFixture;
}

interface RefusalFixture {
  readonly id: string;
  readonly runningPrefix?: number;
  readonly steps: readonly StepFixture[];
  readonly error: { readonly kind: "stalled" | "overrun"; readonly steps: number };
}

interface DriveFixture {
  readonly schema: string;
  readonly budget: { readonly stepCeiling: number; readonly fuel: string; readonly deadlineMs: number };
  readonly placements: { readonly wire: readonly string[]; readonly refusals: readonly string[] };
  readonly transcripts: readonly TranscriptFixture[];
  readonly refusals: readonly RefusalFixture[];
  readonly completedEvent: {
    readonly kind: string;
    readonly cases: readonly { readonly id: string; readonly job: string; readonly completion: CompletionFixture; readonly payload: { readonly job: string; readonly outcome: { readonly tag: string; readonly val: readonly number[] } } }[];
  };
}

function loadFixture(): DriveFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🧵️spawned-job-drive/🔣️.json"), "utf8")) as DriveFixture;
}

function transcript(fixture: { readonly runningPrefix?: number; readonly steps: readonly StepFixture[] }): SpawnedJobStep[] {
  const steps: SpawnedJobStep[] = Array.from({ length: fixture.runningPrefix ?? 0 }, () => ({ status: "running" }) as const);
  for (const step of fixture.steps) steps.push(step.status === "running" ? { status: "running" } : { status: step.status, value: Uint8Array.from(step.value ?? []) });
  return steps;
}

function outcomeOf(fixture: OutcomeFixture): SpawnedJobCompletion["outcome"] {
  return fixture.ok !== undefined ? { ok: Uint8Array.from(fixture.ok) } : { fault: Uint8Array.from(fixture.fault ?? []) };
}

/** 🎬️ A guest that answers exactly the scripted transcript — the ONE seam a host pump has. */
function scriptedPort(script: readonly SpawnedJobStep[], log: { started: { job: bigint; kind: string; input: Uint8Array }[]; stepped: bigint[] }): SpawnedJobPort {
  let cursor = 0;
  return {
    startJob: async (job, kind, input) => {
      log.started.push({ job, kind, input });
    },
    stepJob: async (job) => {
      log.stepped.push(job);
      const step = script[Math.min(cursor, script.length - 1)] ?? { status: "running" as const };
      cursor += 1;
      // 🧬️ Answers in the RAW jco variant shape (`{tag, val}`), not the normalized one, so the
      // decoder is exercised rather than bypassed.
      return step.status === "running" ? { tag: "running", val: undefined } : { tag: step.status, val: Array.from(step.value) };
    },
  };
}

export async function testSpawnedJobDriveContract(): Promise<void> {
  const fixture = loadFixture();

  // 1️⃣ The host's static admission budget is the fixture's — the same three numbers the Rust twin reads.
  assert.equal(fixture.budget.stepCeiling, SPAWNED_JOB_STEP_CEILING, "step ceiling");
  assert.equal(BigInt(fixture.budget.fuel), SPAWNED_JOB_FUEL, "fuel");
  assert.equal(fixture.budget.deadlineMs, SPAWNED_JOB_DEADLINE_MS, "deadline");

  // 2️⃣ The placement vocabulary is closed, ordered and round-trips; nothing outside it resolves.
  assert.deepEqual([...JOB_PLACEMENTS], [...fixture.placements.wire], "the WIT enum order is part of the contract");
  for (const name of fixture.placements.wire) assert.equal(jobPlacementFromWireName(name), name, `${name} must round-trip`);
  for (const name of fixture.placements.refusals) assert.equal(jobPlacementFromWireName(name), undefined, `${JSON.stringify(name)} must not resolve`);
  for (const nonString of [undefined, null, 1, { tag: "isolated" }]) assert.equal(jobPlacementFromWireName(nonString), undefined, `${JSON.stringify(nonString)} must not resolve`);

  // 3️⃣ Every fixture transcript reaches exactly the declared completion.
  assert.ok(fixture.transcripts.length >= 5, `fixture must keep driving every terminal shape: ${fixture.transcripts.length}`);
  for (const row of fixture.transcripts) {
    const completion = spawnedJobCompletion(transcript(row));
    assert.equal(completion.steps, row.completion.steps, `${row.id}: step count`);
    assert.deepEqual(completion.outcome, outcomeOf(row.completion.outcome), `${row.id}: outcome`);
  }

  // 4️⃣ Every fixture refusal refuses, typed, with the declared step count.
  assert.ok(fixture.refusals.length >= 3, "fixture must keep driving both refusal kinds");
  for (const row of fixture.refusals) {
    let thrown: unknown;
    try {
      spawnedJobCompletion(transcript(row));
    } catch (error) {
      thrown = error;
    }
    assert.ok(thrown instanceof SpawnedJobDriveError, `${row.id}: must refuse with the typed error`);
    assert.equal((thrown as SpawnedJobDriveError).reason, row.error.kind, `${row.id}: refusal kind`);
    assert.equal((thrown as SpawnedJobDriveError).steps, row.error.steps, `${row.id}: refusal step count`);
  }

  // 5️⃣ The exact wgpu defect as a law: a host that admits the effect and never steps it produces no
  //    completion, loudly — where before it produced a console warning and silence.
  assert.throws(() => spawnedJobCompletion([]), SpawnedJobDriveError, "an unpumped job must not complete");

  // 6️⃣ The wire decoder: the recorded `spawn-job` shape becomes a friendly effect with its u64 intact.
  const recorded: WireVariant = { tag: "spawn-job", val: { job: 916n, kind: "framework.reserved.tool", input: Uint8Array.from([70, 82, 82, 69, 83, 86, 48, 49]), placement: "isolated" } };
  const friendly = wireEffectToFriendly(recorded, (bytes) => [...bytes]);
  assert.deepEqual(friendly, { spawnJob: { job: 916n, kind: "framework.reserved.tool", input: Uint8Array.from([70, 82, 82, 69, 83, 86, 48, 49]), placement: "isolated" } }, "recorded spawn-job");
  assert.equal(wireSpawnedJobs([recorded, { tag: "notify", val: { message: "x" } }]).length, 1, "only jobs are jobs");
  for (const broken of [{ job: 916n, kind: "k", input: new Uint8Array(), placement: "pooled" }, { job: -1n, kind: "k", input: new Uint8Array(), placement: "inline" }, { job: 916n, kind: "", input: new Uint8Array(), placement: "inline" }]) {
    assert.throws(() => wireEffectToFriendly({ tag: "spawn-job", val: broken }, (bytes) => [...bytes]), `${JSON.stringify(String(broken.placement))} must refuse`);
  }
  assert.deepEqual(wireEffectToFriendly({ tag: "cancel-job", val: { job: 916n } }, (bytes) => [...bytes]), { cancelJob: { job: 916n } });

  // 7️⃣ The `job-step` decoder reads BOTH real shapes of the same door: jco's raw `{tag, val}` and the
  //    shard worker's normalized `{status, value}`.
  assert.deepEqual(wireJobStep({ tag: "running", val: undefined }), { status: "running" });
  assert.deepEqual(wireJobStep({ status: "running" }), { status: "running" });
  assert.deepEqual(wireJobStep({ tag: "done", val: [1, 2] }), { status: "done", value: Uint8Array.from([1, 2]) });
  assert.deepEqual(wireJobStep({ status: "done", value: Uint8Array.from([1, 2]) }), { status: "done", value: Uint8Array.from([1, 2]) });
  assert.deepEqual(wireJobStep({ status: "failed", value: Uint8Array.from([3]) }), { status: "failed", value: Uint8Array.from([3]) });
  assert.throws(() => wireJobStep({ tag: "sleeping" }), "an unknown job-step status must be loud");

  // 8️⃣ The shared pump, over a SCRIPTED guest: it starts once, steps until terminal and no further.
  for (const row of fixture.transcripts) {
    const script = transcript(row);
    const log = { started: [] as { job: bigint; kind: string; input: Uint8Array }[], stepped: [] as bigint[] };
    const completion = await driveSpawnedJob({ job: 916n, kind: "framework.reserved.tool", input: Uint8Array.from([1]), port: scriptedPort(script, log) });
    assert.equal(log.started.length, 1, `${row.id}: started exactly once`);
    assert.equal(log.started[0]!.job, 916n, `${row.id}: started the right job`);
    assert.equal(log.stepped.length, row.completion.steps, `${row.id}: stepped exactly to the terminal step`);
    assert.deepEqual(completion.outcome, outcomeOf(row.completion.outcome), `${row.id}: pumped outcome`);
  }

  // 9️⃣ A guest that never terminates stalls the pump loudly at the ceiling, having stepped that many times.
  {
    const log = { started: [] as { job: bigint; kind: string; input: Uint8Array }[], stepped: [] as bigint[] };
    await assert.rejects(
      driveSpawnedJob({ job: 7n, kind: "k", input: new Uint8Array(), port: scriptedPort([{ status: "running" }], log) }),
      (error: unknown) => error instanceof SpawnedJobDriveError && error.reason === "stalled" && error.steps === SPAWNED_JOB_STEP_CEILING,
    );
    assert.equal(log.stepped.length, SPAWNED_JOB_STEP_CEILING, "the ceiling is the number of round trips, not a guess");
  }

  // 🔟 The event the guest gets back, validated by ajv against the fixture's own schema.
  const ajv = new Ajv({ strict: true });
  const validate = ajv.compile({
    type: "object",
    required: ["kind", "payload"],
    additionalProperties: false,
    properties: {
      kind: { const: "job-completed" },
      payload: {
        type: "object",
        required: ["job", "outcome"],
        additionalProperties: false,
        properties: {
          job: { type: "string", pattern: "^[0-9]+$" },
          outcome: {
            type: "object",
            required: ["tag", "val"],
            additionalProperties: false,
            properties: { tag: { enum: ["ok", "fault"] }, val: { type: "array", items: { type: "integer", minimum: 0, maximum: 255 } } },
          },
        },
      },
    },
  });
  assert.equal(fixture.completedEvent.kind, "job-completed");
  assert.ok(fixture.completedEvent.cases.length >= 2, "both outcome arms must be driven");
  for (const row of fixture.completedEvent.cases) {
    const event = spawnedJobCompletedEvent(BigInt(row.job), { steps: row.completion.steps, outcome: outcomeOf(row.completion.outcome) });
    assert.equal(event.kind, "job-completed", `${row.id}: envelope kind`);
    assert.equal(event.payload.job, BigInt(row.job), `${row.id}: the guest correlates on the job id alone`);
    assert.equal(event.payload.outcome.tag, row.payload.outcome.tag, `${row.id}: outcome arm`);
    assert.deepEqual([...event.payload.outcome.val], [...row.payload.outcome.val], `${row.id}: outcome bytes`);
    // 🔍️ ajv is the independent reader: it never sees our types, only the JSON shape, with `job`
    // projected to its decimal text because a `bigint` is not JSON.
    assert.ok(validate({ kind: event.kind, payload: { job: String(event.payload.job), outcome: { tag: event.payload.outcome.tag, val: [...event.payload.outcome.val] } } }), `${row.id}: ajv ${JSON.stringify(validate.errors)}`);
  }

  console.log(`spawned-job-drive transcripts=${fixture.transcripts.length} refusals=${fixture.refusals.length} placements=${JOB_PLACEMENTS.length} ceiling=${SPAWNED_JOB_STEP_CEILING} events=${fixture.completedEvent.cases.length} oracle=ajv`);
}
