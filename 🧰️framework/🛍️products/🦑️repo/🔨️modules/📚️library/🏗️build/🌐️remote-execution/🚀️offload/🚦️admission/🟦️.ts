import {existsSync,mkdirSync,readdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {availableParallelism} from "node:os";
import {evaluateHostCapacityV1,type HostCapacityPolicyV1,type HostReasonV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {currentHostProbeV1,readHostSampleV1,type HostProbeV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {claimHeldV1,processAliveV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadModeV1} from "../📐️policy/🟦️.ts";

const limits=OFFLOAD_POLICY_V1.limits;
/** 🚦️ The machine's own answer to "may one more compiler unit start here now". */
export type OffloadAdmissionV1=Readonly<{verdict:"admit"|"wait";reason:HostReasonV1;atMs:number}>;
/** 🔌️ The seam of the host-capacity decision: tests hand in a probe that reports a pressured machine or a policy, production uses the machine and the authored policy. */
export type OffloadAdmissionPortsV1=Readonly<{probe?:HostProbeV1;policy?:HostCapacityPolicyV1;now?:()=>number;signal?:AbortSignal}>;

/** 🚦️ Asks the single admission decision of the host (`evaluateHostCapacityV1`) about one more unit: a sample of the machine, an empty ledger (what is committed by other builds already shows in the sample's free memory, swap, pressure and load) and one waiting request for one job of the unit's size. `wait` means the unit would queue here. */
export async function localUnitAdmissionV1(ports:OffloadAdmissionPortsV1={}):Promise<OffloadAdmissionV1>{
 const now=(ports.now??Date.now)(),policy=ports.policy??(await import("../../../🌡️host-capacity/🎛️control/🟦️.ts")).BUILD_HOST_CAPACITY_POLICY_V1;
 const signal=ports.signal,probe=ports.probe??currentHostProbeV1(signal);
 signal?.throwIfAborted();
 const record=await readHostSampleV1(policy,signal===undefined||probe.signal===signal?probe:{...probe,signal:probe.signal===undefined?signal:AbortSignal.any([probe.signal,signal])},undefined,()=>now),host:HostSampleV1=record.host;
 signal?.throwIfAborted();
 const request:HostRequestV1={id:"offload-"+process.pid,operation:"rustc",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:0,perJobMegabytes:limits.unitMegabytes,cpuPerJob:1};
 const decision=evaluateHostCapacityV1(policy,host,[],[{request,arrivedAtMs:now,pid:process.pid}],now).decisions[0]!;
 return Object.freeze({verdict:decision.verdict,reason:decision.reason,atMs:now});
}

const memoPath=(root:string,name:string)=>join(root,"offload-"+name+".json");
/** 📝️ Reads a short-lived verdict shared by every compiler call of the machine, so a build of hundreds of units samples the host and asks the fleet about once per memo window. */
export function readOffloadMemoV1<T extends {atMs:number}>(root:string,name:string,now:number,windowMilliseconds:number):T|undefined{
 try{const value=JSON.parse(readFileSync(memoPath(root,name),"utf8")) as T;return now-value.atMs<windowMilliseconds&&now>=value.atMs?value:undefined;}catch{return undefined;}
}
/** 🖊️ Writes the shared verdict atomically; a failure only costs the next caller one more question. */
export function writeOffloadMemoV1(root:string,name:string,value:{atMs:number}):void{
 try{mkdirSync(root,{recursive:true});const path=memoPath(root,name),temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,path);}catch{}
}
/** 🚦️ The host's verdict, shared within the memo window. */
export async function sharedUnitAdmissionV1(root:string,ports:OffloadAdmissionPortsV1={}):Promise<OffloadAdmissionV1>{
 const now=(ports.now??Date.now)(),memo=readOffloadMemoV1<OffloadAdmissionV1>(root,"admission",now,limits.admissionMemoMilliseconds);
 if(memo)return memo;
 const fresh=await localUnitAdmissionV1(ports);writeOffloadMemoV1(root,"admission",fresh);return fresh;
}

/** ⏳️ How long a unit would queue for a compile slot of this machine, from the slot files of the unit cache without joining the queue: `0` while a slot is free, else the age of the oldest live waiter (at least 1). A unit with no slot limit never queues. */
export function slotQueueMillisecondsV1(root:string,slots:number,now:number=Date.now()):number{
 if(slots<1)return 0;
 const layout=UNIT_CACHE_POLICY_V1.layout;let free=false;
 for(let index=0;index<slots;index++)if(!claimHeldV1(join(root,layout.slots),String(index))){free=true;break;}
 if(free)return 0;
 let oldest=0;
 try{
  for(const name of readdirSync(join(root,layout.tickets)).sort()){
   const match=/^(\d{15})-[0-9a-f]+-(\d+)$/u.exec(name);
   if(match&&processAliveV1(Number(match[2]))){oldest=Math.max(0,now-Number(match[1]));break;}
  }
 }catch{}
 return Math.max(1,oldest);
}

/** 🌊️ Returns the configured fleet fanout ceiling when a scheduler is published and a local unit would queue. The ceiling is separate from the Cargo launcher’s local compiler lease. */
export async function offloadFanOutJobsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|undefined>{
 if(offloadModeV1(environment)==="never")return undefined;
 const fleet=await import("../../../🌐️remote-cache/🛰️fleet/🟦️.ts"),root=environment.SEMIO_WORKSPACE_ROOT??fleet.fleetCacheRootV1(environment)??workspaceRootFromV1(process.cwd());
 const published=environment.SEMIO_REMOTE_EXECUTION_ENDPOINT||fleet.readFleetExecutionV1(environment,root===undefined?{}:{root})?.endpoint;
 if(!published)return undefined;
 return (await localUnitAdmissionV1(ports)).verdict==="wait"?Math.min(limits.fanOutJobs,availableParallelism()):undefined;
}

/** 🧭️ The checkout that holds `cwd`: the nearest directory with the replica's `.cache/f/machine.json` (the build gate runs before the launcher states `SEMIO_WORKSPACE_ROOT`). */
export function workspaceRootFromV1(cwd:string):string|undefined{
 for(let at=cwd;;at=dirname(at)){if(existsSync(join(at,".cache","f","machine.json")))return at;if(dirname(at)===at)return undefined;}
}

const costPath=(root:string,crate:string)=>join(root,"c",crate.replace(/[^A-Za-z0-9_-]/gu,"_").slice(0,120));
/** 🧮️ What compiling a unit of this crate took here the last time (an average of the recorded local compiles), in milliseconds; `undefined` when none was recorded. Cheap: one small file per crate beside the unit cache. */
export function readUnitCostV1(root:string,crate:string):number|undefined{
 try{const value=Number(readFileSync(costPath(root,crate),"utf8"));return Number.isFinite(value)&&value>=0?value:undefined;}catch{return undefined;}
}
/** 🖊️ Records a local compile of a unit: the cost moves halfway to the new measurement. */
export function writeUnitCostV1(root:string,crate:string,milliseconds:number):void{
 try{const before=readUnitCostV1(root,crate),next=before===undefined?milliseconds:Math.round((before+milliseconds)/2),path=costPath(root,crate),temporary=path+"."+randomBytes(3).toString("hex");mkdirSync(join(root,"c"),{recursive:true});writeFileSync(temporary,String(next));renameSync(temporary,path);}catch{}
}
/** 🎯️ Whether a unit is worth a round trip: its recorded local cost is not below the measured round trip; a unit without a record is offloaded (the first compile teaches the cost). */
export function worthARoundTripV1(root:string,crate:string):boolean{
 const cost=readUnitCostV1(root,crate);return cost===undefined||cost>=limits.roundTripMilliseconds;
}
