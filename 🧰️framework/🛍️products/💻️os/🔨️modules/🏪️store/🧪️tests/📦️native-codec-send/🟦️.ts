import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../📦️codec/🧵️send/📜️script.ts", import.meta.url);
/** 🧵️ Validates native codec semantic fixtures independently of Rust Send checking. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import _ from "lodash";

//#region 🧵️CodecSendOracle
export function testNativeCodecSendFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", testSourceUrl.href), "utf8"));
  const contract = JSON.parse(readFileSync(new URL("../../🧪️testing/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(contract);
  const validate = ajv.getSchema(`${contract.$id}#/$defs/NullableI32`)!;
  for (const snapshot of fixture.snapshots) {
    assert(validate(snapshot.n), JSON.stringify(validate.errors));
    assert.deepEqual(JSON.parse(JSON.stringify(snapshot)), _.cloneDeep(snapshot));
    if (snapshot.n !== null) {
      const bytes = Buffer.alloc(4);
      bytes.writeInt32LE(snapshot.n);
      assert.equal(bytes.readInt32LE(), snapshot.n);
    }
  }
  assert.deepEqual(fixture.slots.map((slot: { name: string }) => slot.name), ["compile_dsl", "print_mirror"]);
  for (const hostile of [2147483648, -2147483649, 0.5]) assert.equal(validate(hostile), false);
  assert(fixture.slots.every((slot: { send: boolean }) => slot.send));
  assert.equal(fixture.invariants.localExecutorFallback, false);
  console.log("[DEBUG] native codec Send laws preserve real nullable I32 Ajv admission, Buffer and lodash snapshot identity");
}
//#endregion 🧵️CodecSendOracle
