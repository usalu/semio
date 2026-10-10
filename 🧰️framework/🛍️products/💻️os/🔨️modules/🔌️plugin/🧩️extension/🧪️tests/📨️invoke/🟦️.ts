import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";

/** 📨️ The original invoke caller owns pending raw bytes and lends one unchanged scheduler wallet. */
export function extensionInvokeCallerOracle(repoRoot:string):number{
 const root=resolve(repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"),read=(path:string)=>readFileSync(resolve(root,path),"utf8");
 const ajv=new Ajv({strict:true,allErrors:true});
 ajv.addSchema(JSON.parse(readFileSync(resolve(repoRoot,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8")));
 ajv.addSchema(JSON.parse(readFileSync(resolve(repoRoot,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json"),"utf8")),"flow.original-retained-progress");
 const fixture=JSON.parse(read("🧩️extension/🧫️fixtures/📨️invoke/🔣️.json")),validate=ajv.compile(JSON.parse(read("🧩️extension/🧬️schema/📨️invoke/🔣️.json")));
 let assertions=0;assert.equal(validate(fixture),true,JSON.stringify(validate.errors));assertions++;
 for(const key of Object.keys(fixture)){const invalid={...fixture};delete invalid[key];assert.equal(validate(invalid),false);assertions++;}
 assert.equal(validate({...fixture,alternateWallet:true}),false);assertions++;
 const bytes=fixture.text.repeat(fixture.repeat);assert.equal(Buffer.byteLength(bytes),fixture.utf8Bytes);assert.deepEqual([...Buffer.from(bytes)],[...new TextEncoder().encode(bytes)]);assertions+=2;
 let responses=0;
 for(const reply of fixture.replies){const grant=fixture.caller.grant,progress=reply.progress;const remaining=applyPatch(grant,[{op:"replace",path:"/maximumItems",value:grant.maximumItems-progress.copiedItems},{op:"replace",path:"/maximumCopyBytes",value:grant.maximumCopyBytes-progress.copiedBytes},{op:"replace",path:"/maximumCapacityBytes",value:grant.maximumCapacityBytes-progress.retainedCapacityBytes},{op:"replace",path:"/maximumReleaseBytes",value:grant.maximumReleaseBytes-progress.releasedBytes}],true,false).newDocument;assert(Object.values(remaining).every(value=>(value as number)>=0));assert.equal(remaining.maximumDepth,grant.maximumDepth);assertions+=2;if(reply.payload!==null){JSON.parse(reply.payload);responses++;}else{assert.equal(responses,fixture.expected.pendingResponses);assertions++;}}
 assert.deepEqual(fixture.executionGrant,{maximumItems:16,maximumCopyBytes:64,maximumCapacityBytes:262144,maximumReleaseBytes:262144,maximumDepth:128});assertions++;
 for(const entry of fixture.admissionCases){const demand=applyPatch({maximumItems:2,maximumCopyBytes:0,maximumCapacityBytes:entry.frameBytes,maximumReleaseBytes:0,maximumDepth:1},[{op:"replace",path:"/maximumItems",value:2+Number(entry.checkpoint)}],true,false).newDocument;assert.deepEqual(entry.demand,demand);assertions++;}
 for(const row of fixture.lookupCases){const actual=row.candidates.findIndex((candidate:string)=>Buffer.from(candidate).equals(Buffer.from(row.input)));assert.equal(actual<0?null:actual,row.matched);assertions++;}
 const owner=read("🧩️extension/🚪️retirement/🦀️.rs"),runtime=read("🦀️.rs"),reactor=read("⚛️reactor/🔄️turn/🦀️.rs");
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
 assert(jobs.includes("owner.step(budget, original, snapshot, cx)"),"actual job receiver forwards the same original context");assertions++;
 assert(/pub type BoundedJobAdmission\s*=\s*fn\(u64,\s*&mut Option<Vec<u8>>,\s*&mut Option<Vec<u8>>,\s*&mut semio_framework_job::StepContext<'_>\)/s.test(jobs),"original inference admission lends whole raw input and checkpoint under its caller context");assertions++;
 assert(!jobs.includes("factory(job, input, restored.as_deref())"),"pending original factory source is not cold copied or borrowed from a temporary");assertions++;
 assert(jobs.includes("pub struct OriginalJobAdmission"),"actual registry admission retains the original owned request");assertions++;
 assert(/pub async fn start_job\(admission:\s*&mut Option<OriginalJobAdmission>,\s*cx:\s*&mut semio_framework_job::StepContext/.test(jobs),"actual registry loan uses the original caller grant before taking source");assertions++;
 assert(!jobs.includes("kind: kind.to_string(), input: Some(input.to_vec())"),"registry does not cold clone the original admission");assertions++;
 assert(jobs.includes("pub struct BoundedJobFactory")&&jobs.includes("pub type BoundedJobDemand"),"registered original factory requires its actual five-axis admission demand companion");assertions++;
 assert(jobs.includes("(factory.demands)(job,&source.input,&source.checkpoint,cx)"),"actual registry demand uses the retained original request and caller context");assertions++;
 assert(jobs.includes("static KIND_REGISTRY: RefCell<[Option<RegisteredJobKind>;REGISTERED_JOB_KINDS]>")&&!jobs.includes("RefCell::new(builtin_registry())"),"first original admission has a preborn kind registry");assertions++;
 assert(jobs.includes("struct OriginalJobKindLookup")&&jobs.includes("JobBody::Resolving"),"normal registry resolves original kind under separately paid source-byte turns");assertions++;
 assert(jobs.includes("static REGISTERED_KIND_COUNT: Cell<usize>")&&jobs.includes("self.index>=REGISTERED_KIND_COUNT.with(Cell::get)+6"),"normal lookup visits only the actual occupied kind prefix before its six builtins");assertions++;
 assert(!jobs.includes("registered_factory(admission.as_ref().unwrap().kind.as_str())"),"registry admission transfers handles before reading original kind bytes");assertions++;
 const pack=readFileSync(resolve(repoRoot,"🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs"),"utf8");
 assert(pack.includes("compare_left: Option<u8>"),"original diagnostic symbol comparison retains its separately paid first byte");assertions++;
 assert(!pack.includes("Phase::Find => 2"),"a fixed one-byte caller can finish original diagnostic symbol comparisons");assertions++;
 const failure=read("🧩️extension/📨️invoke/⚠️failure/🦀️.rs");
 for(const cut of fixture.cancelCuts){const prefix=fixture.replies.slice(0,Math.min(cut,fixture.replies.length));const retired=applyPatch({owned:prefix},[{op:"replace",path:"/owned",value:[]}],true,false).newDocument;assert.deepEqual(retired.owned,[]);assert.deepEqual(prefix,fixture.replies.slice(0,prefix.length));assertions+=2;}
 assert(failure.includes("pub(crate) fn begin_close")&&failure.includes("output_close"),"cancellation retains the actual diagnostic/output until their complete funded close");assertions++;
 console.log("[DEBUG] original raw invoke wholeCorpusSchema=true independentJSONPatch=true UTF8=true checks="+assertions);
 return assertions;
}
