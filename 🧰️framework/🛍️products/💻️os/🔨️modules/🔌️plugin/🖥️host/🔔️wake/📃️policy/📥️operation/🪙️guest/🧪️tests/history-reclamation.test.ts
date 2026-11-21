import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import cases from "./🧫️fixtures/history-reclamation.json";

test("an omitted settled claim yields its slot only after its publication handback", () => {
 const db = new Database(":memory:");
 try {
  const slot = db.query(`
   SELECT CASE
    WHEN $matches AND $reserved THEN 'duplicate'
    WHEN $matches THEN 'existing'
    WHEN $reserved OR $incoming <= $highWater THEN 'unavailable'
    WHEN $status = 0 OR ($status = 2 AND $acknowledged > 0 AND $acknowledged = $prior) THEN 'available'
    ELSE 'unavailable'
   END AS selected
  `);
  for (const row of cases) {
   const result = slot.get({
    $matches: Number(row.matches), $reserved: Number(row.reserved),
    $incoming: row.incoming, $highWater: row.highWater, $status: row.status,
    $acknowledged: row.acknowledged, $prior: row.prior,
   }) as { selected: string };
   expect(result.selected, row.name).toBe(row.expected);
  }
 } finally {
  db.close();
 }
});
