import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../🔗️backbone/✂️detach/📜️script.ts", import.meta.url);
/** ✂️ Checks refusal conservation models; real Store ownership is tested separately. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import _ from "lodash";
import Ajv from "ajv";

//#region ✂️BackboneDetachOracle
export function testBackboneDetachFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", testSourceUrl.href), "utf8"));
  assert.ok(fixture.cleanup, "Original detach examples declare independent full cleanup authority");
  const contract = JSON.parse(readFileSync(new URL("../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const valid = new Ajv({strict:false,allErrors:true}).compile({...contract,$ref:"#/$defs/Grant"});
  for (const grant of Object.values(fixture.cleanup) as Record<string,number>[]) {
    assert.equal(valid(grant),true);
    for (const axis of contract.$defs.Grant.required) { const missing={...grant};delete missing[axis];assert.equal(valid(missing),false);assert.equal(valid({...grant,[axis]:-1}),false); }
  }
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
  console.log("[DEBUG] original detach examples / independent genuine Grant Ajv omissions / Lodash identity / Buffer refusal conservation");
}
//#endregion ✂️BackboneDetachOracle
