import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

test("Pending mounted render remains scheduled until original paid identity retry",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  const readiness=database.query("SELECT (? AND NOT ? AND NOT ? AND ? AND ? >= ?) AS ready");
  for(const row of fixture.cases)expect(Boolean((readiness.get(Number(row.pending),Number(row.producer),Number(row.job),Number(row.reconciler),row.acknowledged,row.revision) as {ready:number}).ready)).toBe(row.expected);
  const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
  for(const name of ["has_publishable_work","has_drivable_work","has_work"]){
   const start=source.lastIndexOf("fn "+name+"(");
   const end=source.indexOf("\n}",start);
   expect(source.slice(start,end)).toContain("pending_render_ready");
  }
  const retry=source.slice(source.indexOf("pub fn take_pending_render_on_original("),source.indexOf("pub fn take_deferred_ready("));
  expect(retry).toContain("pending_render_ready");expect(retry).toContain("cx.consume_retained(");expect(retry).toContain("cx.consume_fuel(1)");
 }finally{database.close();}
});
