import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 🚪️ Independent JSON Schema admission verdicts for the native value derive hooks. */
export function testValueSemanticAdmission(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const row of fixture.cases) assert.equal(validate(row.input), row.admitted, row.id);
  assert.equal(validate(fixture.unitCase.normalized), fixture.unitCase.admitted);
  console.log(`[DEBUG] Value semantic admission: ${fixture.cases.length} neutral verdicts agree with independent strict Ajv`);
}
