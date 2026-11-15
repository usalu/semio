/** 🏊️ Contract: the graph farm asks the machine-wide gate for its workers in the fair queue beside the grant its invocation holds, is answered with the jobs and megabytes the gate admits, holds the grant until it is released (no timer gives it back) and can cancel a wait that is never answered. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {getEventListeners} from "node:events";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {requestBuildFarmCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"farm-capacity-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const grants=(directory:string)=>existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")):[];
const queue=(directory:string)=>existsSync(join(directory,"queue"))?readdirSync(join(directory,"queue")):[];
const settled=async(directory:string,count:number)=>{const started=Date.now();while(grants(directory).length!==count&&Date.now()-started<2000)await new Promise(done=>setTimeout(done,10));return grants(directory).length;};

test("with the gate off the farm gets every job it asks for and nothing is reserved",async()=>{
 const directory=fresh(),request=requestBuildFarmCapacityV1({preferredJobs:6,environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY:"off"},directory,policy:fast,write:()=>{}}),granted=await request.granted;
 expect(granted).toEqual({jobs:6,megabytes:fast.kinds.script.megabytes});request.release();
 expect(grants(directory)).toEqual([]);
},60000);

test("the farm books no fixed block, so one worker of the per-job size is the smallest request and fits a host with little spare memory",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},sample=async()=>({host:host({totalMegabytes:16384,availableMegabytes:2048+fast.kinds.script.megabytes+256}),swappedOutMegabytes:0});
 const request=requestBuildFarmCapacityV1({preferredJobs:6,environment,directory,policy:fast,write:()=>{},sample}),granted=await request.granted;
 expect(granted).toBeDefined();expect(granted!.jobs).toBeGreaterThanOrEqual(1);expect(granted!.megabytes).toBe(fast.kinds.script.megabytes);
 const row=JSON.parse(readFileSync(join(directory,"grants",grants(directory)[0]!),"utf8"));expect(row.reservedMegabytes).toBe(granted!.jobs*fast.kinds.script.megabytes);
 request.release();
},60000);

test("a fresh grant is requested beside the grant the invocation holds, answered with jobs and megabytes, held until released and then given back",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},sample=async()=>({host:host(),swappedOutMegabytes:0}),lines:string[]=[];
 await withBuildCapacityV1(["show","projects"],environment,async outer=>{
  expect(outer.handle).toBeDefined();expect(grants(directory).length).toBe(1);
  const request=requestBuildFarmCapacityV1({preferredJobs:6,environment,directory,policy:fast,write:line=>lines.push(line),sample}),granted=await request.granted;
  expect(granted).toBeDefined();expect(granted!.jobs).toBeGreaterThanOrEqual(1);expect(granted!.jobs).toBeLessThanOrEqual(6);expect(granted!.megabytes).toBe(fast.kinds.script.megabytes);
  expect(grants(directory).length).toBe(2);expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBe(outer.handle!.id);
  await new Promise(done=>setTimeout(done,600));
  expect(grants(directory).length).toBe(2);
  request.release();
  expect(await settled(directory,1)).toBe(1);
  expect(lines.length).toBeGreaterThan(0);
 },{directory,policy:fast,strip:args=>args.filter(argument=>!argument.startsWith("--tool")),write:()=>{},sample});
 expect(grants(directory).length).toBe(0);
},60000);

test("a wait the gate never answers is cancelled by releasing the request or by its signal and leaves no waiter behind",async()=>{
 for(const how of["release","signal"] as const){
  const directory=fresh(),controller=new AbortController(),request=requestBuildFarmCapacityV1({preferredJobs:6,environment:{LANG:"en_US.UTF-8"},directory,policy:fast,write:()=>{},signal:controller.signal,sample:async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0})});
  await new Promise(done=>setTimeout(done,200));
  expect(queue(directory).length).toBe(1);
  if(how==="release")request.release();else controller.abort(new Error("cancelled by law"));
  expect(await request.granted).toBeUndefined();
  expect(queue(directory)).toEqual([]);expect(grants(directory)).toEqual([]);
 }
},60000);
test("a request leaves no listener on the long-lived signal of its invocation once it is released, so one farm per chunk never pins the environment of every earlier chunk",async()=>{
 const sample=async()=>({host:host(),swappedOutMegabytes:0}),listeners=(signal:AbortSignal)=>getEventListeners(signal,"abort").length;
 for(const gate of["on","off","waiting"] as const){
  const directory=fresh(),controller=new AbortController();
  for(let round=0;round<5;round++){
   const request=requestBuildFarmCapacityV1({preferredJobs:6,environment:gate==="off"?{SEMIO_HOST_CAPACITY:"off"}:{LANG:"en_US.UTF-8"},directory,policy:fast,write:()=>{},signal:controller.signal,sample:gate==="waiting"?async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0}):sample});
   if(gate==="waiting")await new Promise(done=>setTimeout(done,50));else expect(await request.granted).toBeDefined();
   request.release();await request.granted;
  }
  expect(listeners(controller.signal)).toBe(0);
 }
},60000);

test("on a 16 GB machine the workers of the farm are admitted inside the reservation of the build that asks, charged to it, and the host books the build once",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},machine={available:12000},sample=async()=>({host:host({totalMegabytes:16384,availableMegabytes:machine.available}),swappedOutMegabytes:0}),lines:string[]=[];
 await withBuildCapacityV1(["show","projects"],environment,async outer=>{
  const reserved=outer.handle!.reservedMegabytes;
  machine.available=3000;
  const request=requestBuildFarmCapacityV1({preferredJobs:8,environment,directory,policy:fast,write:line=>lines.push(line),sample}),granted=await request.granted;
  expect(granted,"admitted although the machine has no free memory left above the floor").toBeDefined();
  expect(granted!.jobs).toBeGreaterThanOrEqual(1);expect(granted!.megabytes).toBe(fast.kinds.script.megabytes);
  const rows=grants(directory).map(name=>JSON.parse(readFileSync(join(directory,"grants",name),"utf8")) as {id:string;holder?:string;reservedMegabytes:number});
  const child=rows.find(row=>row.holder!==undefined)!;
  expect(child.holder,"the grant of the workers names the build").toBe(outer.handle!.id);
  expect(rows.filter(row=>row.holder===undefined).reduce((sum,row)=>sum+row.reservedMegabytes,0),"the host books the reservation of the build once").toBe(reserved);
  expect(reserved-child.reservedMegabytes,"the parent keeps at least what the lend rule leaves it").toBeGreaterThanOrEqual(Math.floor(reserved*0.45));
  expect(lines.some(line=>/inside the reservation of/u.test(line))).toBe(true);
  request.release();
 },{directory,policy:fast,strip:args=>args.filter(argument=>!argument.startsWith("--tool")),write:line=>lines.push(line),sample});
},60000);
