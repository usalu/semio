/** 🪆️ Contract: a graph plan asked for by a command that runs under a live grant is charged to that grant and never queues at the machine behind it; what the grant cannot carry is refused at once and declared to the holder, so the command can never wait for itself. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {hostNeedPathV1,newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {reserveGraphPlanCapacityV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"nested-capacity-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:fixture.hostMegabytes,availableMegabytes:12000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const rows=(directory:string)=>existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".json")):[];
const outer=(directory:string,body:(id:string,environment:NodeJS.ProcessEnv)=>Promise<void>)=>{
 const id=newHostCapacityIdV1();
 return withHostCapacityV1({directory,policy:fast,request:{id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:fixture.parentMegabytes,perJobMegabytes:0,cpuPerJob:1,minimumMegabytes:fixture.parentMegabytes,label:"outer"},control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample},async()=>body(id,{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory}));
};
const plan=(directory:string,environment:NodeJS.ProcessEnv,parentGrant:string,over:Partial<typeof fixture.plan>={})=>reserveGraphPlanCapacityV1({...fixture.plan,...over,parentGrant,environment,directory,policy:fast,write:()=>undefined,sample});

test("a plan the parent grant can carry only in part is admitted within the grant with what it holds, books no second grant, and gives the room back",async()=>{
 const directory=fresh();
 await outer(directory,async(id,environment)=>{
  const first=await plan(directory,environment,id);
  expect(first.megabytes).toBe(fixture.expected.reducedGrantMegabytes);
  expect(rows(directory)).toEqual([`${id}.json`]);
  await expect(plan(directory,environment,id,{megabytes:fixture.second.megabytes,minimumMegabytes:fixture.second.minimumMegabytes})).rejects.toThrow(/enclosing grant outer holds 4500 MB and only 0 MB are free/u);
  first.attach(process.pid);first.release();first.release();
  const second=await plan(directory,environment,id,{megabytes:fixture.second.megabytes,minimumMegabytes:fixture.second.minimumMegabytes});
  expect(second.megabytes).toBe(fixture.expected.afterReleaseGrantMegabytes);second.release();
  expect(rows(directory)).toEqual([`${id}.json`]);
 });
},60000);

test("a plan whose minimum exceeds the parent grant is refused at once, never waited for, and the need is declared to the holder",async()=>{
 const directory=fresh();
 await outer(directory,async(id,environment)=>{
  const started=Date.now();
  await expect(plan(directory,environment,id,{minimumMegabytes:fixture.plan.megabytes})).rejects.toThrow(/needs at least 6553 MB.*enclosing grant outer holds 4500 MB/su);
  expect(Date.now()-started).toBeLessThan(3000);
  expect(Number(readFileSync(hostNeedPathV1(directory,id),"utf8"))).toBe(fixture.expected.refusedMinimumMegabytes);
  expect(rows(directory)).toEqual([`${id}.json`]);
 });
},60000);

test("a parent that is gone is no parent: the plan queues at the gate like any other",async()=>{
 const directory=fresh(),reservation=await plan(directory,{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory},newHostCapacityIdV1(),{megabytes:500,minimumMegabytes:500});
 expect(rows(directory).length).toBe(1);reservation.release();
},60000);
