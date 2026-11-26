import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import law from "./🧫️fixtures/capture-batch.json";

test("frozen phases conserve work and copy currency across different funding windows", () => {
  const db = new Database(":memory:");
  try {
    const units = db.query(`
      WITH extents AS (SELECT 48 + 184 * $rows AS escrow),
           phases AS (
             SELECT 'copy' AS name, (152 + escrow) / 8 AS items, 152 + escrow AS bytes,
                    (152 + escrow) / 8 AS pointers, 152 + escrow + (152 + escrow) / 8 AS work FROM extents
             UNION ALL SELECT 'header', 1 + $words + 2, 4, $words + 1, 1 + $words + 2
             UNION ALL SELECT 'sre', 6 + $rows * (23 + 7 + 4), 1 + 5 * 9 + $rows * (23 * 8 + 1 + 1 + 4 * 8 + 2),
                    6 + $rows * (23 + 8 + 6), 6 + $rows * (23 + 7 + 4)
             UNION ALL SELECT 'claim', 1 + 23, 1 + 23 * 8, 2 + 23, 1 + 23
           )
      SELECT * FROM phases ORDER BY CASE name WHEN 'copy' THEN 0 WHEN 'header' THEN 1 WHEN 'sre' THEN 2 ELSE 3 END
    `);
    const measured = units.all({ $rows: law.rows.length, $words: law.headerWords.length }) as {
      name: keyof typeof law.phases; items: number; bytes: number; pointers: number; work: number;
    }[];
    const turns = db.query(`
      WITH w AS (SELECT min($items, 32) AS slots)
      SELECT CASE WHEN $claim THEN 1 + ($actions - 1 + slots - 1) / slots
                  ELSE ($actions + slots - 1) / slots END AS turns FROM w
    `);
    for (const phase of measured) {
      expect([phase.items, phase.bytes, phase.pointers, phase.work]).toEqual(law.phases[phase.name]);
    }
    for (const row of law.schedules) {
      expect(measured.map(({ name, items }) => (turns.get({ $items: row.items, $actions: items, $claim: Number(name === "claim") }) as { turns: number }).turns), row.name).toEqual(row.turns);
    }

    const stopped = db.query(`
      WITH paid AS (
        SELECT CASE $mode WHEN 'copy' THEN min(8, $limit) WHEN 'fuel' THEN min(8, $limit - 1) ELSE 8 * $limit END AS bytes,
               CASE $mode WHEN 'copy' THEN 1 WHEN 'fuel' THEN 1 ELSE $limit END AS items
      ) SELECT bytes, items, bytes + items AS work FROM paid
    `);
    for (const row of law.stops) {
      const actual = stopped.get({ $mode: row.mode, $limit: row.limit }) as { bytes: number; items: number; work: number };
      expect(actual, row.name).toEqual({ bytes: row.bytes, items: row.items, work: row.work });
    }
    const conserve = db.query("SELECT ($used <= $held) AND ($status != 2 OR $used = $held) AS valid");
    for (const row of law.rows) {
      for (let axis = 0; axis < 5; axis++) {
        const result = conserve.get({ $status: row[7], $held: row[8 + axis], $used: row[13 + axis] + row[18 + axis] }) as { valid: number };
        expect(result.valid).toBe(1);
      }
    }
  } finally {
    db.close();
  }
});
