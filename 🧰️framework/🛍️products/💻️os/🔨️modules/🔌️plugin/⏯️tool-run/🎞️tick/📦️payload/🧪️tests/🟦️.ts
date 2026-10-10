/** 📨️ Original OS payload source laws retain the declared Toy caller authority. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";

test("original tick payload uses the same declared Toy caller policy and paid borrowed context",()=>{
 const policy=JSON.parse(readFileSync(new URL("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json",import.meta.url),"utf8"));
 expect(policy.native.grant).toEqual([1,4096,65536,262144,64]);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE policy(position INTEGER PRIMARY KEY,value INTEGER)");
  for(const[i,value]of policy.native.grant.entries())db.query("INSERT INTO policy VALUES(?,?)").run(i,value);
  expect(db.query("SELECT value FROM policy ORDER BY position").all().map(row=>(row as {value:number}).value)).toEqual(policy.native.grant);
 }finally{db.close();}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");
 for(const token of["advance_original_tick_payload","append_original","consume_retained","acknowledge_chunk","retained_grant"])expect(source).toContain(token);
 for(const token of["payload_from_bytes","Arc::new","StepContext::new",".finish()",".encode()","to_vec()"])expect(source).not.toContain(token);
 console.log("[DEBUG] Actual OS payload caller grants and complete source laws agree with independent SQLite");
});
