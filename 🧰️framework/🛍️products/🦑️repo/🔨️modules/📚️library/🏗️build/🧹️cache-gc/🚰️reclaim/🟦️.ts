import {lstatSync,readdirSync,rmSync} from "node:fs";
import {join} from "node:path";
import {BUILD_JOURNAL_V1} from "../../🕸️graph/🕰️generations/🔖️journal/🟨️.mjs";
import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";
import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {sweepDeadProducerPublicationsV1} from "../../👁️observation/🪦️producer-publications/🟨️.mjs";

/** 🔌️ What the reclaim needs from the repository: the single-flight lease, the retained graph partition owner, the optional member indexing of its groups, its free space and the two shared recoveries; tests replace them. */
export type GraphReclaimPortsV1=Readonly<{
 exclusive(resource:string):Promise<Readonly<{release():void}>|undefined>;
 open():Promise<Readonly<{close():Promise<void>}>>;
 exists?():boolean;
 index?(image:object):Promise<Readonly<{groups:number;indexed:number;busy:number;refused:number;compacted:number;cancelled:boolean}>>;
 supersede?(image:object,dryRun:boolean):Promise<Readonly<{groups:number;busy:number;skipped:number;headers:number;removed:number;bytes:number;cancelled:boolean}>>;
 closures?(image:object,dryRun:boolean):Promise<Readonly<{candidates:number;removed:number;bytes:number;busy:number;unvouched:number;cancelled:boolean}>>;
 publications?(image:object,dryRun:boolean):Promise<Readonly<{candidates:number;removed:number;bytes:number;cancelled:boolean}>>;
 journals?(image:object,dryRun:boolean):Promise<Readonly<{candidates:number;removed:number;bytes:number;kept:number;cancelled:boolean}>>;
 closureStores?(image:object,dryRun:boolean):Promise<Readonly<{stores:number;entries:number;layers:number;values:number;kept:number;bytes:number;graceMilliseconds:number;cancelled:boolean}>>;
 free(image:object):Promise<number>;
 selected(image:object,request:Readonly<{minimumAvailableBytes:number;wait:false}>):Promise<unknown>;
 optional(image:object,request:Readonly<{minimumAvailableBytes:number}>):Promise<unknown>;
 lru?(image:object,dryRun:boolean):Promise<Readonly<{usedBytes:number;maximumBytes:number;targetBytes:number;remainingBytes:number;retiredGroups:number;retiredFiles:number;retiredBytes:number;busy:number;protected:number;cancelled:boolean}>>;
}>;

/** 🪦️ Sweeps the per-process producer publications (`p-<pid>`) of processes that are gone below the selections of the graph image: nothing reads them once their process ended, a live process keeps its own. */
export async function graphProducerPublicationsV1(imageRoot:string,dryRun:boolean,signal:AbortSignal):Promise<Readonly<{candidates:number;removed:number;bytes:number;cancelled:boolean}>>{
 const report=await sweepDeadProducerPublicationsV1(join(imageRoot,"inference","selections"),{dryRun,signal});
 return{candidates:report.candidates.length,removed:report.removed,bytes:report.bytes,cancelled:report.cancelled};
}

/** 🔖️ One build journal of the generation store: the files `<key>.json` and `<key>.<n>.rows` that carry the finished waves of one unsealed build, its newest write and its bytes. */
export type BuildJournalGroupV1=Readonly<{key:string;files:readonly string[];newestMs:number;bytes:number}>;

/** 🧮️ Picks the journals no build can resume: a build that is alive rewrites its own journal at every wave and only one build holds the generation writer at a time, so the newest journal and every journal written within the grace are kept and every other one is dead, because later builds start from another membership and therefore name another journal. The newest one is dead as well once it rests beyond the grace without its header `<key>.json`: the header is the only document `readBuildJournalV1` resumes from, so rows files without it are a build killed in its first wave that no code reads. Pure and language-agnostic.
 * @see ./../🧪️tests/🗞️journals/🟦️.ts
 * @see ./../../🕸️graph/🕰️generations/🔖️journal/🟨️.mjs */
