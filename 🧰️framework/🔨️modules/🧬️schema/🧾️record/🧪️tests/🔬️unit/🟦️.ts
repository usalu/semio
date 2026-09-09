/** 🧪️ Declared record boundaries agree with an independent JSON Schema oracle. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseSchemaRecord } from "../../🟦️.ts";
import vectors from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import emptySchema from "../../🔣️.json" with { type: "json" };

/** 🧾️ Checks allowed field ownership without evaluating foreign accessors. */
export function testSchemaRecordOracle(): void {
  const empty = new Ajv({ strict: true }).compile(emptySchema);
  for (const vector of vectors.emptyRecordCases) {
    assert.equal(empty(vector.value), vector.valid, "empty record schema admission");
    if (vector.valid) assert.equal(parseSchemaRecord(vector.value, []), vector.value);
    else assert.throws(() => parseSchemaRecord(vector.value, []));
  }
  const validate = new Ajv().compile({ type: "object", properties: Object.fromEntries(vectors.keys.map((key) => [key, {}])), additionalProperties: false });
  for (const value of vectors.valid) {
    assert.equal(validate(value), true);
    assert.equal(parseSchemaRecord(value, vectors.keys), value);
  }
  for (const value of vectors.invalid) {
    assert.equal(validate(value), false);
    assert.throws(() => parseSchemaRecord(value, vectors.keys));
  }
  let reads = 0;
  const accessor = Object.defineProperty({}, "name", { enumerable: true, get() { reads += 1; return "foreign"; } });
  for (const value of [new Date(0), Object.create({ name: "inherited" }), { [Symbol("name")]: "hidden" }, Object.defineProperty({}, "name", { value: "hidden" }), accessor]) assert.throws(() => parseSchemaRecord(value, vectors.keys));
  assert.equal(reads, 0);
  const dictionary = Object.assign(Object.create(null), { name: "plain" });
  assert.equal(parseSchemaRecord(dictionary, vectors.keys), dictionary);
  console.log("[DEBUG] Shared schema record parser agrees with 11 Ajv vectors and rejects non-JSON ownership/accessors without evaluation");
}
