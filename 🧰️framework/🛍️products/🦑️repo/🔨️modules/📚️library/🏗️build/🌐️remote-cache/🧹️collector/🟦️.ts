import {randomUUID} from "node:crypto";
import {link,readdir,readFile,rename,stat,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1} from "../🔬️action-result/🟦️.ts";

/** 📇️ One stored value as the collector sees it: its slash-separated key below the store root (`<namespace>/<kind>/<first two characters>/<name>`), its size and when it was last used. */
export type RemoteCacheRowV1=Readonly<{key:string;size:number;accessed:number}>;

/** 🧱️ The finite bounds of one collection: total bytes, idle age (0 keeps entries until space is needed), the bytes a pending write needs and the bytes writers already reserved. */
export type RemoteCacheBoundsV1=Readonly<{maximumBytes:number;maximumAgeMilliseconds:number;now:number;need?:number;reserved?:number}>;

/** 🗑️ Why a value goes: it was unused longer than allowed, the store is over its bound and it is among the least recently used, or it is an action result nobody can use any more. */
export type RemoteCacheDropReasonV1="dangling"|"age"|"budget";

/** 🧾️ One planned removal. */
export type RemoteCacheDropV1=Readonly<{key:string;size:number;reason:RemoteCacheDropReasonV1}>;

/** 📏️ Entries and bytes of one side of a collection. */
export type RemoteCacheFillV1=Readonly<{entries:number;bytes:number}>;

/** 📋️ What one collection did or, for a plan, would do. */
export type RemoteCacheCollectionReportV1=Readonly<{version:1;dryRun:boolean;scanned:RemoteCacheFillV1;removed:RemoteCacheFillV1&Readonly<Record<RemoteCacheDropReasonV1,number>>;remaining:RemoteCacheFillV1}>;

/** 🔌️ Progress and cancellation of one directory walk or collection. */
export type RemoteCacheWalkHooksV1=Readonly<{signal?:AbortSignal;onProgress?:(stage:"scan"|"verify"|"remove",done:number,bytes:number)=>void}>;

export const REMOTE_CACHE_COLLECTOR_LOCK_V1=".collector.lock",REMOTE_CACHE_SERVER_LOCK_V1=".server.lock";
const LOW_WATER_NUMERATOR=9,LOW_WATER_DENOMINATOR=10,CHECK_EVERY=256,STALE_LOCK_MILLISECONDS=6*3600000,MAXIMUM_ACTION_RESULT_BYTES=16*1048576;

/** 🧮️ Decides, as a pure function, what to remove: unusable entries first, then entries idle longer than the age, then least recently used entries (oldest first, ties by key) until the fill, including pending reservations, is at most nine tenths of the bound less the bytes a write needs. */
export function planRemoteCacheCollectionV1(rows:Iterable<RemoteCacheRowV1>,bounds:RemoteCacheBoundsV1,dangling:ReadonlySet<string>=new Set()):readonly RemoteCacheDropV1[]{
 const drops:RemoteCacheDropV1[]=[],kept:RemoteCacheRowV1[]=[],need=bounds.need??0,reserved=bounds.reserved??0;let bytes=0;
 for(const row of rows){
  if(dangling.has(row.key))drops.push({key:row.key,size:row.size,reason:"dangling"});
  else if(bounds.maximumAgeMilliseconds>0&&bounds.now-row.accessed>bounds.maximumAgeMilliseconds)drops.push({key:row.key,size:row.size,reason:"age"});
  else{kept.push(row);bytes+=row.size;}
 }
 if(bytes+reserved>bounds.maximumBytes-need){
  const target=Math.floor(bounds.maximumBytes*LOW_WATER_NUMERATOR/LOW_WATER_DENOMINATOR)-need;
  kept.sort((left,right)=>left.accessed-right.accessed||(left.key<right.key?-1:left.key>right.key?1:0));
  for(const row of kept){if(bytes+reserved<=target)break;drops.push({key:row.key,size:row.size,reason:"budget"});bytes-=row.size;}
 }
 return drops;
}

