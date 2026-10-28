import{observeTreeMegabytesV1}from"../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import{estimateHostRatioV1,hostEstimateLineageV1,learnableHostRunV1,readHostEstimateStoreV1,recordHostOutcomeV1,updateHostEstimateStoreV1,type HostRunStatusV1}from"../../../🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import type{HostCapacityPolicyV1}from"../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import type{TestRunnerIdV1,TestWorkerPlanV1}from"../🟦️.ts";

const protocol=schema.definitions.Protocol.const;

/** 📚️ Where a runner learns its size: the estimate policy of the host capacity gate, the directory of the gate (its `estimates.json` is the one store of the machine) and a clock. */
export type TestWorkerLearningV1=Readonly<{policy:HostCapacityPolicyV1;directory:string;now?:()=>number}>;

/** 🪜️ Names the classes of a runner from the general to the specific: the runner (a class of its own, so what one runner learns never shaves another), then what the caller names (a suite, a project). */
export function testWorkerLineageV1(runner:TestRunnerIdV1,specific?:string):readonly string[]{
 return hostEstimateLineageV1(`${protocol.estimateOperation}-${runner}`,...(specific===undefined||specific===""?[]:[specific.replace(/\s+/gu,"_").slice(0,120)]));
}

/** 🎓️ The learned share of the declared size of a class in basis points; 10000 while nothing was learned. */
export function learnedTestWorkerRatioV1(learning:TestWorkerLearningV1|undefined,runner:TestRunnerIdV1,specific?:string):number{
 if(learning===undefined)return 10000;
 return estimateHostRatioV1(learning.policy,readHostEstimateStoreV1(learning.directory),testWorkerLineageV1(runner,specific),(learning.now??Date.now)()).ratioBasisPoints;
}

/** 👁️ Follows the peak the process tree under `pid` holds while a runner works; `stop` ends the sampling and answers the peak in megabytes with the time after which it first came within 5 % of it (0 and 0 on a platform that cannot observe a tree). */
export function observeTestWorkerPeakV1(pid:number=process.pid,intervalMilliseconds:number=protocol.observeMilliseconds,observe:(pid:number)=>Promise<number|null>=observeTreeMegabytesV1,now:()=>number=Date.now):Readonly<{stop:()=>Promise<Readonly<{peakMegabytes:number;peakAtMilliseconds:number}>>}>{
 const started=now(),marks:[number,number][]=[];let peak=0,pending=Promise.resolve();
 const sample=()=>{pending=pending.then(async()=>{const megabytes=await observe(pid).catch(()=>null);if(megabytes!==null){peak=Math.max(peak,megabytes);if(marks.length<512)marks.push([now()-started,megabytes]);}});};
 sample();
 const timer=setInterval(sample,intervalMilliseconds);timer.unref();
 return Object.freeze({stop:async()=>{clearInterval(timer);sample();await pending;return Object.freeze({peakMegabytes:Math.round(peak),peakAtMilliseconds:marks.find(([,megabytes])=>megabytes>=peak*0.95)?.[0]??0});}});
}

/** ✍️ Teaches the classes of a runner what one finished run held against the size the plan declared; a failed run teaches only when it already exceeded the estimate, a platform that cannot observe teaches nothing, and a store that cannot be written never fails the run. */
export async function recordTestWorkersV1(learning:TestWorkerLearningV1,plan:TestWorkerPlanV1,specific:string|undefined,run:Readonly<{status:HostRunStatusV1;peakMegabytes:number;peakAtMilliseconds:number;activeMilliseconds:number}>):Promise<boolean>{
 if(run.peakMegabytes<=0)return false;
 const lineage=testWorkerLineageV1(plan.runner,specific),now=learning.now??Date.now,ratio=Math.max(1,Math.round(run.peakMegabytes*10000/plan.declaredMegabytes));
 if(!learnableHostRunV1(run.status,ratio,estimateHostRatioV1(learning.policy,readHostEstimateStoreV1(learning.directory),lineage,now())))return false;
 return updateHostEstimateStoreV1(learning.directory,learning.policy,store=>recordHostOutcomeV1(learning.policy,store,lineage,{operation:`${protocol.estimateOperation}-${plan.runner}`,jobs:plan.workers,peakMegabytes:run.peakMegabytes,defaultMegabytes:plan.declaredMegabytes,activeMilliseconds:run.activeMilliseconds,peakAtMilliseconds:run.peakAtMilliseconds,endedAtMs:now()}),{now}).catch(()=>false);
}

/** 🔭️ Runs a runner under observation and teaches the store what its tree held; without learning the body simply runs. */
export async function withTestWorkerLearningV1<T>(learning:TestWorkerLearningV1|undefined,plan:TestWorkerPlanV1,specific:string|undefined,body:()=>Promise<T>):Promise<T>{
 if(learning===undefined)return body();
 const observer=observeTestWorkerPeakV1(process.pid,protocol.observeMilliseconds,undefined,learning.now),started=(learning.now??Date.now)();
 let status:HostRunStatusV1="ok";
 try{return await body();}catch(error){status="failed";throw error;}
 finally{const peak=await observer.stop();await recordTestWorkersV1(learning,plan,specific,{status,peakMegabytes:peak.peakMegabytes,peakAtMilliseconds:peak.peakAtMilliseconds,activeMilliseconds:Math.max(peak.peakAtMilliseconds,(learning.now??Date.now)()-started)});}
}
