import{totalmem}from"node:os";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {runtimeCollectGarbageV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧊️ Freezes a declaration recursively. */
const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🧠️ The declared limits of the graph memory governor: the schema file is the single source. */
export const GRAPH_MEMORY_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));
const P=GRAPH_MEMORY_POLICY_V1,MEGABYTE=1048576;

/** 🗣️ The bilingual texts of the governor, read from the schema file. */
export const GRAPH_MEMORY_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));

/** 🌐️ The languages of the process: the one the environment names, else every supported language in declared order (English first, German second); there is no default language. */
export function selectGraphMemoryLanguagesV1(environment){
 const named=P.languageEnvironment.map(name=>environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value=>P.languages.includes(value));
 return named===undefined?[...P.languages]:[named];
}

/** 🗣️ Renders one governor message as one line per language of the environment. */
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

/** 📏️ The managed memory budget of one graph construction in bytes: the explicit environment value, else derived from the machine: the declared share of the host capacity grant the process inherited (fixed plus per job megabytes) when it has one, else the declared share of the host memory, within the declared minimum and maximum: a process never plans beyond what the machine admitted it for, because the garbage a bigger budget tolerates is resident (and compressed) memory of every other command. Nothing is a constant of one machine. */
export function graphMemoryBudgetBytesV1(environment,policy=P,hostMegabytes=hostMemoryMegabytesV1()){
 const text=environment[policy.budgetEnvironment];
 if(text!==undefined){
  const explicit=Number(text);
  if(!Number.isFinite(explicit)||explicit<policy.minimumBudgetMegabytes)throw Error("Graph memory budget of at least "+policy.minimumBudgetMegabytes+" MB required: "+policy.budgetEnvironment);
  return Math.floor(explicit)*MEGABYTE;
 }
 let plan;try{plan=JSON.parse(environment[policy.planEnvironment]??"null");}catch{}
 const part=value=>typeof value==="number"&&Number.isFinite(value)&&value>0?value:0,granted=part(plan?.memoryMegabytes)+part(plan?.fixedMegabytes);
 const derived=granted>0?Math.floor(granted*policy.grantShare):Math.floor(part(hostMegabytes)*policy.hostShare);
 return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.maximumBudgetMegabytes,derived))*MEGABYTE;
}

/**
 * ⚖️ Decides whether the next project may start: never beyond the worker count, otherwise while the managed memory is within the budget (within `resumeRatio` of it after it was exceeded, so admission does not flap).
 * Memory above the budget is answered with one collection (the caller collects and decides again with `collected` set); what a collection cannot release is retained data, which degrades the run to one project at a time (`forced` while nothing runs) and never deadlocks.
 * @param {typeof GRAPH_MEMORY_POLICY_V1} policy
 * @param {{used:number;budget:number;running:number;workers:number;blocked:boolean;collected:boolean}} state
 * @returns {{admit:boolean;collect:boolean;reason:"workers"|"within"|"collect"|"over"|"forced"}}
 */
export function decideMemoryAdmissionV1(policy,{used,budget,running,workers,blocked,collected}){
 if(running>=workers)return{admit:false,collect:false,reason:"workers"};
 if(used<=(blocked?budget*policy.resumeRatio:budget))return{admit:true,collect:false,reason:"within"};
 if(!collected)return{admit:false,collect:true,reason:"collect"};
 return running===0?{admit:true,collect:false,reason:"forced"}:{admit:false,collect:false,reason:"over"};
}

/** 🔌️ The live managed bytes of the JavaScript heap and the collection that releases them; both are ports so every platform and every test supplies its own. The sample reads the runtime counters (constant time): a heap walk costs time proportional to the heap and ran every few milliseconds. */
export function createMemoryPortsV1(){
 return Object.freeze({
  sample:()=>{const memory=process.memoryUsage();return memory.heapUsed+memory.external;},
  collect:()=>{runtimeCollectGarbageV1(true);},
 });
}

