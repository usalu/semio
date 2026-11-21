import {registeredToolExecutableV1} from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,markHostCapacityPhaseV1,newHostCapacityIdV1,readHostCapacityPolicyV1,requestHostRebuildCapacityV1,withHostCapacityV1,withUnmanagedHostV1,type HostCapacityOptionsV1,type HostGrantV1,type HostPhaseSegmentV1,type HostRunV1,type HostSampleV1,type HostWaiterV1} from "../../../🟦️.ts";
import {Database} from "bun:sqlite";
import {ownsHostRebuildAskV1,ownsQueuedHostRebuildV1,processStartV1} from "../../../🎫️gate/🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";
import rebuildFixture from "./🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hp-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const entry=fileURLToPath(new URL("../../../🟦️.ts",import.meta.url));
const grantRow=(directory:string,id:string)=>JSON.parse(readFileSync(join(directory,"grants",`${id}.json`),"utf8")) as HostGrantV1;
const until=async(check:()=>boolean):Promise<void>=>{for(let poll=0;poll<250;poll++){if(check())return;await sleep(20);}throw Error("Expected capacity ledger observation was not published");};
const queued=(directory:string):HostWaiterV1[]=>readdirSync(join(directory,"queue")).filter(file=>file.endsWith(".json")).flatMap(file=>{
 try{return[JSON.parse(readFileSync(join(directory,"queue",file),"utf8")) as HostWaiterV1];}catch{return[];}
});

test("a rebuild phase observation alone cannot increase a full grant behind an earlier waiter",async()=>{
 const directory=fresh(),abort=new AbortController(),sample=async()=>({host:{...host(),availableMegabytes:10240},swappedOutMegabytes:0});
 const owner=options(directory,{request:request({fixedMegabytes:4096}),observe:async()=>3072,resize:phase=>phase==="rebuild"?{reservedMegabytes:14336,rampMilliseconds:30000}:undefined});
 await withHostCapacityV1(owner,async handle=>{
  const earlier=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:8192}),control:{...control(),signal:abort.signal},sample}),async()=>undefined).catch(()=>undefined);
  try{
   for(let poll=0;poll<40&&readdirSync(join(directory,"queue")).length===0;poll++)await sleep(20);
   expect(readdirSync(join(directory,"queue"))).toHaveLength(1);
   const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
   expect(markHostCapacityPhaseV1(environment,"rebuild")).toBe(true);await sleep(150);
   expect(grantRow(directory,handle.id).reservedMegabytes).toBe(4096);
  }finally{abort.abort();await earlier;}
 });
},30000);

