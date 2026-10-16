import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PublicationOutcomeCursor,type PublicationLane} from "../🟦️.ts";
import faultCustodyCorpus from "../⚠️fault/🧪️tests/🧫️fixtures/🔣️.json";
import faultCustodySchema from "../⚠️fault/🧬️schema/🔣️.json";
import {MountedFaultCustody} from "../⚠️fault/🟦️.ts";
import { familyText } from "../../../../🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";

test("member replay and cancellation borrow the actual original context through both parent paths",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);expect(corpus.originalClose.memberPaths).toEqual(["storeReplacement","privateChildGroup"]);
 const db=new Database(":memory:");try{expect(db.query("select ? as debit, ? as paths").get(1,corpus.originalClose.memberPaths.length)).toEqual({debit:1,paths:2});}finally{db.close();}
 const root=join(import.meta.dir,"../../../.."),document=readFileSync(join(root,"🗃️document/🦀️.rs"),"utf8"),start=document.indexOf("fn drive_member_open("),member=document.slice(start,document.indexOf("fn ",start+40));
 expect(member).toContain("cx:&mut semio_framework_job::StepContext<'_>");expect(member).not.toContain("StepContext::new");expect(member).not.toContain("StepBudget::new");
 const closeStart=document.indexOf("fn drive_rejected_members("),close=document.slice(closeStart,document.indexOf("fn drive_initializer",closeStart));
 expect(close).toContain("MemberOpenOperation::close_step(open,same)");expect(close).toContain("with_retained_loan(child");expect(close).not.toContain("plugin_sdk_fault(error.into_message())");
 const group=readFileSync(join(root,"🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs"),"utf8"),retireStart=group.indexOf("fn retire_open("),retire=group.slice(retireStart,group.indexOf("fn make_lane",retireStart));
 expect(retire).toContain("cx:&mut StepContext<'_>");expect(retire).toContain("open.close_step(same)");expect(retire).toContain("with_retained_loan(grant");expect(retire).not.toContain("into_message");
 console.log("[DEBUG] Ajv/SQLite original Member store-replacement and private-group paths require one real live recipient debit and preserve full close failure without cold context or demand-derived fuel");
});

test("a paid ToolRun turn stops before any other publication family",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);expect(corpus.originalClose.singleFamilyPerLiveTurn).toBe(true);
 const db=new Database(":memory:");try{const expected=db.query("select ?-? as paid, ? as families").get(7,0,1);expect(expected).toEqual({paid:7,families:1});}finally{db.close();}
 const source=[["🖼️render/🦀️.rs","🧷️command-routes/🦀️.rs","🧾️typed-operations/🦀️.rs","🔌️plugin-app-impl/🦀️.rs"].map.map(s=>readFileSync(join(import.meta.dir,"../../../../🧵️instance/"+s),"utf8")).join("\n"),familyText()].join("\n"),start=source.indexOf("async fn advance_typed_operation_publication("),body=source.slice(start,source.indexOf("fn take_typed_operation_preparation_receipt",start));
 const extension=familyText(),owner=extension.indexOf("impl InstanceExtension for ToolRunExtension"),tool=extension.slice(extension.indexOf("async fn drive_turn",owner),extension.indexOf("fn render_body",owner));
 const registry=readFileSync(join(import.meta.dir,"../../../../🧵️instance/🧩️extensions/🦀️.rs"),"utf8"),chain=registry.slice(registry.indexOf("impl<First: InstanceExtension, Rest: InstanceExtension>"));
 expect(start).toBeGreaterThanOrEqual(0);expect(owner).toBeGreaterThanOrEqual(0);expect(tool).toContain("let before = cx.retained_progress()");
 expect(tool).toContain("cx.retained_progress() != before");
 expect(tool).toContain("return Ok(true)");
 expect(body).toContain("let mut retained=cx.retained_grant()");
 expect(body).toContain("if InstanceExtensions::drive_turn(self, &mut retained, identity, cx).await?");
 expect(chain).toContain("*retained = cx.retained_grant()");
 console.log("[DEBUG] SQLite one-family receiving reference requires actual posted ToolRun effects to stop before a stale raw time-travel/local-query loan");
});

