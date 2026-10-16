import{AsyncLocalStorage}from"node:async_hooks";
import type{ResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import requestSchema from"./📥️issuance/🧬️schema/📥️request.json" with{type:"json"};
import cleanupSchema from"./🧬️schema/🧹️cleanup.json" with{type:"json"};
import type{AcceptedPhysicalCleanupV1,PhysicalCleanupRequestV1,AcceptedPhysicalCleanupFactsV1,AcceptedPhysicalCleanupBodyV1}from"./🧹️cleanup/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import policySchema from "../🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, readScriptPolicy, type ScriptInvocation, type ScriptPolicy } from "../🟦️.ts";
import {readScriptProcessJsonV1,readScriptProcessJsonWireV1} from "./📥️json/🟦️.ts";

/** 📨️ Transfers the original process deadline without rebasing child authority. */
export type ScriptProcessEnvelope = Readonly<{ version: 1; policy: ScriptPolicy; deadlineEpochMilliseconds: number | null; capabilities: object }>;
export const SCRIPT_PROCESS_INVOCATION_ENV = "SEMIO_SCRIPT_PROCESS_INVOCATION";

/** 🔎️ Validates the authored process envelope and its remaining original authority. */
export function readScriptProcessEnvelope(value: unknown, now: number): ScriptProcessEnvelope {
  value=readScriptProcessJsonV1(value);
  if (schema.properties.policy.$ref !== policySchema.$id) throw Error("Original Script policy schema owner required");
  const admittedSchema = { ...schema, properties: { ...schema.properties, policy: policySchema } };
  if (validateJsonSchemaSubset(admittedSchema, value).length) throw Error("Complete original Script process envelope required");
  const envelope = value as ScriptProcessEnvelope;
  if (!Number.isFinite(now)) throw Error("Original Script process clock refused");
  if (envelope.deadlineEpochMilliseconds !== null) {
    const remaining = envelope.deadlineEpochMilliseconds - now;
    if (remaining <= 0 || remaining > envelope.policy.maximumElapsedMilliseconds) throw Error("Original Script process deadline refused");
  }
  return envelope;
}

/** 🌱️ Creates the original process deadline from its explicitly supplied authored policy. */
export function createScriptProcessEnvelope(policy: ScriptPolicy, capabilities: object, now: number): ScriptProcessEnvelope {
  const original = readScriptPolicy(readScriptProcessJsonV1(policy));
  return readScriptProcessEnvelope({ version: 1, policy: original, deadlineEpochMilliseconds: original.maximumElapsedMilliseconds === 0 ? null : now + original.maximumElapsedMilliseconds, capabilities }, now);
}

