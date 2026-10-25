import {createHash,randomBytes} from "node:crypto";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {acquireQueuedResourceLease,acquireResourceLease,checkResourceLeaseControl,type ResourceLease,type ResourceLeaseControl,withLeaseTaskV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const policy=schema.definitions.Policy.const;

/** 🎟️ How many publications may write into the shared Nx cache image at once: `SEMIO_NX_PUT_SLOTS` within 1..16, else the authored default. */
export function nxPutSlotCountV1(environment:Readonly<Record<string,string|undefined>>):number{
 const raw=Number(environment.SEMIO_NX_PUT_SLOTS);
 return Number.isSafeInteger(raw)&&raw>=1?Math.min(raw,policy.maximumSlots):policy.slots;
}

const key=(root:string)=>createHash("sha256").update(resolve(root).replaceAll("\\","/")).digest("hex").slice(0,16);
/** 🔒️ The exclusive lease resource of one slot of one image. */
export function nxPutSlotResourceV1(root:string,index:number):string{return `nx:put-slot:${key(root)}:${index}`;}
/** 🚪️ The queued gate that orders waiting publications first in, first out. */
export function nxPutGateResourceV1(root:string):string{return `nx:put-gate:${key(root)}`;}

/** 🎫️ One admitted publication: its slot and a probe of how many other slots are in use right now. */
export interface NxPutSlotV1 {readonly index:number;others():Promise<number>}

const timedOut=(resource:string)=>(error:unknown)=>error instanceof Error&&error.message==="Resource lease timed out: "+resource;
const sleep=(milliseconds:number,signal:AbortSignal)=>new Promise<void>((done,fail)=>{
 if(signal.aborted)return fail(signal.reason instanceof Error?signal.reason:Error("Nx put slot wait cancelled"));
 const timer=setTimeout(()=>{signal.removeEventListener("abort",abort);done();},milliseconds),abort=()=>{clearTimeout(timer);fail(signal.reason instanceof Error?signal.reason:Error("Nx put slot wait cancelled"));};
 signal.addEventListener("abort",abort,{once:true});
});

/** 🧵️ Runs one publication as its own lease task, so concurrent publications of one process wait for each other's gate and slot exactly like publications of separate processes. */
export function withNxPutSlotV1<T>(options:Readonly<{root:string;leaseDirectory:string;control:ResourceLeaseControl;slots:number}>,body:(slot:NxPutSlotV1)=>Promise<T>):Promise<T>{return withLeaseTaskV1(()=>admitNxPutSlotV1(options,body));}

/** 🆓️ Runs one publication in a free slot. The oldest waiter holds the gate and polls the slots with a bounded back-off; the gate is released as soon as a slot is taken, and a slot holder waits for nothing but the shared writer custody, so the wait graph stays acyclic. Cancellation and the lease control are honoured on every poll. */
async function admitNxPutSlotV1<T>(options:Readonly<{root:string;leaseDirectory:string;control:ResourceLeaseControl;slots:number}>,body:(slot:NxPutSlotV1)=>Promise<T>):Promise<T>{
 const {root,leaseDirectory,control}=options,count=Math.min(Math.max(1,options.slots),policy.maximumSlots),base={directory:leaseDirectory,mode:"exclusive" as const,signal:control.signal,control};
 let held:{index:number;lease:ResourceLease}|undefined;
 const gate=await acquireQueuedResourceLease({...base,resource:nxPutGateResourceV1(root),owner:randomBytes(8).toString("hex")});
 try{
  for(let delay=policy.pollInitialMilliseconds;!held;delay=Math.min(policy.pollMaximumMilliseconds,delay*2)){
   checkResourceLeaseControl(control);
   for(let index=0;index<count&&!held;index++){
    const resource=nxPutSlotResourceV1(root,index);
    try{held={index,lease:await acquireResourceLease({...base,resource,timeoutMs:0})};}catch(error){if(!timedOut(resource)(error))throw error;}
   }
   if(!held)await sleep(delay,control.signal);
  }
 }finally{gate.release();}
 if(!held)throw Error("Nx put slot admission required");
 const slot=held,others=async()=>{let busy=0;for(let index=0;index<count;index++){if(index===slot.index)continue;const resource=nxPutSlotResourceV1(root,index);try{(await acquireResourceLease({...base,resource,timeoutMs:0})).release();}catch(error){if(!timedOut(resource)(error))throw error;busy++;}}return busy;};
 try{return await body(Object.freeze({index:slot.index,others}));}finally{slot.lease.release();}
}
