import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

interface Field { readonly id: number; readonly key: string; readonly optional: boolean; readonly labels: readonly { readonly label: string; readonly ordinal: number }[] }
interface Case { readonly factory: string; readonly owner: string; readonly fields: readonly Field[] }
const fixture: { readonly cases: readonly Case[]; readonly genericWitnesses: readonly unknown[] } = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎛️metadata/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🎛️metadata/🔣️.json", import.meta.url), "utf8"));
const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");

test("independent closed metadata descriptors retain all eleven record contracts", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  expect(new Set(fixture.cases.map(row => row.factory)).size).toBe(11);
  expect(fixture.cases.reduce((count, row) => count + row.fields.length, 0)).toBe(112);
  for (const row of fixture.cases) {
    expect(new Set(row.fields.map(field => field.id)).size).toBe(row.fields.length);
    expect(new Set(row.fields.map(field => field.key)).size).toBe(row.fields.length);
    for (const field of row.fields) expect(new Set(field.labels.map(label => label.ordinal)).size).toBe(field.labels.length);
  }
});

test("actual authored metadata has a controlled branch and every handwritten shape owns it", () => {
  for (const row of fixture.cases) {
    expect(source).toContain(`dwg_metadata!(${row.factory}`);
    expect(source).toContain(`${row.factory}_controlled`);
    expect(source).toContain(`${row.factory}_producer`);
    const at = source.indexOf(`dsl::DslField for ${row.owner}`);
    expect(at).toBeGreaterThan(0);
    expect(source.slice(at, source.indexOf("fn to_value(&self)", at))).toContain("fn shape_controlled");
  }
  expect(source).toContain("control.allocate_vec::<dsl::FieldSpec>(COUNT)?");
  expect(source).toContain("<$child as dsl::DslField>::shape_controlled($control)");
  expect(source).toContain("dsl::schema::producer::boxed");
});
