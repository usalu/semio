import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PublicationOutcomeCursor,type PublicationLane} from "../🟦️.ts";
import faultCustodyCorpus from "../⚠️fault/🧫️fixtures/🔣️.json";
import faultCustodySchema from "../⚠️fault/🧬️schema/🔣️.json";
import {MountedFaultCustody} from "../⚠️fault/🟦️.ts";

test("original mounted close faults remain owned across refused admission and physical frame closure",()=>{
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),host=readFileSync(join(import.meta.dir,"../../../../🦀️.rs"),"utf8");
 expect(new Ajv({strict:false}).compile(faultCustodySchema)(faultCustodyCorpus)).toBe(true);
 expect(source).toContain("store::artifact_retirement_owned_birth_demands(&self.retained_close_fault)");expect(source).toContain("store::artifact_retirement_admit_owned(&mut self.retained_close_fault,&mut self.retained_close_fault_retirement,grant)");expect(source).toContain("store::artifact_retirement_box_close_step(&mut self.retained_close_fault_retirement,grant)");expect(source).toContain("self.retained_close_fault_refusal=Some(original)");expect(host).toContain("&& self.retained_close_fault_retirement.is_none()");expect(host).toContain("&& self.retained_close_fault_refusal.is_none()");
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
 const host=readFileSync(join(root,"🦀️.rs"),"utf8"),frontier=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");
 const start=host.indexOf("fn retirement_step(&mut self, grant: RetainedCloneGrant)",host.indexOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation"));
 expect(start).toBeGreaterThanOrEqual(0);const body=host.slice(start,host.indexOf("fn terminal_is_empty",start));
 expect(body).not.toContain("artifact_mutations.pop()");expect(body).not.toContain("drop(self.completion.take())");expect(body).not.toContain("drop(chunk)");
 expect(frontier).toContain("A::admit_completion_retirement(&mut self.publication,grant)");
 expect(frontier).toContain("self.completion=Some(original)");
 expect(frontier).toContain("release_bytes:owner.frame_release_bytes()");
 expect(frontier).toContain("if owner.terminal_is_empty(){let bytes=owner.frame_release_bytes()");
 for(const axis of ["copy_bytes","capacity_bytes","release_bytes","depth"]){expect(frontier).toContain("grant.maximum_"+axis+"<demand."+axis);}
 const ladder=readFileSync(join(root,"🪜️close-ladder/🦀️.rs"),"utf8");expect(ladder).toContain("CloseRung::DirectIngress => self.window_config_store.close_direct_ingress(grant)");expect(ladder).toContain("CloseRung::GrantedMounted => self.granted_mounted_retirement_step(grant)");
 const publicationStart=host.indexOf("impl<A: ArtifactApp> PendingArtifactStorePublication<A>");
 const end=/\r?\n    }\r?\n/.exec(host.slice(publicationStart));expect(end).not.toBeNull();const publication=host.slice(publicationStart,publicationStart+end!.index+end![0].length);
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
test("mounted publication source captures before close and delivers only after original frame release",()=>{
 const root=join(import.meta.dir,"../../../.."),host=readFileSync(join(root,"🦀️.rs"),"utf8"),frontier=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");
 const capture=frontier.indexOf("PendingPublicationDisposition::Rejected"),close=frontier.indexOf("owner.close_step(grant)");expect(capture).toBeLessThan(close);
 expect(frontier).toContain("self.terminal_fault.get_or_insert(report)");expect(frontier).toContain("self.pending_publication_outcome.phase=PendingPublicationOutcomePhase::Released");expect(frontier).toContain("owner.lane().0==TypedOperationResultLane::WindowTransient");
 const publisher=host.slice(host.indexOf("async fn publish_mounted_typed_operation_unit"),host.indexOf("let live_revision",host.indexOf("async fn publish_mounted_typed_operation_unit")));
 expect(publisher.indexOf("mounted.pending_publication_outcome.pending()")).toBeLessThan(publisher.indexOf("mounted.reject_cancelled_publication()"));expect(host).toContain("&& !self.pending_publication_outcome.pending()");
});
