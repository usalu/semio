import{expect,test}from"bun:test";
import{churnProtocolV1,churnTextV1}from"../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{nativeBuildJobStallV1,nativeBuildJobWaitSubjectV1,watchNativeBuildJobV1}from"../🟦️.ts";

const path="/w/🧵️job/⏱️context/🦀️.rs",retry=(language:string,attempt:number)=>churnTextV1(language,"retry",{path,reason:"one of its inputs changed",attempt,maximum:"∞",delay:30000});
const lease=(resource:string,elapsedMs:number)=>`[script] ${JSON.stringify({owner:"repository:build",command:"descriptor:build",stage:"running",activity:{resource,mode:"shared",elapsedMs}})}`;

test("every declared language of a churn retry notice names its path as the waited subject",()=>{for(const language of Object.keys(churnProtocolV1().texts))expect(nativeBuildJobWaitSubjectV1(retry(language,16))).toBe(path);});
test("lease activity counts as waiting only after it waited, and plain output is progress",()=>{expect(nativeBuildJobWaitSubjectV1(lease("semio.compiler.cpu/v1",1001))).toBe("semio.compiler.cpu/v1");expect(nativeBuildJobWaitSubjectV1(lease("cargo:foreign-source",0))).toBeUndefined();expect(nativeBuildJobWaitSubjectV1("   Compiling serde v1")).toBeUndefined();expect(nativeBuildJobWaitSubjectV1("[script] {not json")).toBeUndefined();});
test("a job that only repeats wait notices is refused with its waited resource after stallMs, and any progress clears it",()=>{
  const watch=watchNativeBuildJobV1(0),stall=900000;
  expect(watch.observe("[script] {\"stage\":\"running\"}",1000)).toBe("progress");
  for(let attempt=1;attempt<=40;attempt++)expect(watch.observe(retry("en",attempt),1000+attempt*30000)).toBe("wait");
  const now=1000+40*30000;
  expect(nativeBuildJobStallV1("describe-tool",watch,now,stall)).toBe(`Native build stalled: job:describe-tool made no progress for 1200 s and has only waited on ${path} since 1170 s`);
  expect(nativeBuildJobStallV1("describe-tool",watch,now,stall*2)).toBeUndefined();
  expect(watch.observe("   Compiling serde v1",now+1)).toBe("progress");
  expect(nativeBuildJobStallV1("describe-tool",watch,now+stall*3,stall)).toBeUndefined();
});
test("a job that never printed anything is not a waiter and a queued job names the lease it waits on",()=>{
  const silent=watchNativeBuildJobV1(0);expect(nativeBuildJobStallV1("quiet",silent,10**9,1)).toBeUndefined();
  const queued=watchNativeBuildJobV1(0);queued.observe(lease("semio.compiler.cpu/v1",1000),1000);queued.observe(lease("semio.compiler.cpu/v1",2000),2000);
  expect(nativeBuildJobStallV1("describe-tool",queued,901000,900000)).toContain("only waited on semio.compiler.cpu/v1");
});
