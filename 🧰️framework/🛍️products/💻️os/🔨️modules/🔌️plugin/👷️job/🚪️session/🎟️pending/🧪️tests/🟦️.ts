import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
test("original reserved worker defers admission and preserves source through denied original turns",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,refilledPolicyLoop:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_event(rank INTEGER,state TEXT,body INTEGER)");law.events.forEach((state,rank)=>db.query("INSERT INTO original_event VALUES(?,?,?)").run(rank,state,law.bodyCredits[rank%law.bodyCredits.length]!));expect(db.query("SELECT state FROM original_event ORDER BY rank").all()).toEqual(law.events.map(state=>({state})));expect(db.query("SELECT sum(?1) AS copied,sum(?2) AS born FROM original_event WHERE state='originalAlias'").get(law.aliasReceipt.copiedItems,law.aliasReceipt.retainedCapacityBytes)).toEqual({copied:1,born:0});}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub(crate) fn pending(job:J,operation:semio_framework_job::Operation)");expect(source).toContain("cx.admit_original_cancel_alias()");expect(source).toContain("MountedWorkerJobSession::try_admit_owned(&mut self.job,&mut self.params,cx)");expect(source).toContain("now_us:cx.clock_source()");
 console.log("[DEBUG] Ajv/SQLite original queued worker event order and alias item1/copy0; source denial/runtime closure require native evidence");
});
