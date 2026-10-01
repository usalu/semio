import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { addSemioMutationLeafSchemasV1, semioSchemaAjvV1 } from "../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { applyPatch } from "fast-json-patch";

/** 🧪️ Independent JSON Patch reference for exact-window configuration ownership. */
export function testRewritingWindowConfigOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  assert(!existsSync(new URL("../../../../🎚️config/🧬️schema/🔣️.json", import.meta.url)), "Rewriting must not declare an app configuration owner");
  const ajv = semioSchemaAjvV1({ allErrors: true });
  addSemioMutationLeafSchemasV1(ajv, new URL("../../🧬️schema/🧬️mutations", import.meta.url));
  const validate = ajv.compile(schema);
  const validateMutation = ajv.compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8")));
  let windows = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.base)]));
  const generations: Record<string, number> = Object.fromEntries(Object.keys(windows).map((id) => [id, 0]));
  for (const row of fixture.cases) {
    const mutation: { kind: string; camera?: unknown; value?: unknown } = JSON.parse(readFileSync(new URL(`./../../🧫️fixtures/🧬️mutations/${row.witness}/🧾️wire-witness/🦠️mutation/🔣️.json`, import.meta.url), "utf8"));
    assert(validateMutation(mutation), JSON.stringify(validateMutation.errors));
    const field = mutation.kind === "set-camera" ? "camera" : "lodMode";
    const value = mutation.kind === "set-camera" ? mutation.camera : mutation.value;
    windows = applyPatch(windows, [{ op: "replace", path: `/${row.windowId}/${field}`, value }], true, false).newDocument;
    generations[row.windowId] += 1;
    assert.deepEqual(windows, row.expected);
    for (const state of Object.values(windows)) assert(validate(state), JSON.stringify(validate.errors));
  }
  assert.deepEqual(generations, fixture.expectedWindowGenerations);
}
