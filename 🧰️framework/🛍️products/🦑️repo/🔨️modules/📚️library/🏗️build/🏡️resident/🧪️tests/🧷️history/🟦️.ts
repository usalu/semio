/** 🧷️ Contract: a learned value never makes a machine unable to start. On a simulated 16 GB host, whatever history the host estimate store holds for the slice class (none, one spike, sixteen spikes of 90 GB, spikes among sane runs, a damaged file) the booking of the graph step stays within the share of the physical memory the gate can ever grant, the real gate admits it, and the slice peak lives in that one store: no second learned file exists. The plan pays with the grant of that one reservation, and a service limit that leaves less than the booking caps the booking.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,hostProfileV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {reserveGraphPlanCapacityV1} from "../../../🌡️host-capacity/🟦️.ts";
import {HOST_SLICE_CLASS_V1,learnedHostSlicePeakV1,recordHostSlicePeakV1} from "../../../🗓️scheduler/📈️learn/🟦️.ts";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {sliceCeilingMegabytesV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {HOST_POLICY_V1} from "../../🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number;learnedGrowthBasisPoints:number},roots:string[]=[];
const fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"slice-history-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=(available:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:2});
const histories:{name:string;prepare:(environment:NodeJS.ProcessEnv)=>Promise<void>}[]=[
 {name:"no history",prepare:async()=>undefined},
 {name:"one spike of 90 GB",prepare:async environment=>{await recordHostSlicePeakV1(environment,90000,1e9);}},
 {name:"sixteen spikes of 90 GB",prepare:async environment=>{for(let index=0;index<16;index++)await recordHostSlicePeakV1(environment,90000,1e9);}},
 {name:"spikes among sane runs",prepare:async environment=>{for(let index=0;index<16;index++)await recordHostSlicePeakV1(environment,index%3===0?70000:5200,1e9);}},
 {name:"a damaged store",prepare:async environment=>{writeFileSync(join(environment.SEMIO_HOST_CAPACITY_DIRECTORY!,"estimates.json"),"{\"version\":1,\"classes\":");}},
];

for(const history of histories)test("a simulated 16 GB host with "+history.name+" books within the share the gate can grant and the gate admits the graph step",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384",SEMIO_HOST_CAPACITY_DIRECTORY:directory};
 await history.prepare(environment);
 const hostMegabytes=hostProfileV1(environment).totalMegabytes,ceiling=sliceCeilingMegabytesV1({hostMegabytes,hostCapBasisPoints:elasticity.hostCapBasisPoints,maximumMegabytes:elasticity.maximumMegabytes});
 expect(hostMegabytes).toBe(16384);expect(ceiling).toBe(6553);
 expect(learnedHostSlicePeakV1(environment,ceiling)).toBeLessThanOrEqual(ceiling);
 const asked:number[]=[],granted:number[]=[],listeners:Record<string,(value:unknown)=>void>={};
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,hostMegabytes,elasticity,unlearnedMegabytes:HOST_POLICY_V1.sliceUnlearnedMegabytes,
  learned:{read:()=>learnedHostSlicePeakV1(environment,ceiling),write:()=>undefined},
  reserve:async({megabytes,minimumMegabytes,signal}:{megabytes:number;minimumMegabytes?:number;signal?:AbortSignal})=>{asked.push(megabytes);const reservation=await reserveGraphPlanCapacityV1({megabytes,minimumMegabytes,signal,environment,directory,policy:fast,write:()=>undefined,sample:async()=>({host:host(7500),swappedOutMegabytes:0})});granted.push(reservation.megabytes);return reservation;},
  createWorker:()=>{queueMicrotask(()=>listeners.message?.({type:"done",graph:{}}));return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 await Promise.race([pool.run({root:"/tmp/history"}),new Promise((_,reject)=>setTimeout(()=>reject(new Error("the graph step was not admitted")),20000))]);await pool.close();
 expect(asked.length).toBe(1);expect(asked[0]!).toBeLessThanOrEqual(6553);expect(asked[0]!).toBeGreaterThanOrEqual(elasticity.minimumMegabytes);
 expect(granted[0]!).toBeGreaterThan(0);expect(granted[0]!).toBeLessThanOrEqual(asked[0]!);
},60000);

test("the slice peak is learned in the host estimate store only, bounded by the ceiling, and the temp-file learner is gone",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory};
 expect(learnedHostSlicePeakV1(environment,6553)).toBe(0);
 await recordHostSlicePeakV1(environment,90000,6553);await recordHostSlicePeakV1(environment,4000,6553);
 expect(learnedHostSlicePeakV1(environment,6553)).toBe(0);
 for(let index=0;index<12;index++)await recordHostSlicePeakV1(environment,index===0?90000:4000,6553);
 const learned=learnedHostSlicePeakV1(environment,6553);
 expect(learned).toBeGreaterThanOrEqual(4000);expect(learned).toBeLessThanOrEqual(6553);
 expect(learnedHostSlicePeakV1(environment,3000)).toBe(3000);
 expect(JSON.parse(readFileSync(join(directory,"estimates.json"),"utf8")).classes[HOST_SLICE_CLASS_V1].samples.length).toBeGreaterThan(1);
 const source=readFileSync(new URL("../../../⌨️command/🌐️host/🟦️.ts",import.meta.url),"utf8");
 expect(source).not.toContain("semio-slice-peak");
});

test("a service limit that leaves less than the booking caps the booking, a limit that lends its own cgroup does not, and a load never learns from a cancelled run",async()=>{
 const asked:number[]=[],written:number[]=[],listeners:Record<string,(value:unknown)=>void>={};
 const harness=(headroom:()=>unknown,learnedWrite:(megabytes:number)=>void,finalMessage:unknown)=>createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,marginMegabytes:512,headroom:headroom as never,learned:{read:()=>6627,write:learnedWrite},observe:async()=>3000,sampleMilliseconds:5,
  reserve:async({megabytes}:{megabytes:number})=>{asked.push(megabytes);return{megabytes,attach:()=>undefined,release:()=>undefined};},
  createWorker:()=>{setTimeout(()=>listeners.message?.(finalMessage),60);return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 const MiB=1048576,limited=()=>({limitBytes:7125*MiB,currentBytes:5500*MiB,availableBytes:7500*MiB});
 const first=harness(limited,megabytes=>written.push(megabytes),{type:"done",graph:{}});await first.run({root:"/tmp/limit"});await first.close();
 expect(asked[0]).toBe(7125-5500-512);expect(written).toEqual([3000]);
 const open=harness(()=>({limitBytes:Infinity,currentBytes:5500*MiB,availableBytes:7500*MiB}),megabytes=>written.push(megabytes),{type:"done",graph:{}});await open.run({root:"/tmp/open"});await open.close();
 expect(asked[1]).toBe(6553);
 const controller=new AbortController(),cancelled=harness(()=>({limitBytes:Infinity,currentBytes:0,availableBytes:7500*MiB}),megabytes=>written.push(megabytes),{type:"progress"});
 const running=cancelled.run({root:"/tmp/cancel"},{signal:controller.signal}).then(()=>undefined,error=>error);await new Promise(done=>setTimeout(done,30));controller.abort(new Error("stop"));await running;await cancelled.close();
 expect(written).toEqual([3000,3000]);
});
