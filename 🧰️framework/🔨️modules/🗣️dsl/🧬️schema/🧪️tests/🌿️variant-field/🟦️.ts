/** 🌿️ Ajv independently checks the single named-record field contract. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧪️ Validates authored statements and arity without the DSL implementation. */
export function testDeclaredVariantFieldOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../../🧪️tests/🧫️fixtures/🫳️borrowed-object/🌿️variant-field/🔣️.json", import.meta.url), "utf8"));
  const reference = new Ajv({ strict: true }).compile({
    type: "array", minItems: 1, maxItems: 1,
    items: {
      type: "array", minItems: 2, maxItems: 2,
      items: [
        { type: "string", enum: ["flag", "text"] },
        { type: "object", required: ["0"], additionalProperties: false, properties: { "0": { anyOf: [{ type: "boolean" }, { type: "string" }, { type: "null" }] } } },
      ],
      additionalItems: false,
    },
  });
  for (const row of fixture.valid) {
    const named = [[row.keyword, { "0": row.value }]];
    assert.deepEqual(named, row.named);
    assert(reference(named), JSON.stringify(reference.errors));
    for (const count of fixture.refusedStatementCounts) assert(!reference(Array.from({ length: count }, () => named[0])));
  }
}

