import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import fixture from "./scheduling.json";
import schema from "./scheduling.schema.json";

test("native poll yields only on full bursts while observing every stop", async () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const db = new Database(":memory:");
  try {
    const schedule = db.query(`
      WITH RECURSIVE steps(n) AS (
        VALUES(1) UNION ALL SELECT n + 1 FROM steps WHERE n < ?1
      ), statuses AS (
        SELECT n, CASE
          WHEN n = ?2 THEN 'cancelled'
          WHEN n = ?3 THEN 'complete'
          WHEN n = ?4 THEN 'deadline'
          WHEN n = ?5 THEN 'delivered'
          ELSE 'pending'
        END AS status FROM steps
      ), cutoff AS (
        SELECT COALESCE(MIN(n) FILTER (WHERE status NOT IN ('pending', 'delivered')), ?1) AS n
        FROM statuses
      )
      SELECT COUNT(*) FILTER (WHERE status <> 'cancelled') AS observedSteps,
        JSON_GROUP_ARRAY(statuses.n ORDER BY statuses.n) FILTER (WHERE status IN ('pending', 'delivered') AND statuses.n % ?6 = 0) AS yields,
        MAX(status) FILTER (WHERE statuses.n = cutoff.n) AS ending
      FROM statuses JOIN cutoff ON statuses.n <= cutoff.n`);
    for (const row of fixture.cases) {
      const observed = schedule.get(row.maximumSteps, row.cancelBeforeStep ?? null, row.completeAt ?? null, row.expiresAfterStep ?? null, row.deliverAt ?? null, fixture.stepsPerYield) as { observedSteps: number, yields: string, ending: string };
      expect({ ...observed, yields: JSON.parse(observed.yields) }).toEqual({ observedSteps: row.observedSteps, yields: row.yields, ending: row.ending });
    }
  } finally {
    db.close();
  }
  const source = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  const hotLoop = source.slice(source.indexOf("pub(super) fn poll_turn"), source.indexOf("fn poll_fault"));
  expect(source).toContain(`const POLL_STEPS_PER_YIELD: usize = ${fixture.stepsPerYield};`);
  expect(hotLoop).toMatch(/loop \{\s*scope\.checkpoint\(\)\?;\s*let status = poll\.drive_step/u);
  expect(hotLoop).toContain("GuestPollSessionStatus::Pending | GuestPollSessionStatus::Delivered if std::time::Instant::now() < deadline");
  expect(hotLoop).toMatch(/steps_since_yield \+= 1;\s*if steps_since_yield == POLL_STEPS_PER_YIELD \{\s*std::thread::yield_now\(\);\s*steps_since_yield = 0;/u);
});
