import {randomUUID} from "node:crypto";
import {link,readdir,readFile,rename,stat,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1,type RemoteCacheDigestV1} from "../🔬️action-result/🟦️.ts";
import {classifySlotsV1,SLOT_GRACE_MILLISECONDS_V1,slotBudgetTierV1,type SlotVerdictV1} from "../../🪢️slots/🟦️.ts";
import {readRemoteCacheMetaV1,readRemoteCacheReferencesV1,removeRemoteCacheMetaV1,scanRemoteCacheMetaV1,type RemoteCacheMetaV1} from "../🏷️retention/🟦️.ts";

/** 📇️ One stored value as the collector sees it: its slash-separated key below the store root (`<namespace>/<kind>/<first two characters>/<name>`), its size and when it was last used. */
export type RemoteCacheRowV1=Readonly<{key:string;size:number;accessed:number}>;

/** 🧱️ The finite bounds of one collection: total bytes, idle age (0 keeps entries until space is needed), the bytes a pending write needs and the bytes writers already reserved. */
export type RemoteCacheBoundsV1=Readonly<{maximumBytes:number;maximumAgeMilliseconds:number;now:number;need?:number;reserved?:number}>;

/** 🗑️ Why a value goes: it is an action result nobody can use any more, a newer key of its slot superseded it and the grace passed, it was unused longer than allowed (values without a slot), or the store is over its bound and it is among the least valuable. */
export type RemoteCacheDropReasonV1="dangling"|"superseded"|"age"|"budget";

/** 🧾️ One planned removal. */
export type RemoteCacheDropV1=Readonly<{key:string;size:number;reason:RemoteCacheDropReasonV1;storedAt?:number}>;

/** 📏️ Entries and bytes of one side of a collection. */
export type RemoteCacheFillV1=Readonly<{entries:number;bytes:number}>;

/** 📋️ What one collection did or, for a plan, would do. */
export type RemoteCacheCollectionReportV1=Readonly<{version:1;dryRun:boolean;scanned:RemoteCacheFillV1;removed:RemoteCacheFillV1&Readonly<Record<RemoteCacheDropReasonV1,number>>;remaining:RemoteCacheFillV1;reclaimed:RemoteCacheFillV1;deferred:boolean}>;

/** 🔌️ Progress and cancellation of one directory walk or collection. */
export type RemoteCacheWalkHooksV1=Readonly<{concurrency?:number;signal?:AbortSignal;onProgress?:(stage:"scan"|"verify"|"remove",done:number,bytes:number)=>void;onStep?:(point:RemoteCacheStepV1)=>void}>;

/** 💥️ The points of one removal at which a crash can be simulated: before the value goes, after it went and after its sidecar went. */
export type RemoteCacheStepV1="before-unlink"|"after-unlink"|"after-meta";

/** 🪢️ What retention adds to a plan: the sidecar of every slotted value (by key), the grace in milliseconds and the predicate of values an active claim protects. */
export type RemoteCacheRetentionV1=Readonly<{metas:ReadonlyMap<string,Pick<RemoteCacheMetaV1,"slot"|"storedAt">>;graceMilliseconds:number;claimed?:(key:string)=>boolean}>;

export const REMOTE_CACHE_COLLECTOR_LOCK_V1=".collector.lock",REMOTE_CACHE_SERVER_LOCK_V1=".server.lock";
const LOW_WATER_NUMERATOR=9,LOW_WATER_DENOMINATOR=10,CHECK_EVERY=256,STALE_LOCK_MILLISECONDS=6*3600000,MAXIMUM_ACTION_RESULT_BYTES=16*1048576;

