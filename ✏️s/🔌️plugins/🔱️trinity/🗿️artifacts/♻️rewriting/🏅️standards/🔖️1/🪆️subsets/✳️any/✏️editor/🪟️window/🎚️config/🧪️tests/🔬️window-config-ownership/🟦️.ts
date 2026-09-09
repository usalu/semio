import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";

/** 🧪️ Independent JSON Patch reference for exact-window configuration ownership. */
export function testRewritingWindowConfigOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("./../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url), "utf8"));
  assert(!existsSync(new URL("../../../../🎚️config/🧬️schema/../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url)), "Rewriting must not declare an app configuration owner");
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-owner");
  const validate = ajv.compile(schema);
  const validateMutation = ajv.compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url), "utf8")));
  let windows = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.base)]));
  const generations: Record<string, number> = Object.fromEntries(Object.keys(windows).map((id) => [id, 0]));
  for (const row of fixture.cases) {
    assert(validateMutation(row.mutation), JSON.stringify(validateMutation.errors));
    const field = row.mutation.kind === "set-camera" ? "camera" : "lodMode";
    const value = row.mutation.kind === "set-camera" ? row.mutation.camera : row.mutation.value;
    windows = applyPatch(windows, [{ op: "replace", path: `/${row.windowId}/${field}`, value }], true, false).newDocument;
    generations[row.windowId] += 1;
    assert.deepEqual(windows, row.expected);
    for (const state of Object.values(windows)) assert(validate(state), JSON.stringify(validate.errors));
  }
  assert.deepEqual(generations, fixture.expectedWindowGenerations);
  console.log("[DEBUG] Rewriting window config oracle: independent camera/LOD updates, per-window generations, empty app config");
}
