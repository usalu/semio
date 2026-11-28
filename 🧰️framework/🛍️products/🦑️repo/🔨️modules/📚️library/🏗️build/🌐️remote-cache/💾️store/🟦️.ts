import {createHash,randomUUID} from "node:crypto";
import {appendFile,link,mkdir,open,readdir,readFile,rename,rm,stat,unlink,utimes,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {requireRemoteCacheShapeV1} from "../🟦️.ts";
import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import type {RemoteCacheBlobRowV1,RemoteCacheDropV1} from "../🧹️collector/🟦️.ts";
import {acquireRemoteCacheLockV1,judgeRemoteCacheStalenessV1,liveRemoteCacheReferencesV1,planRemoteCacheBlobBoundV1,planRemoteCacheCollectionV1,REMOTE_CACHE_COLLECTOR_LOCK_V1,removeRemoteCacheEntryV1,scanRemoteCacheDirectoryV1,type RemoteCacheStepV1} from "../🧹️collector/🟦️.ts";
import {parseRemoteCacheActionResultV1} from "../🔬️action-result/🟦️.ts";
import {readRemoteCacheReferencesV1,removeRemoteCacheMetaV1,scanRemoteCacheMetaV1,writeRemoteCacheMetaV1,type RemoteCacheMetaV1} from "../🏷️retention/🟦️.ts";
import {protectedKindV1,requireSlotShapeV1,SLOT_GRACE_MILLISECONDS_V1,type DeclarationV1,type StaleCauseV1,type StalenessModeV1,type TombstoneV1} from "../../🪢️slots/🟦️.ts";
import {stalenessOnArrivalV1,stalenessReportV1,type StalenessReasonV1,type StalenessVerdictV1,type TreeDigestsV1} from "../🏷️retention/🍂️staleness/🟦️.ts";
import {reclaimCasObjectsV1} from "../../🗄️cas/🪦️reclaim/🟦️.ts";
import {CasCapRefusalV1} from "../../🗄️cas/🚦️guard/🟦️.ts";
import {remoteCacheFileBudgetOfMachineV1,remoteCacheWidthV1} from "../📏️fd-budget/🟦️.ts";
import {ownProcessIdentityV1,processHolderAliveV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

/** 🗂️ Names the three keyspaces one store partitions: Bazel action results, Bazel content blobs and Nx task archives; the only place that spells them, so every other module reads them from here. */
export const REMOTE_CACHE_KINDS_V1={ac:"ac",cas:"cas",nx:"nx"} as const;

/** 🧺️ One keyspace name of {@link REMOTE_CACHE_KINDS_V1}. */
export type RemoteCacheKindV1=typeof REMOTE_CACHE_KINDS_V1[keyof typeof REMOTE_CACHE_KINDS_V1];

/** 📮️ Reports why one write ended: stored, replaced an older value, found an equal or protected value, or was refused. */
export type RemoteCachePutOutcomeV1="created"|"replaced"|"exists"|"digest-mismatch"|"length-mismatch"|"too-large"|"capacity"|"unavailable"|"aborted";

const hostPutFailureV1=(error:unknown):RemoteCachePutOutcomeV1=>{const row=error as {code?:unknown;key?:unknown}|null;return error instanceof CasCapRefusalV1||row?.code==="ENOSPC"||row?.code==="EDQUOT"||(typeof row?.key==="string"&&row.key.endsWith("disk.low-space"))?"capacity":"unavailable";};

/** 🚨️ The shortest time between two collections that a capacity refusal of the host store asks for: the refusal says the store is at its cap in force, the next upload of the same unit comes seconds later (`retry-after`), so the room is made now and not when the admitted bytes next reach their trigger (five percent of the bound, at most every thirty seconds). */
const CAPACITY_RELIEF_MILLISECONDS_V1=5000;

/** 📡️ Progress and housekeeping events of one store, published to the caller without any logging dependency. */
export type RemoteCacheStoreEventV1=Readonly<{stage:"scan";entries:number;bytes:number}>|Readonly<{stage:"evict";entries:number;bytes:number}>|Readonly<{stage:"reclaim";entries:number;bytes:number}>|Readonly<{stage:"ready";entries:number;bytes:number}>;

/** 🥀️ How a store judges staleness: the mode (`off`, `dry-run` only records, `enforce` retires and refuses), the digests of the synchronized tree, the toolchain identities that are the fleet's (`undefined` judges none), how many examples a report keeps, the milliseconds after which an entry that looks stale on arrival is looked at again and the manifest names of an action result that carry a declaration. */
export type RemoteCacheStalenessOptionsV1=Readonly<{mode:StalenessModeV1;tree:TreeDigestsV1;toolchains?:()=>ReadonlySet<string>|undefined;examples:number;recheckMilliseconds:number;manifestNames:readonly string[]}>;

/** 🧱️ Authors the finite bounds of one store: total bytes, bytes per entry and the idle age after which an entry without a slot is dropped (0 keeps entries until space is needed), the grace in milliseconds that protects entries of builds in flight (default ten minutes), the staleness policy (absent judges nothing), the predicate of entries an active claim holds and a hook that tests use to simulate a crash between the steps of a removal. */
export type RemoteCacheStoreOptionsV1=Readonly<{fileBudget?:number;scanRetryMilliseconds?:number;writeBudgetMilliseconds?:number;directory:string;startupScan?:"await"|"background";maximumBytes:number;maximumEntryBytes:number;maximumAgeMilliseconds:number;host?:CasStoreV1;pinBudgetBytes?():number|undefined;blobBoundBytes?():Promise<number|undefined>|number|undefined;graph?:Readonly<{sharePercent:number;supersedeGraceMilliseconds:number}>;graceMilliseconds?:number;claimed?:(namespace:string,kind:RemoteCacheKindV1,key:string)=>boolean;staleness?:RemoteCacheStalenessOptionsV1;onStep?:(point:RemoteCacheStepV1)=>void;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1)=>void}>;

/** 📏️ Describes the current fill of one store. */
export type RemoteCacheStoreUsageV1=Readonly<{entries:number;bytes:number;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;slotted:number;evictedEntries:number;evictedBytes:number;blobs:Readonly<{referencedBytes:number|null;boundBytes:number|null;evictedBytes:number;graph:Readonly<{protectedEntries:number;protectedBytes:number;reservedBytes:number|null;evictedEntries:number;supersededBytes:number}>}>}>;

/** 🪢️ What a write says about its entry: the slot id (a dedupe hint), for action results the blobs they reference (Nx entries reference their host store body) and what the entry declared it was built from. */
export type RemoteCachePutSlotV1=Readonly<{slot:string;refs?:readonly string[];declaration?:DeclarationV1}>;

/** 📤️ One stored value ready to be streamed. */
export type RemoteCacheEntryV1=Readonly<{path:string;size:number}>;

/** 🔑️ Names how a write treats an existing value: `replace` overwrites, `keep` refuses to overwrite and `content` is idempotent because the key is the content digest. */
export type RemoteCachePutModeV1="replace"|"keep"|"content";

/** 🪢️ One entry brought back to current by `claim`. */
export type RemoteCacheClaimOutcomeV1="claimed"|"missing";

const NAMESPACE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,KEYS:Readonly<Record<RemoteCacheKindV1,RegExp>>={ac:/^[0-9a-f]{64}$/u,cas:/^[0-9a-f]{64}$/u,nx:/^[a-z0-9_-]{1,128}$/u};


/** 🔎️ Admits only namespaces and keys that map to exactly one portable file name. */
export function requireRemoteCacheAddressV1(namespace:string,kind:RemoteCacheKindV1,key:string):void{
 if(!NAMESPACE.test(namespace))throw Error("Lowercase remote cache namespace required");
 if(!KEYS[kind]?.test(key))throw Error(`Remote cache key for ${kind} required`);
}

/** 🏛️ Stores remote cache values as files in a bounded directory with atomic writes, content verification and least-recently-used eviction; one process owns a directory at a time, other processes may only collect it. */
export class RemoteCacheStoreV1{
 private readonly index=new Map<string,{size:number;accessed:number}>();
 private readonly slots=new Map<string,RemoteCacheMetaV1>();
 private readonly inflight=new Set<string>();
 private bytes=0;
 private evictedEntries=0;
 private evictedBytes=0;
 private evictedBlobBytes=0;
 private relievedAt=0;
 private referencedBlobBytes:number|null=null;
 private lastBound:number|null=null;
 private graphProtected:Readonly<{keys:ReadonlySet<string>;bytes:number}>={keys:new Set(),bytes:0};
 private graphEvicted=0;
 private readonly superseded=new Map<string,number>();
 private admitted=0;
 private readonly born=new Map<string,number>();
 private lastTrigger=0;
 private readonly refCache=new Map<string,readonly string[]>();
 private readonly blobSizes=new Map<string,number>();
 private reserved=0;
 private clock=0;
 private collecting:Promise<void>=Promise.resolve();
 private verdicts:ReadonlyMap<string,StalenessVerdictV1>|undefined;
 private report:ReturnType<typeof stalenessReportV1>|undefined;
 private readonly refused:Record<string,number>={};
 private retired=new Map<string,StaleCauseV1>();
 private readonly writing=join(".tmp",`${process.pid}-${ownProcessIdentityV1().startMs}-${randomUUID()}`);
 private readonly options:RemoteCacheStoreOptionsV1; private constructor(options:RemoteCacheStoreOptionsV1){this.options=options;}
 private get grace():number{return this.options.graceMilliseconds??SLOT_GRACE_MILLISECONDS_V1;}
 private storedClock=0;
 private stamp():number{return this.storedClock=Math.max(Date.now(),this.storedClock+1);}

 /** 🚪️ Opens the directory, drops the partial writes of dead processes and indexes the stored values, publishing progress and stopping when cancelled. */
 static async open(options:RemoteCacheStoreOptionsV1):Promise<RemoteCacheStoreV1>{
  requireRemoteCacheShapeV1("StoreBounds",Object.fromEntries(Object.entries({maximumBytes:options.maximumBytes,maximumEntryBytes:options.maximumEntryBytes,maximumAgeMilliseconds:options.maximumAgeMilliseconds}).filter(([,value])=>value!==undefined)),"Finite remote cache bounds required");
  if(options.maximumEntryBytes>options.maximumBytes)throw Error("Finite remote cache bounds required: an entry cannot exceed the store");
  const store=new RemoteCacheStoreV1(options);
  await mkdir(store.root,{recursive:true});await mkdir(join(store.root,".tmp"),{recursive:true});
  try{for(const row of JSON.parse(await readFile(join(store.root,".retention","retired.json"),"utf8")) as unknown[]){try{requireSlotShapeV1("Tombstone",row,"tombstone");const{blob,...cause}=row as TombstoneV1;store.retired.set(blob,cause as StaleCauseV1);}catch{}}}catch{}
  for(const name of await readdir(join(store.root,".tmp")))if(!/^\d+-\d+-/u.test(name)||!processHolderAliveV1({pid:Number.parseInt(name,10),startMs:Number(name.split("-")[1])}))await rm(join(store.root,".tmp",name),{recursive:true,force:true});
  await mkdir(store.temporary,{recursive:true});
  const scanned=store.exclusively(()=>store.refresh(options.startupScan==="background"?4:32)).then(()=>{options.onEvent?.({stage:"ready",entries:store.index.size,bytes:store.bytes});});
  if(options.startupScan==="background")void scanned.catch(()=>undefined);else await scanned;
  return store;
 }

 /** 📁️ The versioned root below the store directory. */
 get root():string{return join(this.options.directory,"v1");}
 private get temporary():string{return join(this.root,this.writing);}
 private relative(namespace:string,kind:RemoteCacheKindV1,key:string):string{return `${namespace}/${kind}/${key.slice(0,2)}/${key}`;}
 private path(relative:string):string{return join(this.root,...relative.split("/"));}
 private checkpoint():void{this.options.signal?.throwIfAborted();}
 private tick():number{return this.clock=Math.max(Date.now(),this.clock+1);}


 private referenced(namespace:string,kind:RemoteCacheKindV1):boolean{return kind===REMOTE_CACHE_KINDS_V1.nx&&this.options.host!==undefined;}
 private async reference(relative:string):Promise<string|undefined>{
  const path=this.path(relative);let text;try{text=await readFile(path,"utf8");}catch{return undefined;}
  try{const row=JSON.parse(text) as {hash?:unknown};if(typeof row.hash==="string"&&/^[0-9a-f]{64}$/u.test(row.hash))return row.hash;}catch{}
  await unlink(path).catch(()=>undefined);return undefined;
 }
 private async resolve(namespace:string,kind:RemoteCacheKindV1,key:string):Promise<RemoteCacheEntryV1|undefined>{
  const relative=this.relative(namespace,kind,key),hash=await this.reference(relative);
  if(hash===undefined)return undefined;
  const entry=await this.options.host!.get(hash);
  if(entry===undefined){await unlink(this.path(relative)).catch(()=>undefined);return undefined;}
  const now=this.tick()/1000,row=this.index.get(relative);if(row)row.accessed=this.clock;
  void utimes(this.path(relative),now,now).catch(()=>{});
  return Object.freeze({path:entry.path,size:entry.size});
 }

 /** 🔄️ Rebuilds the index from the files on disk, so values collected or removed by another process stop counting, then collects what exceeds the bounds. */
 /** 🔒️ Runs a full scan only while no other process of this directory scans (the successor of a handover and the process it replaces never scan at the same time): the collector lock is the turn, taken again every `scanRetryMilliseconds` until it is free. */
 async exclusively(work:()=>Promise<void>):Promise<void>{
  for(;;){
   this.checkpoint();
   const lock=await acquireRemoteCacheLockV1(this.root,REMOTE_CACHE_COLLECTOR_LOCK_V1);
   if(!lock.held){try{await work();}finally{await lock.release();}return;}
   await new Promise<void>(done=>{const wait=setTimeout(done,this.options.scanRetryMilliseconds??2000);wait.unref();});
  }
 }

 async refresh(requested:number=4):Promise<void>{
  const width=remoteCacheWidthV1(requested,this.options.fileBudget??remoteCacheFileBudgetOfMachineV1());
  const began=Date.now(),found=await scanRemoteCacheDirectoryV1(this.root,{concurrency:width,...this.options.signal?{signal:this.options.signal}:{},onProgress:(_stage,entries,bytes)=>this.options.onEvent?.({stage:"scan",entries,bytes})});
  const metas=await scanRemoteCacheMetaV1(this.root,{has:(relative:string)=>found.has(relative)||this.index.has(relative)},this.options.signal,width);
  const touched=[...this.index].filter(([,row])=>row.accessed>=began),stored=[...this.slots].filter(([,meta])=>meta.storedAt>=began);
  this.index.clear();this.slots.clear();this.bytes=0;
  for(const[relative,row]of found)this.index.set(relative,row);
  for(const[relative,row]of touched)this.index.set(relative,row);
  for(const row of this.index.values()){this.clock=Math.max(this.clock,row.accessed);this.bytes+=row.size;}
  for(const[relative,meta]of [...metas,...stored]){this.slots.set(relative,meta);this.storedClock=Math.max(this.storedClock,meta.storedAt);}
  for(const key of this.refCache.keys())if(!this.index.has(key))this.refCache.delete(key);
  for(const key of this.born.keys())if(!this.index.has(key))this.born.delete(key);
  await this.collect();
 }

 /** 📊️ Reports the current fill. */
 usage():RemoteCacheStoreUsageV1{return Object.freeze({entries:this.index.size,bytes:this.bytes,reservedBytes:this.reserved,maximumBytes:this.options.maximumBytes,maximumEntryBytes:this.options.maximumEntryBytes,slotted:this.slots.size,evictedEntries:this.evictedEntries,evictedBytes:this.evictedBytes,blobs:{referencedBytes:this.referencedBlobBytes,boundBytes:this.lastBound,evictedBytes:this.evictedBlobBytes,graph:{protectedEntries:this.graphProtected.keys.size,protectedBytes:this.graphProtected.bytes,reservedBytes:this.graphReserve(this.lastBound),evictedEntries:this.graphEvicted,supersededBytes:this.supersededBytes()}}});}
 private graphReserve(bound:number|null):number|null{const graph=this.options.graph;return bound===null||graph===undefined?null:Math.floor(bound*graph.sharePercent/100);}
 private supersededBytes():number{let bytes=0;const now=Date.now();for(const [hash,until] of this.superseded)if(until>now)bytes+=this.blobSizes.get(hash)??0;return bytes;}
 private isGraph(relative:string):boolean{return protectedKindV1(this.slots.get(relative)?.declaration?.kind);}

 /** 🔍️ Looks one value up on disk and marks it recently used; a value another process removed is forgotten. */
 async read(namespace:string,kind:RemoteCacheKindV1,key:string):Promise<RemoteCacheEntryV1|undefined>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  if(kind==="cas"&&this.options.host)return this.options.host.get(key);
  if(this.referenced(namespace,kind))return this.resolve(namespace,kind,key);
  const relative=this.relative(namespace,kind,key),path=this.path(relative);let info;
  try{info=await stat(path);}catch(error){
   if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;
   const gone=this.index.get(relative);
   if(gone){this.index.delete(relative);this.bytes-=gone.size;}
   return undefined;
  }
  const now=this.tick(),row=this.index.get(relative);
  if(row){this.bytes+=info.size-row.size;row.size=info.size;row.accessed=now;}else{this.index.set(relative,{size:info.size,accessed:now});this.bytes+=info.size;}
  void utimes(path,now/1000,now/1000).catch(()=>{});
  return Object.freeze({path,size:info.size});
 }

 /** 📖️ Reads one small value completely and marks it recently used; `undefined` when it is absent or larger than the limit. */
 async readBytes(namespace:string,kind:RemoteCacheKindV1,key:string,limit:number):Promise<Uint8Array|undefined>{
  if(kind==="cas"&&this.options.host){const entry=await this.options.host.get(key);return entry&&entry.size<=limit?this.options.host.read(key).then(bytes=>bytes):undefined;}
  const entry=await this.read(namespace,kind,key);
  if(!entry||entry.size>limit)return undefined;
  try{return await readFile(entry.path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
 }

 /** 🧪️ Tells whether a value exists without touching it; a value another process or the host collector removed reads as absent and is forgotten. */
 async has(namespace:string,kind:RemoteCacheKindV1,key:string):Promise<boolean>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  if(kind==="cas"&&this.options.host)return (await this.options.host.has([key]))[0]===true;
  if(this.referenced(namespace,kind))return (await this.resolve(namespace,kind,key))!==undefined;
  const relative=this.relative(namespace,kind,key);
  try{await stat(this.path(relative));return true;}catch(error){
   if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;
   const gone=this.index.get(relative);if(gone){this.index.delete(relative);this.bytes-=gone.size;}
   return false;
  }
 }

 /** 🗑️ Removes one value; a missing value is not an error. */
 async remove(namespace:string,kind:RemoteCacheKindV1,key:string):Promise<void>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  if(kind==="cas"&&this.options.host)return;
  const relative=this.relative(namespace,kind,key),row=this.index.get(relative);
  try{await unlink(this.path(relative));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(row){this.index.delete(relative);this.bytes-=row.size;}
  this.slots.delete(relative);this.refCache.delete(relative);await removeRemoteCacheMetaV1(this.root,relative).catch(()=>undefined);
 }

 /** 🧹️ Plans and removes, in this order: unusable values, values that are stale against the synchronized tree or unreached (in enforce mode only; never one an active claim holds or one stored within the grace), values that declared nothing idle longer than the authored age, then the least valuable values (stale or unreached, undeclared, live; least recently used first) until the fill is at most nine tenths of the bound; every sweep records what it found (and, in dry-run mode, what it would retire) for `stalenessStatus`; afterwards it offers the blobs no remaining entry references to the host store, which keeps every blob a live pin, a recent access or a workspace hard link protects. Calls queue behind each other. */
 collect(need:number=0):Promise<void>{
  const run=async()=>{
   const claimed=(relative:string):boolean=>{
    if(this.inflight.has(relative))return true;
    if(!this.options.claimed)return false;
    const [namespace,kind,,key]=relative.split("/");return this.options.claimed(namespace!,kind as RemoteCacheKindV1,key!);
   };
   const now=Date.now(),judged=await this.judge(now),staleness=this.options.staleness;
   const plan=planRemoteCacheCollectionV1([...this.index].map(([key,row])=>({key,...row})),{maximumBytes:this.options.maximumBytes,maximumAgeMilliseconds:this.options.maximumAgeMilliseconds,now,need,reserved:this.reserved},new Set(),{metas:this.slots,graceMilliseconds:this.grace,claimed:relative=>claimed(relative)||this.isGraph(relative),...judged&&staleness?.mode==="enforce"?{verdicts:judged}:{}});
   const blobDrops=await this.planBlobBound(plan,judged,claimed);
   let removedEntries=0,removedBytes=0,retiredNow=0;const gone:string[]=[],refs=new Set<string>(),tombstones=new Map<string,StaleCauseV1>(),graphRows:Record<string,unknown>[]=[],reasons:Record<string,number>={};
   for(const drop of [...plan,...blobDrops]){
    this.checkpoint();
    const meta=this.slots.get(drop.key),own=meta?.refs??this.refCache.get(drop.key);let outcome;
    try{outcome=await removeRemoteCacheEntryV1(this.root,drop,{...this.options.onStep?{onStep:this.options.onStep}:{},claimed,graceMilliseconds:this.grace});}catch(error){if(/^E[A-Z]+$/u.test(String((error as NodeJS.ErrnoException).code)))continue;throw error;}
    if(outcome==="kept")continue;
    const row=this.index.get(drop.key),graph=protectedKindV1(meta?.declaration?.kind);
    if(graph){this.graphEvicted++;graphRows.push({version:1,atMs:now,kind:"graph-entry",key:drop.key,reason:drop.reason,tier:meta?.declaration===undefined?1:2,bytes:drop.size,accessed:row?.accessed??null,storedAt:meta?.storedAt??null,slot:meta?.slot??null});}
    reasons[drop.reason]=(reasons[drop.reason]??0)+1;
    if(row){this.index.delete(drop.key);this.bytes-=row.size;}
    this.slots.delete(drop.key);this.refCache.delete(drop.key);gone.push(drop.key);const cause=drop.reason==="stale"&&judged?.get(drop.key)?.state==="stale"?judged.get(drop.key)!.cause:undefined;if(own)for(const hash of own){refs.add(hash);if(cause!==undefined)tombstones.set(hash,cause);}
    if(drop.reason==="stale")retiredNow++;
    removedEntries++;removedBytes+=drop.size;
   }
   if(judged&&staleness)await this.record(now,judged,retiredNow,tombstones);
   if(removedEntries>0)await this.journal([{version:1,atMs:now,kind:"collection",entries:removedEntries,bytes:removedBytes,reasons,graphEntries:graphRows.length,remaining:this.index.size,referencedBlobBytes:this.referencedBlobBytes,boundBytes:this.lastBound},...graphRows]);
   if(removedEntries){this.evictedEntries+=removedEntries;this.evictedBytes+=removedBytes;this.options.onEvent?.({stage:"evict",entries:removedEntries,bytes:removedBytes});}
   if(this.options.host&&refs.size>0){
    const live=await liveRemoteCacheReferencesV1(this.root,this.index.keys(),this.slots,this.options.signal,(namespace,tree)=>this.readBytes(namespace,"cas",tree.hash,Math.min(tree.size,16*1048576))),orphaned=[...refs].filter(hash=>!live.has(hash));
    if(orphaned.length>0){
     const freed=await reclaimCasObjectsV1(this.options.host,orphaned,{...this.options.signal?{signal:this.options.signal}:{}});
     if(freed.entries>0)this.options.onEvent?.({stage:"reclaim",entries:freed.entries,bytes:freed.bytes});
    }
   }
   await this.pinLive().catch(()=>undefined);
  };
  this.collecting=this.collecting.then(run,run);return this.collecting;
 }

 private async journal(rows:readonly Record<string,unknown>[]):Promise<void>{
  try{
   const directory=join(this.root,".retention");await mkdir(directory,{recursive:true});
   const log=join(directory,"evictions.jsonl");
   if((await stat(log).catch(()=>undefined))?.size!>5*1048576)await rename(log,log+".1");
   await appendFile(log,rows.map(row=>JSON.stringify(row)).join("\n")+"\n");
  }catch{}
 }

 private sizeTrigger():void{
  const now=Date.now();
  if(this.lastBound===null||this.admitted<this.lastBound*0.05||now-this.lastTrigger<30000)return;
  this.admitted=0;this.lastTrigger=now;void this.collect().catch(()=>undefined);
 }

 /** 🚨️ Makes room after the host store refused a write at its cap in force: one collection (the blob bound included) at most once per `CAPACITY_RELIEF_MILLISECONDS_V1`, in the background; the refused write is answered 507 as before and its retry meets the room. */
 private relieve():void{
  const now=Date.now();if(now-this.relievedAt<CAPACITY_RELIEF_MILLISECONDS_V1)return;
  this.relievedAt=now;void this.collect().catch(()=>undefined);
 }
 private async blobSize(hash:string):Promise<number>{
  const known=this.blobSizes.get(hash);if(known!==undefined)return known;
  const size=await stat(this.options.host!.objectPath(hash)).then(info=>info.size,()=>undefined);
  if(size===undefined)return 0;this.blobSizes.set(hash,size);return size;
 }
 private async entryRefs(relative:string):Promise<readonly string[]>{
  const meta=this.slots.get(relative);if(meta)return meta.refs;
  const known=this.refCache.get(relative);if(known)return known;
  const kind=relative.split("/")[1] as RemoteCacheKindV1,found=await this.referencesOf(relative,kind).catch(()=>[] as readonly string[]);
  this.refCache.set(relative,found);return found;
 }

 /** 📐️ Keeps the bytes of the blobs the entries reference within the bound that follows the host store's cap in force: the entries of the plan are already leaving, so when the rest still references more than the bound the least valuable entries (stale or unreached, undeclared, declared; least recently used first) leave too, down to nine tenths of it; this is the size budget and applies in every retention mode, it never judges staleness. Without a host store or a bound nothing is planned. */
 private async planBlobBound(plan:readonly RemoteCacheDropV1[],judged:ReadonlyMap<string,StalenessVerdictV1>|undefined,claimed:(relative:string)=>boolean):Promise<readonly RemoteCacheDropV1[]>{
  const host=this.options.host,bound=host===undefined?undefined:await this.options.blobBoundBytes?.();
  if(host===undefined||bound===undefined){this.lastBound=null;this.referencedBlobBytes=null;return [];}
  this.lastBound=bound;
  const leaving=new Set(plan.map(drop=>drop.key)),enforcing=this.options.staleness?.mode==="enforce",now=Date.now(),rows:RemoteCacheBlobRowV1[]=[],sizes=new Map<string,number>();
  for(const [relative,row] of this.index){
   if(leaving.has(relative)||relative.split("/")[1]==="cas")continue;
   const refs=await this.entryRefs(relative),state=judged?.get(relative)?.state,stored=this.slots.get(relative)?.storedAt;
   for(const hash of refs)if(!sizes.has(hash))sizes.set(hash,await this.blobSize(hash));
   const meta=this.slots.get(relative),slotted=meta!==undefined&&protectedKindV1(meta.declaration?.kind)&&meta.slot!==undefined&&meta.storedAt!==undefined;
   rows.push({key:relative,accessed:row.accessed,refs,tier:state==="unreached"||(state==="stale"&&enforcing)?0:state===undefined||state==="undeclared"?1:2,guarded:claimed(relative)||now-(stored??this.born.get(relative)??0)<this.grace,...slotted?{graph:{slot:meta!.slot,storedAt:meta!.storedAt}}:{}});
  }
  const reserve=this.graphReserve(bound),result=planRemoteCacheBlobBoundV1(rows,hash=>sizes.get(hash)??0,bound,reserve===null?undefined:{budgetBytes:reserve,now,graceMilliseconds:this.options.graph!.supersedeGraceMilliseconds});
  this.graphProtected={keys:new Set(result.protectedKeys),bytes:result.protectedBytes};
  this.referencedBlobBytes=result.after;
  const sizeOf=new Map(rows.map(row=>[row.key,this.index.get(row.key)!.size]));
  const drops=result.keys.map(key=>({key,size:sizeOf.get(key)!,reason:"budget" as const,...this.slots.get(key)===undefined?{}:{storedAt:this.slots.get(key)!.storedAt}}));
  const freed=result.before-result.after;if(freed>0)this.evictedBlobBytes+=freed;
  return drops;
 }

 /** 📌️ Pins the blobs of the synchronized state (the newest entries with a sidecar that are not stale or unreached, until `pinBudgetBytes` of blobs are pinned: the older ones stay unpinned, so the host store's own eviction can take them before the partition is ever critical and the entry then misses) in the host store under this process's holder, so the host store's own eviction by age, cap or partition pressure, which knows no entries, cannot take a blob a live entry needs; the pin lives one hour at most and is renewed by every collection (the maintenance timer), and a dead process loses it at once. Answers the number of pinned blobs. */
 async pinLive():Promise<number>{
  const host=this.options.host;if(!host)return 0;
  const hashes=new Set<string>(),now=Date.now(),declared=this.options.pinBudgetBytes?.(),budget=declared===undefined?this.lastBound??undefined:this.lastBound===null?declared:Math.min(declared,this.lastBound),live=[...this.slots].filter(([key])=>{const state=this.verdicts?.get(key)?.state;return state!=="stale"&&state!=="unreached";}).sort((left,right)=>right[1].storedAt-left[1].storedAt);
  let pinned=0;
  for(const [hash,until] of this.superseded){if(until>now)hashes.add(hash);else this.superseded.delete(hash);}
  for(const [key,meta] of this.slots)if(protectedKindV1(meta.declaration?.kind)&&(this.graphProtected.keys.has(key)||this.graphProtected.keys.size===0))for(const hash of meta.refs)hashes.add(hash);
  for(const[,meta]of live){
   if(budget!==undefined&&pinned>=budget)break;
   for(const hash of meta.refs){
    if(hashes.has(hash))continue;hashes.add(hash);
    if(budget!==undefined)pinned+=await stat(host.objectPath(hash)).then(info=>info.size,()=>0);
   }
  }
  await host.pin(`remote-cache-live-${process.pid}`,[...hashes],3600000);return hashes.size;
 }

 private async judge(now:number):Promise<Map<string,StalenessVerdictV1>|undefined>{
  const staleness=this.options.staleness;if(staleness===undefined||staleness.mode==="off")return undefined;
  const verdicts=await judgeRemoteCacheStalenessV1(this.index,this.slots,{now,graceMilliseconds:this.grace,tree:staleness.tree,toolchains:staleness.toolchains?.(),retired:this.retired,signal:this.options.signal});
  await staleness.tree.flush().catch(()=>undefined);this.verdicts=verdicts;return verdicts;
 }
 private async record(now:number,verdicts:ReadonlyMap<string,StalenessVerdictV1>,retired:number,tombstones:ReadonlyMap<string,StaleCauseV1>):Promise<void>{
  const staleness=this.options.staleness!,report=stalenessReportV1({mode:staleness.mode,now,verdicts,retired,refused:this.refused,examples:staleness.examples});
  this.report=report;
  try{
   const directory=join(this.root,".retention");await mkdir(directory,{recursive:true});
   if(tombstones.size>0){for(const hash of tombstones.keys())this.retired.delete(hash);this.retired=new Map([...this.retired,...tombstones].slice(-200000));const temporary=join(directory,`retired-${process.pid}.tmp`);await writeFile(temporary,JSON.stringify([...this.retired].map(([blob,cause])=>({blob,...cause}))));await rename(temporary,join(directory,"retired.json"));}
   const log=join(directory,"sweeps.jsonl");
   if((await stat(log).catch(()=>undefined))?.size!>5*1048576)await rename(log,log+".1");
   await appendFile(log,JSON.stringify(report)+"\n");
  }catch{}
 }

 /** 🥀️ What the last staleness sweep found and what the publish gate refused or would refuse since this store opened (`StalenessReport` of the slot schema); `undefined` before the first sweep or when staleness is off. */
 stalenessStatus():ReturnType<typeof stalenessReportV1>|undefined{return this.report===undefined?undefined:{...this.report,refused:{...this.refused}};}

 /** 🥀️ The mode and the manifest names of the staleness policy (`undefined` when the store judges none). */
 get stalenessPolicy():Readonly<{mode:StalenessModeV1;manifestNames:readonly string[]}>|undefined{const staleness=this.options.staleness;return staleness===undefined||staleness.mode==="off"?undefined:{mode:staleness.mode,manifestNames:staleness.manifestNames};}

 /** 🚪️ The publish gate: whether an entry arriving with this declaration is stale on arrival. In enforce mode answers the reason (and counts it as refused); in dry-run mode counts it as one that would be refused and answers nothing; answers nothing when it is current. */
 async staleOnArrival(declaration:DeclarationV1):Promise<Readonly<{reason:StalenessReasonV1;detail:string}>|undefined>{
  const staleness=this.options.staleness;if(staleness===undefined||staleness.mode==="off")return undefined;
  const found=await stalenessOnArrivalV1(declaration,{digest:path=>staleness.tree.digest(path),recheckMilliseconds:staleness.recheckMilliseconds,toolchains:staleness.toolchains?.()});
  if(found===undefined)return undefined;
  const name=staleness.mode==="enforce"?"stale-on-arrival":"would-refuse-stale-on-arrival";this.refused[name]=(this.refused[name]??0)+1;
  return staleness.mode==="enforce"?found:undefined;
 }

 private async publish(relative:string,slot:RemoteCachePutSlotV1,kind:RemoteCacheKindV1,refs:readonly string[]):Promise<void>{
  const previous=this.slots.get(relative),declaration=slot.declaration??previous?.declaration,meta:RemoteCacheMetaV1={version:1,slot:slot.slot,storedAt:this.stamp(),refs:slot.refs??refs,...declaration===undefined?{}:{declaration}};
  if(previous!==undefined&&protectedKindV1(previous.declaration?.kind)&&this.options.graph!==undefined){const until=Date.now()+this.options.graph.supersedeGraceMilliseconds,next=new Set(meta.refs);for(const hash of previous.refs)if(!next.has(hash))this.superseded.set(hash,until);}
  try{await writeRemoteCacheMetaV1(this.root,this.temporary,relative,meta);this.slots.set(relative,meta);}
  catch(error){if(this.options.signal?.aborted)throw error;}
 }
 private async referencesOf(relative:string,kind:RemoteCacheKindV1):Promise<readonly string[]>{
  if(kind==="cas")return [];
  if(this.referenced(relative.split("/")[0]!,kind)){const hash=await this.reference(relative);return hash===undefined?[]:[hash];}
  const namespace=relative.split("/")[0]!;
  return readRemoteCacheReferencesV1(this.path(relative),kind,parseRemoteCacheActionResultV1,tree=>this.readBytes(namespace,"cas",tree.hash,Math.min(tree.size,16*1048576)));
 }

 /** 🪢️ Makes an existing key current in its slot: its slot id and claim time are recorded (the key was stored again, or a client got a hit on it after a revert, or knows its slot only after the run); a read never does this. Answers `missing` for a key that is not stored. */
 async claim(namespace:string,kind:RemoteCacheKindV1,key:string,slot:string):Promise<RemoteCacheClaimOutcomeV1>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  if(kind==="cas")return "missing";
  const relative=this.relative(namespace,kind,key);
  if(!await this.has(namespace,kind,key))return "missing";
  await this.publish(relative,{slot},kind,await this.referencesOf(relative,kind));return "claimed";
 }

 /** 🔭️ The action results stored under one slot of a namespace, newest first and at most `limit` (16): what a client asks to find the stored unit its own key is nearest to (`SlotKeys` of the fleet schema). Reads the slot index only, never a file.
  * @see ../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧬️schema/🔣️.json */
 keysOfSlot(namespace:string,slot:string,limit=16):readonly Readonly<{key:string;storedAtMs:number}>[]{
  const prefix=namespace+"/ac/",rows:{key:string;storedAtMs:number}[]=[];
  for(const [relative,meta] of this.slots)if(meta.slot===slot&&relative.startsWith(prefix))rows.push({key:relative.slice(relative.lastIndexOf("/")+1),storedAtMs:meta.storedAt});
  return rows.sort((left,right)=>right.storedAtMs-left.storedAtMs).slice(0,Math.max(0,Math.min(16,limit)));
 }

 /** ✍️ Streams one value into the store: the body is bounded, hashed when `verify` is set, written to a temporary file and then published atomically; nothing partial is ever visible. */
 async put(namespace:string,kind:RemoteCacheKindV1,key:string,body:AsyncIterable<Uint8Array>|null,input:Readonly<{mode:RemoteCachePutModeV1;verify?:boolean;declaredLength?:number|undefined;slot?:RemoteCachePutSlotV1|undefined;transient?:boolean}>):Promise<RemoteCachePutOutcomeV1>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  const relative=this.relative(namespace,kind,key),slotted=input.slot!==undefined&&kind!=="cas";
  if(slotted)this.inflight.add(relative);
  try{
   this.refCache.delete(relative);
   const outcome=await this.store(namespace,kind,key,body,input,relative);
   if(outcome==="created"||outcome==="replaced")this.born.set(relative,Date.now());
   if(slotted&&(outcome==="created"||outcome==="replaced"||outcome==="exists"))await this.publish(relative,input.slot!,kind,await this.referencesOf(relative,kind));
   return outcome;
  }finally{if(slotted)this.inflight.delete(relative);}
 }

 private writeSignal():AbortSignal{return AbortSignal.any([...this.options.signal===undefined?[]:[this.options.signal],AbortSignal.timeout((this.options.writeBudgetMilliseconds??10000)+(this.options.host?.policy?.admissionBudgetMs??0))]);}
 private async store(namespace:string,kind:RemoteCacheKindV1,key:string,body:AsyncIterable<Uint8Array>|null,input:Readonly<{mode:RemoteCachePutModeV1;verify?:boolean;declaredLength?:number|undefined;transient?:boolean}>,relative:string):Promise<RemoteCachePutOutcomeV1>{
  const declared=input.declaredLength;
  if(kind==="cas"&&this.options.host){
   if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
   let result;
   try{result=await this.options.host.putChecked(body??[],{hash:key,maxBytes:this.options.maximumEntryBytes,...(declared===undefined?{}:{size:declared}),...(input.transient===true?{transient:true}:{})},this.writeSignal());}
   catch(error){if(this.options.signal?.aborted)throw error;if(error instanceof CasCapRefusalV1)this.relieve();return hostPutFailureV1(error);}
   if(result.outcome==="created"){this.admitted+=result.digest?.size??declared??0;this.sizeTrigger();}
   return result.outcome==="exists"?"exists":result.outcome;
  }
  if(this.referenced(namespace,kind)){
   const known=input.mode!=="replace"&&await this.has(namespace,kind,key);
   if(known)return "exists";
   if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
   let result;
   try{result=await this.options.host!.putChecked(body??[],{maxBytes:this.options.maximumEntryBytes,...(declared===undefined?{}:{size:declared})},this.writeSignal());}
   catch(error){if(this.options.signal?.aborted)throw error;if(error instanceof CasCapRefusalV1)this.relieve();return hostPutFailureV1(error);}
   if(!result.digest)return result.outcome as RemoteCachePutOutcomeV1;
   if(result.outcome==="created"){this.admitted+=result.digest.size;this.sizeTrigger();}
   const target=this.path(relative),temporary=join(this.temporary,randomUUID());
   await mkdir(this.path(`${namespace}/${kind}/${key.slice(0,2)}`),{recursive:true});
   const reference=JSON.stringify({version:1,hash:result.digest.hash,size:result.digest.size});
   await writeFile(temporary,reference);await rename(temporary,target);
   const earlier=this.index.get(relative);if(earlier)this.bytes-=earlier.size;
   this.index.set(relative,{size:reference.length,accessed:this.tick()});this.bytes+=reference.length;
   return "created";
  }
  if(input.mode!=="replace"&&await this.has(namespace,kind,key))return "exists";
  if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
  const temporary=join(this.temporary,randomUUID()),hash=input.verify?createHash("sha256"):undefined;let held=0,written=0,handle;
  const reserve=async(extra:number):Promise<boolean>=>{
   if(this.bytes+this.reserved+extra>this.options.maximumBytes){await this.collect(extra);if(this.bytes+this.reserved+extra>this.options.maximumBytes)return false;}
   this.reserved+=extra;held+=extra;return true;
  };
  try{
   if(declared!==undefined&&!await reserve(declared))return "capacity";
   handle=await open(temporary,"wx");
   if(body){
    let outcome:RemoteCachePutOutcomeV1|undefined;
    try{
     for await(const chunk of body){
      this.checkpoint();written+=chunk.byteLength;
      if(written>this.options.maximumEntryBytes){outcome="too-large";break;}
      if(written>held&&!await reserve(written-held)){outcome="capacity";break;}
      hash?.update(chunk);await handle.write(chunk);
     }
    }catch(error){if((error as NodeJS.ErrnoException).syscall!==undefined)throw error;outcome="aborted";}
    if(outcome)return outcome;
   }
   await handle.close();handle=undefined;
   if(declared!==undefined&&written!==declared)return "length-mismatch";
   if(hash&&hash.digest("hex")!==key)return "digest-mismatch";
   await mkdir(this.path(`${namespace}/${kind}/${key.slice(0,2)}`),{recursive:true});
   const final=this.path(relative);
   if(input.mode==="keep"){try{await link(temporary,final);}catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return "exists";throw error;}}
   else await rename(temporary,final);
   const previous=this.index.get(relative);
   if(previous)this.bytes-=previous.size;this.bytes+=written;this.index.set(relative,{size:written,accessed:this.tick()});
   return previous?"replaced":"created";
  }finally{
   this.reserved-=held;await handle?.close().catch(()=>{});await unlink(temporary).catch(()=>{});
   if(this.bytes+this.reserved>this.options.maximumBytes)void this.collect().catch(()=>{});
  }
 }

 /** 🔚️ Removes this process's partial-write area; stored values stay for the next start. */
 async close():Promise<void>{await this.collecting.catch(()=>{});await this.options.host?.release(`remote-cache-live-${process.pid}`).catch(()=>undefined);await rm(this.temporary,{recursive:true,force:true});}
}
