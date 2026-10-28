import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {markHostCapacityPhaseV1,newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostPhaseSegmentV1,type HostRunV1,type HostSampleV1} from "../../../🟦️.ts";
import decisionFixture from "../../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hp-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control(),environment:{},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const entry=fileURLToPath(new URL("../../../🟦️.ts",import.meta.url));
const grantRow=(directory:string,id:string)=>JSON.parse(readFileSync(join(directory,"grants",`${id}.json`),"utf8")) as {reservedMegabytes:number;phase?:string;resumedAtMs?:number;rampMilliseconds?:number};

test("Phases marked by the process itself and by a child process cut the run into segments, re-size the grant at each boundary and are learned with the run",async()=>{
 const directory=fresh(),runs:HostRunV1[]=[],calls:string[]=[],seen:Record<string,{reservedMegabytes:number;phase?:string;resumedAtMs?:number;rampMilliseconds?:number}>={};
 let level=400;
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:5000}),learn:run=>{runs.push(run);},observe:async()=>level,resize:(phase,context)=>{calls.push(`${phase}:${context.reservedMegabytes}`);return phase==="rebuild"?{reservedMegabytes:7000,rampMilliseconds:90000}:phase==="execution"?{reservedMegabytes:1600,rampMilliseconds:60000}:undefined;}}),async handle=>{
  const environment={...process.env,...handle.environment,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
  await sleep(120);expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=900;await sleep(120);
  const child=spawnSync("bun",["-e",`import {markHostCapacityPhaseV1} from ${JSON.stringify(entry)};process.exit(markHostCapacityPhaseV1(process.env,"rebuild")?0:3);`],{env:environment,encoding:"utf8"});
  expect(child.status,child.stderr).toBe(0);await sleep(60);level=6000;await sleep(200);
  seen.rebuild=grantRow(directory,handle.id);
  expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=800;await sleep(120);
  expect(markHostCapacityPhaseV1(environment,"execution")).toBe(true);await sleep(60);level=1500;await sleep(250);
  seen.execution=grantRow(directory,handle.id);
 });
 expect(seen.rebuild).toMatchObject({reservedMegabytes:7000,phase:"rebuild",rampMilliseconds:90000});expect(seen.rebuild!.resumedAtMs).toBeGreaterThan(0);
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
  await withHostCapacityV1(options(directory,{environment,request:request({fixedMegabytes:100})}),async inner=>{
   expect(inner.inherited).toBe(true);expect(inner.environment.SEMIO_HOST_CAPACITY_PHASE_MARKS).toBe("off");
   expect(markHostCapacityPhaseV1({...environment,...inner.environment},"execution")).toBe(false);
  });
  await sleep(80);
 });
 expect(runs[0]!.phases,"nothing the nested invocation tried reached the phases of its parent").toBeUndefined();
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
