import assert from "node:assert/strict";
import { applyPatch } from "fast-json-patch";
import fixture from "./🔣️.json";

/** 🎯️ RFC 6902 independently projects the host target onto the shared native action fixtures. */
export function testWindowActionContextOracle(): void {
  assert.notEqual(fixture.activeWindowId, fixture.clickedWindowId);
  for (const test of fixture.cases) {
    const result = applyPatch(structuredClone(test.args ?? {}), [{ op: "add", path: "/windowId", value: fixture.clickedWindowId }], true).newDocument;
    assert.deepEqual(result, test.expected);
  }
  console.log(`[DEBUG] JSON Patch matched ${fixture.cases.length} concrete-window action target cases`);
}
