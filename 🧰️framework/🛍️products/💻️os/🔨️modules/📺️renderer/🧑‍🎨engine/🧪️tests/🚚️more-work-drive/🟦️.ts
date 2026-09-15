/** 🚚️ The shard worker's WORKER-OWNED `MoreWork` drive, as laws.
 *
 * In the browser the host round trip IS the guest's pump: a reactor that answers `more-work` while
 * publishing nothing costs a worker POST, a poll, a structured clone and a main-thread pickup just to
 * be asked again. Measured on the procedural 3d React door 2026-09-15, **32.9 of 60.7 worker crossings
 * per `flowEvalTick` hop posted no events and returned no patch**
 * (`📓️ui-turn-patch-batching-2026-09-15.md` §7 item 2). The worker now runs those turns itself.
 *
 * Five things are asserted here, and the last two are what keep the others honest:
 *
 * 1. {@link shardTurnCarriesNothingV1} refuses on EVERY field of WIT `turn-result` a host reads — a
 *    carrier it forgot would be silently dropped by the drive;
 * 2. {@link driveShardTurnMoreWorkV1} stops on carried output, on a non-`more-work` status, on a
 *    pending host input, on the derived step ceiling, on the derived wall budget and on a closed
 *    actor, and never polls a turn that already carried something;
 * 3. the drive's bounds are DERIVED from the wall the host granted and what one guest turn MEASURES
 *    ({@link shardTurnDriveStepsV1}), and the derivation agrees with the reactor's own contract
 *    fixture — including the coherence clause that made the old 8 ms wall hold admit no turn at all;
 * 4. the GENERATED worker (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`) contains that same
 *    loop with those same constants, its macrotask yield and its host-input counter — the twin is
 *    verified against the emitted source, not assumed;
 * 5. the generated worker no longer posts a heartbeat per turn crossing: both beats ride the reply.
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { driveShardTurnMoreWorkV1, shardTurnCarriesNothingV1, shardTurnDriveBudgetMsV1, shardTurnDriveStepsV1, type ShardTurnCarriers } from "../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";

/** 🧬️ The materialization module is READ, never imported: it reaches the repo library's lease store
 * (`node:sqlite`) and cannot be bundled into a renderer test environment — the same reason its own
 * constants are declared rather than imported. Reading the literal straight out of the file is what
 * makes "held equal by the suite" true rather than aspirational. */
const HERE = dirname(fileURLToPath(import.meta.url));
const MATERIALIZATION_SOURCE = readFileSync(resolve(HERE, "../../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts"), "utf8");
const declared = (name: string): number => {
  const match = new RegExp(`export const ${name} = (\\d+);`).exec(MATERIALIZATION_SOURCE);
  if (!match) throw new Error(`materialization declares no ${name}`);
  return Number(match[1]);
};
const SHARD_TURN_GUEST_COST_MS = declared("SHARD_TURN_GUEST_COST_MS");
const SHARD_TURN_DRIVE_STEP_CEILING = declared("SHARD_TURN_DRIVE_STEP_CEILING");

/** 🧫️ The language-agnostic drive contract the Rust laws read — the one owner of every number below. */
const CONTRACT = JSON.parse(readFileSync(resolve(HERE, "../../../../🔌️plugin/⚛️reactor/🧫️fixtures/🚚️more-work-drive.json"), "utf8")) as {
  readonly reactorExecutorHoldMs: number;
  readonly measuredGuestTurnCostMs: number;
  readonly hostGrantWallMs: number;
  readonly driveStepCeiling: number;
  readonly driveBudgetMs: number;
  readonly silentTurnRuns: readonly number[];
  readonly crossingsPerSilentRun: number;
  readonly hostContinuationCeiling: number;
  readonly ingressInterruptTurns: number;
};

const silent = (overrides: Partial<ShardTurnCarriers> = {}): ShardTurnCarriers => ({
  uiPatches: [],
  effects: [],
  presence: [],
  nextWake: null,
  lifecycleReceipt: undefined,
  uiPatchReceipt: undefined,
  commandIngress: { tag: "idle" },
  coldPairIngress: { tag: "idle" },
  status: { tag: "more-work" },
  ...overrides,
});

