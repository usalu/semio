/** 🧵️ Validates native codec semantic fixtures independently of Rust Send checking. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import _ from "lodash";

//#region 🧵️CodecSendOracle
export function testNativeCodecSendFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const snapshot of fixture.snapshots) {
    assert.deepEqual(JSON.parse(JSON.stringify(snapshot)), _.cloneDeep(snapshot));
    if (snapshot.n !== null) {
      const bytes = Buffer.alloc(4);
      bytes.writeInt32LE(snapshot.n);
      assert.equal(bytes.readInt32LE(), snapshot.n);
    }
  }
  assert.deepEqual(fixture.slots.map((slot: { name: string }) => slot.name), ["compile_dsl", "print_mirror"]);
  for (const hostile of [{ ...fixture, snapshots: [{ n: 2147483648 }, ...fixture.snapshots.slice(1)] }, { ...fixture, snapshots: [{ n: -2147483649 }, ...fixture.snapshots.slice(1)] }, { ...fixture, snapshots: [{ n: 0.5 }, ...fixture.snapshots.slice(1)] }, { ...fixture, slots: [{ ...fixture.slots[0], send: false }, fixture.slots[1]] }, { ...fixture, invariants: { ...fixture.invariants, localExecutorFallback: true } }]) assert.equal(validate(hostile), false);
  console.log("[DEBUG] native codec schema/Lodash/Buffer oracle: 4 exact snapshot values, 2 declared Send slots, 5 hostiles; no Rust Send or bounded serialization claim");
}
//#endregion 🧵️CodecSendOracle
