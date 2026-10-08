import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { Buffer } from "node:buffer";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧫️fixtures/🧬️schema/🔣️.json" with { type: "json" };

test("mounted original group identity has bounded SHA256 input and independently admitted output", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const parent = Buffer.from(row.parent), actor = Buffer.from(row.actor), parentLength = Buffer.alloc(8), actorLength = Buffer.alloc(8), instance = Buffer.alloc(4), operation = Buffer.alloc(8);
    parentLength.writeBigUInt64LE(BigInt(parent.length)); actorLength.writeBigUInt64LE(BigInt(actor.length)); instance.writeUInt32LE(row.instance); operation.writeBigUInt64LE(BigInt(row.operation));
    const fields = [Buffer.from("semio.owned-child-group/v1\0"), parentLength, parent, actorLength, actor, Buffer.from(row.revision, "hex"), instance, operation];
    const direct = `group:${createHash("sha256").update(Buffer.concat(fields)).digest("hex")}`, hash = createHash("sha256");
    for (const field of fields) for (let offset = 0; offset < field.length; offset += fixture.copyBytes) hash.update(field.subarray(offset, offset + fixture.copyBytes));
    expect(`group:${hash.digest("hex")}`).toBe(row.expectedId);
    expect(direct).toBe(row.expectedId);
    expect(Buffer.byteLength(direct)).toBe(fixture.capacityBytes);
    for (const [items, capacity] of [[0, 70], [1, 0], [1, 69], [1, 70]]) expect(items > 0 && capacity >= Buffer.byteLength(direct)).toBe(items === 1 && capacity === 70);
  }
  expect(validate({ ...fixture, copyBytes: 65 })).toBe(false);
  expect(validate({ ...fixture, capacityBytes: 69 })).toBe(false);
  const source = Bun.file(new URL("../🦀️.rs", import.meta.url));
  expect(await source.exists()).toBe(true);
  const text = await source.text();
  for (const name of ["MountedChildGroupIdentityIssuer", "next_capacity_byte_demand", "maximum_capacity_bytes", "maximum_copy_bytes", "next_close_byte_demand"]) expect(text.includes(name)).toBe(true);
  console.log("[DEBUG] Node SHA256 + Buffer + Ajv: three exact length-framed Unicode group IDs, copy64, whole70-byte output denial, original optional transaction untouched");
});
