import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

/** 🔁️ Independent closed-form area oracle for retained extrusion profiles. */
export function retainedExtrusionOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔁️retained-extrusion/🔣️.json", import.meta.url), "utf8"));
  let assertions = 0;
  assert.deepEqual(fixture.cases.map((row: { profile: string }) => row.profile), ["rectangle-wire", "hexagon-wire", "rectangle-face"]);
  for (const row of fixture.cases) {
    const area = row.profile === "hexagon-wire" ? row.sides * row.radius ** 2 * Math.sin(2 * Math.PI / row.sides) / 2 : row.width * row.height;
    assert.ok(Math.abs(area - row.area) < 1e-12);
    assertions++;
  }
  assert.equal(assertions, 3);
  return assertions;
}
