import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{execFile}from"node:child_process";
import{mkdirSync}from"node:fs";
import{basename}from"node:path";
import{platform as osPlatform}from"node:os";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{writeSliceReportTextV1}from"./🏷️owner/🟦️.ts";

/** 📋️ The declared numbers of the slice memory report: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const SLICE_REPORT_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
const STEPS=schema.definitions.Steps.const,steps=[];let movedAt=performance.now(),movedSign="";
const P=SLICE_REPORT_POLICY_V1,MEGABYTE=1048576,megabytes=bytes=>Math.round(bytes/MEGABYTE*10)/10,sources=new Map(),finals=new Map(),startedAt=performance.now();
let phase={name:"start",done:0,total:0},stage,stop,measured={};
const collectionSources=new Map();

/** 🧮️ Describes one collection a part of the read holds across owners for the report: a Map, Set or array by its entries, a plain object by its keys; `sample()` answers the entries the size is estimated from. */
export function describeCollectionV1(name,collection){
 if(collection instanceof Map)return{name,entries:collection.size,sample:()=>take(collection.entries(),P.collectionSample)};
 if(collection instanceof Set)return{name,entries:collection.size,sample:()=>take(collection.values(),P.collectionSample)};
 if(Array.isArray(collection))return{name,entries:collection.length,sample:()=>collection.slice(0,P.collectionSample)};
 const keys=Object.keys(collection??{});return{name,entries:keys.length,sample:()=>keys.slice(0,P.collectionSample).map(key=>collection[key])};
}
const take=(iterator,count)=>{const out=[];for(const item of iterator){out.push(item);if(out.length>=count)break;}return out;};

/** 🪶️ Enumerates live diagnostic collections and releases their tracking entries even between reports. */
export function createWeakCollectionTrackerV1(){
 const references=new Set(),dead=new FinalizationRegistry(ref=>references.delete(ref));
 return {
  track(value){const ref=new WeakRef(value);references.add(ref);dead.register(value,ref,ref);},
  *[Symbol.iterator](){
   for(const ref of references){
    const value=ref.deref();
    if(value===undefined){references.delete(ref);dead.unregister(ref);}
    else yield value;
   }
  }
 };
}

/** 🔌️ Registers a function that answers the collections of one part as `{name,entries,sample()}` or `{name,entries,bytes}`; returns the function that removes it. */
export function registerSliceCollectionsV1(name,read){collectionSources.set(name,read);return()=>{if(collectionSources.get(name)===read)collectionSources.delete(name);};}

/** 📏️ The estimated bytes of `entries` entries: entries times the mean size of the sampled ones (a string by twice its length, anything else by the length of its JSON, a sample that cannot be written by 64). */
export function estimateCollectionBytesV1(entries,sample){
 if(sample.length===0)return 0;
 let sum=0;for(const item of sample){if(typeof item==="string")sum+=item.length*2;else try{sum+=(JSON.stringify(item)??"").length;}catch{sum+=64;}}
 return Math.round(entries*sum/sample.length);
}

/** 🧾️ The `top` collections by estimated bytes, bounded to `bytes` characters of JSON. */
export function collectSliceCollectionsV1(policy=P){
 const rows=[];
 for(const[,read]of collectionSources){try{for(const row of read()){try{rows.push({name:String(row.name).slice(0,80),entries:row.entries,estimatedBytes:typeof row.bytes==="number"?Math.round(row.bytes):estimateCollectionBytesV1(row.entries,row.sample?.()??[])});}catch{}}}catch{}}
 rows.sort((a,b)=>b.estimatedBytes-a.estimatedBytes);
 const kept=rows.slice(0,policy.collectionsTop);
 while(kept.length>0&&JSON.stringify(kept).length>policy.collectionsBytes)kept.pop();
 return kept;
}

/** 🔌️ Lets a part of the read tell the report what it holds: `read()` answers plain JSON (counts, names, sizes); a part that throws is reported as its message. Returns the function that removes the source. */
export function registerSliceReportSourceV1(name,read){sources.delete(name);finals.delete(name);sources.set(name,read);return()=>{if(sources.get(name)===read){try{finals.set(name,read());}catch{}sources.delete(name);}};}

