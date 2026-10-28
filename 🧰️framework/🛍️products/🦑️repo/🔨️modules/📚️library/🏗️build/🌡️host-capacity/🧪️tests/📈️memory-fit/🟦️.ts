/** 📈️ Tests for build host-capacity memory-fit. */
import {expect,test} from "bun:test";
import {emptyHostEstimateStoreV1,estimateHostRatioV1,evaluateHostCapacityV1,rampHostRequestV1,readHostCapacityPolicyV1,recordHostOutcomeV1,scaleHostRequestV1,sizeHostRequestV1,type HostCapacityPolicyV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../🟦️.ts";
import recorded from "./🧫️fixtures/🔣️.json";

type Trace=Readonly<{ratio:number;active:number}>;
type Result=Readonly<{minFree:number;ramping:number;concurrent:number;finished:boolean}>;

const TOTAL=16384,FOREIGN=6000,DEFAULT=3584,STEP=1000,HORIZON=4*3600_000;
const previous:HostCapacityPolicyV1=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:{...buildPolicy.hostCapacity.memory,maximumRamping:64}});
const traces:readonly Trace[]=recorded.classes.run.map(sample=>({ratio:sample.ratioBasisPoints,active:sample.activeMilliseconds}));
const hot=traces.filter(row=>row.ratio<8000),cold=traces.filter(row=>row.ratio>=8000);
const train=(policy:HostCapacityPolicyV1,rows:readonly Trace[])=>rows.reduce((store,row,index)=>recordHostOutcomeV1(policy,store,["run","run:test"],{operation:"run",jobs:1,peakMegabytes:Math.round(row.ratio*DEFAULT/10000),defaultMegabytes:DEFAULT,activeMilliseconds:row.active,peakAtMilliseconds:row.active,endedAtMs:1_000_000+index}),emptyHostEstimateStoreV1());

/** 🧮️ Simulates a burst of identical `run` requests on a 16 GB host that holds 6 GB for foreign processes: every second the gate decides, a run's memory grows linearly from 20 % to its recorded peak at the end of its recorded active time (the latest peak the estimator can assume), grants see their own memory at the 15 s heartbeat only, and the host reports pressure from the memory that is really free. `learned` is the shape of the reservation: the production sizing with a store trained on `rows`, or `previous` (60 s ramp for everything, no ramp slots). */
function simulate(policy:HostCapacityPolicyV1,learned:boolean,rows:readonly Trace[],draw:readonly Trace[],seed:number,count=8):Result{
 const store=train(BUILD_HOST_CAPACITY_POLICY_V1,rows),estimate=estimateHostRatioV1(policy,store,["run","run:test"],2_000_000),base=sizeHostRequestV1(policy,{id:"base00001",operation:"run",preferredJobs:1,kinds:["test"]})!;
 const scaled=scaleHostRequestV1(policy,base,estimate.ratioBasisPoints,TOTAL-2048),request:HostRequestV1=learned?rampHostRequestV1(base,scaled,estimate):scaled;
 let state=seed;const random=()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};
 const runs=Array.from({length:count},()=>{const row=draw[Math.floor(random()*draw.length)]!;return {peak:row.ratio*DEFAULT/10000,ttp:row.active,duration:row.active+30000};});
 const waiters=runs.map((_,index)=>({request:{...request,id:`req${String(index).padStart(6,"0")}`},arrivedAtMs:index,pid:1}));
 const started=new Map<string,number>(),waiting=new Set(waiters.map(row=>row.request.id));
 const real=(index:number,at:number)=>runs[index]!.peak*(0.2+0.8*Math.min(1,(at-started.get(waiters[index]!.request.id)!)/runs[index]!.ttp));
 let grants:HostGrantV1[]=[],finished=0,minFree=Infinity,ramping=0,concurrent=0;
 for(let now=0;now<HORIZON&&finished<runs.length;now+=STEP){
  grants=grants.filter(grant=>{const index=waiters.findIndex(row=>row.request.id===grant.id);if(now-started.get(grant.id)!>=runs[index]!.duration){finished++;return false;}return true;});
  const held=grants.reduce((sum,grant)=>sum+real(waiters.findIndex(row=>row.request.id===grant.id),now),0),free=TOTAL-FOREIGN-held;
  minFree=Math.min(minFree,free);concurrent=Math.max(concurrent,grants.length);
  ramping=Math.max(ramping,grants.filter(grant=>grant.rampSlot===true&&now-grant.admittedAtMs<(grant.rampMilliseconds??policy.memory.rampMilliseconds)).length);
  grants=grants.map(grant=>{const age=now-grant.admittedAtMs;return age>=3000&&(age-3000)%policy.wait.heartbeatMilliseconds<STEP?{...grant,observedMegabytes:Math.round(real(waiters.findIndex(row=>row.request.id===grant.id),now))}:grant;});
  const available=Math.max(0,free),pressure=available*10000<TOTAL*policy.pressure.availableCriticalBasisPoints?"critical" as const:available*10000<TOTAL*policy.pressure.availableWarnBasisPoints?"warn" as const:"normal" as const;
  const host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:8,totalMegabytes:TOTAL,availableMegabytes:Math.round(available),pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:null,openFilesMaximum:null,attachedImages:null};
  const queue=waiters.filter(row=>waiting.has(row.request.id));if(queue.length===0)continue;
  for(const decision of evaluateHostCapacityV1(policy,host,grants,queue,now).decisions)if(decision.verdict==="admit"){
   const waiter=waiters.find(row=>row.request.id===decision.id)!;waiting.delete(decision.id);started.set(decision.id,now);
   grants.push({id:decision.id,operation:"run",jobs:decision.jobs,cpu:decision.jobs,reservedMegabytes:decision.reservedMegabytes,admittedAtMs:now,observedMegabytes:null,pid:1,bootAtMs:0,...waiter.request.rampMilliseconds===undefined?{}:{rampMilliseconds:waiter.request.rampMilliseconds},...waiter.request.rampSlot===undefined?{}:{rampSlot:waiter.request.rampSlot}});
  }
 }
 return {minFree:Math.round(minFree),ramping,concurrent,finished:finished===runs.length};
}
const seeds=Array.from({length:16},(_,index)=>index+1);
const overshoot=(rows:readonly Result[])=>rows.reduce((sum,row)=>sum+Math.max(0,-row.minFree),0);

