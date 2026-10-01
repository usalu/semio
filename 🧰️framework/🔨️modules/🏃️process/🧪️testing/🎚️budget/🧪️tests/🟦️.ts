import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { TEST_LEVELS, TEST_LEVEL_BUDGET_MS } from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("neutral test vocabulary preserves portable levels and exact budgets", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  expect(TEST_LEVELS).toEqual(Object.keys(fixture.levels));
  expect(TEST_LEVEL_BUDGET_MS).toEqual(fixture.levels);
  const source = readFileSync(resolve(import.meta.dir, "../🟦️.ts"), "utf8");
  const javascript = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
  const exports: Record<string, unknown> = {};
  new Function("exports", javascript)(exports);
  expect(exports.TEST_LEVELS).toEqual(TEST_LEVELS);
  expect(exports.TEST_LEVEL_BUDGET_MS).toEqual(TEST_LEVEL_BUDGET_MS);
  expect(source).not.toContain("🛍️products");
});
