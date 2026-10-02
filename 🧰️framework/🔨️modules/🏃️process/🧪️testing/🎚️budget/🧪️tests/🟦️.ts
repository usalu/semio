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


test("neutral level selection follows portable CLI and environment laws", async () => {
  const api = await import("../🟦️.ts");
  const keys = ["SEMIO_TEST_LEVEL", "SEMIO_TEST_BUDGET_MS", "SEMIO_COVERAGE"];
  const before = Object.fromEntries(keys.map(key => [key, process.env[key]]));
  try {
    for (const row of fixture.cases) {
      for (const key of keys) delete process.env[key];
      Object.assign(process.env, row.environment);
      const selected = api.resolveTestLevel(row.segments, row.minimum as typeof TEST_LEVELS[number]);
      expect({ ...selected, budgetMs: api.testLevelBudgetMs(), rank: api.testLevelRank(), coverage: process.env.SEMIO_COVERAGE ?? null }).toEqual(row.expected);
      const independent = row.segments[0] in fixture.levels ? row.segments[0] : (row.environment as Record<string, string>).SEMIO_TEST_LEVEL;
      const ranks = Object.keys(fixture.levels);
      const level = ranks[Math.max(Math.max(0, ranks.indexOf(independent ?? "")), ranks.indexOf(row.minimum))]!;
      expect(selected.level).toBe(level);
      expect(api.testLevelAtLeast("long")).toBe(ranks.indexOf(level) >= ranks.indexOf("long"));
      expect(api.atTestLevel({ runIf: (active: boolean) => active }, "quick")).toBe(ranks.indexOf(level) >= ranks.indexOf("quick"));
    }
  } finally { for (const key of keys) { if (before[key] === undefined) delete process.env[key]; else process.env[key] = before[key]; } }
});
