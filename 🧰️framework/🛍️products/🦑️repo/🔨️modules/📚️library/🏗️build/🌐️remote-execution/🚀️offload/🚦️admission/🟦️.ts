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
export type OffloadAdmissionPortsV1=Readonly<{probe?:HostProbeV1;policy?:HostCapacityPolicyV1;now?:()=>number;signal?:AbortSignal;doorRefused?:boolean;fleet?:Readonly<{fetch?:typeof fetch;timeoutMilliseconds?:number}>}>;

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
export function writeOffloadMemoV1<T extends {atMs:number}>(root:string,name:string,value:T):void{
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

/** 🌡️ How many other machines could take a unit of this machine right now, asked of the scheduler itself (`/v1/fleet/idle?origin=`: signed like every fleet call, one request with a time limit): the answer is the counting the scheduler runs when a unit is submitted (platform key with toolchain and target, action format, headroom, slots, refusal cool-down, never the requester) over its live table at that moment, so the door and the submission cannot disagree about a table; only time can move between the two. `0` when no scheduler is published, it does not answer in time or nobody qualifies; never throws. */
export async function fleetHeadroomV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number>{
 const answer=await fleetIdleAnswerV1(environment,ports);
 return Number.isInteger(answer?.candidates)&&answer!.candidates!>0?answer!.candidates!:0;
}

/** 🛰️ How many action slots other machines of the same platform and toolchain have free right now, the width a build sizes its jobs to (`planFleetJobsV1`): the `slots` of the scheduler's idle answer, else its candidate count for a scheduler that does not count slots yet; `null` when offloading is off, no scheduler is published or it does not answer in time, so the build stays at its local grant. Never throws. */
export async function fleetIdleSlotsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|null>{
 if(offloadModeV1(environment)==="never")return null;
 const answer=await fleetIdleAnswerV1(environment,ports);
 if(answer===undefined)return null;
 const value=Number.isInteger(answer.slots)?answer.slots:answer.candidates;
 return Number.isInteger(value)&&value!>=0?value!:null;
}

/** 📡️ The scheduler's own idle counting for this machine (`/v1/fleet/idle?origin=`, signed, one request with a time limit); `undefined` when none is published, it does not answer in time or refuses. Never throws. */
async function fleetIdleAnswerV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1):Promise<{candidates?:number;slots?:number}|undefined>{
 try{
  const fleet=await import("../../../🌐️remote-cache/🛰️fleet/🟦️.ts"),{resolveRemoteExecutionEndpointV1,resolveRemoteExecutionSchedulerV1}=await import("../../🧭️endpoint/🟦️.ts"),{remoteExecutionOriginV1}=await import("../../🌡️load/🟦️.ts");
  const root=environment.SEMIO_WORKSPACE_ROOT??fleet.fleetCacheRootV1(environment)??workspaceRootFromV1(process.cwd());
  const published=resolveRemoteExecutionEndpointV1({environment,...root===undefined?{}:{root}})?.endpoint;
  if(!published)return undefined;
  const scheduler=await resolveRemoteExecutionSchedulerV1({environment,...root===undefined?{}:{root}},published);
  if(scheduler===undefined)return undefined;
  const call=ports.fleet?.fetch??(root===undefined?fetch:fleet.remoteCacheFetchV1({...environment,SEMIO_WORKSPACE_ROOT:root},{root,audience:"execution",roleOf:()=>"submit"})),own=root===undefined?"":remoteExecutionOriginV1(root,environment);
  const response=await call(scheduler.url.replace(/\/+$/u,"")+"/v1/fleet/idle?origin="+encodeURIComponent(own),{signal:AbortSignal.any([...ports.signal?[ports.signal]:[],AbortSignal.timeout(ports.fleet?.timeoutMilliseconds??limits.fleetStatusMilliseconds)])});
  if(!response.ok){await response.body?.cancel();return undefined;}
  return await response.json() as {candidates?:number;slots?:number};
 }catch{return undefined;}
}