/** 🧮️ Decides, as a pure function, what to remove: unusable entries first; with `retention`, entries a newer key of their slot superseded beyond the grace and that no claim protects; then entries without a slot idle longer than the age (slotted entries are the synchronized state and are not aged out); then, until the fill including pending reservations is at most nine tenths of the bound less the bytes a write needs, the least valuable entries (superseded within the grace, then entries without a slot, then current entries; least recently used first, ties by key). Entries stored within the grace and claimed entries are never taken. */
export function planRemoteCacheCollectionV1(rows:Iterable<RemoteCacheRowV1>,bounds:RemoteCacheBoundsV1,dangling:ReadonlySet<string>=new Set(),retention?:RemoteCacheRetentionV1):readonly RemoteCacheDropV1[]{
 const drops:RemoteCacheDropV1[]=[],kept:RemoteCacheRowV1[]=[],need=bounds.need??0,reserved=bounds.reserved??0,tier=new Map<string,number>(),guarded=new Set<string>();let bytes=0;
 const all=[...rows],verdicts:ReadonlyMap<string,SlotVerdictV1>=retention?classifySlotsV1(all.map(row=>{const meta=retention.metas.get(row.key);return {key:row.key,slot:meta?.slot??null,storedAt:meta?.storedAt??0,accessed:row.accessed,size:row.size};}),{now:bounds.now,graceMilliseconds:retention.graceMilliseconds}):new Map();
 for(const row of all){
  const verdict=verdicts.get(row.key),state=verdict?.state??"unslotted",stored=retention?.metas.get(row.key)?.storedAt,claimed=retention?.claimed?.(row.key)===true;
  if(dangling.has(row.key))drops.push({key:row.key,size:row.size,reason:"dangling"});
  else if(state==="expired"&&!claimed)drops.push({key:row.key,size:row.size,reason:"superseded",...(stored===undefined?{}:{storedAt:stored})});
  else if(state==="unslotted"&&!claimed&&bounds.maximumAgeMilliseconds>0&&bounds.now-row.accessed>bounds.maximumAgeMilliseconds)drops.push({key:row.key,size:row.size,reason:"age"});
  else{
   kept.push(row);bytes+=row.size;tier.set(row.key,slotBudgetTierV1(state));
   if(claimed||(stored!==undefined&&retention!==undefined&&bounds.now-stored<retention.graceMilliseconds))guarded.add(row.key);
  }
 }
 if(bytes+reserved>bounds.maximumBytes-need){
  const target=Math.floor(bounds.maximumBytes*LOW_WATER_NUMERATOR/LOW_WATER_DENOMINATOR)-need;
  kept.sort((left,right)=>(tier.get(left.key)!-tier.get(right.key)!)||left.accessed-right.accessed||(left.key<right.key?-1:left.key>right.key?1:0));
  for(const row of kept){if(bytes+reserved<=target)break;if(guarded.has(row.key))continue;const stored=retention?.metas.get(row.key)?.storedAt;drops.push({key:row.key,size:row.size,reason:"budget",...(stored===undefined?{}:{storedAt:stored})});bytes-=row.size;}
 }
 return drops;
}

