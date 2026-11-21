import Ajv from "ajv";
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import law from "./🧫️.json";
import schema from "./🧬️.json";

test("a full escrow charges one reclaim and one guard birth only for acknowledged rows", () => {
  expect(new Ajv().compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  try {
    const oracle = db.query("SELECT ?1 > 0 AS admitted, 2 * (?1 > 0) AS chargedItems, 2 * (?1 > 0) AS chargedWork");
    for (const { acknowledged, admitted, chargedItems, chargedWork } of law.cases) {
      expect(acknowledged).toBeLessThanOrEqual(law.cells);
      expect(oracle.get(acknowledged)).toEqual({ admitted: Number(admitted), chargedItems, chargedWork });
    }
  } finally {
    db.close();
  }
});
