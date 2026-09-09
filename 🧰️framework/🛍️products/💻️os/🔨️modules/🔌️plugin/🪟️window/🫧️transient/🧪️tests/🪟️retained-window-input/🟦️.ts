import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧪️ Checks the neutral retained window input identity contract independently. */
export function testRetainedWindowInputOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
  const schema = { type: "array", uniqueItems: true, items: { type: "object", additionalProperties: false, required: ["windowId", "windowKindId", "generation"], properties: { windowId: { type: "string", minLength: 1 }, windowKindId: { type: "string", minLength: 1 }, generation: { type: "integer", minimum: 0 } } } };
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture.cases), JSON.stringify(validate.errors));
  const identities = new Set(fixture.cases.map((row: { windowId: string; windowKindId: string; generation: number }) => JSON.stringify([row.windowId, row.windowKindId, row.generation])));
  assert.equal(identities.size, fixture.expectedUniqueOwners);
  assert.equal(validate([...fixture.cases, fixture.cases[0]]), false);
  console.log(`[DEBUG] retained window input: ${identities.size} distinct owner/generation tuples agree with Ajv uniqueItems`);
}
