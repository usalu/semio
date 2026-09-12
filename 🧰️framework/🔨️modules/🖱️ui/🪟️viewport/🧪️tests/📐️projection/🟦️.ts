import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020";
import jsonPatch from "fast-json-patch";
import { Matrix4, PerspectiveCamera } from "three";
import schema from "../../🧊️3d/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/📐️projection/🔣️.json";
import {
  defaultViewport3dProjectionPreferences,
  deriveActiveProjection,
  parseViewport3dProjectionPreferences,
  parseViewport3dProjectionSpec,
} from "../../🧊️3d/🧬️schema/🟦️.ts";

type Patch = Parameters<typeof jsonPatch.applyPatch>[1];

/** 🧪️ Proves strict neutral projection admission, full-bank retention, and pure derivation against shared fixtures and independent JSON tools. */
export function testViewport3dProjectionValues(): void {
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const preferencesSchema = ajv.getSchema(`${schema.$id}#/$defs/projectionPreferences`);
  const specSchema = ajv.getSchema(`${schema.$id}#/$defs/projectionSpec`);
  assert.ok(preferencesSchema);
  assert.ok(specSchema);

  const defaults = defaultViewport3dProjectionPreferences();
  assert.deepEqual(defaults, fixture.defaultPreferences);
  assert.equal(preferencesSchema(defaults), true, JSON.stringify(preferencesSchema.errors));
  assert.deepEqual(parseViewport3dProjectionPreferences(JSON.parse(JSON.stringify(defaults))), defaults);

  let combinations = 0;
  for (const mode of fixture.modes) {
    for (const orientation of fixture.orientations) {
      const value = { mode, orientation };
      assert.equal(specSchema(value), true, JSON.stringify(specSchema.errors));
      assert.deepEqual(parseViewport3dProjectionSpec(JSON.parse(JSON.stringify(value))), value);
      combinations += 1;
    }
  }

  const perspectiveSpec = parseViewport3dProjectionSpec({ mode: fixture.modes[5], orientation: { type: "free" } });
  assert.equal(perspectiveSpec.mode.kind, "threePoint");
  if (perspectiveSpec.mode.kind !== "threePoint") throw new TypeError("Expected three-point oracle mode");
  const perspective = new PerspectiveCamera(perspectiveSpec.mode.fov, 16 / 9, 0.2, 1000);
  perspective.updateProjectionMatrix();
  assert.ok(perspective.projectionMatrix.elements.every(Number.isFinite));

  const obliqueSpec = parseViewport3dProjectionSpec({ mode: fixture.modes[2], orientation: { type: "free" } });
  assert.equal(obliqueSpec.mode.kind, "oblique");
  if (obliqueSpec.mode.kind !== "oblique") throw new TypeError("Expected oblique oracle mode");
  const shear = new Matrix4().identity();
  const radians = (obliqueSpec.mode.angle * Math.PI) / 180;
  shear.elements[8] = -obliqueSpec.mode.depthScale * Math.cos(radians);
  shear.elements[9] = -obliqueSpec.mode.depthScale * Math.sin(radians);
  assert.ok(shear.elements.every(Number.isFinite));

  const curvilinearSpec = parseViewport3dProjectionSpec({ mode: fixture.modes[6], orientation: { type: "free" } });
  assert.equal(curvilinearSpec.mode.kind, "curvilinear");
  if (curvilinearSpec.mode.kind !== "curvilinear") throw new TypeError("Expected curvilinear oracle mode");
  assert.equal(curvilinearSpec.mode.fov, 200);
  const capture = new PerspectiveCamera(Math.min(curvilinearSpec.mode.fov, 160), 16 / 9, 0.2, 1000);
  capture.updateProjectionMatrix();
  assert.ok(capture.projectionMatrix.elements.every(Number.isFinite));

  for (const row of fixture.derivations) {
    const patched = jsonPatch.applyPatch(structuredClone(fixture.defaultPreferences), row.patch as Patch, true).newDocument;
    const preferences = parseViewport3dProjectionPreferences(patched);
    const expected = parseViewport3dProjectionSpec(row.expected);
    const actual = deriveActiveProjection(preferences);
    assert.equal(preferencesSchema(preferences), true, row.name);
    assert.equal(specSchema(actual), true, row.name);
    assert.deepEqual(actual, expected, row.name);
  }

  for (const row of fixture.preferenceRejections) {
    const invalid = jsonPatch.applyPatch(structuredClone(fixture.defaultPreferences), row.patch as Patch, true).newDocument;
    assert.equal(preferencesSchema(invalid), false, row.name);
    assert.throws(() => parseViewport3dProjectionPreferences(invalid), TypeError, row.name);
  }
  for (const invalid of fixture.specRejections) {
    assert.equal(specSchema(invalid), false, JSON.stringify(invalid));
    assert.throws(() => parseViewport3dProjectionSpec(invalid), TypeError, JSON.stringify(invalid));
  }

  for (const nonfinite of [NaN, Infinity, -Infinity]) {
    for (const field of ["axonometricAngleA", "axonometricAngleB", "obliqueAngle", "obliqueDepth", "fov", "twoPointShift", "curvilinearFov", "curvilinearStrength"] as const) {
      const invalid = { ...fixture.defaultPreferences, [field]: nonfinite };
      assert.equal(preferencesSchema(invalid), false, `${field} ${nonfinite}`);
      assert.throws(() => parseViewport3dProjectionPreferences(invalid), TypeError, `${field} ${nonfinite}`);
    }
    for (const invalid of [
      { mode: { kind: "axonometric", variant: "trimetric", angleA: nonfinite, angleB: 13 }, orientation: { type: "free" } },
      { mode: { kind: "axonometric", variant: "trimetric", angleA: 22, angleB: nonfinite }, orientation: { type: "free" } },
      { mode: { kind: "oblique", variant: "cabinet", angle: nonfinite, depthScale: 0.5 }, orientation: { type: "free" } },
      { mode: { kind: "oblique", variant: "cabinet", angle: 45, depthScale: nonfinite }, orientation: { type: "free" } },
      { mode: { kind: "onePoint", fov: nonfinite }, orientation: { type: "free" } },
      { mode: { kind: "twoPoint", fov: nonfinite, verticalShift: 0 }, orientation: { type: "free" } },
      { mode: { kind: "twoPoint", fov: 50, verticalShift: nonfinite }, orientation: { type: "free" } },
      { mode: { kind: "threePoint", fov: nonfinite }, orientation: { type: "free" } },
      { mode: { kind: "curvilinear", fov: nonfinite, strength: 1, mapping: "fisheye" }, orientation: { type: "free" } },
      { mode: { kind: "curvilinear", fov: 120, strength: nonfinite, mapping: "fisheye" }, orientation: { type: "free" } },
    ]) {
      assert.equal(specSchema(invalid), false, JSON.stringify(invalid));
      assert.throws(() => parseViewport3dProjectionSpec(invalid), TypeError);
    }
  }

  const edited = jsonPatch.applyPatch(structuredClone(fixture.defaultPreferences), fixture.retention.editWhileOrthographic as Patch, true).newDocument;
  const retained = parseViewport3dProjectionPreferences(edited);
  assert.equal(retained.kind, "orthographic");
  assert.equal(retained.curvilinearFov, 150);
  assert.equal(retained.curvilinearStrength, 0.4);
  assert.equal(retained.curvilinearMapping, "panini");
  const recalled = jsonPatch.applyPatch(structuredClone(retained), fixture.retention.recallCurvilinear as Patch, true).newDocument;
  assert.deepEqual(deriveActiveProjection(parseViewport3dProjectionPreferences(recalled)), fixture.retention.expected);
  assert.deepEqual(defaults, fixture.defaultPreferences);
  console.log(`[DEBUG] Shared viewport projection values matched Ajv, fast-json-patch, Three.js finite matrices, ${combinations} mode-orientation pairs including outside-control active values, ${fixture.derivations.length} derivations, and strict rejection laws`);
}
