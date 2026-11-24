import {validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{NativeBuildPassV1}from"../📋️contract/🟦️.ts";
import{ADMISSION_POLICY_V1,componentFinalKeyV1,componentFinalPeaksV1,predictPeakBytesV1,quantileR7V1,type UnitObservationV1}from"../../🎟️admission/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🎫️ What a running pass holds here: the `local` slot (a jobserver token, at most one at a time) or a `light` one (the cargo process and the input shipping only, its units compile on workers). */
export type PassJobSlotV1="local"|"light";

/** 🛰️ Whether everything a pass still does after its inputs are shipped can run on a worker: its target is host independent, so its units, its final cdylib among them, are offloadable. Any other pass keeps the strictly sequential local behaviour.
 * @see ./🧬️schema/🔣️.json (`Policy.offloadableTargetPrefixes`) */
export const nativeBuildPassOffloadableV1=(pass:Pick<NativeBuildPassV1,"target">):boolean=>policy.offloadableTargetPrefixes.some(prefix=>pass.target.startsWith(prefix));

/** 🧵️ Why a local component job may or may not start beside the running ones (also why a plan of local jobs is as large as it is): the first job always starts, the reservation fits, every listed final fits, the authored cap or the local grant is reached, the free memory above the floor is too small or cannot be read, or memory pressure is raised. */
export type LocalJobReasonV1="local-first"|"local-fits"|"local-all"|"local-cap"|"local-grant"|"local-memory"|"local-pressure"|"local-unknown";
/** 🌡️ The memory pressure level of the machine as the capacity probe reports it. */
export type LocalJobPressureV1="normal"|"warn"|"critical"|null;
/** 📸️ One memory sample of this machine: the free megabytes (`null` when unreadable), the megabytes the compilers of this build already hold of them (`ownedMegabytes`, added back because the reservations are charged at their whole peak) and the pressure level. */
export type LocalJobSampleV1=Readonly<{availableMegabytes:number|null;ownedMegabytes:number;pressure:LocalJobPressureV1}>;
/** 📣️ What the runner tells about a local job beyond the first: it waits for memory (repeated as progress) or it starts and reserves its peak. */
export type LocalJobEventV1=Readonly<{kind:"waiting"|"started";running:number;peakMegabytes:number;freeMegabytes:number;floorMegabytes:number;reason:LocalJobReasonV1;waitedMilliseconds:number}>;
/** 🏠️ How a lane runs component jobs on THIS machine beside the first one: at most `maximum` local jobs at once, each holding a reservation of its predicted `peak` (megabytes) that is charged against the memory `sample` reads before every start, above `floorMegabytes` that stays free. */
export type LocalJobsOptionsV1<T>=Readonly<{maximum:number;floorMegabytes:number;peak:(pass:T)=>number|Promise<number>;sample:()=>Promise<LocalJobSampleV1>;report?:(pass:T,event:LocalJobEventV1)=>void;pollMilliseconds?:number;progressMilliseconds?:number;now?:()=>number}>;

const pause=(milliseconds:number,wake:Iterable<Promise<void>>,signal?:AbortSignal):Promise<void>=>new Promise<void>(resolve=>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 const done=()=>{if(timer!==undefined)clearTimeout(timer);signal?.removeEventListener("abort",done);resolve();};
 timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",done,{once:true});
 if(signal?.aborted)done();
 for(const task of wake)void task.then(done,done);
});

/** 🧵️ Runs the passes of one lane in order with up to `jobs` offloadable passes at once. The first running pass holds the local slot. Without `local` every other pass holds a light one (the strictly sequential behaviour of a build without a fleet when `jobs` is one). With `local` up to `local.maximum` passes hold a local slot: each one beyond the first holds a real reservation of its predicted peak, starts only while the free memory above the floor carries it beside the running reservations, otherwise waits (progress reported, ended by a settling job, the poll, `check` or `signal`) and re-reads the memory before each start, so a running build picks up memory that was freed; the light slots are the rest of `jobs`. A pass that is not offloadable waits for every running pass and runs alone. The first failure stops launching, is handed to `fail` at once (the driver ends the sibling cargo children) and is thrown after every running pass settled.
 * @see ./🧪️tests/🟦️.ts */
