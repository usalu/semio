import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import law from "./🧫️fixtures/vm-refinement.json";
import schema from "./🧫️fixtures/vm-refinement.schema.json";

test("original VM credit preserves its issue and charges fresh work once", () => {
 expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
 const db = new Database(":memory:");
 try {
  const refine = db.query(`
   WITH fresh AS (SELECT $performed - $redeemed AS work)
   SELECT CASE WHEN work BETWEEN 0 AND $issued THEN min($observed, $issued - work) END AS remaining
   FROM fresh
  `);
  for (const { name, issued, observed, performed, redeemed, expected } of law) {
   const { remaining } = refine.get({ $issued: issued, $observed: observed, $performed: performed, $redeemed: redeemed }) as { remaining: number | null };
   expect(remaining, name).toBe(expected);
  }
 } finally {
  db.close();
 }
});
