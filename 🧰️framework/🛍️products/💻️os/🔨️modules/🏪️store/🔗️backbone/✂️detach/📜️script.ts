/** ✂️ Checks refusal conservation models; real Store ownership is tested separately. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import _ from "lodash";

//#region ✂️BackboneDetachOracle
export function testBackboneDetachFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const maximum = BigInt(fixture.generationMaximum);
  const exactMaximum = Buffer.alloc(8, 255).readBigUInt64LE();
  assert.equal(maximum, exactMaximum);
  for (const row of fixture.cases) {
    const root = { descriptor: { uri: "detach-local" }, generation: row.failure === "generation" ? maximum : 1n, backbone: {}, payload: Buffer.alloc(fixture.payload.length, fixture.payload.byte) };
    const before = { ...root };
    const occupied = row.failure === "capacity" ? fixture.capacity : 0;
    const refused = occupied >= fixture.capacity || root.generation >= maximum;
    const next = refused ? root : { ...root, descriptor: null, generation: root.generation + 1n };
    const observed = { refused, panicked: false, descriptorPreserved: _.eq(next.descriptor, before.descriptor), generationPreserved: next.generation === before.generation, backbonePreserved: _.eq(next.backbone, before.backbone), payloadPreserved: _.eq(next.payload, before.payload) && next.payload.equals(before.payload) };
    assert.deepEqual(observed, row.expected);
  }
  for (const hostile of [{ ...fixture, generationMaximum: "18446744073709551616" }, { ...fixture, capacity: 1025 }, { ...fixture, cases: fixture.cases.map((row: object) => ({ ...row, expected: {} })) }, { ...fixture, extra: true }]) assert.equal(validate(hostile), false);
  console.log("[DEBUG] backbone detach schema/Lodash/Buffer oracle: 2 refusal models, 4 hostiles; 5 pending lifecycle boundaries remain unimplemented/unexecuted");
}
//#endregion ✂️BackboneDetachOracle
