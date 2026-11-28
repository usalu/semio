/** 🚏️ Contract: a request that waits for memory the machine really has is not starved by the idle rest of long-running reservations. On the live 16 GB gate (4973 MB available, cargo grants reserving 1536 MB and holding 531 to 550 MB for an hour) a graph plan waited 3138 s for its 2048 MB minimum, because every retained reservation was charged in full as unrealized memory (2080 MB) and the room above the floor read 845 MB; a settled retained grant is charged what it holds plus a margin, so the planner is admitted at the room that is free, the oldest request stays the head of the line (a newer request that fits alone does not overtake it) and two planners never book the same room. The verdicts and reservations of every case equal those of the independent Python oracle.
 * @see ./🐍️.py
 * @see ./🔣️.json
 * @see ../../📏️decision/🟦️.ts `settledRetainedHostMegabytesV1` */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,settledRetainedHostMegabytesV1,withUnmanagedHostV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";
import fixture from "./🔣️.json";

type Oracle={name:string;decisions:Record<string,{verdict:"admit"|"wait";reservedMegabytes:number}>}[];
const python=testPythonV1()??Bun.which("python3")??"python3",script=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",script]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as Oracle;})();
const now=1_000_000_000,policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),{memory:{retainedKeepMegabytes:_a,retainedKeepBasisPoints:_b,...kept}}=buildPolicy.hostCapacity,unsettled=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:kept} as never);
type Row=(typeof fixture)["grants"][number]&{rampMilliseconds?:number};
const grantOf=(row:Row,age:number):HostGrantV1=>({id:row.id,operation:row.operation,jobs:1,cpu:1,reservedMegabytes:row.reservedMegabytes,observedMegabytes:row.observedMegabytes,admittedAtMs:now-age*1000,pid:100,bootAtMs:0,label:row.id,priority:"batch",session:row.session,...(row as {retain?:boolean}).retain?{retainReservation:true}:{},...(row as {quiet?:boolean}).quiet?{quiet:true,rampMilliseconds:300000}:{}}) as never;
const waiterOf=(row:(typeof fixture)["waiters"][number],at:number):HostWaiterV1=>({arrivedAtMs:now-row.waitedSeconds*1000,pid:at+1,session:row.session,request:{id:row.id,operation:row.operation,priority:row.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:row.fixedMegabytes,perJobMegabytes:0,cpuPerJob:1,label:row.operation,...(row as {minimumMegabytes?:number}).minimumMegabytes===undefined?{}:{minimumMegabytes:(row as {minimumMegabytes:number}).minimumMegabytes},...(row as {floorMegabytes?:number}).floorMegabytes===undefined?{}:{floorMegabytes:(row as {floorMegabytes:number}).floorMegabytes},...(row as {retain?:boolean}).retain?{retainReservation:true}:{},...(row as {quiet?:boolean}).quiet?{quiet:true,rampMilliseconds:300000}:{}}}) as never;
const decide=(at:number,use=policy)=>{
 const row=fixture.cases[at] as (typeof fixture)["cases"][number]&{grantAgeSeconds?:number;extraGrants?:Row[];settle?:boolean},ledger=[...fixture.grants.map(each=>grantOf(each,(each as {retain?:boolean}).retain&&each.operation!=="graph-plan"&&row.grantAgeSeconds!==undefined?row.grantAgeSeconds:each.ageSeconds)),...(row.extraGrants??[]).map(each=>grantOf(each,each.ageSeconds))];
 const host={version:1,atMs:now,platform:"darwin",cores:fixture.cores,totalMegabytes:fixture.totalMegabytes,availableMegabytes:row.availableMegabytes,pressure:"normal",swapUsedMegabytes:fixture.swapUsedMegabytes,swapOutMegabytesPerSecond:fixture.swapOutMegabytesPerSecond,load:fixture.load,openFiles:6000,openFilesMaximum:122880,attachedImages:21,compressedMegabytes:fixture.compressedMegabytes} as HostSampleV1;
 const queue=row.waiters.map((id,at)=>waiterOf(fixture.waiters.find(each=>each.id===id)!,at));
 return evaluateHostCapacityV1(row.settle===false?unsettled:use,withUnmanagedHostV1(host,ledger),ledger,queue,now);
};