test("a full rebuild replacement lends before custody and books the exact footprint after the earlier waiter",async()=>{
 const directory=fresh(),stop=new AbortController(),floor=hostFloorMegabytesV1(policy,host());
 let available=24000,waiting=false,earlierRunning=false,booked=false;let releaseEarlier:()=>void=()=>undefined;
 const sample=async()=>({host:{...host(),availableMegabytes:available},swappedOutMegabytes:0});
 const fork={...control(),signal:stop.signal},growPolicy={...policy,wait:{...policy.wait,lendAfterMilliseconds:100}};
 const earlierRequest=request({fixedMegabytes:8192}),owner=options(directory,{policy:growPolicy,request:request({fixedMegabytes:4096}),sample,observe:async()=>3072,waited:()=>waiting?20000:0,children:()=>[],resize:phase=>phase==="rebuild"?{reservedMegabytes:14336,rampMilliseconds:30000}:undefined});
 await withHostCapacityV1(owner,async handle=>{
  await until(()=>grantRow(directory,handle.id).observedMegabytes===3072);
  available=floor+8500;
  const earlier=withHostCapacityV1(options(directory,{policy:growPolicy,request:earlierRequest,control:fork,sample,observe:async()=>0}),async()=>{
   earlierRunning=true;await new Promise<void>(done=>{releaseEarlier=done;});
  });
  let rebuild:Promise<(()=>void)|undefined>|undefined,releaseRebuild:(()=>void)|undefined;
  try{
   await until(()=>queued(directory).some(row=>row.request.id===earlierRequest.id));
   const arrival=queued(directory).find(row=>row.request.id===earlierRequest.id)!.arrivedAtMs;
   const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
   waiting=true;rebuild=requestHostRebuildCapacityV1(environment,fork).then(value=>{booked=value!==undefined;releaseRebuild=value;return value;});
   await until(()=>queued(directory).some(row=>row.rebuild));
   const replacement=queued(directory).find(row=>row.rebuild)!;
   expect(replacement.arrivedAtMs).toBeGreaterThan(arrival);expect(replacement.request.fixedMegabytes).toBe(14336);
   const holder=grantRow(directory,handle.id),owns=(pid:number,start:string)=>pid===process.pid&&start===processStartV1(pid);
   expect(ownsQueuedHostRebuildV1(directory,holder,replacement,owns)).toBe(true);
   expect(ownsQueuedHostRebuildV1(directory,holder,{...replacement,arrivedAtMs:replacement.arrivedAtMs+1},owns)).toBe(false);
   expect(ownsQueuedHostRebuildV1(directory,holder,replacement,()=>false)).toBe(false);
   await until(()=>earlierRunning);expect(grantRow(directory,handle.id).lentSinceMs).toBeDefined();expect(grantRow(directory,handle.id).observedMegabytes).toBe(3072);expect(booked).toBe(false);
   releaseEarlier();await earlier;available=24000;
   await expect(rebuild).resolves.toBeFunction();waiting=false;
   const admitted=grantRow(directory,handle.id);expect(admitted.reservedMegabytes).toBe(14336);expect(admitted.lentSinceMs).toBeUndefined();expect(admitted.phase).toBeUndefined();
   expect(ownsQueuedHostRebuildV1(directory,admitted,replacement,owns)).toBe(false);
   expect(ownsHostRebuildAskV1(directory,admitted,replacement.arrivedAtMs,owns)).toBe(true);
   expect(markHostCapacityPhaseV1(environment,"rebuild")).toBe(true);await until(()=>grantRow(directory,handle.id).phase==="rebuild");
   expect(grantRow(directory,handle.id).resumedAtMs).toBe(admitted.resumedAtMs);expect(queued(directory)).toEqual([]);
  }finally{waiting=false;stop.abort();releaseEarlier();await Promise.allSettled([earlier,...rebuild===undefined?[]:[rebuild]]);releaseRebuild?.();}
 });
},30000);

test("a lent owner replacement credits its resident memory exactly once and ignores stale funding scopes",()=>{
 const database=new Database(":memory:"),now=Date.now(),strict=readHostCapacityPolicyV1({...policy,memory:{...policy.memory,floorBasisPoints:1000,floorMinimumMegabytes:1000,floorMaximumMegabytes:1000,budgetBasisPoints:10000}});
 try{for(const row of rebuildFixture.lentReplacement){
  const sample={...host(),totalMegabytes:row.totalMegabytes,availableMegabytes:row.availableMegabytes},id="rebuild-owner";
  const grant:HostGrantV1={id,operation:"show",jobs:1,cpu:1,reservedMegabytes:row.observedMegabytes,observedMegabytes:row.observedMegabytes,admittedAtMs:now-1000,lentSinceMs:now-100,pid:1234,bootAtMs:1,rebuildScope:"funding-current"};
  const waiter:HostWaiterV1={request:request({id,fixedMegabytes:row.requiredMegabytes}),arrivedAtMs:now-50,pid:grant.pid,upgrade:true,rebuild:row.current?grant.rebuildScope:"funding-old"};
  expect(grant.rebuildScope).toBeDefined();
  const expected=database.query("select case when ?1=0 then 'stale' when ?2>?3-?4 then 'never' when ?2-?5>?6-?4 then 'memory' else 'fits' end as reason").get(Number(row.current),row.requiredMegabytes,row.totalMegabytes,hostFloorMegabytesV1(strict,sample),row.observedMegabytes,row.availableMegabytes)as{reason:string};
  const result=evaluateHostCapacityV1(strict,withUnmanagedHostV1(sample,[grant]),[grant],[waiter],now),mine=result.decisions[0];
  expect(expected.reason).toBe(row.expected);expect(mine?.reason??"stale").toBe(row.expected);if(row.expected==="fits")expect(mine?.reservedMegabytes).toBe(row.requiredMegabytes);
 }}finally{database.close();}
});

