import assert from "node:assert/strict";
import { testFemPcgWireOracle } from "../📦️pcg-wire/🟦️.ts";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/⛽️publication-grant/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/⛽️publication-grant/🔣️.json" with { type: "json" };

export function testFemPcgPublicationGrantOracle(): void {
  testFemPcgWireOracle();
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...fixture, foreignOwner: true }));
  const admitsOrder = new Ajv2020({ strict: true }).compile({ type: "integer", minimum: 0, maximum: fixture.construction.backingCeilingBytes / fixture.construction.scalarBytes });
  for (const row of fixture.construction.cases) {
    assert.equal(row.order * fixture.construction.scalarBytes, row.requestedBytes);
    assert.equal(!admitsOrder(row.order), row.rejected);
    const before = { initialized: 0, allocatedBytes: 0, faulted: false };
    const after = { initialized: 0, allocatedBytes: row.rejected ? 0 : row.requestedBytes, faulted: row.rejected };
    const patch: Operation[] = row.rejected ? [{ op: "replace", path: "/faulted", value: true }] : [{ op: "replace", path: "/allocatedBytes", value: row.requestedBytes }];
    assert.deepEqual(applyPatch(before, patch, true, false).newDocument, after);
  }
  for (const row of fixture.cases) {
    const admitted = row.fuel > 0 && row.deadline > 0;
    const before = { pending: true, cursor: 0, fuelRemaining: row.fuel, outcome: "yield", terminalEmpty: false };
    const next = { ...before };
    if (admitted) {
      next.fuelRemaining -= 1;
    }
    const patch: Operation[] = admitted ? [
      { op: "replace", path: "/fuelRemaining", value: 0 },
    ] : [];
    next.terminalEmpty = true;
    patch.push({ op: "replace", path: "/terminalEmpty", value: true });
    assert.deepEqual(next, row.expected);
    assert.deepEqual(applyPatch(before, patch, true, false).newDocument, row.expected);
  }
  console.log(`[DEBUG] FEM PCG publication admission: ${fixture.cases.length} cases agree with fast-json-patch ${fixture.oracle.version}`);
}
