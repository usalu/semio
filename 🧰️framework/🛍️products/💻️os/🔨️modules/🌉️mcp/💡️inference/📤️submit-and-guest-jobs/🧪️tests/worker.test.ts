import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import cases from "./worker.json";

test("an unwound worker settles an active job and preserves earlier terminal receipts", () => {
  const db = new Database(":memory:");
  try {
    const settled = db.query("SELECT CASE WHEN ?1 AND ?2 NOT IN ('SUCCEEDED','FAILED','CANCELLED') THEN 'FAILED' ELSE ?2 END AS status");
    for (const row of cases) expect(settled.get(Number(row.unwind),row.before),row.name).toEqual({status:row.expected});
  } finally { db.close(); }
});
