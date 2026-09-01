/** ✍️ Checks bytewise source copy and separate physical retention with Node's streaming UTF-8 decoder. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

//#region ✍️WriterOracle
export function testInputWriterFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const source = Buffer.from(fixture.source.text, "utf8");
  assert.equal(source.toString("hex"), fixture.source.utf8Hex);
  assert.equal(source.byteLength, fixture.source.logicalBytes);
  const owned = Buffer.alloc(fixture.source.minimumCapacity);
  const decoder = new TextDecoder("utf-8", { fatal: true });
  let copied = 0, decoded = "";
  for (const [index, grant] of fixture.copy.byteGrants.entries()) {
    const end = Math.min(source.byteLength, copied + Number(grant));
    source.copy(owned, copied, copied, end);
    decoded += decoder.decode(owned.subarray(copied, end), { stream: true });
    copied = end;
    assert.equal(copied, fixture.copy.copied[index]);
    assert.equal(owned.byteLength, fixture.source.minimumCapacity);
  }
  decoded += decoder.decode();
  assert.equal(decoded, fixture.source.text);
  assert.equal(owned.subarray(0, copied).toString("hex"), fixture.source.utf8Hex);
  const copiedOwned = Buffer.from(owned.subarray(0, copied));
  let inspected = 0;
  for (const [index, grant] of fixture.close.byteGrants.entries()) {
    const end = Math.min(copied, inspected + Number(grant));
    owned.fill(0, inspected, end);
    inspected = end;
    assert.equal(inspected, fixture.close.inspected[index]);
    assert.equal(fixture.close.physicalReleasedDuringInspection[index], 0);
    assert.equal(owned.byteLength, fixture.source.minimumCapacity);
    assert.equal(owned.subarray(0, copied).toString("hex"), fixture.close.scrubHexAfterGrant[index]);
  }
  for (const frontier of fixture.unwind.frontiers) {
    assert(frontier >= 0 && frontier <= source.byteLength);
    assert.equal(source.subarray(0, frontier).toString("hex"), copiedOwned.subarray(0, frontier).toString("hex"));
  }
  assert.equal(new Set(fixture.refusals).size, 10);
  assert.equal(source.byteLength * 2, fixture.work.oneByteValidationAndCopyTurns);
  for (const vector of fixture.utf8) {
    const bytes = Buffer.from(vector.hex, "hex");
    const incremental = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });
    let valid = true, text = "";
    try {
      for (const byte of bytes) text += incremental.decode(Uint8Array.of(byte), { stream: true });
      text += incremental.decode();
    } catch { valid = false; }
    assert.equal(valid, vector.valid, vector.name);
    if (valid) assert.equal(Buffer.from(text, "utf8").toString("hex"), vector.hex, vector.name);
  }
  const hostiles = [
    { ...fixture, source: { ...fixture.source, logicalBytes: 9 } },
    { ...fixture, close: { ...fixture.close, physicalReleasedDuringInspection: [0, 1, 7] } },
    { ...fixture, authority: { ...fixture.authority, mutableRootEscapes: true } },
    { ...fixture, authority: { ...fixture.authority, callerVerdictAccepted: true } },
    { ...fixture, physical: { ...fixture.physical, chargeActualCapacity: false } },
    { ...fixture, extra: true },
  ];
  for (const hostile of hostiles) assert.equal(validate(hostile), false);
  console.log(`[DEBUG] input writer oracle: ${fixture.copy.byteGrants.length} byte-copy frontiers, ${fixture.close.byteGrants.length} retained-backing frontiers, ${fixture.utf8.length} incremental UTF-8 vectors, ${hostiles.length} schema hostiles; native writer, admission, unwind and allocation are separate gates`);
}
//#endregion ✍️WriterOracle
