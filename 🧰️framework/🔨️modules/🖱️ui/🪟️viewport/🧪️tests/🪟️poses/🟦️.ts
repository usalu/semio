import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020";
import jsonPatch from "fast-json-patch";
import planarSchema from "../../◻️2d/🧬️schema/🔣️.json";
import orbitSchema from "../../🧊️3d/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🪟️poses/🔣️.json";
import { parseViewport2d } from "../../◻️2d/🧬️schema/🟦️.ts";
import { parseViewport3dOrbit } from "../../🧊️3d/🧬️schema/🟦️.ts";

/** 🧪️ Compares first-party navigation admission and addressed replacement with independent schema and patch implementations. */
export function testViewportOwnership(): void {
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  const validators = { "2d": ajv.compile(planarSchema), "3d": ajv.compile(orbitSchema) };
  for (const row of fixture.cases) {
    const dimension = row.dimension as "2d" | "3d";
    const parse = dimension === "2d" ? parseViewport2d : parseViewport3dOrbit;
    assert.equal(validators[dimension](row.value), row.valid, row.name);
    if (row.valid) assert.deepEqual(parse(row.value), row.value, row.name);
    else assert.throws(() => parse(row.value), TypeError, row.name);
  }
  for (const value of [NaN, Infinity, -Infinity]) {
    for (const field of ["x", "y", "zoom"]) {
      const input = { x: 0, y: 0, zoom: 1, [field]: value };
      assert.equal(validators["2d"](input), false);
      assert.throws(() => parseViewport2d(input));
    }
    for (const invalid of [
      { ...fixture.windowIsolation.left, position: [value, 0, 1] },
      { ...fixture.windowIsolation.left, target: [0, value, 1] },
      { ...fixture.windowIsolation.left, up: [0, value, 1] },
      { ...fixture.windowIsolation.left, zoom: value },
    ]) {
      assert.equal(validators["3d"](invalid), false);
      assert.throws(() => parseViewport3dOrbit(invalid));
    }
  }
  const before = { windows: { left: fixture.windowIsolation.left, right: fixture.windowIsolation.right }, document: { authoredCameras: ["semantic-camera"] }, os: { locale: "de" } };
  const expected = jsonPatch.applyPatch(structuredClone(before), [{ op: "replace", path: "/windows/left", value: fixture.windowIsolation.replacement }], true).newDocument;
  const actual = structuredClone(before);
  actual.windows.left = parseViewport3dOrbit(fixture.windowIsolation.replacement);
  assert.deepEqual(actual, expected);
  assert.deepEqual(actual.windows.right, before.windows.right);
  assert.deepEqual(actual.document, before.document);
  assert.deepEqual(actual.os, before.os);
  console.log(`[DEBUG] Shared viewport admission matches Ajv for ${fixture.cases.length} neutral cases and nonfinite fields; exact-window replacement matches fast-json-patch`);
}
