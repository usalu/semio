/** 🧮️ Contract: a graph plan books the memory of its slice worker in the machine-wide gate before the worker starts and holds it until it is released, so a build that does not fit beside it queues behind it in the fair queue, starts when the plan ends, and a plan that waits can be cancelled without leaving a waiter behind. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {reserveGraphPlanCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"plan-capacity-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:30000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const grants=(directory:string)=>existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".json")):[];
const queue=(directory:string)=>existsSync(join(directory,"queue"))?readdirSync(join(directory,"queue")):[];
const until=async(test:()=>boolean,milliseconds=3000)=>{const started=Date.now();while(!test()&&Date.now()-started<milliseconds)await new Promise(done=>setTimeout(done,10));return test();};
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const build=(directory:string,megabytes:number,body:()=>Promise<void>)=>withHostCapacityV1({directory,policy:fast,request:{id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample},body);

test("with the gate off a plan reserves nothing and is never refused",async()=>{
 const directory=fresh(),reservation=await reserveGraphPlanCapacityV1({megabytes:6000,environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY:"off"},directory,policy:fast,write:()=>undefined});
 reservation.attach(process.pid);reservation.release();reservation.release();expect(grants(directory)).toEqual([]);
},60000);

test("a plan books its megabytes, a build that does not fit beside it queues behind it and starts when the plan is released, and the booking records the worker it observes",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},order:string[]=[];
 const reservation=await reserveGraphPlanCapacityV1({megabytes:20000,environment,directory,policy:fast,write:()=>undefined,sample});
 expect(grants(directory).length).toBe(1);
 const grant=JSON.parse(readFileSync(join(directory,"grants",grants(directory)[0]!),"utf8"));
 expect(grant.label).toBe("graph-plan");expect(grant.reservedMegabytes).toBe(20000);
 reservation.attach(process.pid);
 const queued=build(directory,20000,async()=>{order.push("build");});
 expect(await until(()=>queue(directory).length===1)).toBe(true);await new Promise(done=>setTimeout(done,200));expect(order).toEqual([]);
 order.push("release");reservation.release();await queued;
 expect(order).toEqual(["release","build"]);expect(await until(()=>grants(directory).length===0)).toBe(true);
},60000);

test("a build that fits beside the plan runs at once",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},reservation=await reserveGraphPlanCapacityV1({megabytes:6000,environment,directory,policy:fast,write:()=>undefined,sample});
 let ran=false;await build(directory,6000,async()=>{ran=true;expect(grants(directory).length).toBe(2);});expect(ran).toBe(true);reservation.release();
 expect(await until(()=>grants(directory).length===0)).toBe(true);
},60000);

test("a plan that waits for memory is cancelled by its signal, rejects with the reason and leaves no waiter behind",async()=>{
 const directory=fresh(),controller=new AbortController(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"};
 const waiting=reserveGraphPlanCapacityV1({megabytes:6000,environment,directory,policy:fast,write:()=>undefined,signal:controller.signal,sample:async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0})}).then(()=>undefined,error=>error);
 expect(await until(()=>queue(directory).length===1)).toBe(true);controller.abort(new Error("plan cancelled"));
 expect((await waiting).message).toBe("plan cancelled");expect(queue(directory)).toEqual([]);expect(grants(directory)).toEqual([]);
},60000);
