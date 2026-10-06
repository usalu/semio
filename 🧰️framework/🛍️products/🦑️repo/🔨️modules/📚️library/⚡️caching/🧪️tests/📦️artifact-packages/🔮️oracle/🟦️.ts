import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { Validator } from "jsonschema";

const root = resolve(import.meta.dir, "../../..");
const contract = JSON.parse(readFileSync(resolve(root, "📦️artifacts/🧭️package-inventory/🧬️schema/🔣️.json"), "utf8"));
const examples = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/artifact-packages/🔣️.json"), "utf8"));

test("validates actual artifact inventory payloads through two independent schema engines", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(contract);
  const oracle = new Validator();
  const observed = [];
  for (const value of examples.accepted) {
    expect(validate(value), JSON.stringify(validate.errors)).toBe(true);
    expect(oracle.validate(value, contract).valid).toBe(true);
    observed.push(true);
  }
  for (const rejected of examples.rejected) {
    expect(validate(rejected.value), rejected.id).toBe(false);
    expect(oracle.validate(rejected.value, contract).valid, rejected.id).toBe(false);
    observed.push(false);
  }
  expect(observed).toEqual([true, false, false, false]);
  console.log("[DEBUG] artifact inventory payload verdicts=" + JSON.stringify(observed));
});
