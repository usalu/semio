import assert from "node:assert/strict";
import { applyPatch } from "fast-json-patch";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/📦️physical-close/🔣️.json" with { type: "json" };
import schema from "../../🧫️fixtures/📦️physical-close/📐️schema.json" with { type: "json" };

/** 📦️ Distinguishes initialized payload length from the physical page released under a grant. */
export function testJobPayloadPhysicalClose(): void {
  assert.equal(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture), true);
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
      remainingLogicalBytes: closed.logicalBytes, remainingPages: closed.pages, chargedTotal: before.physicalBytes - closed.physicalBytes,
    };
    assert.deepEqual(output, fixture.expected, row.name);
  }
}
