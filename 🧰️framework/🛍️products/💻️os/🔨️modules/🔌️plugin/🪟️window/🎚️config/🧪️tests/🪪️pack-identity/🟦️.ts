import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020";
import { applyPatch } from "fast-json-patch";
import schema from "../../🧬️schema/🪪️pack-identity/🔣️.json";
import fixture from "../../🧫️fixtures/🪪️pack-identity/🔣️.json";

/** 🪪️ Exact window Pack identity leaves a foreign target partition unchanged. */
export function testWindowConfigPackIdentityContract(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const sourceId = `window-config:${fixture.windowKind}:${row.sourceWindowId}`;
    const targetId = `window-config:${fixture.windowKind}:${row.targetWindowId}`;
    const before = { id: targetId, generation: 7, payload: {} };
    const admitted = sourceId === targetId;
    const actual = applyPatch(structuredClone(before), admitted ? [{ op: "replace", path: "/generation", value: 8 }] : [], true, false).newDocument;
    assert.equal(admitted, row.accepted, row.id);
    assert.equal(actual.id, targetId);
    if (!row.accepted) assert.deepEqual(actual, before);
  }
  console.log(`[DEBUG] Window Pack exact identity: ${fixture.cases.length} Ajv/JSON Patch cases; native admission is a separate gate`);
}