export function staleBuildJournalGroupsV1(groups:readonly BuildJournalGroupV1[],nowMs:number,graceMs:number):readonly BuildJournalGroupV1[]{
 const newest=groups.reduce<BuildJournalGroupV1|undefined>((best,group)=>best===undefined||group.newestMs>best.newestMs?group:best,undefined);
 return groups.filter(group=>(group!==newest||!group.files.includes(group.key+".json"))&&nowMs-group.newestMs>graceMs);
}

/** 🗞️ Reads the journals of the generation store below the graph image, sorted by key. */
export function readBuildJournalGroupsV1(imageRoot:string):readonly BuildJournalGroupV1[]{
 const directory=join(imageRoot,"generations",BUILD_JOURNAL_V1.directory),groups=new Map<string,{files:string[];newestMs:number;bytes:number}>();
 let names:string[]=[];try{names=readdirSync(directory);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 for(const name of names){
  const key=/^([0-9a-f]{64})\.(?:json|\d+\.rows)$/u.exec(name)?.[1];if(key===undefined)continue;
  let row;try{row=lstatSync(join(directory,name));}catch{continue;}
  if(!row.isFile())continue;
  const group=groups.get(key)??{files:[],newestMs:0,bytes:0};group.files.push(name);group.newestMs=Math.max(group.newestMs,row.mtimeMs);group.bytes+=row.size;groups.set(key,group);
 }
 return[...groups].sort((left,right)=>left[0]<right[0]?-1:1).map(([key,group])=>Object.freeze({key,...group}));
}

/** 🪦️ Retires the dead build journals below the generation store of the graph image (every abandoned build leaves gigabytes of rows files that only its own key could resume); a dry run only counts them. */
export async function graphStaleBuildJournalsV1(imageRoot:string,dryRun:boolean,signal:AbortSignal,nowMs:number=Date.now()):Promise<Readonly<{candidates:number;removed:number;bytes:number;kept:number;cancelled:boolean}>>{
 const groups=readBuildJournalGroupsV1(imageRoot),stale=staleBuildJournalGroupsV1(groups,nowMs,CACHE_GC_LIMITS_V1.journalGraceMilliseconds),directory=join(imageRoot,"generations",BUILD_JOURNAL_V1.directory);
 let removed=0,bytes=0,cancelled=false;
 if(!dryRun)for(const group of stale){
  if(signal.aborted){cancelled=true;break;}
  for(const name of[...group.files].sort((left,right)=>Number(left.endsWith(".json"))-Number(right.endsWith(".json"))))rmSync(join(directory,name),{force:true});
  removed++;bytes+=group.bytes;
 }
 return{candidates:stale.length,removed,bytes:dryRun?stale.reduce((sum,group)=>sum+group.bytes,0):bytes,kept:groups.length-stale.length,cancelled};
}

/** 🚰️ Reclaims optional graph caches up to the ledger's target free space, in the background and under the shared hold of the retained graph image: first it removes the observation blocks no record names any more (the inventory of the graph image is finite and every re-inference supersedes blocks), retires the least recently written optional groups while the optional cache holds more than its low-water mark (its own limit, whatever the free space), then collects the closure stores (entries superseded in their slot and layers or values no entry names, after a grace), then it indexes the groups no member index vouches for yet (leaves are read without a lease, each group's non-blocking writer lease is held only while rows are appended, because a group is retired by its index and an lstat of each member), then reclaims scope by scope, never the exclusive maintenance of the image unless that is not enough. Under physical pressure, first tries idle selected publications and indexed optional scopes without waiting on live graph writers. Returns the localized report lines. */
export async function reclaimGraphOptionalV1(workspace:string,ledger:CacheLedgerV1,dryRun:boolean,language:CacheLanguageV1,ports:GraphReclaimPortsV1):Promise<string>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(language,key,variables),reclaim=ledger.areas.find(row=>row.id==="graph-inference")?.reclaim;
 if(!reclaim)return text("gc.graph.undeclared");
 if(ports.exists&&!ports.exists())return text("gc.graph.absent");
 const lease=await ports.exclusive("repository-graph-optional-collector:"+workspace);
 if(!lease)return text("gc.graph.running");
 let image:Awaited<ReturnType<GraphReclaimPortsV1["open"]>>|undefined;
 try{
  image=await ports.open();
  const lines:string[]=[],finish=(line:string)=>[...lines,line].join("\n"),targetAvailableBytes=reclaim.targetAvailableBytes;
  const recovered=(before:number,after:number)=>finish(text("gc.graph.done",{freed:formatCacheBytesV1(Math.max(0,after-before)),free:formatCacheBytesV1(after),target:formatCacheBytesV1(targetAvailableBytes)}));
  const before=dryRun?undefined:await ports.free(image),pressed=before!==undefined&&before<targetAvailableBytes;
  if(ports.publications){const report=await ports.publications(image,dryRun);if(report.candidates)lines.push(text(dryRun?"gc.graph.publications.plan":"gc.graph.publications.done",{count:report.candidates,bytes:formatCacheBytesV1(report.bytes)}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(ports.journals){const report=await ports.journals(image,dryRun);if(report.candidates)lines.push(text(dryRun?"gc.graph.journals.plan":"gc.graph.journals.done",{count:report.candidates,bytes:formatCacheBytesV1(report.bytes),kept:report.kept}));if(report.cancelled)return finish(text("error.cancelled"));}
 if(pressed){
   await ports.selected(image,{minimumAvailableBytes:targetAvailableBytes,wait:false});
   let free=await ports.free(image);
   if(free<targetAvailableBytes){
    await ports.optional(image,{minimumAvailableBytes:targetAvailableBytes});
    free=await ports.free(image);
   }
   if(free>=targetAvailableBytes)return recovered(before,free);
  }
  if(ports.lru){const report=await ports.lru(image,dryRun),bytes=formatCacheBytesV1;lines.push(text(report.usedBytes<=report.targetBytes?"gc.graph.quota.within":dryRun?"gc.graph.quota.plan":"gc.graph.quota.done",{used:bytes(report.usedBytes),limit:bytes(report.maximumBytes),target:bytes(report.targetBytes),remaining:bytes(report.remainingBytes),groups:report.retiredGroups,files:report.retiredFiles,bytes:bytes(report.retiredBytes),busy:report.busy,protected:report.protected}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(ports.closures){const report=await ports.closures(image,dryRun);if(report.unvouched)lines.push(text("gc.graph.closures.unvouched"));else if(report.candidates)lines.push(text(dryRun?"gc.graph.closures.plan":"gc.graph.closures.done",{removed:report.removed,candidates:report.candidates,megabytes:Math.round(report.bytes/1048576),busy:report.busy}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(ports.supersede){const report=await ports.supersede(image,dryRun);if(report.removed||report.busy||report.skipped)lines.push(text(dryRun?"gc.graph.supersede.plan":"gc.graph.superseded",{removed:report.removed,megabytes:Math.round(report.bytes/1048576),groups:report.groups,kept:report.skipped,busy:report.busy}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(ports.closureStores){const report=await ports.closureStores(image,dryRun);if(report.entries||report.layers||report.values)lines.push(text(dryRun?"gc.graph.ce.plan":"gc.graph.ce.done",{entries:report.entries,layers:report.layers,values:report.values,megabytes:Math.round(report.bytes/1048576),stores:report.stores,kept:report.kept,minutes:Math.round(report.graceMilliseconds/60000)}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(!dryRun&&ports.index){const report=await ports.index(image);if(report.indexed||report.refused||report.compacted||report.busy)lines.push(text("gc.graph.indexed",{indexed:report.indexed,groups:report.groups,busy:report.busy,refused:report.refused,compacted:report.compacted}));if(report.cancelled)return finish(text("error.cancelled"));}
  const current=await ports.free(image),variables={free:formatCacheBytesV1(current),target:formatCacheBytesV1(targetAvailableBytes)};
  if(current>=targetAvailableBytes)return pressed?recovered(before,current):finish(text("gc.graph.enough",variables));
  if(dryRun)return finish(text("gc.graph.plan",variables));
  if(!pressed){
   await ports.selected(image,{minimumAvailableBytes:targetAvailableBytes,wait:false});
   const free=await ports.free(image);
   if(free>=targetAvailableBytes)return recovered(current,free);
  }
  await ports.optional(image,{minimumAvailableBytes:targetAvailableBytes});
  return recovered(pressed?before:current,await ports.free(image));
 }finally{
  try{await image?.close();}finally{lease.release();}
 }
}
