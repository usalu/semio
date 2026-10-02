import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️retirement/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/♻️retirement/🔣️.json", import.meta.url), "utf8"));
const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
test("closed recursive retirement wire corpus has independent AJV and owned schema parity", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  expect(new Set(fixture.cases.map((row: { id: string }) => row.id)).size).toBe(10);
  for (const invalid of [{ ...fixture, extra: true }, { ...fixture, cases: fixture.cases.map((row: object) => ({ ...row, extra: true })) }]) {
    expect(validate(invalid)).toBe(false);
    expect(validateJsonSchemaSubset(schema, invalid).length).toBeGreaterThan(0);
  }
});
test("actual recursive PDF owners explicitly own iterative child retirement", () => {
  for (const owner of ["color_space", "function", "action", "outline_item", "form_field"]) {
    expect(source.includes(`retire_with="retire_pdf_${owner}"`)).toBe(true);
    expect(source.includes(`fn retire_pdf_${owner}`)).toBe(true);
  }
});
