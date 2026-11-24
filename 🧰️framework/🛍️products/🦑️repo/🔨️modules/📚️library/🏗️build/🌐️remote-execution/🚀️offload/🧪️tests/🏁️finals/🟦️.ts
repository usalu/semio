import { afterAll, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { OFFLOAD_POLICY_V1, offloadExpenseV1 } from "../../📐️policy/🟦️.ts";
import { awaitFleetFitV1, readFamilyCostV1, readUnitExpenseV1, writeUnitCostV1, writeUnitPeakV1 } from "../../🚦️admission/🟦️.ts";
import vectors from "./🔣️.json" with { type: "json" };

const roots: string[] = [], fresh = (): string => { const root = mkdtempSync(join(tmpdir(), "semio-finals-")); roots.push(root); return root; };
afterAll(() => { for (const root of roots) rmSync(root, { recursive: true, force: true }); });
type State = { linked: boolean; cost: number | null; learned: number | null; familyCosts: number[]; familyPeaks: number[] };
const LINKED = { types: ["cdylib"], target: "wasm32-wasip2" }, PLAIN = { types: ["lib"], target: undefined };
function materialize(state: State): string {
  const root = fresh();
  state.familyCosts.forEach((value, index) => writeUnitCostV1(root, `family_${index}_component`, value));
  state.familyPeaks.forEach((value, index) => writeUnitPeakV1(root, `peak_${index}_component`, value));
  if (state.cost !== null) writeUnitCostV1(root, "subject", state.cost);
  if (state.learned !== null) writeUnitPeakV1(root, "subject", state.learned);
  return root;
}
const oracle = (states: State[]): { expensive: boolean; source: string; peak: number }[] => {
  const result = spawnSync(testPythonV1() ?? "python3", ["-I", fileURLToPath(new URL("./🐍️.py", import.meta.url))], { input: JSON.stringify({ thresholds: vectors.thresholds, states }), encoding: "utf8", maxBuffer: 1 << 26 });
  expect(result.status, result.stderr).toBe(0);
  return JSON.parse(result.stdout);
};

test("the declared thresholds are the ones of the vectors", () => {
  const finals = OFFLOAD_POLICY_V1.finals, thresholds = vectors.thresholds;
  expect([finals.minimumMilliseconds, finals.minimumMegabytes, finals.seedMilliseconds]).toEqual([thresholds.minimumMilliseconds, thresholds.minimumMegabytes, thresholds.seedMilliseconds]);
  expect([OFFLOAD_POLICY_V1.peak.unknownMegabytes, OFFLOAD_POLICY_V1.wasm.peakMegabytes, OFFLOAD_POLICY_V1.peak.marginPercent, OFFLOAD_POLICY_V1.peak.minimumMegabytes, OFFLOAD_POLICY_V1.peak.maximumMegabytes, OFFLOAD_POLICY_V1.peak.family.quantilePercent, OFFLOAD_POLICY_V1.peak.family.minimumCrates]).toEqual([thresholds.unknownMegabytes, thresholds.coldMegabytes, thresholds.marginPercent, thresholds.minimumMegabytes64, thresholds.maximumMegabytes, thresholds.quantilePercent, thresholds.minimumCrates]);
});

test("what placement and the gate read from the cost folder follows the language-neutral vectors and the independent model, for a plain and for a linked wasm unit", () => {
  const expected = oracle(vectors.cases.map((row) => row.state));
  for (const [index, row] of vectors.cases.entries()) {
    expect(expected[index], row.name).toEqual(row.expect);
    const root = materialize(row.state), unit = row.state.linked ? LINKED : PLAIN, read = readUnitExpenseV1(root, "subject", unit.types, unit.target);
    expect({ expensive: read.expense.expensive, source: read.expense.source, peak: read.peak }, row.name).toEqual(row.expect);
  }
});
test("the grid of every combination of its inputs agrees with the independent model", () => {
  const states: State[] = [];
  for (const linked of [false, true]) for (const cost of [null, 1000, 119999, 120000, 900000]) for (const learned of [null, 100, 3276, 3277, 6550, 20000]) for (const familyCosts of [[], [1, 2], [1000, 2000, 3000], [30000, 600000, 2200000]]) for (const familyPeaks of [[], [100, 200], [300, 400, 500], [3000, 3200, 6000]]) states.push({ linked, cost, learned, familyCosts, familyPeaks });
  const expected = oracle(states);
  expect(expected.length).toBe(states.length);
  for (const [index, state] of states.entries()) {
    const root = materialize(state), unit = state.linked ? LINKED : PLAIN, read = readUnitExpenseV1(root, "subject", unit.types, unit.target);
    expect({ expensive: read.expense.expensive, source: read.expense.source, peak: read.peak }, JSON.stringify(state)).toEqual(expected[index]!);
  }
}, 120000);

test("the family compile time reads only the crates of the family and needs the minimum number of them", () => {
  const root = fresh();
  writeUnitCostV1(root, "a_component", 1000); writeUnitCostV1(root, "b_component", 2000); writeUnitCostV1(root, "plain", 99999999);
  expect(readFamilyCostV1(root)).toBeUndefined();
  writeUnitCostV1(root, "c_component", 3000);
  expect(readFamilyCostV1(root)).toBe(2800);
  expect(offloadExpenseV1({ linked: true, placedMegabytes: 100 }).source).toBe("seed");
});

function fleetOf(rows: () => { slots: number; candidates: number }, seen: string[] = []): typeof fetch {
  return (async (url: string | URL | Request) => { seen.push(String(url)); return new Response(JSON.stringify(rows()), { status: 200, headers: { "content-type": "application/json" } }); }) as typeof fetch;
}
const environment = () => ({ SEMIO_WORKSPACE_ROOT: fresh(), SEMIO_REMOTE_EXECUTION_ENDPOINT: "grpc://192.0.2.1:8980" });

test("waiting for a worker that has room asks the scheduler at the unit's peak, polls until the wait is over and answers at once when none is published", async () => {
  const seen: string[] = [];
  let clock = 0, slots = 0;
  const base = { now: () => clock, sleep: async (milliseconds: number) => { clock += milliseconds; if (clock >= 6000) slots = 1; }, peakMegabytes: 7938, pollMilliseconds: 2000 };
  expect(await awaitFleetFitV1(environment(), { ...base, fleet: { fetch: fleetOf(() => ({ slots: 1, candidates: 1 }), seen) }, waitMilliseconds: 30000 }), "a worker has room at once").toBe(true);
  expect(seen.length).toBe(1);
  expect(seen[0]).toContain("/v1/fleet/idle?origin=");
  expect(seen[0]).toContain("&peak=7938");
  clock = 0; slots = 0; seen.length = 0;
  expect(await awaitFleetFitV1(environment(), { ...base, fleet: { fetch: fleetOf(() => ({ slots, candidates: 1 }), seen) }, waitMilliseconds: 30000 }), "room frees after 6 s").toBe(true);
  expect(seen.length).toBe(4);
  clock = 0; slots = -1; seen.length = 0;
  expect(await awaitFleetFitV1(environment(), { ...base, sleep: async (milliseconds: number) => { clock += milliseconds; }, fleet: { fetch: fleetOf(() => ({ slots: 0, candidates: 0 }), seen) }, waitMilliseconds: 6000 }), "never room: gives up after the wait").toBe(false);
  expect(seen.length).toBe(4);
  expect(await awaitFleetFitV1({ SEMIO_OFFLOAD: "never" }, { ...base, waitMilliseconds: 30000 }), "offloading off").toBe(false);
  const quiet = await awaitFleetFitV1({ SEMIO_WORKSPACE_ROOT: fresh() }, { ...base, waitMilliseconds: 30000 });
  expect(quiet, "no scheduler published: no wait").toBe(false);
});
