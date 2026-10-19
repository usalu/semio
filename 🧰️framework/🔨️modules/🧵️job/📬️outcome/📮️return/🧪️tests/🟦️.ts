/** 📮️ Original page return conservation agrees with independent SQLite ownership states. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("supplied pages preserve bounded return slots and the same source until actual close", async () => {
 const law=await Bun.file(new URL("../🔣️.json",import.meta.url)).json();
 const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try {
  db.run("CREATE TABLE pages(slot INTEGER PRIMARY KEY,state TEXT,source INTEGER,pointer INTEGER,released INTEGER)");
  const states=Array.from({length:law.capacity},(_,slot)=>({slot,state:"issued",source:7,pointer:100+slot,released:0}));
  for(const row of states) db.query("INSERT INTO pages VALUES($slot,$state,$source,$pointer,$released)").run(Object.fromEntries(Object.entries(row).map(([k,v])=>["$"+k,v])));
  expect(states.every(row=>row.state!=="vacant")).toBe(true);
  expect(db.query("SELECT count(*) AS n FROM pages WHERE state!='vacant'").get()).toEqual({n:law.capacity});
  for(const row of states) {
   row.state="returned";
   db.query("UPDATE pages SET state='returned' WHERE slot=? AND state='issued'").run(row.slot);
   for(const denied of ["items","release","depth","foreign-source"]) {
    expect(row.pointer).toBe(100+row.slot);expect(row.released).toBe(0);
    expect(db.query("SELECT state,pointer,released FROM pages WHERE slot=?").get(row.slot)).toEqual({state:"returned",pointer:row.pointer,released:0});
   }
   row.state="loan";row.released=law.pageBytes;
   db.query("UPDATE pages SET state='loan',released=? WHERE slot=?").run(law.pageBytes,row.slot);
   expect(db.query("SELECT count(*) AS n FROM pages WHERE state!='vacant'").get()).toEqual({n:law.capacity-row.slot});
   row.state="vacant";db.query("UPDATE pages SET state='vacant' WHERE slot=?").run(row.slot);
  }
  expect(db.query("SELECT sum(released) AS n FROM pages").get()).toEqual({n:law.capacity*law.pageBytes});
  expect(states.every(row=>row.state==="vacant")).toBe(true);
  db.exec("CREATE TABLE supplied(ordinal INTEGER PRIMARY KEY, pointer INTEGER)");
  for(const ordinal of law.pageOrder)db.query("INSERT INTO supplied VALUES(?,?)").run(ordinal,100+ordinal);
  expect(db.query("SELECT ordinal,pointer FROM supplied ORDER BY ordinal").all()).toEqual(law.pageOrder.toReversed().map((ordinal:number)=>({ordinal,pointer:100+ordinal})));
  for(const[index,offset]of law.restart.offsets.entries()){
   const length=Math.min(law.pageBytes,law.restart.fileBytes-offset),eof=offset+length===law.restart.fileBytes;
   expect({length,eof}).toEqual({length:law.restart.lengths[index],eof:law.restart.eof[index]});
   expect(db.query("SELECT min(?,?-?) AS length, ?+min(?,?-?)>=? AS eof").get(law.pageBytes,law.restart.fileBytes,offset,offset,law.pageBytes,law.restart.fileBytes,offset,law.restart.fileBytes)).toEqual({length,eof:Number(eof)});
  }
  for(const row of law.cases) {
   expect(Math.ceil(row.bytes/law.pageBytes)<=Math.min(law.operationPages,Math.floor(row.stockBytes/law.pageBytes))).toBe(row.accepted);
   expect(db.query("SELECT (? + ? - 1) / ? <= min(?, ? / ?) AS accepted").get(row.bytes,law.pageBytes,law.pageBytes,law.operationPages,row.stockBytes,law.pageBytes)).toEqual({accepted:Number(row.accepted)});
  }
 } finally {db.close();}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("OriginalCallerSourceLoan")).toBe(true);
 expect(source.includes("admit_loan(cx)")).toBe(true);
 expect(source.includes("source.close_step(cx.original_source_parent()?,cx)")).toBe(true);
 expect(source.includes("Box::new")).toBe(false);
 expect(source.includes("to_vec()")).toBe(false);
 expect(source.includes("impl Drop for OriginalSuppliedJobPage")).toBe(true);
 const native=readFileSync(new URL("../../../../../🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs",import.meta.url),"utf8");
 expect(native.includes("NativeIoStreamRefusal")).toBe(true);
 expect(native.includes("original_total_retained_grant()")).toBe(true);
 expect(native.includes("validate_original_stream_extent(length,cx)")).toBe(true);
 expect(native.includes("pub fn restart_read_page(")).toBe(true);expect(native.includes("pub fn close_result_step(")).toBe(true);expect(native.includes("reservation.publish_last(bytes,cx)")).toBe(true);
 const world=readFileSync(new URL("../../../../../🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/📡️assets/🦀️.rs",import.meta.url),"utf8");
 expect(world.includes("WorldAssetResponseBacking::Original(page)=>page.bytes()")).toBe(true);
 const receiver=world.slice(world.indexOf("pub fn try_from_original("),world.indexOf("pub fn bytes(&self)",world.indexOf("pub fn try_from_original(")));
 expect(receiver.includes("to_vec")).toBe(false);expect(receiver.includes("Original(page)")).toBe(true);
 expect(world.includes("pub fn push_page_at(")).toBe(true);expect(world.includes("self.received_pages != self.page_len")).toBe(true);
 console.log("[DEBUG] Ajv/SQLite bounded original page return, denial/pointer conservation, source-before-vacancy and authored oversize frontier verified");
});