/** 🚶️ Walks a store root (`<namespace>/<kind>/<first two characters>/<name>`) without following links, skips the partial-write area, publishes progress and stops when cancelled; file times carry the least-recently-used clock. */
export async function scanRemoteCacheDirectoryV1(root:string,hooks:RemoteCacheWalkHooksV1={}):Promise<Map<string,{size:number;accessed:number}>>{
 const index=new Map<string,{size:number;accessed:number}>();let bytes=0;
 const walk=async(directory:string,depth:number,prefix:readonly string[]):Promise<void>=>{
  hooks.signal?.throwIfAborted();
  let rows;try{rows=await readdir(directory,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
  for(const row of rows){
   if(depth===0&&row.name.startsWith("."))continue;
   if(depth<3){if(row.isDirectory())await walk(join(directory,row.name),depth+1,[...prefix,row.name]);continue;}
   if(!row.isFile())continue;
   let info;try{info=await stat(join(directory,row.name));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}
   index.set([...prefix,row.name].join("/"),{size:info.size,accessed:info.mtimeMs});bytes+=info.size;
   if(index.size%CHECK_EVERY===0){hooks.signal?.throwIfAborted();hooks.onProgress?.("scan",index.size,bytes);}
  }
 };
 await walk(root,0,[]);hooks.onProgress?.("scan",index.size,bytes);return index;
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🔐️ The outcome of asking for a lock: another living process holds it, or it is ours until `release`. */
export type RemoteCacheLockV1=Readonly<{held:true;lock:string}>|Readonly<{held:false;release():Promise<void>}>;

/** 🔐️ Takes a named lock of a store root: a file that appears atomically with the owner's process id and time already inside (written beside it and hard-linked into place, so no taker ever sees it empty). A lock of a dead process or older than six hours is taken over by renaming it away, which only one taker can do, and a lock held by a living process is reported instead of waited for. */
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

/** 🔎️ Finds the action results nobody can use any more: results that are malformed, or whose output blobs left the content store of their namespace. */
export async function findDanglingRemoteCacheResultsV1(root:string,index:ReadonlyMap<string,unknown>,hooks:RemoteCacheWalkHooksV1={}):Promise<ReadonlySet<string>>{
 const dangling=new Set<string>();let done=0;
 for(const key of index.keys()){
  const [namespace,kind]=key.split("/");
  if(kind!=="ac")continue;
  if(++done%CHECK_EVERY===0){hooks.signal?.throwIfAborted();hooks.onProgress?.("verify",done,0);}
  let bytes;try{bytes=await readFile(join(root,...key.split("/")));}catch{continue;}
  if(bytes.length>MAXIMUM_ACTION_RESULT_BYTES){dangling.add(key);continue;}
  try{
   const parsed=parseRemoteCacheActionResultV1(bytes);
   for(const value of[...parsed.references,...parsed.trees])if(!index.has(`${namespace}/cas/${value.hash.slice(0,2)}/${value.hash}`)){dangling.add(key);break;}
  }catch{dangling.add(key);}
 }
 hooks.onProgress?.("verify",done,0);return dangling;
}

/** 🧹️ Everything one directory collection needs. */
export type RemoteCacheCollectionRequestV1=Readonly<{root:string;maximumBytes:number;maximumAgeMilliseconds:number;dryRun:boolean;verifyResults:boolean;now?:number}>&RemoteCacheWalkHooksV1;

const fill=(rows:Iterable<{size:number}>):RemoteCacheFillV1=>{let entries=0,bytes=0;for(const row of rows){entries++;bytes+=row.size;}return {entries,bytes};};

/** 🧹️ Collects a store directory the way a running server would, from outside: scans it, finds unusable action results, plans age and least-recently-used removals against the bound and, unless the request only plans, removes them. It holds the collector lock while removing and answers `locked` when another collector is running; removals are individually atomic, so cancelling leaves a consistent store. */
export async function collectRemoteCacheDirectoryV1(request:RemoteCacheCollectionRequestV1):Promise<Readonly<{locked:true;lock:string}>|Readonly<{locked:false;value:RemoteCacheCollectionReportV1}>>{
 const run=async():Promise<RemoteCacheCollectionReportV1>=>{
  const hooks:RemoteCacheWalkHooksV1={...request.signal?{signal:request.signal}:{},...request.onProgress?{onProgress:request.onProgress}:{}};
  const index=await scanRemoteCacheDirectoryV1(request.root,hooks),scanned=fill(index.values()),dangling=request.verifyResults?await findDanglingRemoteCacheResultsV1(request.root,index,hooks):new Set<string>();
  const rows=[...index].map(([key,row])=>({key,...row})),plan=planRemoteCacheCollectionV1(rows,{maximumBytes:request.maximumBytes,maximumAgeMilliseconds:request.maximumAgeMilliseconds,now:request.now??Date.now()},dangling),counts={dangling:0,age:0,budget:0};
  let done=0,bytes=0;
  for(const drop of plan){
   counts[drop.reason]++;bytes+=drop.size;
   if(request.dryRun)continue;
   request.signal?.throwIfAborted();
   try{await unlink(join(request.root,...drop.key.split("/")));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
   if(++done%CHECK_EVERY===0)request.onProgress?.("remove",done,bytes);
  }
  request.onProgress?.("remove",plan.length,bytes);
  return {version:1,dryRun:request.dryRun,scanned,removed:{entries:plan.length,bytes,...counts},remaining:{entries:scanned.entries-plan.length,bytes:scanned.bytes-bytes}};
 };
 if(request.dryRun)return {locked:false,value:await run()};
 const lock=await acquireRemoteCacheLockV1(request.root,REMOTE_CACHE_COLLECTOR_LOCK_V1);
 if(lock.held)return {locked:true,lock:lock.lock};
 try{return {locked:false,value:await run()};}finally{await lock.release();}
}
