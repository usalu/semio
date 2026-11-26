import { afterAll, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { OFFLOAD_POLICY_V1 } from "../../📐️policy/🟦️.ts";
import { offloadGateV1, readOffloadOverheadV1, worthARoundTripV1, writeOffloadOverheadV1, writeUnitCostV1 } from "../../🚦️admission/🟦️.ts";
import vectors from "./🔣️.json" with { type: "json" };

const roots: string[] = [], fresh = (): string => { const root = mkdtempSync(join(tmpdir(), "semio-gate-")); roots.push(root); return root; };
afterAll(() => { for (const root of roots) rmSync(root, { recursive: true, force: true }); });

test("the offload gate follows the language-neutral vectors, asking only the signals it needs, local first", async () => {
  expect(vectors.queueThresholdMilliseconds).toBe(OFFLOAD_POLICY_V1.limits.queueThresholdMilliseconds);
  expect(vectors.unmeasuredWaitMilliseconds).toBe(OFFLOAD_POLICY_V1.limits.unmeasuredWaitMilliseconds);
  expect(vectors.finalsWaitMilliseconds).toBe(OFFLOAD_POLICY_V1.finals.waitMilliseconds);
  for (const row of vectors.cases) {
    const asked: string[] = [];
    const answer = await offloadGateV1({
      mode: row.mode as "distribute" | "pin" | "never", door: row.door, fleetRequired: row.fleetRequired,
      expensive: () => { asked.push("expensive"); return row.expensive; },
      workerFits: async (waitMilliseconds) => { asked.push("fits"); expect(waitMilliseconds, row.name).toBe(vectors.finalsWaitMilliseconds); return row.fits; },
      localSlot: async (waitMilliseconds) => { asked.push(waitMilliseconds === 0 ? "slot-now" : "slot-wait"); if (waitMilliseconds > 0) expect(waitMilliseconds, row.name).toBe(vectors.unmeasuredWaitMilliseconds); return waitMilliseconds === 0 ? row.slotNow : row.slotWait; },
      beyondHost: () => { asked.push("beyond-host"); return row.beyondHost === true; },
      worth: () => { asked.push("worth"); return row.worth as "worth" | "cheap" | "unmeasured"; },
      slotQueueMilliseconds: () => { asked.push("slot"); return row.slotQueueMilliseconds; },
      host: async () => { asked.push("host"); return row.host as "admit" | "wait"; },
      broker: async () => { asked.push("broker"); return row.broker as "admit" | "wait" | "unknown"; },
    });
    expect(answer, row.name).toEqual({ offload: row.offload, queue: row.queue, reason: row.reason as never });
    expect(asked, row.name).toEqual(row.asked);
  }
});

test("the round trip test compares the recorded local compile time with the measured overhead of a round trip, else the authored floor", () => {
  for (const row of vectors.worthCases) {
    expect(row.floor, row.name).toBe(OFFLOAD_POLICY_V1.limits.roundTripMilliseconds);
    const root = fresh();
    if (row.cost !== null) writeUnitCostV1(root, "unit", row.cost);
    if (row.overhead !== null) writeOffloadOverheadV1(root, row.overhead);
    expect(worthARoundTripV1(root, "unit"), row.name).toBe(row.worth as never);
  }
});

test("the overhead estimate follows the fleet: it starts at the first measurement and moves halfway to each new one, and bad values are ignored", () => {
  const root = fresh();
  expect(readOffloadOverheadV1(root)).toBe(OFFLOAD_POLICY_V1.limits.roundTripMilliseconds);
  writeOffloadOverheadV1(root, 10000);expect(readOffloadOverheadV1(root)).toBe(10000);
  writeOffloadOverheadV1(root, 20000);expect(readOffloadOverheadV1(root)).toBe(15000);
  writeOffloadOverheadV1(root, Number.NaN);writeOffloadOverheadV1(root, -1);expect(readOffloadOverheadV1(root)).toBe(15000);
});

test("the offload gate agrees with an independent model over every combination of its signals", async () => {
  const rows: Record<string, unknown>[] = [];
  for (const mode of ["never", "pin", "distribute"]) for (const expensive of [false, true]) for (const fits of [false, true]) for (const beyondHost of [false, true]) for (const slotNow of [false, true]) for (const slotWait of [false, true]) for (const door of [false, true]) for (const worth of ["worth", "cheap", "unmeasured"]) for (const slotQueueMilliseconds of [0, 1499, 1500]) for (const host of ["admit", "wait"]) for (const broker of ["admit", "wait", "unknown"]) rows.push({ mode, expensive, fits, beyondHost, slotNow, slotWait, door, worth, slotQueueMilliseconds, host, broker });
  rows.push(...vectors.cases.filter(row=>row.fleetRequired));
  const result = spawnSync(testPythonV1() ?? "python3", ["-I", fileURLToPath(new URL("./🐍️.py", import.meta.url))], { input: JSON.stringify({ threshold: OFFLOAD_POLICY_V1.limits.queueThresholdMilliseconds, rows }), encoding: "utf8", maxBuffer: 1 << 26 });
  expect(result.status, result.stderr).toBe(0);
  const model = JSON.parse(result.stdout) as [boolean, boolean, string][];
  expect(model.length).toBe(rows.length);
  for (const [index, row] of rows.entries()) {
    const answer = await offloadGateV1({ mode: row.mode as never, door: row.door as boolean, fleetRequired: row.fleetRequired === true, expensive: () => row.expensive as boolean, workerFits: async () => row.fits as boolean, localSlot: async (wait) => wait === 0 ? row.slotNow as boolean : row.slotWait as boolean, worth: () => row.worth as never, beyondHost: () => row.beyondHost === true, slotQueueMilliseconds: () => row.slotQueueMilliseconds as number, host: async () => row.host as never, broker: async () => row.broker as never });
    expect<unknown>([answer.offload, answer.queue, answer.reason], JSON.stringify(row)).toEqual(model[index]!);
  }
});
