import {expect,test}from"bun:test";
import Ajv from"ajv";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {testPythonV1}from"../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import * as jobs from"../../🟦️.ts";
import vectors from"./🔣️.json"with{type:"json"};

const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const sample=async()=>({availableMegabytes:8000,ownedMegabytes:0,pressure:"normal" as const});
const parent=(grant:number,baseline?:number)=>new jobs.ParentComponentMemoryV1(grant,baseline);
const pass={target:"wasm32-wasip2"};
const routeOf=(value:string):jobs.PassJobSlotV1=>{if(value==="local"||value==="fleet")return value;throw Error(`unknown pass job slot ${value}`);};
test("the Python route oracle agrees with the language-neutral component memory cases",()=>{
 const result=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify(vectors),encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(JSON.parse(result.stdout)).toEqual(vectors.map(row=>row.result));
});
test("Ajv independently bounds local and fleet footprints for the authored cases",()=>{
 const ajv=new Ajv({strict:true});
 for(const row of vectors){
  const fleet=row.fleet,local=(row.baseline??0)+row.peak+(row.localFootprint??0),allowance=ajv.compile({type:"integer",...row.grant===null?{}:{maximum:row.grant}});
  const available=fleet?.slots===0&&fleet.waitForCompatibleSlot?fleet.laterSlots:fleet?.slots;
  const routes:jobs.PassJobSlotV1[]=allowance(local)?["local"]:available&&fleet&&!row.native&&!row.critical&&allowance((row.baseline??0)+fleet.footprint)?["fleet"]:[];
  const refusals=!routes.length||routes[0]==="fleet"&&fleet?.fails;
  expect({...refusals?{required:local}:{},routes},row.name).toEqual({...row.result,routes:row.result.routes.map(routeOf)});
 }
});
for(const row of vectors)test(row.name,async()=>{
 const routes:jobs.PassJobSlotV1[]=[];
 const sampleHost=row.critical?async()=>({availableMegabytes:8000,ownedMegabytes:0,pressure:"critical" as const}):sample;
 let checks=0, polls=0, refusal:unknown;
 try{
  await jobs.runPassesInJobSlotsV1([row.native?{target:"aarch64-apple-darwin"}:pass],{
   jobs:row.jobs,check:()=>{if(++checks>8)throw Error("memory admission did not settle");},
   ...row.grant===null?{}:{parentMemory:parent(row.grant,row.baseline)},
   local:{maximum:2,peak:()=>row.peak,footprintMegabytes:row.localFootprint??0,floorMegabytes:0,sample:sampleHost,pollMilliseconds:1,now:()=>checks*5000},
   ...row.fleet?{fleet:{maximum:2,slots:async()=>polls++===0?row.fleet!.slots:row.fleet!.laterSlots??row.fleet!.slots,footprintMegabytes:row.fleet.footprint,floorMegabytes:0,sample:sampleHost,waitForCompatibleSlot:row.fleet.waitForCompatibleSlot}}:{},
   run:async(_pass,slot)=>{routes.push(slot);if(slot==="fleet"&&row.fleet?.fails)throw new jobs.FleetPlacementFailedErrorV1("lost test worker");},
  });
 }catch(error){refusal=error;}
 expect(routes).toEqual(row.result.routes.map(routeOf));
 if(row.result.required===undefined)expect(refusal).toBeUndefined();
 else{
  expect(refusal).toMatchObject({name:"ParentComponentMemoryRequiredErrorV1",requiredMegabytes:row.result.required,allowanceMegabytes:row.grant});
  expect(checks).toBeLessThanOrEqual(row.fleet?.fails?2:row.fleet?.waitForCompatibleSlot?7:1);
  if(row.fleet?.waitForCompatibleSlot&&!row.fleet.laterSlots)expect(polls).toBe(7);
 }
});
test("cancellation retires a busy fleet wait before any pass is started",async()=>{
 const signal=new AbortController(),memory=parent(100),reason=new Error("cancel the funded wait"),routes:jobs.PassJobSlotV1[]=[];
 const work=jobs.runPassesInJobSlotsV1([pass],{
  jobs:3,check:()=>undefined,parentMemory:memory,signal:signal.signal,
  local:{maximum:1,peak:()=>360,floorMegabytes:0,sample},
  fleet:{maximum:1,slots:async()=>0,footprintMegabytes:64,floorMegabytes:0,sample,waitForCompatibleSlot:true,report:(_pass,event)=>{
   expect(event.waitsFor).toBe("fleet");
   signal.abort(reason);
  }},
  run:async(_pass,slot)=>{routes.push(slot);},
 });
 await expect(work).rejects.toBe(reason);
 expect(routes).toEqual([]);
 expect(memory.remainingMegabytes).toBe(100);
});
test("independent profile lanes share remaining parent memory and release it on settlement",async()=>{
 const memory=parent(600), gates=new Map<string,()=>void>(), started:string[]=[];
 const lane=(id:string)=>jobs.runPassesInJobSlotsV1([{...pass,id}],{
  jobs:1,check:()=>undefined,parentMemory:memory,local:{maximum:1,peak:()=>400,floorMegabytes:0,sample,pollMilliseconds:1},
  run:row=>new Promise<void>(resolve=>{started.push(row.id);gates.set(row.id,resolve);}),
 });
 const first=lane("first"), second=lane("second");
 await new Promise(resolve=>setTimeout(resolve,15));
 const before=[...started];
 gates.get("first")?.();
 for(let i=0;i<20&&!gates.has("second");i++)await new Promise(resolve=>setTimeout(resolve,1));
 gates.get("second")?.();
 await Promise.all([first,second]);
 expect(before).toEqual(["first"]);
 expect(started).toEqual(["first","second"]);
});
test("an affordable fleet fallback waits for the shared grant, then acquires the full local reservation",async()=>{
 const memory=parent(600),routes:jobs.PassJobSlotV1[]=[];
 let releaseFirst!:()=>void,announceFirst!:()=>void,announceFleet!:()=>void;
 const held=new Promise<void>(resolve=>{releaseFirst=resolve;}),firstStarted=new Promise<void>(resolve=>{announceFirst=resolve;}),fleetStarted=new Promise<void>(resolve=>{announceFleet=resolve;});
 const common={jobs:1,check:()=>undefined,parentMemory:memory,local:{maximum:1,peak:()=>400,floorMegabytes:0,sample,pollMilliseconds:1}};
 const first=jobs.runPassesInJobSlotsV1([pass],{...common,run:async()=>{announceFirst();await held;}});
 await firstStarted;
 const second=jobs.runPassesInJobSlotsV1([pass],{
  ...common,fleet:{maximum:1,slots:async()=>1,footprintMegabytes:64,floorMegabytes:0,sample},
  run:async(_pass,slot)=>{routes.push(slot);if(slot==="fleet"){announceFleet();throw new jobs.FleetPlacementFailedErrorV1("lost test worker");}expect(memory.remainingMegabytes).toBe(200);},
 });
 await fleetStarted;
 await Bun.sleep(10);
 const waiting=[...routes];
 releaseFirst();
 await Promise.all([first,second]);
 expect(waiting).toEqual(["fleet"]);
 expect(routes).toEqual(["fleet","local"]);
 expect(memory.remainingMegabytes).toBe(600);
});