describe("🚚️ a reactor turn that carried nothing", () => {
  it("is recognised only when every declared carrier is empty", () => {
    expect(shardTurnCarriesNothingV1(silent())).toBe(true);
    expect(shardTurnCarriesNothingV1(silent({ status: { tag: "idle" } }))).toBe(true);
  });

  const carriers: ReadonlyArray<readonly [string, Partial<ShardTurnCarriers>]> = [
    ["a ui patch", { uiPatches: [{ surface: { instance: 1, surface: "window" } }] }],
    ["an effect", { effects: [{ tag: "notify" }] }],
    ["a presence update", { presence: [{ surface: "1:window" }] }],
    ["a wake", { nextWake: 16 }],
    ["a lifecycle receipt", { lifecycleReceipt: new Uint8Array([1]) }],
    ["a ui patch receipt", { uiPatchReceipt: new Uint8Array([1]) }],
    ["a non-idle command ingress", { commandIngress: { tag: "applied", val: 7 } }],
    ["a command ingress fault", { commandIngress: { tag: "fault", val: {} } }],
    ["a non-idle cold pair ingress", { coldPairIngress: { tag: "page-accepted", val: {} } }],
  ];
  for (const [name, overrides] of carriers) {
    it(`refuses ${name}`, () => {
      expect(shardTurnCarriesNothingV1(silent(overrides))).toBe(false);
    });
  }

  it("refuses a malformed or absent result rather than dropping it", () => {
    expect(shardTurnCarriesNothingV1(undefined)).toBe(false);
    expect(shardTurnCarriesNothingV1(null)).toBe(false);
    expect(shardTurnCarriesNothingV1({})).toBe(false);
    expect(shardTurnCarriesNothingV1(silent({ uiPatches: undefined }))).toBe(false);
    expect(shardTurnCarriesNothingV1(silent({ commandIngress: undefined }))).toBe(false);
  });
});

/** ⏱️ A virtual clock, so the drive is asserted in milliseconds and turns without waiting for either. */
const drive = (results: readonly ShardTurnCarriers[], options: { readonly steps?: number; readonly budgetMs?: number; readonly closeAfter?: number; readonly inputAfter?: number; readonly costMs?: number } = {}) => {
  let nowMs = 0;
  let index = 0;
  const polled: number[] = [];
  return {
    run: async () =>
      driveShardTurnMoreWorkV1({
        first: results[0],
        poll: async () => {
          polled.push(nowMs);
          nowMs += options.costMs ?? CONTRACT.measuredGuestTurnCostMs;
          index += 1;
          return results[Math.min(index, results.length - 1)];
        },
        now: () => nowMs,
        live: () => options.closeAfter === undefined || polled.length < options.closeAfter,
        inputPending: () => options.inputAfter !== undefined && polled.length >= options.inputAfter,
        steps: options.steps ?? SHARD_TURN_DRIVE_STEP_CEILING,
        budgetMs: options.budgetMs ?? CONTRACT.driveBudgetMs,
      }),
    polled,
  };
};

