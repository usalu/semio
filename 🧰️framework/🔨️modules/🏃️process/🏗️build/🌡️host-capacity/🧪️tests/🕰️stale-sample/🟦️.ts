import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {agedHostSampleV1,evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostSampleV1} from "../../🟦️.ts";
import {currentSample} from "../../🎫️gate/📖️status/🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1(decisionFixture.policy),host=fixture.host as HostSampleV1,control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>Promise.resolve()};

test("A sample older than the stale threshold reads as raised pressure with partial counters, a fresher one is unchanged",()=>{
 for(const item of fixture.ages){
  const base={...host,pressure:(item as {hostPressure?:string}).hostPressure??"normal"} as HostSampleV1,aged=agedHostSampleV1(base,base.atMs+item.ageMs);
  expect(aged.pressure,item.name).toBe(item.pressure as never);
  expect(aged.counters==="partial",item.name).toBe(item.partial);
 }
});

test("A failing live read keeps the last good sample and its age, a stale one is aged, a working read replaces it",async()=>{
 for(const item of fixture.fallback){
  const directory=mkdtempSync(join(tmpdir(),"hc-stale-"));
  try{
   writeFileSync(join(directory,"sample.json"),JSON.stringify({host,swappedOutMegabytes:0,declared:""}));
   const now=host.atMs+item.ageMs,newer={...host,atMs:now},sample=async()=>{if(item.fails)throw Error("starved");return{host:newer,swappedOutMegabytes:0};};
   const result=await currentSample(directory,policy,control,{now:()=>now,sample},{});
   if(item.expect==="last-good")expect(result,item.name).toEqual(host);
   else if(item.expect==="aged")expect(result,item.name).toEqual(agedHostSampleV1(host,now));
   else expect(result,item.name).toEqual(newer);
  }finally{rmSync(directory,{recursive:true,force:true});}
 }
});

test("Without any earlier sample a failing read still fails, never a made-up host",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hc-stale-"));
 try{await expect(currentSample(directory,policy,control,{now:()=>host.atMs,sample:async()=>{throw Error("starved");}},{})).rejects.toThrow("starved");}
 finally{rmSync(directory,{recursive:true,force:true});}
});

test("Stale data admits one run at a time with its minimum jobs while a fresh sample admits both",()=>{
 const at=host.atMs+policy.load.overrideAfterMilliseconds+1000,waiters=[{request:fixture.request,arrivedAtMs:host.atMs,pid:1},{request:fixture.second,arrivedAtMs:host.atMs+1,pid:2}] as never;
 const fresh=evaluateHostCapacityV1(policy,{...host,atMs:at},[],waiters,at),stale=evaluateHostCapacityV1(policy,agedHostSampleV1(host,at),[],waiters,at);
 expect(fresh.decisions.filter(row=>row.verdict==="admit").length).toBe(fixture.admission.freshAdmitted);
 const admitted=stale.decisions.filter(row=>row.verdict==="admit");
 expect(admitted.length).toBe(fixture.admission.staleAdmitted);
 expect(admitted[0]!.reason).toBe(fixture.admission.staleReason as never);
 expect(admitted[0]!.jobs).toBe(fixture.admission.staleJobs);
});