test("two funded boundaries retain one full reservation until the last exact receipt leaves",async()=>{
 const directory=fresh();
 const sample=async()=>({host:{...host(),availableMegabytes:24000},swappedOutMegabytes:0});
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:4096}),sample,resize:phase=>({reservedMegabytes:phase==="rebuild"?14336:4096,rampMilliseconds:30000})}),async handle=>{
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  const bounded={...control(),signal:AbortSignal.timeout(5000)},[first,second]=await Promise.all([requestHostRebuildCapacityV1(environment,bounded),requestHostRebuildCapacityV1(environment,bounded)]);
  try{
   expect(first).toBeFunction();expect(second).toBeFunction();expect(first).not.toBe(second);
   const admitted=grantRow(directory,handle.id);expect(admitted.reservedMegabytes).toBe(14336);expect(queued(directory)).toEqual([]);
   const folder=join(directory,"grants",`${handle.id}.rebuild.${admitted.rebuildScope}`);
   const receipts=readdirSync(folder).filter(name=>name.endsWith(".receipt"));expect(receipts).toHaveLength(2);
   const proofs=receipts.map(name=>JSON.parse(readFileSync(join(folder,name),"utf8")));expect(new Set(proofs.map(row=>row.id)).size).toBe(2);
   markHostCapacityPhaseV1(environment,"rebuild");await until(()=>grantRow(directory,handle.id).phase==="rebuild");
   first!();markHostCapacityPhaseV1(environment,"plan");await sleep(200);
   expect(grantRow(directory,handle.id).reservedMegabytes).toBe(14336);expect(grantRow(directory,handle.id).phase).toBe("rebuild");
   second!();await until(()=>grantRow(directory,handle.id).reservedMegabytes===4096);
   expect(grantRow(directory,handle.id).rebuildBookedAtMs).toBeUndefined();expect(readdirSync(folder)).toEqual([]);
  }finally{first?.();second?.();}
 });
},30000);

test("a request from a retired holder cannot use a new holder with the same grant id",async()=>{
 const directory=fresh(),original=request({fixedMegabytes:4096});let available=24000,previousScope:string|undefined,lost:Promise<unknown>|undefined;
 const setup=options(directory,{request:original,sample:async()=>({host:{...host(),availableMegabytes:available},swappedOutMegabytes:0}),resize:phase=>phase==="rebuild"?{reservedMegabytes:14336,rampMilliseconds:30000}:undefined});
 await withHostCapacityV1(setup,async handle=>{
  previousScope=grantRow(directory,handle.id).rebuildScope;available=12000;
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  lost=requestHostRebuildCapacityV1(environment,{...control(),signal:AbortSignal.timeout(5000)}).catch(error=>error);
  await until(()=>queued(directory).some(row=>row.rebuild===previousScope));
 });
 available=24000;
 await withHostCapacityV1(setup,async handle=>{
  expect(grantRow(directory,handle.id).rebuildScope).not.toBe(previousScope);expect(await lost).toBeInstanceOf(Error);
  expect(grantRow(directory,handle.id).rebuildBookedAtMs).toBeUndefined();
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  const release=await requestHostRebuildCapacityV1(environment,{...control(),signal:AbortSignal.timeout(5000)});try{expect(release).toBeFunction();}finally{release?.();}
 });
},15000);

test("a structurally impossible rebuild fails at the waiting boundary without entering custody",async()=>{
 const directory=fresh();
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:4096}),resize:phase=>phase==="rebuild"?{reservedMegabytes:32768,rampMilliseconds:30000}:undefined}),async handle=>{
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  await expect(requestHostRebuildCapacityV1(environment,control())).rejects.toThrow(/never|cannot/u);
  expect(grantRow(directory,handle.id).reservedMegabytes).toBe(4096);expect(grantRow(directory,handle.id).rebuildBookedAtMs).toBeUndefined();expect(queued(directory)).toEqual([]);
 });
},5000);

test("a cancelled exact rebuild request never admits a partial reservation or leaves a waiter",async()=>{
 const directory=fresh(),stop=new AbortController();let available=24000;
 const sample=async()=>({host:{...host(),availableMegabytes:available},swappedOutMegabytes:0});
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:4096}),sample,observe:async()=>3072,resize:phase=>phase==="rebuild"?{reservedMegabytes:14336,rampMilliseconds:30000}:undefined}),async handle=>{
  await until(()=>grantRow(directory,handle.id).observedMegabytes===3072);
  available=14000;
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory},pending=requestHostRebuildCapacityV1(environment,{...control(),signal:stop.signal});
  try{
   await until(()=>queued(directory).some(row=>row.rebuild));await sleep(200);
   expect(grantRow(directory,handle.id).reservedMegabytes).toBe(4096);expect(grantRow(directory,handle.id).rebuildBookedAtMs).toBeUndefined();
   const reason=Error("cancelled rebuild fixture");stop.abort(reason);await expect(pending).rejects.toBe(reason);await until(()=>queued(directory).length===0);
  }finally{stop.abort();await pending.catch(()=>undefined);}
 });
},30000);

