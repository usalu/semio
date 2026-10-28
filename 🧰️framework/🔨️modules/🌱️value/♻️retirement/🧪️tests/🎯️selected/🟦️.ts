import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";
test("selected plain body extent agrees with SQL integer division", () => {
 const db = new Database(":memory:");
 for (const count of fixture.counts) for (const body of fixture.bodyBytes) for (const width of [1, 4]) {
   const row = db.query("SELECT min(CAST(? / ? AS INTEGER), ?) * ? AS bytes").get(body, width, count, width) as { bytes: number };
   expect(row.bytes).toBe(Math.min(Math.floor(body / width), count) * width);
 }
 db.close();
});

test("selected loan keeps its original body when a grant currency is denied", () => {
 const db = new Database(":memory:");
 const law = fixture.selectedLoan;
 const spans = db.query("WITH RECURSIVE remaining(n) AS (VALUES (?) UNION ALL SELECT n - min(n, ?) FROM remaining WHERE n > 0) SELECT min(n, ?) AS bytes FROM remaining WHERE n > 0").all(law.count, law.bodyBytes, law.bodyBytes) as {bytes:number}[];
 expect(spans.map(row => row.bytes)).toEqual([4096,4096,5]);
 const quote = [1,law.bodyBytes,80,law.count,2];
 for (let axis=0; axis<law.deniedCurrencies.length; axis++) {
   const denied = [...quote]; denied[axis]--;
   const row = db.query("SELECT (? >= ? AND ? >= ? AND ? >= ? AND ? >= ? AND ? >= ?) AS admitted").get(...denied.flatMap((value,index)=>[value,quote[index]])) as {admitted:number};
   expect(row.admitted).toBe(0);
   expect(Math.min(law.count,law.bodyBytes)).toBe(4096);
 }
 db.close();
});

test("the original typed cursor selects the complete 48 byte span",()=>{
 const db=new Database(":memory:");const law=fixture.selectedCursor;
 const row=db.query("SELECT min(?,CAST(? / ? AS INTEGER)) * ? AS span").get(law.count,law.bodyBytes,law.width,law.width) as {span:number};expect(row.span).toBe(law.expectedSpan);expect(row.span-1).toBe(47);db.close();
});
