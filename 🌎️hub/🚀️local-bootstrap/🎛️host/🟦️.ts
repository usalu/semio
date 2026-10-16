import {AsyncLocalStorage} from "node:async_hooks";
import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {readScriptProcessJsonV1} from "@semio-tech/process/routing/invocation/process/json";
import type {LocalProfile} from "../🛂authentication/🟦️.ts";
const policy=Object.freeze({...schema.definitions.Policy.const});
export type LocalHubStartDataV1=Readonly<{port?:number;capture?:boolean;credentialSignIn?:boolean;traceLevel?:"off"|"error"|"warn"|"info"|"debug"|"trace";adminSubjects?:readonly string[];isolatedSecuritySmoke?:boolean;adminToken?:string;inferenceCheckpointControl?:boolean}>;
export type LocalHubHostEventV1="run-start"|"run-close"|"accept"|"settle"|"stop"|"close";
export type LocalHubHostStateV1=Readonly<{phase:"open"|"closing"|"closed";runs:number;pending:number;births:number}>;
/** 📏️ Returns finite immutable lifecycle policy DATA without issuing a Host. */
export function localHubHostPolicyV1():Readonly<typeof policy>{return policy;}
/** 🧾️ Captures only closed own-data bootstrap values before any asynchronous work. */
export function captureLocalHubStartV1(profiles:readonly LocalProfile[],options:LocalHubStartDataV1):Readonly<{profiles:readonly LocalProfile[];options:LocalHubStartDataV1}>{const data=readScriptProcessJsonV1({profiles,options});if(validateJsonSchemaSubset(schema.definitions.Start,data,schema).length||Buffer.byteLength(JSON.stringify(data))>8192)throw Error("Finite local Hub profiles and options required");return data as Readonly<{profiles:readonly LocalProfile[];options:LocalHubStartDataV1}>;}
/** 🔢️ Reduces portable lifecycle DATA; it cannot authenticate physical or process custody. */
export function advanceLocalHubHostStateV1(state:LocalHubHostStateV1,event:LocalHubHostEventV1):LocalHubHostStateV1{
 let{phase,runs,pending,births}=state;if(phase==="closed")throw Error("Local Hub Host already closed");
 if(event==="run-start"){if(phase!=="open"||runs>=policy.maximumActiveRuns||births>=policy.maximumRunBirths)throw Error("Finite local Hub run admission refused");runs++;births++;}
 else if(event==="run-close"){if(runs<1)throw Error("Actual local Hub run required");runs--;}
 else if(event==="accept"){if(phase!=="open"||pending>=policy.maximumPendingOperations)throw Error("Finite local Hub operation admission refused");pending++;}
 else if(event==="settle"){if(pending<1)throw Error("Accepted local Hub work required");pending--;}
 else if(event==="stop"){phase="closing";}
 else if(event==="close"){if(phase!=="closing"||runs||pending)throw Error("Joined local Hub children and operations required");phase="closed";}
 else throw Error("Closed local Hub event required");return Object.freeze({phase,runs,pending,births});
}
/** 🧵️ Bounds and drains first-party in-process work; this ledger does not mint physical authority. */
export class LocalHubHostLedgerV1{
 private accepting=true;
 private live=true;
 private pending=new Set<Promise<unknown>>();
 private failed=false;
 private sequence=0;
 private readonly failures:{sequence:number;error:unknown}[]=[];
 private readonly frame=new AsyncLocalStorage<{live:boolean}>();
 run<T>(body:()=>Promise<T>):Promise<T>{if(!this.live||this.failed||!this.accepting&&!this.frame.getStore()?.live||this.pending.size>=policy.maximumPendingOperations||!Number.isSafeInteger(this.sequence+1)||typeof body!=="function")throw Error("Live bounded local Hub operation required");const sequence=++this.sequence,frame={live:true},work=Promise.resolve().then(()=>this.frame.run(frame,body));this.pending.add(work);void work.then(()=>{frame.live=false;this.pending.delete(work);},error=>{frame.live=false;this.failed=true;this.accepting=false;this.failures.push({sequence,error});this.pending.delete(work);});return work;}
 inside():boolean{return this.frame.getStore()?.live===true;}
 stop():void{this.accepting=false;}
 async drain():Promise<void>{if(this.frame.getStore()?.live)throw Error("Local Hub owner cannot drain its own operation");this.accepting=false;while(this.pending.size)await Promise.allSettled([...this.pending]);if(this.failed){const errors=this.failures.slice().sort((left,right)=>left.sequence-right.sequence).map(row=>row.error);if(errors.length===1)throw errors[0];throw new AggregateError(errors,"Accepted local Hub operations failed");}}
 close():void{if(this.pending.size)throw Error("Local Hub accepted work must settle before release");this.accepting=false;this.live=false;}
}

export type LocalHubReadinessProgressV1=Readonly<{observation:string;observedAt:number}>;
/** 📈️ Tracks only observed readiness progress; actual operation authority retains the total deadline. */
export function advanceLocalHubReadinessProgressV1(previous:LocalHubReadinessProgressV1|null,observation:string,now:number,bound:number):LocalHubReadinessProgressV1{
 if(typeof observation!=="string"||Buffer.byteLength(observation)>policy.maximumOutputBytes+policy.maximumReadinessBytes||!Number.isSafeInteger(now)||now<0||!Number.isSafeInteger(bound)||bound<1||bound>policy.maximumReadinessStallMilliseconds||previous&&(!Number.isSafeInteger(previous.observedAt)||previous.observedAt<0||typeof previous.observation!=="string"))throw Error("Finite readiness observation required");
 if(!previous||previous.observation!==observation)return Object.freeze({observation,observedAt:now});
 if(now-previous.observedAt>=bound)throw Error(`hub readiness stalled — nothing changed for ${now-previous.observedAt} ms; last observation: ${observation}`);return previous;
}
