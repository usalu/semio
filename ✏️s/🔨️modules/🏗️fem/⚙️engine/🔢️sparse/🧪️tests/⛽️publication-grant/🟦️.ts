import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/⛽️publication-grant/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/⛽️publication-grant/🔣️.json" with { type: "json" };

export function testFemPcgPublicationGrantOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...fixture, foreignOwner: true }));
  for (const row of fixture.cases) {
    const admitted = row.fuel > 0 && row.deadline > 0;
    const before = { pending: true, cursor: 0, fuelRemaining: row.fuel, outcome: "yield", terminalEmpty: false };
    const next = { ...before };
    if (admitted) {
      next.pending = row.kind === "complete";
      next.fuelRemaining -= 1;
      next.outcome = row.kind;
    }
    const patch: Operation[] = admitted ? [
      { op: "replace", path: "/pending", value: row.kind === "complete" },
      { op: "replace", path: "/fuelRemaining", value: 0 },
      { op: "replace", path: "/outcome", value: row.kind },
    ] : [];
    next.terminalEmpty = true;
    patch.push({ op: "replace", path: "/terminalEmpty", value: true });
    assert.deepEqual(next, row.expected);
    assert.deepEqual(applyPatch(before, patch, true, false).newDocument, row.expected);
  }
  console.log(`[DEBUG] FEM PCG publication admission: ${fixture.cases.length} cases agree with fast-json-patch ${fixture.oracle.version}`);
}
