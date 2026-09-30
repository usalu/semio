/** 🚪️ Neutral extension close vectors agree with AJV and independent UTF-8 encoders. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

export function extensionRetirementOracle(repoRoot: string): number {
  const directory = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️extension-retirement");
  const fixture = JSON.parse(readFileSync(resolve(directory, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(directory, "📐️schema.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const native = Buffer.from(fixture.resource, "utf8");
  const portable = new TextEncoder().encode(fixture.resource);
  assert.deepEqual([...portable], [...native]);
  assert.equal(native.length, fixture.resourceBytes);
  for (const grant of fixture.zeroGrants) assert(grant.some((value: number) => value === 0));
  for (const [field, value] of [["replacementSlots", 2], ["resourceBytes", -1], ["lifecycle", ["terminal", "open"]]] as const) {
    assert.equal(validate({ ...fixture, [field]: value }), false);
  }
  const metadata = JSON.stringify(fixture.metadata);
  assert.deepEqual(JSON.parse(metadata), fixture.metadata);
  assert.equal(validate({ ...fixture, dropBoundary: { ...fixture.dropBoundary, liveOwnerDrops: 1 } }), false);
  const allocation = fixture.allocationAdmission;
  assert.equal(new ArrayBuffer(allocation.vectorCapacity * allocation.podWidth).byteLength, Buffer.alloc(allocation.vectorCapacity * allocation.podWidth).byteLength);
  assert.equal(validate({ ...fixture, allocationAdmission: { ...allocation, tinyBytes: 0 } }), false);
  const actor = fixture.actorAllocation;
  assert.equal(Buffer.from("x".repeat(actor.stringBytes), "utf8").byteLength, actor.stringBytes);
  assert(actor.smallBytes < actor.stringBytes && actor.stringBytes < actor.adequateBytes);
  return 13;
}
