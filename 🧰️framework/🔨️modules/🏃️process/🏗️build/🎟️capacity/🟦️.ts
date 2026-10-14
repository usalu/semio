import { randomBytes } from "node:crypto";
import { availableParallelism } from "node:os";
import { setTimeout as delay } from "node:timers/promises";
import { acquireResourceLease, acquireQueuedResourceLease, checkResourceLeaseControl, resourceLeasePathV1, resourceQueueTicketV1, type ResourceLeaseControl, type ResourceLease, type LeaseWait } from "../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, type ScriptInvocation } from "../../🧭️routing/📥️invocation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🎛️ Requires the original compiler caller's cancellation, deadline, progress and continuation ports. */
export type CompilerAdmissionControl = ResourceLeaseControl;

/** 🔄️ Binds admission to the same incoming invocation and one nonrenewable requested child budget. */
export function scriptCompilerAdmissionControlV1(invocation: ScriptInvocation, command: string, requestedMilliseconds: number, additionalSignal?: AbortSignal): CompilerAdmissionControl {
 checkScriptInvocation(invocation);
 if (!Number.isSafeInteger(requestedMilliseconds) || requestedMilliseconds < 0) throw Error("Compiler requested deadline refused");
 const started = performance.now(), check = () => { checkScriptInvocation(invocation); additionalSignal?.throwIfAborted(); };
 return {
  signal: additionalSignal ? AbortSignal.any([invocation.control.signal, additionalSignal]) : invocation.control.signal,
  remainingMilliseconds: () => { check(); const original = invocation.control.remainingMilliseconds(), requested = requestedMilliseconds === 0 ? null : requestedMilliseconds - (performance.now() - started); return original === null ? requested : requested === null ? original : Math.min(original, requested); },
  publish: async () => { check(); await invocation.control.publish({ owner: invocation.policy.owner, command, stage: "running" }); check(); },
  yieldContinuation: async () => { check(); await invocation.control.yieldContinuation(); check(); },
 };
}

/** 🧷️ Intersects one existing caller with a single authored phase without replacing any cancellation authority. */
export function requestedCompilerAdmissionControlV1(original: CompilerAdmissionControl, requestedMilliseconds: number): CompilerAdmissionControl {
 checkResourceLeaseControl(original);
 if (!Number.isSafeInteger(requestedMilliseconds) || requestedMilliseconds < 0) throw Error("Compiler requested deadline refused");
 const started = performance.now();
 return {
  signal: original.signal,
  remainingMilliseconds: () => { const remaining = checkResourceLeaseControl(original), requested = requestedMilliseconds === 0 ? null : requestedMilliseconds - (performance.now() - started); return remaining === null ? requested : requested === null ? remaining : Math.min(remaining, requested); },
  publish: progress => original.publish(progress),
  yieldContinuation: () => original.yieldContinuation(),
 };
}

/** 🏗️ Exposes one explicitly authored native phase's existing signal and nonrenewable budget. */
export function phaseCompilerAdmissionControlV1(signal: AbortSignal, maximumElapsedMilliseconds: number, publish: (progress: LeaseWait) => void | Promise<void>, yieldContinuation: () => Promise<void>): CompilerAdmissionControl {
 if (!Number.isSafeInteger(maximumElapsedMilliseconds) || maximumElapsedMilliseconds < 0) throw Error("Authored compiler phase deadline refused");
 const started = performance.now();
 return { signal, remainingMilliseconds: () => maximumElapsedMilliseconds === 0 ? null : maximumElapsedMilliseconds - (performance.now() - started), publish, yieldContinuation };
}

/** ⏱️ Converts remaining authority into a child grant without converting a finite fraction to ordinary zero. */
export function compilerAdmissionBudgetV1(control: CompilerAdmissionControl): number {
 const remaining = checkResourceLeaseControl(control);
 if (remaining === null) return 0;
 const budget = Math.floor(remaining);
 if (budget < 1) throw Error("Original compiler admission deadline exhausted");
 return budget;
}

