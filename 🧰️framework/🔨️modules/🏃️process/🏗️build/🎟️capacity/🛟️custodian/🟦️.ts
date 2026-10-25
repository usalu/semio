
import {fileURLToPath} from "node:url";
import {checkResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";
import {captureOwnedCommandResultV1,readOwnedCommandCaptureV1,type OwnedCommandCaptureV1} from "../../../🎛️owned-execution/📥️capture/🟦️.ts";
import {requireIssuedScriptInvocationV1,receiveScriptProcessInvocation,scriptInvocationEnvironment,withIssuedResourceControlV1,withIssuedResourceOperationV1,createRequestedIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ScriptInvocation} from "../../../🧭️routing/📥️invocation/🟦️.ts";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {acquireCompilerCapacityLeaseV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { startToolV1 } from "../../../📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runtimeCommandNameV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 📋️ Declares a finite paid process plan before its custodian acquires actual host capacity. */
export type PaidProcessCustodianPlanV1=Readonly<{version:1;coverage:"direct-child";directory:string;parallelism:number;jobs:readonly Readonly<{program:string;args:readonly string[];cwd:string;parallelism:number;output:Readonly<{maximumBytes:number;maximumLines:number}>}>[]}>;
/** 📬️ Requests declared jobs and receives results; this port never spawns local compute. */
export interface PaidProcessCustodianPortV1{run(index:number):Promise<OwnedCommandCaptureV1>}

/** 🔎️ Admits the closed finite declaration without treating its contents as paid authority. */
export function readPaidProcessCustodianPlanV1(value:unknown):PaidProcessCustodianPlanV1{
 if(Buffer.byteLength(JSON.stringify(value),"utf8")>schema.definitions.Protocol.const.maximumPlanBytes)throw Error("Paid process plan UTF8 transport exhausted");
 if(validateJsonSchemaSubset(schema.definitions.Plan,value,schema).length)throw Error("Closed finite paid process plan required");
 const plan=value as PaidProcessCustodianPlanV1;if(plan.jobs.some(job=>job.program.length>256||job.cwd.length>256||job.parallelism>plan.parallelism||[job.program,job.cwd,...job.args].some(text=>text.includes("\0"))))throw Error("Paid process job exceeds its declared reservation");
 return Object.freeze({...plan,jobs:Object.freeze(plan.jobs.map(job=>Object.freeze({...job,args:Object.freeze([...job.args]),output:Object.freeze({...job.output})})))});
}

/** 🏦️ Owns every real reservation and direct child in a dedicated process through accepted work settlement. */
export async function servePaidProcessCustodianV1():Promise<void>{
 if(typeof process.send!=="function"||!process.connected)throw Error("Actual inherited process channel required");
 const controller=new AbortController(),pending=new Set<Promise<void>>(),used=new Set<number>();let accepting=true,running=0,finish!:()=>void,planned=false;
 const stopped=new Promise<void>(resolve=>{finish=resolve;}),send=(value:object)=>{if(process.connected)try{process.send!(value,()=>{});}catch{}},stop=(cancel:boolean)=>{accepting=false;if(cancel)controller.abort(Error("Paid process owner stopped"));finish();};
 process.once("disconnect",()=>stop(true));
 await receiveScriptProcessInvocation(process.env,original=>withIssuedResourceControlV1(original,"process:paid-custodian",0,async control=>{
  let capacity:Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>|undefined;
  try{
   const plan=await new Promise<PaidProcessCustodianPlanV1>((resolve,reject)=>{
    const interrupt=()=>reject(controller.signal.reason);controller.signal.addEventListener("abort",interrupt,{once:true});
    process.once("message",value=>{controller.signal.removeEventListener("abort",interrupt);try{planned=true;resolve(readPaidProcessCustodianPlanV1(value));}catch(error){reject(error);}});if(controller.signal.aborted)interrupt();
   });
   const admission=createRequestedIssuedResourceControlV1(control,0,controller.signal);
   process.on("message",value=>{
    if(!validateJsonSchemaSubset(schema.definitions.Stop,value,schema).length){stop((value as{kind:string}).kind==="cancel");return;}
    if(validateJsonSchemaSubset(schema.definitions.Request,value,schema).length){stop(true);return;}
    const index=(value as{index:number}).index,job=plan.jobs[index];
    if(!accepting||!capacity||!job||used.has(index)||running+job.parallelism>capacity.parallelism){send({version:1,kind:"refused",index});return;}
    try{checkResourceLeaseControl(admission);}catch{stop(true);send({version:1,kind:"refused",index});return;}
    used.add(index);running+=job.parallelism;
    const work=withIssuedResourceOperationV1(admission,async()=>{
     try{const result=await captureOwnedCommandResultV1(job.program,job.args,job.cwd,{control:admission,output:job.output,env:scriptInvocationEnvironment(original,{...process.env,SEMIO_COMPILER_CAPACITY_HELD:undefined,SEMIO_COMPILER_PARALLELISM:undefined})});send({version:1,kind:"result",index,result});}
     catch(error){send({version:1,kind:"refused",index});throw error;}
     finally{running-=job.parallelism;}
    });pending.add(work);void work.finally(()=>pending.delete(work)).catch(()=>{});
   });
   capacity=await acquireCompilerCapacityLeaseV1({directory:plan.directory,control:admission,environment:{},request:{kind:"exact",parallelism:plan.parallelism}});
   const abort=()=>stop(true);control.signal.addEventListener("abort",abort,{once:true});if(control.signal.aborted)abort();
   send({version:1,kind:"ready",parallelism:capacity.parallelism});
   try{await stopped;}finally{control.signal.removeEventListener("abort",abort);}
  }finally{
   accepting=false;while(pending.size)await Promise.allSettled([...pending]);capacity?.release();send({version:1,kind:"settled"});if(process.connected)process.disconnect();
  }
 }));
 if(!planned)throw Error("Paid process declaration absent");
}

/** 🪢️ Retains the custodian on caller cancellation and joins it after accepted direct children settle. */
export function withPaidProcessCustodianV1<T>(original:ScriptInvocation,declaration:PaidProcessCustodianPlanV1,run:(port:PaidProcessCustodianPortV1)=>Promise<T>):Promise<T>{
 requireIssuedScriptInvocationV1(original);const plan=readPaidProcessCustodianPlanV1(declaration);
 return withIssuedResourceControlV1(original,"process:paid-custodian-client",0,control=>withIssuedResourceOperationV1(control,async()=>{
  const child=startToolV1(runtimeCommandNameV1(),[fileURLToPath(new URL("./📜️script.ts",import.meta.url))],{cwd:process.cwd(),env:scriptInvocationEnvironment(original,process.env),stdio:["ignore","pipe","pipe","ipc"],detached:process.platform!=="win32",windowsHide:true}),requests=new Map<number,{resolve:(value:OwnedCommandCaptureV1)=>void;reject:(error:unknown)=>void}>();let accepting=true,readyResolve!:()=>void,readyReject!:(error:unknown)=>void,diagnostics="",settled=false,closed=false;
  const ready=new Promise<void>((resolve,reject)=>{readyResolve=resolve;readyReject=reject;}),send=(value:object)=>{if(child.connected)try{child.send(value,()=>{});}catch{}},stop=(cancel:boolean)=>{accepting=false;send({version:1,kind:cancel?"cancel":"drain"});},abort=()=>stop(true);
  for(const stream of[child.stdout,child.stderr])stream!.on("data",chunk=>{diagnostics=(diagnostics+String(chunk)).slice(-65536);});
  const close=new Promise<void>(resolve=>{child.once("error",error=>readyReject(error));child.once("close",()=>{closed=true;readyReject(Error("Paid process custodian closed: "+diagnostics));for(const request of requests.values())request.reject(Error("Paid process custodian closed"));requests.clear();resolve();});});
  child.on("message",value=>{
   if(validateJsonSchemaSubset(schema.definitions.Response,value,schema).length){stop(true);readyReject(Error("Closed paid process response required"));return;}const message=value as{version?:unknown;kind?:unknown;parallelism?:unknown;index?:unknown;result?:unknown};if(message.version!==1)return;
   if(message.kind==="ready"){if(message.parallelism===plan.parallelism)readyResolve();else readyReject(Error("Paid process capacity mismatch"));return;}
   if(message.kind==="settled"){settled=true;return;}const request=requests.get(message.index as number);if(!request)return;requests.delete(message.index as number);
   if(message.kind==="result")try{request.resolve(readOwnedCommandCaptureV1(message.result));}catch(error){request.reject(error);}else request.reject(Error("Paid process job refused"));
  });
  control.signal.addEventListener("abort",abort,{once:true});send(plan);if(control.signal.aborted)abort();
  let value:T|undefined,failed=false,failure:unknown,cleanupFailed=false,cleanupFailure:unknown;
  try{
   await ready;checkResourceLeaseControl(control);
   const port=Object.freeze({run:(index:number)=>withIssuedResourceOperationV1(control,async()=>{checkResourceLeaseControl(control);if(!accepting||!Number.isSafeInteger(index)||!plan.jobs[index]||requests.has(index))throw Error("Declared paid process request required");return await new Promise<OwnedCommandCaptureV1>((resolve,reject)=>{requests.set(index,{resolve,reject});send({version:1,kind:"run",index});});})});
   value=await run(port);
  }catch(error){failed=true;failure=error;}
  try{stop(control.signal.aborted);await close;if(!closed||!settled)throw Error("Paid process custodian settlement absent");}catch(error){cleanupFailed=true;cleanupFailure=error;}finally{control.signal.removeEventListener("abort",abort);}
  if(failed)throw failure;if(cleanupFailed)throw cleanupFailure;return value as T;
 }));
}
