/** ⚖️ Original error receipt debits conserve independently authored currencies before publication. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import inputs from "./🧫️fixtures/📥️inputs.json";
import expected from "./🧫️fixtures/📤️expected.json";
import closing from "./🧫️fixtures/🎟️grants.json";
import schema from "../🧬️schema/🔣️.json";
import invocation from "./🧫️fixtures/📨️invocation.json";
import invocationSchema from "../🧬️schema/📨️invocation.json";
import mediaCustody from "../../../../🏃️runtime/🎞️command/📥️custody/🧪️tests/🧫️fixtures/🔣️.json";
import { retainedCloneWireSchema as grant } from "@semio-tech/value-ts/retained-clone/wire/schema";
import resourceTurns from "./🧫️fixtures/🚪️resources.json";
import resourceSchema from "../🧬️schema/🚪️resources.json";
import renderInput from "./🧫️fixtures/🎬️render.json";
import renderSchema from "../🧬️schema/🎬️render.json";
import serviceInput from "./🧫️fixtures/💡️service.json";
import serviceSchema from "../🧬️schema/💡️service.json";
import { readModuleTreeSource } from "../../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readPluginRuntimeSource } from "../../../../🏃️runtime/📖️source-text/🟦️.ts";
import {pluginSubtreeText, pluginTestTreeText} from "../../../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
const original=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
/** 📖️ The text the unsplit reactor module held: the core root tree and the module trees of the crates its pieces moved into. */
const reactorSource=():string=>["🦀️.rs","⏲️tasks/🦀️.rs","🔄️turn/🦀️.rs","📨️pending/🦀️.rs","🩹️patches/🦀️.rs","📥️cold-pair/🦀️.rs"].map(piece=>readModuleTreeSource(new URL("../../../../"+piece,import.meta.url))).join("\n");
/** 🎬️ Pending native mesh preparation keeps its original surface and publishes only a completed tree. */
test("original render context reaches the actual instance before tree publication",()=>{
 expect(new Ajv({strict:true}).compile(renderSchema)(renderInput)).toBe(true);
 const surface=pluginSubtreeText("📜️declaration","🪟️surface"),instance=pluginSubtreeText("🧵️instance"),runtime=readPluginRuntimeSource(),turn=pluginSubtreeText("⚛️reactor","🔄️turn"),failure=readFileSync(new URL("../../../../../📤️fault/🦀️.rs",import.meta.url),"utf8");
 expect(surface).toContain("cx:&mut semio_framework_job::StepContext<'_>");expect(surface).toContain("OriginalRenderResult<ComponentTree>");expect(instance).toContain("pending_render_failure");expect(instance).toContain("render_with_request_context");expect(instance).toContain("&interaction,cx)");expect(runtime).toContain("plugin_render_surface");expect(runtime).toContain("original_render_result");expect(turn).toContain("plugin_render_surface(runtime, instance, &surface_key,&mut |source,cx|grant.commit_source(source,cx),cx)");expect(runtime).toContain("receive(instance.app.original_render_tree_source(cx)?,cx)?");expect(instance).toContain("pending_render_tree_identity");expect(turn).toContain("Ok(None)");expect(failure).toContain("project_original_turn_failure(source,rest)");
 const db=new Database(":memory:");try{for(const phase of renderInput.phases){const result=db.query("SELECT CASE WHEN ? = 'complete' THEN 1 ELSE 0 END AS published,CASE WHEN ? = 'pending' THEN 1 ELSE 0 END AS retained").get(phase.outcome,phase.outcome)as{published:number;retained:number};expect(result).toEqual({published:Number(phase.published),retained:Number(phase.retained)});}expect((db.query("SELECT ? AS message").get(renderInput.assembly.message)as{message:string}).message).toBe(renderInput.assembly.message);}finally{db.close()}
 const patches=pluginSubtreeText("⚛️reactor","🩹️patches"),native=pluginTestTreeText("⚛️reactor","🩹️patches","🧪️tests","🔬️unit");
 expect(patches).toContain("commit_source(&mut self, source: &mut Option<ComponentTree>, cx: &mut StepContext");
 expect(native).toContain('assert!(!grant.commit_source(&mut source,cx).expect("closing lifetime preserves source and grant"))');
 expect(native).toContain("source.as_ref().unwrap().root.children.get(0).unwrap().key.as_ptr(),pointer");
 expect(native).not.toMatch(/commit_source\((?:leaf|ComponentTree|root)/);
 const receiverDb=new Database(":memory:");try{for(const phase of renderInput.receiver){const actual=receiverDb.query("SELECT CASE WHEN ? THEN NULL ELSE ? END AS source,CASE WHEN ? THEN ? ELSE NULL END AS receiver,? AS presence").get(Number(phase.admitted),phase.treeIdentity,Number(phase.admitted),phase.treeIdentity,Number(phase.admitted))as{source:string|null;receiver:string|null;presence:number};expect(actual.source!==null).toBe(phase.sourceRetained);expect(actual.receiver).toBe(phase.receiverIdentity);expect(Boolean(actual.presence)).toBe(phase.presenceDrained);}}finally{receiverDb.close()}
});
/** 🚪️ The real resource boundary lends one context and returns every original cause without another debit. */
test("resource closure retains full native failures and debits original effects once",()=>{
 expect(new Ajv({strict:true}).compile(resourceSchema)(resourceTurns)).toBe(true);
 const owner=readFileSync(new URL("../../../../../🧩️extension/🚪️retirement/🦀️.rs",import.meta.url),"utf8"),runtime=readPluginRuntimeSource(),turn=pluginSubtreeText("⚛️reactor","🔄️turn");
 expect(owner).toContain("fn close_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_framework_job::OriginalTurnFailure>");expect(owner).toContain("owner.close_step(same)");expect(owner).toContain("original_lifecycle_step(cx");expect(runtime).toContain("extension_retirement_turn(cx:");expect(turn).toContain("extension_retirement_turn(same)");
 const section=turn.slice(turn.indexOf("let original_extension_grant"),turn.indexOf("let close_cleanup_work = close_cleanup_work || extension_retirement_work"));expect(section).not.toContain("cx.consume_retained(progress)");
 const db=new Database(":memory:",{safeIntegers:true});try{for(const row of resourceTurns){for(let axis=0;axis<4;axis++){const sum=db.query("SELECT ? + ? AS spent").get(row.before[axis]!,row.producer[axis]!)as{spent:bigint};expect(sum.spent).toBe(BigInt(row.after[axis]!));}expect(db.query("SELECT ? - 1 AS fuel").get(row.fuelBefore)).toEqual({fuel:BigInt(row.fuelAfter)});for(const cause of row.causes)expect((db.query("SELECT ? AS original").get(cause)as{original:string}).original).toBe(cause);}}finally{db.close()}
});
test("original failure debit input admits exact receipt and original policy, separate expected outcomes",()=>{const ajv=new Ajv({strict:true});const authority=Object.values(schema.properties).find(v=>"$ref"in v&&v.$ref!=="#/$defs/progress")! as{$ref:string};const canonical=new URL("../🧬️schema/🔣️.json",import.meta.url).href;ajv.addSchema(grant,new URL(authority.$ref,canonical).href);const admit=ajv.compile({...schema,$id:canonical});for(const input of inputs)expect(admit(input)).toBe(true);const admitGrant=ajv.compile(grant);for(const policy of Object.values(closing))expect(admitGrant(policy)).toBe(true);expect(admit({...inputs[0],published:true})).toBe(false);});
test("independent SQLite and integer arithmetic preserve real failed child currencies and saturating fuel",()=>{for(let index=0;index<inputs.length;index++){const input=inputs[index]!,output=expected[index]!,db=new Database(":memory:",{safeIntegers:true});try{const metrics=["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"]as const;const axes=["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes"]as const;let accepted=true;for(let axis=0;axis<4;axis++){const key=metrics[axis]!,limit=axes[axis]!,initial=BigInt(input.recipient[key]),child=BigInt(input.originalReceipt[key]);const observed=db.query("SELECT ? + ? AS spent").get(initial,child)as{spent:bigint};expect(observed.spent).toBe(initial+child);expect(observed.spent).toBe(BigInt(output.recipient[key]));accepted&&=observed.spent<=BigInt(input.grant[limit]);}const fuel=db.query("SELECT max(0,? - ?) AS remaining").get(input.fuel,input.originalReceipt.copiedItems)as{remaining:bigint};expect(fuel.remaining).toBe(BigInt(output.fuel));expect(accepted).toBe(output.accepted);expect(JSON.parse(JSON.stringify(input.originalReceipt))).toEqual(input.originalReceipt);}finally{db.close();}}});
test("actual original owner retains canonical Job primary and receipt causes before publication",()=>{expect(original).toContain("close_fault:ManuallyDrop<Option<OriginalTurnFailure>>");expect(original).toContain("debit_original_error");expect(original).toContain("close_fault");expect(original).toContain("debit_fault");expect(original).not.toContain("failure.primary");const start=original.indexOf("fn debit_original_error"),end=original.indexOf("\n }",start),body=original.slice(start,end);expect(original).toContain("cx.consume_fuel(actual_work);if let Err(error)=cx.consume_retained(step.progress())");expect(body).toContain("retained_progress()");expect(body).toContain("Some(OriginalTurnFailure::from(original))");expect(body.indexOf("**close=Some")).toBeLessThan(body.indexOf("consume_retained"));expect(body).toContain("receipt_refusal=cx.consume_retained(progress).err()");expect(body).toContain("receipt_refusal.is_none()");});

/** 🔁️ The existing retained request returns its whole failure while preserving every pending original owner. */
test("Retained extension failure returns to original Entry without a poisoned pending owner",()=>{
 const turn=pluginSubtreeText("⚛️reactor","🔄️turn"),start=turn.indexOf("impl RetainedExtensionInvocation{"),end=turn.indexOf("enum CommandIngressOwner",start),body=turn.slice(start,end);expect(start).toBeGreaterThanOrEqual(0);expect(body).toContain("Result<Option<semio_framework::kernel::RequestOutcome>,semio_framework_job::OriginalTurnFailure>");expect(body).not.toContain("close_refusal");expect(body).toContain("failure.advance(cx)?");expect(body).toContain("close_original_retained_step(cx");expect(body).toContain("RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow_mut()[index]=Some(owner))");expect(body).toContain("result.map(|_|None)");expect(body).not.toContain("to_fault");const db=new Database(":memory:");try{expect(new Ajv({strict:true}).compile(invocationSchema)(invocation)).toBe(true);for(const row of invocation){const state=row.outcome;const observed=db.query("SELECT CASE WHEN ? = 'completed' THEN 0 ELSE 1 END AS retained, CASE WHEN ? = 'failed' THEN 1 ELSE 0 END AS failure").get(state,state)as{retained:number;failure:number};expect(observed).toEqual({retained:Number(row.ownerRetained),failure:Number(row.failureReturned)});}}finally{db.close()}
});

/** 🧹️ The actual retiring lane lends the initiating context and returns the complete native cause. */
test("failed decoded ingress retirement preserves the same original caller and whole failure",()=>{
 const turn=pluginSubtreeText("⚛️reactor","🔄️turn"),reactor=reactorSource();
 expect(turn).toContain("command.retire_failed_step_on_original(cx)");expect(turn).toContain("OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure)");expect(turn).toContain("retire_command_ingress(entry.state,cx)?");expect(turn).toContain("step_retiring_command_ingress(cx)?");expect(reactor).toContain("turn::close_command_ingress_step(state.key,cx)?");
 const db=new Database(":memory:");try{for(const key of ["primary","receipt","context"]as const){const source=mediaCustody.failure[key];const row=db.query("SELECT ? AS cause").get(source)as{cause:string};expect(row.cause).toBe(source);}}finally{db.close()}
});

/** 🪜️ Public app fixtures retain the same pending invocation and full domain cause. */
test("public fixture preview and lifecycle helpers preserve their original source",()=>{
 const source=pluginSubtreeText("🧪️testing"),start=source.indexOf("async fn preview_fixture_on_original"),end=source.indexOf("    /// 🤖️ Previews",start),body=source.slice(start,end);
 expect(start).toBeGreaterThanOrEqual(0);expect(body).toContain("app.command_ctx().preview_addressed_action(invocation,meta,cx).await?");expect(body).toContain("OriginalFixtureCaller::maximum_turns()");expect(body).toContain("cx.should_yield()");expect(body).not.toContain("StepContext::new");expect(body).not.toContain("into_message");expect(source).toContain("bounded_ownership::PluginLifecyclePrimary::Domain(domain)");expect(source).toContain("failure:Some(failure)");expect(source).toContain("PluginApp::maintenance_step(app,same,loan)?");
});

/** 🧬️ Checks the owned response contract against independent exact integer and JSON output. */
test("actual extension response carries original Fault separately into typed command",()=>{
 const reactor=reactorSource(),runtime=readPluginRuntimeSource(),native=readFileSync(new URL("../../../../🧪️tests/🔬️extension-continuation/🦀️.rs",import.meta.url),"utf8"),fixture=JSON.parse(readFileSync(new URL("../../../../🧪️tests/🧫️fixtures/owned-extension-response.json",import.meta.url),"utf8"));
 const start=reactor.indexOf("pub fn extension_response_args"),end=reactor.indexOf("pub(crate) fn take_extension_response",start),body=reactor.slice(start,end);expect(body).toContain("outcome: Result<Vec<u8>, semio_framework::Fault>");expect(body).toContain("Some(fault)");expect(body).not.toContain("fault.message.clone()");expect(body).not.toContain("fault.code.0.clone()");expect(body).not.toContain("Fault::from_value");expect(runtime).toContain("handle_response_action(action, args, original_fault, &meta, identity, cx)");expect(native).toContain("original_extension_response_donates_full_native_fault_into_typed_dispatch");
 const db=new Database(":memory:",{safeIntegers:true});try{const exact=db.query("SELECT ? AS scalar").get(fixture.exactUnsigned)as{scalar:string};expect(exact.scalar).toBe(BigInt(fixture.exactUnsigned).toString());const progress=db.query("SELECT ? AS copied, ? AS released").get(fixture.fault.retainedProgress.copiedBytes,fixture.fault.retainedProgress.releasedBytes)as{copied:bigint;released:bigint};expect(progress).toEqual({copied:577n,released:8537n});expect(JSON.parse(JSON.stringify(fixture.expected))).toEqual({ok:false,output:null});}finally{db.close();}
});

/** 🪙️ Source-root admissions consume the independently authored caller before any owning fixture body. */
test("original Plugin fixture roots charge work and cumulative stock before the body",()=>{
 const caller=readFileSync(new URL("../../../../../🧪️testing/🎟️caller/🦀️.rs",import.meta.url),"utf8");
 expect(caller).toContain("fixture_remaining(policy.grant,root)?");expect(caller).toContain("fixture_remaining(policy.grant,root.checked_add(ledger)?)?");expect(caller).toContain("policy.fuel.checked_sub(receipt.copied_items as u64)");expect(caller).toContain("!self.receipt.fits(self.policy.grant)||!progress.fits(grant)");
 const db=new Database(":memory:",{safeIntegers:true});try{
  for(const input of inputs){const prior=input.recipient.copiedItems,child=input.originalReceipt.copiedItems;const result=db.query("SELECT max(0,? - ?) AS fuel,? + ? AS items").get(input.fuel,child,prior,child)as{fuel:bigint;items:bigint};expect(result).toEqual({fuel:BigInt(Math.max(0,input.fuel-child)),items:BigInt(prior+child)});}
 }finally{db.close()}
});


/** 💡️ Registered service work stays within one request and the actual original context loan. */
test("original inference service uses one cumulative request work limit and context loan",()=>{
 expect(new Ajv({strict:true}).compile(serviceSchema)(serviceInput)).toBe(true);
 const gateway=readFileSync(new URL("../../../../../💡️inference/🚪️gateway/🦀️.rs",import.meta.url),"utf8");
 const native=readFileSync(new URL("../../../../../💡️inference/🚪️gateway/🧪️tests/🦀️.rs",import.meta.url),"utf8");
 expect(native).toContain("OriginalFixtureCaller::admit()");expect(native).toContain("cx.with_retained_loan(grant");expect(native).toContain("cx.original_recipient_identity(),recipient");expect(native).not.toContain("StepContext::new");expect(native).not.toContain("maximum_capacity_bytes:quote.capacity_bytes");expect(native).not.toContain("cancel.clone()");
 expect(gateway).toContain("cx.with_retained_loan(retained");expect(gateway).toContain("with_original_work_loan");expect(gateway).toContain("request.budgets.work_units.saturating_sub(self.execution_work_spent)");expect(gateway).toContain("original_work_units_spent()");expect(gateway).toContain("error.cause=Some(failure)");expect(gateway).toContain("self.discarded_execution=result.execution");
 const db=new Database(":memory:",{safeIntegers:true});try{
  for(const row of serviceInput){for(let axis=0;axis<5;axis++){const available=axis===4?row.stock[axis]!:Math.max(0,row.stock[axis]!-row.spent[axis]!);expect(db.query("SELECT min(?,?) AS loan").get(available,row.request[axis]!)).toEqual({loan:BigInt(row.loan[axis]!)});}
   expect(db.query("SELECT min(?,max(0,?-?)) AS work").get(row.fuel,row.workLimit,row.workSpent)).toEqual({work:BigInt(row.workLoan)});
   expect(db.query("SELECT ?+? AS spent,?-? AS unposted").get(row.workSpent,row.childWork,row.childWork,row.childWork)).toEqual({spent:BigInt(row.workAfter),unposted:BigInt(row.unposted)});
  }
 }finally{db.close()}
});

/** 📥️ Cancellation before child admission still permits original input retirement. */
test("original receiving cancellation closes captured inputs without minting another source loan",()=>{
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/📨️receiving.json",import.meta.url),"utf8")),rows=JSON.parse(readFileSync(new URL("./🧫️fixtures/📨️receiving.json",import.meta.url),"utf8"))as Array<{cancelled:boolean;loanPresent:boolean;dispatchAllowed:boolean;admissionNeeded:boolean;closeAllowed:boolean}>;
 expect(new Ajv({strict:true}).compile(schema)(rows)).toBe(true);
 const source=readFileSync(new URL("../../../../🔄️turn/📥️receiving/🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("parent:cx.original_source_parent()?.source_identity()?");
 expect(source).toContain("self.recipient!=cx.original_recipient_identity()");
 expect(source).toContain("std::ptr::fn_addr_eq(self.clock,cx.clock_source())");
 expect(source).toContain("self.deadline!=cx.deadline_us()");
 expect(source).toContain("self.loan.is_none()&&cx.is_cancelled(){return Ok(true)}");
 expect(source).not.toContain("original_source_parent()?as*const");
 const db=new Database(":memory:");try{for(const row of rows){const actual=db.query("SELECT NOT ? AS dispatch,NOT ? AND NOT ? AS admission,? AS closing").get(Number(row.cancelled),Number(row.cancelled),Number(row.loanPresent),Number(row.cancelled))as{dispatch:number;admission:number;closing:number};expect(Boolean(actual.dispatch)).toBe(row.dispatchAllowed);expect(Boolean(actual.admission)).toBe(row.admissionNeeded);expect(Boolean(actual.closing)).toBe(row.closeAllowed);}}finally{db.close()}
});

/** 🎯️ Actual addressed actions and deferred fixture callers borrow the received original slots. */
test("addressed action dispatch forwards original arguments and whole fault through one context",()=>{
 const trait=readFileSync(new URL("../../../../🔌️app/🦀️.rs",import.meta.url),"utf8"),instance=pluginSubtreeText("🧵️instance"),fixture=readFileSync(new URL("../../../../../🧪️testing/🦀️.rs",import.meta.url),"utf8");
 const start=instance.indexOf("async fn handle_action_invocation("),end=instance.indexOf("\n    }\n",start),body=instance.slice(start,end);
 expect(start).toBeGreaterThanOrEqual(0);expect(trait).toContain("address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>");
 expect(body).toContain("dispatch_action(&address.action_id, source_args, original_fault, meta, identity, cx)");expect(body).not.toContain("invocation.arguments");expect(body).not.toContain("into_message");expect(body).not.toContain("StepContext::new");
 const driverStart=fixture.indexOf("pub async fn dispatch_addressed_fixture_on_original"),driverEnd=fixture.indexOf("async fn preview_fixture_on_original",driverStart),driver=fixture.slice(driverStart,driverEnd);
 expect(driver).toContain("app.handle_action_invocation(address,args,fault,active_mode,meta,identity,cx).await?");expect(driver).toContain("app.maintenance_step(cx,grant)?");expect(driver).not.toContain("context()");expect(driver).not.toContain("StepContext::new");
 const db=new Database(":memory:");try{for(const phase of renderInput.receiver){const actual=db.query("SELECT NOT ? AS sourceRetained,? AS received").get(Number(phase.admitted),Number(phase.admitted))as{sourceRetained:number;received:number};expect(Boolean(actual.sourceRetained)).toBe(phase.sourceRetained);expect(Boolean(actual.received)).toBe(phase.receiverIdentity!==null);}}finally{db.close()}
});
