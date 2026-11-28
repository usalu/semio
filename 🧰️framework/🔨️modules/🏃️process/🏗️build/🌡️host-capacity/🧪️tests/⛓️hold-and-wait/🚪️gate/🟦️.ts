/** 🚪️ Contract: the gate refuses a request that this host can never serve with an actionable message in both languages instead of queueing it for ever, and breaks a hold-and-wait between running grants through the preempt marker: the youngest batch grant is told through `preempted` of its handle, releases its memory and the growth of the other grant is admitted.
 * @see ../🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {evaluateHostCapacityV1,hostGrowthLabelV1,hostPreemptPathV1,newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../../🟦️.ts";
import base from "../../🧫️fixtures/🔣️.json";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"hold-and-wait-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,pollMilliseconds:10,progressMilliseconds:50,bypassMilliseconds:100,deadlockMilliseconds:300,unreachableMilliseconds:300,sampleMaximumAgeMilliseconds:0}} as never);
const host=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:16,totalMegabytes:16384,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:8000,openFilesMaximum:245760,attachedImages:0}) as never;
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(fixedMegabytes:number,more:object={}):HostRequestV1=>({id:newHostCapacityIdV1(),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1,...more}) as HostRequestV1;
const ports=(directory:string,available:number,lines:string[],language="en_US.UTF-8")=>({directory,policy,environment:{LANG:language},write:(line:string)=>{lines.push(line);},observe:async()=>null,sample:async()=>({host:host(available),swappedOutMegabytes:0})});

test("a request that needs more than the host could give even with nothing of ours running is refused after the unreachable time, in English and German",async()=>{
 for(const [language,text] of [["en_US.UTF-8","give out only"],["de_DE.UTF-8","nur"]] as const){
  const lines:string[]=[],started=Date.now();
  await expect(withHostCapacityV1({...ports(fresh(),5660,lines,language),request:request(6004,{minimumMegabytes:6004}),control:control()},async granted=>granted)).rejects.toThrow(text);
  expect(Date.now()-started).toBeGreaterThanOrEqual(policy.wait.unreachableMilliseconds!);
  expect(Date.now()-started).toBeLessThan(10_000);
 }
});

test("memory that other applications give back restarts the unreachable count: a request that fits is admitted, never refused",async()=>{
 const lines:string[]=[];let available=5660;
 const waiting=withHostCapacityV1({...ports(fresh(),0,lines),sample:async()=>({host:host(available),swappedOutMegabytes:0}),request:request(6004,{minimumMegabytes:6004}),control:control()},async granted=>granted);
 setTimeout(()=>{available=12000;},150);
 expect((await waiting).reservedMegabytes).toBe(6004);
});

test("two running grants that both wait for the growth of their work: the youngest is preempted, finishes its body and the other grows",async()=>{
 const directory=fresh(),lines:string[]=[],preempted:string[]=[],grown:string[]=[],available=12000;
 const spare=evaluateHostCapacityV1(policy,host(available),[1,2].map(at=>({id:`seed-grant-${at}`,operation:"command",jobs:1,cpu:1,reservedMegabytes:1536,admittedAtMs:Date.now(),observedMegabytes:null,pid:1,bootAtMs:0}) as HostGrantV1),[],Date.now()).headroomMegabytes;
 const extra=spare+600;let admitted=0,both:()=>void=()=>undefined;const together=new Promise<void>(done=>{both=done;});
 const job=(name:string)=>withHostCapacityV1({...ports(directory,available,lines),request:request(1536,{label:name}),control:control()},async handle=>{
  if(++admitted===2)both();
  await together;
  const stop=new AbortController(),growing=withHostCapacityV1({...ports(directory,available,lines),request:request(extra,{operation:"cargo-growth",label:hostGrowthLabelV1(handle.id)}),control:control(stop.signal)},async()=>{grown.push(name);}).catch(()=>undefined);
  await new Promise<void>(done=>{handle.preempted!.addEventListener("abort",()=>{preempted.push(name);stop.abort(new Error("preempted"));done();},{once:true});void growing.then(()=>done());});
  await growing;
  return handle.id;
 });
 const first=job("job-first"),pause=await new Promise(done=>setTimeout(done,60)),second=job("job-second");
 void pause;
 const [a,b]=await Promise.all([first,second]);
 expect(preempted).toEqual(["job-second"]);
 expect(grown).toEqual(["job-first"]);
 expect(existsSync(hostPreemptPathV1(directory,b)),"the marker goes with the grant").toBe(false);
 expect(a).not.toBe(b);
 expect(lines.some(line=>line.includes("preempting job-second"))).toBe(true);
 expect(lines.some(line=>line.includes("was preempted")||line.includes("preempted after"))).toBe(true);
});
