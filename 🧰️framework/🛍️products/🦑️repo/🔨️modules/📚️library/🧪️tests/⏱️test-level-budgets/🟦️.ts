import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { TEST_LEVEL_BUDGET_MS, PACKAGE_TEST_BUDGET_MS, packageTestBudgetMs } from "../../📦️packages/🟦️typescript/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️test-level-budgets/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/⏱️test-level-budgets/🔣️.json", import.meta.url), "utf8"));

/** 🧼️ Schema-first test-level budgets stay synchronized with [[TEST_LEVEL_BUDGET_MS]] and package overrides. */
test("test-level budgets fixture matches schema and TS levels", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  const prev = process.env.SEMIO_TEST_BUDGET_MS;
  delete process.env.SEMIO_TEST_BUDGET_MS;
  try {
    expect(TEST_LEVEL_BUDGET_MS).toEqual(fixture.levels);
    expect(PACKAGE_TEST_BUDGET_MS).toEqual(fixture.packages);
    expect(packageTestBudgetMs(["semio-s-plugin-writer"], "quick")).toBe(fixture.packages["semio-s-plugin-writer"].quick);
    expect(packageTestBudgetMs(["semio-s-plugin-unknown"], "quick")).toBe(fixture.levels.quick);
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    expect(packageTestBudgetMs(["semio-s-plugin-writer"], "quick")).toBe(42000);
  } finally {
    if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
    else process.env.SEMIO_TEST_BUDGET_MS = prev;
  }
});
