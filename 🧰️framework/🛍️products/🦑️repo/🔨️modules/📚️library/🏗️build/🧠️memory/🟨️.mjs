import{BUILD_HOST_CAPACITY_POLICY_V1}from"../🌡️host-capacity/🎛️control/🟦️.ts";
import{readLiveHostCapacityReservationV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {noticeLanguagesV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import{totalmem,freemem}from"node:os";
import{mkdirSync,readFileSync,renameSync,writeFileSync}from"node:fs";
import{execFileSync}from"node:child_process";
import{dirname}from"node:path";
import{cgroupMemoryHeadroomV1}from"../🧱️job-limits/🐧️cgroup/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {runtimeCollectGarbageV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧊️ Freezes a declaration recursively. */
const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🧠️ The declared limits of the graph memory governor: the schema file is the single source. */
export const GRAPH_MEMORY_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));
const P=GRAPH_MEMORY_POLICY_V1,MEGABYTE=1048576;

/** 🗣️ The bilingual texts of the governor, read from the schema file. */
export const GRAPH_MEMORY_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));

/** 🌐️ The languages of the process: the one the language protocol chooses, else every supported language in declared order (English first, German second); there is no default language. */
export function selectGraphMemoryLanguagesV1(environment){
 return [...noticeLanguagesV1(P.languages,environment)];
}