/** 🏁️ What the source `name` answered when it was removed, or undefined while it is registered or never was. */
export function readFinalSliceReportSourceV1(name){return sources.has(name)?undefined:finals.get(name);}

/** 📖️ What the source `name` answers now, or undefined while it is not registered or throws. */
export function readSliceReportSourceV1(name){const read=sources.get(name);if(read===undefined)return undefined;try{return read();}catch{return undefined;}}


const fill=(text,fields)=>text.replace(/\{(\w+)\}/gu,(_,name)=>String(fields[name]??""));

/** 🪜️ Runs `run` as the step `id` of the read with its declared bound. The progress line names the innermost active step. The inventory step can supply timing diagnostics and completed owner checks. A step that ends or fails is removed.
 * @see ./🧬️schema/🔣️.json */
export async function sliceReadStepV1(id,run,detail={},diagnostics){
 const row={id,startedMs:performance.now(),detail,diagnostics,boundMilliseconds:detail.boundMilliseconds??STEPS.steps[id]?.boundMilliseconds};
 if(row.boundMilliseconds===undefined)throw Error("Declared slice read step required: "+id);
 steps.push(row);movedAt=performance.now();
 try{return await run();}finally{const at=steps.lastIndexOf(row);if(at>=0)steps.splice(at,1);movedAt=performance.now();}
}

/** 🪧️ What the progress line says about the innermost running step: `{phase,done,total}` with the step in both languages, the seconds it has run as `done` and its bound as `total`, or undefined while no step runs. */
export function describeSliceReadStepV1(now=performance.now()){
 const row=steps.at(-1);if(row===undefined)return undefined;
 const seconds=Math.round((now-row.startedMs)/1000),bound=Math.round(row.boundMilliseconds/1000),spec=STEPS.steps[row.id],fields={round:1,...row.detail};
 if(row.id==="inventory"){
  const count=readSliceReportSourceV1("caches")?.interned;
  if(Number.isSafeInteger(count)&&count>=0)row.observations=Math.max(row.observations??0,count);
  try{
   const sample=row.diagnostics?.(),completed=sample?.completed;
   const discovery=sample?.active?.phase==="scoped-discovery"?sample.active:Array.isArray(completed)&&completed.length<=schema.definitions.InventoryAwait.properties.completed.maxItems?completed.find(value=>value.phase==="scoped-discovery"):undefined,checks=discovery?.completedOwners;
   if(Number.isSafeInteger(checks)&&checks>(row.completedOwnerChecks??0))row.completedOwnerChecks=checks;
   const verified=discovery?.verifiedMemoRows;
   if(Number.isSafeInteger(verified)&&verified>(row.verifiedMemoRows??0))row.verifiedMemoRows=verified;
  }catch{}
 }
 const observations=language=>row.observations===undefined?"":" ("+fill(STEPS.inventoryObservations[language],{rows:row.observations})+")";
 const ownerChecks=language=>row.completedOwnerChecks===undefined?"":" ("+fill(STEPS.inventoryOwnerChecks[language],{rows:row.completedOwnerChecks})+")";
 const memoChecks=language=>row.verifiedMemoRows===undefined?"":" ("+fill(STEPS.inventoryMemoChecks[language],{rows:row.verifiedMemoRows})+")";
 const text=language=>fill(spec[language],fields)+observations(language)+ownerChecks(language)+memoChecks(language)+" ("+STEPS.counter[language]+")"+(seconds>bound?" ("+fill(STEPS.overBound[language],{bound})+")":"");
 return{id:row.id,phase:text("en")+" / "+text("de"),done:seconds,total:bound};
}