test("mounted retirement requires original caller and preserves both complete failure owners",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const law=corpus.originalClose,db=new Database(":memory:");
 try{db.exec("create table original_failure(role text primary key,owner text)");db.query("insert into original_failure values (?,?)").run("primary",JSON.stringify(law.primary));db.query("insert into original_failure values (?,?)").run("receiptRefusal",JSON.stringify(law.receiptRefusal));expect(db.query("select count(*) as owners from original_failure").get()).toEqual({owners:2});for(const role of ["primary","receiptRefusal"]as const){const row=db.query("select owner from original_failure where role=?").get(role)as{owner:string};expect(JSON.parse(row.owner)).toEqual(law[role]);}}
 finally{db.close();}
 const path=join(import.meta.dir,"../../../../🧵️instance/🔄️full-operation/🦀️.rs"),source=readFileSync(path,"utf8"),start=source.indexOf("fn granted_mounted_retirement_step(");
 expect(start).toBeGreaterThanOrEqual(0);const body=source.slice(start);
 expect(body).toContain("cx:&mut semio_framework_job::StepContext<'_>");
 expect(body).toContain("Result<RetainedCloneStep,crate::PluginLifecycleFailure>");
 expect(body).toContain("self.retire_typed_operation_unit(id,cx)");
 expect(body).toContain("owner.close_step(cx)");
 expect(body).toContain("owner.retirement_step(cx)");
 expect(body).not.toContain("retained_close_fault=Some(original)");
 expect(body).not.toContain("Err(ValueError::literal");
 console.log("[DEBUG] SQLite preserves complete primary/source/physical receipt and independent receiving refusal; actual mounted retirement must borrow original context and forward both owners");
});

test("original mounted close faults remain owned across refused admission and physical frame closure",()=>{
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),host=familyText();
 expect(new Ajv({strict:false}).compile(faultCustodySchema)(faultCustodyCorpus)).toBe(true);
 expect(source).toContain("store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.retained_close_fault)");expect(source).toContain("store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.retained_close_fault,&mut self.retained_close_fault_retirement,grant)");expect(source).toContain("store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retained_close_fault_retirement,grant)");expect(source).toContain("self.retained_close_fault_refusal=Some(original)");expect(host).toContain("&& self.retained_close_fault_retirement.is_none()");expect(host).toContain("&& self.retained_close_fault_refusal.is_none()");
 for(const row of faultCustodyCorpus.cases){
  const original={origin:row.origin,code:row.code,message:row.message};const cursor=new MountedFaultCustody(original,row.existingCode,64,488),grant={items:1,copy:488,capacity:64,release:64,depth:2};let oracle={original,active:null as typeof original|null,report:row.existingCode as string|null};
  expect(cursor.capture({...grant,copy:0},v=>v.code)).toBe(row.existingCode!==null);expect(cursor.source).toBe(original);expect(cursor.capture(grant,v=>v.code)).toBe(true);oracle=jsonPatch.applyPatch(oracle,[{op:"replace",path:"/report",value:row.existingCode??row.code}]).newDocument;
  expect(cursor.admit({...grant,capacity:63},()=>true)).toBe(false);expect(cursor.admit(grant,()=>false)).toBe(false);expect(cursor.source).toBe(original);expect(cursor.admit(grant,()=>true)).toBe(true);oracle=jsonPatch.applyPatch(oracle,[{op:"move",from:"/original",path:"/active"}]).newDocument;expect(cursor.source).toBe(oracle.active);
  expect(cursor.releaseFrame(grant)).toBe(false);expect(cursor.observeBodyTerminal()).toBe(true);expect(cursor.releaseFrame({...grant,release:63})).toBe(false);expect(cursor.releaseFrame(grant)).toBe(true);oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/active"}]).newDocument;expect(cursor.pending).toBe(false);expect(cursor.report).toBe(oracle.report);
  const refused=new MountedFaultCustody(original,row.existingCode,64,488);refused.capture(grant,v=>v.code);refused.admit(grant,()=>true);refused.retainRefusal(original);expect(refused.observeBodyTerminal()).toBe(false);expect(refused.releaseFrame(grant)).toBe(false);expect(refused.pending).toBe(true);expect(refused.source).toBe(original);
 }
});

