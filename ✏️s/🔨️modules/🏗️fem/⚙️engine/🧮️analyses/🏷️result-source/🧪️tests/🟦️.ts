import assert from "node:assert/strict";
import { Buffer } from "node:buffer";
import Ajv from "ajv";
import { Matrix3, Vector3 } from "three";
import { applyPatch } from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { encodeResultSourceIdentity, decodeResultSourceIdentity } from "../🟦️.ts";

/** 🧫️ Independent schema, UTF-8, JSON Patch and matrix oracles preserve typed result sources. */
export function testResultSourceIdentity(): void {
  const valid = new Ajv({ strict: true }).compile(schema);
  for (const { identity, ...source } of fixture.sources) {
    assert.equal(valid(source), true);
    const expected = `scope:1:${Buffer.byteLength(source.kind)}:${source.kind}${Buffer.byteLength(source.id)}:${source.id}`;
    assert.equal(identity, expected);
    assert.equal(encodeResultSourceIdentity(source), identity);
    assert.deepEqual(decodeResultSourceIdentity(identity), source);
  }
  for (const identity of fixture.invalid) assert.throws(() => decodeResultSourceIdentity(identity));
  const { length: l, e, iy, force, factor } = fixture.cantilever, ei = e * iy;
  const stiffness = new Matrix3().set(12 * ei / l ** 3, -6 * ei / l ** 2, 0, -6 * ei / l ** 2, 4 * ei / l, 0, 0, 0, 1);
  const tip = new Vector3(force, 0, 0).applyMatrix3(stiffness.invert()).x;
  assert(Math.abs(tip - fixture.expected.loadCaseTy) < 1e-12);
  assert(Math.abs(factor * tip - fixture.expected.combinationTy) < 1e-12);
  const projection: Record<string, number> = {};
  applyPatch(projection, fixture.sources.slice(0, 2).map((source, index) => ({ op: "add" as const, path: `/${source.identity.replaceAll("~", "~0").replaceAll("/", "~1")}`, value: index === 0 ? tip : factor * tip })), true, true);
  assert.equal(Object.keys(projection).length, fixture.expected.count);
  assert.equal(projection[fixture.sources[0]!.identity], tip);
  assert.equal(projection[fixture.sources[1]!.identity], factor * tip);
  console.log(`[DEBUG] FEM typed result-source oracle: ${fixture.sources.length} canonical identities, two same-named physical sources retained, independent Three/Buffer/JSONPatch agreement`);
}
