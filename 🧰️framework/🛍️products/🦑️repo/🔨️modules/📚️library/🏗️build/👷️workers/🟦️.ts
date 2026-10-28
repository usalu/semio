import {availableParallelism,totalmem} from "node:os";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {hostProfileV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎯️ What a task does, derived from its target name and continuity, never from a build tool. */
export type WorkerActionKindV1="test"|"check"|"generate"|"build"|"script"|"service";

/** 🔏️ How one action kind uses persistent workers. */
export type WorkerKindPolicyV1=Readonly<{workers:boolean;maxInstances:number;multiplex:boolean;inProcess:boolean}>;

/** 🔧️ The authored tool-neutral worker pool policy, stored with the neutral build policy. */
export type WorkerPolicyV1=Readonly<{version:1;enabled:boolean;protocol:"json";localJobs:Readonly<{maximum:number}>;memory:Readonly<{hostShareBasisPoints:number;perWorkerMegabytes:number}>;sandboxing:boolean;cancellation:boolean;resetAfterRequests:number;classification:readonly Readonly<{kind:WorkerActionKindV1;continuous?:boolean;pattern?:string}>[];defaultKind:Exclude<WorkerActionKindV1,"service">;kinds:Readonly<Record<WorkerActionKindV1,WorkerKindPolicyV1>>}>;

/** 🗓️ A policy resolved against one host and one invocation. */
export type WorkerPlanV1=Readonly<{version:1;enabled:boolean;protocol:"json";localJobs:number;memory:Readonly<{totalMegabytes:number;perWorkerMegabytes:number;instances:number}>;sandboxing:boolean;cancellation:boolean;resetAfterRequests:number;kinds:Readonly<Record<WorkerActionKindV1,WorkerKindPolicyV1>>}>;

/** ☁️ The host facts a plan depends on. */
export type WorkerHostV1=Readonly<{cores:number;memoryBytes:number}>;

/** 🧵️ What the caller may override: the local job count and whether workers run at all. */
export type WorkerOverridesV1=Readonly<{jobs?:number;workers?:boolean}>;

/** 👷️ How exactly a build tool honours one contract field. */
export type WorkerSupportV1="exact"|"approximate"|"unsupported";

/** 🗺️ What one build tool does with one contract field. */
export type WorkerMappingV1=Readonly<{field:string;support:WorkerSupportV1;setting:string}>;

export const WORKER_ACTION_KINDS_V1=Object.freeze(["test","check","generate","build","script","service"] as const);

const MEGABYTE=1048576,clamp=(value:number,low:number,high:number)=>Math.min(high,Math.max(low,value));

/** 📖️ Reads the authored worker policy and refuses anything the closed schema or the pattern grammar rejects. */
export function readWorkerPolicyV1(value:unknown):WorkerPolicyV1{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/Policy"},value);
 if(errors.length)throw Error("Closed worker policy required: "+JSON.stringify(errors[0]));
 const policy=value as WorkerPolicyV1;
 for(const rule of policy.classification)if(rule.pattern!==undefined)new RegExp(rule.pattern,"u");
 for(const kind of WORKER_ACTION_KINDS_V1)if(policy.kinds[kind].multiplex&&policy.kinds[kind].inProcess)throw Error(`Multiplexed workers can not run actions in their own process: ${kind}`);
 return policy;
}

/** 🏷️ Classifies a task by its continuity and target name; the first matching rule wins, otherwise the default kind. */
export function workerActionKindV1(policy:WorkerPolicyV1,target:string,continuous:boolean):WorkerActionKindV1{
 for(const rule of policy.classification)if(rule.continuous!==undefined?rule.continuous===continuous:new RegExp(rule.pattern!,"u").test(target))return rule.kind;
 return policy.defaultKind;
}

/** 🖥️ Reads the facts of the running host. */
export function currentWorkerHostV1(environment:Readonly<Record<string,string|undefined>>=process.env):WorkerHostV1{const host=hostProfileV1(environment,{totalMegabytes:Math.floor(totalmem()/1048576),cores:availableParallelism(),declared:false});return Object.freeze({cores:host.cores,memoryBytes:host.totalMegabytes*1048576});}

/** 🧮️ Resolves the policy for one host: the local job count, the memory the pools may hold in total and how many instances each kind keeps. Every kind that uses workers first gets one instance, in the priority order test, check, generate, build, script, as far as the memory holds instances; the remaining instances then go to the kinds in the same order, up to their bound and the job count. A kind that finds no instance left runs without workers. */
export function planWorkersV1(policy:WorkerPolicyV1,host:WorkerHostV1,overrides:WorkerOverridesV1={}):WorkerPlanV1{
 const enabled=overrides.workers??policy.enabled,jobs=overrides.jobs??clamp(host.cores,1,policy.localJobs.maximum);
 if(!Number.isSafeInteger(jobs)||jobs<1)throw Error("Worker job count must be a positive integer");
 const totalMegabytes=Math.max(64,Math.floor(host.memoryBytes/MEGABYTE*policy.memory.hostShareBasisPoints/10000)),instances=clamp(Math.floor(totalMegabytes/policy.memory.perWorkerMegabytes),1,1024);
 const wanted=WORKER_ACTION_KINDS_V1.filter(kind=>enabled&&policy.kinds[kind].workers),counts=new Map<WorkerActionKindV1,number>(wanted.slice(0,instances).map(kind=>[kind,1] as const));
 let remaining=instances-counts.size;
 for(const kind of counts.keys()){const add=Math.min(Math.max(0,Math.min(policy.kinds[kind].maxInstances,jobs)-1),remaining);counts.set(kind,1+add);remaining-=add;}
 const kinds={} as Record<WorkerActionKindV1,WorkerKindPolicyV1>;
 for(const kind of WORKER_ACTION_KINDS_V1){
  const row=policy.kinds[kind],count=counts.get(kind);
  kinds[kind]=Object.freeze(count===undefined?{workers:false,maxInstances:1,multiplex:false,inProcess:false}:{workers:true,maxInstances:count,multiplex:row.multiplex,inProcess:row.inProcess});
 }
 return Object.freeze({version:1,enabled,protocol:policy.protocol,localJobs:jobs,memory:Object.freeze({totalMegabytes,perWorkerMegabytes:policy.memory.perWorkerMegabytes,instances}),sandboxing:policy.sandboxing,cancellation:policy.cancellation,resetAfterRequests:policy.resetAfterRequests,kinds:Object.freeze(kinds)});
}

/** ✅️ Validates a resolved plan against the closed schema. */
export function requireWorkerPlanV1(value:unknown):asserts value is WorkerPlanV1{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/Plan"},value);
 if(errors.length)throw Error("Closed worker plan required: "+JSON.stringify(errors[0]));
}

/** 🎛️ Reads the caller's worker switch: `on`, `off` or absent. */
export function readWorkerSwitchV1(value:string|undefined):boolean|undefined{
 if(value===undefined||value==="")return undefined;
 if(value==="on")return true;
 if(value==="off")return false;
 throw Error(`Worker switch must be on or off: ${value}`);
}

/** 🧾️ Lists every contract field a build tool has to map exactly once, so a mapping report can be checked for completeness. */
export function workerContractFieldsV1():readonly string[]{
 return Object.freeze(["protocol","localJobs","memory.totalMegabytes","memory.perWorkerMegabytes","sandboxing","cancellation","resetAfterRequests",...WORKER_ACTION_KINDS_V1.flatMap(kind=>["workers","maxInstances","multiplex","inProcess"].map(field=>`kinds.${kind}.${field}`))]);
}
