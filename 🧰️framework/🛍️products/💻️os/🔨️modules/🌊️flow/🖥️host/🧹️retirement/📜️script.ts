/** 🧹️ Flow session byte ownership fixtures and independent JSON oracle; source validation only. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";

//#region 🔣️SessionOwnership
const fixture = await Bun.file(new URL("./🔣️session.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🔣️session.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
const text = fixture.text.text.repeat(fixture.text.repeat); const preview = fixture.preview.text.repeat(fixture.preview.repeat);
const owners = [text, "{}", "mesh", preview, "pending", "geometry", "output", "label", preview, "label", text];
assert.equal(owners.reduce((sum, owner) => sum + Buffer.byteLength(owner), 0), fixture.expected.releasedBytes);
assert(fixture.text.reservedCapacity > Buffer.byteLength(text));
assert.equal(stableStringify({ label: text }), JSON.stringify({ label: text }));
for (const grant of fixture.grants) {
  let total = 0;
  for (const owner of owners) { let left = Buffer.byteLength(owner); while (left) { const released = Math.min(grant, left); total += released; left -= released; } }
  assert.equal(total, fixture.expected.releasedBytes);
}
for (const mutant of [{ ...fixture, extra: true }, { ...fixture, grants: [16384] }, { ...fixture, expected: { ...fixture.expected, zeroGrant: "progress" } }]) assert(!validate(mutant));
console.log("[DEBUG] Flow session-retirement source fixtures=1 hostileRejections=3 bytes=42405 grants=1,64,4096 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🔣️SessionOwnership
