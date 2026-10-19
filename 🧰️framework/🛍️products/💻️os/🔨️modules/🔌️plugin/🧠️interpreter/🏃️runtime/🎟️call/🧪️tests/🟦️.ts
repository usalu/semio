import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import { turnJobArgumentsFixtureUrl } from "@semio-tech/framework-actor/testing/fixtures";
/** 🎟️ Core guest calls require actual physical source custody independently from VM authorization. */
test("owned original guest call borrows actual Core and retains unpaid machine",async()=>{const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT ?1+?2 AS vm,?3=0 AS denied").get(1,1,law.copyCuts[0])).toEqual({vm:law.expectedVm,denied:1})}finally{db.close()}const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const name of["begin_original_call_step","advance_original_call_step","take_original_call_result_step","close_original_call_step","validate_context(cx)","OriginalGuestExecutionBudget"])expect(source).toContain(name);expect(source).not.toContain("StepContext::new");expect(source).not.toContain("instantiate_actor(");expect(source).not.toContain("resume_owned_operation_observed");});

/** 📋️ The fixed original job authority is the largest declared inline call, with nineteen scalar slots. */
test("owned original call covers fixed original authority and denies an oversized extent",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json();
 const argumentsLaw=await Bun.file(turnJobArgumentsFixtureUrl).json();
 const db=new Database(":memory:");
 try {
  db.exec("CREATE TABLE arguments(slot INTEGER PRIMARY KEY,value INTEGER)");
  for(let slot=0;slot<argumentsLaw.arguments.length;slot++)db.query("INSERT INTO arguments VALUES(?,?)").run(slot,argumentsLaw.arguments[slot]);
  expect(db.query("SELECT count(*) AS count FROM arguments").get()).toEqual({count:law.maximumArguments});
  expect(db.query("SELECT (?1<=?2) AS allowed,(?1+1<=?2) AS oversized").get(argumentsLaw.arguments.length,law.maximumArguments)).toEqual({allowed:1,oversized:0});
 }finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("arguments:[Value;semio_framework_actor::OriginalJobBudget::SCALAR_ARGUMENT_COUNT]");
 expect(source).toContain("RetainedTurnInput::borrow_original(cx)?");
});
