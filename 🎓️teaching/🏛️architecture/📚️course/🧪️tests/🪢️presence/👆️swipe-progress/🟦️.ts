import { expect, it } from "vitest";
import { DatabaseSync } from "node:sqlite";
import vectors from "../../../🧫️fixtures/👆️swipe-progress/🔣️.json";
import { swipeAttemptLimit, swipeNavigationProgress } from "../../../🎭️e2e/🚶️learner/👆️progress/🟦️.ts";

it("swipe destination progress preserves real scroll priority and refuses unproved advancement", () => {
  const database = new DatabaseSync(":memory:");
  try {
    expect(new Set(vectors.cases.map(row => row.id)).size).toBe(vectors.cases.length);
    for (const { id, expected, ...value } of vectors.cases) {
      const oracle = database.prepare("SELECT CASE WHEN min(?,?,?,?)<0 THEN 'refuse' WHEN ?=? THEN 'advance' WHEN ? IS NULL THEN 'pending' WHEN ?<>? THEN 'refuse' WHEN ?>0 AND ?-?>=min(1,?) AND ?-?>=min(1,?) THEN 'scroll' ELSE 'pending' END AS outcome").get(value.before.position,value.before.remaining,value.after.position,value.after.remaining,value.observed,value.next,value.observed,value.observed,value.from,value.before.remaining,value.after.position,value.before.position,value.before.remaining,value.before.remaining,value.after.remaining,value.before.remaining) as { outcome: string };
      let actual: string;
      try { actual = swipeNavigationProgress(value); } catch { actual = "refuse"; }
      expect(actual, id).toBe(expected);
      expect(actual, id).toBe(oracle.outcome);
    }
    for (const row of vectors.bounds) expect(swipeAttemptLimit(row.range)).toBe(row.attempts);
    expect(() => swipeAttemptLimit(-1)).toThrow();
    expect(() => swipeAttemptLimit(Infinity)).toThrow();
    expect(() => swipeNavigationProgress({from:"learner",next:"badges",observed:"learner",before:{position:NaN,remaining:0},after:{position:0,remaining:0}})).toThrow();
  } finally {
    database.close();
  }
});
