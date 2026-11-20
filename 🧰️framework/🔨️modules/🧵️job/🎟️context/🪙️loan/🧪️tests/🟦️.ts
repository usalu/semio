import {test} from "bun:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import { valueOrderedRetirementSchemaUrl, cloneWireProgressSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
/** 🧾️ One authored loan input: the original grant, the receipt it was received under, and the request. */
type LoanInput={id:string;original:Record<string,number>;received:Record<string,number>;requested:Record<string,number>};
const base=resolve(import.meta.dirname,"..");
const jobSource=(path:string)=>readFileSync(resolve(base,"../..",path),"utf8");
const storeRoot=resolve(base,"../../../../..","🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
const storeSource=(path:string)=>readFileSync(resolve(storeRoot,path),"utf8");

/** 🔍️ Locates a mounted native fixture body before checking how it spends its caller's loan. */
function rustTestBody(source:string,name:string):string {
 const start=source.indexOf(`fn ${name}()`);
 assert(start>=0,`mounted Rust fixture ${name} exists`);
 const end=source.indexOf("\n}",start);
 assert(end>=0,`mounted Rust fixture ${name} has a complete body`);
 return source.slice(start,end);
}

/** 🪙️ Only original grant/receipt/request inputs authorize a loan; SQLite independently supplies its bounded output. */
test("same original retained loans admit closed input and independent five-axis outputs",()=>{
 const read=(path:string|URL):any=>JSON.parse(readFileSync(path,"utf8")),ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(read(valueOrderedRetirementSchemaUrl));ajv.addSchema(read(cloneWireProgressSchemaUrl),"semio.original-retained-progress");const inputs=read(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json")),expected=read(resolve(base,"🧪️tests/🧫️fixtures/🧾️expected.json")),validate=ajv.compile<LoanInput[]>(read(resolve(base,"🧬️schema/🔣️.json")));assert(validate(inputs),JSON.stringify(validate.errors));const caller=read(resolve(base,"🧪️tests/🧫️fixtures/🎟️caller.json")),validateCaller=ajv.compile<{clockUs:number;deadlineUs:number}>(read(resolve(base,"🧬️schema/🎟️caller.json")));assert(validateCaller(caller),JSON.stringify(validateCaller.errors));assert(caller.clockUs<caller.deadlineUs);assert(!validate([{...inputs[0],freshBudget:true}]));
 const axes=[["maximumItems","copiedItems"],["maximumCopyBytes","copiedBytes"],["maximumCapacityBytes","retainedCapacityBytes"],["maximumReleaseBytes","releasedBytes"],["maximumDepth",null]]as const,db=new Database(":memory:");try{const results=inputs.map((input:{id:string;original:Record<string,number>;received:Record<string,number>;requested:Record<string,number>})=>{if(axes.some(([axis,currency])=>currency!==null&&input.received[currency]!>input.original[axis]!))return{id:input.id,remaining:null};return{id:input.id,remaining:Object.fromEntries(axes.map(([axis,currency])=>{const paid=currency?input.received[currency]!:0,row=db.query("SELECT min(?1,?2-?3) AS amount").get(input.requested[axis]!,input.original[axis]!,paid)as{amount:number};assert.equal(BigInt(paid)+BigInt(row.amount)<=BigInt(input.original[axis]!),true);return[axis,row.amount]}))}});assert.deepEqual(results,expected)}finally{db.close()}
 const job=jobSource("⏱️context/🦀️.rs");assert(job.includes("pub fn with_retained_loan"),"actual original context owns its narrow same-recipient loan");assert(job.includes("impl Drop for OriginalRetainedLoan"),"original physical ceiling restores on return and unwind");
});

/** 🪢️ SQLite independently checks original identity before restoration or successful publication. */
test("retained loan fences context swap replacement and unwind without losing prior causes",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(resolve(base,path),"utf8")),ajv=new Ajv({strict:true,allErrors:true}),inputs=read("🧪️tests/🧫️fixtures/🪢️identity.json"),expected=read("🧪️tests/🧫️fixtures/🪢️expected.json"),validate=ajv.compile<object[]>(read("🧬️schema/🪢️identity.json"));assert(validate(inputs),JSON.stringify(validate.errors));assert(!validate([{...inputs[0],restoreOriginal:true}]));
 const db=new Database(":memory:");try{const actual=inputs.map((row:any)=>{const keys=["recipient","cursor","operation","generation","payload","clock","deadline"],matched=keys.every(key=>(db.query("SELECT ?1 = ?2 AS same").get(row.original[key],row.replacement[key])as{same:number}).same===1);return{id:row.id,restore:matched,refuse:!matched&&row.outcome!=="unwind",primary:row.primary,receiptRefusal:row.receiptRefusal,movedOriginalRemainsNarrowed:!matched,replacementRemainsOriginal:!matched}});assert.deepEqual(actual,expected)}finally{db.close()}
 const job=jobSource("⏱️context/🦀️.rs"),failure=jobSource("⚠️failure/🦀️.rs");assert(job.includes("OriginalRetainedLoanIdentity"),"loan guard retains actual receiver identity");assert(job.includes("if self.matches_original()"),"unwind only restores the exact original receiver");assert(job.includes("R:Copy")&&job.includes("Result<R,OriginalTurnFailure>"),"refusal cannot drop an owned successful output or truncate prior failures");assert(failure.includes("pub context_refusal:Option<ValueError>"),"original pair and distinct context refusal have independent custody");
});

