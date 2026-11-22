/** 🧮️ Contract: the machine's own read and the pool are one budget of memory and cores. The read has the first claim (through the asks of the capacity governor), the pool gets what it leaves and gives it back within the derived window of one pool poll plus the kill limit; there is no time after which the pool stops yielding; the pool never runs more children than the spare memory holds while the read exists; neither starves when both have small owners; swap shrinks the pool before the gate stops the farm. Hosts declared: 8 GB / 4 cores and 16 GB / 8 cores (the host matrix of R135). Oracle: the Python implementation of the fixtures and the gate's own policy numbers. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolSizingV1,type OwnerPoolHostV1}from"../../🟦️.ts";
import{createMemoryGovernorV1,GRAPH_MEMORY_POLICY_V1 as M}from"../../../../../../🧠️memory/🟨️.mjs";
import{createHostAskV1,hostAskClaimV1,readHostAsksV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🟦️.ts";
import policy from"../../../../../../🔣️policy.json";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576;
const hosts=[{name:"8 GB / 4 cores",cores:4,total:8192,other:1000,owner:800,solo:3500,child:900},{name:"16 GB / 8 cores",cores:8,total:16384,other:1500,owner:1000,solo:6000,child:900}];
const floorOf=(total:number)=>Math.max(P.floorMinimumMegabytes,Math.min(P.floorMaximumMegabytes,Math.floor(total*P.floorShareBasisPoints/10000)));
const hostAt=(h:typeof hosts[number],available:number,over:Partial<OwnerPoolHostV1>&{foreground?:Partial<OwnerPoolHostV1["foreground"]>}={}):OwnerPoolHostV1=>({cores:h.cores,totalMegabytes:h.total,availableMegabytes:available,pressure:"normal",load:null,swapOutMegabytesPerSecond:null,idleCores:null,...over,foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...over.foreground}});

type Sim={admittedAtMs:number|null;childrenAt:number[];startedAtMs:number};
async function simulate(h:typeof hosts[number],withAsk:boolean):Promise<Sim>{
 const directory=mkdtempSync(join(tmpdir(),"shared-budget-"));scratch.push(directory);
 let now=1_000_000,children=h.cores,killAt:number|null=null,nextPoll=now,ownerA=true,ownerB=false;
 const floor=floorOf(h.total),available=()=>h.total-h.other-(ownerA?h.owner:0)-(ownerB?h.solo:0)-children*h.child;
 const writer=createHostAskV1({directory,now:()=>now,pid:process.pid});
 const poolStep=()=>{
  if(now<nextPoll)return;nextPoll=now+P.targetPollMilliseconds;
  const claim=hostAskClaimV1(readHostAsksV1(directory,now,()=>true));
  const target=ownerPoolSizingV1({host:hostAt(h,available(),{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...claim===undefined?{}:{ask:claim}}}),children,childMegabytes:h.child});
  if(target.reason==="waiting"){killAt??=now+P.yieldKillMilliseconds;if(now>=killAt)children=0;}
  else{killAt=null;children=Math.min(children,target.children);}
 };
 const sim:Sim={admittedAtMs:null,childrenAt:[],startedAtMs:now};
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>(available()-floor)*MB,estimate:key=>({a:h.owner,b:h.solo}[key as "a"|"b"])*MB,estimateBytes:h.owner*MB,ownerLimitBytes:h.total*MB,now:()=>now,sleep:async ms=>{now+=ms;poolStep();sim.childrenAt.push(children);},...withAsk?{ask:writer}:{},policy:M});
 let release!:()=>void;const hold=new Promise<void>(done=>{release=done;});
 const a=governor.run(async()=>{await hold;},{kind:"k",key:"a"});
 await new Promise(done=>setTimeout(done,5));
 sim.startedAtMs=now;
 const b=governor.run(async()=>{sim.admittedAtMs=now;ownerB=true;ownerA=false;release();},{kind:"k",key:"b",limitBytes:h.solo*MB*2});
 await Promise.all([a,b]);governor.dispose();return sim;
}

for(const h of hosts){
 test(`${h.name}: an alone owner of the read drains the pool and is admitted within one pool poll plus the kill limit; without the ask it waits for the bound of the governor`,async()=>{
  const window=P.targetPollMilliseconds*(1+Math.ceil(P.yieldKillMilliseconds/P.targetPollMilliseconds))+M.admitMinimumMilliseconds+2*M.pollMilliseconds;
  const asked=await simulate(h,true);
  expect(asked.admittedAtMs).not.toBeNull();
  expect(asked.admittedAtMs!-asked.startedAtMs,"derived window: one pool poll, then the polls until the kill limit has passed").toBeLessThanOrEqual(window);
  expect(asked.childrenAt.at(-1)).toBe(0);
  expect(h.cores,"the pool held children when the ask came").toBeGreaterThan(0);
  const control=await simulate(h,false);
  expect(control.admittedAtMs!-control.startedAtMs,"the old behaviour: only the bound of the governor ends the wait").toBeGreaterThanOrEqual(M.waitBoundMilliseconds);
 });

 test(`${h.name}: the pool never exceeds the spare memory while the read exists, for every available size, claim and child count`,()=>{
  const floor=floorOf(h.total),need=h.child;
  for(let available=0;available<=h.total;available+=256)for(const children of[0,1,2,h.cores])for(const claim of[0,400,1500,4000,h.total])for(const alone of[false,true]){
   const ask=claim>0?{megabytes:claim,alone}:undefined;
   const sizing=ownerPoolSizingV1({host:hostAt(h,available,{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,...ask===undefined?{}:{ask}}}),children,childMegabytes:need});
   const drained=available-floor+children*need,counted=claim<=drained?claim:0,spare=available-floor-counted;
   const bound=Math.max(0,children+Math.floor(spare/need));
   expect(sizing.children,JSON.stringify({available,children,claim,alone})).toBeLessThanOrEqual(alone&&claim>0?0:bound);
  }
 });

 test(`${h.name}: both starve never with small owners: the read's owner fits and the pool still runs a child`,()=>{
  const floor=floorOf(h.total);
  for(let available=floor+h.owner+h.child;available<=h.total-h.other;available+=256){
   const claim=h.owner,sizing=ownerPoolSizingV1({host:hostAt(h,available,{foreground:{cpu:1,unrealizedMegabytes:0,waiters:0,ask:{megabytes:claim,alone:false}}}),children:0,childMegabytes:h.child});
   expect(sizing.children,"the pool runs").toBeGreaterThanOrEqual(1);
   expect(available-floor-sizing.children*h.child,"the read's owner still fits after the pool grew").toBeGreaterThanOrEqual(claim);
  }
 });

 test(`${h.name}: swap shrinks the pool in proportion before the gate stops the farm, and ends it where the gate stops`,()=>{
  expect(P.swapActiveBasisPointsPerSecond).toBe(policy.hostCapacity.swap.activeBasisPointsPerSecond);
  expect(P.swapOutBasisPointsPerSecond).toBe(policy.hostCapacity.swap.outBasisPointsPerSecond);
  const lowSwap=h.total*P.swapActiveBasisPointsPerSecond/10000,highSwap=h.total*P.swapOutBasisPointsPerSecond/10000;
  let before=h.cores;
  for(let swap=0;swap<=Math.ceil(highSwap);swap++){
   const sizing=ownerPoolSizingV1({host:hostAt(h,h.total-h.other,{idleCores:h.cores,swapOutMegabytesPerSecond:swap}),children:h.cores,childMegabytes:h.child});
   expect(sizing.children,String(swap)).toBeLessThanOrEqual(before);before=sizing.children;
   if(swap>lowSwap&&swap<highSwap)expect(sizing.children,"cut before the gate stops anything").toBeLessThan(h.cores);
  }
  expect(before).toBe(0);
  const grown=ownerPoolSizingV1({host:hostAt(h,h.total-h.other,{idleCores:h.cores,swapOutMegabytesPerSecond:lowSwap+1}),children:0,childMegabytes:h.child});
  expect(grown.children,"no growth while it swaps").toBe(0);
 });

 test(`${h.name}: no time makes the pool stop yielding to a request that its children hold up`,()=>{
  const floor=floorOf(h.total),available=floor+h.child,waiting=2*h.child;
  for(const waited of[P.yieldAfterMilliseconds,10*60_000,6*3_600_000,48*3_600_000]){
   const sizing=ownerPoolSizingV1({host:hostAt(h,available,{idleCores:h.cores,foreground:{cpu:1,unrealizedMegabytes:0,waiters:1,waitingMegabytes:waiting,oldestWaitMilliseconds:waited}}),children:4,childMegabytes:h.child});
   expect(sizing,String(waited)).toMatchObject({children:0,reason:"waiting"});
  }
  const never=ownerPoolSizingV1({host:hostAt(h,available,{idleCores:h.cores,foreground:{cpu:1,unrealizedMegabytes:0,waiters:1,waitingMegabytes:h.total*2,oldestWaitMilliseconds:48*3_600_000}}),children:2,childMegabytes:h.child});
  expect(never.reason,"a wait the children did not cause is no reason to stop them").not.toBe("waiting");
 });
}

test("the policy keeps no time after which the pool stops yielding",()=>{
 expect(Object.keys(P)).not.toContain("yieldUntilMilliseconds");
});
