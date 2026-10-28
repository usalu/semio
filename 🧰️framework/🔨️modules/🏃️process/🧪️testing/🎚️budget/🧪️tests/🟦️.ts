import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import ts from "typescript";
import { TEST_LEVELS, TEST_LEVEL_BUDGET_MS } from "../🟦️.ts";
import { testPythonV1 } from "../../🧰️toolchain/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("neutral test vocabulary preserves portable levels and exact budgets", () => {
  expect<readonly string[]>(TEST_LEVELS).toEqual(Object.keys(fixture.levels));
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
      const observed = { ...selected, budgetMs: api.testLevelBudgetMs(), rank: api.testLevelRank(), coverage: process.env.SEMIO_COVERAGE ?? null };
      expect(observed).toEqual<typeof row.expected>(row.expected);
      const independent = row.segments[0] in fixture.levels ? row.segments[0] : (row.environment as Record<string, string>).SEMIO_TEST_LEVEL;
      const ranks = Object.keys(fixture.levels);
      const level = ranks[Math.max(Math.max(0, ranks.indexOf(independent ?? "")), ranks.indexOf(row.minimum))]!;
      expect<string>(selected.level).toBe(level);
      expect(api.testLevelAtLeast("long")).toBe(ranks.indexOf(level) >= ranks.indexOf("long"));
      expect(api.atTestLevel({ runIf: (active: boolean) => active }, "quick")).toBe(ranks.indexOf(level) >= ranks.indexOf("quick"));
    }
  } finally { for (const key of keys) { if (before[key] === undefined) delete process.env[key]; else process.env[key] = before[key]; } }
});

test("suites that start processes declare a level whose budget covers a contended run", async () => {
  const table = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🗂️suites/🔣️.json"), "utf8")) as { suites: { id: string; budgetMs: number }[] };
  const declared = new Map(table.suites.map(row => [row.id, row.budgetMs]));
  for (const row of fixture.processSuites) {
    expect(declared.has(row.suite), row.suite).toBe(true);
    expect(declared.get(row.suite)!, `${row.suite}: ${row.reason}`).toBeGreaterThanOrEqual(fixture.levels[row.minimum as keyof typeof fixture.levels]);
  }
  const python = testPythonV1();
  if(!python)throw Error("Independent Python oracle required");
  const oracle = "import json,sys\ntable=json.load(open(sys.argv[2],encoding='utf8'));budget={row['id']:row['budgetMs'] for row in table['suites']};print(json.dumps({name:budget[name] for name in json.loads(sys.argv[1])}))";
  const answer = Bun.spawnSync([python, "-c", oracle, JSON.stringify(fixture.processSuites.map(row => row.suite)), resolve(import.meta.dir, "../../🗂️suites/🔣️.json")], { stdout: "pipe", stderr: "pipe" });
  expect(answer.exitCode, answer.stderr.toString()).toBe(0);
  expect(JSON.parse(answer.stdout.toString())).toEqual(Object.fromEntries(fixture.processSuites.map(row => [row.suite, declared.get(row.suite)])));
});
