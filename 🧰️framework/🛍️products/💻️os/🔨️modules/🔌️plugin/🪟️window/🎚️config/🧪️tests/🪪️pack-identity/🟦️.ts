import assert from "node:assert/strict";

import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🪪️pack-identity/🔣️.json";

/** 🪪️ Exact window Pack identity neither changes nor materializes a foreign target partition. */
export function testWindowConfigPackIdentityContract(): void {
  for (const row of fixture.cases) {
    const sourceId = `window-config:${fixture.windowKind}:${row.sourceWindowId}`;
    const targetId = `window-config:${fixture.windowKind}:${row.targetWindowId}`;
    const before = {
      partitions: {
        [sourceId]: { generation: 3, payload: {} },
        ...(row.targetExists && targetId !== sourceId ? { [targetId]: { generation: 7, payload: {} } } : {}),
      },
    };
    const admitted = sourceId === targetId;
    const actual = applyPatch(
      structuredClone(before),
      admitted ? [{ op: "replace", path: `/partitions/${sourceId.replaceAll("~", "~0").replaceAll("/", "~1")}/generation`, value: 8 }] : [],
      true,
      false,
    ).newDocument;
    assert.equal(admitted, row.accepted, row.id);
    if (!row.accepted) assert.deepEqual(actual, before);
    if (!row.targetExists && !row.accepted) assert.equal(Object.hasOwn(actual.partitions, targetId), false, `${row.id}: foreign target remains absent`);
  }
}