test("an available peer publication withdraws the queued rebuild before giving graph custody to the caller",async()=>{
 const directory=fresh();let available=24000;
 const sample=async()=>({host:{...host(),availableMegabytes:available},swappedOutMegabytes:0});
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:4096}),sample,resize:phase=>phase==="rebuild"?{reservedMegabytes:14336,rampMilliseconds:30000}:undefined}),async handle=>{
  available=14000;
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory},bounded={...control(),signal:AbortSignal.timeout(5000)};
  await expect(requestHostRebuildCapacityV1(environment,bounded,async()=>queued(directory).some(row=>Boolean(row.rebuild)))).resolves.toBeUndefined();
  await until(()=>queued(directory).length===0);
  const grant=grantRow(directory,handle.id);expect(grant.reservedMegabytes).toBe(4096);expect(grant.rebuildBookedAtMs).toBeUndefined();
  expect(readdirSync(join(directory,"grants",`${handle.id}.rebuild.${grant.rebuildScope}`))).toEqual([]);
 });
},10000);

test("Phases marked by the process itself and by a child process cut the run into segments, re-size the grant at each boundary and are learned with the run",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[],calls:string[]=[],seen:Record<string,{reservedMegabytes:number;phase?:string;resumedAtMs?:number;rampMilliseconds?:number}>={};
 let level=400;
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:5000}),learn:run=>{runs.push(run);},observe:async()=>level,resize:(phase,context)=>{calls.push(`${phase}:${context.reservedMegabytes}`);return phase==="rebuild"?{reservedMegabytes:7000,rampMilliseconds:30000}:phase==="execution"?{reservedMegabytes:1600,rampMilliseconds:60000}:undefined;}}),async handle=>{
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  await sleep(120);expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=900;await sleep(120);
  const release=await requestHostRebuildCapacityV1(environment,control());expect(release).toBeFunction();
  const child=spawnSync(registeredToolExecutableV1("bun"),["-e",`import {markHostCapacityPhaseV1} from ${JSON.stringify(entry)};process.exit(markHostCapacityPhaseV1(process.env,"rebuild")?0:3);`],{env:environment,encoding:"utf8"});
  expect(child.status,child.stderr).toBe(0);await sleep(60);level=6000;await sleep(200);
  seen.rebuild=grantRow(directory,handle.id);
  release!();expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=800;await sleep(120);seen.afterRebuild=grantRow(directory,handle.id);
  expect(markHostCapacityPhaseV1(environment,"execution")).toBe(true);await sleep(60);level=1500;await sleep(250);
  seen.execution=grantRow(directory,handle.id);
 });
 expect(seen.rebuild).toMatchObject({reservedMegabytes:7000,phase:"rebuild",rampMilliseconds:policy.wait.staleMilliseconds});expect(seen.rebuild!.resumedAtMs).toBeGreaterThan(0);expect(seen.afterRebuild).toMatchObject({phase:"plan",rampMilliseconds:policy.memory.rampMilliseconds});
 expect(seen.execution).toMatchObject({reservedMegabytes:1600,phase:"execution",rampMilliseconds:60000});
 expect(calls.map(row=>row.split(":")[0])).toEqual(["plan","rebuild","plan","execution"]);expect(calls[1]).toBe("rebuild:5000");expect(calls[3]).toBe("execution:7000");
 expect(runs.length).toBe(1);
 const phases=runs[0]!.phases as readonly HostPhaseSegmentV1[];
 expect(phases.map(row=>row.phase)).toEqual(["assembly","plan","rebuild","plan","execution"]);
 expect(phases[0]!.peakMegabytes).toBe(400);expect(phases[1]!.peakMegabytes).toBe(900);expect(phases[2]!.peakMegabytes).toBe(6000);expect(phases[3]!.peakMegabytes,"the builder still holds its memory when the plan stretch after it begins").toBeGreaterThanOrEqual(800);expect(phases[4]!.peakMegabytes).toBe(1500);
 for(let index=1;index<phases.length;index++)expect(phases[index]!.startMilliseconds).toBeGreaterThanOrEqual(phases[index-1]!.startMilliseconds+phases[index-1]!.activeMilliseconds-1);
 expect(runs[0]!.peakMegabytes).toBe(6000);
 expect(readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".phases")),"the phase file goes with the grant").toEqual([]);
},30000);

