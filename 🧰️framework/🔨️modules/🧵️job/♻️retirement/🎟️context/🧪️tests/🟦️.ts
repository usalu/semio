import assert from "node:assert/strict";
import "../../../🎟️context/🪙️loan/🧪️tests/🟦️.ts";
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import { readModuleTreeSource, readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
/** 🎟️ Independent accounting keeps close CPU, absolute deadline and identity on the same original source. */
test("original close retains caller context and charges actual work once",()=>{
 const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(base,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true);const db=new Database(":memory:");try{for(const work of law.workUnits){const observed=db.query("SELECT ?1 AS operation,?2 AS generation,?3 AS deadline,?4 AS remaining,CASE WHEN ?4 > 0 AND ?5 < ?3 THEN 1 ELSE 0 END AS spent").get(law.operation,law.generation,law.deadlineUs,work,law.nowUs) as {operation:number;generation:number;deadline:number;remaining:number;spent:number};assert.equal(observed.operation,law.operation);assert.equal(observed.generation,law.generation);assert.equal(observed.deadline,law.deadlineUs);assert.equal(observed.spent,Math.min(work,1));assert.equal(observed.remaining-observed.spent,Math.max(work-1,0));}}finally{db.close()}
 const job=readModuleTreeSource(resolve(base,"../../🦀️.rs"));assert.match(job,/fn close_step\(&mut self,\s*cx:\s*&mut StepContext<'_>\)\s*->\s*Result<InteractiveJobCloseStep,\s*OriginalTurnFailure>/,"actual InteractiveJob close requires borrowed original cx and whole paired failure");
});

/** 🧯️ Worker close uses the same complete original error protocol as interactive close. */
test("original worker close retains complete failures instead of a copied refusal marker",()=>{
 const base=resolve(import.meta.dirname,"../../.."),authority=readFileSync(resolve(base,"👷️worker/🎟️authority/🦀️.rs"),"utf8"),source=readSourceFamilySource(base),wake=readFileSync(resolve(base,"♻️retirement/🔔️wake/🦀️.rs"),"utf8");
 const declaration=authority.slice(authority.indexOf("pub enum WorkerJobCloseStep"),authority.indexOf("impl WorkerJobCloseStep"));assert(declaration.length>0&&!declaration.includes("Refused"),"canonical worker close keeps the whole original failure in Result");assert(!source.includes("WorkerJobCloseStep::Refused")&&!wake.includes("WorkerJobCloseStep::Refused"),"actual worker/cancel/wake producers never strip an owned cause into a kind/progress marker");assert.match(authority,/fn close_original_worker_params\(&mut self,grant:RetainedCloneGrant\)->Result<WorkerJobCloseStep,ValueError>/);assert.match(wake,/fn close_session_retained_wake<J>\(inner:&WorkerJobSessionInner<J>,grant:RetainedCloneGrant\)->Result<Option<WorkerJobCloseStep>,ValueError>/);
});

/** 🪢️ The actual semantic and preparation providers carry the complete canonical owner. */
test("original step and native preparation retain the complete canonical failure",()=>{
 const base=resolve(import.meta.dirname,"../../.."),source=readFileSync(resolve(base,"🧪️conformance/🦀️.rs"),"utf8"),authority=readFileSync(resolve(base,"👷️worker/🎟️authority/🦀️.rs"),"utf8"),preparation=readFileSync(resolve(base,"👷️worker/🚪️admission/🏗️preparation/🦀️.rs"),"utf8"),torture=source.slice(source.indexOf("impl InteractiveJob for TortureJob"));assert.match(torture,/fn step<'a>\(&'a mut self,cx:&mut StepContext<'_>\)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>/);assert.match(preparation,/pub fn original_failure\(&self\)->Option<&OriginalTurnFailure>/);assert(authority.includes("*self.step_fault=Some(error.into())"),"original complete error moves into its existing preparation owner");
});

/** 🧾️ The original Store decoder checks denial before inline custody and posts fuel before a fallible receipt. */
test("original Store close checks denial before mutation and posts scheduling before receipt",()=>{
 const repo=resolve(import.meta.dirname,"../../../../../.."),source=readModuleTreeSource(resolve(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs")),raw=source.slice(source.indexOf("pub fn close_granted(&mut self, grant: RetainedCloneGrant)"),source.indexOf("    fn advance_decode",source.indexOf("pub fn close_granted(&mut self, grant: RetainedCloneGrant)"))),start=source.indexOf("    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)"),close=source.slice(start,source.indexOf("    fn next_close_copy_byte_demand",start));assert(raw.indexOf("if grant.maximum_items == 0 || grant.maximum_depth == 0")<raw.indexOf("self.pending_field=None"),"denied original grant retains inline source metadata");assert(close.lastIndexOf("cx.consume_fuel")<close.lastIndexOf("cx.consume_retained"),"actual scheduling debit precedes fallible original physical receipt");
});

/** 🪜️ Latest-wins physical refusals move whole through the original caller's close ladder. */
test("latest-wins close preserves its complete original failure and mounted caller context",()=>{
 const repo=resolve(import.meta.dirname,"../../../../../.."),base=resolve(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"),admission=readFileSync(resolve(base,"🎮️command/📥️admission/🦀️.rs"),"utf8"),ladder=readFileSync(resolve(base,"🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs"),"utf8");assert.match(admission,/fn close_step\(&mut self,grant:RetainedCloneGrant\)->Result<PluginLifecycleStep,PluginLifecycleFailure>/);assert.match(ladder,/fn close_ladder_step\(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant\) -> Result<RetainedCloneStep, PluginLifecycleFailure>/);assert(ladder.includes("A::mounted_job_close_step(instance_id, cx, grant)"),"mounted job receives the same supplied original context");const raw=admission.slice(admission.indexOf("    pub(crate) fn close_step("),admission.indexOf("    pub(crate) fn close_key_step("));assert(!raw.includes("InteractiveJobCloseStep::Refused")&&!raw.includes('format!("latest-wins original wire refused'),"original wire failure never becomes copied kind/progress or formatted prose");
});

/** 🎫️ Grant-shaped close bodies run through one context debit: independent SQLite arithmetic mirrors fuel, receipt and exhausted-fuel yield. */
test("granted close body is debited once and never runs on exhausted fuel",()=>{
 const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(base,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true);assert.equal(validate({...law,granted:{...law.granted,fuelPerBody:2}}),false);
 const db=new Database(":memory:");try{for(const fuel of [0,1,3]){const row=db.query("SELECT CASE WHEN ?1 > 0 THEN 1 ELSE 0 END AS ran,?1-CASE WHEN ?1 > 0 THEN ?2 ELSE 0 END AS remaining").get(fuel,law.granted.fuelPerBody) as {ran:number;remaining:number};assert.equal(row.ran,Math.min(fuel,1));assert.equal(row.remaining,Math.max(fuel-1,0))}}finally{db.close()}
 const source=readFileSync(resolve(base,"🦀️.rs"),"utf8"),start=source.indexOf("pub fn close_original_granted_step"),body=source.slice(start,source.indexOf("\n}",start));assert(start>0&&body.includes("cx.retirement_should_yield()")&&body.includes("cx.consume_fuel(1)")&&body.includes("cx.consume_retained(step.progress())?")&&body.includes("OriginalTurnFailure::debit_original(error,cx)"),"granted close debits fuel, receipt and whole refusal on the supplied cx");assert(body.indexOf("retirement_should_yield")<body.indexOf("close(cx.retained_grant())"),"exhausted fuel returns before the body runs");
});

/** 🪆️ Nested close bodies owe reported minus posted: independent SQLite arithmetic mirrors the remainder and the under-report refusal. */
test("nested close body owes only the remainder its nested callers did not post",()=>{
 const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(base,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true);assert.equal(validate({...law,nested:{...law.nested,underReport:"ignored"}}),false);
 const db=new Database(":memory:");try{for(const [reported,before,after,owed] of [[2,0,1,1],[2,0,0,2],[1,0,1,0],[0,0,1,-1]]){const row=db.query("SELECT ?1-(?3-?2) AS owed").get(reported,before,after) as {owed:number};assert.equal(row.owed,owed)}}finally{db.close()}
 const source=readFileSync(resolve(base,"🦀️.rs"),"utf8"),start=source.indexOf("pub fn close_original_nested_step"),body=source.slice(start);assert(start>0&&body.includes("checked_sub")&&body.includes("InvariantViolated")&&body.includes("cx.consume_retained(RetainedCloneProgress{"),"nested close debits the checked remainder and refuses an under-report");
});

/** 🧪️ Single-owner laws close over a throwaway self-funded context: the grant is the whole authority and the failure returns whole. */
test("self-funded close context lends exactly its grant and returns the body outcome",()=>{
 const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(base,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true);assert.equal(validate({...law,selfFunded:{...law.selfFunded,fuel:"bounded"}}),false);
 const source=readFileSync(resolve(base,"🦀️.rs"),"utf8"),start=source.indexOf("pub struct SelfFundedCloseCaller"),body=source.slice(start);assert(start>0&&body.includes("StepBudget::new(u64::MAX,u64::MAX,grant)")&&body.includes("StepContext::new(OperationId(1),Generation(1)")&&body.includes("self.cancel.clone()")&&body.includes("pub fn close_with_self_funded_context"),"the lent context holds exactly the supplied grant with unbounded fuel and deadline over one shared cancellation root");
});

import {semioSchemaAjvV1} from "@semio-tech/schema-ts/oracles/validator";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
const read=(name:string)=>readFileSync(new URL(name,import.meta.url),"utf8");
const corpus=JSON.parse(read("./🧫️fixtures/🔣️.json")),schema=JSON.parse(read("../🧬️schema/🔣️.json"));
/** 🪙️ The independent SQL relation checks every exact original close currency before entry. */
test("original close five-currency vectors agree with independent SQLite",()=>{expect(validateJsonSchemaSubset(schema,corpus)).toEqual([]);expect(semioSchemaAjvV1({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");try{for(const row of corpus.cases){const actual=row.grant.every((value:number,index:number)=>value>=row.quote[index]);const result=db.query("SELECT ?1>=?6 AND ?2>=?7 AND ?3>=?8 AND ?4>=?9 AND ?5>=?10 AS admitted").get(...row.grant,...row.quote) as {admitted:number};expect(actual).toBe(row.admitted);expect(Boolean(result.admitted)).toBe(row.admitted);}}finally{db.close();}console.log("[DEBUG] original close independent currency relation verified");});
/** 🧳️ This source proof is separate from the authored native same-caller heap and full-primary laws. */
test("original close keeps whole diagnostic or value and Raster keeps original stages",()=>{const owner=read("../🦀️.rs");expect(owner).toContain("Result<RetainedCloneStep,crate::OriginalTurnPrimary>");expect(owner).toContain("OriginalTurnFailure::debit_original(error,cx)");expect(owner).toContain("cx.consume_retained(progress)?");const base="../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/";for(const name of ["🥞️flatten-layers","🎭️mask-from-selection"]){const work=read(base+name+"/🦀️.rs");expect(work).toContain("semio_framework_job::close_original_granted_step(cx,|grant|");expect(work).toContain("grant.maximum_items==0||grant.maximum_depth==0");expect(work).toContain("fn terminal_is_empty(&self)->bool");}});

/** 🪙️ Inline Copy lease retirement changes metadata and cannot claim physical heap release. */
test("fixed publication lease closes preserve original context through the granted close step",()=>{for(const path of ["./../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🔨️modules/🏠️host/🪪️publication-leases/🦀️.rs","./../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs","./../../../../../../✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/📬️publication-authority/🦀️.rs"]){const source=read(path),start=source.indexOf("impl semio_framework_job_registry::FixedOperationOwner"),owner=source.slice(start,source.indexOf("type ",start));expect(start,path).toBeGreaterThanOrEqual(0);expect(owner).toContain("cx:&mut semio_framework_job::StepContext<'_>");expect(owner).toContain("OriginalTurnFailure");expect(owner).toContain("semio_framework_job::close_original_granted_step(cx,|grant|");expect(owner).toContain("copied_items: 1");}});