/** 💬️ Renders one governor message as one line per language of the environment. */
export function renderGraphMemoryMessagesV1(key,fields,environment){
 return selectGraphMemoryLanguagesV1(environment).map(language=>{
  const template=GRAPH_MEMORY_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed graph memory message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/** 🖥️ The memory of the host in megabytes as this process may use it (the container limit when one applies), 0 when unknown. */
export function hostMemoryMegabytesV1(){
 const total=totalmem(),limit=typeof process.constrainedMemory==="function"?process.constrainedMemory():0;
 const bytes=Number.isFinite(limit)&&limit>0&&limit<total?limit:total;
 return Number.isFinite(bytes)&&bytes>0?Math.floor(bytes/MEGABYTE):0;
}

/** 🎫️ Partitions the qualified live reservation between the graph parent, fixed overhead and owned workers; an enlarged carried tool envelope cannot fund additional memory. An explicit parent budget leaves one minimum owned worker when the reservation can pay for that worker beside the parent, so a scaled-down grant still infers outside the parent. */
export function graphMemoryAllocationV1(environment,policy=P,hostMegabytes=hostMemoryMegabytesV1()){
 const reservation=readLiveHostCapacityReservationV1(environment,BUILD_HOST_CAPACITY_POLICY_V1);
 const carried=environment[policy.planEnvironment]!==undefined&&environment[policy.planEnvironment]!=="",part=value=>typeof value==="number"&&Number.isFinite(value)&&value>0?value:0;
 if(reservation!==undefined&&reservation.reservedMegabytes===0)throw Error("Active graph parent memory reservation required");
 const derived=reservation!==undefined?Math.floor(reservation.reservedMegabytes*policy.grantShare):carried?policy.minimumBudgetMegabytes:Math.floor(part(hostMegabytes)*policy.hostShare);
 let parent=Math.min(reservation?.reservedMegabytes??Infinity,Math.max(policy.minimumBudgetMegabytes,Math.min(policy.maximumBudgetMegabytes,derived))),text=environment[policy.budgetEnvironment];
 if(text!==undefined){const explicit=Number(text);if(!Number.isFinite(explicit)||explicit<policy.minimumBudgetMegabytes)throw Error("Graph memory budget of at least "+policy.minimumBudgetMegabytes+" MB required: "+policy.budgetEnvironment);parent=Math.min(Math.floor(explicit),parent);}
 const plan=JSON.parse(environment[policy.planEnvironment]||"null");
 const total=reservation?.reservedMegabytes??0,room=Math.max(0,total-parent),explicitParent=text!==undefined,parentFloor=256;
 let fixed=reservation===undefined?0:Math.min(Math.floor(part(plan?.fixedMegabytes)),room);
 if(explicitParent&&room>=policy.minimumBudgetMegabytes)fixed=Math.min(fixed,room-policy.minimumBudgetMegabytes);
 let worker=Math.max(0,room-fixed);
 if(explicitParent&&worker<policy.minimumBudgetMegabytes&&total-policy.minimumBudgetMegabytes>=parentFloor){parent=total-policy.minimumBudgetMegabytes;fixed=0;worker=policy.minimumBudgetMegabytes;}
 return Object.freeze({qualified:reservation!==undefined,reservedMegabytes:total,parentMegabytes:parent,fixedMegabytes:fixed,workerMegabytes:worker,jobs:reservation?.jobs??0});
}
/** 🧒️ The memory JavaScriptCore is told the graph child process may use (`BUN_JSC_forceRAMSize`), which sets how far its heap grows before it collects: the declared child size, never the capacity grant of the run, because the grant pays for every process of the run and a larger size only lets the child keep more garbage (36 refreshed projects in one process: 2 GiB peaks at 5.5 GB resident and 2.0 GB footprint, 8 GiB at 7.5 GB and 2.8 GB, with identical rows and wall time).
 * @see ./🧬️schema/🔣️.json */
export function graphChildHeapBytesV1(policy=P){return policy.childHeapMegabytes*MEGABYTE;}

/** 🛰️ The memory JavaScriptCore is told the host graph service may keep (`BUN_JSC_forceRAMSize`): the service holds the resident graph and plans in process, so its collector target is larger than a one-shot child's but still declared, never the host's RAM.
 * @see ./🧬️schema/🔣️.json */
export function graphServiceHeapBytesV1(policy=P){return policy.serviceHeapMegabytes*MEGABYTE;}

/** 🔌️ The charged resident bytes of the process and the collection that releases them; both are ports so every platform and every test supplies its own. The sample reads the runtime counters (constant time): a heap walk costs time proportional to the heap and ran every few milliseconds. */
/** 📏️ Charges the managed bytes (heap in use plus external buffers), never the resident pages: with a small collector target the pages hold freed memory the allocator has not returned yet, which no admission can release; the host free-memory floor still guards the machine. */
export function graphMemoryPressureBytesV1(counters){if(![counters.rss,counters.heapUsed,counters.external].every(value=>Number.isSafeInteger(value)&&value>=0))throw Error("Actual graph parent memory counters required");return counters.heapUsed+counters.external;}
export function createMemoryPortsV1(){
 return Object.freeze({
  sample:()=>graphMemoryPressureBytesV1(process.memoryUsage()),
  collect:()=>{runtimeCollectGarbageV1(true);},
 });
}

/** 🍎️ The memory a macOS host can give out in bytes from the text of `vm_stat`: the free and the inactive pages (the file cache the kernel returns on demand); `os.freemem()` counts the free pages alone, which on a host that runs for days is a few megabytes whatever is reclaimable, so a governor reading it would see no headroom ever. Undefined for a text that names no page size, free or inactive count. */
export function darwinAvailableBytesV1(text){
 const size=/page size of (\d+) bytes/u.exec(text),pages=name=>{const found=new RegExp("^"+name+":\\s+(\\d+)\\.","mu").exec(text);return found?Number(found[1]):undefined;},free=pages("Pages free"),inactive=pages("Pages inactive");
 return size&&free!==undefined&&inactive!==undefined?(free+inactive)*Number(size[1]):undefined;
}

/** 🐧️ The memory a Linux host can give out in bytes from the text of `/proc/meminfo` (`MemAvailable`), undefined where the kernel does not report it. */
export function linuxAvailableBytesV1(text){
 const found=/^MemAvailable:\s+(\d+)\s+kB/mu.exec(text);
 return found?Number(found[1])*1024:undefined;
}

/** 🌡️ The reader of the memory the host can still give out: `MemAvailable` on Linux, free plus inactive pages on macOS (`vm_stat`), the operating system's own free memory elsewhere and wherever the platform source cannot be read; an answer is kept for `maximumAgeMilliseconds` so a governor that samples every few hundred milliseconds does not start a process per sample. The sources are ports. */
export function createAvailableMemoryV1({platform=process.platform,read=path=>readFileSync(path,"utf8"),run=()=>execFileSync("vm_stat",{encoding:"utf8",timeout:2000}),free=freemem,now=()=>performance.now(),maximumAgeMilliseconds=1000}={}){
 let at=-Infinity,value=0;
 const source=()=>{try{if(platform==="linux")return linuxAvailableBytesV1(read("/proc/meminfo"));if(platform==="darwin")return darwinAvailableBytesV1(run());}catch{}return undefined;};
 return()=>{const time=now();if(time-at>=maximumAgeMilliseconds||at===-Infinity){const found=source();value=Number.isFinite(found)&&found>=0?found:free();at=time;}return value;};
}

/** 🌡️ The reader of this host. */
export const availableMemoryBytesV1=createAvailableMemoryV1();

/** 📏️ The bytes the service process and its pool may still take: where the environment declares a memory limit (`SEMIO_GRAPH_MEMORY_LIMIT_MB`, a private harness or a platform without cgroups) that limit less the resident memory of this process, otherwise the headroom of the cgroup they share (`memory.max` of the smallest enclosing limit minus `memory.current`) less the safety margin, a declared share of the limit; where no cgroup is readable the host answers with its total and free memory under the same margin. */
export function serviceMemoryHeadroomBytesV1({ports,policy=P,cgroup=cgroupMemoryHeadroomV1,free=availableMemoryBytesV1,total=totalmem,environment=process.env,usage=()=>process.memoryUsage().rss}={}){
 const declared=Number(environment[policy.limitEnvironment]);
 if(Number.isFinite(declared)&&declared>0){const limit=declared*MEGABYTE;return Math.floor(limit-usage()-limit*policy.safetyMarginBasisPoints/10000);}
 const reading=cgroup(ports),limit=reading!==undefined&&Number.isFinite(reading.limitBytes)?reading.limitBytes:total(),current=reading!==undefined?reading.currentBytes:total()-free();
 return Math.floor(limit-current-limit*policy.safetyMarginBasisPoints/10000);
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/** 💾️ The durable learned owner peaks in one JSON file (atomic replace, a missing or damaged file reads as nothing learned). */
export function createGraphOwnerPeaksFileV1(path){
 return Object.freeze({
  read:()=>{try{const value=JSON.parse(readFileSync(path,"utf8"));return value!==null&&typeof value==="object"&&!Array.isArray(value)?value:{};}catch{return{};}},
  write:peaks=>{try{mkdirSync(dirname(path),{recursive:true});const temporary=path+"."+process.pid+".tmp";writeFileSync(temporary,JSON.stringify(peaks));renameSync(temporary,path);}catch{}},
 });
}

/**
 * 🧮️ The admission law of one more owner: the first owner always runs, an owner of a kind without a learned peak runs alone, otherwise the measured headroom (usage and safety margin already taken off), less what every in-flight owner is still expected to take (its estimate, never below what it has taken so far, less what it has taken), must cover the estimate of the new owner.
 * @param {{headroom:number;running:number;inflight:readonly {estimate:number;taken:number}[];estimate:number;learned:boolean}} state
 */
export function graphOwnerAdmissionV1({headroom,running,inflight,estimate,learned}){
 if(running===0)return true;
 if(!learned)return false;
 const remaining=inflight.reduce((sum,owner)=>sum+Math.max(0,owner.estimate-owner.taken),0);
 return headroom-remaining>=estimate;
}

/**
 * 🚦️ Admits project inferences against the headroom the service has left, soundly before any cost is learned: the estimate of an in-flight owner is the larger of the learned peak of its kind (else the declared guess) and the growth observed since it started (the measured growth of the headroom split between the running owners in proportion to what each is still expected to take); one more owner starts only when the measured headroom less the remaining estimates covers its own estimate; an owner of a kind without a learned peak runs alone until one of that kind completes; the peak of a kind is learned when an owner completes and persists through the `peaks` port; one owner always runs, so nothing deadlocks.
 * @param {{workers:number;headroom:()=>number;estimateBytes?:number;decide?:(decision:{key:string;used:number;budget:number;kind?:string})=>void;peaks?:{read:()=>Record<string,number>;write:(peaks:Record<string,number>)=>void};sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Record<string,string|undefined>;policy?:typeof GRAPH_MEMORY_POLICY_V1}} options
 */
export function createMemoryGovernorV1({workers,headroom,estimateBytes=P.initialOwnerMegabytes*MEGABYTE,peaks,sleep=defaultSleep,now=()=>performance.now(),signal,report,decide,environment={},policy=P}){
 if(!Number.isSafeInteger(workers)||workers<1||typeof headroom!=="function"||!(estimateBytes>0))throw Error("Positive worker count, headroom reading and owner estimate required");
 const queue=[],owners=new Set(),stats={waits:0,waitedMilliseconds:0,peakOwners:0},learned=new Map(Object.entries(peaks?.read()??{}).filter(([,bytes])=>Number.isFinite(bytes)&&bytes>=0));
 let last=headroom(),timer,pumping=false,serialSaid=false,lastReport=-Infinity,lastAdmit=-Infinity;
  const say=(key,fields)=>{decide?.({key,used:fields.used,budget:fields.budget,kind:fields.kind});for(const line of renderGraphMemoryMessagesV1(key,{running:owners.size,kind:"",...fields},environment))report?.(line);};
 const megabytes=bytes=>Math.round(bytes/MEGABYTE),peakOf=kind=>learned.has(kind)?learned.get(kind):estimateBytes;
 const sample=()=>{
  const free=headroom();
  if(owners.size===0){last=free;return free;}
  const delta=last-free;last=free;
  const room=[...owners].map(owner=>Math.max(0,peakOf(owner.kind)-owner.taken)),open=room.reduce((sum,value)=>sum+value,0);
  [...owners].forEach((owner,index)=>{owner.net=Math.max(0,owner.net+(delta>0&&open>0?delta*room[index]/open:delta/owners.size));owner.taken=Math.max(owner.taken,owner.net);});
  return free;
 };
 const inflight=()=>[...owners].map(owner=>({estimate:peakOf(owner.kind),taken:owner.taken}));
 const fail=error=>{for(const waiter of queue.splice(0))waiter.reject(error);};
 const pump=async()=>{
  if(pumping)return;pumping=true;
  try{
   while(queue.length&&owners.size<workers){
    signal?.throwIfAborted();
    const spacing=lastAdmit+policy.admitMinimumMilliseconds-now();
    if(owners.size>0&&spacing>0){await sleep(spacing,signal);continue;}
    const free=sample(),next=queue[0],known=learned.has(next.kind),estimate=peakOf(next.kind),state={headroom:free,running:owners.size,inflight:inflight(),estimate,learned:known};
    if(graphOwnerAdmissionV1(state)){
     if(owners.size===0&&free<estimate){if(!serialSaid){serialSaid=true;say("serial",{used:megabytes(estimate),budget:megabytes(free)});}}
     else if(serialSaid){serialSaid=false;say("resumed",{used:megabytes(estimate),budget:megabytes(free)});}
     queue.shift();const owner={kind:next.kind,taken:0,net:0};owners.add(owner);next.owner=owner;stats.peakOwners=Math.max(stats.peakOwners,owners.size);lastAdmit=now();
     if(next.waited){stats.waits++;stats.waitedMilliseconds+=now()-next.since;}
     next.resolve(owner);continue;
    }
    for(const waiter of queue)waiter.waited=true;
    if(now()-lastReport>=policy.progressMilliseconds){
     lastReport=now();
     if(!known)say("learning",{kind:next.kind,used:megabytes(estimate),budget:megabytes(free)});
     else say("waiting",{used:megabytes(state.inflight.reduce((sum,row)=>sum+Math.max(row.estimate,row.taken),0)),budget:megabytes(free+state.inflight.reduce((sum,row)=>sum+row.taken,0))});
    }
    await sleep(policy.pollMilliseconds,signal);
   }
  }catch(error){fail(error);}
  finally{pumping=false;}
 };
 const onAbort=()=>fail(signal.reason);
 signal?.addEventListener("abort",onAbort,{once:true});
 return Object.freeze({
  async run(task,{kind="owner"}={}){
   signal?.throwIfAborted();
   const owner=await new Promise((resolve,reject)=>{queue.push({kind,resolve,reject,since:now(),waited:false});pump();});
   if(timer===undefined){timer=setInterval(sample,policy.pollMilliseconds);timer.unref?.();}
   try{return await task();}
   finally{
    sample();
    const known=learned.get(owner.kind);
    if(known===undefined||owner.taken>known){learned.set(owner.kind,Math.max(owner.taken,known??0));peaks?.write(Object.fromEntries(learned));}
    owners.delete(owner);if(owners.size===0&&timer!==undefined){clearInterval(timer);timer=undefined;}if(queue.length)pump();
   }
  },
  pressure:()=>headroom()<Math.max(estimateBytes*(learned.size===0?1:0),...learned.values()),
  stats:()=>({waits:stats.waits,waitedMilliseconds:Math.round(stats.waitedMilliseconds),peakOwners:stats.peakOwners,ownerCostMegabytes:megabytes(Math.max(estimateBytes*(learned.size===0?1:0),...learned.values())),kinds:Object.fromEntries([...learned].map(([kind,bytes])=>[kind,megabytes(bytes)]))}),
  dispose:()=>{signal?.removeEventListener("abort",onAbort);clearInterval(timer);timer=undefined;},
 });
}

/** 🛂️ The governor of the project inference of the service: the headroom is the cgroup headroom of the service (see {@link serviceMemoryHeadroomBytesV1}), shared by the process and its pool. */
export function createGraphParentMemoryGovernorV1(options){return createMemoryGovernorV1({...options,headroom:options.headroom??(()=>serviceMemoryHeadroomBytesV1({policy:options.policy??P}))});}

/**
 * 🧶️ Creates a bounded table that makes equal long strings one instance: equal paths, input lists and keys of thousands of projects cost memory once. A table over its character bound is cleared, never grown.
 * @param {{maximumCharacters?:number;minimumLength?:number}} [options]
 */
export function createStringInternerV1({maximumCharacters=P.maximumInternedCharacters,minimumLength=P.minimumInternedLength}={}){
 const table=new Map();let characters=0,hits=0,clears=0;
 return Object.freeze({
  share(text){
   if(text.length<minimumLength)return text;
   const known=table.get(text);if(known!==undefined){hits++;return known;}
   if(characters+text.length>maximumCharacters){table.clear();characters=0;clears++;}
   table.set(text,text);characters+=text.length;return text;
  },
  stats:()=>({entries:table.size,characters,hits,clears}),
  clear(){table.clear();characters=0;},
 });
}

/**
 * 🔤️ Replaces every long string value of a freshly parsed JSON value by the shared instance of `interner`, in place and without touching object keys, array lengths or any non-string value, so the value serializes to the identical bytes.
 * @template T
 * @param {T} value
 * @param {ReturnType<typeof createStringInternerV1>} interner
 * @returns {T}
 */
export function internJsonStringsV1(value,interner){
 const pending=[value];
 while(pending.length){
  const item=pending.pop();
  if(Array.isArray(item)){
   for(let index=0;index<item.length;index++){const child=item[index];if(typeof child==="string")item[index]=interner.share(child);else if(child!==null&&typeof child==="object")pending.push(child);}
  }else if(item!==null&&typeof item==="object"){
   for(const key of Object.keys(item)){const child=item[key];if(typeof child==="string")item[key]=interner.share(child);else if(child!==null&&typeof child==="object")pending.push(child);}
  }
 }
 return value;
}

/** 🌍️ The process wide table that shares strings between inference results, stored fragments and expanded input lists. */
export const graphStringInternerV1=createStringInternerV1();
