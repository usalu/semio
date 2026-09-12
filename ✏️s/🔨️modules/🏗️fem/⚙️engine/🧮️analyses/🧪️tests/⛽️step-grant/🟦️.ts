import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/⛽️step-grant/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/⛽️step-grant/🔣️.json" with { type: "json" };

export function testFemAssemblyStepGrantOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...fixture, foreignOwner: true }));
  for (const row of fixture.cases) {
    const admitted = row.fuel > 0 && row.deadline > 0;
    const next = structuredClone(row.before);
    if (admitted) {
      if (next.checkpointDue) next.checkpointDue = false;
      else if (next.previewDue) next.previewDue = false;
      else if (!next.complete) next.pendingBuild = true;
    }
    const field = { checkpoint: "checkpointDue", preview: "previewDue", work: "pendingBuild" }[row.transition as "checkpoint" | "preview" | "work"];
    const patch: Operation[] = admitted && field ? [{ op: "replace", path: "/" + field, value: row.transition === "work" }] : [];
    const oracle = applyPatch(structuredClone(row.before), patch, true, false).newDocument;
    assert.deepEqual(next, row.after);
    assert.deepEqual(oracle, row.after);
    assert.equal(admitted ? row.fuel - 1 : row.fuel, row.fuelRemaining);
    assert.equal(admitted && next.complete ? "complete" : "yield", row.outcome);
  }
  console.log(`[DEBUG] FEM assembly step grant: ${fixture.cases.length} strict fixture cases match fast-json-patch ${fixture.oracle.version}`);
}
