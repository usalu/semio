import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("original vector issuer and refusal work follow the independent transition model", () => {
  const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
  const db = new Database(":memory:");
  const rows = db.query(`SELECT work FROM (
    SELECT 1 AS ordinal, 1 AS work UNION ALL
    SELECT 2, 1 UNION ALL
    SELECT 3, CASE WHEN ? THEN 2 ELSE 4 END UNION ALL
    SELECT 4, 4
  ) ORDER BY ordinal`).all(fixture.refuseFirst ? 1 : 0) as { work: number }[];
  expect(rows.map(row => row.work)).toEqual(fixture.initialWork);
  db.close();
});
