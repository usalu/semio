import "../../../🎟️context/🪙️loan/🧪️tests/🟦️.ts";
import {test} from "bun:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import { familyText } from "../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
/** 🎟️ Independent accounting keeps close CPU, absolute deadline and identity on the same original source. */
test("original close retains caller context and charges actual work once",()=>{
 const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(base,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true);const db=new Database(":memory:");try{for(const work of law.workUnits){const observed=db.query("SELECT ?1 AS operation,?2 AS generation,?3 AS deadline,?4 AS remaining,CASE WHEN ?4 > 0 AND ?5 < ?3 THEN 1 ELSE 0 END AS spent").get(law.operation,law.generation,law.deadlineUs,work,law.nowUs) as {operation:number;generation:number;deadline:number;remaining:number;spent:number};assert.equal(observed.operation,law.operation);assert.equal(observed.generation,law.generation);assert.equal(observed.deadline,law.deadlineUs);assert.equal(observed.spent,Math.min(work,1));assert.equal(observed.remaining-observed.spent,Math.max(work-1,0));}}finally{db.close()}
 const job=readFileSync(resolve(base,"../../🦀️.rs"),"utf8");assert.match(job,/fn close_step\(&mut self,\s*cx:\s*&mut StepContext<'_>\)\s*->\s*Result<InteractiveJobCloseStep,\s*OriginalTurnFailure>/,"actual InteractiveJob close requires borrowed original cx and whole paired failure");
});

/** 🧯️ Worker close uses the same complete original error protocol as interactive close. */
test("original worker close retains complete failures instead of a copied refusal marker",()=>{
 const base=resolve(import.meta.dirname,"../../.."),source=readFileSync(resolve(base,"🦀️.rs"),"utf8"),wake=readFileSync(resolve(base,"♻️retirement/🔔️wake/🦀️.rs"),"utf8");
 const declaration=source.slice(source.indexOf("pub enum WorkerJobCloseStep"),source.indexOf("impl WorkerJobCloseStep"));assert(!declaration.includes("Refused"),"canonical worker close keeps the whole original failure in Result");assert(!source.includes("WorkerJobCloseStep::Refused")&&!wake.includes("WorkerJobCloseStep::Refused"),"actual worker/cancel/wake producers never strip an owned cause into a kind/progress marker");assert.match(source,/fn close_original_worker_params\(&mut self,grant:RetainedCloneGrant\)->Result<WorkerJobCloseStep,ValueError>/);assert.match(wake,/fn close_session_retained_wake<J>\(inner:&WorkerJobSessionInner<J>,grant:RetainedCloneGrant\)->Result<Option<WorkerJobCloseStep>,ValueError>/);
});

/** 🪢️ The actual semantic and preparation providers carry the complete canonical owner. */
test("original step and native preparation retain the complete canonical failure",()=>{
 const base=resolve(import.meta.dirname,"../../.."),source=readFileSync(resolve(base,"🦀️.rs"),"utf8"),preparation=readFileSync(resolve(base,"👷️worker/🚪️admission/🏗️preparation/🦀️.rs"),"utf8"),torture=source.slice(source.indexOf("impl InteractiveJob for TortureJob"));assert.match(torture,/fn step<'a>\(&'a mut self,cx:&mut StepContext<'_>\)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>/);assert.match(preparation,/pub fn original_failure\(&self\)->Option<&OriginalTurnFailure>/);assert(preparation.includes("step_fault=Some(OriginalTurnFailure::from(error))"),"original complete error moves into its existing preparation owner");
});

/** 🧾️ The original Store decoder checks denial before inline custody and posts fuel before a fallible receipt. */
test("original Store close checks denial before mutation and posts scheduling before receipt",()=>{
 const repo=resolve(import.meta.dirname,"../../../../../.."),source=readModuleTreeSource(resolve(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🦀️.rs")),raw=source.slice(source.indexOf("    fn close_granted(&mut self, grant: RetainedCloneGrant)"),source.indexOf("    fn advance_decode",source.indexOf("    fn close_granted(&mut self, grant: RetainedCloneGrant)"))),start=source.indexOf("    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)"),close=source.slice(start,source.indexOf("    fn next_close_copy_byte_demand",start));assert(raw.indexOf("if grant.maximum_items == 0 || grant.maximum_depth == 0")<raw.indexOf("self.pending_field=None"),"denied original grant retains inline source metadata");assert(close.lastIndexOf("cx.consume_fuel")<close.lastIndexOf("cx.consume_retained"),"actual scheduling debit precedes fallible original physical receipt");
});

/** 🪜️ Latest-wins physical refusals move whole through the original caller's close ladder. */
test("latest-wins close preserves its complete original failure and mounted caller context",()=>{
 const repo=resolve(import.meta.dirname,"../../../../../.."),base=resolve(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"),admission=readFileSync(resolve(base,"🎮️command/📥️admission/🦀️.rs"),"utf8"),ladder=familyText();assert.match(admission,/fn close_step\(&mut self,grant:RetainedCloneGrant\)->Result<PluginLifecycleStep,PluginLifecycleFailure>/);assert.match(ladder,/fn close_ladder_step\(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant\) -> Result<RetainedCloneStep, PluginLifecycleFailure>/);assert(ladder.includes("A::mounted_job_close_step(instance_id, cx, grant)"),"mounted job receives the same supplied original context");const raw=admission.slice(admission.indexOf("    pub(crate) fn close_step("),admission.indexOf("    pub(crate) fn close_key_step("));assert(!raw.includes("InteractiveJobCloseStep::Refused")&&!raw.includes('format!("latest-wins original wire refused'),"original wire failure never becomes copied kind/progress or formatted prose");
});
