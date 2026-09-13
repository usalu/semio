import assert from "node:assert/strict";
import { createRequire } from "node:module";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/📦️preparation-owners/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/📦️preparation-owners/🔣️.json" with { type: "json" };

export function testMeshPreparationOwners(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  const { applyPatch } = createRequire(import.meta.url)("fast-json-patch");
  for (const widths of corpus.slotWidths) for (const row of corpus.cases) {
    const lengths = [row.maximumPoints, row.maximumPoints, row.maximumTriangles * 3];
    let live = [false, false, false], fault = false;
    const reservations: boolean[][] = [];
    for (const [index, length] of lengths.entries()) {
      if (length * widths[index] > corpus.maximumBackingBytes) fault = true;
      else live = applyPatch(live, [{ op: "replace", path: "/" + index, value: true }], true, false).newDocument;
      reservations.push([...live]);
      if (fault) break;
    }
    assert.deepEqual({ reservations, fault, preservesIndexOnHandoff: !fault && live[1] }, row.expected, row.id);
    const retained = applyPatch(live, [0, 2].filter(() => !fault).map((index) => ({ op: "replace", path: "/" + index, value: false })), true, false).newDocument;
    assert.equal(retained[1], live[1], row.id + " retains the lookup owner after handoff");
  }
  for (const row of corpus.fillCases) {
    let state = { points: 0, constraints: 0, pendingPoint: false, pendingIndex: false };
    const update = (field: keyof typeof state, value: number | boolean) => {
      state = applyPatch(state, [{ op: "replace", path: "/" + field, value }], true, false).newDocument;
    };
    for (let index = 0; index < row.outer.length; index++) {
      update("pendingPoint", true);
      if (state.points === row.maximumPoints) break;
      update("points", state.points + 1);
      update("pendingPoint", false);
      update("pendingIndex", true);
      if (index > 0) {
        if (state.constraints === row.maximumTriangles * 3) break;
        update("constraints", state.constraints + 1);
      }
      update("pendingIndex", false);
    }
    if (!state.pendingPoint && !state.pendingIndex && state.constraints < row.maximumTriangles * 3) update("constraints", state.constraints + 1);
    assert.deepEqual(state, row.expected, row.id);
  }
  console.log("[DEBUG] Mesh preparation ownership matches four reservation cases at both pointer widths and three pre-append refusal cases through JSONPatch");
}
