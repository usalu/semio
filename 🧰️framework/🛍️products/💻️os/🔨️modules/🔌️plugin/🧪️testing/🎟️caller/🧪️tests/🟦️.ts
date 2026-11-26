import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {test} from "bun:test";
import assert from "node:assert/strict";
import {readFileSync,readdirSync} from "node:fs";
import {resolve,basename} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import { valueOrderedRetirementSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";
/** 🧩️ Text of a cut test family: the module at `path` and every sibling part, support and extension module that carries its name as prefix. */
const familyText=(root:string,path:string)=>{const dir=resolve(root,path,".."),parent=resolve(dir,".."),base=basename(dir);return readdirSync(parent).filter(n=>n===base||n.startsWith(base+"-")).sort().map(n=>readModuleTreeSource(resolve(parent,n,"🦀️.rs"))).join("\n");};
/** 🎟️ Closed caller input authorizes finite work before the fixture body; SQLite independently witnesses identity and expiry. */
test("Plugin fixture caller admits independent finite original authority",()=>{
 const base=resolve(import.meta.dirname,".."),repo=resolve(base,"../../../../../../.."),read=(path:string)=>JSON.parse(readFileSync(resolve(base,path),"utf8")),ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(JSON.parse(readFileSync(valueOrderedRetirementSchemaUrl,"utf8")));const caller=read("🧪️tests/🧫️fixtures/🎟️caller.json"),validate=ajv.compile(read("🧬️schema/🔣️.json"));const accepted:boolean=validate(caller);assert(accepted,JSON.stringify(validate.errors));assert(!validate({...caller,demandCapacity:1}));const expected=read("🧪️tests/🧫️fixtures/🧾️expected.json"),db=new Database(":memory:");try{const actual=db.query("SELECT ?1 AS operation,?2 AS generation,?3 AS deadlineUs,?4 >= ?3 AS expired,?5 = 'original-parent' AS originalParent").get(caller.operation,caller.generation,caller.deadlineUs,caller.clockUs,caller.sourceKind)as{operation:number;generation:number;deadlineUs:number;expired:number;originalParent:number};assert.deepEqual({...actual,expired:!!actual.expired,originalParent:!!actual.originalParent,oneShot:true,callerRetainsFailure:true},expected)}finally{db.close()}
});

test("Exact window replay has a bounded fixture allowance", () => {
 const caller = JSON.parse(readFileSync(new URL("./🧫️fixtures/🎟️caller.json", import.meta.url), "utf8"));
 const sqlite = new Database(":memory:");
 try {
  const allowance = sqlite.query("SELECT ?1 < ?2 AND ?2 < ?3 AS bounded").get(caller.maximumTurns, caller.maximumWindowRestoreTurns, caller.maximumCloseTurns);
  assert.deepEqual(allowance, { bounded: 1 });
 } finally { sqlite.close(); }
});
/** 🪙️ Real fixture helpers borrow one original context across phase loans and return the complete original failure. */
test("Plugin settle close and history consumers preserve original fixture authority",()=>{
 const root = resolve(import.meta.dirname, "../../..");
 const source = readModuleTreeSource(resolve(root, "🧪️testing/🦀️.rs"));
 const wrapper = source.indexOf("fn settle_registered_typed_operation<");
 const forwarding = source.slice(wrapper, source.indexOf("\n}", wrapper));
 assert(forwarding.includes("settle_registered_typed_operation_within(app, receiver, mounted_policy, identity, cx, OriginalFixtureCaller::maximum_turns())"));
 const phaseCalls = {
  settle_registered_typed_operation_within: [
   "MountedOwnerTurnV1::admit(receiver, caller, phase, mounted_policy, cx)",
   "app.mounted_owner_maintenance_step(&mut turn)",
  ],
  close_registered_fixture_app: ["original_lifecycle_step(cx", "app.close_step(same, grant)"],
 };
 for (const [name, calls] of Object.entries(phaseCalls)) {
  const start = source.indexOf("fn " + name + "<");
  const body = source.slice(start, source.indexOf("\n///", start + 1));
  assert(body.includes("cx:&mut semio_framework_job::StepContext"), name + " borrows caller-owned context");
  assert(body.includes("PluginLifecycleFailure"), name + " preserves complete owned causes");
  assert(calls.every(call => body.includes(call)), name + " lends same original phase");
  for (const forbidden of ["fixture_step_context", "runtime_lifecycle_step", "into_message()", "typed_operation_refusal()"]) {
   assert(!body.includes(forbidden), name + " avoids " + forbidden);
  }
 }
 const caller = readFileSync(resolve(root, "🧪️testing/🎟️caller/🦀️.rs"), "utf8");
 assert(source.includes("OriginalFixtureCaller") && caller.includes("original fixture context can only be issued once"), "original fixture caller retains one-shot finite budget");
 const history = readModuleTreeSource(resolve(root, "🧪️tests/🧪️history-edit-acceptance/🦀️.rs"));
 assert(!history.includes("advance_typed_operation_publication(&mut identity, acceptance_policy().maintenance)") && !history.includes("runtime_lifecycle_grant()"), "history consumer carries original context");
});

/** 🔄️ Registered close admits each owner under a fresh turn while the phase loan still limits physical spend. */
test("Registered fixture close renews the caller grant before quoting each close step", () => {
 const source = readModuleTreeSource(new URL("../../🦀️.rs", import.meta.url));
 const start = source.indexOf("fn close_registered_fixture_app<");
 assert(start >= 0);
 const body = source.slice(start, source.indexOf("\n}", start));
 const loop = body.indexOf("while close_turn < maximum_turns && idle_streak < maximum_idle_turns");
 const ceiling = body.indexOf("cx.original_turn_ceiling()");
 const renew = body.indexOf("fixture_renew_turn(cx, fresh_fuel, fresh_grant)?", loop);
 const quote = body.indexOf("app.close_retirement_demands", loop);
 assert(ceiling >= 0 && loop > ceiling && renew > loop && quote > renew);
 assert(body.includes("OriginalFixtureCaller::maximum_close_turns()"));
 assert(body.includes("OriginalFixtureCaller::maximum_idle_turns()"));

 const caller = JSON.parse(readFileSync(new URL("./🧫️fixtures/🎟️caller.json", import.meta.url), "utf8"));

 const closeCopy = 32768;
 const quoteCopy = 104;
 const copies = [54, closeCopy];
 const sqlite = new Database(":memory:");
 try {
  const close = sqlite.query("SELECT ?1 > ?2 AND ?1 <= min(?3, ?4) AS funded").get(
   caller.maximumCloseTurns, caller.maximumTurns, caller.grant.maximumItems, caller.fuel,
  ) as { funded: number };
  assert.equal(close.funded, 1);
  const idle = sqlite.query("SELECT ?1 < ?2 AS runnable");
  assert.deepEqual(
   [caller.maximumIdleTurns - 1, caller.maximumIdleTurns].map(turn =>
    (idle.get(turn, caller.maximumIdleTurns) as { runnable: number }).runnable),
   [1, 0],
  );
  const query = sqlite.query("SELECT min(?1, ?2) >= ?3 AS funded");
  const funded = copies.map(copy => Number(Math.min(copy, closeCopy) >= quoteCopy));
  assert.deepEqual(funded, [0, 1]);
  assert.deepEqual(copies.map(copy => (query.get(copy, closeCopy, quoteCopy) as { funded: number }).funded), funded);
 } finally {
  sqlite.close();
 }
});

/** 📨️ The defining complete failure wire uses the same closed caller input through output and issuer retirement. */
test("Original extension failure wire retains one finite caller through complete close",()=>{
 const root=resolve(import.meta.dirname,"../../.."),source=readFileSync(resolve(root,"🧩️extension/📨️invoke/⚠️failure/🧪️tests/🦀️.rs"),"utf8"),start=source.indexOf("fn original_extension_context_failure_preserves_complete_canonical_wire(){"),bodyEnd=source.indexOf("\n#[",start+1),body=source.slice(start,bodyEnd<0?undefined:bodyEnd);assert(start>=0);assert.equal(body.match(/caller\.context\(\)/g)?.length,1);assert(body.includes("cx.with_retained_loan(loan")&&body.includes("caller.retire(fuel)")&&body.includes("close_original_retained_step(same"));assert(!body.includes("StepContext::new")&&!body.includes("StepBudget::new")&&!body.includes("maximum_capacity_bytes:demand"));assert(body.includes("serde_json::Value")&&body.includes("assert_eq!(observed,expected)"));assert(body.includes("caller_birth.requested_bytes+caller_close.requested_bytes"));assert(body.includes("slots[slot].take()")&&body.includes("slot=1-slot;slots[slot]=Some(owner)"));
});

/** 📄️ The fixed physical block outlives zero and short release loans; SQLite independently computes the closed release verdicts. */
test("Plugin original fixture fixed page retirement has independent physical release cases",()=>{
 const base=resolve(import.meta.dirname,".."),read=(path:string)=>JSON.parse(readFileSync(resolve(base,path),"utf8")),input=read("🧪️tests/🧫️fixtures/📄️retirement.json"),validate=new Ajv({strict:true}).compile(read("🧬️schema/📄️retirement.json"));const accepted:boolean=validate(input);assert(accepted);const db=new Database(":memory:");try{for(const row of input.cases){const actual=db.query("SELECT CASE WHEN ?1 > 0 AND ?2 >= ?3 THEN ?3 ELSE 0 END AS releasedBytes").get(row.items,row.releaseBytes,input.blockBytes)as{releasedBytes:number};assert.equal(actual.releasedBytes,row.releasedBytes)}}finally{db.close()}
});

/** 🧰️ Builder contract wrappers spend their supplied caller context and require explicit funded closure before Drop. */
test("Builder contract fixture wrappers retain original caller through dispatch and explicit close",()=>{
 const root=resolve(import.meta.dirname,"../../.."),source=familyText(root,"🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs");for(const name of ["ContractApp","ContractComposedApp"]){const start=source.indexOf("impl Drop for "+name+" {"),end=source.indexOf("\n    impl "+name+" {",start),body=source.slice(start,end);assert(body.includes("close_terminal_is_empty()"));assert(!body.includes("close_registered_fixture_app")&&!body.includes("drain_and_close_composed_fixture"),name+" Drop only witnesses prior original financed closure")}for(const name of ["handle_intent_settled","settle_contract_app"]){const start=source.indexOf("async fn "+name+"("),end=source.indexOf("\n    }",start),body=source.slice(start,end);assert(body.includes("cx:&mut semio_framework_job::StepContext"),name+" borrows original caller");assert(body.includes("identity,cx)")||body.includes("fixture_identity(),cx)"),name+" lends same original context")}
});

/** 🧵️ Tool-run and time-travel helpers carry the initiating original context through every publication and closure. */
test("External tool and history fixture callers retain supplied original context",()=>{
 const root=resolve(import.meta.dirname,"../../.."),tool=familyText(root,"🧪️tests/🔬️tool-run/🦀️.rs"),member=readFileSync(resolve(root,"🧪️tests/🧪️tool-run-member/🦀️.rs"),"utf8"),history=familyText(root,"🧪️tests/🧪️time-travel/🦀️.rs");assert(tool.includes("context:&'context mut semio_framework_job::StepContext"),"tool caller borrows original context");assert(tool.includes("advance_typed_operation_publication(&mut caller.identity,caller.context)"));assert(tool.includes("close_registered_fixture_app(app,caller.policy,caller.context)"));assert(member.includes("close_registered_fixture_app(app,caller.policy,caller.context)"));assert(history.includes("advance_typed_operation_publication(identity,cx)"));assert(history.includes("close_registered_fixture_app(app,mounted_policy,cx)"));
});

/** 🪢️ Source loans and paid parent retirement retain the actual admitted fixture parent. */
test("Plugin fixture source loans use its canonical admitted parent", () => {
 const caller = readModuleTreeSource(new URL("../🦀️.rs", import.meta.url));
 const job = readModuleTreeSource(new URL("../../../../../../../🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs", import.meta.url));
 assert(caller.includes("OriginalJobFixtureCaller::admit("));
 assert(caller.includes("self.caller.lend_refreshed_context().with_original_decoder(&mut self.decoder)"));
 assert(job.includes("admission.prepare(&mut authority,&mut cancel,e)"));
 assert(job.includes("admission.admit_backing(e)"));
 assert(job.includes("admission.publish(&mut parent,e)"));
 assert(job.includes("parent.borrow_context(budget,self.clock,&mut self.sequence,&mut self.spent)"));
 assert(caller.includes("self.caller.settle_lent_fuel(fuel)"));
 assert(caller.includes("self.caller.retire_step(loan)"));
 assert(!caller.includes("StepContext::with_payload_authority("));
});

/** 🧷️ Acceptance annotations retain the same owned original failure through child seeding and reload. */
test("History acceptance annotations retain complete original failures",()=>{const source=readModuleTreeSource(new URL("../../../🧪️tests/🧪️history-edit-acceptance/🦀️.rs",import.meta.url));assert(source.includes('reason.context("the seed does not land".into())'));assert(source.includes('reason.context("after the conflict was resolved the finalized document does not survive save and load".into())'));assert(!source.includes('map_err(|reason| format!("the seed does not land: {reason}"))'));assert(source.includes("Original(AcceptanceError)"));assert(source.includes("Err(AcceptanceSeedFault::Original(reason)) => return AcceptanceVerdict::Fail(reason)"));assert(!source.includes('AcceptanceSeedFault::Base(format!("the base document does not load: {fault:?}"))'));});

/** 📨️ Closed unit/data fixture commands use the same retained action endpoint and one genuine decoder. */
test("Original typed fixture action projection agrees with schemas and independent SQLite",()=>{const base=resolve(import.meta.dirname,".."),law=JSON.parse(readFileSync(resolve(base,"🧪️tests/🧫️fixtures/📨️command.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(base,"🧬️schema/📨️command.json"),"utf8"));assert.deepEqual(validateJsonSchemaSubset(schema,law),[]);const accepted:boolean=new Ajv({strict:true}).compile(schema)(law);assert(accepted);const db=new Database(":memory:");try{for(const row of law.cases){const projected=row.variant==="Noop"?null:{value:row.value};const other=db.query("SELECT CASE WHEN ?1='Noop' THEN 'null' ELSE json_object('value',?2) END AS args").get(row.variant,row.value)as{args:string};assert.deepEqual(projected,JSON.parse(other.args));assert.deepEqual(projected,row.expectedArgs);}}finally{db.close()}const caller=readModuleTreeSource(new URL("../🦀️.rs",import.meta.url)),helpers=readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url));assert(caller.includes(".with_original_decoder(&mut self.decoder)"));assert(caller.includes("fn original_plugin_fixture_decoder_constructs_and_closes_under_the_same_source"));assert(caller.includes("caller.retire(fuel).unwrap();assert!(caller.decoder.terminal_is_empty())"));assert(helpers.includes("fn original_fixture_command_projection_matches_the_closed_unit_and_data_cases"));assert(helpers.includes("dispatch_fixture_action(app,action,args,meta,identity,cx,mounted_policy.maintenance)"));assert(!readFileSync(new URL("../../🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs",import.meta.url),"utf8").includes(".dispatch_typed("));});
