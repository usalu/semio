/** 📣️ Tests that a run which declares what it needs teaches its class at least that, however it ended. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {declareHostCapacityNeedV1,emptyHostEstimateStoreV1,estimateHostRatioV1,hostEstimateLineageV1,hostRunAfterNeedV1,learnableHostRunV1,newHostCapacityIdV1,readHostCapacityPolicyV1,recordHostOutcomeV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostRequestV1,type HostRunV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "../📐️estimates/🧫️fixtures/🔣️.json";
import fixtures from "./🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,estimates:fixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(id:string):HostRequestV1=>({id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:512,perJobMegabytes:0,cpuPerJob:1});
const options=(directory:string,id:string,learn:(run:HostRunV1)=>void):HostCapacityOptionsV1=>({directory,policy,request:request(id),control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),learn});

for(const row of fixtures.runs)test(`Law: ${row.name}`,()=>{
 const next=hostRunAfterNeedV1({peakMegabytes:row.peak,status:row.status as "ok"},row.need);
 expect<unknown>({peakMegabytes:next.peakMegabytes,status:next.status,limited:(next as {limited?:boolean}).limited===true,learnable:learnableHostRunV1(next.status as "ok",{limited:(next as {limited?:boolean}).limited===true})}).toEqual(row.result);
});

test("Law: a class that once ran small and then refused a known need is sized at that need or more on its next run",()=>{
 const lineage=hostEstimateLineageV1("run","component"),defaultMegabytes=8190;
 let store=recordHostOutcomeV1(policy,emptyHostEstimateStoreV1(),lineage,{operation:"run",jobs:1,peakMegabytes:1500,defaultMegabytes,activeMilliseconds:60000,peakAtMilliseconds:30000,endedAtMs:1000,keepPeak:true});
 const before=estimateHostRatioV1(policy,store,lineage,2000).ratioBasisPoints;
 const refused=hostRunAfterNeedV1({peakMegabytes:400,status:"failed" as const},3757);
 expect(learnableHostRunV1(refused.status,{limited:refused.limited})).toBe(true);
 for(let index=0;index<policy.estimates.minimumSamples;index++)store=recordHostOutcomeV1(policy,store,lineage,{operation:"run",jobs:1,peakMegabytes:refused.peakMegabytes,defaultMegabytes,activeMilliseconds:1000,peakAtMilliseconds:500,endedAtMs:2000+index,keepPeak:true});
 const after=estimateHostRatioV1(policy,store,lineage,3000).ratioBasisPoints;
 expect(after).toBeGreaterThan(before);
 expect(Math.round(after*defaultMegabytes/10000)).toBeGreaterThanOrEqual(3757);
});

test("A run declares its need beside its grant, the largest declaration wins, and the holder hands it to learn even when the body failed",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hn-"));roots.push(directory);
 const runs:HostRunV1[]=[],id=newHostCapacityIdV1();
 await expect(withHostCapacityV1(options(directory,id,run=>{runs.push(run);}),async handle=>{
  const environment={LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:handle.id};
  expect(declareHostCapacityNeedV1(environment,3757)).toBe(true);
  expect(declareHostCapacityNeedV1(environment,2000)).toBe(true);
  await new Promise(done=>setTimeout(done,50));
  throw new Error("refused");
 })).rejects.toThrow("refused");
 expect(runs).toHaveLength(1);
 expect(runs[0]!.status).toBe("failed");
 expect(runs[0]!.needMegabytes).toBe(3757);
 expect(declareHostCapacityNeedV1({LANG:"en_US.UTF-8"},3757)).toBe(false);
});
