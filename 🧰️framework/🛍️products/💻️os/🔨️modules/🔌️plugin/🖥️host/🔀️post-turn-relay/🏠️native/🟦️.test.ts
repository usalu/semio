/** 🪙️ Settlement runs only after source claims have been checked in their original order. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import corpus from "./escrow-capture.json";

test("every claim is committed before ledger and journal settlement", () => {
  const db = new Database(":memory:");
  try {
    const order = db.query(`
      WITH RECURSIVE
        claim(position) AS (
          SELECT 1 WHERE ?1 > 0
          UNION ALL
          SELECT position + 1 FROM claim WHERE position < ?1
        ),
        steps(group_index, ordinal, name) AS (
          SELECT 0, key, value FROM json_each('["Copy","Header","Sre","ReceiptOrigin"]')
          UNION ALL
          SELECT position, key, value
            FROM claim, json_each('["ClaimOrigin","StageClaim","ClaimSlot","VerifyClaim","ClaimDelta","CommitClaim"]')
          UNION ALL
          SELECT ?1 + 1, key, value FROM json_each('["Coverage","OriginDelta","Purpose","Scope","Bootstrap","Ledger","Journal"]')
        )
      SELECT name FROM steps ORDER BY group_index, ordinal
    `);
    for (const row of corpus.cases) {
      expect(order.all(row.rows).map((step) => (step as { name: string }).name)).toEqual(row.steps);
    }
  } finally {
    db.close();
  }
});
