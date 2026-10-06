import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { TEST_LEVEL_BUDGET_MS } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { PACKAGE_TEST_BUDGET_MS, packageTestBudgetMs } from "../../📦️packages/🟦️typescript/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️test-level-budgets/🔣️.json", import.meta.url), "utf8"));

const levels = JSON.parse(readFileSync(new URL("../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🧫️fixtures/🔣️.json", import.meta.url), "utf8")).levels;

/** 🧼️ Schema-first test-level budgets stay synchronized with [[TEST_LEVEL_BUDGET_MS]] and package overrides. */
test("test-level budgets fixture matches schema and TS levels", () => {
  expect(fixture["schema"]).toEqual("semio.repo.library.test-level-budgets/v1");
  const prev = process.env.SEMIO_TEST_BUDGET_MS;
  delete process.env.SEMIO_TEST_BUDGET_MS;
  try {
    expect(TEST_LEVEL_BUDGET_MS).toEqual(levels);
    expect(PACKAGE_TEST_BUDGET_MS).toEqual(fixture.packages);
    expect(packageTestBudgetMs(["semio-hub-writer"], "quick")).toBe(fixture.packages["semio-hub-writer"].quick);
    expect(packageTestBudgetMs(["semio-s-plugin-unknown"], "quick")).toBe(levels.quick);
    process.env.SEMIO_TEST_BUDGET_MS = "42000";
    expect(packageTestBudgetMs(["semio-hub-writer"], "quick")).toBe(42000);
  } finally {
    if (prev === undefined) delete process.env.SEMIO_TEST_BUDGET_MS;
    else process.env.SEMIO_TEST_BUDGET_MS = prev;
  }
});
