import assert from "node:assert/strict";
import { test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import { TextError } from "../🟦️.ts";
import { ValueError, type ValueRefusalKind } from "../../../🌱️value/⚠️refusal/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import diagnosticSchema from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
import valueSchema from "../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json" with { type: "json" };

test("owned TextError scalar construction preserves schema kind wire and display against Ajv and SQLite", () => {
  const ajv = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(diagnosticSchema);
  const validate = ajv.getSchema(`${diagnosticSchema.$id}#/$defs/TextError`); assert(validate);
  for (const row of fixture.wireCases) assert.equal(validate(row.input), row.accepted, row.id);
  const db = new Database(":memory:"); const query = db.query("SELECT ? || ' at ' || ? || ':' || ? AS display");
  try {
    for (const row of fixture.cases) {
      const expected = "expected" in row.expectedWire ? row.expectedWire.expected : undefined;
      const error = expected === undefined ? new TextError(row.kind as ValueRefusalKind, row.message, row.span) : TextError.expected(row.kind as ValueRefusalKind, row.message, row.span, expected);
      const reference = query.get(row.message, row.span.line, row.span.column) as { display: string };
      assert.equal(error.kind, row.kind, row.id); assert.deepEqual(error.toWire(), row.expectedWire, row.id); assert.equal(error.toString(), reference.display, row.id); assert.equal(error.toString(), row.expectedDisplay, row.id);
    }
  } finally { db.close(); }
});
test("actual TextError source boundary preserves original owned ValueError kind and dotted message", () => {
  for (const row of fixture.cases.filter(row => row.operation === "fromValueError")) {
    const sourceSpan = { ...row.span };
    const error = TextError.fromValueError(new ValueError(row.kind as ValueRefusalKind, row.message), sourceSpan);
    sourceSpan.line = 99;
    assert.equal(error.kind, row.kind); assert.equal(error.message, row.message); assert.deepEqual(error.span, row.span); assert.deepEqual(error.toWire(), row.expectedWire); assert.equal(error.toString(), row.expectedDisplay);
  }
  console.log("[DEBUG] TextError actual portable boundary retains eight kinds, exact span and dotted message without prose classification");
});
