/** 🎫️ Compiler capacity lease: admission control, worker counting and the capacity lease itself; a leaf, so the scope that subdivides a lease never imports the module index that re-exports it. */
import { randomBytes } from "node:crypto";
import { availableParallelism } from "node:os";
import { waitingDelayV1 as delay } from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import { acquireResourceLease, acquireQueuedResourceLease, checkResourceLeaseControl, resourceLeasePathV1, resourceQueueTicketV1, type ResourceLeaseControl, type ResourceLease, type LeaseWait } from "../../../🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, type ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import{requireIssuedResourceControlV1,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,createRenewableIssuedResourceControlV1}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type{ProgressDeadlinePolicyV1}from"../../../⏱️budget/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

/** 🎛️ Requires the original compiler caller's cancellation, deadline, progress and continuation ports. */
export type CompilerAdmissionControl = ResourceLeaseControl;

/** 🔄️ Binds admission to the same incoming invocation and one nonrenewable requested child budget. */
export function scriptCompilerAdmissionControlV1(invocation: ScriptInvocation, command: string, requestedMilliseconds: number, additionalSignal?: AbortSignal): CompilerAdmissionControl {
 return createIssuedResourceControlV1(invocation,command,requestedMilliseconds,additionalSignal);
}

/** 🧷️ Intersects one existing caller with a single authored phase without replacing any cancellation authority. */
export function requestedCompilerAdmissionControlV1(original: CompilerAdmissionControl, requestedMilliseconds: number): CompilerAdmissionControl {
 return createRequestedIssuedResourceControlV1(original,requestedMilliseconds);
}

/** ⏳️ Intersects one existing caller with a phase whose deadline is an idle window renewed by observable work (child CPU time, child output, declared waits, finished observation steps) inside a nonrenewable maximum derived from the declared cost. */
export function renewableCompilerAdmissionControlV1(original: CompilerAdmissionControl, policy: ProgressDeadlinePolicyV1, units: number = 0): CompilerAdmissionControl {
 return createRenewableIssuedResourceControlV1(original,policy,units);
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

/** 🎟️ Records one compiler's actual admitted host capacity. */
export type CompilerCapacityLeaseV1 = ResourceLease & Readonly<{parallelism:number}>;

/** 🧵️ Reads a bounded worker hint without granting a compiler reservation. */
export function compilerCapacityWorkerCountV1(environment: Readonly<Record<string,string|undefined>> = process.env): number {
 const protocol = schema.definitions.Protocol.const;
 if (environment[protocol.parallelismEnvironment] === undefined) return availableParallelism();
 const parallelism = Number(environment[protocol.parallelismEnvironment]);
 if (!Number.isSafeInteger(parallelism) || parallelism < 1 || parallelism > availableParallelism()) throw Error("Compiler worker hint exceeds available host workers");
 return parallelism;
}

/** 🧭️ Declares exact funded slots, opportunistic bounded slots (at least `minimumParallelism`, default one, at most `maximumParallelism`), or exclusive host ownership. */
export type CompilerCapacityRequestV1=Readonly<{kind:"exact";parallelism:number}|{kind:"available";maximumParallelism:number;minimumParallelism?:number}|{kind:"whole-host"}>;
/** 🛂️ Snapshots one closed request before any asynchronous admission. */
export function readCompilerCapacityRequestV1(value:CompilerCapacityRequestV1):CompilerCapacityRequestV1{const snapshot=Object.freeze({...value});if(validateJsonSchemaSubset(schema.definitions.Request,snapshot).length||snapshot.kind==="available"&&(snapshot.minimumParallelism??1)>snapshot.maximumParallelism)throw Error("Invalid compiler capacity request");return snapshot;}
/** 🧮️ Owns exclusive host capacity or actual bounded cores behind the fair shared admission and picker. */
export async function acquireCompilerCapacityLeaseV1(options: {readonly directory:string;readonly control:CompilerAdmissionControl;readonly request:CompilerCapacityRequestV1;readonly environment?:Readonly<Record<string,string|undefined>>}):Promise<CompilerCapacityLeaseV1>{
 const control=options.control;requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);
 const request=readCompilerCapacityRequestV1(options.request),environment=Object.freeze({...options.environment??process.env}),protocol=schema.definitions.Protocol.const;if(environment[protocol.inheritedEnvironment]==="true")throw Error("Actual compiler capacity owner required; environment markers cannot lend CPUs");
 const host=availableParallelism(),maximum=request.kind==="whole-host"?host:request.kind==="exact"?request.parallelism:Math.min(host,request.maximumParallelism);if(maximum>host||request.kind==="available"&&(request.minimumParallelism??1)>host)throw Error("Invalid compiler capacity request");const minimum=request.kind==="available"?request.minimumParallelism??1:maximum;
 const signal=control.signal,owner=randomBytes(16).toString("base64url"),directory=options.directory,resource=schema.properties.resource.const,mode=request.kind==="whole-host"?"exclusive":"shared";
 resourceLeasePathV1(directory,resource,resourceQueueTicketV1({arrivedAtMs:Date.now(),pid:process.pid,owner}));
 const gate=await acquireQueuedResourceLease({directory,resource,mode,sharedCohort:mode==="shared",owner,signal,control}),cores:ResourceLease[]=[];let released=false;const started=Date.now();let nextProgress=0;
 const releaseCores=()=>{const failures:unknown[]=[];for(const core of cores.splice(0))try{core.release();}catch(error){failures.push(error);}if(failures.length)throw new AggregateError(failures,"Compiler core retirement failed");};
 const retire=(failed=false,failure?:unknown)=>{if(released)return;released=true;const failures:unknown[]=[];try{releaseCores();}catch(error){failures.push(error);}try{gate.release();}catch(error){failures.push(error);}if(failures.length)throw new AggregateError(failed?[failure,...failures]:failures,"Compiler capacity retirement failed");};
 try{
  checkResourceLeaseControl(control);if(request.kind==="whole-host")return{...gate,parallelism:host,release:()=>gate.release()};
  const picker=await acquireQueuedResourceLease({directory,resource:protocol.pickerResource,mode:"exclusive",owner,signal,control});
  let pickerFailed=false,pickerFailure:unknown;try{while(true){
   for(let index=0;index<host&&cores.length<maximum;index++){try{cores.push(await acquireResourceLease({directory,resource:protocol.coreResourcePrefix+index,mode:"exclusive",signal,control,timeoutMs:0}));}catch(error){if(!(error instanceof Error)||!error.message.startsWith("Resource lease timed out: "))throw error;}}
   checkResourceLeaseControl(control);if(cores.length===maximum||request.kind==="available"&&cores.length>=minimum)break;
   releaseCores();const elapsedMs=Date.now()-started;if(elapsedMs>=nextProgress){await control.publish({resource,mode,elapsedMs});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);nextProgress=elapsedMs+1000;}await delay(Math.min(protocol.pollMs,checkResourceLeaseControl(control)??Infinity),undefined,{signal});checkResourceLeaseControl(control);
  }}catch(error){pickerFailed=true;pickerFailure=error;throw error;}finally{try{picker.release();}catch(error){if(pickerFailed)throw new AggregateError([pickerFailure,error],"Compiler picker work and retirement failed");throw error;}}
  return{resource,mode,parallelism:cores.length,release:()=>retire()};
 }catch(error){retire(true,error);throw error;}
}