/** 📐️ The managed bytes that stay live after one collection: the fixed working set of the process (discovery results, parsers, caches) that no admission can influence, so a governor charges only what grows beyond it. */
export function measureMemoryBaselineV1(ports=createMemoryPortsV1()){
 ports.collect();return ports.sample();
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🚦️ Gates the start of concurrent project inferences by the managed memory above `baselineBytes` (the fixed working set that no admission can influence, see {@link measureMemoryBaselineV1}): at most `workers` run, a new one starts only while the heap is within `budgetBytes` (after one rate limited collection), waiters start in arrival order, progress is reported at most every `progressMilliseconds` and an abort rejects every waiter.
 * @param {{budgetBytes:number;workers:number;baselineBytes?:number;sample?:()=>number;collect?:()=>void;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;signal?:AbortSignal;report?:(line:string)=>void;environment?:Record<string,string|undefined>;policy?:typeof GRAPH_MEMORY_POLICY_V1}} options
 */
export function createMemoryGovernorV1({budgetBytes,workers,baselineBytes=0,sample,collect,sleep=defaultSleep,now=()=>performance.now(),signal,report,environment={},policy=P}){
 if(!Number.isSafeInteger(workers)||workers<1||!(budgetBytes>0)||!(baselineBytes>=0))throw Error("Positive worker count and memory budget required");
 const defaults=createMemoryPortsV1(),read=sample??defaults.sample,collectNow=collect??defaults.collect;
 const queue=[],stats={waits:0,waitedMilliseconds:0,collections:0,peakBytes:0};
 let running=0,blocked=false,pumping=false,serialSaid=false,retained=false,lastCollect=-Infinity,lastReport=-Infinity,lastSampleAt=-Infinity,lastAdmit=-Infinity,lastRelease=0,lastUsed=0;
 const say=(key,fields)=>{for(const line of renderGraphMemoryMessagesV1(key,{budget:Math.round(budgetBytes/MEGABYTE),running,...fields},environment))report?.(line);};
 const observe=fresh=>{const at=now();if(fresh||at-lastSampleAt>=policy.sampleMilliseconds){lastUsed=Math.max(0,read()-baselineBytes);lastSampleAt=at;if(lastUsed>stats.peakBytes)stats.peakBytes=lastUsed;}return lastUsed;};
 const settled=()=>lastCollect>=lastRelease||retained&&now()-lastCollect<policy.retainedMilliseconds;
 const mayCollect=()=>now()-lastCollect>=policy.collectMinimumMilliseconds&&!settled();
 const fail=error=>{for(const waiter of queue.splice(0))waiter.reject(error);};
 const pump=async()=>{
  if(pumping)return;pumping=true;
  try{
   while(queue.length){
    signal?.throwIfAborted();
    const spacing=lastAdmit+policy.admitMinimumMilliseconds-now();
    if(running>0&&spacing>0){await sleep(spacing,signal);continue;}
    let used=observe(false),verdict=decideMemoryAdmissionV1(policy,{used,budget:budgetBytes,running,workers,blocked,collected:false});
    if(verdict.collect){
     if(mayCollect()){collectNow();lastCollect=now();stats.collections++;retained=running===0&&observe(true)>budgetBytes;}
     used=observe(true);verdict=decideMemoryAdmissionV1(policy,{used,budget:budgetBytes,running,workers,blocked,collected:settled()});
     if(verdict.collect)verdict={admit:false,collect:false,reason:"over"};
    }
    if(verdict.admit){
     if(verdict.reason==="within"){if(blocked){blocked=false;say("resumed",{used:Math.round(used/MEGABYTE)});}serialSaid=false;}
     else{blocked=true;if(!serialSaid){serialSaid=true;say("serial",{used:Math.round(used/MEGABYTE)});}}
     const waiter=queue.shift();running++;lastAdmit=now();
     if(waiter.waited){stats.waits++;stats.waitedMilliseconds+=now()-waiter.since;}
     waiter.resolve();
     continue;
    }
    if(verdict.reason==="workers")return;
    blocked=true;for(const waiter of queue)waiter.waited=true;
    if(now()-lastReport>=policy.progressMilliseconds){lastReport=now();say("waiting",{used:Math.round(used/MEGABYTE)});}
    await sleep(policy.pollMilliseconds,signal);
   }
  }catch(error){fail(error);}
  finally{pumping=false;}
 };
 signal?.addEventListener("abort",()=>fail(signal.reason),{once:true});
 return Object.freeze({
  async run(task){
   signal?.throwIfAborted();
   await new Promise((resolve,reject)=>{queue.push({resolve,reject,since:now(),waited:false});pump();});
   try{return await task();}
   finally{running--;lastRelease=now();if(queue.length)pump();}
  },
  stats:()=>({waits:stats.waits,waitedMilliseconds:Math.round(stats.waitedMilliseconds),collections:stats.collections,peakMegabytes:Math.round(stats.peakBytes/MEGABYTE)}),
 });
}

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
 * 🧶️ Replaces every long string value of a freshly parsed JSON value by the shared instance of `interner`, in place and without touching object keys, array lengths or any non-string value, so the value serializes to the identical bytes.
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