/** 🔔️ The moment this process last made progress that a waiter of the fleet can rely on: a step began or ended, or the owners of the read counted more done, in flight or inferred (the processor time of the process counts too, so one large owner that runs for minutes is not idle). A claim of the fleet is renewed only while this is younger than `idleMilliseconds`. */
export function sliceReadMovedAtV1(){
 const owners=readSliceReportSourceV1("owners"),ledger=owners?.ledger,sign=JSON.stringify([ledger?.done,ledger?.running,owners?.finished,owners?.taken,owners?.cpuSeconds,steps.map(row=>row.id)]);
 if(sign!==movedSign){movedSign=sign;movedAt=performance.now();}
 return movedAt;
}

/** 🫀️ Whether this process has made progress within the idle bound of the schema (the heartbeat of a fleet claim asks it). */
export function sliceReadActiveV1(now=performance.now()){return now-sliceReadMovedAtV1()<STEPS.idleMilliseconds;}

/** 💓️ Tells `emit({phase:"taken",done,total})` every `milliseconds` while the owners source counts more owners decided (taken from the store or the fleet, or inferred) than the last time told. Later passes carry their own progress identity: a worker that takes thousands of owners shows work to whoever watches its output, which a worker that says nothing for ten minutes never does. Returns the function that stops it; the timer never keeps the process alive.
 * @see ./🧬️schema/🔣️.json */
export function startSliceProgressV1({emit,read=readSliceReportSourceV1,final=readFinalSliceReportSourceV1,milliseconds=P.progressMilliseconds}){
 let told,toldPass,finished;
 const timer=setInterval(()=>{
  const step=describeSliceReadStepV1(),live=read("owners"),sample=live??final("owners"),ledger=sample?.ledger,done=Number(ledger?.done),total=Number(ledger?.total);
  const pass=Number.isSafeInteger(sample?.pass)&&sample.pass>1?sample.pass:1;
  const valid=Number.isSafeInteger(done)&&Number.isSafeInteger(total)&&total>0,fresh=valid&&(live===undefined?finished!==ledger:done!==told||pass!==toldPass);
  if(step!==undefined&&!(step.id==="owners-round"&&fresh)){
   if(step.id!=="owners-round")told=undefined;
   try{emit({phase:step.phase,done:step.done,total:step.total});}catch{}
   return;
  }
  if(!fresh)return;
  const suffix=pass>1?` (pass ${pass})`:"";
  if(live===undefined){
   finished=ledger;told=undefined;
   try{emit({phase:"owners done"+suffix,done,total});}catch{}
   return;
  }
  finished=undefined;told=done;toldPass=pass;
  try{emit({phase:"taken"+suffix,done,total});}catch{}
 },milliseconds);
 timer.unref?.();
 return()=>clearInterval(timer);
}

/** 📍️ Notes the phase of the read (`{phase,done,total}` of a generation progress event) and the plan stage that starts (`{stage,operation,managedMegabytes,residentMegabytes}`). */
export function noteSliceReportPhaseV1(event){if(typeof event?.phase==="string")phase={name:event.phase,done:Number(event.done)||0,total:Number(event.total)||0};}
export function noteSliceReportStageV1(event){if(typeof event?.stage==="string")stage={stage:event.stage,operation:String(event.operation??""),managedMegabytes:Number(event.managedMegabytes)||0,residentMegabytes:Number(event.residentMegabytes)||0};}
export function noteSliceReportStopV1(event){stop={stage:String(event.stage),operation:String(event.operation),usedMegabytes:Number(event.usedMegabytes)||0,budgetMegabytes:Number(event.budgetMegabytes)||0};}

/** 🧮️ The bytes a value holds when every object, array, set and map is charged ONCE however many referrers it has (`seen` carries the ones already charged): an array or set costs 16 plus 8 per element, an object 16 plus 16 per key, a string only the slot that refers to it (the characters belong to the interner and the row dictionary). The JSON estimate of `estimateCollectionBytesV1` charges a structure shared by thousands of entries once per referrer (263 GB for a sidecar of 100 MB).
 * @see ./🧬️schema/🔣️.json */
