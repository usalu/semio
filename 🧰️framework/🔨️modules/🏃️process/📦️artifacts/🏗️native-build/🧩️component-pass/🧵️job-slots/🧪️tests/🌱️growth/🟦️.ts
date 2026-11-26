/** 🌱️ Tests that a known inner requirement waits for the parent grant to grow instead of refusing outright. */
import {expect,test}from"bun:test";
import * as jobs from"../../🟦️.ts";

const sample=async()=>({availableMegabytes:8000,ownedMegabytes:0,pressure:"normal" as const});
const pass={target:"wasm32-wasip2"};
const run=(memory:jobs.ParentComponentMemoryV1,over:{signal?:AbortSignal;check?:()=>void}={})=>{
 const routes:jobs.PassJobSlotV1[]=[];let clock=0;
 const work=jobs.runPassesInJobSlotsV1([pass],{jobs:1,check:over.check??(()=>undefined),parentMemory:memory,...over.signal?{signal:over.signal}:{},local:{maximum:1,peak:()=>3757,floorMegabytes:0,sample,pollMilliseconds:1,now:()=>clock+=1000},run:async(_pass,slot)=>{routes.push(slot);}});
 return{work,routes};
};

test("Law: a requirement above the grant waits while the grant can still grow to it and starts once the live grant carries it",async()=>{
 let allowance=1638,declared=0,reads=0;
 const memory=new jobs.ParentComponentMemoryV1(1638,0,{ceilingMegabytes:8190,waitMilliseconds:600000,refresh:()=>{reads++;if(reads>=3)allowance=4000;return allowance;},declare:megabytes=>{declared=Math.max(declared,megabytes);}});
 const {work,routes}=run(memory);
 await work;
 expect(routes).toEqual(["local"]);
 expect(declared).toBe(3757);
 expect(memory.allowanceMegabytes).toBe(4000);
});

test("Law: a requirement the grant can never reach is refused at once and the live allowance never shrinks the grant",async()=>{
 const memory=new jobs.ParentComponentMemoryV1(1638,0,{ceilingMegabytes:2000,waitMilliseconds:600000,refresh:()=>100,declare:()=>undefined});
 await expect(run(memory).work).rejects.toMatchObject({name:"ParentComponentMemoryRequiredErrorV1",requiredMegabytes:3757,allowanceMegabytes:1638});
 expect(memory.allowanceMegabytes).toBe(1638);
});

test("Law: the wait for a growing grant is bounded and then refuses with the requirement",async()=>{
 const memory=new jobs.ParentComponentMemoryV1(1638,0,{ceilingMegabytes:8190,waitMilliseconds:5000,refresh:()=>1638,declare:()=>undefined});
 await expect(run(memory).work).rejects.toMatchObject({name:"ParentComponentMemoryRequiredErrorV1",requiredMegabytes:3757,allowanceMegabytes:1638});
});

test("Law: cancelling retires the wait for a growing grant",async()=>{
 const controller=new AbortController(),reason=new Error("cancel the grant wait"),memory=new jobs.ParentComponentMemoryV1(1638,0,{ceilingMegabytes:8190,waitMilliseconds:600000,refresh:()=>1638,declare:()=>undefined});
 let checks=0;
 const {work,routes}=run(memory,{signal:controller.signal,check:()=>{if(++checks===3)controller.abort(reason);}});
 await expect(work).rejects.toBe(reason);
 expect(routes).toEqual([]);
});
