/** 🦀️ Contract of cargo admission: a cargo run is sized by its tool alone, jobs times the footprint of one job, so it is granted the jobs the spare memory carries (at least one) and never waits behind the constant planning floor that graph runs learned.
 * @see ../🛰️planned-grant/🟦️.ts
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/cargo-admission-scaling.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {recordHostOutcomeV1,updateHostEstimateStoreV1,emptyHostEstimateStoreV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {withBuildCapacityV1} from "../../🟦️.ts";
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"cs-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:cases.host.cores,totalMegabytes:cases.host.totalMegabytes,availableMegabytes:cases.host.availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:100,openFilesMaximum:1000000,attachedImages:null});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool"));

for(const item of cases.runs)test(`cargo run: ${item.name}`,async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},abort=new AbortController();
 await updateHostEstimateStoreV1(directory,policy,()=>Array.from({length:cases.learnedPlanRuns},(_,index)=>index).reduce((store,index)=>recordHostOutcomeV1(policy,store,["phase:plan"],{operation:"run",jobs:1,peakMegabytes:cases.learnedPlanRatioBasisPoints*policy.estimates.phaseReferenceMegabytes/10000,defaultMegabytes:policy.estimates.phaseReferenceMegabytes,activeMilliseconds:1000,peakAtMilliseconds:500,endedAtMs:Date.now()-index}),emptyHostEstimateStoreV1()));
 const timer=setTimeout(()=>abort.abort(),4000);
 let jobs=0,parallelism="";
 try{
  await withBuildCapacityV1(item.args,environment,async granted=>{jobs=granted.handle!.jobs;parallelism=granted.args.find(argument=>argument.startsWith("--parallelism="))!;},{directory,policy,strip,sample,observe:async()=>45,write:()=>undefined,exitCode:()=>0,control:{signal:abort.signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined}});
 }finally{clearTimeout(timer);}
 expect(jobs).toBeGreaterThanOrEqual(item.jobsAtLeast);
 expect(jobs).toBeLessThan(item.jobsBelow);
 expect(parallelism).toBe(`--parallelism=${jobs}`);
},30000);
