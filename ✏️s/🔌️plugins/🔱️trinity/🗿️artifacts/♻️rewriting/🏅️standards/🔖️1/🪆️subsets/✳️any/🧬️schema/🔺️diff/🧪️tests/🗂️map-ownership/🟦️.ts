import assert from "node:assert/strict";
import jsonPatch from "fast-json-patch";
import fixture from "./🔣️.json" with { type: "json" };

/** 🗂️ Independently validates null-valued entry and sequential map-change fixtures. */
export function testRewritingMapOwnershipOracle(): void {
  for (const row of fixture.cases) {
    const actual = jsonPatch.applyPatch(structuredClone(row.before), row.patches as jsonPatch.Operation[], true, false).newDocument;
    assert.deepEqual(actual, row.after, row.name);
  }
  console.log("[DEBUG] independent JSON Patch preserved null-valued entries and validated insert/remove composition");
}
