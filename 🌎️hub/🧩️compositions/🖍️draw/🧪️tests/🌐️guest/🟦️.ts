import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import Ajv from "ajv";
import { Database } from "bun:sqlite";

type Scenario = { appId: string; actor: string; expectedStatus: string };
type Caller = { clock: string; lifetimeUs: number; fuel: number; retirementFuel: number; executionFuel: number; executionFuelPerCall: number; maximumTurns: number };
type Scenarios = { version: number; caller: Caller; scenarios: Scenario[] };

/** 🧭 Validates the shared Draw guest opening contract with an independent schema oracle. */
export async function validateDrawGuestScenarios(repoRoot: string, descriptorPath: string): Promise<void> {
  const root = join(repoRoot, "🌎️hub/🧩️compositions/🖍️draw");
  const guest = join(root, "🧪️tests/🌐️guest");
  await validateDrawReleasePolicy(join(root, "🧪️tests/🔬️surface"));
  const read = async (path: string) => JSON.parse(await readFile(path, "utf8"));
  const [contract, schema, descriptor] = await Promise.all([read(join(guest, "scenarios.json")), read(join(guest, "schema.json")), read(descriptorPath)]);
  const validate = new Ajv().compile<Scenarios>(schema);
  assert.ok(validate(contract), JSON.stringify(validate.errors));
  const scenarios = contract as Scenarios;
  assert.ok(scenarios.caller.retirementFuel < scenarios.caller.fuel);
  assert.ok(scenarios.caller.executionFuelPerCall <= scenarios.caller.executionFuel);
  assert.deepEqual(scenarios.scenarios.map(scenario => scenario.actor).sort(), ["editor", "viewer"]);
  for (const scenario of scenarios.scenarios) {
    assert.ok(descriptor.manifest.apps.some((app: { id: string; role: string }) => app.id === scenario.appId && app.role === scenario.actor));
  }
  const duplicate = structuredClone(scenarios);
  duplicate.scenarios[1] = structuredClone(duplicate.scenarios[0]!);
  assert.equal(validate(duplicate), false, "duplicate editor must fail");
  const mismatch = structuredClone(scenarios);
  mismatch.scenarios[1]!.appId = mismatch.scenarios[0]!.appId;
  assert.equal(validate(mismatch), false, "viewer with editor coordinate must fail");
  for (const field of ["lifetimeUs", "fuel", "retirementFuel", "executionFuel", "executionFuelPerCall", "maximumTurns"] as const) {
    for (const value of [0, Number.MAX_SAFE_INTEGER]) {
      const unbounded = structuredClone(scenarios);
      unbounded.caller[field] = value;
      assert.equal(validate(unbounded), false, `${field} must retain its finite authored bound`);
    }
  }
  const modeledClock = structuredClone(scenarios);
  modeledClock.caller.clock = "fixture-fixed";
  assert.equal(validate(modeledClock), false, "real guest must use its original native clock");
}

type ReleaseVector = { phase: "close" | "maintenance" | "preparation"; callerBytes: number; requestedBytes: number; admitted: boolean };
type ReleaseFixture = { version: number; maximumExportBytes: number; vectors: ReleaseVector[] };

/** ⚖️ Evaluates the portable Draw close vectors using SQLite's independent scalar predicates. */
export async function validateDrawReleasePolicy(root: string): Promise<void> {
  const read = async (filename: string) => JSON.parse(await readFile(join(root, filename), "utf8"));
  const [fixture, schema] = await Promise.all([read("release-policy.json"), read("release-policy.schema.json")]);
  const validate = new Ajv().compile<ReleaseFixture>(schema);
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const db = new Database(":memory:");
  try {
    const evaluate = db.query<{ admitted: number }, [number, number, number]>("SELECT ?1 <= min(?2, ?3) AS admitted");
    for (const vector of fixture.vectors) {
      const phaseCeiling = vector.phase === "maintenance" ? 1024 ** 2 : fixture.maximumExportBytes;
      assert.equal(evaluate.get(vector.requestedBytes, vector.callerBytes, phaseCeiling)?.admitted, Number(vector.admitted), JSON.stringify(vector));
    }
  } finally {
    db.close();
  }
}
