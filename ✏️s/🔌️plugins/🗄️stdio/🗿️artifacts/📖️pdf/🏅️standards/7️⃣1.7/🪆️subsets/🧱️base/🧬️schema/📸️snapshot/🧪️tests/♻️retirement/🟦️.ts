import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️retirement/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/♻️retirement/🔣️.json", import.meta.url), "utf8"));
const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
test("closed recursive retirement wire corpus has independent AJV and owned schema parity", () => {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  const validators = new Map(Object.keys(schema.$defs).map(owner => [owner, ajv.getSchema(`${schema.$id}#/$defs/${owner}`)!]));
  for (const row of fixture.cases) {
    const domain = { ...schema, oneOf: undefined, $ref: `#/$defs/${row.owner}` };
    expect(validators.get(row.owner)!(row.wire)).toBe(true);
    expect(validateJsonSchemaSubset(domain, row.wire)).toEqual([]);
    expect(validators.get(row.owner)!({ ...row.wire, extra: true })).toBe(false);
    expect(validateJsonSchemaSubset(domain, { ...row.wire, extra: true }).length).toBeGreaterThan(0);
  }
  expect(new Set(fixture.cases.map((row: { id: string }) => row.id)).size).toBe(10);
});
test("actual recursive PDF owners explicitly own iterative child retirement", () => {
  for (const owner of ["color_space", "function", "action", "outline_item", "form_field"]) {
    expect(source.includes(`retire_with="retire_pdf_${owner}"`)).toBe(true);
    expect(source.includes(`fn retire_pdf_${owner}`)).toBe(true);
  }
});
