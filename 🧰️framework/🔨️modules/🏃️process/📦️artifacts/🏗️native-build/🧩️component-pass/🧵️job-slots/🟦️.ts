import {validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{NativeBuildPassV1}from"../📋️contract/🟦️.ts";
import{ADMISSION_POLICY_V1,componentFinalKeyV1,componentFinalPeaksV1,predictPeakBytesV1,quantileR7V1,type UnitObservationV1}from"../../🎟️admission/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🎫️ What a running pass holds here: a `local` slot (a jobserver token and a reservation of its predicted peak when charged) or a `fleet` one (only its local footprint while compilation runs on workers). */
export type PassJobSlotV1="local"|"fleet";

/** 🛰️ Whether everything a pass still does after its inputs are shipped can run on a worker: its target is host independent, so its units, its final cdylib among them, are offloadable. Any other pass keeps the strictly sequential local behaviour.
 * @see ./🧬️schema/🔣️.json (`Policy.offloadableTargetPrefixes`) */
export const nativeBuildPassOffloadableV1=(pass:Pick<NativeBuildPassV1,"target">):boolean=>policy.offloadableTargetPrefixes.some(prefix=>pass.target.startsWith(prefix));

/** 🧵️ Why a local component job may or may not start beside the running ones (also why a plan of local jobs is as large as it is): the first job always starts, the reservation fits, every listed final fits, the authored cap or the local grant is reached, the free memory above the floor is too small or cannot be read, or memory pressure is raised. */
export type LocalJobReasonV1="local-first"|"local-fits"|"local-all"|"local-cap"|"local-grant"|"parent-memory"|"local-memory"|"local-pressure"|"local-unknown";
/** 🌡️ The memory pressure level of the machine as the capacity probe reports it. */
export type LocalJobPressureV1="normal"|"warn"|"critical"|null;
/** 📸️ One memory sample of this machine: the free megabytes (`null` when unreadable), the megabytes the compilers of this build already hold of them (`ownedMegabytes`, added back because the reservations are charged at their whole peak) and the pressure level. */
export type LocalJobSampleV1=Readonly<{availableMegabytes:number|null;ownedMegabytes:number;pressure:LocalJobPressureV1}>;
/** 📣️ What the runner tells about a local job that waits for memory or starts and reserves its peak. */
export type LocalJobEventV1=Readonly<{kind:"waiting"|"started";running:number;peakMegabytes:number;freeMegabytes:number;floorMegabytes:number;reason:LocalJobReasonV1;waitedMilliseconds:number}>;
/** 🏠️ Up to `maximum` local jobs start in one lane. The parent grant, when present, covers the predicted `peak` and `footprintMegabytes` for each job. Further jobs also observe the host memory sample and floor. */
export type LocalJobsOptionsV1<T>=Readonly<{maximum:number;floorMegabytes:number;peak:(pass:T)=>number|Promise<number>;sample:()=>Promise<LocalJobSampleV1>;footprintMegabytes?:number;report?:(pass:T,event:LocalJobEventV1)=>void;pollMilliseconds?:number;progressMilliseconds?:number;now?:()=>number}>;

/** 🌐️ Why a pass may or may not start on a fleet slot: a worker slot is free and the small local footprint fits, the fleet does not answer, every worker slot that fits a final is taken, the authored cap of fleet jobs is reached, memory pressure is critical or the free memory above the floor does not carry another fleet job. */
export type FleetJobReasonV1="fleet-fits"|"fleet-none"|"fleet-busy"|"fleet-cap"|"fleet-pressure"|"fleet-memory";
/** 🧭️ Which kind of slot a waiting pass is waiting for: a `local` one (no worker slot can take it), a `fleet` one (the local jobs cannot grow) or `either`, whichever frees first. */
export type FleetJobWaitV1="local"|"fleet"|"either";
/** 📣️ What the runner tells about a fleet slot: a pass starts on it and reserves its footprint, or it waits (repeated as progress) for `waitsFor`. `slots` are the worker slots that fit one more final now (`null` when the fleet does not answer), `running` the fleet jobs held here. */
export type FleetJobEventV1=Readonly<{kind:"waiting"|"started";waitsFor:FleetJobWaitV1|null;running:number;slots:number|null;footprintMegabytes:number;peakMegabytes:number;freeMegabytes:number;floorMegabytes:number;fleetReason:FleetJobReasonV1|null;localReason:LocalJobReasonV1|null;waitedMilliseconds:number}>;
/** 🌐️ How a lane runs component jobs on the FLEET: up to `maximum` fleet jobs at once, each charging `footprintMegabytes` of this machine (its cargo process, the rustc wrappers and the input shipping; its units and final run on workers), started only while `slots` (the worker slots that fit one more final, re-read every `pollMilliseconds`) are free and the memory `sample` shows the footprints above the hard `floorMegabytes`; local memory pressure does not stop them, critical pressure does. */
export type FleetJobsOptionsV1<T>=Readonly<{maximum:number;slots:()=>Promise<number|null|undefined>;footprintMegabytes:number;floorMegabytes:number;sample:()=>Promise<LocalJobSampleV1>;waitForCompatibleSlot?:boolean;report?:(pass:T,event:FleetJobEventV1)=>void;pollMilliseconds?:number;progressMilliseconds?:number;now?:()=>number}>;
/** 🛑️ Raised by the body of a pass that holds a fleet slot when the worker placement of its final failed: the runner gives the fleet slot back and queues the pass for a LOCAL slot, where it waits for its reservation instead of compiling in memory nobody reserved. */
export class FleetPlacementFailedErrorV1 extends Error{constructor(message:string){super(message);this.name="FleetPlacementFailedErrorV1";}}

/** 🌱️ How a parent grant that may still grow tells and learns: `ceilingMegabytes` is the most this machine could ever give it (its memory less the floor of the capacity gate), `exact` that its holder never grows it in place (a requirement above it is carried only by `acquire`, a supplementary reservation of the difference beside the grant, `undefined` when the gate refuses it; without `acquire` an exact grant refuses at once), `waitMilliseconds` how long a requirement above the grant waits for it, `refresh` reads the allowance it holds now (megabytes, undefined when unreadable) and `declare` tells the holder what the run needs, so the class learns it even when the run is refused. */
export type ParentComponentGrowthV1=Readonly<{ceilingMegabytes:number;waitMilliseconds:number;refresh:()=>number|undefined;declare:(megabytes:number)=>void;exact?:boolean;acquire?:(extraMegabytes:number,signal?:AbortSignal)=>Promise<Readonly<{release:()=>void}>|undefined>}>;

/** 🧮️ The explicit memory allowance shared across every profile lane of a component build, with each pass charged until its body settles. A grant that may still grow (`growth`) follows the allowance the holder grants now, never below what it held, so a requirement above it waits instead of refusing while it can still be carried. */
export class ParentComponentMemoryV1{
 #heldMegabytes=0;
 #allowanceMegabytes:number;
 constructor(allowanceMegabytes:number,readonly baselineMegabytes=0,readonly growth?:ParentComponentGrowthV1){this.#allowanceMegabytes=allowanceMegabytes;}
 get allowanceMegabytes():number{return this.#allowanceMegabytes;}
 get maximumPassMegabytes():number{return this.#allowanceMegabytes-this.baselineMegabytes;}
 get remainingMegabytes():number{return this.maximumPassMegabytes-this.#heldMegabytes;}
 refresh():number{
  const live=this.growth?.refresh();
  if(live!==undefined&&Number.isFinite(live))this.#allowanceMegabytes=Math.max(this.#allowanceMegabytes,Math.floor(live));
  return this.#allowanceMegabytes;
 }
 #releases:(()=>void)[]=[];
 growable(megabytes:number):boolean{return this.growth!==undefined&&this.baselineMegabytes+megabytes<=this.growth.ceilingMegabytes&&(this.growth.acquire!==undefined||this.growth.exact!==true);}
 async acquire(megabytes:number,signal?:AbortSignal):Promise<boolean>{
  const extra=this.baselineMegabytes+megabytes-this.#allowanceMegabytes;
  if(extra<=0)return true;
  const held=await this.growth?.acquire?.(extra,signal);
  if(held===undefined)return false;
  this.#releases.push(held.release);this.#allowanceMegabytes+=extra;return true;
 }
 releaseGrowth():void{for(const release of this.#releases.splice(0))release();}
 reserve(megabytes:number):()=>void{
  if(megabytes>this.remainingMegabytes)throw this.required(megabytes);
  this.#heldMegabytes+=megabytes;
  return()=>{this.#heldMegabytes-=megabytes;};
 }
 required(megabytes:number):ParentComponentMemoryRequiredErrorV1{return new ParentComponentMemoryRequiredErrorV1(this.baselineMegabytes+megabytes,this.#allowanceMegabytes);}
}

/** 🛑️ No admissible route remains and the parent allowance cannot carry the required local job. */
export class ParentComponentMemoryRequiredErrorV1 extends Error{
 constructor(readonly requiredMegabytes:number,readonly allowanceMegabytes:number){
  const messages=schema.definitions.Messages.const,values={required:requiredMegabytes,allowance:allowanceMegabytes};
  super([messages.en.errors.parentMemoryRequired,messages.de.errors.parentMemoryRequired].map(message=>fill(message,values)).join(" / "));
  this.name="ParentComponentMemoryRequiredErrorV1";
 }
}

const pause=(milliseconds:number,wake:Iterable<Promise<void>>,signal?:AbortSignal):Promise<void>=>new Promise<void>(resolve=>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 const done=()=>{if(timer!==undefined)clearTimeout(timer);signal?.removeEventListener("abort",done);resolve();};
 timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",done,{once:true});
 if(signal?.aborted)done();
 for(const task of wake)void task.then(done,done);
});

/** 🧵️ Runs up to `jobs` offloadable passes. A shared `parentMemory` grant charges the first and subsequent local passes at their predicted peak plus footprint, and fleet passes at their local footprint. Without a parent grant the first pass starts locally under the host guard. Further local passes observe host memory and pressure. A fleet placement failure requeues the pass for local admission only. Non-offloadable passes run alone within the lane. The first failure calls `fail` and is thrown after running passes settle.
 * @see ./🧪️tests/🟦️.ts
 * @see ./🧪️tests/🌐️fleet/🟦️.ts */
export async function runPassesInJobSlotsV1<T extends Pick<NativeBuildPassV1,"target">>(passes:readonly T[],options:Readonly<{jobs:number;check:()=>void;mayStart?:()=>boolean;run:(pass:T,slot:PassJobSlotV1)=>Promise<void>;fail?:(error:unknown)=>void;offloadable?:(pass:T)=>boolean;fleet?:FleetJobsOptionsV1<T>;signal?:AbortSignal}>&(Readonly<{local?:LocalJobsOptionsV1<T>;parentMemory?:undefined}>|Readonly<{local:LocalJobsOptionsV1<T>;parentMemory:ParentComponentMemoryV1}>)):Promise<void>{
 const {parentMemory,local,fleet}=options;
 const running=new Set<Promise<void>>(),reservations=new Map<Promise<void>,number>(),footprints=new Map<Promise<void>,number>(),offloadable=options.offloadable??nativeBuildPassOffloadableV1,localMaximum=Math.max(1,local?.maximum??1),staticFleet=Math.max(0,options.jobs-localMaximum),fleetMaximum=fleet?Math.max(0,Math.min(fleet.maximum,options.jobs-(parentMemory?0:1))):staticFleet,now=local?.now??fleet?.now??Date.now,pending=[...passes],retries:T[]=[],states=new Map<T,{peak:number|undefined;waitedSince:number|undefined;reportedAt:number}>();
 let failure:{error:unknown}|undefined,localHeld=0,fleetHeld=0,idle:number|null|undefined,idleAt=-Infinity;
 const open = (): boolean => !failure && options.mayStart?.() !== false;
 const sum=(rows:Map<Promise<void>,number>)=>{let total=0;for(const value of rows.values())total+=value;return total;};
 const idleNow=async():Promise<number|null>=>{
  if(!fleet)return Math.max(0,staticFleet-fleetHeld);
  const at=now();
  if(idle===undefined||at-idleAt>=(fleet.pollMilliseconds??policy.fleetPollMilliseconds)){idleAt=at;const value=await fleet.slots().catch(()=>null);idle=value===null||value===undefined?null:Math.max(0,Math.floor(value));}
  return idle;
 };
 const launch=(pass:T,slot:PassJobSlotV1,peak:number,footprint:number,release?:()=>void):Promise<void>=>{
  if(slot==="local")localHeld++;else{fleetHeld++;if(typeof idle==="number")idle=Math.max(0,idle-1);}
  const task:Promise<void>=(async()=>options.run(pass,slot))().catch(error=>{
   if(slot==="fleet"&&error instanceof FleetPlacementFailedErrorV1&&!failure){states.delete(pass);retries.push(pass);return;}
   if(!failure){failure={error};options.fail?.(error);}
  }).finally(()=>{release?.();running.delete(task);reservations.delete(task);footprints.delete(task);if(slot==="local")localHeld--;else{fleetHeld--;idleAt=-Infinity;}});
  running.add(task);if(slot==="local")reservations.set(task,peak);else footprints.set(task,footprint);return task;
 };
 const attempt=async(pass:T,retry:boolean,localOnly=retry):Promise<"started"|"poll"|"settle">=>{
  if(running.size>=options.jobs)return"settle";
  let state=states.get(pass);if(!state){state={peak:undefined,waitedSince:undefined,reportedAt:-Infinity};states.set(pass,state);}
  let slot:PassJobSlotV1|undefined,verdict:LocalJobAdmitDecisionV1|undefined,fleetVerdict:FleetJobAdmitDecisionV1|undefined,slots:number|null=null;
  if(parentMemory&&state.peak===undefined)state.peak=Math.ceil(await local!.peak(pass));
  const required=(state.peak??0)+(local?.footprintMegabytes??0),footprint=fleet?.footprintMegabytes??0;
  parentMemory?.refresh();
  const remaining=()=>parentMemory?.remainingMegabytes??Infinity;
  if(localHeld===0&&required<=remaining())slot="local";
  else{
    if(!localOnly){
     if(fleet&&footprint<=(parentMemory?.maximumPassMegabytes??Infinity)){
      slots=await idleNow();
      const sample=await fleet.sample().catch(()=>({availableMegabytes:null,ownedMegabytes:0,pressure:null}) as LocalJobSampleV1);
      fleetVerdict=admitFleetComponentJobV1({running:fleetHeld,slots,maximum:fleetMaximum,availableMegabytes:sample.availableMegabytes,reservedMegabytes:Math.ceil(sum(footprints)),floorMegabytes:fleet.floorMegabytes,pressure:sample.pressure,footprintMegabytes:fleet.footprintMegabytes});
      if(fleetVerdict.admit&&footprint<=remaining())slot="fleet";
     }else if(!parentMemory&&fleetHeld<staticFleet)slot="fleet";
    }
    if(!slot&&local&&localHeld<localMaximum&&required<=remaining()){
     state.peak??=await local.peak(pass);
     const sample=await local.sample(),held=sum(reservations);
     verdict=required>remaining()?{admit:false,reason:"parent-memory",freeMegabytes:remaining()}:admitLocalComponentJobV1({running:localHeld,maximum:localMaximum,availableMegabytes:sample.availableMegabytes,ownedMegabytes:Math.min(sample.ownedMegabytes,held),reservedMegabytes:Math.ceil(held+sum(footprints)),floorMegabytes:local.floorMegabytes,pressure:sample.pressure,peakMegabytes:parentMemory?required:state.peak});
     if(verdict.admit)slot="local";
    }else if(!slot&&parentMemory&&required>remaining())verdict={admit:false,reason:"parent-memory",freeMegabytes:remaining()};
  }
  if(!open())return"settle";
  options.signal?.throwIfAborted();
  const waited=state.waitedSince===undefined?0:now()-state.waitedSince;
  if(slot){
   if(slot==="local"&&local&&state.peak===undefined)state.peak=await local.peak(pass);
   if(!open())return"settle";
   const extra=slot==="local"&&localHeld>0,peak=state.peak??0;
   if(extra&&local&&verdict)local.report?.(pass,{kind:"started",running:localHeld,peakMegabytes:peak,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:waited});
   if(slot==="fleet"&&fleet)fleet.report?.(pass,{kind:"started",waitsFor:null,running:fleetHeld,slots,footprintMegabytes:fleet.footprintMegabytes,peakMegabytes:peak,freeMegabytes:fleetVerdict?.freeMegabytes??0,floorMegabytes:fleet.floorMegabytes,fleetReason:fleetVerdict?.reason??null,localReason:verdict?.reason??null,waitedMilliseconds:waited});
   const release=parentMemory?.reserve(slot==="local"?required:footprint);
   (retry?retries:pending).shift();states.delete(pass);launch(pass,slot,parentMemory?required:peak,footprint,release);
   return"started";
  }
  const localFits=parentMemory===undefined||required<=parentMemory.maximumPassMegabytes;
  const fleetBusy=fleet?.waitForCompatibleSlot&&fleetVerdict?.reason==="fleet-busy"&&waited<policy.fleetBusyWaitMilliseconds;
  const growing=parentMemory!==undefined&&!localFits&&!fleetBusy&&parentMemory.growable(required)&&(state.waitedSince===undefined||waited<parentMemory.growth!.waitMilliseconds);
  if(growing&&parentMemory.growth!.acquire!==undefined){
   if(await parentMemory.acquire(required,options.signal))return"poll";
   throw parentMemory.required(required);
  }
  if(growing&&state.waitedSince===undefined)parentMemory.growth!.declare(parentMemory.baselineMegabytes+required);
  if(parentMemory&&!localFits&&!growing&&!fleetBusy&&!fleetVerdict?.admit)throw parentMemory.required(required);
  if(!fleetVerdict&&!(verdict&&local))return"settle";
  const at=now();state.waitedSince??=at;
  if(at-state.reportedAt>=(local?.progressMilliseconds??fleet?.progressMilliseconds??policy.localJobsProgressMilliseconds)){
   state.reportedAt=at;
   if(fleet){const fleetWait=fleetVerdict!==undefined&&fleetVerdict.reason!=="fleet-none",localWait=local!==undefined&&localFits,waitsFor:FleetJobWaitV1=fleetWait&&localWait?"either":fleetWait?"fleet":"local";fleet.report?.(pass,{kind:"waiting",waitsFor,running:fleetHeld,slots,footprintMegabytes:fleet.footprintMegabytes,peakMegabytes:state.peak??0,freeMegabytes:verdict?.freeMegabytes??fleetVerdict?.freeMegabytes??0,floorMegabytes:local?.floorMegabytes??fleet.floorMegabytes,fleetReason:fleetVerdict?.reason??null,localReason:verdict?.reason??null,waitedMilliseconds:at-state.waitedSince});}
   else if(verdict&&local)local.report?.(pass,{kind:"waiting",running:localHeld,peakMegabytes:state.peak!,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:at-state.waitedSince});
  }
  return"poll";
 };
 try{while(open()){
  const retry=retries[0],next=pending[0];
  if(retry===undefined&&next===undefined){if(!running.size)break;await Promise.race(running);continue;}
  options.check();
  options.signal?.throwIfAborted();
  const sequential=next!==undefined&&(!offloadable(next)||!parentMemory&&options.jobs<=1);
  if(retry===undefined&&sequential&&!parentMemory){if(running.size){await Promise.race(running);continue;}pending.shift();await launch(next,"local",0,0);continue;}
  let outcome:"started"|"poll"|"settle"="settle";
  if(retry!==undefined)outcome=await attempt(retry,true);
  if(!open())break;
  if(outcome!=="started"&&next!==undefined&&(!sequential||parentMemory&&retry===undefined&&!running.size)){
   const second=await attempt(next,false,sequential);
   if(second==="started"&&sequential&&running.size)await Promise.race(running);
   outcome=second==="started"?"started":second==="poll"||outcome==="poll"?"poll":"settle";
  }
  if(!open())break;
  if(outcome==="poll"||outcome==="settle"&&!running.size)await pause(local?.pollMilliseconds??fleet?.pollMilliseconds??policy.localJobsPollMilliseconds,running,options.signal);
  else if(outcome==="settle")await Promise.race(running);
 }}catch(error){if(!failure){failure={error};options.fail?.(error);}}
 while(running.size)await Promise.allSettled([...running]);
 if(failure)throw failure.error;
}

/** 🧵️ Why the concurrent component jobs of a build are fewer than every worker slot that fits one wasm final: no worker can take one, the authored cap, the memory the light remote jobs may hold here or raised memory pressure. */
export type ComponentJobsReasonV1="jobs-local-only"|"jobs-all"|"jobs-cap"|"jobs-memory"|"jobs-pressure";
/** 🌐️ What one component jobs plan reads: the local grant (`local`), the worker slots whose machine has the free memory for one component final at its predicted peak (`fleetSlots`, `null` when no fleet answers) and the live memory facts of this machine. */
export type ComponentJobsHostV1=Readonly<{local:number;fleetSlots:number|null;availableMegabytes:number|null;pressure:"normal"|"warn"|"critical"|null;unitMegabytes:number}>;
/** 🎟️ The planned concurrent component jobs: `jobs` is `local` (jobs that hold a local slot) plus `remote` (jobs whose remaining work is offloadable and that hold only the cargo process and the input shipping here). */
export type ComponentJobsDecisionV1=Readonly<{jobs:number;local:number;remote:number;reason:ComponentJobsReasonV1}>;

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
function requireDefinition<T>(definition:"FleetJobAdmitHost"|"FleetJobAdmitDecision"|"ComponentJobsHost"|"ComponentJobsDecision"|"LocalJobsHost"|"LocalJobsDecision"|"LocalJobAdmitHost"|"LocalJobAdmitDecision"|"ComponentJobSlotsHost"|"ComponentJobSlotsDecision",value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:errors[0]!}));
 return value as T;
}

/** 🧵️ Sizes the concurrent component jobs of one build: the local grant plus the worker slots that can take a wasm final (`fleetSlots`, counted at the predicted peak of the final), bounded by the authored cap (`componentJobsMaximum`) and by the memory the local residue of an offloaded job (`componentJobResidueMegabytes`: the cargo process, its fingerprint checks and the input shipping) may hold beside the local compiles; none under critical memory pressure. Raised pressure takes nothing off the fleet share: a remote job holds only its small known footprint here, so the memory term alone bounds it. Without a fleet answer the plan is the local grant, so a build with the fleet down behaves as before.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🧵️plan/🐍️oracle/🐍️.py */
export function planComponentJobsV1(hostValue:ComponentJobsHostV1):ComponentJobsDecisionV1{
 const host=requireDefinition<ComponentJobsHostV1>("ComponentJobsHost",hostValue);
 if(host.fleetSlots===null||host.fleetSlots===0)return Object.freeze(requireDefinition<ComponentJobsDecisionV1>("ComponentJobsDecision",{jobs:host.local,local:host.local,remote:0,reason:"jobs-local-only"}));
 let remote=host.fleetSlots,reason:ComponentJobsReasonV1="jobs-all";
 const limit=(value:number,why:ComponentJobsReasonV1)=>{if(value<remote){remote=Math.max(0,value);reason=why;}};
 limit(Math.max(0,policy.componentJobsMaximum-host.local),"jobs-cap");
 if(host.availableMegabytes!==null)limit(Math.floor(Math.max(0,host.availableMegabytes-policy.reserveMegabytes-host.local*host.unitMegabytes)/policy.componentJobResidueMegabytes),"jobs-memory");
 if(host.pressure==="critical")limit(0,"jobs-pressure");
 return Object.freeze(requireDefinition<ComponentJobsDecisionV1>("ComponentJobsDecision",{jobs:host.local+remote,local:host.local,remote,reason}));
}

/** 🧵️ What the slots of one lane are made of: the concurrent jobs the fleet plan allows (`fleetJobs`, the first local one counted, 1 without a fleet), the local compile grant (`tokens`) and the local jobs the caller wants at most (`wanted`). */
export type ComponentJobSlotsHostV1=Readonly<{fleetJobs:number;tokens:number;wanted:number}>;
/** 🧵️ The slots of one lane: `jobs` concurrent component jobs in all, `light` of them hold only the cargo process and the input shipping here (their units go to workers), `local` of them may hold a local slot, each beyond the first with a memory reservation. */
export type ComponentJobSlotsDecisionV1=Readonly<{jobs:number;light:number;local:number}>;

/** 🧵️ Combines BOTH resources into the slots of a lane: the light jobs the fleet carries (one less than `fleetJobs`, because the first job is local) and the local jobs (the authored `localJobsMaximum`, the local grant and what the caller wants), all bounded by the authored cap (`componentJobsMaximum`) of concurrent component jobs. A fleet that already fills the cap leaves one local slot, so a build with workers behaves as before; without a fleet the lane is local jobs only.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🧵️local/🐍️oracle/🐍️.py */
export function planComponentJobSlotsV1(hostValue:ComponentJobSlotsHostV1):ComponentJobSlotsDecisionV1{
 const host=requireDefinition<ComponentJobSlotsHostV1>("ComponentJobSlotsHost",hostValue),light=Math.max(0,Math.min(host.fleetJobs-1,policy.componentJobsMaximum-1)),wanted=Math.max(1,Math.min(host.wanted,policy.localJobsMaximum,host.tokens)),jobs=Math.max(1,Math.min(light+wanted,policy.componentJobsMaximum));
 return Object.freeze(requireDefinition<ComponentJobSlotsDecisionV1>("ComponentJobSlotsDecision",{jobs,light,local:Math.max(1,Math.min(wanted,jobs-light))}));
}

/** 🏠️ What one decision to start a local component job beside the running ones reads: the local jobs running now and the most that may run (`maximum`: authored cap and local grant), the free megabytes now (`null` when unreadable), how many of them the compilers of this build hold already (`ownedMegabytes`), the megabytes reserved by the running local jobs at their predicted peaks, the floor that stays free, the memory pressure and the predicted peak of the job that asks. */
export type LocalJobAdmitHostV1=Readonly<{running:number;maximum:number;availableMegabytes:number|null;ownedMegabytes:number;reservedMegabytes:number;floorMegabytes:number;pressure:LocalJobPressureV1;peakMegabytes:number}>;
/** 🎟️ The verdict: `admit`, why, and the megabytes that are free for a new reservation (free now plus what the running jobs hold of it, less their reservations and the floor). */
export type LocalJobAdmitDecisionV1=Readonly<{admit:boolean;reason:LocalJobReasonV1;freeMegabytes:number}>;
/** 🌐️ What a plan of local jobs reads: the local grant, the free memory now, the floor, the pressure and the predicted peaks of the next finals in start order. */
export type LocalJobsHostV1=Readonly<{tokens:number;availableMegabytes:number|null;floorMegabytes:number;pressure:LocalJobPressureV1;peaksMegabytes:readonly number[]}>;
/** 🏠️ The local component jobs a build may hold at once from memory alone: `jobs` finals, the first of them always. */
export type LocalJobsDecisionV1=Readonly<{jobs:number;reason:LocalJobReasonV1}>;

/** 🎟️ Decides whether one more component job may start on THIS machine: the first always does, further ones only below the cap, with readable memory and normal pressure, and while their predicted peak fits the memory that is free above the floor once the running jobs hold their whole reservation. The compilers of the running jobs already use part of that reservation, so their measured memory (at most the reservation) is added back to the free memory instead of being charged twice. Reservation and floor are never overridden by a waiting job: a job that does not fit waits.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🧵️local/🐍️oracle/🐍️.py */
export function admitLocalComponentJobV1(hostValue:LocalJobAdmitHostV1):LocalJobAdmitDecisionV1{
 const host=requireDefinition<LocalJobAdmitHostV1>("LocalJobAdmitHost",hostValue),owned=Math.min(host.ownedMegabytes,host.reservedMegabytes),free=host.availableMegabytes===null?0:Math.max(-4194304,host.availableMegabytes+owned-host.floorMegabytes-host.reservedMegabytes);
 const decide=(admit:boolean,reason:LocalJobReasonV1)=>Object.freeze(requireDefinition<LocalJobAdmitDecisionV1>("LocalJobAdmitDecision",{admit,reason,freeMegabytes:free}));
 if(host.running===0)return decide(true,"local-first");
 if(host.running>=host.maximum)return decide(false,"local-cap");
 if(host.availableMegabytes===null)return decide(false,"local-unknown");
 if(host.pressure!==null&&host.pressure!=="normal")return decide(false,"local-pressure");
 return host.peakMegabytes<=free?decide(true,"local-fits"):decide(false,"local-memory");
}

/** 🏠️ Sizes the local component jobs of one build from memory alone, with nothing running yet: the longest prefix of `peaksMegabytes` (the next finals in start order) whose whole reservations fit the free memory above the floor, at least the first, bounded by the authored cap (`localJobsMaximum`) and the local grant; one job when the memory cannot be read or its pressure is raised. It is the same law `admitLocalComponentJobV1` applies before each start, so the plan and the live decision agree.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🧵️local/🐍️oracle/🐍️.py */
export function planLocalComponentJobsV1(hostValue:LocalJobsHostV1):LocalJobsDecisionV1{
 const host=requireDefinition<LocalJobsHostV1>("LocalJobsHost",hostValue);
 let jobs=host.peaksMegabytes.length,reason:LocalJobReasonV1="local-all";
 const limit=(value:number,why:LocalJobReasonV1)=>{if(value<jobs){jobs=Math.max(1,value);reason=why;}};
 limit(policy.localJobsMaximum,"local-cap");
 limit(host.tokens,"local-grant");
 if(host.availableMegabytes===null)limit(1,"local-unknown");
 else{let sum=0,fit=0;const budget=host.availableMegabytes-host.floorMegabytes;for(const peak of host.peaksMegabytes){if(sum+peak>budget)break;sum+=peak;fit++;}limit(fit,"local-memory");}
 if(host.pressure!==null&&host.pressure!=="normal")limit(1,"local-pressure");
 return Object.freeze(requireDefinition<LocalJobsDecisionV1>("LocalJobsDecision",{jobs,reason}));
}

/** 🧵️ The component finals of `profile` as the scheduler now runs them, on one front-end, one code generation and one wasm-ld thread: an observation that kept more than `serialFinalCpuRatio` CPUs busy ran with a threaded linker, so its peak (measured 4.8 GiB at 6.6 CPUs against 2.0-3.0 GiB serial for the same final) does not forecast the serial run. A final without any serial observation keeps all of its rows.
 * @see ./🧪️tests/🧵️local/🐍️oracle/🐍️.py */
export function serialFinalHistoryV1(history:ReadonlyMap<string,readonly UnitObservationV1[]>,profile:string):ReadonlyMap<string,readonly UnitObservationV1[]>{
 const kept=new Map<string,readonly UnitObservationV1[]>();
 for(const[key,rows]of history){
  const serial=componentFinalKeyV1(key,profile)?rows.filter(row=>row.cpuMs<=row.wallMs*policy.serialFinalCpuRatio):rows;
  kept.set(key,serial.length?serial:rows);
 }
 return kept;
}

/** 📏️ The peak (megabytes) a pass is reserved at as a local job: its component finals are counted at the broker's own prediction from the recorded peaks of that crate (`predictPeakBytesV1`: headroom times the p95 of the last window), a final never recorded at the headroom times the `unmeasuredQuantile` of the recorded finals of the profile (the bound of the admission policy while fewer than `componentColdMinimumRows` exist), so an unseen big final is not mistaken for the median one; the pass holds the largest `units` finals at once (its cargo links up to `units` of them side by side).
 * @see ../../🎟️admission/🟦️.ts */
export function componentPassPeakMegabytesV1(recorded:ReadonlyMap<string,readonly UnitObservationV1[]>,pass:Readonly<{profile:string;packages:readonly Readonly<{name:string}>[]}>,units:number):number{
 const history=serialFinalHistoryV1(recorded,pass.profile),family=componentFinalPeaksV1(history,pass.profile),crates=new Map<string,number[]>();
 for(const[key,rows]of history){
  if(!componentFinalKeyV1(key,pass.profile))continue;
  const identity=key.split("|")[0]??"",crate=identity.slice(identity.indexOf("@")+1);
  crates.set(crate,[...crates.get(crate)??[],...rows.map(row=>row.peakRssBytes)]);
 }
 const unmeasured=family.length<ADMISSION_POLICY_V1.componentColdMinimumRows?ADMISSION_POLICY_V1.componentFinalBytes:Math.max(Math.ceil(ADMISSION_POLICY_V1.headroom*quantileR7V1(family,policy.unmeasuredQuantile)),ADMISSION_POLICY_V1.floorBytes);
 const peaks=pass.packages.map(row=>{const recorded=crates.get(row.name.replaceAll("-","_"));return recorded?.length?predictPeakBytesV1(recorded,{kloc:0,component:family}):unmeasured;}).sort((left,right)=>right-left);
 return Math.max(1,Math.ceil(peaks.slice(0,Math.max(1,units)).reduce((sum,value)=>sum+value,0)/1048576));
}

/** 🖨️ Describes a plan in the given language (English first, German second). */
export function describeComponentJobsV1(language:"en"|"de",values:Readonly<Record<string,string|number>>):string{
 const catalogue=schema.definitions.Messages.const[language],reason=values.reason;
 return fill(catalogue.jobs,{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{}});
}

/** 🖨️ Describes a local job event or plan of local jobs in both languages on one line (English first, German second), the way the driver logs its other bilingual lines. */
export function describeLocalComponentJobsV1(name:"plan"|"waiting"|"started",values:Readonly<Record<string,string|number>>):string{
 return(["en","de"] as const).map(language=>{const catalogue=schema.definitions.Messages.const[language].local,reason=values.reason;return fill(catalogue[name],{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{}});}).join(" / ");
}

/** 🌐️ What one decision to start a pass on a fleet slot reads: the fleet jobs held here (`running`) and the most that may run (`maximum`), the worker slots that fit one more final right now (`slots`, `null` when the fleet does not answer), the free megabytes now (`null` when unreadable), the megabytes the fleet jobs here already charge, the hard floor that stays free, the memory pressure and the footprint of the job that asks. */
export type FleetJobAdmitHostV1=Readonly<{running:number;maximum:number;slots:number|null;availableMegabytes:number|null;reservedMegabytes:number;floorMegabytes:number;pressure:LocalJobPressureV1;footprintMegabytes:number}>;
/** 🎟️ The verdict: `admit`, why, and the megabytes that are free for a new footprint (free now less the footprints held and the hard floor). */
export type FleetJobAdmitDecisionV1=Readonly<{admit:boolean;reason:FleetJobReasonV1;freeMegabytes:number}>;

/** 🌐️ Decides whether one more pass may start on a fleet slot: while the cap is not reached, a worker slot that fits a final is free and the memory pressure is not critical, the pass is charged only its small local footprint against the free memory above the hard floor, so raised local memory pressure and the reservations of local finals never keep a fleet-placed pass away. Unreadable memory does not stop it either: its footprint is a few hundred megabytes, and the host warden acts on a real overshoot.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🌐️fleet/🐍️oracle/🐍️.py */
export function admitFleetComponentJobV1(hostValue:FleetJobAdmitHostV1):FleetJobAdmitDecisionV1{
 const host=requireDefinition<FleetJobAdmitHostV1>("FleetJobAdmitHost",hostValue),free=host.availableMegabytes===null?0:Math.max(-4194304,host.availableMegabytes-host.floorMegabytes-host.reservedMegabytes);
 const decide=(admit:boolean,reason:FleetJobReasonV1)=>Object.freeze(requireDefinition<FleetJobAdmitDecisionV1>("FleetJobAdmitDecision",{admit,reason,freeMegabytes:free}));
 if(host.running>=host.maximum)return decide(false,"fleet-cap");
 if(host.slots===null)return decide(false,"fleet-none");
 if(host.slots<=0)return decide(false,"fleet-busy");
 if(host.pressure==="critical")return decide(false,"fleet-pressure");
 if(host.availableMegabytes!==null&&host.footprintMegabytes>free)return decide(false,"fleet-memory");
 return decide(true,"fleet-fits");
}

/** 🌐️ The authored cap of concurrent component jobs (local and fleet slots together).
 * @see ./🧬️schema/🔣️.json */
export const componentJobsLimitV1:number=policy.componentJobsMaximum;

/** 📏️ The local footprint (megabytes) one fleet job is charged and the hard floor that stays free beside the fleet jobs: authored, measured on the play build (one pass held 14 rustc wrappers of about 32 MB and a cargo of 9 MB, 460 MB).
 * @see ./🧬️schema/🔣️.json */
export const componentFleetChargeV1={footprintMegabytes:policy.fleetPassFootprintMegabytes,floorMegabytes:policy.fleetFloorMegabytes,pollMilliseconds:policy.fleetPollMilliseconds} as const;

/** 🖨️ Describes a fleet slot event or plan in both languages on one line (English first, German second): `waiting` is worded by the kind of slot the pass waits for (`values.waitsFor`).
 * @see ./🧬️schema/🔣️.json */
export function describeFleetComponentJobsV1(name:"plan"|"started"|"waiting",values:Readonly<Record<string,string|number>>):string{
 return(["en","de"] as const).map(language=>{
  const catalogue=schema.definitions.Messages.const[language].fleet,key=name==="waiting"?({local:"waitingLocal",fleet:"waitingFleet",either:"waitingEither"} as const)[values.waitsFor as "local"|"fleet"|"either"]:name;
  const reason=values.reason,localReason=values.localReason;
  return fill(catalogue[key],{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{},...typeof localReason==="string"&&localReason in schema.definitions.Messages.const[language].local.reasons?{lreason:schema.definitions.Messages.const[language].local.reasons[localReason as keyof typeof schema.definitions.Messages.const.en.local.reasons]}:{}});
 }).join(" / ");
}
