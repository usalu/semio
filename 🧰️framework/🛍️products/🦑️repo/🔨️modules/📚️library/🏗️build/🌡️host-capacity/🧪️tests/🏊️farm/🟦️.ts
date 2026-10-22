/** 🏊️ Contract: the graph farm asks the machine-wide gate for its workers in the fair queue beside the grant its invocation holds, is answered with the jobs and megabytes the gate admits, holds the grant until it is released (no timer gives it back) and can cancel a wait that is never answered. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,realpathSync,rmSync} from "node:fs";
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
 const directory=fresh(),request=requestBuildFarmCapacityV1({preferredJobs:6,environment:{SEMIO_HOST_CAPACITY:"off"},directory,policy:fast,write:()=>{}}),granted=await request.granted;
 expect(granted).toEqual({jobs:6,megabytes:fast.kinds.build.megabytes});request.release();
 expect(grants(directory)).toEqual([]);
},60000);

test("a fresh grant is requested beside the grant the invocation holds, answered with jobs and megabytes, held until released and then given back",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={},sample=async()=>({host:host(),swappedOutMegabytes:0}),lines:string[]=[];
 await withBuildCapacityV1(["show","projects"],environment,async outer=>{
  expect(outer.handle).toBeDefined();expect(grants(directory).length).toBe(1);
  const request=requestBuildFarmCapacityV1({preferredJobs:6,environment,directory,policy:fast,write:line=>lines.push(line),sample}),granted=await request.granted;
  expect(granted).toBeDefined();expect(granted!.jobs).toBeGreaterThanOrEqual(1);expect(granted!.jobs).toBeLessThanOrEqual(6);expect(granted!.megabytes).toBe(fast.kinds.build.megabytes);
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
  const directory=fresh(),controller=new AbortController(),request=requestBuildFarmCapacityV1({preferredJobs:6,environment:{},directory,policy:fast,write:()=>{},signal:controller.signal,sample:async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0})});
  await new Promise(done=>setTimeout(done,200));
  expect(queue(directory).length).toBe(1);
  if(how==="release")request.release();else controller.abort(new Error("cancelled by law"));
  expect(await request.granted).toBeUndefined();
  expect(queue(directory)).toEqual([]);expect(grants(directory)).toEqual([]);
 }
},60000);
