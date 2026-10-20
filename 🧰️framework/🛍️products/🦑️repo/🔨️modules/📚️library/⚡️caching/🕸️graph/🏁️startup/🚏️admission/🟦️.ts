import {createHash} from "node:crypto";
import {totalmem} from "node:os";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {acquireResourceLease,type ResourceLease} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {nxDaemonNamespaceResourceV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🚏️ The two daemon families that are counted separately: finite run daemons and dev watch daemons. */
export type NxDaemonKindV1="finite"|"watch";
/** 🚏️ The limits of this host and environment. */
export type NxDaemonLimitsV1=Readonly<{finite:number;watch:number;idleMilliseconds:number;checkMilliseconds:number}>;
/** 🚏️ The answer to "may this run use a daemon". */
export type NxDaemonDecisionV1=Readonly<{enabled:boolean;reason:"explicit"|"reuse"|"admitted"|"saturated"|"disabled"|"refused"}>;

const natural=(value:string|undefined):number|undefined=>value!==undefined&&/^\d{1,9}$/u.test(value)?Number(value):undefined;

/** 📏️ Daemon limits: the authored maxima, the finite one reduced to the memory share (a quarter of the host memory divided by 2 GiB per daemon), overridable by `SEMIO_NX_DAEMONS=<finite>[,<watch>]` (0 turns the family off) and `SEMIO_NX_DAEMON_IDLE_MS`. */
export function nxDaemonLimitsV1(environment:Readonly<Record<string,string|undefined>>,totalMemoryBytes:number=totalmem()):NxDaemonLimitsV1{
 const budget=Math.floor(totalMemoryBytes/1048576/policy.memoryShareDenominator/policy.megabytesPerDaemon),[finiteText,watchText]=(environment.SEMIO_NX_DAEMONS??"").split(","),finiteOverride=natural(finiteText),watchOverride=natural(watchText);
 return Object.freeze({
  finite:Math.min(policy.maximumSlots,finiteOverride??Math.min(policy.finite,budget)),
  watch:Math.min(policy.maximumSlots,watchOverride??policy.watch),
  idleMilliseconds:natural(environment.SEMIO_NX_DAEMON_IDLE_MS)??policy.idleMilliseconds,
  checkMilliseconds:policy.checkMilliseconds
 });
}

/** ⚖️ The admission rule: an explicit `NX_DAEMON` wins, a namespace with a live daemon is reused, otherwise a daemon is admitted while fewer than `limit` of its family live. */
export function decideNxDaemonV1(input:Readonly<{explicit:string|undefined;namespaceLive:boolean;live:number;limit:number}>):NxDaemonDecisionV1{
 if(input.explicit!==undefined)return Object.freeze({enabled:input.explicit!=="false",reason:"explicit"});
 if(input.namespaceLive)return Object.freeze({enabled:true,reason:"reuse"});
 if(input.limit<=0)return Object.freeze({enabled:false,reason:"disabled"});
 return input.live<input.limit?Object.freeze({enabled:true,reason:"admitted"}):Object.freeze({enabled:false,reason:"saturated"});
}

/** 🕰️ True when a finite daemon has had no request open and none started for the idle period; watch daemons never idle out. */
export function nxDaemonIdleV1(state:Readonly<{kind:NxDaemonKindV1;openRequests:number;lastActivity:number;now:number;idleMilliseconds:number}>):boolean{
 return state.kind==="finite"&&state.openRequests===0&&state.idleMilliseconds>0&&state.now-state.lastActivity>=state.idleMilliseconds;
}

const key=(root:string)=>createHash("sha256").update(resolve(root).replaceAll("\\","/")).digest("hex").slice(0,16);
/** 🎟️ The lease resource of one daemon slot of one checkout. */
export function nxDaemonSlotResourceV1(root:string,kind:NxDaemonKindV1,index:number):string{return `nx:daemon-slot:${key(root)}:${kind}:${index}`;}

const timedOut=(resource:string)=>(error:unknown)=>error instanceof Error&&error.message==="Resource lease timed out: "+resource;
async function take(directory:string,resource:string,signal:AbortSignal):Promise<ResourceLease|undefined>{
 try{return await acquireResourceLease({directory,resource,mode:"exclusive",signal,timeoutMs:0});}catch(error){if(timedOut(resource)(error))return undefined;throw error;}
}

/** 🔎️ True while another process holds the resource (probed with a try-lock that is released at once). */
export async function nxDaemonResourceHeldV1(directory:string,resource:string,signal:AbortSignal):Promise<boolean>{
 const lease=await take(directory,resource,signal);if(!lease)return true;lease.release();return false;
}

/** 🔢️ Counts the slots of one family held by live daemons. */
export async function countNxDaemonSlotsV1(directory:string,root:string,kind:NxDaemonKindV1,signal:AbortSignal):Promise<number>{
 let live=0;for(let index=0;index<policy.maximumSlots;index++)if(await nxDaemonResourceHeldV1(directory,nxDaemonSlotResourceV1(root,kind,index),signal))live++;return live;
}

/** 🧷️ A daemon takes the first free slot of its family for its whole life and refuses to start (throws) when all `limit` slots are in use, so the cap holds even when two clients admitted a daemon at the same moment. */
export async function acquireNxDaemonSlotV1(directory:string,root:string,kind:NxDaemonKindV1,limit:number,signal:AbortSignal):Promise<ResourceLease>{
 for(let index=0;index<Math.min(limit,policy.maximumSlots);index++){const lease=await take(directory,nxDaemonSlotResourceV1(root,kind,index),signal);if(lease)return lease;}
 throw Error(`Nx daemon slots exhausted (${kind} limit ${limit})`);
}

/** 🚪️ Decides for one finite run whether Nx may use (and start) a daemon in the run's namespace. */
export async function nxDaemonAdmissionV1(root:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal):Promise<NxDaemonDecisionV1>{
 const directory=repositoryLeaseDirectoryV1(root),limits=nxDaemonLimitsV1(environment),namespace=environment.NX_WORKSPACE_DATA_DIRECTORY;
 if(environment.NX_DAEMON!==undefined)return decideNxDaemonV1({explicit:environment.NX_DAEMON,namespaceLive:false,live:0,limit:limits.finite});
 const namespaceLive=namespace!==undefined&&await nxDaemonResourceHeldV1(directory,nxDaemonNamespaceResourceV1(namespace),signal);
 return decideNxDaemonV1({explicit:undefined,namespaceLive,live:namespaceLive?0:await countNxDaemonSlotsV1(directory,root,"finite",signal),limit:limits.finite});
}
