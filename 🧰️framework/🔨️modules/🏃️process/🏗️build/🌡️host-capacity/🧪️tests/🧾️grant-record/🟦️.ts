import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {hostGrowthLabelV1,isHostCapacityV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

/** 🧾️ Every grant row a writer of the gate leaves on disk is read back by the current reader: the row is exactly a schema `Grant`, its measurements live in the `<id>.stats` sidecar (`GrantStats`), and `readHostCapacityStatusV1` lists it instead of refusing it as unrecognized. */
const policy=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hcg-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host:HostSampleV1={version:1,atMs:0,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
const request=(over:object)=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:2000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:object):HostCapacityOptionsV1=>({directory,policy,request:request({}),control,environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:{...host,atMs:Date.now()},swappedOutMegabytes:0}),...over});
const read=(path:string)=>JSON.parse(readFileSync(path,"utf8")) as Record<string,unknown>;
const parent=newHostCapacityIdV1();
const cases:Array<[string,object]>=[
 ["a normal grant",{request:request({})}],
 ["a unit growth of a parent grant",{request:request({operation:"cargo-growth",label:hostGrowthLabelV1(parent)})}],
 ["the supplementary reservation of a component pass inside a parent grant",{request:request({operation:"cargo-growth",label:hostGrowthLabelV1(parent),fixedMegabytes:759,priority:"interactive"})}],
 ["a labelled grant",{request:request({label:"component:energy"})}]
];
for(const [name,over] of cases)test(`${name}: the row on disk is a Grant, its measurements are a GrantStats sidecar and status reads it`,async()=>{
 const directory=fresh();
 await withHostCapacityV1(options(directory,over),async handle=>{
  const row=join(directory,"grants",`${handle.id}.json`);
  await sleep(policy.wait.heartbeatMilliseconds*8);
  const value=read(row);
  expect(isHostCapacityV1("Grant",value),JSON.stringify(value)).toBe(true);
  for(const key of ["peakMegabytes","observations","deviationMegabytes"])expect(value).not.toHaveProperty(key);
  const format=read(join(directory,"grants",`${handle.id}.format`));
  expect(isHostCapacityV1("GrantFormat",format)).toBe(true);
  expect(format.format).toBe(1);
  const stats=join(directory,"grants",`${handle.id}.stats`);
  expect(existsSync(stats)).toBe(true);
  expect(isHostCapacityV1("GrantStats",read(stats))).toBe(true);
  const status=await readHostCapacityStatusV1({directory,policy,control,sample:async()=>({host:{...host,atMs:Date.now()},swappedOutMegabytes:0})});
  expect(status.grants.map(grant=>grant.id)).toContain(handle.id);
  expect(status.grants.find(grant=>grant.id===handle.id)!.foreign).toBeUndefined();
 });
 expect(readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".json")||name.endsWith(".format"))).toEqual([]);
});