describe("🚚️ the worker-owned MoreWork drive", () => {
  it("absorbs k silent more-work turns into ONE crossing, for every run the contract declares", async () => {
    for (const run of CONTRACT.silentTurnRuns) {
      const carrying = silent({ uiPatches: [{ surface: { instance: 1, surface: "window" } }] });
      const driven = drive([...Array.from({ length: run }, () => silent()), carrying]);
      const settled = await driven.run();
      expect(settled.stopped).toBe("carried");
      expect(settled.polls).toBe(run);
      expect(settled.result).toBe(carrying);
    }
    expect(CONTRACT.crossingsPerSilentRun).toBe(1);
  });

  it("never polls a turn that already carried something", async () => {
    const driven = drive([silent({ effects: [{ tag: "notify" }] }), silent()]);
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "carried", polls: 0 });
    expect(driven.polled).toEqual([]);
  });

  it("never polls a turn that already went idle", async () => {
    const driven = drive([silent({ status: { tag: "idle" } }), silent()]);
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "idle", polls: 0 });
  });

  it("reads camelCase and kebab-case spellings of the same status", async () => {
    const driven = drive([silent({ status: { tag: "moreWork" } }), silent({ status: { tag: "idle" } })]);
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "idle", polls: 1 });
  });

  it("crosses back within one turn of a host-owned input becoming pending", async () => {
    for (let inputAfter = 0; inputAfter < CONTRACT.driveStepCeiling; inputAfter += 1) {
      const driven = drive([silent()], { inputAfter });
      const settled = await driven.run();
      expect(settled.stopped).toBe("input");
      expect(settled.polls).toBeLessThanOrEqual(inputAfter + CONTRACT.ingressInterruptTurns);
    }
  });

  it("puts a pending host input ahead of its own step ceiling and wall budget", async () => {
    const driven = drive([silent()], { inputAfter: 1, steps: 512, budgetMs: 10_000 });
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "input", polls: 1 });
  });

  it("stops at the backstop step ceiling only when every turn is free", async () => {
    const driven = drive([silent()], { costMs: 0, steps: SHARD_TURN_DRIVE_STEP_CEILING });
    const settled = await driven.run();
    // 🚚️ The backstop counts TURNS per crossing and the first one is `first`, so k admits k − 1 polls.
    expect(settled).toMatchObject({ stopped: "steps", polls: SHARD_TURN_DRIVE_STEP_CEILING - 1 });
  });

  it("spends its whole granted wall before it crosses back, never the wall rounded down to whole turns", async () => {
    const driven = drive([silent()], { costMs: CONTRACT.measuredGuestTurnCostMs, steps: SHARD_TURN_DRIVE_STEP_CEILING });
    const settled = await driven.run();
    expect(settled.stopped).toBe("budget");
    // 🐛️ The rounded wall (`steps × cost` = 91 ms) would have stopped one whole turn earlier with 9 ms
    // of the host's own grant unspent — and a crossing that carries nothing is what this lane removes.
    expect(settled.polls).toBe(Math.ceil(CONTRACT.hostGrantWallMs / CONTRACT.measuredGuestTurnCostMs));
    expect(settled.polls).toBeGreaterThan(CONTRACT.driveStepCeiling);
  });

  it("stops the moment the actor stops being live", async () => {
    const driven = drive([silent()], { costMs: 0, closeAfter: 2 });
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "closed", polls: 2 });
  });

  /** ⛔️ `PluginRuntime`'s command-ingress drain and `PLUGIN_UI_CONTINUATION_LIMIT` both bound host
   * CROSSINGS, so the drive may only make that bound cover more guest work, never less. Twin of the
   * reactor's `the_hosts_continuation_ceiling_covers_more_guest_turns_under_the_drive_never_fewer`. */
  it("costs at most ceil(k / steps) host continuations for a guest that needs k turns, never more than k", async () => {
    for (let needed = 1; needed <= 32; needed += 1) {
      const carrying = silent({ uiPatches: [{ surface: { instance: 1, surface: "window" } }] });
      let turns = 0;
      const nextTurn = (): ShardTurnCarriers => { turns += 1; return turns >= needed ? carrying : silent(); };
      let crossings = 0;
      let stopped = "";
      do {
        crossings += 1;
        expect(crossings).toBeLessThanOrEqual(needed);
        const settled = await driveShardTurnMoreWorkV1({
          first: nextTurn(),
          poll: async () => nextTurn(),
          now: () => 0,
          live: () => true,
          inputPending: () => false,
          steps: CONTRACT.driveStepCeiling,
          budgetMs: CONTRACT.driveBudgetMs,
        });
        stopped = settled.stopped;
      } while (stopped !== "carried");
      expect(crossings).toBe(Math.ceil(needed / CONTRACT.driveStepCeiling));
      expect(turns).toBe(needed);
    }
    expect(CONTRACT.hostContinuationCeiling * CONTRACT.driveStepCeiling).toBeGreaterThan(CONTRACT.hostContinuationCeiling);
  });
});

