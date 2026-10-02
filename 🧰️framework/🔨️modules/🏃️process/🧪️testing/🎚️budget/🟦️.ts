/** ⏱️ Owns ordered test levels and their portable wall-clock budgets. */
export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;

export type TestLevel = (typeof TEST_LEVELS)[number];

export const TEST_LEVEL_BUDGET_MS: Record<TestLevel, number> = {
  fundamental: 15_000,
  quick: 300_000,
  long: 900_000,
  exhaustive: 1_800_000,
};

export function isTestLevel(value: string | undefined): value is TestLevel {
  return !!value && (TEST_LEVELS as readonly string[]).includes(value);
}

/** ⏱️Reads the active test level (`SEMIO_TEST_LEVEL`, defaulting to `fundamental`) — set by [[resolveTestLevel]]. */
export function activeTestLevel(): TestLevel {
  return isTestLevel(process.env.SEMIO_TEST_LEVEL) ? (process.env.SEMIO_TEST_LEVEL as TestLevel) : "fundamental";
}

/**
 * 🎚️Resolves the test level from `segments[0]` (if it names a level) or `SEMIO_TEST_LEVEL`, else `fundamental`.
 * `minimum` is the floor a suite declares when its own fixed cost (independent oracles, generated-bundle
 * renders, taxonomy loads) already exceeds a lower level's budget, so the suite is levelled honestly
 * instead of being killed at every invocation. Sets `process.env.SEMIO_TEST_LEVEL` so every child process
 * spawned afterwards (vitest, cargo, go, pytest, dotnet) inherits it without explicit plumbing.
 * Returns the remaining segments.
 */
export function resolveTestLevel(segments: string[], minimum: TestLevel = "fundamental"): { level: TestLevel; rest: string[] } {
  const [first, ...restIfLevel] = segments;
  const requested = isTestLevel(first) ? first : activeTestLevel();
  const level = testLevelRank(requested) >= testLevelRank(minimum) ? requested : minimum;
  process.env.SEMIO_TEST_LEVEL = level;
  if (level === "exhaustive" && process.env.SEMIO_COVERAGE === undefined) process.env.SEMIO_COVERAGE = "1";
  return { level, rest: isTestLevel(first) ? restIfLevel : segments };
}

/** 🎚️Numeric rank of a test level (0=fundamental..3=exhaustive), for `if (testLevelRank() >= testLevelRank("long"))`-style gating in test files. */
export function testLevelRank(level: string | undefined = process.env.SEMIO_TEST_LEVEL): number {
  const idx = TEST_LEVELS.indexOf((isTestLevel(level) ? level : "fundamental") as TestLevel);
  return idx === -1 ? 0 : idx;
}

/** 🎚️True when the active level reaches `level` — the predicate behind level-gated test cases and level-gated `includeSource` entries. */
export function testLevelAtLeast(level: TestLevel): boolean {
  return testLevelRank() >= testLevelRank(level);
}

/**
 * 🎚️Level-gates one Vitest case factory: `atTestLevel(it, "long")` runs the case from `long` upwards and
 * reports it as skipped below that, so a case that outgrows its level's wall-clock budget moves level
 * instead of being deleted or silently killed. Structurally typed on `runIf` so this library never
 * depends on Vitest's own types — and typed by what `runIf` RETURNS rather than by the factory itself,
 * because Vitest's `TestAPI.runIf` yields the chainable API, not another `TestAPI`.
 */
export function atTestLevel<Gated>(factory: { runIf(condition: boolean): Gated }, level: TestLevel): Gated {
  return factory.runIf(testLevelAtLeast(level));
}

/** ⏱️Wall-clock budget (ms) for the given test level — `SEMIO_TEST_BUDGET_MS` override, else [[TEST_LEVEL_BUDGET_MS]]. */
export function testLevelBudgetMs(level: TestLevel = activeTestLevel()): number {
  return Number(process.env.SEMIO_TEST_BUDGET_MS ?? TEST_LEVEL_BUDGET_MS[level]);
}

/** ⏱️Wall-clock budget (seconds, rounded up) for the given test level — for toolchains that take second-granularity timeouts. */
export function testLevelBudgetSeconds(level: TestLevel = activeTestLevel()): number {
  return Math.ceil(testLevelBudgetMs(level) / 1000);
}