export function sharedBytesV1(value,seen=new Set()){
 if(value===null||typeof value!=="object")return 0;
 if(seen.has(value))return 0;
 seen.add(value);
 if(ArrayBuffer.isView(value))return value.byteLength+16;
 let bytes=16;
 if(Array.isArray(value)||value instanceof Set){for(const item of value)bytes+=8+sharedBytesV1(item,seen);return bytes;}
 if(value instanceof Map){for(const[key,item]of value)bytes+=16+sharedBytesV1(key,seen)+sharedBytesV1(item,seen);return bytes;}
 for(const item of Object.values(value))bytes+=16+sharedBytesV1(item,seen);
 return bytes;
}

/** 🏷️ The role of a process by its command line: the base name of the first script it runs, else of its executable. */
export function processRoleV1(command){
 const words=String(command).trim().split(/\s+/u),script=words.find(word=>/\.(?:ts|mts|mjs|cjs|js)$/u.test(word));
 return basename(script??words[0]??"?").slice(0,60);
}

/** 🌳️ The process tree below `root` from a `ps -A -o pid=,ppid=,rss=,vsz=,command=` table: the root itself and its descendants with their resident and virtual megabytes and role, the largest `top` of them, the count and the summed resident megabytes of the descendants.
 * @see ./🧬️schema/🔣️.json */
export function parseProcessTreeV1(text,root,top=P.censusChildren){
 const rows=new Map(),children=new Map();
 for(const line of String(text).split("\n")){
  const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/u.exec(line);if(match===null)continue;
  const row={pid:Number(match[1]),parent:Number(match[2]),rssMegabytes:Math.round(Number(match[3])/1024*10)/10,vszMegabytes:Math.round(Number(match[4])/1024*10)/10,role:processRoleV1(match[5])};
  if(/^(?:\S*\/)?ps -A -o pid=,ppid=,rss=,vsz=,command=/u.test(match[5]))continue;
  rows.set(row.pid,row);if(!children.has(row.parent))children.set(row.parent,[]);children.get(row.parent).push(row.pid);
 }
 const found=[],queue=[...children.get(root)??[]],visited=new Set([root]);
 while(queue.length>0){const pid=queue.shift();if(visited.has(pid))continue;visited.add(pid);const row=rows.get(pid);if(row!==undefined){found.push(row);queue.push(...children.get(pid)??[]);}}
 const self=rows.get(root);
 return{...self===undefined?{}:{self:{rssMegabytes:self.rssMegabytes,vszMegabytes:self.vszMegabytes}},count:found.length,rssMegabytes:Math.round(found.reduce((total,row)=>total+row.rssMegabytes,0)*10)/10,children:found.sort((a,b)=>b.rssMegabytes-a.rssMegabytes).slice(0,top).map(({pid,role,rssMegabytes,vszMegabytes})=>({pid,role,rssMegabytes,vszMegabytes}))};
}

const psTable=()=>new Promise((resolve,reject)=>execFile("ps",["-A","-o","pid=,ppid=,rss=,vsz=,command="],{maxBuffer:1<<25,timeout:P.censusTimeoutMilliseconds,windowsHide:true},(error,stdout)=>error===null?resolve(stdout):reject(error)));

/** 📏️ What the process holds that `process.memoryUsage()` does not show: its physical footprint (macOS `phys_footprint`), the footprint of its whole tree and the process table of its children with their roles. Every reading is optional (a platform or a command that fails leaves its field out), and runs asynchronously so a report never waits for it.
 * @see ./🧬️schema/🔣️.json */
export async function measureSliceProcessesV1({footprint,table=osPlatform()==="win32"?undefined:psTable,pid=process.pid}={}){
 const out={},megabytes=bytes=>Number.isFinite(bytes)?Math.round(bytes/MEGABYTE):undefined;
 if(table!==undefined)try{out.processes=parseProcessTreeV1(await table(),pid);}catch{}
 try{const own=megabytes(await footprint?.self?.());if(own!==undefined)out.footprintMegabytes=own;}catch{}
 try{if((out.processes?.count??0)>0){const tree=megabytes(await footprint?.tree?.());if(tree!==undefined)out.treeFootprintMegabytes=tree;}}catch{}
 return out;
}

