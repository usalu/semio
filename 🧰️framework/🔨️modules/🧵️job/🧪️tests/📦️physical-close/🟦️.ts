import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📦️ Distinguishes initialized payload length from the physical page released under a grant. */
export function testJobPayloadPhysicalClose(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    assert(row.insufficientGrant < fixture.pageBytes);
    const before = { logicalBytes: row.logicalBytes, physicalBytes: fixture.pageBytes, pages: 1 };
    assert.deepEqual(applyPatch(structuredClone(before), [], true).newDocument, before);
    const closed = applyPatch(structuredClone(before), [
      { op: "replace", path: "/logicalBytes", value: 0 },
      { op: "replace", path: "/physicalBytes", value: 0 },
      { op: "replace", path: "/pages", value: 0 },
    ], true).newDocument;
    const output = {
      refusedItems: 0, refusedBytes: 0, retainedPointer: true,
      releasedItems: before.pages - closed.pages, releasedBytes: before.physicalBytes - closed.physicalBytes,
      remainingLogicalBytes: closed.logicalBytes, remainingPages: closed.pages,
    };
    assert.deepEqual(output, fixture.expected, row.name);
  }
  console.log("[DEBUG] Job payload physical release agrees with Ajv and fast-json-patch for empty, one-byte, short and full pages");
}