/** 🌊️ Returns the fleet fanout ceiling when offloading can relieve this machine: a scheduler is published, at least one other machine has headroom (`fleetHeadroomV1`), and either the door refused the whole build (`doorRefused`, the one verdict the build then hands to every compiler call through `SEMIO_OFFLOAD_PRESSURE`) or a local unit would queue. `undefined` keeps the caller waiting as before: no scheduler, nobody with headroom, or a calm machine. Under `pin` every unit goes to the named machine, so the ceiling is granted without pressure. */
export async function offloadFanOutJobsV1(environment:Readonly<Record<string,string|undefined>>,ports:OffloadAdmissionPortsV1={}):Promise<number|undefined>{
 if(offloadModeV1(environment)==="never")return undefined;
 const fleet=await import("../../../🌐️remote-cache/🛰️fleet/🟦️.ts"),{resolveRemoteExecutionEndpointV1}=await import("../../🧭️endpoint/🟦️.ts"),root=environment.SEMIO_WORKSPACE_ROOT??fleet.fleetCacheRootV1(environment)??workspaceRootFromV1(process.cwd());
 const published=resolveRemoteExecutionEndpointV1({environment,...root===undefined?{}:{root}})?.endpoint;
 if(!published)return undefined;
 if(offloadModeV1(environment)==="pin")return Math.min(limits.fanOutJobs,availableParallelism());
 if(ports.doorRefused!==true&&(await localUnitAdmissionV1(ports)).verdict!=="wait")return undefined;
 return await fleetHeadroomV1(environment,ports)>0?Math.min(limits.fanOutJobs,availableParallelism()):undefined;
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

/** 🧭️ The signals one unit's offload decision reads, each asked only when the cheaper ones before it did not decide: a unit worth a round trip, the door's verdict on the whole build (`SEMIO_OFFLOAD_PRESSURE=door`), the unit-cache slot queue, the host-capacity verdict and the build's own admission broker (`wait` when the unit would queue behind the memory budget or a frozen broker, `unknown` when no broker answers). */
export type OffloadGateInputV1=Readonly<{mode:"distribute"|"pin"|"never";door:boolean;worth:()=>boolean;slotQueueMilliseconds:()=>number;host:()=>Promise<"admit"|"wait">;broker:()=>Promise<"admit"|"wait"|"unknown">}>;
/** 🧭️ Why a unit is offered to the fleet or stays: the first signal that decides (`headroom`: this machine is calm, the unit goes where another machine has room). */
export type OffloadGateReasonV1="never"|"pin"|"cheap"|"door"|"slot-queue"|"host-wait"|"broker-wait"|"headroom";
/** 🚪️ Whether a unit is offered to the fleet, and whether it queues there. `pin` offers every unit and queues it on the named machine. `distribute` offers every unit worth a round trip: where this machine is saturated (the door refused the build, a slot queue, the host or the build's memory budget would make the unit wait) the unit queues on the fleet because waiting here would be no better; where this machine is calm, the fleet takes it only if another machine has headroom now, else it compiles here. Unknown signals count as calm. */
export async function offloadGateV1(input:OffloadGateInputV1):Promise<Readonly<{offload:boolean;queue:boolean;reason:OffloadGateReasonV1}>>{
 if(input.mode==="never")return{offload:false,queue:false,reason:"never"};
 if(input.mode==="pin")return{offload:true,queue:true,reason:"pin"};
 if(!input.worth())return{offload:false,queue:false,reason:"cheap"};
 if(input.door)return{offload:true,queue:true,reason:"door"};
 if(input.slotQueueMilliseconds()>=limits.queueThresholdMilliseconds)return{offload:true,queue:true,reason:"slot-queue"};
 if(await input.host()==="wait")return{offload:true,queue:true,reason:"host-wait"};
 if(await input.broker()==="wait")return{offload:true,queue:true,reason:"broker-wait"};
 return{offload:true,queue:false,reason:"headroom"};
}