/** 🎟️ One compiler's admitted host capacity or allocation borrowed from an enclosing jobserver. */
export type CompilerCapacityLeaseV1 = ResourceLease & Readonly<{parallelism:number}>;

/** 🧵️ Reads an enclosing compiler's reserved worker budget or all CPUs owned by an independent compiler. */
export function compilerCapacityWorkerCountV1(environment: Readonly<Record<string,string|undefined>> = process.env): number {
 const protocol = schema.definitions.Protocol.const;
 if (environment[protocol.inheritedEnvironment] !== "true") return availableParallelism();
 const parallelism = Number(environment[protocol.parallelismEnvironment]);
 if (!Number.isSafeInteger(parallelism) || parallelism < 1 || parallelism > availableParallelism()) throw Error("Inherited compiler capacity requires an admitted worker count");
 return parallelism;
}

/** 🧮️ Owns all host CPUs, or a requested count of bounded cores picked first come first served behind a fair shared admission gate. */
export async function acquireCompilerCapacityLeaseV1(options: { readonly directory: string; readonly control: CompilerAdmissionControl; readonly environment?: Readonly<Record<string,string|undefined>>; readonly requestedParallelism?:number; }): Promise<CompilerCapacityLeaseV1> {
 checkResourceLeaseControl(options.control);
 const signal = options.control.signal, owner = randomBytes(16).toString("base64url");
 const request=options.requestedParallelism===undefined?{}:{requestedParallelism:options.requestedParallelism};
 if(validateJsonSchemaSubset(schema.definitions.Request,request).length||options.requestedParallelism!==undefined&&options.requestedParallelism>availableParallelism())throw Error("Invalid compiler capacity request");
 const environment=options.environment??process.env,protocol=schema.definitions.Protocol.const,resource=schema.properties.resource.const,requested=options.requestedParallelism,mode=requested===undefined?"exclusive":"shared";
 resourceLeasePathV1(options.directory,resource,resourceQueueTicketV1({arrivedAtMs:Date.now(),pid:process.pid,owner}));
 if(environment[protocol.inheritedEnvironment]==="true"){
  const parallelism=compilerCapacityWorkerCountV1(environment);checkResourceLeaseControl(options.control);return{resource,mode,parallelism:requested===undefined?parallelism:Math.min(requested,parallelism),release(){}};
 }
 const gate=await acquireQueuedResourceLease({directory:options.directory,resource,mode,sharedCohort:mode==="shared",owner,signal,control:options.control});
 const cores:ResourceLease[]=[];let released=false;const started=Date.now();let nextProgress=0;
 const releaseCores=()=>{for(const core of cores.splice(0))core.release();};
 try{
  checkResourceLeaseControl(options.control);
  if(requested===undefined)return{...gate,parallelism:availableParallelism(),release:()=>gate.release()};
  const picker=await acquireQueuedResourceLease({directory:options.directory,resource:protocol.pickerResource,mode:"exclusive",owner,signal,control:options.control});
  try{
   while(cores.length<requested){
    for(let index=0;index<availableParallelism()&&cores.length<requested;index++){
     try{cores.push(await acquireResourceLease({directory:options.directory,resource:protocol.coreResourcePrefix+index,mode:"exclusive",signal,control:options.control,timeoutMs:0}));}
     catch(error){if(!(error instanceof Error)||!error.message.startsWith("Resource lease timed out: "))throw error;}
    }
    checkResourceLeaseControl(options.control);
    if(cores.length<requested){releaseCores();const elapsedMs=Date.now()-started;if(elapsedMs>=nextProgress){await options.control.publish({resource,mode,elapsedMs});checkResourceLeaseControl(options.control);await options.control.yieldContinuation();checkResourceLeaseControl(options.control);nextProgress=elapsedMs+1000;}await delay(Math.min(protocol.pollMs,checkResourceLeaseControl(options.control)??Infinity),undefined,{signal});checkResourceLeaseControl(options.control);}
   }
  }finally{picker.release();}
  return{resource,mode,parallelism:requested,release(){if(released)return;released=true;try{releaseCores();}finally{gate.release();}}};
 }catch(error){try{releaseCores();}finally{gate.release();}throw error;}
}
