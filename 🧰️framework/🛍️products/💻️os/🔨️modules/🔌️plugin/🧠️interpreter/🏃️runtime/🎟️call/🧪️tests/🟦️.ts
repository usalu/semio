import "./failed-input.🟦️.ts";
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {turnJobArgumentsFixtureUrl} from "@semio-tech/framework-actor/testing/fixtures";
/** 🎟️ Core calls retain their genuine source independently from VM authorization. */
test("owned original guest call borrows actual Core and retains unpaid machine",async()=>{const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("SELECT ?1+?2 AS vm,?3=0 AS denied").get(1,1,law.copyCuts[0])).toEqual({vm:law.expectedVm,denied:1})}finally{db.close()}const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");for(const name of["begin_original_call_step","advance_original_call_step","take_original_call_result_into_step","close_original_call_step","validate_context(cx)","OriginalGuestExecutionBudget"])expect(source).toContain(name);expect(source).not.toContain("StepContext::new");expect(source).not.toContain("instantiate_actor(");expect(source).not.toContain("resume_owned_operation_observed");});
/** 📋️ Failure acknowledgement adds its real four-scalar selector prefix to the nineteen-scalar job authority. */
test("owned original call covers the largest authored scalar ABI and rejects an oversized extent",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),argumentsLaw=await Bun.file(turnJobArgumentsFixtureUrl).json();const db=new Database(":memory:");
 try{expect(argumentsLaw.arguments.length).toBe(law.budgetArguments);expect(db.query("SELECT ?1+?2 AS maximum,(?3<=?1+?2) AS allowed,(?3+1<=?1+?2) AS oversized").get(argumentsLaw.arguments.length,law.prefixArguments,law.maximumArguments)).toEqual({maximum:law.maximumArguments,allowed:1,oversized:0})}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("arguments:[Value;OwnedSemioExport::MAX_SCALAR_ARGUMENTS]");expect(source).toContain("RetainedTurnInput::borrow_original(cx)?");
 const section=(id:number,bytes:number[])=>[id,bytes.length,...bytes],name=Array.from(new TextEncoder().encode("last"));const signature=[1,0x60,law.maximumArguments,...Array<number>(law.maximumArguments).fill(0x7e),1,0x7e],body=[0,0x20,law.maximumArguments-1,0x0b];const bytes=new Uint8Array([0,97,115,109,1,0,0,0,...section(1,signature),...section(3,[1,0]),...section(7,[1,name.length,...name,0,0]),...section(10,[1,body.length,...body])]);const instance=new WebAssembly.Instance(new WebAssembly.Module(bytes));expect((instance.exports.last as(...values:bigint[])=>bigint)(...Array.from({length:law.maximumArguments},(_,index)=>BigInt(index)))).toBe(BigInt(law.maximumArguments-1));
});
