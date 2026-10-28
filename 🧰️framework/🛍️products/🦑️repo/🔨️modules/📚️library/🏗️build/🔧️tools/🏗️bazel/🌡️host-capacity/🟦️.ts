import {readHostCapacityPlanV1,readLiveHostCapacityPlanV1,type HostActionKindV1,type HostCapacityPolicyV1,type HostPlanV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {WORKER_ACTION_KINDS_V1,type WorkerHostV1,type WorkerMappingV1} from "../../../👷️workers/🟦️.ts";
import {bazelWorkerMnemonicV1} from "../👷️workers/🟦️.ts";

/** 🧰️ What the host capacity plan becomes in Bazel: command flags and a report of how exactly each contract field is honoured. */
export type BazelHostResourceSettingsV1=Readonly<{flags:readonly string[];mapping:readonly WorkerMappingV1[]}>;

/** 🗺️ Renders the plan into Bazel's own local resource model: `--local_resources=cpu=` and `memory=` bound what the local actions may use together, `--jobs` (from the worker plan, which takes its job count from the same grant) bounds how many start; the fixed block of the invocation and the floor stay outside Bazel. */
export function bazelHostResourceSettingsV1(plan:HostPlanV1):BazelHostResourceSettingsV1{
 const cpu=`--local_resources=cpu=${plan.cpu}`,memory=`--local_resources=memory=${plan.memoryMegabytes}`,mapping:WorkerMappingV1[]=[
  {field:"jobs",support:"exact",setting:`--jobs=${plan.jobs} (worker plan)`},
  {field:"cpu",support:"exact",setting:cpu},
  {field:"memoryMegabytes",support:"exact",setting:memory},
  {field:"fixedMegabytes",support:"unsupported",setting:"held by the invocation outside Bazel"},
  {field:"floorMegabytes",support:"unsupported",setting:"enforced by admission before Bazel starts"},
 ];
 return Object.freeze({flags:Object.freeze([cpu,memory]),mapping:Object.freeze(mapping)});
}

/** 🖥️ The host the worker pools see when a grant is in force: its cores and memory, never more than the grant. */
export function bazelGrantedWorkerHostV1(plan:HostPlanV1|undefined,host:WorkerHostV1):WorkerHostV1{
 return plan===undefined?host:Object.freeze({cores:Math.min(host.cores,plan.cpu),memoryBytes:Math.min(host.memoryBytes,(plan.memoryMegabytes+plan.fixedMegabytes)*1048576)});
}

/** 🔢️ The jobs the run's grant holds now (the carried plan followed to the grant row: after a partial or reduced growth fewer than the tool was started with, after a later growth more), undefined when the process runs under no grant. */
export function bazelLiveHostJobsV1(environment:Readonly<Record<string,string|undefined>>):number|undefined{return readLiveHostCapacityPlanV1(environment)?.jobs;}

/** 🚦️ The width of a build's real actions: a born action (its work really runs, and its birth already grew a quiet grant) starts only while fewer actions run than the jobs the grant holds now (`jobs`, read live; undefined means no grant and no bound; an idle build always starts one), so a run whose growth was partial never runs wider than its memory carries although its tool was started with the full jobs. Waiters start in arrival order when an action leaves or when `recheck` (every `pollMilliseconds` while someone waits) finds more jobs; a waiter whose key leaves (its connection closed) or whose signal aborts is refused. */
export type BazelActionWidthV1<K>=Readonly<{enter:(key:K,signal?:AbortSignal)=>Promise<void>;leave:(key:K)=>void;recheck:()=>void;running:()=>number;waiting:()=>number}>;
export function createBazelActionWidthV1<K>(jobs:()=>number|undefined,pollMilliseconds:number):BazelActionWidthV1<K>{
 const running=new Set<K>(),queue:{key:K;accept:()=>void;refuse:(error:unknown)=>void}[]=[];let timer:ReturnType<typeof setInterval>|undefined;
 const recheck=()=>{
  while(queue.length){const limit=jobs();if(limit!==undefined&&running.size>0&&running.size>=limit)break;const next=queue.shift()!;running.add(next.key);next.accept();}
  if(queue.length&&timer===undefined){timer=setInterval(recheck,pollMilliseconds);timer.unref?.();}else if(!queue.length&&timer!==undefined){clearInterval(timer);timer=undefined;}
 };
 return Object.freeze({
  enter:(key:K,signal?:AbortSignal)=>new Promise<void>((accept,refuse)=>{
   if(signal?.aborted){refuse(signal.reason);return;}
   const abort=()=>{const at=queue.indexOf(row);if(at>=0){queue.splice(at,1);recheck();refuse(signal!.reason);}},row={key,accept:()=>{signal?.removeEventListener("abort",abort);accept();},refuse:(error:unknown)=>{signal?.removeEventListener("abort",abort);refuse(error);}};
   signal?.addEventListener("abort",abort,{once:true});queue.push(row);recheck();
  }),
  leave:(key:K)=>{if(running.delete(key)){recheck();return;}const at=queue.findIndex(row=>row.key===key);if(at>=0){const[row]=queue.splice(at,1);recheck();row!.refuse(Error("bazel-action-left-before-start"));}},
  recheck,running:()=>running.size,waiting:()=>queue.length,
 });
}

/** 📥️ Reads the plan of the enclosing grant from the environment. */
export function bazelHostPlanV1(environment:Readonly<Record<string,string|undefined>>):HostPlanV1|undefined{return readHostCapacityPlanV1(environment);}

/** 📜️ Renders the per-kind estimates of the policy into Starlark `resource_set` callbacks keyed by mnemonic, so Bazel's local resource manager books every action at its estimate instead of its default of one core and 250 MB. */
export function bazelResourceSetsV1(kinds:HostCapacityPolicyV1["kinds"]):string{
 const lines:string[]=[];
 for(const kind of WORKER_ACTION_KINDS_V1.filter((row):row is Exclude<typeof row,"service">=>row!=="service"))lines.push(`def _resources_${kind}(os, inputs):`,`    return {"cpu": ${kinds[kind as HostActionKindV1].cpu}, "memory": ${kinds[kind as HostActionKindV1].megabytes}}`,"");
 lines.push(`_RESOURCE_SETS = {${WORKER_ACTION_KINDS_V1.filter(kind=>kind!=="service").map(kind=>`"${bazelWorkerMnemonicV1(kind)}": _resources_${kind}`).join(", ")}}`,"","");
 return lines.join("\n");
}

/** 🧮️ Bazel's own local resource manager as a function: an action starts while the sum of started actions fits both pools, and one action always starts on an idle pool even when it is larger than the pool. */
export function bazelLocalConcurrencyV1(pool:Readonly<{jobs:number;cpu:number;memoryMegabytes:number}>,action:Readonly<{cpu:number;memoryMegabytes:number}>,count:number):number{
 let running=0;
 while(running<count&&running<pool.jobs&&(running===0||(running+1)*action.cpu<=pool.cpu&&(running+1)*action.memoryMegabytes<=pool.memoryMegabytes))running++;
 return running;
}
