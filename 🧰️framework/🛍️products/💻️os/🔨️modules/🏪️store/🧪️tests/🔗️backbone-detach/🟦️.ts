import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../🔗️backbone/✂️detach/📜️script.ts", import.meta.url);
/** ✂️ Checks refusal conservation models; real Store ownership is tested separately. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import _ from "lodash";

//#region ✂️BackboneDetachOracle
export function testBackboneDetachFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", testSourceUrl.href), "utf8"));
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
}
//#endregion ✂️BackboneDetachOracle
