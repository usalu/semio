/** 🎟️ Original worker authority keeps semantic custody behind its complete once-only receipt. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import law from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("lower original authority receives every currency once before any semantic loan",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,mutableAuthority:true})).toBe(false);const database=new Database(":memory:");try{database.exec("create table currency(axis integer primary key,issued integer,actual integer,receiver integer);create table originals(identity integer primary key,pending integer check(pending in(0,1)),loans integer)");for(const row of law.receipts){database.exec("delete from currency;delete from originals");for(let axis=0;axis<5;axis++)database.query("insert into currency values(?,?,?,?)").run(axis,row.issued[axis]!,row.actual[axis]!,row.receiver[axis]!);database.query("insert into originals values(1,1,0)").run();const accepted=row.actual.every((actual,axis)=>actual<=row.issued[axis]!&&actual<=row.receiver[axis]!);expect(accepted).toBe(row.received);expect(database.query("select min(actual<=issued and actual<=receiver) as accepted from currency").get()).toEqual({accepted:Number(accepted)});expect(database.query("select identity from originals where pending=0").get()).toBeNull();if(accepted){expect(database.query("update originals set pending=0 where identity=1 and pending=1").run().changes).toBe(1);expect(database.query("update originals set pending=0 where identity=1 and pending=1").run().changes).toBe(0);expect(database.query("select identity from originals where pending=0").get()).toEqual({identity:1});}else expect(database.query("select identity,pending from originals").get()).toEqual({identity:1,pending:1});}expect(law.close.nativeBackingOwnedByAuthority).toBe(true);expect(law.close.terminalDropBytes).toBe(0);}finally{database.close();}});

import closeReceiptCases from "../♻️retirement/🧪️tests/close-receipt.json";
import closeReceiptSchema from "../♻️retirement/🧪️tests/close-receipt.schema.json";

test("worker close only settles newly reported effects alongside a reserved redemption", () => {
  expect(new Ajv({ strict: true }).compile(closeReceiptSchema)(closeReceiptCases)).toBe(true);
  const sql = new Database(":memory:");
  try {
    const oracle = sql.query<{unposted:number|null}, [number,number,number]>(
      "SELECT CASE WHEN ?3 >= ((?1 + ?2) - ?2) THEN ?3 - ((?1 + ?2) - ?2) END AS unposted",
    );
    for (const row of closeReceiptCases) {
      const result = row.posted.map((posted, i) => oracle.get(posted, row.reserved[i]!, row.reported[i]!)!.unposted);
      expect(result.some(value => value === null) ? null : result).toEqual(row.unposted);
    }
  } finally { sql.close(); }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  const receiver = source.slice(source.indexOf("pub fn receive_original_worker_close_effects"), source.indexOf("pub fn receive_original_worker_close_result"));
  expect(receiver).toContain("original_worker_current_progress(");
  expect(receiver).toContain("cx.reserved_return_progress()");
});
