import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧊️ Checks GLTF-owned semantic identity policy against independent Ajv. */
export function runDefinitionChecks(): number {
  const load = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const schema = load("../../🧬️schema/🔣️.json");
  const fixture = load("../../🧫️fixtures/📜️definition/🔣️.json");
  const ajv = new Ajv({ strict: true });
  const cases = ajv.compile(schema.$defs.DefinitionConstraintCases);
  assert.equal(cases(fixture), true, JSON.stringify(cases.errors));
  const validate = ajv.compile(schema);
  for (const row of fixture.cases) {
    const source = load("../../📜️artifact-definition.json");
    source[row.category][0].id = row.identity;
    assert.equal(validate(source), row.accepted, row.id);
  }
  return fixture.cases.length + 1;
}
