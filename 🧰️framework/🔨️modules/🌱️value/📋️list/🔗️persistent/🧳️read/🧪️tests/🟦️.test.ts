import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";

test("original persistent row order and resident recovery laws", () => {
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try {
  for(const row of law.cases){
   const actual=db.query("WITH RECURSIVE rows(n) AS (SELECT 0 WHERE ?1>0 UNION ALL SELECT n+1 FROM rows WHERE n+1<?1) SELECT n FROM rows ORDER BY n").all(row.length) as {n:number}[];
   expect(actual.map(value=>value.n)).toEqual(row.expected);
  }
  expect(law.residentRowOutput&&law.shortWorkRetainsOriginal&&law.originalRecovery).toBe(true);
 } finally {db.close()}
});
