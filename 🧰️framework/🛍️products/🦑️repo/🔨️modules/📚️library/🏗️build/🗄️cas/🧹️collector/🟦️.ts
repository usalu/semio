import {lstat,mkdir,open,readdir,readFile,rename,rmdir,stat,unlink,writeFile} from "node:fs/promises";
import {setTimeout as sleep} from "node:timers/promises";
import {basename,dirname,join} from "node:path";
import type {FileHandle} from "node:fs/promises";
import {casCensusPathV1} from "../🗃️store/🟦️.ts";
import {compactCasShardV1,foldCasShardV1,openCasShardViewV1,type CasShardViewV1} from "../📒️ledger/🟦️.ts";
import {casPinDirectoryV1,compactCasTransientsV1,readCasPinsV1,readCasTransientsV1,stampCasTransientsV1} from "../📌️pins/🟦️.ts";
import {casPartitionNeedBytesV1,casPartitionUsedFractionV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {CAS_GRAPH_CLOSURE_AREA_V1,CAS_POLICY_V1,casTextV1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {createCasPlannerV1,type CasPlanTotalsV1,type CasReasonV1} from "./🧮️plan.ts";
import {casKeeperCountSyncV1,planCasHolderEvictionV1,readCasHeldV1,reconcileCasKeepersV1,recordCasHolderEvictionV1,resetCasHeldCacheV1,type CasHeldV1,type CasHolderPlanBlobV1} from "../🫂️holders/🟦️.ts";
import {reclaimCasObjectsV1} from "../🪦️reclaim/🟦️.ts";
import {processIdentityForPidV1,parseProcessIdentityV1,processHolderAliveV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

export {casCutV1,createCasPlannerV1} from "./🧮️plan.ts";
export type {CasCutV1,CasPlanTotalsV1,CasReasonV1} from "./🧮️plan.ts";

/** 📂️ One loose-file area outside the store whose bytes count against the host cap. The graph owns retirement of source inventories. Other files named `<sha256>.json` are protected while a live holder pins that digest. */
export type CasAreaSourceV1=Readonly<{id:string;directory:string;skip?:RegExp}>;
/** 📣️ Progress of one collection. */
export type CasProgressV1=Readonly<{stage:"scan"|"evict"|"compact";entries:number;usedBytes:number;freedBytes:number}>;
/** 🔌️ Everything a collection needs from its host; `partition` is the partition that holds the store as measured by the caller (none: only the cap is enforced). */
export type CasCollectOptionsV1=Readonly<{
 root:string;policy?:CasPolicyV1;incremental?:boolean;areas?:readonly CasAreaSourceV1[];externalBytes?:number;partition?:CasPartitionV1;signal?:AbortSignal;dryRun?:boolean;
 now?():number;pid?:number;progress?(event:CasProgressV1):void|Promise<void>;
 automaticScope?():Promise<string|undefined>;
 completedAutomatic?(hint:CasUsageHintV1):void;
 beforeEvict?(path:string):void|Promise<void>;
 openLock?(path:string):Promise<FileHandle>;
 role?:string;unreferencedOnly?:boolean;unreferencedStepBytes?:number;waitMs?:number;
}>;
/** 🌡️ What a collection saw of the partition that holds the store: its share in use before it, and the bytes it was above its low share. */
export type CasCollectedPartitionV1=Readonly<{partition:string;totalBytes:number;usedFraction:number;needBytes:number}>;
/** 🫂️ What the registered reference holders (unit-cache roots) cost a collection: how many objects and bytes their live entries keep, and what the capacity stage took from them when the unreferenced objects did not cover the need (entries together with the objects only they referenced, `reason` is always `capacity`). */
export type CasHolderEvictionV1=Readonly<{holders:number;entries:number;heldBlobs:number;heldBytes:number;evictedEntries:number;evictedBlobs:number;freedBytes:number;protectedEntries:number;reason:"capacity"}>;
/** 🔒️ Who holds the collection lock of a store and how long ago it last made progress. */
export type CasLockHolderV1=Readonly<{pid:number;beatMs:number}>;
/** 🧾️ What a collection did; `locked` is a collection that found another one running, `lockless` one that ran without the lock because the disk was too full to create it. */
export type CasCollectionV1=Readonly<{status:"idle"|"collected"|"locked";partial:boolean;dryRun:boolean;scanned:number;evicted:number;freedBytes:number;busy:number;skipped:number;totals:CasPlanTotalsV1;byReason:Readonly<Record<CasReasonV1,number>>;graphClosureBytes?:number;holders?:CasHolderEvictionV1;unreferencedOnly?:true;lockless?:true;lockedBy?:CasLockHolderV1;partition?:CasCollectedPartitionV1}>;

const reconciledAt=new Map<string,number>();
const STORE="store",zero:CasPlanTotalsV1={totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,capShortfallBytes:0,partitionShortfallBytes:0,needBytes:0};
const sorted=(names:string[]):string[]=>names.sort((left,right)=>left<right?-1:left>right?1:0);
const LOCK_POLL_MS=100;
const TEMPORARY=/(^\.)|(\.tmp$)/u;

type Entry={area:string;path:string;size:number;access:number;protectedEntry:boolean;expired?:boolean;shard?:string;hash?:string;pin?:string;held?:boolean;freeable?:boolean;kind?:CasObjectClassV1};
const pinOfFileV1=(path:string):string|undefined=>{const name=basename(path);return /^[0-9a-f]{64}\.json$/u.test(name)?name.slice(0,64):undefined;};

const fullDisk=(error:unknown):boolean=>["ENOSPC","EDQUOT"].includes((error as NodeJS.ErrnoException)?.code??"");
const withoutLock=async():Promise<void>=>undefined;

type LockV1={release:()=>Promise<void>;beat:(progress:number,atMs:number)=>Promise<void>;lockless:boolean};
const noBeat=async():Promise<void>=>undefined;

async function acquire(root:string,policy:CasPolicyV1,nowMs:number,pid:number,openLock:(path:string)=>Promise<FileHandle>):Promise<LockV1|undefined>{
 const lock=join(root,"v1","gc.lock");await mkdir(join(root,"v1"),{recursive:true});
 for(let attempt=0;attempt<2;attempt++){
  try{
   const handle=await openLock(lock);
   try{await handle.writeFile(JSON.stringify({...processIdentityForPidV1(pid),atMs:nowMs}));}catch(error){await handle.close().catch(()=>undefined);await unlink(lock).catch(()=>undefined);if(fullDisk(error))return{release:withoutLock,beat:noBeat,lockless:true};throw error;}
   await handle.close();
   let last=-1;
   return{release:async()=>{await unlink(lock).catch(()=>undefined);},lockless:false,beat:async(progress,atMs)=>{
    if(progress===last)return;last=progress;
    try{
     if((JSON.parse(await readFile(lock,"utf8")) as {pid:number}).pid!==pid)return;
     const temporary=`${lock}.${pid}.tmp`;await writeFile(temporary,JSON.stringify({...processIdentityForPidV1(pid),atMs}));await rename(temporary,lock);
    }catch{}
   }};
  }catch(error){
   if(fullDisk(error))return{release:withoutLock,beat:noBeat,lockless:true};
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   try{
    const held=JSON.parse(await readFile(lock,"utf8")) as {pid:number;startMs:number;atMs:number},holder=parseProcessIdentityV1(held);
    if(holder!==undefined&&processHolderAliveV1(holder)&&nowMs-held.atMs<policy.lockStaleMs)return undefined;
   }catch{}
   await unlink(lock).catch(()=>undefined);
  }
 }
 return undefined;
}

async function readLockHolderV1(root:string,nowMs:number):Promise<CasLockHolderV1|undefined>{
 try{const row=JSON.parse(await readFile(join(root,"v1","gc.lock"),"utf8")) as {pid:number;atMs:number};return typeof row.pid==="number"&&typeof row.atMs==="number"?{pid:row.pid,beatMs:Math.max(0,nowMs-row.atMs)}:undefined;}catch{return undefined;}
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

async function* entries(options:CasCollectOptionsV1,policy:CasPolicyV1,pins:ReadonlyMap<string,string>,nowMs:number,views:Map<string,CasShardViewV1>,transients:ReadonlyMap<string,number>=new Map(),held:ReadonlyMap<string,string>=new Map()):AsyncGenerator<Entry>{
 const objects=join(options.root,"v1","o");let shards:string[];
 try{shards=sorted(await readdir(objects));}catch{shards=[];}
 for(const shard of shards){
  options.signal?.throwIfAborted();let view=views.get(shard);if(!view){view=await openCasShardViewV1(options.root,shard,{nowMs:nowMs,quietMs:policy.ledgerQuietMs});views.set(shard,view);}const ledger=view.snapshot();let names:string[];
  try{names=sorted(await readdir(join(objects,shard)));}catch{continue;}
  for(const name of names){
   const path=join(objects,shard,name),hash=shard+name;let info;try{info=await lstat(path);}catch{continue;}
   if(!info.isFile())continue;
   const access=Math.max(info.mtimeMs,ledger.get(hash)??0);
   const isHeld=held.has(hash),links=info.nlink-(isHeld?casKeeperCountSyncV1(options.root,policy,hash):0);
   const until=transients.get(hash),leased=until!==undefined&&until>nowMs,expired=until!==undefined&&until<=nowMs&&access<=until&&!pins.has(hash)&&links<=1;
   const free=!pins.has(hash)&&links<=1&&!leased&&nowMs-access>=policy.minAgeMs;
   const kind:CasObjectClassV1=pins.has(hash)?"pinned":isHeld?"held":leased?"leased":links>1?"linked":expired?"expired":nowMs-access<policy.minAgeMs?"young":"unreferenced";
   yield{area:STORE,path,size:info.size,access,protectedEntry:pins.has(hash)||links>1||leased||isHeld||(!expired&&nowMs-access<policy.minAgeMs),expired,shard,hash,held:isHeld,freeable:isHeld&&free,kind};
  }
 }
 for(const area of options.areas??[]){
  for await(const file of walk(area.directory,area.skip??TEMPORARY,options.signal)){
   const access=Math.max(file.mtimeMs,file.atimeMs);
   const pin=pinOfFileV1(file.path);
   yield{area:area.id,path:file.path,size:file.size,access,protectedEntry:area.id===CAS_GRAPH_CLOSURE_AREA_V1||nowMs-access<policy.minAgeMs||(pin!==undefined&&pins.has(pin)),pin};
  }
 }
}

async function* nothing():AsyncGenerator<Entry>{}

const holderSummary=(held:CasHeldV1,blobs:ReadonlyMap<string,CasHolderPlanBlobV1>):CasHolderEvictionV1=>({holders:held.holders.length,entries:held.entries.length,heldBlobs:blobs.size,heldBytes:[...blobs.values()].reduce((sum,blob)=>sum+blob.size,0),evictedEntries:0,evictedBlobs:0,freedBytes:0,protectedEntries:0,reason:"capacity"});

/** 🫥️ The held objects the scan saw, plus every object a held entry names that the scan did not see but that is on disk (an entry published while the collector walked the store, whose objects landed in a shard already walked): alive and never freeable in this pass, so only an entry whose object is really gone counts as dead and the capacity stage never removes a unit that was just compiled. One `lstat` per unseen digest. */
async function heldOnDisk(root:string,policy:CasPolicyV1,held:CasHeldV1,scanned:ReadonlyMap<string,CasHolderPlanBlobV1&{access:number}>):Promise<Map<string,CasHolderPlanBlobV1&{access:number}>>{
 const blobs=new Map(scanned),gone=new Set<string>();
 for(const entry of held.entries)for(const digest of entry.digests){
  if(blobs.has(digest)||gone.has(digest))continue;
  const info=await lstat(join(root,"v1","o",digest.slice(0,policy.fanOutHexLength),digest.slice(policy.fanOutHexLength))).catch(()=>undefined);
  if(info?.isFile())blobs.set(digest,{size:info.size,access:info.mtimeMs,freeable:false});else gone.add(digest);
 }
 return blobs;
}

/** 🫂️ The capacity stage, only for the store's own cap: when every object nobody references is gone and the cap still needs `need` bytes, the least recently used entries of the registered holders are removed, never below the protected share `holderProtectedShare` of the cap per live holder (their files first, recorded in the holder's own eviction log with the reason `capacity`, the pid and the role of the evicting process), then the objects no remaining entry references, a pin, a lease or a hard link keeps, and the minimum age protects, so no holder is ever left with an entry whose objects were taken behind its back. A dry run only counts. */
async function evictHolders(options:CasCollectOptionsV1,policy:CasPolicyV1,held:CasHeldV1,scanned:ReadonlyMap<string,CasHolderPlanBlobV1&{access:number}>,need:number,now:()=>number):Promise<CasHolderEvictionV1>{
 const blobs=await heldOnDisk(options.root,policy,held,scanned),rows=held.entries.map(entry=>({id:entry.file,holder:entry.holder,access:entry.digests.reduce((latest,digest)=>Math.max(latest,blobs.get(digest)?.access??0),entry.digests.some(digest=>blobs.has(digest))?0:entry.mtimeMs),digests:entry.digests}));
 const floor=Math.floor(policy.holderProtectedShare*(policy.heldCapBytes??policy.capBytes)),plan=planCasHolderEvictionV1(rows,blobs,need,new Map(held.holders.map(holder=>[holder.holder,floor]))),summary={...holderSummary(held,blobs),protectedEntries:plan.protectedEntries};
 if(plan.entries.length===0)return summary;
 if(options.dryRun)return{...summary,evictedEntries:plan.entries.length,evictedBlobs:plan.digests.length,freedBytes:plan.freedBytes};
 const owner=new Map(held.entries.map(entry=>[entry.file,entry.holder])),counts=new Map<string,{entries:number;bytes:number}>();
 let removed=0;
 for(const file of plan.entries){
  options.signal?.throwIfAborted();
  try{await unlink(file);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")continue;}
  removed++;
  try{await rmdir(dirname(file));}catch{}
 }
 const fan=policy.fanOutHexLength,after=await readCasHeldV1(options.root,now(),options.signal),reclaim=await reclaimCasObjectsV1({root:options.root,policy,objectPath:hash=>join(options.root,"v1","o",hash.slice(0,fan),hash.slice(fan))},plan.digests,{now,signal:options.signal,held:after.digests});
 for(const file of plan.entries){const holder=owner.get(file);if(holder!==undefined){const row=counts.get(holder)??{entries:0,bytes:0};row.entries++;counts.set(holder,row);}}
 for(const digest of plan.digests){const holder=held.digests.get(digest),size=blobs.get(digest)?.size??0;if(holder!==undefined){const row=counts.get(holder)??{entries:0,bytes:0};row.bytes+=size;counts.set(holder,row);}}
 for(const holder of held.holders){const row=counts.get(holder.holder);if(row)await recordCasHolderEvictionV1(holder,row,now(),options.role);}
 return{...summary,evictedEntries:removed,evictedBlobs:reclaim.entries,freedBytes:reclaim.bytes};
}

async function sweepTemporary(root:string,policy:CasPolicyV1,nowMs:number):Promise<void>{
 const directory=join(root,"v1","t");let names:string[];try{names=await readdir(directory);}catch{return;}
 for(const name of names)try{if(nowMs-(await lstat(join(directory,name))).mtimeMs>policy.temporaryMaxAgeMs)await unlink(join(directory,name));}catch{}
}

/** ⏳️ Takes the lock, and while another live collector holds it waits up to `waitMs` (polling, cancellable) so that a manual collection joins the next free window instead of being refused; the holder's heartbeat proves its progress, so only a collector that stops progressing is taken over. */
async function acquireWithin(root:string,policy:CasPolicyV1,now:()=>number,pid:number,openLock:(path:string)=>Promise<FileHandle>,waitMs:number,signal?:AbortSignal):Promise<LockV1|undefined>{
 const deadline=Date.now()+waitMs;
 for(;;){
  const lock=await acquire(root,policy,now(),pid,openLock);
  if(lock||Date.now()>=deadline)return lock;
  try{await sleep(Math.min(LOCK_POLL_MS,Math.max(1,deadline-Date.now())),undefined,{signal});}catch{signal?.throwIfAborted();}
 }
}

/** 🧹️ Collects the host store and its declared areas: scans once (bounded memory, shard by shard), decides with the streaming least-recently-accessed planner (down to the low watermark of the cap and, when the caller measured the partition, down to the low used share of the partition), then evicts in a second pass that re-checks pins and fresh access right before every removal. Single-flight per root through a lock file with stale-owner takeover. Idempotent and stateless, so a crash or cancellation leaves a consistent store and the next run simply continues; objects are written by atomic rename, so no partial object is ever visible. It frees space before it needs any: stale temporary files go first, and when the disk is too full to create the lock the collection runs without it (every removal is idempotent and re-checked, so two collectors cannot do harm) and skips the ledger compaction and the usage hint, so a full disk never keeps it from deleting. */
export async function collectCasV1(options:CasCollectOptionsV1):Promise<CasCollectionV1>{
 const policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,pid=options.pid??process.pid,signal=options.signal;
 if(!options.dryRun)await sweepTemporary(options.root,policy,now());
 const lock=await acquireWithin(options.root,policy,now,pid,options.openLock??(path=>open(path,"wx")),options.waitMs??0,signal)??(options.unreferencedOnly===true?{release:withoutLock,beat:noBeat,lockless:true}:undefined),release=lock?.release;
 if(!lock||!release){const lockedBy=await readLockHolderV1(options.root,now());return{status:"locked",partial:false,dryRun:options.dryRun===true,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:zero,byReason:{age:0,area:0,host:0},...lockedBy?{lockedBy}:{}};}
 const views=new Map<string,CasShardViewV1>();let scanned=0,evicted=0,freed=0,busy=0,skipped=0,used=0,graphClosureBytes=0,lastReport=-Infinity,partial=false;const byReason:Record<CasReasonV1,number>={age:0,area:0,host:0};
 const report=async(stage:CasProgressV1["stage"],force=false):Promise<void>=>{
  signal?.throwIfAborted();const at=now();
  if(!force&&at-lastReport<policy.progressMs)return;
  lastReport=at;await lock.beat(scanned+evicted+busy+skipped,at);await options.progress?.({stage,entries:scanned,usedBytes:used,freedBytes:freed});signal?.throwIfAborted();
 };
 try{
  const scope=lock.lockless?undefined:await options.automaticScope?.();
  const finishUsage=async(bytes:number)=>{
   const exact=scope!==undefined&&!signal?.aborted&&await options.automaticScope?.()===scope&&!signal?.aborted;
   const atMs=now();
   if(await recordUsage(options,bytes,atMs,exact?scope:undefined)&&exact&&scope!==undefined)
    options.completedAutomatic?.({atMs,bytes,automaticScope:scope});
  };
  const started=now(),partitionNeed=casPartitionNeedBytesV1(options.partition,policy),partition=options.partition?Object.freeze({partition:options.partition.partition,totalBytes:options.partition.totalBytes,usedFraction:casPartitionUsedFractionV1(options.partition),needBytes:partitionNeed}):undefined,lockless=lock.lockless?{lockless:true as const}:{};
  const bounds=[{id:STORE,capBytes:null,maxAgeMs:null},...(options.areas??[]).map(area=>{const row=policy.areas.find(candidate=>candidate.id===area.id);return{id:area.id,capBytes:row?.capBytes??null,maxAgeMs:row?.maxAgeMs??null};})];
  const planner=createCasPlannerV1({policy,incremental:options.incremental===true,areas:bounds,externalBytes:options.externalBytes??0,nowMs:started,partitionNeedBytes:partitionNeed,unreferencedOnly:options.unreferencedOnly,unreferencedStepBytes:options.unreferencedStepBytes});
  let pins=await readCasPinsV1(options.root,started),transients=await readCasTransientsV1(options.root);
  const scannedEntries:CensusRowV1[]=[];resetCasHeldCacheV1();let held:CasHeldV1=await readCasHeldV1(options.root,started,signal);if(!options.dryRun&&!lock.lockless&&options.unreferencedOnly!==true&&!(started-(reconciledAt.get(options.root)??Number.NEGATIVE_INFINITY)<policy.holderRenewMs)){await reconcileCasKeepersV1(options.root,policy,held,signal).catch(()=>undefined);reconciledAt.set(options.root,started);}const heldBlobs=new Map<string,CasHolderPlanBlobV1&{access:number}>();
  try{
   for await(const entry of entries(options,policy,pins,started,views,transients,held.digests)){
    planner.add(entry.area,entry.access,entry.size,entry.protectedEntry,entry.expired);scanned++;used+=entry.size;if(entry.kind!==undefined&&entry.hash!==undefined)scannedEntries.push({hash:entry.hash,size:entry.size,kind:entry.kind});
    if(entry.held&&entry.hash!==undefined)heldBlobs.set(entry.hash,{size:entry.size,access:entry.access,freeable:entry.freeable===true});
    if(entry.area===CAS_GRAPH_CLOSURE_AREA_V1)graphClosureBytes+=entry.size;
    if(scanned%policy.checkEvery===0)await report("scan");
   }
  }catch(error){if(signal?.aborted){partial=true;}else throw error;}
  const plan=partial?undefined:planner.finish();
  if(!plan)return{status:"collected",partial:true,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals:zero,byReason,...lockless,...partition?{partition}:{}};
  const totals=plan.totals,graphClosures=graphClosureBytes>0?{graphClosureBytes}:{};
  const census=partial||options.dryRun?undefined:censusOf(scannedEntries,pins,held,started),counted=new Map(census?scannedEntries.map(row=>[row.hash,row]):[]);
  const heldNeed=options.unreferencedOnly!==true&&totals.capShortfallBytes>0&&heldBlobs.size>0;let holders:CasHolderEvictionV1|undefined=held.holders.length>0?holderSummary(held,heldBlobs):undefined;
  if(totals.plannedBytes<=0&&!heldNeed){
   await compactAll(options,policy,started,views,progress=>lock.beat(scanned+evicted+progress,now()));await finishUsage(totals.keptBytes-(options.externalBytes??0));if(census)await writeCasCensusV1(options.root,census);
   return{status:"idle",partial:false,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason,...graphClosures,...holders?{holders}:{},...options.unreferencedOnly?{unreferencedOnly:true as const}:{},...lockless,...partition?{partition}:{}};
  }
  const evaluator=plan.evaluator();if(totals.plannedBytes>0)held=await readCasHeldV1(options.root,now(),signal);pins=await readCasPinsV1(options.root,now());let transientStamp=await stampCasTransientsV1(options.root);transients=await readCasTransientsV1(options.root);let currentShard:string|undefined,view:CasShardViewV1|undefined,pinStamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;
  try{
   const pending:{entry:Entry;reason:CasReasonV1;view:CasShardViewV1|undefined}[]=[];
   const flush=async():Promise<void>=>{
    if(pending.length===0)return;
    const batch=pending.splice(0);
    for(const shardView of new Set(batch.map(row=>row.view)))await shardView?.refresh();
    {const next=await stampCasTransientsV1(options.root);if(next!==transientStamp){transientStamp=next;transients=await readCasTransientsV1(options.root);}}const stamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;if(stamp!==pinStamp){pinStamp=stamp;pins=await readCasPinsV1(options.root,now());}
    for(const {entry,reason,view:shardView} of batch){
     const fresh=entry.hash?Math.max((await lstat(entry.path).catch(()=>undefined))?.mtimeMs??-1,shardView?.latest(entry.hash)??0):(await lstat(entry.path).catch(()=>undefined))?.atimeMs??-1;
     if(fresh<0)continue;
     if(entry.hash!==undefined&&(transients.get(entry.hash)??0)>now()){skipped++;continue;}
     if(fresh>entry.access||fresh>=started||((entry.hash??entry.pin)!==undefined&&pins.has((entry.hash??entry.pin)!))){skipped++;continue;}
     try{await unlink(entry.path);evicted++;freed+=entry.size;used-=entry.size;byReason[reason]++;if(census&&entry.hash!==undefined)forgetCensusV1(census,counted,entry.hash);}
     catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")continue;if(code==="EBUSY"||code==="EPERM"||code==="EACCES"){busy++;continue;}throw error;}
    }
    await report("evict");
   };
   for await(const entry of totals.plannedBytes>0?entries(options,policy,pins,started,views,transients,held.digests):nothing()){
    if(entry.shard!==currentShard){await flush();currentShard=entry.shard;pins=await readCasPinsV1(options.root,now());{const next=await stampCasTransientsV1(options.root);if(next!==transientStamp){transientStamp=next;transients=await readCasTransientsV1(options.root);}}view=entry.shard?views.get(entry.shard):undefined;}
    if(entry.protectedEntry)continue;
    const reason=evaluator.decide(entry.area,entry.access,entry.size,entry.expired);
    if(reason===null)continue;
    if(options.dryRun){evicted++;freed+=entry.size;byReason[reason]++;continue;}
    await options.beforeEvict?.(entry.path);
    pending.push({entry,reason,view});
    if(pending.length>=policy.checkEvery)await flush();
   }
   await flush();
  }catch(error){if(signal?.aborted)partial=true;else throw error;}
  if(!partial&&heldNeed)try{
   const row=await evictHolders(options,policy,held,heldBlobs,totals.capShortfallBytes,now);
   holders=row;evicted+=row.evictedBlobs;freed+=row.freedBytes;used-=row.freedBytes;
  }catch(error){if(signal?.aborted)partial=true;else throw error;}
  if(!partial&&!options.dryRun){await compactAll(options,policy,started,views,progress=>lock.beat(scanned+evicted+progress,now()));await finishUsage(used);if(census&&!holders?.evictedBlobs)await writeCasCensusV1(options.root,census);}
  await options.progress?.({stage:"evict",entries:scanned,usedBytes:used,freedBytes:freed});
  return{status:"collected",partial,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason,...graphClosures,...holders?{holders}:{},...options.unreferencedOnly?{unreferencedOnly:true as const}:{},...lockless,...partition?{partition}:{}};
 }finally{await release();}
}

async function compactAll(options:CasCollectOptionsV1,policy:CasPolicyV1,nowMs:number,views:ReadonlyMap<string,CasShardViewV1>,beat:(progress:number)=>Promise<void>=noBeat):Promise<void>{
 if(options.dryRun)return;
 await compactCasTransientsV1(options.root,policy,nowMs,digest=>lstat(join(options.root,"v1","o",digest.slice(0,policy.fanOutHexLength),digest.slice(policy.fanOutHexLength))).then(()=>true,()=>false)).catch(()=>0);
 const objects=join(options.root,"v1","o");let shards:string[];try{shards=sorted(await readdir(objects));}catch{return;}
 let compacted=0;
 for(const shard of shards){
  options.signal?.throwIfAborted();await beat(++compacted);let names:string[];try{names=await readdir(join(objects,shard));}catch{continue;}
  const live=new Set(names.map(name=>shard+name));
  try{const view=views.get(shard);if(view)await view.refresh();await compactCasShardV1(options.root,shard,live,view?view.snapshot():await foldCasShardV1(options.root,shard),nowMs,policy);}catch(error){if(fullDisk(error))return;throw error;}
 }
}

/** 📈️ The file that tells a writer how many bytes the store and its areas held when a collection last finished, so that its admission can judge the cap without scanning a store of millions of objects. */
export const casUsageHintPathV1=(root:string):string=>join(root,"v1","usage.json");

/** 📈️ Reads the usage hint: the bytes the last finished collection left and when; `undefined` when there is none or it is unreadable. */
export type CasUsageHintV1=Readonly<{bytes:number;atMs:number;automaticScope?:string}>;
const automaticScopePattern=new RegExp(schema.definitions.UsageHint.properties.automaticScope.pattern,"u");
export async function readCasUsageHintV1(root:string):Promise<CasUsageHintV1|undefined>{
 try{
  const row=JSON.parse(await readFile(casUsageHintPathV1(root),"utf8")) as {version?:unknown;bytes?:unknown;atMs?:unknown;automaticScope?:unknown;measuredBytes?:unknown;measuredAtMs?:unknown};
  if(row.version!==1||typeof row.bytes!=="number"||!Number.isSafeInteger(row.bytes)||row.bytes<0||typeof row.atMs!=="number"||!Number.isSafeInteger(row.atMs)||row.atMs<=0)return undefined;
  if(typeof row.measuredBytes==="number"&&Number.isSafeInteger(row.measuredBytes)&&row.measuredBytes>=0&&typeof row.measuredAtMs==="number"&&Number.isSafeInteger(row.measuredAtMs)&&row.measuredAtMs>row.atMs)return Object.freeze({bytes:row.measuredBytes,atMs:row.measuredAtMs});
  return Object.freeze({bytes:row.bytes,atMs:row.atMs,...typeof row.automaticScope==="string"&&automaticScopePattern.test(row.automaticScope)?{automaticScope:row.automaticScope}:{}});
 }catch{return undefined;}
}

/** 📈️ Writes a plain measurement of the store next to the hint of the last collection, so every reader (admission, blob bound, status) judges the cap on bytes no older than the last walk instead of on the last finished collection, which a store above its cap may not finish for an hour; returns whether the file was written. */
export async function recordCasMeasurementV1(root:string,bytes:number,atMs:number):Promise<boolean>{
 const target=casUsageHintPathV1(root),temporary=`${target}.${process.pid}.measure.tmp`;
 try{
  const row=await readFile(target,"utf8").then(text=>JSON.parse(text) as Record<string,unknown>,()=>undefined),measured=Math.max(0,Math.round(bytes));
  const next=row?.version===1&&typeof row.bytes==="number"&&typeof row.atMs==="number"?{...row,measuredBytes:measured,measuredAtMs:atMs}:{version:1,bytes:measured,atMs};
  await writeFile(temporary,JSON.stringify(next));await rename(temporary,target);return true;
 }catch{await unlink(temporary).catch(()=>undefined);return false;}
}

async function recordUsage(options:CasCollectOptionsV1,bytes:number,atMs:number,automaticScope?:string):Promise<boolean>{
 if(options.dryRun)return false;
 const target=casUsageHintPathV1(options.root),temporary=`${target}.${process.pid}.tmp`;
 try{await writeFile(temporary,JSON.stringify({version:1,bytes:Math.max(0,Math.round(bytes)),atMs,...automaticScope===undefined?{}:{automaticScope}}));await rename(temporary,target);return true;}
 catch{await unlink(temporary).catch(()=>undefined);return false;}
}

/** 🏷️ Why an object of the store is still there: a live pin, a live entry of a registered holder, a live input lease, a foreign hard link, an ended lease (first to go), the in-flight grace, or nothing at all. */
export type CasObjectClassV1="pinned"|"held"|"leased"|"linked"|"expired"|"young"|"unreferenced";
/** 📊️ The counters of the store as its last complete scan found them, minus what that collection removed: objects and bytes per class and per holder (an object counts for the first holder naming it), so a status answers in milliseconds without walking the store. @see ../🧬️schema/🔣️.json */
export type CasCensusV1=Readonly<{version:1;atMs:number;objects:number;bytes:number;classes:Record<CasObjectClassV1,{objects:number;bytes:number}>;holders:{kind:"pin"|"unit-cache";holder:string;label:string;entries:number;objects:number;bytes:number}[]}>;
/** 🧮️ One object as the census scan counted it. */
export type CensusRowV1={hash:string;size:number;kind:CasObjectClassV1};
const CAS_OBJECT_CLASSES_V1:readonly CasObjectClassV1[]=["pinned","held","leased","linked","expired","young","unreferenced"];

function censusOf(rows:readonly CensusRowV1[],pins:ReadonlyMap<string,string>,held:CasHeldV1,atMs:number):CasCensusV1{
 const classes=Object.fromEntries(CAS_OBJECT_CLASSES_V1.map(kind=>[kind,{objects:0,bytes:0}])) as CasCensusV1["classes"],holders=new Map<string,CasCensusV1["holders"][number]>();
 for(const holder of held.holders)holders.set(holder.holder,{kind:"unit-cache",holder:holder.holder,label:holder.root,entries:held.entries.filter(entry=>entry.holder===holder.holder).length,objects:0,bytes:0});
 let objects=0,bytes=0;
 for(const row of rows){
  objects++;bytes+=row.size;classes[row.kind].objects++;classes[row.kind].bytes+=row.size;
  const owner=row.kind==="pinned"?pins.get(row.hash):row.kind==="held"?held.digests.get(row.hash):undefined;if(owner===undefined)continue;
  let total=holders.get(owner);if(total===undefined)holders.set(owner,total={kind:"pin",holder:owner,label:owner,entries:0,objects:0,bytes:0});
  total.objects++;total.bytes+=row.size;
 }
 return{version:1,atMs,objects,bytes,classes,holders:[...holders.values()].sort((left,right)=>right.bytes-left.bytes)};
}
/** ➖️ Takes one removed object out of the census under the class and size the census scan counted it with; an object the scan never counted (stored since) changes nothing, so a class never goes below zero. */
export function forgetCensusV1(census:CasCensusV1,counted:Map<string,CensusRowV1>,hash:string):void{
 const known=counted.get(hash);if(known===undefined)return;counted.delete(hash);
 const row=census.classes[known.kind];row.objects--;row.bytes-=known.size;(census as {objects:number}).objects--;(census as {bytes:number}).bytes-=known.size;
}

/** 📊️ Where the census of a store lives. */
export {casCensusPathV1};
/** 📊️ Writes the census atomically; best effort, a full disk only loses the counters. */
export async function writeCasCensusV1(root:string,census:CasCensusV1):Promise<void>{
 const target=casCensusPathV1(root),temporary=`${target}.${process.pid}.tmp`;
 try{await writeFile(temporary,JSON.stringify(census));await rename(temporary,target);}catch{await unlink(temporary).catch(()=>undefined);}
}
/** 📊️ Reads the census of the last complete scan; `undefined` when none was written or it is unreadable. */
export async function readCasCensusV1(root:string):Promise<CasCensusV1|undefined>{
 try{const row=JSON.parse(await readFile(casCensusPathV1(root),"utf8")) as CasCensusV1;return row.version===1&&Number.isSafeInteger(row.atMs)&&Number.isSafeInteger(row.bytes)&&CAS_OBJECT_CLASSES_V1.every(kind=>typeof row.classes?.[kind]?.bytes==="number")&&Array.isArray(row.holders)?row:undefined;}catch{return undefined;}
}

/** 📣️ The one-line report of a collection in the requested language. */
export function formatCasCollectionV1(result:CasCollectionV1,policy:CasPolicyV1,language:"en"|"de"):string{
 const percent=(fraction:number):number=>Math.round(fraction*1000)/10;
 const mib=(bytes:number):number=>Math.round(bytes/1048576*10)/10,gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100;
 if(result.status==="locked")return result.lockedBy?casTextV1("lockedBy",language,{pid:result.lockedBy.pid,beatSeconds:Math.round(result.lockedBy.beatMs/1000)}):casTextV1("locked",language);
 if(result.unreferencedOnly)return casTextV1(result.dryRun?"previewUnreferenced":"doneUnreferenced",language,{objects:result.evicted,gib:gib(result.freedBytes),kept:Math.max(0,result.scanned-result.evicted),graceSeconds:Math.round(policy.minAgeMs/1000),partial:result.partial?casTextV1("partial",language):""});
 const partition=result.partition&&result.partition.needBytes>0?result.partition:undefined,policyLine=partition?casTextV1("partitionState",language,{partition:partition.partition,usedPercent:percent(partition.usedFraction),highPercent:percent(policy.partition.highUsedFraction),lowPercent:percent(policy.partition.lowUsedFraction)}):"";
 if(result.status==="idle"&&!partition&&result.totals.shortfallBytes<=0)return casTextV1("idle",language,{usedGib:gib(result.totals.totalBytes),capGib:gib(policy.capBytes)});
 const text=casTextV1(result.dryRun?"previewCollection":"done",language,{evicted:result.evicted,freedMib:mib(result.freedBytes),keptGib:gib(result.totals.keptBytes),protectedMib:mib(result.totals.protectedBytes),partial:result.partial?casTextV1("partial",language):""})+policyLine;
 const evicted=result.holders&&result.holders.evictedEntries>0?" "+casTextV1("holderEvicted",language,{entries:result.holders.evictedEntries,holders:result.holders.holders,blobs:result.holders.evictedBlobs,mib:mib(result.holders.freedBytes)}):"";
 const remaining=Math.max(0,result.totals.capShortfallBytes-(result.holders?.freedBytes??0));
 const shielded=result.holders&&result.holders.protectedEntries>0&&remaining>0?" "+casTextV1("holderProtected",language,{entries:result.holders.protectedEntries,floorGib:gib(Math.floor(policy.holderProtectedShare*(policy.heldCapBytes??policy.capBytes)))}):"";
 const pressure=result.partition&&result.totals.partitionShortfallBytes>0?" "+casTextV1("partitionNotStore",language,{usedPercent:percent(result.partition.usedFraction),storeGib:gib(result.totals.keptBytes),sharePercent:percent(result.totals.keptBytes/result.partition.totalBytes),overGib:gib(result.totals.partitionShortfallBytes)}):"";
 if(remaining<=0)return text+evicted+pressure;
 return text+evicted+shielded+" "+casTextV1("shortfall",language,{overMib:mib(remaining)})+(result.graphClosureBytes?casTextV1("graphClosures",language,{protectedMib:mib(result.graphClosureBytes)}):"")+pressure;
}