/** 🚶️ Walks a store root (`<namespace>/<kind>/<first two characters>/<name>`) without following links, skips the partial-write area, publishes progress and stops when cancelled; file times carry the least-recently-used clock. */
export async function scanRemoteCacheDirectoryV1(root:string,hooks:RemoteCacheWalkHooksV1={}):Promise<Map<string,{size:number;accessed:number}>>{
 const index=new Map<string,{size:number;accessed:number}>();let bytes=0;
 const walk=async(directory:string,depth:number,prefix:readonly string[]):Promise<void>=>{
  hooks.signal?.throwIfAborted();
  let rows;try{rows=await readdir(directory,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
  const files:string[]=[],width=hooks.concurrency??64;
  for(const row of rows){
   if(depth===0&&row.name.startsWith("."))continue;
   if(depth<3){if(row.isDirectory())await walk(join(directory,row.name),depth+1,[...prefix,row.name]);continue;}
   if(row.isFile())files.push(row.name);
  }
  for(let at=0;at<files.length;at+=width){
   if(at>0&&width<=16)await new Promise<void>(done=>{setImmediate(done);});
   const batch=files.slice(at,at+width),infos=await Promise.all(batch.map(async name=>{try{return await stat(join(directory,name));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}}));
   batch.forEach((name,offset)=>{const info=infos[offset];if(info===undefined)return;index.set([...prefix,name].join("/"),{size:info.size,accessed:info.mtimeMs});bytes+=info.size;});
   hooks.signal?.throwIfAborted();hooks.onProgress?.("scan",index.size,bytes);
  }
 };
 await walk(root,0,[]);hooks.onProgress?.("scan",index.size,bytes);return index;
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🔐️ The outcome of asking for a lock: another living process holds it, or it is ours until `release`. */
export type RemoteCacheLockV1=Readonly<{held:true;lock:string}>|Readonly<{held:false;release():Promise<void>}>;

/** 💾️ Takes a named lock of a store root: a file that appears atomically with the owner's process id and time already inside (written beside it and hard-linked into place, so no taker ever sees it empty). A lock of a dead process or older than six hours is taken over by renaming it away, which only one taker can do, and a lock held by a living process is reported instead of waited for. */
export async function acquireRemoteCacheLockV1(root:string,name:string):Promise<RemoteCacheLockV1>{
 const lock=join(root,name),mine=join(root,`.${name}.${process.pid}.${randomUUID()}`),content=`${process.pid}\n${Date.now()}\n`;
 await writeFile(mine,content);
 try{
  for(let attempt=0;attempt<3;attempt++){
   try{await link(mine,lock);return {held:false,release:async()=>{await unlink(lock).catch(()=>{});}};}
   catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
   const seen=await readFile(lock,"utf8").catch(()=>undefined);
   if(seen===undefined)continue;
   const [pid,started]=seen.split("\n").map(Number);
   if(pid!==undefined&&started!==undefined&&Number.isSafeInteger(pid)&&pid>0&&alive(pid)&&Date.now()-started<STALE_LOCK_MILLISECONDS)return {held:true,lock};
   const aside=`${lock}.stale-${randomUUID()}`;
   try{await rename(lock,aside);}catch{continue;}
   if(await readFile(aside,"utf8").catch(()=>undefined)===seen){await unlink(aside).catch(()=>{});continue;}
   await link(aside,lock).catch(()=>{});await unlink(aside).catch(()=>{});return {held:true,lock};
  }
  return {held:true,lock};
 }finally{await unlink(mine).catch(()=>{});}
}

/** 🔎️ Finds the action results nobody can use any more: results that are malformed, or whose output blobs left the content store of their namespace. A server keeps its blobs in the host store, outside the directory: `hasBlobs` asks it, and without it a blob missing from the directory counts as gone. */
export async function findDanglingRemoteCacheResultsV1(root:string,index:ReadonlyMap<string,unknown>,hooks:RemoteCacheWalkHooksV1={},hasBlobs?:(hashes:readonly string[])=>Promise<readonly boolean[]>):Promise<ReadonlySet<string>>{
 const dangling=new Set<string>();let done=0;
 for(const key of index.keys()){
  const [namespace,kind]=key.split("/");
  if(kind!=="ac")continue;
  if(++done%CHECK_EVERY===0){hooks.signal?.throwIfAborted();hooks.onProgress?.("verify",done,0);}
  let bytes;try{bytes=await readFile(join(root,...key.split("/")));}catch{continue;}
  if(bytes.length>MAXIMUM_ACTION_RESULT_BYTES){dangling.add(key);continue;}
  try{
   const parsed=parseRemoteCacheActionResultV1(bytes);
   const missing=[...parsed.references,...parsed.trees].filter(value=>!index.has(`${namespace}/cas/${value.hash.slice(0,2)}/${value.hash}`)).map(value=>value.hash);
   if(missing.length>0&&(hasBlobs===undefined||!(await hasBlobs(missing)).every(Boolean)))dangling.add(key);
  }catch{dangling.add(key);}
 }
 hooks.onProgress?.("verify",done,0);return dangling;
}

/** 🧹️ Everything one directory collection needs; `claimed` protects entries an active claim holds, `graceMilliseconds` is the supersession grace, `reclaim` receives the blobs no remaining entry references and answers what it freed. */
export type RemoteCacheCollectionRequestV1=Readonly<{root:string;maximumBytes:number;maximumAgeMilliseconds:number;dryRun:boolean;verifyResults:boolean;now?:number;graceMilliseconds?:number;claimed?:(key:string)=>boolean;reclaim?:(hashes:readonly string[],dryRun:boolean)=>Promise<RemoteCacheFillV1>;hasBlobs?:(hashes:readonly string[])=>Promise<readonly boolean[]>;readTree?:(namespace:string,tree:RemoteCacheDigestV1)=>Promise<Uint8Array|undefined>}>&RemoteCacheWalkHooksV1;

const fill=(rows:Iterable<{size:number}>):RemoteCacheFillV1=>{let entries=0,bytes=0;for(const row of rows){entries++;bytes+=row.size;}return {entries,bytes};};

/** 🔒️ Tells whether a living server owns the store root: a collector that cannot see the server's claims leaves supersession to it. */
export async function remoteCacheServerRunningV1(root:string):Promise<boolean>{
 const seen=await readFile(join(root,REMOTE_CACHE_SERVER_LOCK_V1),"utf8").catch(()=>undefined);
 if(seen===undefined)return false;
 const [pid,started]=seen.split("\n").map(Number);
 return pid!==undefined&&started!==undefined&&Number.isSafeInteger(pid)&&pid>0&&alive(pid)&&Date.now()-started<STALE_LOCK_MILLISECONDS;
}

/** 🗡️ Removes one planned value safely: a superseded or budget value with a sidecar is re-checked right before it goes (its sidecar is unchanged, so no store or claim made it current meanwhile, and no claim holds it); the value goes first and its sidecar after, so a crash between the two leaves an orphan sidecar the next scan removes and never a value that is half gone. Answers `removed`, `kept` (re-check refused) or `gone`. */
export async function removeRemoteCacheEntryV1(root:string,drop:RemoteCacheDropV1,hooks:Readonly<{onStep?:(point:RemoteCacheStepV1)=>void;claimed?:(key:string)=>boolean;graceMilliseconds?:number}>={}):Promise<"removed"|"kept"|"gone">{
 if(drop.storedAt!==undefined){
  const meta=await readRemoteCacheMetaV1(root,drop.key);
  if(meta===undefined||meta.storedAt!==drop.storedAt||hooks.claimed?.(drop.key)===true)return "kept";
  if(drop.reason==="superseded"&&hooks.graceMilliseconds!==undefined&&Date.now()-meta.storedAt<hooks.graceMilliseconds)return "kept";
 }
 hooks.onStep?.("before-unlink");
 try{await unlink(join(root,...drop.key.split("/")));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;await removeRemoteCacheMetaV1(root,drop.key);return "gone";}
 hooks.onStep?.("after-unlink");
 await removeRemoteCacheMetaV1(root,drop.key);hooks.onStep?.("after-meta");
 return "removed";
}

/** 🔗️ The blobs that remain referenced after a removal: the sidecar references of slotted entries and the references read from the other entries. */
export async function liveRemoteCacheReferencesV1(root:string,remaining:Iterable<string>,metas:ReadonlyMap<string,RemoteCacheMetaV1>,signal?:AbortSignal,readTree?:RemoteCacheCollectionRequestV1["readTree"]):Promise<Set<string>>{
 const live=new Set<string>();let done=0;
 for(const key of remaining){
  const meta=metas.get(key);
  if(meta){for(const hash of meta.refs)live.add(hash);continue;}
  const [namespace,kind]=key.split("/");if(kind!=="ac"&&kind!=="nx")continue;
  if(++done%CHECK_EVERY===0)signal?.throwIfAborted();
  for(const hash of await readRemoteCacheReferencesV1(join(root,...key.split("/")),kind,parseRemoteCacheActionResultV1,readTree&& (tree=>readTree(namespace!,tree))))live.add(hash);
 }
 return live;
}

/** ☁️ Collects a store directory the way a running server would, from outside: scans it, finds unusable action results, plans supersession, age and least-valuable removals against the bound and, unless the request only plans, removes them, then offers the blobs no remaining entry references to `reclaim`. It holds the collector lock while removing and answers `locked` when another collector is running; removals are individually atomic, so cancelling or crashing at any point leaves a readable store. While a living server owns the directory its claims are unknown here, so supersession is left to the server (`deferred`). */
export async function collectRemoteCacheDirectoryV1(request:RemoteCacheCollectionRequestV1):Promise<Readonly<{locked:true;lock:string}>|Readonly<{locked:false;value:RemoteCacheCollectionReportV1}>>{
 const run=async():Promise<RemoteCacheCollectionReportV1>=>{
  const hooks:RemoteCacheWalkHooksV1={...request.signal?{signal:request.signal}:{},...request.onProgress?{onProgress:request.onProgress}:{}};
  const index=await scanRemoteCacheDirectoryV1(request.root,hooks),scanned=fill(index.values()),dangling=request.verifyResults?await findDanglingRemoteCacheResultsV1(request.root,index,hooks,request.hasBlobs):new Set<string>();
  const deferred=request.claimed===undefined&&await remoteCacheServerRunningV1(request.root),metas=await scanRemoteCacheMetaV1(request.root,index,request.signal);
  const rows=[...index].map(([key,row])=>({key,...row})),now=request.now??Date.now(),grace=request.graceMilliseconds??SLOT_GRACE_MILLISECONDS_V1;
  const retention:RemoteCacheRetentionV1|undefined=deferred?undefined:{metas,graceMilliseconds:grace,...request.claimed?{claimed:request.claimed}:{}};
  const plan=planRemoteCacheCollectionV1(rows,{maximumBytes:request.maximumBytes,maximumAgeMilliseconds:request.maximumAgeMilliseconds,now},dangling,retention),counts={dangling:0,superseded:0,age:0,budget:0},dropped=new Set<string>(),sizes=new Map(plan.map(drop=>[drop.key,drop.size]));
  let done=0,bytes=0,entries=0;
  for(const drop of plan){
   if(request.dryRun){counts[drop.reason]++;bytes+=drop.size;entries++;dropped.add(drop.key);continue;}
   request.signal?.throwIfAborted();
   const outcome=await removeRemoteCacheEntryV1(request.root,drop,{...request.onStep?{onStep:request.onStep}:{},...request.claimed?{claimed:request.claimed}:{},graceMilliseconds:grace});
   if(outcome==="removed"){counts[drop.reason]++;bytes+=sizes.get(drop.key)!;entries++;dropped.add(drop.key);}
   if(++done%CHECK_EVERY===0)request.onProgress?.("remove",done,bytes);
  }
  request.onProgress?.("remove",plan.length,bytes);
  let reclaimed:RemoteCacheFillV1={entries:0,bytes:0};
  if(request.reclaim&&dropped.size>0){
   const candidates=new Set<string>();for(const key of dropped){const meta=metas.get(key);if(meta)for(const hash of meta.refs)candidates.add(hash);}
   if(candidates.size>0){
    const live=await liveRemoteCacheReferencesV1(request.root,[...index.keys()].filter(key=>!dropped.has(key)),metas,request.signal,request.readTree);
    reclaimed=await request.reclaim([...candidates].filter(hash=>!live.has(hash)),request.dryRun);
   }
  }
  return {version:1,dryRun:request.dryRun,scanned,removed:{entries,bytes,...counts},remaining:{entries:scanned.entries-entries,bytes:scanned.bytes-bytes},reclaimed,deferred};
 };
 if(request.dryRun)return {locked:false,value:await run()};
 const lock=await acquireRemoteCacheLockV1(request.root,REMOTE_CACHE_COLLECTOR_LOCK_V1);
 if(lock.held)return {locked:true,lock:lock.lock};
 try{return {locked:false,value:await run()};}finally{await lock.release();}
}
