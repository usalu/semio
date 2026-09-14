/** 🤫️ The shard worker's silent-turn hold, as laws.
 *
 * In the browser the host round trip IS the guest's pump: a reactor that answers `more-work` while
 * publishing nothing costs a worker POST, a poll, a structured clone and a main-thread pickup just to
 * be asked again. Measured on the procedural 3d React door 2026-09-14, **89 of 111 crossings per
 * `flowEvalTick` hop posted no events and returned no patch** and carried 80 % of the worker's busy
 * time (`📓️reactor-reconcile-spin-2026-09-14.md` §2).
 *
 * Three things are asserted here, and the third is what keeps the other two honest:
 *
 * 1. {@link shardTurnCarriesNothingV1} refuses on EVERY field of WIT `turn-result` a host reads — a
 *    carrier it forgot would be silently dropped by the hold;
 * 2. {@link driveShardTurnSilentHoldV1} stops on carried output, on a non-`more-work` status, on the
 *    hold, on the poll cap and on a closed actor, and never polls a turn that already carried
 *    something;
 * 3. the GENERATED worker (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`) contains that same
 *    loop with those same constants — the twin is verified against the emitted source, not assumed.
 */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { driveShardTurnSilentHoldV1, shardTurnCarriesNothingV1, type ShardTurnCarriers } from "../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";

/** 🧬️ The materialization module is READ, never imported: it reaches the repo library's lease store
 * (`node:sqlite`) and cannot be bundled into a renderer test environment — the same reason its own
 * constants are declared rather than imported. Reading the literal straight out of the file is what
 * makes "held equal by the suite" true rather than aspirational. */
const MATERIALIZATION_SOURCE = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts"), "utf8");
const declared = (name: string): number => {
  const match = new RegExp(`export const ${name} = (\\d+);`).exec(MATERIALIZATION_SOURCE);
  if (!match) throw new Error(`materialization declares no ${name}`);
  return Number(match[1]);
};
const SHARD_TURN_SILENT_HOLD_MS = declared("SHARD_TURN_SILENT_HOLD_MS");
const SHARD_TURN_SILENT_HOLD_POLLS = declared("SHARD_TURN_SILENT_HOLD_POLLS");

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

describe("🤫️ a reactor turn that carried nothing", () => {
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

/** ⏱️ A virtual clock, so the hold is asserted in milliseconds without waiting for any. */
const drive = (results: readonly ShardTurnCarriers[], options: { readonly holdMs?: number; readonly maxPolls?: number; readonly closeAfter?: number; readonly costMs?: number } = {}) => {
  let nowMs = 0;
  let index = 0;
  const polled: number[] = [];
  return {
    run: async () =>
      driveShardTurnSilentHoldV1({
        first: results[0],
        poll: async () => {
          polled.push(nowMs);
          nowMs += options.costMs ?? 1;
          index += 1;
          return results[Math.min(index, results.length - 1)];
        },
        now: () => nowMs,
        live: () => options.closeAfter === undefined || polled.length < options.closeAfter,
        holdMs: options.holdMs ?? SHARD_TURN_SILENT_HOLD_MS,
        maxPolls: options.maxPolls ?? SHARD_TURN_SILENT_HOLD_POLLS,
      }),
    polled,
  };
};

describe("🤫️ the silent-turn hold", () => {
  it("absorbs silent more-work turns and crosses back with the one that carried something", async () => {
    const carrying = silent({ uiPatches: [{ surface: { instance: 1, surface: "window" } }] });
    const driven = drive([silent(), silent(), silent(), carrying]);
    const settled = await driven.run();
    expect(settled.stopped).toBe("carried");
    expect(settled.polls).toBe(3);
    expect(settled.result).toBe(carrying);
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

  it("spends at most its hold, and the hold is the reactor's own 8 ms executor slice", async () => {
    expect(SHARD_TURN_SILENT_HOLD_MS).toBe(8);
    const driven = drive([silent()], { costMs: 3 });
    const settled = await driven.run();
    expect(settled.stopped).toBe("hold");
    expect(settled.polls).toBe(3);
    expect(driven.polled).toEqual([0, 3, 6]);
  });

  it("stops at the poll cap even when every turn is free", async () => {
    const driven = drive([silent()], { costMs: 0, maxPolls: 5 });
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "polls", polls: 5 });
  });

  it("stops the moment the actor stops being live", async () => {
    const driven = drive([silent()], { costMs: 0, closeAfter: 2 });
    const settled = await driven.run();
    expect(settled).toMatchObject({ stopped: "closed", polls: 2 });
  });
});

describe("🤫️ the generated shard worker", () => {
  const source = MATERIALIZATION_SOURCE;

  it("carries the hold's constants rather than literals of its own", () => {
    expect(source).toContain("const SILENT_HOLD_MS = ${SHARD_TURN_SILENT_HOLD_MS};");
    expect(source).toContain("const SILENT_HOLD_POLLS = ${SHARD_TURN_SILENT_HOLD_POLLS};");
    expect(SHARD_TURN_SILENT_HOLD_POLLS).toBeGreaterThan(0);
  });

  it("drives the same loop, with the same stop conditions, on its turn path", () => {
    expect(source).toContain("function shardTurnCarriesNothing(result)");
    expect(source).toContain("function shardTurnStatusTag(result)");
    for (const stop of ['shardTurnStatusTag(result) === "more-work"', "shardTurnCarriesNothing(result)", "holdPolls < SILENT_HOLD_POLLS", "hopEpochNow() < holdDeadline", "actors.get(actorId) === actor", "actor.activationGeneration === msg.activationGeneration"]) {
      expect(source).toContain(stop);
    }
  });

  it("names every carrier the host-side predicate names", () => {
    for (const carrier of ["uiPatches", "effects", "presence", "nextWake", "lifecycleReceipt", "uiPatchReceipt", "commandIngress", "coldPairIngress"]) {
      expect(source.slice(source.indexOf("function shardTurnCarriesNothing(result)"), source.indexOf("function reply(requestId, value, timings)"))).toContain(carrier);
    }
  });
});