/** 🏪️ Assigned physical callers consume one initial immutable policy and the same cumulative context. */
test("Store original fixture close loans never reconstruct a context inside the funded loop",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(resolve(base,path),"utf8")),ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(JSON.parse(readFileSync(valueOrderedRetirementSchemaUrl,"utf8")));const policies=read("🧪️tests/🧫️fixtures/🎟️store.json"),validate=ajv.compile<{clockUs:number;deadlineUs:number;fuel:number}[]>(read("🧬️schema/🎟️store.json"));assert(validate(policies),JSON.stringify(validate.errors));assert(!validate(policies.map((policy:any)=>({...policy,demandCapacity:1}))));const db=new Database(":memory:");try{for(const policy of policies){const row=db.query("SELECT ?1 < ?2 AND ?3 > 0 AS funded").get(policy.clockUs,policy.deadlineUs,policy.fuel)as{funded:number};assert.equal(row.funded,1)}}finally{db.close()}
 const store=["🧪️tests/🏙️bin-01/🔬️unit/🦀️.rs","🧪️tests/🌴️bin-06/🔬️unit/🦀️.rs"].map(path=>readModuleTreeSource(resolve(storeRoot,path))).join("\n");
 for(const name of ["interactive_envelope_close_reclaims_every_owner_through_the_job_protocol","field_close_failure_preserves_real_receipts_through_the_original_decode_job","normal_decode_release_records_original_wallet_even_after_child_fault"]){
  const body=rustTestBody(store,name);
  assert(body.includes("with_retained_loan"),name+" uses its original same-context loans");
  assert.equal(body.match(/\.context\(\)/g)?.length,1,name+" lends the original context once");
  assert(!body.includes("caller.close(")&&!body.includes("StepBudget::new"),name+" never resets physical receipt or fuel per close");
 }
 const fixture=JSON.parse(storeSource("👥️presence/🧪️tests/🧫️fixtures/🧹️retirement.json"));
 const validatePresence=ajv.compile(read("🧬️schema/🎟️presence.json"));
 assert(validatePresence(fixture.localCapture.wholeCleanupPolicy),JSON.stringify(validatePresence.errors));
 const presence=rustTestBody(storeSource("👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs"),"retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open");
 assert(presence.includes('let policy = &law["wholeCleanupPolicy"]'),"Presence selects the independently authored whole cleanup policy");
 for(const field of ["grant","clockUs","deadlineUs","fuel"]){
  assert(presence.includes(`policy["${field}"]`),`Presence uses its fixture ${field}`);
 }
 assert.equal(presence.match(/OriginalJobFixtureCaller::admit\(/g)?.length,1,"Presence admits one original caller");
 assert(presence.includes("caller.turn(CLOSE_GRANT, |cx| session.close_step(cx))")&&!presence.includes("StepBudget::new")&&!presence.includes("fund_fuel("),"Presence closes through its cumulative caller");
 const caller=jobSource("⏱️context/🧪️testing/🎟️caller/🦀️.rs");
 assert(caller.includes("cx.with_retained_loan(grant")&&caller.includes("&mut self.sequence,&mut self.spent"),"fixture turns narrow the original receipt");
 assert(caller.includes("self.fuel=cx.original_source_fuel_remaining().min(issued_fuel)"),"fixture turns carry forward only remaining fuel");
 const rejected=storeSource("🧪️tests/🚫️rejected-page-close/🦀️.rs");
 assert(rejected.includes('OriginalEnvelopeCaller::admit_cleanup("rejected-setup")')&&rejected.includes("let mut cx=caller.context()")&&rejected.includes("cx.with_retained_loan")&&!rejected.includes("caller.close("),"rejected setup retains its initial cleanup context");
 const doubles=storeSource("🧪️testing/🎭️doubles/🧾️callers/🦀️.rs");
 assert(doubles.includes("self.caller.lend_context()")&&caller.includes("parent.borrow_context(budget,self.clock,&mut self.sequence,&mut self.spent)"),"Store doubles lend their original source and cumulative receipt");
 assert(!doubles.includes("StepBudget::new"),"Store doubles never fund a replacement budget");
});