declare const invocationBrand:unique symbol;
/** 🌱️ Identifies the exact live invocation admitted by the canonical process owner. */
export type IssuedScriptInvocationV1=ScriptInvocation&{readonly[invocationBrand]:true};
type Scope={live:boolean;accepting:boolean;pending:Set<Promise<unknown>>;failures:unknown[];parent?:Scope};
type Birth=Scope&{invocation:IssuedScriptInvocationV1;envelope:ScriptProcessEnvelope};
type ControlScope=Scope&{birth:Birth;invocation:IssuedScriptInvocationV1;signal:AbortSignal;started:number;requestedMilliseconds:number;command:string;deadlineEpochMilliseconds:number|null;parentControl?:ControlScope};
let received=false;
const invocations=new WeakMap<ScriptInvocation,Birth>(),controls=new WeakMap<ResourceLeaseControl,ControlScope>(),context=new AsyncLocalStorage<ScriptInvocation>();
const scope=(parent?:Scope):Scope=>({live:true,accepting:true,pending:new Set(),failures:[],...(parent?{parent}:{})});
function observe<T>(work:Promise<T>):Promise<T>{void work.then(()=>{},()=>{});return work;}
function track<T>(owner:Scope,run:()=>Promise<T>):Promise<T>{
 for(let row:Scope|undefined=owner;row;row=row.parent)if(!row.live||!row.accepting)throw Error("Original operation admission closed");
 const work=Promise.resolve().then(run);
 for(let row:Scope|undefined=owner;row;row=row.parent){const accepted=row;accepted.pending.add(work);void work.then(()=>{accepted.pending.delete(work);},error=>{if(!accepted.failures.length)accepted.failures.push(error);accepted.pending.delete(work);});}
 return work;
}
async function drain(owner:Scope):Promise<void>{owner.accepting=false;while(owner.pending.size)await Promise.allSettled([...owner.pending]);}
async function settle<T>(owner:Scope,run:()=>Promise<T>,check?:()=>void):Promise<T>{let value:T|undefined,failed=false,failure:unknown;try{value=await run();}catch(error){failed=true;failure=error;}try{await drain(owner);if(failed)throw failure;if(owner.failures.length)throw owner.failures[0];check?.();return value as T;}finally{owner.live=false;}}
function birthOf(invocation:ScriptInvocation):Birth{const birth=invocations.get(invocation);if(!birth?.live)throw Error("Live Original process issuance required");return birth;}
function admit(invocation:ScriptInvocation,parent?:ControlScope):Birth{requireIssuedScriptInvocationV1(invocation);const birth=birthOf(invocation);if(!birth.accepting)throw Error("Original process admission closed");for(let row=parent;row;row=row.parentControl)if(!row.live||!row.accepting)throw Error("Original resource admission closed");return birth;}
/** 🔐️ Refuses structural copies and retired process scopes. */
export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
/** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
 try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
 if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScriptInvocationV1));}
 const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),interrupt=()=>controller.abort(Error("Original Script process interrupted")),terminate=()=>controller.abort(Error("Original Script process terminated"));
 const timeout=initial===null?undefined:setTimeout(()=>controller.abort(Error("Original Script process deadline exhausted")),initial);timeout?.unref();process.once("SIGINT",interrupt);process.once("SIGTERM",terminate);
 const invocation:ScriptInvocation=Object.freeze({policy:admitted.policy,capabilities:admitted.capabilities,control:Object.freeze({signal:controller.signal,remainingMilliseconds:()=>deadline===null||initial===null?null:Math.max(0,Math.floor(Math.min(deadline-Date.now(),initial-(performance.now()-started)))),publish:(progress:Parameters<ScriptInvocation["control"]["publish"]>[0])=>{process.stderr.write(`[script] ${JSON.stringify(progress)}\n`);},yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))})});
 const birth:Birth={...scope(),invocation:invocation as IssuedScriptInvocationV1,envelope:admitted};invocations.set(invocation,birth);
 return observe(context.run(invocation,()=>settle(birth,async()=>{requireIssuedScriptInvocationV1(invocation);return await run(invocation);}).then(value=>{checkScriptInvocation(invocation);return value;}).finally(()=>{if(timeout)clearTimeout(timeout);process.removeListener("SIGINT",interrupt);process.removeListener("SIGTERM",terminate);}))); 
 }catch(error){return observe(Promise.reject(error));}
}
/** 📥️ Receives the real process envelope or reuses its exact live in-process parent. */
export function receiveScriptProcessInvocation<T>(environment:Readonly<Record<string,string|undefined>>,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{try{const wire=environment[SCRIPT_PROCESS_INVOCATION_ENV];if(wire===undefined)throw Error("Original parent Script process invocation required");if(!context.getStore()){if(wire!==process.env[SCRIPT_PROCESS_INVOCATION_ENV])throw Error("Actual process parent envelope required");if(received)throw Error("Original process birth already received");received=true;}return withScriptProcessEnvelope(readScriptProcessEnvelope(readScriptProcessJsonWireV1(wire),Date.now()),run);}catch(error){return observe(Promise.reject(error));}}
function remaining(original:ControlScope):number|null{
 requireIssuedScriptInvocationV1(original.invocation);let value=original.invocation.control.remainingMilliseconds();const now=performance.now();for(let row:ControlScope|undefined=original;row;row=row.parentControl){if(!row.live)throw Error("Live Original resource control issuance required");row.signal.throwIfAborted();if(row.deadlineEpochMilliseconds!==null){const absolute=row.deadlineEpochMilliseconds-Date.now();value=value===null?absolute:Math.min(value,absolute);}if(row.requestedMilliseconds!==0){const child=row.requestedMilliseconds-(now-row.started);value=value===null?child:Math.min(value,child);}}if(value!==null&&value<=0)throw Error("Original resource loan deadline exhausted");return value;
}
/** 🪪️ Authenticates the exact issued control rather than its structural ports. */
export function requireIssuedResourceControlV1(control:ResourceLeaseControl):void{const original=controls.get(control);if(!original||control.signal!==original.signal)throw Error("Live Original resource control issuance required");remaining(original);}
/** 📎️ Requires a live control's privately captured exact invocation without issuing new authority. */
export function requireIssuedResourceControlInvocationV1(control:ResourceLeaseControl,invocation:ScriptInvocation):void{requireIssuedScriptInvocationV1(invocation);requireIssuedResourceControlV1(control);if(controls.get(control)!.invocation!==invocation)throw Error("Exact Original control invocation required");}
/** 🧬️ Requires actual inherited restrictions without issuing controls or reconstructing their clocks. */
export function requireIssuedResourceControlRefinementV1(base:ResourceLeaseControl,phase:ResourceLeaseControl):void{requireIssuedResourceControlV1(base);requireIssuedResourceControlV1(phase);const owner=controls.get(base)!,borrower=controls.get(phase)!;if(owner.invocation!==borrower.invocation)throw Error("Exact Original control refinement required");const inherited=new Set<ControlScope>();for(let row:ControlScope|undefined=borrower;row;row=row.parentControl)inherited.add(row);for(let row:ControlScope|undefined=owner;row&&!inherited.has(row);row=row.parentControl)if(row.requestedMilliseconds!==0||row.signal!==row.invocation.control.signal)throw Error("Actual inherited control restriction required");}
/** 📜️ Validates a closed nonrenewable child request. */
export function readOriginalResourceControlRequestV1(value:unknown):Readonly<{version:1;command:string;maximumElapsedMilliseconds:number}>{if(validateJsonSchemaSubset(requestSchema,value).length)throw Error("Closed Original resource control request required");return Object.freeze({...value as{version:1;command:string;maximumElapsedMilliseconds:number}});}
function controlFor(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,parent?:ControlScope,additionalSignal?:AbortSignal):ResourceLeaseControl{
 readOriginalResourceControlRequestV1({version:1,command,maximumElapsedMilliseconds:requestedMilliseconds});if(additionalSignal!==undefined&&!(additionalSignal instanceof AbortSignal))throw Error("Actual additional cancellation signal required");const birth=admit(invocation,parent),incoming=parent?.signal??invocation.control.signal,child:ControlScope={...scope(parent??birth),birth,invocation:invocation as IssuedScriptInvocationV1,command,requestedMilliseconds,started:performance.now(),deadlineEpochMilliseconds:requestedMilliseconds===0?(parent?.deadlineEpochMilliseconds??birth.envelope.deadlineEpochMilliseconds):Math.min(parent?.deadlineEpochMilliseconds??birth.envelope.deadlineEpochMilliseconds??Infinity,Date.now()+requestedMilliseconds),signal:additionalSignal?AbortSignal.any([incoming,additionalSignal]):incoming,...(parent?{parentControl:parent}:{})};
 const control:ResourceLeaseControl=Object.freeze({signal:child.signal,remainingMilliseconds:()=>remaining(child),publish:async(progress:Parameters<ResourceLeaseControl["publish"]>[0])=>{remaining(child);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1")console.log("[DEBUG] Original compiler admission progress "+JSON.stringify({command,...progress}));await invocation.control.publish({owner:invocation.policy.owner,command,stage:"running"});remaining(child);},yieldContinuation:async()=>{remaining(child);await invocation.control.yieldContinuation();remaining(child);}});controls.set(control,child);return control;
}
/** 🎚️ Creates a synchronous control only beneath an actual privately live invocation. */
export function createIssuedResourceControlV1(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,additionalSignal?:AbortSignal):ResourceLeaseControl{return controlFor(invocation,command,requestedMilliseconds,undefined,additionalSignal);}
/** 🧷️ Creates a synchronous narrowed control beneath its exact privately issued parent. */
export function createRequestedIssuedResourceControlV1(control:ResourceLeaseControl,requestedMilliseconds:number,additionalSignal?:AbortSignal):ResourceLeaseControl{requireIssuedResourceControlV1(control);const original=controls.get(control)!;return controlFor(original.invocation,original.command,requestedMilliseconds,original,additionalSignal);}
/** 🎛️ Retains one issued resource loan until its callback and accepted children settle. */
export function withIssuedResourceControlV1<T>(invocation:ScriptInvocation,command:string,requestedMilliseconds:number,run:(control:ResourceLeaseControl)=>Promise<T>,additionalSignal?:AbortSignal):Promise<T>{const control=controlFor(invocation,command,requestedMilliseconds,undefined,additionalSignal),child=controls.get(control)!;return track(child.parent!,()=>settle(child,async()=>{remaining(child);const result=await run(control);remaining(child);return result;},()=>{remaining(child);}));}
/** 🧷️ Retains one narrowed resource loan without renewing parent time or cancellation. */
export function withRequestedIssuedResourceControlV1<T>(control:ResourceLeaseControl,requestedMilliseconds:number,run:(control:ResourceLeaseControl)=>Promise<T>,additionalSignal?:AbortSignal):Promise<T>{const childControl=createRequestedIssuedResourceControlV1(control,requestedMilliseconds,additionalSignal),child=controls.get(childControl)!;return track(child.parent!,()=>settle(child,async()=>{remaining(child);const result=await run(childControl);remaining(child);return result;},()=>{remaining(child);}));}
/** 🛟️ Registers accepted asynchronous custody before its first physical effect and through close. */
export function withIssuedResourceOperationV1<T>(control:ResourceLeaseControl,run:()=>Promise<T>):Promise<T>{requireIssuedResourceControlV1(control);const original=controls.get(control)!;admit(original.invocation,original);return track(original,run);}
type SignalScope={parent:ScriptInvocation;additionalSignal:AbortSignal;stopped:boolean};
const signalScopes=new WeakMap<ScriptInvocation,SignalScope>();
/** 🔗️ Retains narrowed cancellation through callback and accepted-child settlement. */
export function withScriptInvocationSignalV1<T>(invocation:ScriptInvocation,additionalSignal:AbortSignal,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
 const birth=admit(invocation);if(!(additionalSignal instanceof AbortSignal))throw Error("Actual additional cancellation signal required");const signal=AbortSignal.any([invocation.control.signal,additionalSignal]),alias:ScriptInvocation=Object.freeze({policy:invocation.policy,capabilities:invocation.capabilities,control:Object.freeze({...invocation.control,signal})}),child:Birth={...scope(birth),invocation:alias as IssuedScriptInvocationV1,envelope:birth.envelope},stopping:SignalScope={parent:invocation,additionalSignal,stopped:false};invocations.set(alias,child);signalScopes.set(alias,stopping);return track(birth,()=>context.run(alias,()=>settle(child,async()=>{checkScriptInvocation(alias);return run(alias as IssuedScriptInvocationV1);}).then(value=>{requireIssuedScriptInvocationV1(invocation);if(!stopping.stopped)checkScriptInvocation(alias);return value;})));
}
/** 🧬️ Requires private live signal ancestry without issuing or registering invocation authority. */
export function requireIssuedScriptInvocationDescendantV1(ancestor:ScriptInvocation,borrower:ScriptInvocation):void{requireIssuedScriptInvocationV1(ancestor);requireIssuedScriptInvocationV1(borrower);let current=borrower;while(current!==ancestor){const parent=signalScopes.get(current)?.parent;if(!parent)throw Error("Exact Original invocation lineage required");requireIssuedScriptInvocationV1(parent);current=parent;}}
/** 🛑️ Acknowledges handled owned cancellation without retiring or canceling its actual parent. */
export function acknowledgeScriptInvocationSignalStopV1(invocation:ScriptInvocation):void{const signal=signalScopes.get(invocation);if(!signal||!birthOf(invocation).live||!signal.additionalSignal.aborted)throw Error("Actual stopped derived Script signal scope required");requireIssuedScriptInvocationV1(signal.parent);signal.stopped=true;}

/** 📤️ Carries the same admitted deadline into an owned child process environment. */
export function scriptProcessEnvironment(envelope: ScriptProcessEnvelope, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  return { ...environment, [SCRIPT_PROCESS_INVOCATION_ENV]: JSON.stringify(readScriptProcessEnvelope(envelope, Date.now())) };
}

/** 📤️ Hands the caller's current original invocation, with its remaining deadline, to an owned child process environment. */
export function scriptInvocationEnvironment(invocation: ScriptInvocation, environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  requireIssuedScriptInvocationV1(invocation);
  return scriptProcessEnvironment(birthOf(invocation).envelope,environment);
}

type PhysicalCleanupScope={live:boolean;started:number;maximumMilliseconds:number|null;deadlineEpochMilliseconds:number|null;facts:AcceptedPhysicalCleanupFactsV1;body:ControlScope;bodyController:AbortController;cleanupController:AbortController;bodyTimer?:ReturnType<typeof setTimeout>;wholeTimer?:ReturnType<typeof setTimeout>;begin?:Promise<void>;phase:"body"|"cleanup"};
const cleanups=new WeakMap<AcceptedPhysicalCleanupV1,PhysicalCleanupScope>();
function cleanupOf(token:AcceptedPhysicalCleanupV1):PhysicalCleanupScope{const accepted=cleanups.get(token);if(!accepted?.live||!accepted.body.birth.live)throw Error("Live accepted physical cleanup enrollment required");return accepted;}
/** 🛟️ Accepts fixed terminal cleanup before effects, carving finite bodies or retaining an ordinary body's Original. */
export function withAcceptedPhysicalCleanupV1<T>(control:ResourceLeaseControl,request:PhysicalCleanupRequestV1,run:AcceptedPhysicalCleanupBodyV1<T>):Promise<T>{
 requireIssuedResourceControlV1(control);request=readScriptProcessJsonV1(request)as PhysicalCleanupRequestV1;if(validateJsonSchemaSubset(cleanupSchema,request).length)throw Error("Closed physical cleanup funding request required");const original=controls.get(control)!,acceptedAtEpochMilliseconds=Date.now(),available=remaining(original),deadlineEpochMilliseconds=available===null?null:Math.min(original.deadlineEpochMilliseconds??Infinity,acceptedAtEpochMilliseconds+Math.floor(available)),reserveMilliseconds=request.reserveMilliseconds;if(deadlineEpochMilliseconds!==null&&(!Number.isSafeInteger(deadlineEpochMilliseconds)||deadlineEpochMilliseconds-acceptedAtEpochMilliseconds<=reserveMilliseconds))throw Error("Finite Original physical cleanup reserve required before effects");
 const bodyDeadlineEpochMilliseconds=deadlineEpochMilliseconds===null?null:deadlineEpochMilliseconds-reserveMilliseconds,cleanupController=new AbortController(),bodyController=new AbortController(),bodyControl=controlFor(original.invocation,original.command,bodyDeadlineEpochMilliseconds===null?0:bodyDeadlineEpochMilliseconds-acceptedAtEpochMilliseconds,original,bodyController.signal),body=controls.get(bodyControl)!,token=Object.freeze({})as AcceptedPhysicalCleanupV1,facts:AcceptedPhysicalCleanupFactsV1=Object.freeze({version:1,kind:request.kind,resource:request.resource,acceptedAtEpochMilliseconds,deadlineEpochMilliseconds,bodyDeadlineEpochMilliseconds,reserveMilliseconds,cleanupSignal:cleanupController.signal}),bodyTimer=bodyDeadlineEpochMilliseconds===null?undefined:setTimeout(()=>bodyController.abort(Error("Original physical body cutoff reached")),Math.max(0,bodyDeadlineEpochMilliseconds-Date.now())),wholeTimer=deadlineEpochMilliseconds===null?undefined:setTimeout(()=>cleanupController.abort(Error("Original physical cleanup deadline exhausted")),Math.max(0,deadlineEpochMilliseconds-Date.now()));body.deadlineEpochMilliseconds=bodyDeadlineEpochMilliseconds;const accepted:PhysicalCleanupScope={live:true,started:performance.now(),maximumMilliseconds:deadlineEpochMilliseconds===null?null:deadlineEpochMilliseconds-acceptedAtEpochMilliseconds,deadlineEpochMilliseconds,facts,body,bodyController,cleanupController,bodyTimer,wholeTimer,phase:"body"};cleanups.set(token,accepted);
 return track(original,async()=>{let value:T|undefined,failed=false,failure:unknown;try{value=await run(token,bodyControl);}catch(error){failed=true;failure=error;}const failures:unknown[]=failed?[failure]:[];try{await beginAcceptedPhysicalCleanupV1(token);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}try{checkAcceptedPhysicalCleanupV1(token);}catch(error){if(!failures.some(value=>Object.is(value,error)))failures.push(error);}finally{accepted.live=false;body.live=false;clearTimeout(accepted.bodyTimer);clearTimeout(accepted.wholeTimer);}if(failures.length>1)throw new AggregateError(failures,"Original physical body, drain and cleanup settlement failed");if(failures.length)throw failures[0];return value as T;});
}
/** 📋️ Reads immutable funding facts from the exact live private enrollment. */
export function readAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):AcceptedPhysicalCleanupFactsV1{return cleanupOf(token).facts;}
/** 🧹️ Stops body admission once and joins accepted body work before any cleanup phase. */
export function beginAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):Promise<void>{const accepted=cleanupOf(token);if(accepted.begin)return accepted.begin;accepted.body.accepting=false;if(accepted.deadlineEpochMilliseconds===null){accepted.started=performance.now();accepted.maximumMilliseconds=accepted.facts.reserveMilliseconds;accepted.deadlineEpochMilliseconds=Date.now()+accepted.facts.reserveMilliseconds;accepted.wholeTimer=setTimeout(()=>{const error=Error("Original physical cleanup deadline exhausted");accepted.cleanupController.abort(error);accepted.bodyController.abort(error);},accepted.facts.reserveMilliseconds);}accepted.begin=observe((async()=>{await drain(accepted.body);clearTimeout(accepted.bodyTimer);accepted.body.live=false;accepted.phase="cleanup";if(accepted.body.failures.length)throw accepted.body.failures[0];})());return accepted.begin;}
/** ⏱️ Checks only the accepted cleanup phase and its nonrenewable whole-operation deadline. */
export function checkAcceptedPhysicalCleanupV1(token:AcceptedPhysicalCleanupV1):number{const accepted=cleanupOf(token);if(accepted.phase!=="cleanup"||accepted.deadlineEpochMilliseconds===null||accepted.maximumMilliseconds===null)throw Error("Accepted physical body must stop before cleanup");accepted.facts.cleanupSignal.throwIfAborted();const remaining=Math.min(accepted.deadlineEpochMilliseconds-Date.now(),accepted.maximumMilliseconds-(performance.now()-accepted.started));if(remaining<=0)throw Error("Original physical cleanup deadline exhausted");return remaining;}

/** 📨️ Exposes immutable carried data from the exact live parent for domain-owned closed child projection. */
export function readIssuedScriptProcessEnvelopeV1(invocation:ScriptInvocation):ScriptProcessEnvelope{requireIssuedScriptInvocationV1(invocation);return birthOf(invocation).envelope;}

/** 🪪️ Binds a Native owner to this enrollment's exact issued body control before physical effects. */
export function requireAcceptedPhysicalCleanupControlV1(token:AcceptedPhysicalCleanupV1,control:ResourceLeaseControl):void{const accepted=cleanupOf(token);if(controls.get(control)!==accepted.body)throw Error("Exact accepted physical body control required");}
