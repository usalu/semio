/** 👥️ Contract: the door lets three agents build side by side. A run whose class is sized by the peaks its own runs measured takes no ramp slot (a class that measured too little still does), and the project farm asks for its workers as an elastic request of the run's agent session whose grant observes nothing of its own, so it never makes another agent's run wait.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔼️upgrade/👥️agents/🟦️.ts
 * @see ../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-agents16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {readHostCapacityPolicyV1,recordHostOutcomeV1,updateHostEstimateStoreV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {buildCapacityAskV1,requestBuildFarmCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"ag16-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool"));
const args=["run","@x/y:build","--parallel=2"],lineage=buildCapacityAskV1(strip(args),{},policy)!.lineage;
const rows=(directory:string,child:"grants"|"queue")=>readdirSync(join(directory,child)).filter(name=>name.endsWith(".json")).map(name=>JSON.parse(readFileSync(join(directory,child,name),"utf8")));

test("A run whose class is sized by the peaks its own runs measured takes no ramp slot, a class that measured too little still does",async()=>{
 const seen:Record<number,boolean|undefined>={};
 for(const runs of [2,4]){
  const directory=fresh();
  for(let index=0;index<runs;index++)await updateHostEstimateStoreV1(directory,policy,store=>recordHostOutcomeV1(policy,store,lineage,{operation:"run",jobs:2,peakMegabytes:900,defaultMegabytes:1536+2*2048,activeMilliseconds:20_000,peakAtMilliseconds:5_000,endedAtMs:Date.now()-3_600_000*(index+2),keepPeak:true}));
  await withBuildCapacityV1(args,{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},async granted=>{
   const row=rows(directory,"grants").find(grant=>grant.id===granted.handle!.id);
   expect(granted.handle!.reservedMegabytes,`${runs} runs: the reservation is shrunk below the default`).toBeLessThan(1536+2*2048);
   seen[runs]=row.rampSlot;
  },{directory,policy,strip,tool:"bazel",sample:async()=>({host:host(14000),swappedOutMegabytes:0}),observe:async()=>500,write:()=>undefined,exitCode:()=>0});
 }
 expect(seen).toEqual({2:true,4:false});
});

test("The project farm asks for its workers as an elastic request of the run's agent session, and its grant observes nothing of its own",async()=>{
 const directory=fresh(),environment={SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"};
 const waiting=requestBuildFarmCapacityV1({preferredJobs:2,environment,directory,policy,write:()=>undefined,sample:async()=>({host:host(3000),swappedOutMegabytes:0})});
 await sleep(80);
 const [waiter]=rows(directory,"queue");
 expect([waiter.request.elastic,/^[0-9a-f]{12}\.[a-z][a-z0-9]{0,11}$/u.test(waiter.session)]).toEqual([true,true]);
 waiting.release();expect(await waiting.granted).toBeUndefined();
 const held=requestBuildFarmCapacityV1({preferredJobs:2,environment,directory,policy,write:()=>undefined,sample:async()=>({host:host(14000),swappedOutMegabytes:0})});
 expect((await held.granted)?.jobs).toBeGreaterThanOrEqual(1);
 await sleep(150);
 const [grant]=rows(directory,"grants");
 expect([grant.observedMegabytes,typeof grant.session]).toEqual([0,"string"]);
 held.release();
});
