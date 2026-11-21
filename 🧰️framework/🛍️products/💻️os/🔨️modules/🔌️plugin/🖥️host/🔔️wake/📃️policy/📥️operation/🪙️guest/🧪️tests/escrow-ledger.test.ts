import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import law from "./🧫️fixtures/escrow-ledger.json";
import schema from "./🧫️fixtures/escrow-ledger.schema.json";

test("each escrow axis conserves purpose stock through settlement and retirement", () => {
 expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
 const db = new Database(":memory:");
 try {
  const settle = db.query(`
   SELECT $held + $reserve - $redeem - $release AS held,
          $observed + $redeem AS observed,
          $held + $observed + $forfeited + $reserve - $release + $fresh AS total
  `);
  for (const row of law.settlements) {
   const balances = law.axes.map((_, index) => settle.get({
    $held: row.held[index], $observed: row.observed[index], $forfeited: row.forfeited[index],
    $reserve: row.reserve[index], $redeem: row.redeem[index], $release: row.release[index],
    $fresh: row.fresh[index],
   }) as { held: number; observed: number; total: number });
   const actual = balances.some(balance => balance.held < 0) ? null : {
    held: balances.map(balance => balance.held),
    observed: balances.map(balance => balance.observed),
    total: balances.map(balance => balance.total),
   };
   expect(actual, row.name).toEqual(row.expected);
  }

  const forfeit = db.query("SELECT CASE WHEN ? = ? THEN ? + ? END AS balance");
  for (const row of law.retirements) {
   const balances = law.axes.map((_, i) =>
    (forfeit.get(row.held[i], row.journal[i], row.forfeited[i], row.held[i]) as { balance: number | null }).balance);
   expect(balances.includes(null) ? null : balances, row.name).toEqual(row.expected);
  }
 } finally {
  db.close();
 }
});