test("mounted publications keep original payload and allocation through a genuine caller grant",()=>{
 const root=join(import.meta.dir,"../../../..");
 const host=familyText(),frontier=[["../🦀️.rs"].map.map(file=>readFileSync(file,"utf8")).join("\n"),familyText()].join("\n");
 const start=host.indexOf("fn retirement_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>)",host.indexOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation"));
 expect(start).toBeGreaterThanOrEqual(0);const body=host.slice(start,host.indexOf("fn terminal_is_empty",start));
 expect(body).not.toContain("artifact_mutations.pop()");expect(body).not.toContain("drop(self.completion.take())");expect(body).not.toContain("drop(chunk)");
 expect(frontier).toContain("A::admit_completion_retirement(&mut self.publication,grant)");
 expect(frontier).toContain("self.completion=Some(original)");
 expect(frontier).toContain("release_bytes:owner.frame_release_bytes()");
 expect(frontier).toContain("if owner.terminal_is_empty(){let bytes=owner.frame_release_bytes()");
 for(const axis of ["copy_bytes","capacity_bytes","release_bytes","depth"]){expect(frontier).toContain("grant.maximum_"+axis+"<demand."+axis);}
 const ladder=familyText();expect(ladder).toContain("CloseRung::DirectIngress=>self.window_config_store.close_direct_ingress(grant)");expect(ladder).toContain("CloseRung::GrantedMounted=>self.granted_mounted_retirement_step(cx)");
 const owner=readFileSync(join(root,"🧩️composition/📬️publication/🦀️.rs"),"utf8"),publicationStart=owner.indexOf("impl<A: ArtifactApp> PendingArtifactStorePublication<A>");
 const end=/\r?\n}\r?\n/.exec(owner.slice(publicationStart));expect(end).not.toBeNull();const publication=owner.slice(publicationStart,publicationStart+end!.index+end![0].length);
 expect(publication).toContain("fn close_step(&mut self, grant:RetainedCloneGrant)");
 expect(publication).not.toContain("maximum_bytes");
});
test("publication rejection survives body and frame retirement and emits one original fault outcome",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.cases){
  const cursor=new PublicationOutcomeCursor(row.lane as PublicationLane,row.reason,488);const events:string[]=[];const source={reason:row.reason,body:{candidate:"private"},frame:{bytes:64}};let oracle:Record<string,unknown>=structuredClone(source);
  expect(cursor.capture(0,488,1)).toBe(false);expect(cursor.capture(1,487,1)).toBe(row.lane==="windowTransient");if(cursor.state==="uncaptured")expect(cursor.capture(1,488,1)).toBe(true);events.push("capture");
  expect(cursor.releaseFrame(false,64,1,64,1)).toBe(false);expect(source.reason).toBe(row.reason);
  oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/reason"},{op:"remove",path:"/body"}]).newDocument;events.push("body");
  expect(cursor.deliver(row.existingFault,1,488,1).ready).toBe(false);expect(cursor.releaseFrame(true,64,1,63,1)).toBe(false);expect(cursor.releaseFrame(true,64,1,64,1)).toBe(true);oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/frame"}]).newDocument;expect(oracle).toEqual({});events.push("frame");
  if(cursor.nextCopyBytes>0)expect(cursor.deliver(row.existingFault,1,487,1).ready).toBe(false);const outcome=cursor.deliver(row.existingFault,1,488,1);expect(outcome.ready).toBe(true);events.push("deliver");expect(events).toEqual(row.transitions);
  const fault=outcome.fault===null?null:row.existingFault?row.existingFault:(JSON.parse(outcome.fault) as {code:string;message:string}).code;expect(fault).toBe(row.reportCode);expect(Number(outcome.fault!==null)).toBe(row.faultTerminals);if(outcome.fault!==null&&row.existingFault===null)expect((JSON.parse(outcome.fault) as {message:string}).message).toBe(row.reason);expect(cursor.deliver(row.existingFault,1,488,1).ready).toBe(false);
 }
});
test("mounted publication rejection queues its one original fault page, then retires the original publication only after the ACK",()=>{
 const root=join(import.meta.dir,"../../../.."),host=familyText(),frontier=[["../🦀️.rs"].map.map(file=>readFileSync(file,"utf8")).join("\n"),familyText()].join("\n");
 const unit=host.indexOf("fn publish_mounted_typed_operation_unit"),publisher=host.slice(unit,host.indexOf("let live_revision",unit));expect(unit).toBeGreaterThan(0);
 expect(publisher).toContain("if mounted.reject_cancelled_publication()? {");expect(publisher.indexOf("mounted.reject_cancelled_publication()")).toBeLessThan(publisher.indexOf("try_claim_publication"));
 const start=host.indexOf("fn reject_cancelled_publication"),reject=host.slice(start,host.indexOf("fn ",start+40));expect(start).toBeGreaterThan(0);
 expect(reject).toContain("match self.terminal_fault.as_ref()");expect(reject).toContain("pending.begin_close();");expect(reject.indexOf("pending.begin_close();")).toBeLessThan(reject.indexOf("self.queue_page(page)?"));
 expect(reject).toContain("TypedOperationResultLane::Fault");expect(reject).not.toContain("terminal_fault.take()");expect(reject).not.toContain("terminal_fault=");
 expect(frontier).toContain("fn granted_retirement_ready(&self)->bool{self.result_page.is_none()&&");
 expect(frontier).toContain("self.pending_artifact_publication.as_ref().is_some_and(PendingArtifactStorePublication::is_closing)");
 expect(host).toContain("&& self.pending_artifact_publication.is_none()");
});
