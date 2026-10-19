import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
type Case=(typeof fixture.cases)[number];
function decision(row:Case):string{if(!row.pristine||!row.matchingIds)return "refused";if(row.cancelled||!row.work||row.grant!=="exact")return "pending";return "admitted";}
test("Energy batch original admission decisions match independent SQL and same-slot custody vectors",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{for(const row of fixture.cases){const sql=db.query<{verdict:string},[number,number,number,number,string]>("SELECT CASE WHEN ?=0 OR ?=0 THEN 'refused' WHEN ?=1 OR ?=0 OR ?!='exact' THEN 'pending' ELSE 'admitted' END AS verdict").get(+row.pristine,+row.matchingIds,+row.cancelled,+row.work,row.grant)!;expect(decision(row)).toBe(row.expected);expect(sql.verdict).toBe(row.expected);const original={id:row.id};const input=decision(row)==="admitted"?null:original;const output=decision(row)==="admitted"?original:null;expect(input??output).toBe(original);expect(Boolean(input)&&Boolean(output)).toBe(false);}}
 finally{db.close();}
});
test("Energy batch owns actual publication leases and authentic source throughout work and close",()=>{
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 const engine=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 const native=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");
 for(const field of ["operation","generation","payload","clock","clock_source","deadline"])expect(source).toContain(field);
 expect(source).toContain("input_identity.is_pristine()");expect(source).toContain("job:input.take().unwrap()");expect(source).toContain("*output=Some(Self");expect(source).toContain("*slot=Some(original)");expect(source).toContain("close_original_retained_step");
 expect(source).toContain("in_flight.is_none()");expect(source).not.toContain("StepContext::new");expect(engine).not.toContain("pub fn run(model");expect(engine).not.toContain("pub fn close_owned_job");
 expect(native).toContain("observe_heap_allocations_on_this_thread");expect(native).toContain("foreign.turn");expect(native).toContain("heap.released_bytes,0");
});
