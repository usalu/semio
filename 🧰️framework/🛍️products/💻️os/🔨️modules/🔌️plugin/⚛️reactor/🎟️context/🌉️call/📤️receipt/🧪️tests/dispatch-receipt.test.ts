import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import cases from "./🧫️fixtures/dispatch-receipt.json";

test("the fixed receipt observes this dispatch before the issuer separates redemptions", () => {
 const db = new Database(":memory:");
 try {
  const receipt = db.query(`
   WITH receipt(aggregate) AS (
    SELECT CASE WHEN $observed >= $baseline THEN $observed - $baseline + $completion + $carry END
   ), spend(aggregate, fresh) AS (
    SELECT aggregate, CASE WHEN aggregate >= $redeemed THEN aggregate - $redeemed END FROM receipt
   )
   SELECT aggregate, fresh, coalesce(fresh <= $grant, 0) AS accepted FROM spend
  `);
  for (const row of cases) {
   const actual = receipt.get({
    $observed: row.observed, $baseline: row.baseline, $completion: row.completion,
    $carry: row.carry, $redeemed: row.redeemed, $grant: row.grant,
   });
   expect(actual, row.name).toEqual({
    aggregate: row.aggregate, fresh: row.fresh, accepted: Number(row.fresh !== null && row.fresh <= row.grant),
   });
  }
 } finally {
  db.close();
 }
});
