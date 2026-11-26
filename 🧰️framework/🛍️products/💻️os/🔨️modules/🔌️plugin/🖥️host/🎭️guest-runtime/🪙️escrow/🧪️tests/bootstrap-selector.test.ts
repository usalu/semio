import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import cases from "./bootstrap-selector.json";

test("only a valid actor-context SAB1 header takes the bootstrap route", () => {
  const db = new Database(":memory:");
  try {
    const classify = db.query<{ outcome: string }, [string, Uint8Array]>(`
      SELECT CASE
        WHEN length(?2) <> 304 THEN 'escrow'
        WHEN ?1 IN ('admitActorContext', 'closeActorContext')
          AND substr(?2, 1, 8) = X'5341423101003001' THEN 'bootstrap'
        ELSE 'refused'
      END AS outcome
    `);
    for (const row of cases) {
      const header = new Uint8Array(row.length);
      header.set(row.prefix);
      expect(classify.get(row.exportName, header)?.outcome).toBe(row.outcome);
    }
  } finally {
    db.close();
  }
});