/** 🧾️ The report of this moment, bounded to `fileBytes`: sources that do not fit are dropped, the largest first, and named in `dropped`. */
export function buildSliceReportV1({reason,heap,now=Date.now,policy=P}){
 const usage=process.memoryUsage(),stats=heap?.()??undefined,body={};
 for(const[name,read]of sources){try{body[name]=read();}catch(error){body[name]={error:String(error?.message??error).slice(0,200)};}}
 try{const inventory=steps.findLast(row=>row.id==="inventory")?.diagnostics?.();if(inventory!==undefined)body.inventory=inventory;}catch{}
 const report={version:1,atMs:now(),reason,pid:process.pid,uptimeSeconds:Math.round((performance.now()-startedAt)/100)/10,phase,...stage===undefined?{}:{stage},memory:{rss:megabytes(usage.rss),heapUsed:megabytes(usage.heapUsed),heapTotal:megabytes(usage.heapTotal),external:megabytes(usage.external),arrayBuffers:megabytes(usage.arrayBuffers)},guard:{measure:"heapUsed+external",megabytes:megabytes(usage.heapUsed+usage.external),heapUsed:megabytes(usage.heapUsed),external:megabytes(usage.external)},...measured,...stats===undefined?{}:{heap:{heapMegabytes:megabytes(stats.heapSize),extraMegabytes:megabytes(stats.extraMemorySize),objectCount:stats.objectCount,types:Object.entries(stats.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,policy.topTypes).map(([type,count])=>({type,count}))}},...stop===undefined?{}:{stop},collections:collectSliceCollectionsV1(policy),sources:body};
 const dropped=[];
 while(JSON.stringify(report).length>policy.fileBytes&&Object.keys(report.sources).length>0){
  const largest=Object.entries(report.sources).sort((a,b)=>JSON.stringify(b[1]).length-JSON.stringify(a[1]).length)[0][0];
  delete report.sources[largest];dropped.push(largest);
 }
 return dropped.length>0?{...report,dropped}:report;
}

/** 💾️ Writes the generic machine report and returns this process's exact serialized sample. */
export function writeSliceReportV1(root,options){
 try{
  const directory=machinePathV1(root,P.directory),text=JSON.stringify(buildSliceReportV1(options));
  mkdirSync(directory,{recursive:true});writeSliceReportTextV1(join(directory,P.name),text);return text;
 }catch{}
}

/** ⏱️ Writes the report now and every `intervalMilliseconds` until the returned function is called (which writes the last one); the timer never keeps the process alive. A report never walks the heap (a heap snapshot of a 500 MB heap held 4 GB of resident pages for the rest of the process and made the plan guard see 10 GB): it reads the runtime counters, the process table and, through `footprint` (`{self,tree}`), the physical footprint of the process and of its tree.
 * @see ./🧬️schema/🔣️.json */
export function startSliceReportV1({root,heap,footprint,table,intervalMilliseconds=P.intervalMilliseconds}){
 let collected=false,reading=false,latest;
 const publish=reason=>{latest=writeSliceReportV1(root,{reason,heap});};
 const refresh=()=>{if(reading)return;reading=true;measureSliceProcessesV1({footprint,...table===undefined?{}:{table}}).then(row=>{measured=row;},()=>undefined).finally(()=>{reading=false;});};
 refresh();
 publish("start");
 const collect=()=>{
  if(collected||process.memoryUsage().heapUsed/MEGABYTE<P.collectMinimumHeapMegabytes)return;
  collected=true;const started=performance.now();
  try{globalThis.Bun?.gc?.(true);}catch{}
  const usage=process.memoryUsage(),stats=heap?.();
  sources.set("afterCollect",()=>({heapUsed:megabytes(usage.heapUsed),external:megabytes(usage.external),seconds:Math.round((performance.now()-started)/100)/10,...stats===undefined?{}:{types:Object.entries(stats.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,P.topTypes).map(([type,count])=>({type,count}))}}));
 };
 const timer=setInterval(()=>{refresh();collect();publish("interval");},intervalMilliseconds);timer.unref?.();
 return Object.freeze({now:publish,read:()=>latest,stop:()=>{clearInterval(timer);publish("end");}});
}