export async function runPassesInJobSlotsV1<T extends Pick<NativeBuildPassV1,"target">>(passes:readonly T[],options:Readonly<{jobs:number;check:()=>void;run:(pass:T,slot:PassJobSlotV1)=>Promise<void>;fail?:(error:unknown)=>void;offloadable?:(pass:T)=>boolean;local?:LocalJobsOptionsV1<T>;signal?:AbortSignal}>):Promise<void>{
 const running=new Set<Promise<void>>(),reservations=new Map<Promise<void>,number>(),offloadable=options.offloadable??nativeBuildPassOffloadableV1,local=options.local,localMaximum=Math.max(1,local?.maximum??1),lightMaximum=Math.max(0,options.jobs-localMaximum),now=local?.now??Date.now;
 let failure:{error:unknown}|undefined,localHeld=0,lightHeld=0;
 const reserved=()=>{let sum=0;for(const value of reservations.values())sum+=value;return sum;};
 const launch=(pass:T,slot:PassJobSlotV1,peak:number):Promise<void>=>{
  if(slot==="local")localHeld++;else lightHeld++;
  const task:Promise<void>=(async()=>options.run(pass,slot))().catch(error=>{if(!failure){failure={error};options.fail?.(error);}}).finally(()=>{running.delete(task);reservations.delete(task);if(slot==="local")localHeld--;else lightHeld--;});
  running.add(task);if(slot==="local")reservations.set(task,peak);return task;
 };
 try{for(const pass of passes){
  if(failure)break;
  options.check();
  if(options.jobs<=1||!offloadable(pass)){while(running.size)await Promise.race(running);if(failure)break;await launch(pass,"local",0);continue;}
  let peak:number|undefined,waitedSince:number|undefined,reportedAt=-Infinity;
  while(!failure){
   if(waitedSince!==undefined)options.check();
   let slot:PassJobSlotV1|undefined,verdict:LocalJobAdmitDecisionV1|undefined;
   if(running.size<options.jobs){
    if(localHeld===0)slot="local";
    else if(lightHeld<lightMaximum)slot="light";
    else if(local&&localHeld<localMaximum){
     peak??=await local.peak(pass);
     const sample=await local.sample(),held=reserved();
     verdict=admitLocalComponentJobV1({running:localHeld,maximum:localMaximum,availableMegabytes:sample.availableMegabytes,ownedMegabytes:Math.min(sample.ownedMegabytes,held),reservedMegabytes:held,floorMegabytes:local.floorMegabytes,pressure:sample.pressure,peakMegabytes:peak});
     if(verdict.admit)slot="local";
    }
   }
   if(failure)break;
   if(slot){
    if(slot==="local"&&local&&peak===undefined)peak=await local.peak(pass);
    if(failure)break;
    const extra=slot==="local"&&localHeld>0;
    if(extra&&local&&verdict)local.report?.(pass,{kind:"started",running:localHeld,peakMegabytes:peak!,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:waitedSince===undefined?0:now()-waitedSince});
    launch(pass,slot,peak??0);
    break;
   }
   if(verdict&&local){
    const at=now();waitedSince??=at;
    if(at-reportedAt>=(local.progressMilliseconds??policy.localJobsProgressMilliseconds)){reportedAt=at;local.report?.(pass,{kind:"waiting",running:localHeld,peakMegabytes:peak!,freeMegabytes:verdict.freeMegabytes,floorMegabytes:local.floorMegabytes,reason:verdict.reason,waitedMilliseconds:at-waitedSince});}
    await pause(local.pollMilliseconds??policy.localJobsPollMilliseconds,running,options.signal);
   }else await Promise.race(running);
  }
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
function requireDefinition<T>(definition:"ComponentJobsHost"|"ComponentJobsDecision"|"LocalJobsHost"|"LocalJobsDecision"|"LocalJobAdmitHost"|"LocalJobAdmitDecision"|"ComponentJobSlotsHost"|"ComponentJobSlotsDecision",value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:errors[0]!}));
 return value as T;
}

/** 🧵️ Sizes the concurrent component jobs of one build: the local grant plus the worker slots that can take a wasm final (`fleetSlots`, counted at the predicted peak of the final), bounded by the authored cap (`componentJobsMaximum`) and by the memory the local residue of an offloaded job (`componentJobResidueMegabytes`: the cargo process, its fingerprint checks and the input shipping) may hold beside the local compiles; none under critical and half under raised memory pressure. Without a fleet answer the plan is the local grant, so a build with the fleet down behaves as before.
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
 else if(host.pressure==="warn")limit(Math.floor(remote/policy.pressureWarnDivisor),"jobs-pressure");
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

/** 📏️ The peak (megabytes) a pass is reserved at as a local job: its component finals are counted at the broker's own prediction from the recorded peaks of that crate (`predictPeakBytesV1`: headroom times the p95 of the last window), a final never recorded at the headroom times the `unmeasuredQuantile` of the recorded finals of the profile (the bound of the admission policy while fewer than `componentColdMinimumRows` exist), so an unseen big final is not mistaken for the median one; the pass holds the largest `units` finals at once (its cargo links up to `units` of them side by side).
 * @see ../../🎟️admission/🟦️.ts */
export function componentPassPeakMegabytesV1(history:ReadonlyMap<string,readonly UnitObservationV1[]>,pass:Readonly<{profile:string;packages:readonly Readonly<{name:string}>[]}>,units:number):number{
 const family=componentFinalPeaksV1(history,pass.profile),crates=new Map<string,number[]>();
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
