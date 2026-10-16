import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import { readPluginRuntimeSource } from "../../../⚛️reactor/🏃️runtime/📖️source-text/🟦️.ts";
import { valueOrderedRetirementSchemaUrl, cloneWireProgressSchemaUrl } from "../../../../../../../🔨️modules/🌱️value/🧪️tests/🧫️fixtures/🟦️.ts";

/** 📨️ The original invoke caller owns pending raw bytes and lends one unchanged scheduler wallet. */
export function extensionInvokeCallerOracle(repoRoot:string):number{
 const root=resolve(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"),read=(path:string)=>readFileSync(resolve(root,path),"utf8");
 const ajv=new Ajv({strict:true,allErrors:true});
 ajv.addSchema(JSON.parse(readFileSync(valueOrderedRetirementSchemaUrl,"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(cloneWireProgressSchemaUrl,"utf8")),"flow.original-retained-progress");
 const fixture=JSON.parse(read("🧩️extension/🧪️tests/🧫️fixtures/📨️invoke/🔣️.json")),validate=ajv.compile(JSON.parse(read("🧩️extension/🧬️schema/📨️invoke/🔣️.json")));
 let assertions=0;assert.equal(validate(fixture),true,JSON.stringify(validate.errors));assertions++;
 for(const key of Object.keys(fixture)){const invalid={...fixture};delete invalid[key];assert.equal(validate(invalid),false);assertions++;}
 assert.equal(validate({...fixture,alternateWallet:true}),false);assertions++;
 const bytes=fixture.text.repeat(fixture.repeat);assert.equal(Buffer.byteLength(bytes),fixture.utf8Bytes);assert.deepEqual([...Buffer.from(bytes)],[...new TextEncoder().encode(bytes)]);assertions+=2;
 let responses=0;
 for(const reply of fixture.replies){const grant=fixture.caller.grant,progress=reply.progress;const remaining=applyPatch(grant,[{op:"replace" as const,path:"/maximumItems",value:grant.maximumItems-progress.copiedItems},{op:"replace" as const,path:"/maximumCopyBytes",value:grant.maximumCopyBytes-progress.copiedBytes},{op:"replace" as const,path:"/maximumCapacityBytes",value:grant.maximumCapacityBytes-progress.retainedCapacityBytes},{op:"replace" as const,path:"/maximumReleaseBytes",value:grant.maximumReleaseBytes-progress.releasedBytes}],true,false).newDocument;assert(Object.values(remaining).every(value=>(value as number)>=0));assert.equal(remaining.maximumDepth,grant.maximumDepth);assertions+=2;if(reply.payload!==null){JSON.parse(reply.payload);responses++;}else{assert.equal(responses,fixture.expected.pendingResponses);assertions++;}}
 assert.deepEqual(fixture.executionGrant,{maximumItems:16,maximumCopyBytes:64,maximumCapacityBytes:262144,maximumReleaseBytes:262144,maximumDepth:128});assertions++;
 for(const entry of fixture.admissionCases){const demand=applyPatch({maximumItems:2,maximumCopyBytes:0,maximumCapacityBytes:entry.frameBytes,maximumReleaseBytes:0,maximumDepth:1},[{op:"replace" as const,path:"/maximumItems",value:2+Number(entry.checkpoint)}],true,false).newDocument;assert.deepEqual(entry.demand,demand);assertions++;}
 for(const row of fixture.lookupCases){const actual=row.candidates.findIndex((candidate:string)=>Buffer.from(candidate).equals(Buffer.from(row.input)));assert.equal(actual<0?null:actual,row.matched);assertions++;}
 const owner=read("🧩️extension/🚪️retirement/🦀️.rs"),runtime=readPluginRuntimeSource(),reactor=read("⚛️reactor/🔄️turn/🦀️.rs");
 assert(/pub struct ExtensionInvokeStep\s*\{[^}]*pub payload:Option<Vec<u8>>[^}]*pub retained_progress:RetainedCloneProgress/s.test(owner),"original extension result retains optional payload and all actual effects");assertions++;
 assert(/fn invoke\([^;{]*cx:\s*&mut semio_framework_job::StepContext<'_>[^;{]*Result<ExtensionInvokeStep,\s*Fault>/s.test(owner),"original extension receiver borrows the actual caller context");assertions++;
 assert(owner.includes("invoke(capability, request, cx)"));assertions++;
 assert(runtime.includes("invoke(capability, request, cx)"));assertions++;
 assert(reactor.includes("extension_invoke(&capability, &payload, cx)"));assertions++;
 assert(reactor.includes("RetainedExtensionInvocation")&&reactor.includes("answer.payload"));assertions++;
 assert(owner.includes("pub refusal:Option<ValueError>")&&owner.includes("self.refusal=Some("),"refused original output remains in the same invocation step");assertions++;
 assert(reactor.includes("RetainedExtensionFaultReply")&&reactor.includes("failure.advance(cx)"),"actual raw failure crosses the caller-funded original diagnostic projection");assertions++;
 assert(reactor.includes("extension_invocation: extension_invocation_pending"),"actual pending owner keeps the original reactor runnable");assertions++;
 const jobs=read("⚛️reactor/💼️jobs/🦀️.rs");
 assert(/pub trait BoundedJob\s*\{\s*fn step\([^;]*cx:\s*&mut semio_framework_job::StepContext<'_>/s.test(jobs),"bounded inference receiver borrows the original caller context");assertions++;
 assert(jobs.includes("owner.step(original, snapshot, cx)"),"actual job receiver forwards the same original context");assertions++;
 assert(/pub type BoundedJobAdmission\s*=\s*fn\(u64,\s*&mut Option<Vec<u8>>,\s*&mut Option<Vec<u8>>,\s*&mut semio_framework_job::StepContext<'_>\)/s.test(jobs),"original inference admission lends whole raw input and checkpoint under its caller context");assertions++;
 assert(!jobs.includes("factory(job, input, restored.as_deref())"),"pending original factory source is not cold copied or borrowed from a temporary");assertions++;
 assert(jobs.includes("pub use semio_framework::kernel::OriginalJobSource as OriginalJobAdmission"),"actual registry admission shares the defining original owned source");assertions++;
 const originalSource=readFileSync(resolve(repoRoot,"🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs"),"utf8");assert(/pub struct OriginalJobSource[^}]*pub job:u64[^}]*pub kind:String[^}]*pub input:Option<Vec<u8>>[^}]*pub checkpoint:Option<Vec<u8>>/s.test(originalSource),"same defining source retains every original owned admission field");assertions++;
 assert(/pub async fn start_job\(admission:\s*&mut Option<OriginalJobAdmission>,\s*cx:\s*&mut semio_framework_job::StepContext/.test(jobs),"actual registry loan uses the original caller grant before taking source");assertions++;
 assert(!jobs.includes("kind: kind.to_string(), input: Some(input.to_vec())"),"registry does not cold clone the original admission");assertions++;
 assert(jobs.includes("pub struct BoundedJobFactory")&&jobs.includes("pub type BoundedJobDemand"),"registered original factory requires its actual five-axis admission demand companion");assertions++;
 assert(jobs.includes("(factory.demands)(job,&source.input,&source.checkpoint,cx)"),"actual registry demand uses the retained original request and caller context");assertions++;
 assert(jobs.includes("static KIND_REGISTRY: RefCell<[Option<RegisteredJobKind>;REGISTERED_JOB_KINDS]>")&&!jobs.includes("RefCell::new(builtin_registry())"),"first original admission has a preborn kind registry");assertions++;
 assert(jobs.includes("struct OriginalJobKindLookup")&&jobs.includes("JobBody::Resolving"),"normal registry resolves original kind under separately paid source-byte turns");assertions++;
 assert(jobs.includes("static REGISTERED_KIND_COUNT: Cell<usize>")&&jobs.includes("self.index>=REGISTERED_KIND_COUNT.with(Cell::get)+6"),"normal lookup visits only the actual occupied kind prefix before its six builtins");assertions++;
 assert(!jobs.includes("registered_factory(admission.as_ref().unwrap().kind.as_str())"),"registry admission transfers handles before reading original kind bytes");assertions++;
 assert(/pub async fn cancel_job\(job:u64,cx:&mut semio_framework_job::StepContext/.test(jobs),"job cancellation borrows the original caller identity and grant");assertions++;
 assert(jobs.includes("pub fn has_owned_jobs(operation:u64,generation:u64)->bool"),"actor context remains owned until every same-identity job is terminal");assertions++;
 assert(jobs.includes("failed_owner:Option<Box<dyn BoundedJob>>")&&jobs.includes("slot.failed_owner=Some("),"normal body failure preserves its original boxed producer before paid closure");assertions++;
 assert(!jobs.includes("pub struct JobBudget")&&jobs.includes("owner.step(original, snapshot, cx)"),"normal job bodies use the exact original context as their sole fuel and deadline authority");assertions++;
 const custody=fixture.bodyFailure.custody;let ownership={body:true,source:true,published:false};for(const stage of custody){const changes=stage==="body-closed"?[{op:"replace" as const,path:"/body",value:false}]:stage==="source-closed"?[{op:"replace" as const,path:"/source",value:false}]:stage==="failure-published"?[{op:"replace" as const,path:"/published",value:true}]:[];ownership=applyPatch(ownership,changes,true,false).newDocument;if(ownership.published){assert.equal(ownership.body,false);assert.equal(ownership.source,false);assertions+=2;}}assert.deepEqual(ownership,{body:false,source:false,published:true});assertions++;
 const turn=jobs.slice(jobs.indexOf("pub async fn step_job("),jobs.indexOf("//#region 🔖️GuestAbi")),outerGate=turn.indexOf("if cx.should_yield(){return Ok(JobStep::Running(None))}"),cancellation=turn.indexOf("if !slot.cancellation_fault_started");assert(outerGate>=0&&outerGate<cancellation,"outer job terminal work preserves its original source on fuel/deadline denial before cancellation work");assertions++;
 assert(turn.includes("slot.failed_owner.as_mut(){owner.cancel()}"),"cancellation after a body error signals that same retained failed producer");assertions++;
 for(const denied of fixture.jobWorkDenials){const permitted=denied.fuel>0&&denied.now<denied.deadline;const outcome=applyPatch({advanced:false},[{op:"replace" as const,path:"/advanced",value:permitted}],true,false).newDocument;assert.equal(outcome.advanced,false);assertions++;}
 const pack=readFileSync(resolve(repoRoot,"🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs"),"utf8");
 assert(pack.includes("compare_left: Option<u8>"),"original diagnostic symbol comparison retains its separately paid first byte");assertions++;
 assert(!pack.includes("Phase::Find => 2"),"a fixed one-byte caller can finish original diagnostic symbol comparisons");assertions++;
 const failure=read("🧩️extension/📨️invoke/⚠️failure/🦀️.rs");
 const close=jobs.slice(jobs.indexOf("fn close_original<"),jobs.indexOf("fn close_failed_owner("));assert(close.includes("cx.should_yield()")&&close.includes("maximum_items:1")&&close.includes("cx.consume_fuel(1)"),"original cleanup limits its actual child work to the same one-unit caller loan");assertions++;
 assert(failure.includes("cx.should_yield()")&&failure.includes("maximum_items:1")&&failure.includes("cx.consume_fuel(1)"),"normal original diagnostic projection and close debit actual caller work");assertions++;
 const twoPhase=jobs.slice(jobs.indexOf("impl BoundedJob for TwoPhaseBoundedJob"),jobs.indexOf("//#region 🔖️BuiltinKinds"));assert(twoPhase.includes("owner.selected_demand(copy)?"),"normal builtin close forwards the defining selected whole demand");assertions++;
 for(const copy of fixture.jobCloseWork.copyGrants){const work=applyPatch({remaining:fixture.jobCloseWork.fuel,copiedBytes:0},[{op:"replace" as const,path:"/remaining",value:fixture.jobCloseWork.fuel-1},{op:"replace" as const,path:"/copiedBytes",value:Math.min(copy,fixture.jobCloseWork.sourceBytes)}],true,false).newDocument;assert.equal(work.remaining,0);assert.equal(work.copiedBytes,copy);assertions+=2;}
 assert(failure.includes("fn new_terminal_cancellation")&&jobs.includes("new_terminal_cancellation(cause)"),"the terminal job cancellation reply keeps the actual canceled caller context");assertions++;
 for(const cut of fixture.cancelCuts){const prefix=fixture.replies.slice(0,Math.min(cut,fixture.replies.length));const retired=applyPatch({owned:prefix},[{op:"replace" as const,path:"/owned",value:[]}],true,false).newDocument;assert.deepEqual(retired.owned,[]);assert.deepEqual(prefix,fixture.replies.slice(0,prefix.length));assertions+=2;}
 assert(failure.includes("pub(crate) fn begin_close")&&failure.includes("output_close"),"cancellation retains the actual diagnostic/output until their complete funded close");assertions++;
 console.log("[DEBUG] original raw invoke wholeCorpusSchema=true independentJSONPatch=true UTF8=true checks="+assertions);
 return assertions;
}
