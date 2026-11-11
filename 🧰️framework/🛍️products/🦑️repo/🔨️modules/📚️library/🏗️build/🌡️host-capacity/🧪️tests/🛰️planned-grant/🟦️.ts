/** 🛰️ Contract of the door for runs the host service plans: a run waits for planning and graph derivation under a small quiet grant, grows it into the full request only when it executes, and so never starves the graph worker it waits for.
 * @see ./🥒️.feature
 * @see ../🤫️quiet/🟦️.ts
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/repo-tmp-and-admission.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,withHostCapacityV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {admitPlannedBuildCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"pg-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:cases.host.cores,totalMegabytes:cases.host.totalMegabytes,availableMegabytes:cases.host.availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const rowOf=async(directory:string,id:string)=>(await readHostCapacityStatusV1({directory,policy,control:control(),sample})).grants.find(item=>item.id===id)!;

for(const item of cases.runs)test(`planned run: ${item.name}`,async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},ports={directory,policy,strip,sample,plannedByService:true,observe:async()=>45,write:()=>undefined,exitCode:()=>0};
 let waiting:Awaited<ReturnType<typeof rowOf>>|undefined,executing:Awaited<ReturnType<typeof rowOf>>|undefined,parallel:string|undefined;
 await withBuildCapacityV1(item.args,environment,async granted=>{
  waiting=await rowOf(directory,granted.handle!.id);
  await admitPlannedBuildCapacityV1(undefined,environment);
  executing=await rowOf(directory,granted.handle!.id);parallel=environment["SEMIO_BUILD_PARALLEL"];
 },ports);
 expect(waiting!.quiet===true,"waits quiet").toBe(item.waiting.quiet);
 expect(waiting!.reservedMegabytes,"waits small").toBeLessThan(item.waiting.reservedBelowMegabytes);
 expect(executing!.quiet===true,"executes full").toBe(item.executing.quiet);
 expect(executing!.reservedMegabytes,"executes with the full request").toBeGreaterThanOrEqual(item.executing.reservedAtLeastMegabytes);
 expect(Number(parallel),"the plan follows the granted jobs").toBeGreaterThanOrEqual(1);
},30000);

test("the graph worker beside a planned run is admitted at once, because the run holds no execution memory while it waits",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"};let waited=Infinity;
 await withBuildCapacityV1(cases.runs[0]!.args,environment,async()=>{
  const started=Date.now();
  await withHostCapacityV1({directory,policy,request:{id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:cases.worker.megabytes,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample},async()=>{waited=Date.now()-started;});
 },{directory,policy,strip,sample,plannedByService:true,observe:async()=>45,write:()=>undefined,exitCode:()=>0});
 expect(waited).toBeLessThan(cases.worker.admittedWithinMilliseconds);
},30000);
