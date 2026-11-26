import{createMemoryPortsV1,selectGraphMemoryLanguagesV1}from"../🟨️.mjs";
import{planStageLimitBytesV1}from"../🪜️plan-stage/🟨️.mjs";
import{profileHeapOnceV1}from"./🔭️heap-profile/🟨️.mjs";
import{boundedCacheStatsV1}from"../../🧊️bounded/🟨️.mjs";
import{collectSliceCollectionsV1,registerSliceCollectionsV1}from"../📋️slice-report/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🧽️ The declared numbers of the memory settling: the schema file is the single source. */
export const SETTLE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the memory settling, read from the schema file. */
export const SETTLE_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));
const MEGABYTE=1048576,megabytes=bytes=>Math.round(bytes/MEGABYTE);

/** 💬️ Renders one settling line as one line per language of the environment. */
export function renderSettleMessageV1(key,fields,environment=process.env){
 return selectGraphMemoryLanguagesV1(environment).map(language=>{
  const template=SETTLE_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed settle message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

const quiet=(environment,limitBytes)=>!(limitBytes<Infinity||environment.SEMIO_GRAPH_PHASES==="1");
const runtimeShrink=()=>{try{globalThis.Bun?.shrink?.();}catch{}};
const stderrLine=line=>{try{process.stderr.write(line+"\n");}catch{}};

/**
 * 🧽️ Gives back the memory a plan needed once it is answered: runs `release` (what the caller can drop), collects garbage, asks the runtime to shrink its heap (`Bun.shrink` returns freed pages to the operating system, a collection alone leaves them resident) and logs ONE line per language with the managed and resident megabytes and what the resident size fell by. Only on a machine that has a plan limit or with `SEMIO_GRAPH_PHASES=1` (a serving machine keeps its graph and is not touched) unless `force` is set; answers `undefined` when it did nothing.
 * @param {{release?:()=>unknown;ports?:{sample:()=>number;collect:()=>void};resident?:()=>number;shrink?:()=>void;environment?:Record<string,string|undefined>;report?:(line:string)=>void;limitBytes?:number;force?:boolean}} [options]
 */
export async function settleGraphMemoryV1({release,ports=createMemoryPortsV1(),resident=()=>process.memoryUsage().rss,shrink=runtimeShrink,environment=process.env,report=stderrLine,limitBytes=planStageLimitBytesV1(environment),force=false}={}){
 if(!force&&quiet(environment,limitBytes))return undefined;
 const managedBefore=ports.sample(),residentBefore=resident();
 await release?.();
 ports.collect();shrink();ports.collect();
 const managed=ports.sample(),after=resident();
 const result={managedBeforeMegabytes:megabytes(managedBefore),managedMegabytes:megabytes(managed),residentBeforeMegabytes:megabytes(residentBefore),residentMegabytes:megabytes(after),releasedMegabytes:Math.max(0,megabytes(residentBefore)-megabytes(after))};
 for(const line of renderSettleMessageV1("answered",{managed:result.managedMegabytes,resident:result.residentMegabytes,released:result.releasedMegabytes},environment))report(line);
 return result;
}

/** 🧮️ Every collection a graph read holds across owners (the bounded caches and the collections the parts of the read registered), as `{name, entries, bytes}`. */
function retainedCollectionsV1(){return[...boundedCacheStatsV1(),...collectSliceCollectionsV1().map(row=>({name:row.name,entries:row.entries,bytes:row.estimatedBytes}))];}

let registeredV1=false;
/** 🔌️ Registers once per process the collections of the observation, the string interner and the script closures with the report of retained memory, so the whole read (not only a slice worker) names what a wave leaves behind.
 * @see ../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts the same four sources of the slice worker */
export async function registerRetainedCollectionsV1(){
 if(registeredV1)return;registeredV1=true;
 try{
  const[{observationCollectionsV1},{graphStringInternerV1},{closureCollectionsV1},{rowLogCollectionsV1},{inputCollectionsV1}]=await Promise.all([import("../../👁️observation/🟨️.mjs"),import("../🟨️.mjs"),import("../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),import("../../🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs"),import("../../📥️inputs/🟨️.mjs")]);
  registerSliceCollectionsV1("observation",()=>observationCollectionsV1());
  registerSliceCollectionsV1("interner",()=>{const row=graphStringInternerV1.stats();return[{name:"memory:graphStringInterner",entries:row.entries,bytes:row.characters*2+row.entries*48}];});
  registerSliceCollectionsV1("closure",()=>closureCollectionsV1());
  registerSliceCollectionsV1("rowlogs",()=>rowLogCollectionsV1());
  registerSliceCollectionsV1("inputs",()=>inputCollectionsV1());
 }catch{registeredV1=false;}
}

/** 🔎️ Logs, between two waves of one read, the managed and resident megabytes and the bounded caches that hold the most bytes, so the log names what a wave leaves behind; quiet on a machine without a plan limit. */
export function reportRetainedGraphMemoryV1({ports=createMemoryPortsV1(),resident=()=>process.memoryUsage().rss,environment=process.env,report=stderrLine,limitBytes=planStageLimitBytesV1(environment),stats=retainedCollectionsV1}={}){
 const managed=ports.sample();profileHeapOnceV1({managedMegabytes:megabytes(managed)});
 const caches=stats().filter(row=>row.entries>0).sort((a,b)=>b.bytes-a.bytes||b.entries-a.entries).slice(0,SETTLE_POLICY_V1.censusCaches).map(row=>row.name+" "+row.entries+" entries "+megabytes(row.bytes)+" MB").join(", ")||"-";
 for(const line of renderSettleMessageV1("retained",{managed:megabytes(managed),resident:megabytes(resident()),caches},environment))report(line);
}
