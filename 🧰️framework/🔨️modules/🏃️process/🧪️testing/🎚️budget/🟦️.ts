/** ⏱️ Owns ordered test levels and their portable wall-clock budgets. */
export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;

export type TestLevel = (typeof TEST_LEVELS)[number];

export const TEST_LEVEL_BUDGET_MS: Record<TestLevel, number> = {
  fundamental: 15_000,
  quick: 300_000,
  long: 900_000,
  exhaustive: 1_800_000,
};
