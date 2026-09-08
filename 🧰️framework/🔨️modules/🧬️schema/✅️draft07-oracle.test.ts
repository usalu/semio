import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";

/** 🧪️ Third-party draft-07 oracle for the owned Rust validator: the same
 * `🧪️fixtures/✅️draft07-validation-vectors.json` corpus that
 * `owned_validator_agrees_with_the_shared_draft07_vectors` drives must produce the same verdicts in
 * ajv. Ajv is a test-only oracle — no production code in this repo may depend on it.
 * @see https://ajv.js.org/json-schema.html */
type Vector = {
  readonly id: string;
  readonly schema: Record<string, unknown>;
  readonly documents?: readonly Record<string, unknown>[];
  readonly valid: readonly unknown[];
  readonly invalid: readonly { readonly instance: unknown; readonly errorPath: string }[];
};

const vectorsPath = join(dirname(fileURLToPath(import.meta.url)), "🧪️fixtures", "✅️draft07-validation-vectors.json");
const vectors = JSON.parse(readFileSync(vectorsPath, "utf8")) as { readonly dialect: string; readonly cases: readonly Vector[] };
const ajvConstructor = (Ajv as unknown as { readonly default?: typeof Ajv }).default ?? Ajv;

const isAncestorOrSelf = (reported: string, expected: string): boolean => expected === reported || expected.startsWith(`${reported}/`);

describe("draft-07 structural validation vectors", () => {
  it("declares the draft-07 dialect on every case", () => {
    expect(vectors.dialect).toBe("http://json-schema.org/draft-07/schema#");
    expect(vectors.cases.length).toBeGreaterThanOrEqual(7);
    for (const testCase of vectors.cases) expect(testCase.schema.$schema).toBe(vectors.dialect);
  });

  for (const testCase of vectors.cases) {
    it(`agrees with ajv on ${testCase.id}`, () => {
      const ajv = new ajvConstructor({ allErrors: true, strict: false });
      for (const document of testCase.documents ?? []) ajv.addSchema(document);
      const validate = ajv.compile(testCase.schema);

      for (const instance of testCase.valid) {
        expect(validate(instance), `${testCase.id}: ${JSON.stringify(instance)} → ${ajv.errorsText(validate.errors)}`).toBe(true);
      }

      for (const { instance, errorPath } of testCase.invalid) {
        expect(validate(instance), `${testCase.id}: expected ${JSON.stringify(instance)} to be rejected`).toBe(false);
        const reported = (validate.errors ?? []).map((error) => error.instancePath);
        expect(reported.some((path) => isAncestorOrSelf(path, errorPath)), `${testCase.id}: expected an error at or above ${errorPath || "/"}, got ${JSON.stringify(reported)}`).toBe(true);
      }
    });
  }
});
