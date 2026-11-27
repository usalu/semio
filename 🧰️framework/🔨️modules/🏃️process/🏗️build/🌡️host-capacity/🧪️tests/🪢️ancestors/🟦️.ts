/** 🪢️ Contract: a request that can only wait for the grant of an ancestor (the enclosing command that waits for it) is refused with a bilingual error instead of queueing for ever. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {formatHostCapacityErrorV1,hostAncestorDeadlockV1,newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostRequestV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,budgetBasisPoints:fixture.policy.budgetBasisPoints},load:{...decisionFixture.policy.load,overrideAfterMilliseconds:fixture.policy.overrideAfterMilliseconds},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:16384,availableMegabytes:16000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal:AbortSignal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(megabytes:number,label:string):HostRequestV1=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,minimumMegabytes:megabytes,label});
const options=(directory:string,asked:HostRequestV1,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:asked,control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});

for(const row of fixture.cases)test(`Law: ${row.name}`,()=>{
 expect(hostAncestorDeadlockV1(policy,{totalMegabytes:row.totalMegabytes,floorMegabytes:row.floorMegabytes,minimumMegabytes:row.minimumMegabytes,ancestors:row.ancestors,ancestorMegabytes:row.ancestorMegabytes,otherRunning:row.otherRunning,waitedMilliseconds:row.waitedMilliseconds})).toEqual({deadlocked:row.deadlocked,possibleMegabytes:row.possibleMegabytes});
});

test("a request naming the grant it runs in is refused at once when that grant leaves too little, never queued behind it",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"anc-"));roots.push(directory);
 const outer=request(8000,"outer");
 const started=Date.now();
 await withHostCapacityV1(options(directory,outer),async()=>{
  await expect(withHostCapacityV1(options(directory,request(6553,"nested"),{parentGrant:outer.id}),async()=>undefined)).rejects.toThrow(/enclosing command outer holds 8000 MB.*wait for itself for ever/su);
 });
 expect(Date.now()-started).toBeLessThan(5000);
});

test("the grant of a parent process is found without being named, so a nested request in the process tree never waits for it",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"anc-"));roots.push(directory);
 await withHostCapacityV1(options(directory,request(8000,"outer")),async()=>{
  await expect(withHostCapacityV1(options(directory,request(6553,"nested"),{ppid:process.pid}),async()=>undefined)).rejects.toThrow(/enclosing command outer/u);
 });
});

test("an unrelated holder that is no ancestor is waited for, as before",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"anc-"));roots.push(directory);
 const abort=new AbortController();
 await withHostCapacityV1(options(directory,request(8000,"other")),async()=>{
  const waiting=withHostCapacityV1(options(directory,request(6553,"nested"),{control:control(abort.signal)}),async()=>undefined);
  const outcome=await Promise.race([waiting.then(()=>"admitted",error=>String(error.message)),new Promise<string>(done=>setTimeout(()=>done("still waiting"),400))]);
  abort.abort(new Error("stop"));await waiting.catch(()=>undefined);
  expect(outcome).toBe("still waiting");
 });
});

test("the refusal is bilingual and names the way out",()=>{
 const values={operation:"run",minimum:6553,holders:"outer",held:8000,possible:4288,seconds:3,needed:6553};
 expect(formatHostCapacityErrorV1("en","ancestorHolds",values)).toContain("repo capacity run --memory=6553");
 expect(formatHostCapacityErrorV1("de","ancestorHolds",values)).toContain("wartet selbst auf diesen Befehl");
 expect(formatHostCapacityErrorV1("en","nestedExceedsGrant",{operation:"graph-plan",minimum:6553,holders:"outer",granted:4500,free:4500,needed:6553})).toContain("never queues at the machine");
 expect(formatHostCapacityErrorV1("de","nestedExceedsGrant",{operation:"graph-plan",minimum:6553,holders:"outer",granted:4500,free:4500,needed:6553})).toContain("stellt sich nie am Rechner dahinter an");
});
