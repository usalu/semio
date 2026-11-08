import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "./🔣️pop-radix.json";
import schema from "./🧬️pop-radix.json";

test("native pop Work includes every radix digit and occupied level", () => {
  expect(new Ajv({ strict: true }).compile(schema)(law)).toBe(true);
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE paths(length INTEGER, height INTEGER, expected INTEGER)");
    for (const row of law.cases) db.run("INSERT INTO paths VALUES (?, ?, ?)", [row.length, row.height, row.work]);
    const rows = db.query("WITH RECURSIVE digits(length,height,expected,page,n) AS (SELECT length,height,expected,length-1,0 FROM paths UNION ALL SELECT length,height,expected,page/16,n+1 FROM digits WHERE page>0) SELECT length,3+height+n AS actual,expected FROM digits WHERE page=0 ORDER BY length").all() as { actual: number; expected: number }[];
    for (const row of rows) expect(row.actual).toBe(row.expected);
  } finally { db.close(); }
});