test("every case of the snapshot gives the verdicts and reservations of the oracle",()=>{
 expect(expected.length).toBe(fixture.cases.length);
 for(const [at,row] of fixture.cases.entries()){
  const evaluation=decide(at),want=expected[at]!;
  expect(want.name).toBe(row.name);
  for(const [id,answer] of Object.entries(want.decisions)){
   const got=evaluation.decisions.find(decision=>decision.id===id)!;
   expect({id,verdict:got.verdict,reservedMegabytes:got.reservedMegabytes},row.name).toEqual({id,...answer});
  }
 }
});

test("the live snapshot: the planner that waited 3138 s is admitted shrunk, the idle cargo headroom is no longer charged, and everything behind it keeps its order",()=>{
 const evaluation=decide(0),plan=evaluation.decisions.find(row=>row.id==="plan1")!;
 expect(plan).toMatchObject({verdict:"admit",reason:"shrunk"});
 expect(plan.reservedMegabytes).toBeGreaterThanOrEqual(2048);
 expect(evaluation.unrealizedMegabytes,"256 MB margin of two cargo grants and 89 MB of the third").toBe(601);
 for(const id of ["plan2","decoder","editor"])expect(evaluation.decisions.find(row=>row.id===id)).toMatchObject({verdict:"wait",reason:"queue-order"});
});

test("the same snapshot without the settle rule reproduces the starvation: 2080 MB unrealized, 845 MB of room, the planner waits for memory and everyone else for its turn",()=>{
 const evaluation=decide(1);
 expect(evaluation.unrealizedMegabytes).toBe(2080);
 expect(evaluation.headroomMegabytes).toBe(845);
 expect(evaluation.decisions.find(row=>row.id==="plan1")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(evaluation.decisions.filter(row=>row.verdict==="admit")).toEqual([]);
});

test("head of the line reserves the next free room: a newer request that fits alone waits behind the planner, and the planner takes the room the moment its minimum fits",()=>{
 expect(decide(4).decisions.find(row=>row.id==="small")).toMatchObject({verdict:"wait",reason:"queue-order"});
 expect(decide(4).decisions.find(row=>row.id==="plan1")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(decide(5).decisions.find(row=>row.id==="plan1")).toMatchObject({verdict:"admit"});
});

test("two planners never book the same room: one is admitted, the second waits behind it",()=>{
 const rows=decide(6).decisions;
 expect(rows.filter(row=>row.verdict==="admit").map(row=>row.id)).toEqual(["plan1"]);
 expect(rows.find(row=>row.id==="plan2")).toMatchObject({verdict:"wait",reason:"queue-order"});
});

test("the charge of a settled retained grant: holds plus margin, never above its reservation, never for a graph plan, a young grant or an unobserved one",()=>{
 const grant=(over:Partial<HostGrantV1>):HostGrantV1=>({id:"g",operation:"cargo-orchestration",jobs:1,cpu:1,reservedMegabytes:1536,observedMegabytes:550,admittedAtMs:now-3600000,pid:1,bootAtMs:0,retainReservation:true,...over} as never);
 expect(settledRetainedHostMegabytesV1(policy,grant({}),now)).toBe(806);
 expect(settledRetainedHostMegabytesV1(policy,grant({observedMegabytes:1447}),now)).toBe(1536);
 expect(settledRetainedHostMegabytesV1(policy,grant({observedMegabytes:2244}),now)).toBe(1536);
 expect(settledRetainedHostMegabytesV1(policy,grant({operation:"graph-plan"}),now)).toBe(1536);
 expect(settledRetainedHostMegabytesV1(policy,grant({admittedAtMs:now-600000}),now)).toBe(1536);
 expect(settledRetainedHostMegabytesV1(policy,grant({observedMegabytes:null}),now)).toBe(1536);
 expect(settledRetainedHostMegabytesV1(policy,grant({resumedAtMs:now-1000}),now),"a resumed grant settles again").toBe(1536);
 expect(settledRetainedHostMegabytesV1(unsettled,grant({}),now)).toBe(1536);
});

test("the live policy carries the retained settle rule",()=>{
 expect(buildPolicy.hostCapacity.memory.retainedKeepMegabytes).toBe(256);
 expect(buildPolicy.hostCapacity.memory.retainedKeepBasisPoints).toBe(12500);
});
