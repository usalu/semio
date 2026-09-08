/** 🧪️ Validates explicit host preferences against the shared neutral schema and Ajv. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseResolvedPluginViewState } from "../../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

export function testResolvedHostContext(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  const actual = parseResolvedPluginViewState(fixture.valid);
  assert.deepEqual(actual, fixture.valid);
  assert.notEqual(actual, fixture.valid);
  for (const row of fixture.invalid) {
    const value: Record<string, unknown> = structuredClone(fixture.valid);
    if ("remove" in row) for (const key of row.remove) delete value[key];
    if ("set" in row) Object.assign(value, row.set);
    assert.equal(validate(value), false, row.name);
    assert.throws(() => parseResolvedPluginViewState(value), undefined, row.name);
  }
  console.log(`resolved-host-context cases=${fixture.invalid.length + 1} schema=valid explicit-preferences=required`);
}
