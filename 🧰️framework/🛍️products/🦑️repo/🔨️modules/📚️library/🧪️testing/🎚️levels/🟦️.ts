import { activeTestLevel, TEST_LEVELS, type TestLevel, testLevelBudgetMs, testLevelBudgetSeconds } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

function levelsAbove(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(TEST_LEVELS.indexOf(level) + 1);
}

/** ⏱️Cumulative `go test` args for the active level: per-level `-timeout`, keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
export function goLevelTestArgs(level: TestLevel = activeTestLevel()): string[] {
  const args: string[] = ["-timeout", `${testLevelBudgetSeconds(level)}s`];
  if (TEST_LEVELS.indexOf(level) <= TEST_LEVELS.indexOf("quick")) args.push("-short");
  const skipped = levelsAbove(level).map((l) => l[0]!.toUpperCase() + l.slice(1));
  if (skipped.length) args.push("-skip", `^Test(${skipped.join("|")})`);
  return args;
}

/** ⏱️Vitest per-test/hook/teardown timeouts (ms) for the active level budget. */
export function vitestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const ms = String(testLevelBudgetMs(level));
  return ["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms];
}

/** ⏱️`bun test` `--timeout` (ms) for the active level budget. */
export function bunTestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  return ["--timeout", String(testLevelBudgetMs(level))];
}

/** ⏱️Cumulative pytest args for the active level: `-m` marker filter, per-test `--timeout`, and `--timeout-method=thread`. */
export function pytestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const excluded = levelsAbove(level);
  const args = excluded.length ? ["-m", excluded.map((l) => `not ${l}`).join(" and ")] : [];
  args.push(`--timeout=${testLevelBudgetSeconds(level)}`, "--timeout-method=thread");
  return args;
}

/** ⏱️Cumulative dotnet xunit args for the active level: `--filter` for levels above it and `--blame-hang` with a per-test timeout (ms). */
export function dotnetLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const excluded = levelsAbove(level);
  const args = excluded.length ? ["--filter", excluded.map((l) => `Category!=${l}`).join("&")] : [];
  args.push("--blame-hang", "--blame-hang-timeout", String(testLevelBudgetMs(level)));
  return args;
}

/** ⏱️Playwright per-spec timeout (ms) — active level budget via [[testLevelBudgetMs]]. */
export function playwrightTestTimeoutMs(level: TestLevel = activeTestLevel()): number {
  return testLevelBudgetMs(level);
}
