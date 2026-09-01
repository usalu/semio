/** 📥️ Checks single-enqueue wire tuples independently with Node Buffer and strict Ajv. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

//#region 📥️SingleEnqueueOracle
export function testSingleEnqueuePublicationFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const tuple = (state: Record<string, string>): Buffer => {
    const bytes = Buffer.alloc(24);
    for (const [index, field] of fixture.fields.entries()) bytes.writeBigUInt64LE(BigInt(state[field]), index * 8);
    return bytes;
  };
  for (const state of ["old", "half", "committed"]) assert.equal(tuple(fixture[state]).toString("hex"), fixture[state].leHex);
  assert.equal(tuple(fixture.half).equals(tuple(fixture.old)) || tuple(fixture.half).equals(tuple(fixture.committed)), false);
  assert.equal(fixture.old.observedBuildInputGeneration, fixture.committed.observedBuildInputGeneration);
  for (const invariant of Object.keys(fixture.invariants)) {
    const hostile = structuredClone(fixture);
    hostile.invariants[invariant] = true;
    assert.equal(validate(hostile), false);
  }
  console.log("[DEBUG] single-enqueue schema/Buffer oracle: 3 exact 24-byte tuples, 5 hostiles; queue+scene only, unchanged build input; native interlock is a separate gate");
}
//#endregion 📥️SingleEnqueueOracle
