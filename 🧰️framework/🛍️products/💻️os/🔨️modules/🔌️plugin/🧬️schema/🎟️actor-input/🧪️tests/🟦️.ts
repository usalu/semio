import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const owner = resolve(import.meta.dirname, "..");
const schema = JSON.parse(readFileSync(owner + "/🧬️schema/🔣️.json", "utf8"));
const corpus = JSON.parse(readFileSync(owner + "/🧫️fixtures/🔣️.json", "utf8")) as { valid: { operation: string; wire: unknown }[]; invalid: { reason: string; wire: unknown }[] };
const oracle = new Ajv({ strict: false }).compile(schema);

test("every authored wire is admitted by the independent Ajv oracle and the first-party validator", () => {
  for (const { wire } of corpus.valid) {
    expect(oracle(wire)).toBe(true);
    expect(validateJsonSchemaSubset(schema, wire).length).toBe(0);
  }
});

test("every refused wire is refused by both validators", () => {
  for (const { wire } of corpus.invalid) {
    expect(oracle(wire)).toBe(false);
    expect(validateJsonSchemaSubset(schema, wire).length).toBeGreaterThan(0);
  }
});

test("the schema names exactly the operations the Rust wire enum declares", () => {
  const rust = readFileSync(owner + "/🦀️.rs", "utf8");
  const declared = [...rust.matchAll(/Self::\w+ \{ \.\. \} => "([a-z-]+)"/g)].map((match) => match[1]).sort();
  const authored = schema.oneOf.map((entry: { $ref: string }) => schema.$defs[entry.$ref.split("/").pop()!].properties.operation.const).sort();
  expect(authored).toEqual(declared);
  expect([...new Set(corpus.valid.map((entry) => entry.operation))].sort()).toEqual(declared);
});
