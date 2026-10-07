import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "bun:test";
import Ajv from "ajv";

type FragmentCase = {
  readonly id: string;
  readonly operation: "set" | "insert" | "remove";
  readonly path: readonly (string | number)[];
  readonly candidate?: unknown;
  readonly arrayLengthBefore?: number;
  readonly accepted: boolean;
};

type FragmentFixture = {
  readonly input: { readonly schema: Record<string, unknown> };
  readonly payloadRepeatBytes: number;
  readonly base: Record<string, unknown>;
  readonly cases: readonly FragmentCase[];
};

const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "🧫️fixtures", "🩹️fragment-validation-vectors.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as FragmentFixture;
const AjvConstructor = (Ajv as unknown as { readonly default?: typeof Ajv }).default ?? Ajv;

const apply = (source: Record<string, unknown>, testCase: FragmentCase): Record<string, unknown> => {
  const candidate = source;
  let parent: unknown = candidate;
  for (const segment of testCase.path.slice(0, -1)) parent = (parent as Record<string | number, unknown>)[segment];
  const key = testCase.path.at(-1);
  if (key == null) throw new Error("fragment path must address one value");
  if (testCase.operation === "remove") {
    if (Array.isArray(parent)) parent.splice(Number(key), 1);
    else delete (parent as Record<string, unknown>)[String(key)];
  } else if (testCase.operation === "insert" && Array.isArray(parent)) {
    parent.splice(Number(key), 0, testCase.candidate);
  } else {
    (parent as Record<string | number, unknown>)[key] = testCase.candidate;
  }
  return candidate;
};

describe("compact snapshot fragment validation oracle", () => {
  it("matches AJV while preserving the unrelated large payload", () => {
    const ajv = new AjvConstructor({ allErrors: true, strict: false });
    const validate = ajv.compile(fixture.input.schema);
    for (const testCase of fixture.cases) {
      const base = {
        ...structuredClone(fixture.base),
        bytes: testCase.arrayLengthBefore == null ? structuredClone(fixture.base.bytes) : Array.from({ length: testCase.arrayLengthBefore }, () => 0),
        payload: "x".repeat(fixture.payloadRepeatBytes),
      };
      const candidate = apply(base, testCase);
      expect(validate(candidate), `${testCase.id}: ${ajv.errorsText(validate.errors)}`).toBe(testCase.accepted);
      expect((candidate.payload as string).length).toBe(fixture.payloadRepeatBytes);
    }
  });
});