test("A run that never marks a phase reports no segments, and a resize port that throws never fails the run",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>700}),async()=>sleep(120));
 expect(runs.length).toBe(1);expect("phases" in runs[0]!).toBe(false);
 const marked:HostRunV1[]=[];
 await expect(withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{marked.push(run);},observe:async()=>700,resize:()=>{throw Error("broken table");}}),async handle=>{
  markHostCapacityPhaseV1({...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory},"execution");await sleep(150);return "kept";
 })).resolves.toBe("kept");
 expect(marked[0]!.phases!.map(row=>row.phase)).toEqual(["assembly","execution"]);
},30000);

test("Marks are refused without a live grant, for an unknown phase and from a nested invocation that only inherits the grant of its parent; the nested invocation cannot move the phases of its parent",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[];
 expect(markHostCapacityPhaseV1({},"plan")).toBe(false);
 expect(markHostCapacityPhaseV1({SEMIO_HOST_CAPACITY_GRANT:"zzzzzzzz1",SEMIO_HOST_CAPACITY_DIRECTORY:directory},"plan"),"a grant that does not exist").toBe(false);
 expect(markHostCapacityPhaseV1({SEMIO_HOST_CAPACITY_GRANT:"../../etc/x",SEMIO_HOST_CAPACITY_DIRECTORY:directory},"plan"),"an identifier that is no identifier").toBe(false);
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>500}),async outer=>{
  const environment={...process.env,...outer.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  expect(outer.environment.SEMIO_HOST_CAPACITY_PHASE_MARKS).toBe("on");
  expect(markHostCapacityPhaseV1(environment,"bogus" as never)).toBe(false);
  await withHostCapacityV1(options(directory,{environment,pid:process.pid+1,ppid:process.pid+2,request:request({fixedMegabytes:100})}),async inner=>{
   expect(inner.inherited).toBe(true);expect(inner.environment.SEMIO_HOST_CAPACITY_PHASE_MARKS).toBe("off");
   expect(markHostCapacityPhaseV1({...environment,...inner.environment},"execution")).toBe(false);
  });
  await sleep(80);
 });
 expect(runs[0]!.phases,"nothing the nested invocation tried reached the phases of its parent").toBeUndefined();
},30000);

test("The holder re-entering the gate and the process it re-executes itself into continue its invocation and marks the phases of the run; a deeper nested invocation stays refused",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>500}),async outer=>{
  const environment={...process.env,...outer.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  await withHostCapacityV1(options(directory,{environment,pid:process.pid+1,ppid:process.pid,request:request({fixedMegabytes:100})}),async continuation=>{
   expect(continuation.inherited).toBe(true);expect(continuation.environment.SEMIO_HOST_CAPACITY_PHASE_MARKS,"the direct child of the holder keeps the marks").toBe("on");
   expect(markHostCapacityPhaseV1({...environment,...continuation.environment},"plan")).toBe(true);await sleep(60);
   expect(markHostCapacityPhaseV1({...environment,...continuation.environment},"execution")).toBe(true);
  });
  await withHostCapacityV1(options(directory,{environment,pid:process.pid+1,ppid:process.pid+2,request:request({fixedMegabytes:100})}),async nested=>{
   expect(nested.environment.SEMIO_HOST_CAPACITY_PHASE_MARKS,"a process that is neither the holder nor its direct child is a nested invocation").toBe("off");
  });
  await sleep(120);
 });
 expect(runs[0]!.phases?.map(row=>row.phase),"the plan the continuation marked reaches the learned phases of the run").toEqual(expect.arrayContaining(["assembly","plan","execution"]));
},30000);

test("Marks beyond the bound of transitions are ignored, so a flapping process cannot grow the record of a run",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[];
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100}),learn:run=>{runs.push(run);},observe:async()=>500}),async handle=>{
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  for(let index=0;index<90;index++){markHostCapacityPhaseV1(environment,index%2===0?"plan":"execution");if(index%10===9)await sleep(30);}
  await sleep(200);
 });
 expect(runs[0]!.phases!.length).toBeLessThanOrEqual(65);expect(runs[0]!.phases!.length).toBeGreaterThan(2);
},30000);
