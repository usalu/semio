/** 🧯️ Contract: a stage on a low-resource machine never waits for ever for a grant the machine cannot give. What a run teaches is bounded (a failed, cancelled or pressured run teaches nothing; a peak counts at most as the host share and the class ceiling), a learned 200 % request is shrunk to the room the host has when the room covers the operation's declared minimum, and a request that fits no shrink is refused with a named error past `wait.starvedMilliseconds`; hosts of 8, 16 and 32 GB against an independent Python oracle. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {emptyHostEstimateStoreV1,estimateHostPhaseV1,evaluateHostCapacityV1,hostClassCeilingV1,formatHostCapacityErrorV1,hostFloorMegabytesV1,hostLearnCeilingV1,learnableHostRunV1,newHostCapacityIdV1,readHostCapacityPolicyV1,recordHostPhasesV1,scaleHostRequestV1,sizeHostRequestV1,withHostCapacityV1,type HostGrantV1,type HostRequestV1,type HostRunStatusV1,type HostSampleV1} from "../../🟦️.ts";
import library from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";

const{interactiveReserveBasisPoints:_a,interactiveReserveMinimumMegabytes:_b,interactiveReserveMaximumMegabytes:_c,sessionCapBasisPoints:_d,...plainMemory}=library.hostCapacity.memory as Record<string,number>;
const plain={...library.hostCapacity,memory:plainMemory},policy=readHostCapacityPolicyV1(plain),now=1_800_000_000_000;
const sample=(totalMegabytes:number,availableMegabytes:number,pressure:"normal"|"critical"="normal"):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes,availableMegabytes,pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const learned=(total:number):HostRequestV1=>{
 const capacity=total-hostFloorMegabytesV1(policy,{totalMegabytes:total}),base=sizeHostRequestV1(policy,{id:"aaaaaaaa1",operation:"command",host:{totalMegabytes:total}})!;
 return scaleHostRequestV1(policy,base,policy.estimates.maximumRatioBasisPoints,capacity);
};

test("The most one learned peak may count and the ratio a run of 6163 or 20000 MB is stored at equal the Python oracle on 8, 16 and 32 GB hosts",()=>{
 for(const row of fixture.ceilings){
  const ceiling=hostLearnCeilingV1(policy,row.capacityMegabytes,row.declared??undefined);
  if("expect" in row)expect(ceiling,JSON.stringify(row)).toBe(row.expect);
  if("expectRatio" in row){
   const store=recordHostPhasesV1(policy,emptyHostEstimateStoreV1(),["command"],"command",row.default!,{jobs:1,peakMegabytes:row.peak!,activeMilliseconds:1000,peakAtMilliseconds:500,endedAtMs:now,ceilingMegabytes:ceiling,keepPeak:true});
   expect(store.classes["command"]!.samples[0]!.ratioBasisPoints,JSON.stringify(row)).toBe(row.expectRatio);
  }
 }
 expect(new Set(fixture.ceilings.filter(row=>"expect" in row).map(row=>row.expect)).size).toBeGreaterThan(3);
});

test("Only a completed run on a calm host teaches; a run the gate stopped at its limit teaches its grown peak; every other run teaches nothing",()=>{
 for(const row of fixture.learns)expect(learnableHostRunV1(row.status as HostRunStatusV1,{limited:row.limited,pressured:row.pressured}),JSON.stringify(row)).toBe(row.expect);
 expect(fixture.learns.some(row=>row.expect)&&fixture.learns.some(row=>!row.expect)).toBe(true);
});

test("A learned 200 % request is shrunk to the room the host has when the room covers the declared minimum, waits otherwise and is refused past the starved bound, as the Python oracle says on 8, 16 and 32 GB hosts",()=>{
 const reached=new Set<string>();
 for(const row of fixture.admissions){
  const request=learned(row.totalMegabytes),waiter={request,arrivedAtMs:now-row.waited,pid:2},holder:HostGrantV1={id:"bbbbbbbb1",operation:"command",jobs:1,cpu:1,reservedMegabytes:row.holder,admittedAtMs:now-3_600_000,observedMegabytes:row.holder,pid:3,bootAtMs:0};
  const decision=evaluateHostCapacityV1(policy,sample(row.totalMegabytes,row.available,row.hard?"critical":"normal"),[holder],[waiter as never],now).decisions[0]!;
  expect([decision.verdict,decision.reservedMegabytes,decision.reason],JSON.stringify(row)).toEqual(row.expect);
  reached.add(String(row.expect[2]));
 }
 expect(reached).toEqual(new Set(["fits","shrunk","memory","pressure-critical"]));
 expect(policy.operations.find(row=>row.name==="command")!.minimumMegabytes).toBeLessThan(policy.operations.find(row=>row.name==="command")!.fixedMegabytes);
});

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"progress-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const quick=readHostCapacityPolicyV1({...plain,wait:{...library.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,deadlineBatchMilliseconds:1000,deadlineInteractiveMilliseconds:1000,starvedMilliseconds:2500,bypassMilliseconds:600000}});
const asked=(fixedMegabytes:number,minimumMegabytes?:number):HostRequestV1=>({id:newHostCapacityIdV1(),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1,...minimumMegabytes===undefined?{}:{minimumMegabytes}} as HostRequestV1);

test("A request that can never fit ends with a named error, a request that fits the host but not the room keeps waiting, and a shrinkable one is admitted with what the host has",async()=>{
 const directory=fresh(),lines:string[]=[],base={directory,policy:quick,control:control(),sample:async()=>({host:sample(16384,3000),swappedOutMegabytes:0}),observe:()=>Promise.resolve(600)};
 await withHostCapacityV1({...base,request:{...asked(600),label:"holder"},environment:{LANG:"en_US.UTF-8"},write:()=>undefined},async()=>{
  const shrunk=await withHostCapacityV1({...base,request:asked(4096,300),environment:{LANG:"en_US.UTF-8"},write:line=>lines.push(line)},async handle=>handle.reservedMegabytes);
  expect(shrunk).toBeGreaterThanOrEqual(300);expect(shrunk).toBeLessThanOrEqual(3000-2048);
  const never=await withHostCapacityV1({...base,request:asked(40000,30000),environment:{LANG:"en_US.UTF-8"},write:()=>undefined},async()=>"").catch((error:Error)=>error.message);
  expect(never).toContain("command needs at least 30000 MB");
  const abort=new AbortController(),outcome=await Promise.race([withHostCapacityV1({...base,control:{...control(),signal:abort.signal},request:asked(4096,3000),environment:{LANG:"en_US.UTF-8"},write:()=>undefined},async()=>"admitted").catch(()=>"stopped"),new Promise<string>(done=>setTimeout(()=>{abort.abort();done("still waiting");},1500))]);
  expect(outcome,"a request that fits the host but not the room of the moment keeps waiting, it is never refused").toBe("still waiting");
 });
},60_000);

test("The wait-out error exists in English and German and names the holders",()=>{
 const values={operation:"command",minimum:3000,total:16384,floor:2048,capacity:14336,needed:5048,spare:952,seconds:1800,holders:"--language (running, 1 job(s), 3054 MB)"};
 expect(formatHostCapacityErrorV1("en","memoryWaitedOut",values)).toContain("held by --language (running, 1 job(s), 3054 MB)");
 expect(formatHostCapacityErrorV1("de","memoryWaitedOut",values)).toContain("belegt von --language (running, 1 job(s), 3054 MB)");
});

test("A learned assembly or plan size never exceeds the authored phase maximum, however high the stored peaks, and a dry run class has a small declared ceiling, as the Python oracle says",()=>{
 for(const row of fixture.phases){
  let store=emptyHostEstimateStoreV1();
  for(let index=0;index<4;index++)store=recordHostPhasesV1(policy,store,["run"],"run",1024,{jobs:1,peakMegabytes:1,activeMilliseconds:1000,peakAtMilliseconds:500,endedAtMs:now-index,phases:[{phase:"plan",startMilliseconds:0,activeMilliseconds:1000,peakMegabytes:row.peak,peakAtMilliseconds:500}]});
  expect(estimateHostPhaseV1(policy,store,"plan",4096,now)!.megabytes,JSON.stringify(row)).toBe(row.expect);
 }
 expect(fixture.phases.some(row=>row.expect===policy.estimates.maximumPhaseMegabytes)).toBe(true);
 for(const row of fixture.classes)expect(hostClassCeilingV1(policy,row.key)??null,row.key).toBe(row.expect);
 expect(hostClassCeilingV1(policy,"run:dry")).toBeLessThanOrEqual(2304);
});
