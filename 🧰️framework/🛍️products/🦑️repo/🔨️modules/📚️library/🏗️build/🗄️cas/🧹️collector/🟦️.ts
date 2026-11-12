import {lstat,mkdir,open,readdir,readFile,rename,stat,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import type {FileHandle} from "node:fs/promises";
import {compactCasShardV1,foldCasShardV1,openCasShardViewV1,type CasShardViewV1} from "../📒️ledger/🟦️.ts";
import {casPinDirectoryV1,readCasPinsV1} from "../📌️pins/🟦️.ts";
import {casPartitionNeedBytesV1,casPartitionUsedFractionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {CAS_POLICY_V1,casTextV1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {createCasPlannerV1,type CasPlanTotalsV1,type CasReasonV1} from "./🧮️plan.ts";

export {casCutV1,createCasPlannerV1} from "./🧮️plan.ts";
export type {CasCutV1,CasPlanTotalsV1,CasReasonV1} from "./🧮️plan.ts";

/** 📂️ One loose-file area outside the store: independent cache files below `directory` (for example the host's graph memo or a workspace's build-input closures) whose bytes count against the host cap. `id` names the policy bounds it falls under. */
export type CasAreaSourceV1=Readonly<{id:string;directory:string;skip?:RegExp}>;
/** 📣️ Progress of one collection. */
export type CasProgressV1=Readonly<{stage:"scan"|"evict"|"compact";entries:number;usedBytes:number;freedBytes:number}>;
/** 🔌️ Everything a collection needs from its host; `partition` is the partition that holds the store as measured by the caller (none: only the cap is enforced). */
export type CasCollectOptionsV1=Readonly<{
 root:string;policy?:CasPolicyV1;areas?:readonly CasAreaSourceV1[];externalBytes?:number;partition?:CasPartitionV1;signal?:AbortSignal;dryRun?:boolean;
 now?():number;pid?:number;progress?(event:CasProgressV1):void|Promise<void>;
 beforeEvict?(path:string):void|Promise<void>;
 openLock?(path:string):Promise<FileHandle>;
}>;
/** 🌡️ What a collection saw of the partition that holds the store: its share in use before it, and the bytes it was above its low share. */
export type CasCollectedPartitionV1=Readonly<{partition:string;usedFraction:number;needBytes:number}>;
/** 🧾️ What a collection did; `locked` is a collection that found another one running, `lockless` one that ran without the lock because the disk was too full to create it. */
export type CasCollectionV1=Readonly<{status:"idle"|"collected"|"locked";partial:boolean;dryRun:boolean;scanned:number;evicted:number;freedBytes:number;busy:number;skipped:number;totals:CasPlanTotalsV1;byReason:Readonly<Record<CasReasonV1,number>>;lockless?:true;partition?:CasCollectedPartitionV1}>;

const STORE="store",zero:CasPlanTotalsV1={totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,needBytes:0};
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const sorted=(names:string[]):string[]=>names.sort((left,right)=>left<right?-1:left>right?1:0);
const TEMPORARY=/(^\.)|(\.tmp$)/u;

type Entry={area:string;path:string;size:number;access:number;protectedEntry:boolean;shard?:string;hash?:string};

const fullDisk=(error:unknown):boolean=>["ENOSPC","EDQUOT"].includes((error as NodeJS.ErrnoException)?.code??"");
const withoutLock=async():Promise<void>=>undefined;

async function acquire(root:string,policy:CasPolicyV1,nowMs:number,pid:number,openLock:(path:string)=>Promise<FileHandle>):Promise<{release:()=>Promise<void>;lockless:boolean}|undefined>{
 const lock=join(root,"v1","gc.lock");await mkdir(join(root,"v1"),{recursive:true});
 for(let attempt=0;attempt<2;attempt++){
  try{
   const handle=await openLock(lock);
   try{await handle.writeFile(JSON.stringify({pid,atMs:nowMs}));}catch(error){await handle.close().catch(()=>undefined);await unlink(lock).catch(()=>undefined);if(fullDisk(error))return{release:withoutLock,lockless:true};throw error;}
   await handle.close();
   return{release:async()=>{await unlink(lock).catch(()=>undefined);},lockless:false};
  }catch(error){
   if(fullDisk(error))return{release:withoutLock,lockless:true};
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   try{
    const held=JSON.parse(await readFile(lock,"utf8")) as {pid:number;atMs:number};
    if(alive(held.pid)&&nowMs-held.atMs<policy.lockStaleMs)return undefined;
   }catch{}
   await unlink(lock).catch(()=>undefined);
  }
 }
 return undefined;
}

async function* walk(directory:string,skip:RegExp,signal?:AbortSignal):AsyncGenerator<{path:string;size:number;mtimeMs:number;atimeMs:number;nlink:number}>{
 let names:string[];try{names=sorted(await readdir(directory));}catch{return;}
 for(const name of names){
  signal?.throwIfAborted();const path=join(directory,name);let info;
  try{info=await lstat(path);}catch{continue;}
  if(info.isSymbolicLink())continue;
  if(info.isDirectory()){yield*walk(path,skip,signal);continue;}
  if(!info.isFile()||skip.test(name))continue;
  yield{path,size:info.size,mtimeMs:info.mtimeMs,atimeMs:info.atimeMs,nlink:info.nlink};
 }
}

async function* entries(options:CasCollectOptionsV1,policy:CasPolicyV1,pins:ReadonlyMap<string,string>,nowMs:number,views:Map<string,CasShardViewV1>):AsyncGenerator<Entry>{
 const objects=join(options.root,"v1","o");let shards:string[];
 try{shards=sorted(await readdir(objects));}catch{shards=[];}
 for(const shard of shards){
  options.signal?.throwIfAborted();let view=views.get(shard);if(!view){view=await openCasShardViewV1(options.root,shard,{nowMs:nowMs,quietMs:policy.ledgerQuietMs});views.set(shard,view);}const ledger=view.snapshot();let names:string[];
  try{names=sorted(await readdir(join(objects,shard)));}catch{continue;}
  for(const name of names){
   const path=join(objects,shard,name),hash=shard+name;let info;try{info=await lstat(path);}catch{continue;}
   if(!info.isFile())continue;
   const access=Math.max(info.mtimeMs,ledger.get(hash)??0);
   yield{area:STORE,path,size:info.size,access,protectedEntry:pins.has(hash)||info.nlink>1||nowMs-access<policy.minAgeMs,shard,hash};
  }
 }
 for(const area of options.areas??[]){
  for await(const file of walk(area.directory,area.skip??TEMPORARY,options.signal)){
   const access=Math.max(file.mtimeMs,file.atimeMs);
   yield{area:area.id,path:file.path,size:file.size,access,protectedEntry:nowMs-access<policy.minAgeMs};
  }
 }
}

async function sweepTemporary(root:string,policy:CasPolicyV1,nowMs:number):Promise<void>{
 const directory=join(root,"v1","t");let names:string[];try{names=await readdir(directory);}catch{return;}
 for(const name of names)try{if(nowMs-(await lstat(join(directory,name))).mtimeMs>policy.temporaryMaxAgeMs)await unlink(join(directory,name));}catch{}
}

/** 🧹️ Collects the host store and its declared areas: scans once (bounded memory, shard by shard), decides with the streaming least-recently-accessed planner (down to the low watermark of the cap and, when the caller measured the partition, down to the low used share of the partition), then evicts in a second pass that re-checks pins and fresh access right before every removal. Single-flight per root through a lock file with stale-owner takeover. Idempotent and stateless, so a crash or cancellation leaves a consistent store and the next run simply continues; objects are written by atomic rename, so no partial object is ever visible. It frees space before it needs any: stale temporary files go first, and when the disk is too full to create the lock the collection runs without it (every removal is idempotent and re-checked, so two collectors cannot do harm) and skips the ledger compaction and the usage hint, so a full disk never keeps it from deleting. */
export async function collectCasV1(options:CasCollectOptionsV1):Promise<CasCollectionV1>{
 const policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,pid=options.pid??process.pid,signal=options.signal;
 await sweepTemporary(options.root,policy,now());
 const lock=await acquire(options.root,policy,now(),pid,options.openLock??(path=>open(path,"wx"))),release=lock?.release;
 if(!lock||!release)return{status:"locked",partial:false,dryRun:options.dryRun===true,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:zero,byReason:{age:0,area:0,host:0}};
 const views=new Map<string,CasShardViewV1>();let scanned=0,evicted=0,freed=0,busy=0,skipped=0,used=0,lastReport=-Infinity,partial=false;const byReason:Record<CasReasonV1,number>={age:0,area:0,host:0};
 const report=async(stage:CasProgressV1["stage"],force=false):Promise<void>=>{
  signal?.throwIfAborted();const at=now();
  if(!force&&at-lastReport<policy.progressMs)return;
  lastReport=at;await options.progress?.({stage,entries:scanned,usedBytes:used,freedBytes:freed});signal?.throwIfAborted();
 };
 try{
  const started=now(),partitionNeed=casPartitionNeedBytesV1(options.partition,policy),partition=options.partition?Object.freeze({partition:options.partition.partition,usedFraction:casPartitionUsedFractionV1(options.partition),needBytes:partitionNeed}):undefined,lockless=lock.lockless?{lockless:true as const}:{};
  const bounds=[{id:STORE,capBytes:null,maxAgeMs:null},...(options.areas??[]).map(area=>{const row=policy.areas.find(candidate=>candidate.id===area.id);return{id:area.id,capBytes:row?.capBytes??null,maxAgeMs:row?.maxAgeMs??null};})];
  const planner=createCasPlannerV1({policy,areas:bounds,externalBytes:options.externalBytes??0,nowMs:started,partitionNeedBytes:partitionNeed});
  let pins=await readCasPinsV1(options.root,started);
  try{
   for await(const entry of entries(options,policy,pins,started,views)){
    planner.add(entry.area,entry.access,entry.size,entry.protectedEntry);scanned++;used+=entry.size;
    if(scanned%policy.checkEvery===0)await report("scan");
   }
  }catch(error){if(signal?.aborted){partial=true;}else throw error;}
  const plan=partial?undefined:planner.finish();
  if(!plan)return{status:"collected",partial:true,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals:zero,byReason,...lockless,...partition?{partition}:{}};
  const totals=plan.totals;
  if(totals.plannedBytes<=0){
   await compactAll(options,policy,started,views);await recordUsage(options,totals.keptBytes-(options.externalBytes??0),now());
   return{status:"idle",partial:false,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason,...lockless,...partition?{partition}:{}};
  }
  const evaluator=plan.evaluator();pins=await readCasPinsV1(options.root,now());let currentShard:string|undefined,view:CasShardViewV1|undefined,pinStamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;
  try{
   for await(const entry of entries(options,policy,pins,started,views)){
    if(entry.shard!==currentShard){currentShard=entry.shard;pins=await readCasPinsV1(options.root,now());view=entry.shard?views.get(entry.shard):undefined;}
    if(entry.protectedEntry)continue;
    const reason=evaluator.decide(entry.area,entry.access,entry.size);
    if(reason===null)continue;
    if(options.dryRun){evicted++;freed+=entry.size;byReason[reason]++;continue;}
    await options.beforeEvict?.(entry.path);
    await view?.refresh();const stamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;if(stamp!==pinStamp){pinStamp=stamp;pins=await readCasPinsV1(options.root,now());}const fresh=entry.hash?Math.max((await lstat(entry.path).catch(()=>undefined))?.mtimeMs??-1,view?.latest(entry.hash)??0):(await lstat(entry.path).catch(()=>undefined))?.atimeMs??-1;
    if(fresh<0)continue;
    if(fresh>entry.access||fresh>=started||(entry.hash&&pins.has(entry.hash))){skipped++;continue;}
    try{await unlink(entry.path);evicted++;freed+=entry.size;used-=entry.size;byReason[reason]++;}
    catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")continue;if(code==="EBUSY"||code==="EPERM"||code==="EACCES"){busy++;continue;}throw error;}
    if(evicted%policy.checkEvery===0)await report("evict");
   }
  }catch(error){if(signal?.aborted)partial=true;else throw error;}
  if(!partial&&!options.dryRun){await compactAll(options,policy,started,views);await recordUsage(options,used,now());}
  await options.progress?.({stage:"evict",entries:scanned,usedBytes:used,freedBytes:freed});
  return{status:"collected",partial,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason,...lockless,...partition?{partition}:{}};
 }finally{await release();}
}

async function compactAll(options:CasCollectOptionsV1,policy:CasPolicyV1,nowMs:number,views:ReadonlyMap<string,CasShardViewV1>):Promise<void>{
 if(options.dryRun)return;
 const objects=join(options.root,"v1","o");let shards:string[];try{shards=sorted(await readdir(objects));}catch{return;}
 for(const shard of shards){
  options.signal?.throwIfAborted();let names:string[];try{names=await readdir(join(objects,shard));}catch{continue;}
  const live=new Set(names.map(name=>shard+name));
  try{const view=views.get(shard);if(view)await view.refresh();await compactCasShardV1(options.root,shard,live,view?view.snapshot():await foldCasShardV1(options.root,shard),nowMs,policy);}catch(error){if(fullDisk(error))return;throw error;}
 }
}

/** 📈️ The file that tells a writer how many bytes the store and its areas held when a collection last finished, so that its admission can judge the cap without scanning a store of millions of objects. */
export const casUsageHintPathV1=(root:string):string=>join(root,"v1","usage.json");

/** 📈️ Reads the usage hint: the bytes the last finished collection left and when; `undefined` when there is none or it is unreadable. */
export async function readCasUsageHintV1(root:string):Promise<Readonly<{bytes:number;atMs:number}>|undefined>{
 try{
  const row=JSON.parse(await readFile(casUsageHintPathV1(root),"utf8")) as {version?:unknown;bytes?:unknown;atMs?:unknown};
  return row.version===1&&typeof row.bytes==="number"&&Number.isFinite(row.bytes)&&row.bytes>=0&&typeof row.atMs==="number"?Object.freeze({bytes:row.bytes,atMs:row.atMs}):undefined;
 }catch{return undefined;}
}

async function recordUsage(options:CasCollectOptionsV1,bytes:number,atMs:number):Promise<void>{
 if(options.dryRun)return;
 const target=casUsageHintPathV1(options.root),temporary=`${target}.${process.pid}.tmp`;
 try{await writeFile(temporary,JSON.stringify({version:1,bytes:Math.max(0,Math.round(bytes)),atMs}));await rename(temporary,target);}
 catch{await unlink(temporary).catch(()=>undefined);}
}

/** 📣️ The one-line report of a collection in the requested language. */
export function formatCasCollectionV1(result:CasCollectionV1,policy:CasPolicyV1,language:"en"|"de"):string{
 const percent=(fraction:number):number=>Math.round(fraction*1000)/10;
 const mib=(bytes:number):number=>Math.round(bytes/1048576*10)/10,gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100;
 if(result.status==="locked")return casTextV1("locked",language);
 const partition=result.partition&&result.partition.needBytes>0?result.partition:undefined,policyLine=partition?casTextV1("partitionState",language,{partition:partition.partition,usedPercent:percent(partition.usedFraction),highPercent:percent(policy.partition.highUsedFraction),lowPercent:percent(policy.partition.lowUsedFraction)}):"";
 if(result.status==="idle"&&!partition)return casTextV1("idle",language,{usedGib:gib(result.totals.totalBytes),capGib:gib(policy.capBytes)});
 const text=casTextV1("done",language,{evicted:result.evicted,freedMib:mib(result.freedBytes),keptGib:gib(result.totals.keptBytes),protectedMib:mib(result.totals.protectedBytes),partial:result.partial?casTextV1("partial",language):""})+policyLine;
 if(result.totals.shortfallBytes<=0)return text;
 return partition?text+casTextV1("partitionShortfall",language,{overMib:mib(result.totals.shortfallBytes)}):text+" "+casTextV1("shortfall",language,{overMib:mib(result.totals.shortfallBytes)});
}