test("Law: the recorded runs of this machine are real and bimodal: a hot mode, a cold mode of 160 %, and times of tens to thousands of seconds",()=>{
 expect(hot.length).toBeGreaterThanOrEqual(5);expect(cold.length).toBeGreaterThanOrEqual(2);
 expect(Math.min(...traces.map(row=>row.ratio))).toBeLessThan(6000);expect(Math.max(...traces.map(row=>row.ratio))).toBeGreaterThan(16000);
 expect(Math.max(...traces.map(row=>row.active))).toBeGreaterThan(1_000_000);
});

test("Law: a class learned from the recorded runs carries a burst of runs without the real memory of the admitted runs ever exceeding what the host has",()=>{
 const results=seeds.map(seed=>simulate(BUILD_HOST_CAPACITY_POLICY_V1,true,traces,traces,seed));
 for(const [index,row] of results.entries())expect(row.minFree,`seed ${seeds[index]}`).toBeGreaterThanOrEqual(0);
 expect(results.every(row=>row.finished)).toBe(true);
});

test("Law: a reservation learned from hot runs only does not let a burst overshoot the host when the cold mode (160 %) arrives; the previous 60 s ramp without slots did, in the same simulation",()=>{
 const hotOnly=[...hot,...hot,...hot].slice(0,15),now=seeds.map(seed=>simulate(BUILD_HOST_CAPACITY_POLICY_V1,true,hotOnly,traces,seed)),before=seeds.map(seed=>simulate(previous,false,hotOnly,traces,seed));
 const negativeBefore=before.filter(row=>row.minFree<0).length,negativeNow=now.filter(row=>row.minFree<0).length;
 expect(negativeBefore,"the previous configuration overshoots in this simulation").toBeGreaterThanOrEqual(4);
 expect(negativeNow,"a cold run among hot reservations can still exceed its own reservation, in far fewer bursts").toBeLessThanOrEqual(Math.floor(negativeBefore/2));
 expect(overshoot(now),"and by far less memory").toBeLessThanOrEqual(overshoot(before)/4);
 expect(Math.max(...now.map(row=>row.ramping)),"never more shrunk reservations growing at once than the slots allow").toBeLessThanOrEqual(BUILD_HOST_CAPACITY_POLICY_V1.memory.maximumRamping);
 expect(Math.max(...before.map(row=>row.concurrent)),"the previous policy ran five at once").toBeGreaterThan(Math.max(...now.map(row=>row.concurrent)));
 for(const [index,row] of before.entries())if(row.minFree<0)expect(now[index]!.minFree,`seed ${seeds[index]} improves`).toBeGreaterThan(row.minFree);
});

test("Law: a deliberately wrong class (30 % while the runs need 45-167 %) overshoots with at most the ramp slots of runs, and by less than half of what the previous configuration did",()=>{
 const wrong=Array.from({length:16},()=>({ratio:3000,active:200_000})),now=seeds.map(seed=>simulate(BUILD_HOST_CAPACITY_POLICY_V1,true,wrong,traces,seed)),before=seeds.map(seed=>simulate(previous,false,wrong,traces,seed));
 expect(Math.max(...now.map(row=>row.ramping))).toBeLessThanOrEqual(BUILD_HOST_CAPACITY_POLICY_V1.memory.maximumRamping);
 expect(before.filter(row=>row.minFree<0).length,"the previous configuration admitted the whole burst").toBeGreaterThanOrEqual(10);
 expect(overshoot(now)).toBeLessThanOrEqual(overshoot(before)/2);
 expect(Math.max(...before.map(row=>row.concurrent))).toBeGreaterThanOrEqual(7);
});
