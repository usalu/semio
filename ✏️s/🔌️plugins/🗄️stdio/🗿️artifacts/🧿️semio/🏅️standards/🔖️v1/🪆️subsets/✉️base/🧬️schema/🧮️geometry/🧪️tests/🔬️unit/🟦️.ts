import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import * as geometry from "../../🟦️.ts";

/** 🧫️ Shared geometry admission agrees with an independent JSON Schema validator. */
export function testSemioGeometryContract(): void {
  const schema = JSON.parse(readFileSync(new URL("../../🔣️.json", import.meta.url), "utf8"));
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  for (const entry of fixture.cases as { type: string; input: unknown; valid: boolean }[]) {
    const validate = ajv.compile({ $ref: schema.$id + "#/$defs/" + entry.type });
    assert.equal(validate(entry.input), entry.valid, entry.type + ": schema oracle");
    const parser = (geometry as Record<string, (input: unknown) => unknown>)["parse" + entry.type];
    assert.equal(typeof parser, "function", entry.type + ": production parser");
    if (entry.valid) assert.deepEqual(parser!(entry.input), entry.input, entry.type);
    else assert.throws(() => parser!(entry.input), entry.type);
  }
  console.log("[DEBUG] Shared Semio geometry contracts agree with Ajv for " + fixture.cases.length + " neutral vectors");
}