describe("🚚️ the drive's bounds are derived, not chosen", () => {
  it("agrees with the reactor's own contract fixture", () => {
    expect(SHARD_TURN_GUEST_COST_MS).toBe(CONTRACT.measuredGuestTurnCostMs);
    expect(shardTurnDriveStepsV1(CONTRACT.hostGrantWallMs, CONTRACT.measuredGuestTurnCostMs)).toBe(CONTRACT.driveStepCeiling);
    expect(shardTurnDriveBudgetMsV1(CONTRACT.hostGrantWallMs, CONTRACT.measuredGuestTurnCostMs)).toBe(CONTRACT.driveBudgetMs);
    expect(CONTRACT.driveBudgetMs).toBe(CONTRACT.hostGrantWallMs);
  });

  it("states the coherence the reactor's own hold could not give it", () => {
    expect(CONTRACT.measuredGuestTurnCostMs).toBeGreaterThan(CONTRACT.reactorExecutorHoldMs);
    expect(Math.floor(CONTRACT.reactorExecutorHoldMs / CONTRACT.measuredGuestTurnCostMs)).toBe(0);
    expect(shardTurnDriveStepsV1(CONTRACT.reactorExecutorHoldMs, CONTRACT.measuredGuestTurnCostMs)).toBe(1);
  });

  it("never spends more than the wall the host granted, and always admits one turn", () => {
    for (const grant of [0, 1, 4, 16, 50, 100, 200, 100_000]) {
      const steps = shardTurnDriveStepsV1(grant, CONTRACT.measuredGuestTurnCostMs);
      expect(steps).toBeGreaterThanOrEqual(1);
      expect(steps).toBeLessThanOrEqual(Math.max(1, Math.floor(grant / CONTRACT.measuredGuestTurnCostMs)));
      expect(shardTurnDriveBudgetMsV1(grant, CONTRACT.measuredGuestTurnCostMs)).toBeGreaterThanOrEqual(CONTRACT.measuredGuestTurnCostMs);
    }
    expect(shardTurnDriveStepsV1(Number.NaN, CONTRACT.measuredGuestTurnCostMs)).toBe(1);
    expect(shardTurnDriveStepsV1(CONTRACT.hostGrantWallMs, 0)).toBe(1);
  });
});

describe("🚚️ the generated shard worker", () => {
  const source = MATERIALIZATION_SOURCE;

  it("carries the drive's constants rather than literals of its own", () => {
    expect(source).toContain("const GUEST_TURN_COST_MS = ${SHARD_TURN_GUEST_COST_MS};");
    expect(source).toContain("const DRIVE_STEP_CEILING = ${SHARD_TURN_DRIVE_STEP_CEILING};");
    expect(SHARD_TURN_DRIVE_STEP_CEILING).toBeGreaterThan(CONTRACT.driveStepCeiling);
  });

  it("drives the same loop, with the same stop conditions, on its turn path", () => {
    expect(source).toContain("function shardTurnCarriesNothing(result)");
    expect(source).toContain("function shardTurnStatusTag(result)");
    expect(source).toContain("function shardTurnDriveSteps(grantWallMs, guestTurnCostMs)");
    expect(source).toContain("function shardTurnDriveBudgetMs(grantWallMs, guestTurnCostMs)");
    for (const stop of ['driveStopped = "input"', 'driveStopped = "closed"', 'driveStopped = "steps"', 'driveStopped = "budget"', '"carried"', '"idle"']) {
      expect(source).toContain(stop);
    }
    expect(source).toContain("drivePolls + 1 >= driveSteps");
    expect(source).toContain("const driveSteps = DRIVE_STEP_CEILING;");
    expect(source.indexOf('driveStopped = "budget"')).toBeLessThan(source.indexOf('driveStopped = "steps"'));
    expect(source).toContain("hopEpochNow() >= driveDeadline");
    expect(source).toContain("hostInputSeq !== inputMark");
    expect(source).toContain("actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration");
  });

  it("counts every host message and yields a macrotask so the drive can actually see one", () => {
    expect(source).toContain("hostInputSeq += 1;");
    expect(source).toContain("const inputMark = hostInputSeq;");
    expect(source).toContain("await driveYield();");
    expect(source).toContain("new MessageChannel()");
    expect(source).not.toContain("setTimeout(() => resolve");
  });

  it("names every carrier the host-side predicate names", () => {
    for (const carrier of ["uiPatches", "effects", "presence", "nextWake", "lifecycleReceipt", "uiPatchReceipt", "commandIngress", "coldPairIngress"]) {
      expect(source.slice(source.indexOf("function shardTurnCarriesNothing(result)"), source.indexOf("function shardTurnDriveSteps(grantWallMs"))).toContain(carrier);
    }
  });

  it("folds the two per-crossing heartbeats into the crossing itself", () => {
    expect(source).toContain('reply(requestId, result, timings, beat("turn-step"));');
    expect(source).toContain('reply(requestId, step, undefined, beat("turn-step"));');
    expect(source).toContain('self.postMessage({ kind: "heartbeat", ...beat(phase) });');
    expect(source).not.toContain('heartbeat("turn-step")');
    const workerSource = source.slice(source.indexOf("export function shardWorkerSource()"));
    const posted = [...workerSource.matchAll(/postBeat\(/g)].length;
    expect(posted).toBe(5);
    for (const unridden of ['postBeat("progress")', 'postBeat("module-fetch")', 'postBeat("module-ready")', 'postBeat("actor-ready")']) {
      expect(workerSource).toContain(unridden);
    }
  });
});
