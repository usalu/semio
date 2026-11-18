import {createHash,randomUUID} from "node:crypto";
import {link,mkdir,open,readdir,readFile,rename,rm,stat,unlink,utimes,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {requireRemoteCacheShapeV1} from "../🟦️.ts";
import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {liveRemoteCacheReferencesV1,planRemoteCacheCollectionV1,removeRemoteCacheEntryV1,scanRemoteCacheDirectoryV1,type RemoteCacheStepV1} from "../🧹️collector/🟦️.ts";
import {parseRemoteCacheActionResultV1} from "../🔬️action-result/🟦️.ts";
import {readRemoteCacheReferencesV1,removeRemoteCacheMetaV1,scanRemoteCacheMetaV1,writeRemoteCacheMetaV1,type RemoteCacheMetaV1} from "../🏷️retention/🟦️.ts";
import {classifySlotsV1,SLOT_GRACE_MILLISECONDS_V1} from "../../🪢️slots/🟦️.ts";
import {reclaimCasObjectsV1} from "../../🗄️cas/🪦️reclaim/🟦️.ts";

/** 🗂️ Names the three keyspaces one store partitions: Bazel action results, Bazel content blobs and Nx task archives. */
export type RemoteCacheKindV1="ac"|"cas"|"nx";

/** 📮️ Reports why one write ended: stored, replaced an older value, found an equal or protected value, or was refused. */
export type RemoteCachePutOutcomeV1="created"|"replaced"|"exists"|"digest-mismatch"|"length-mismatch"|"too-large"|"capacity"|"aborted";

/** 📡️ Progress and housekeeping events of one store, published to the caller without any logging dependency. */
export type RemoteCacheStoreEventV1=Readonly<{stage:"scan";entries:number;bytes:number}>|Readonly<{stage:"evict";entries:number;bytes:number}>|Readonly<{stage:"reclaim";entries:number;bytes:number}>|Readonly<{stage:"ready";entries:number;bytes:number}>;

/** 🧱️ Authors the finite bounds of one store: total bytes, bytes per entry and the idle age after which an entry without a slot is dropped (0 keeps entries until space is needed), the supersession grace in milliseconds (default ten minutes), the predicate of entries an active claim holds and a hook that tests use to simulate a crash between the steps of a removal. */
export type RemoteCacheStoreOptionsV1=Readonly<{directory:string;maximumBytes:number;maximumEntryBytes:number;maximumAgeMilliseconds:number;host?:CasStoreV1;graceMilliseconds?:number;claimed?:(namespace:string,kind:RemoteCacheKindV1,key:string)=>boolean;onStep?:(point:RemoteCacheStepV1)=>void;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1)=>void}>;

/** 📏️ Describes the current fill of one store. */
export type RemoteCacheStoreUsageV1=Readonly<{entries:number;bytes:number;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;slotted:number;evictedEntries:number;evictedBytes:number}>;

/** 🪢️ What a write says about the slot of its entry: the slot id and, for action results, the blobs they reference (Nx entries reference their host store body). */
export type RemoteCachePutSlotV1=Readonly<{slot:string;refs?:readonly string[]}>;

/** 📤️ One stored value ready to be streamed. */
export type RemoteCacheEntryV1=Readonly<{path:string;size:number}>;

/** 🔑️ Names how a write treats an existing value: `replace` overwrites, `keep` refuses to overwrite and `content` is idempotent because the key is the content digest. */
export type RemoteCachePutModeV1="replace"|"keep"|"content";

/** 🪢️ One entry brought back to current by `claim`. */
export type RemoteCacheClaimOutcomeV1="claimed"|"missing";

const NAMESPACE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,KEYS:Readonly<Record<RemoteCacheKindV1,RegExp>>={ac:/^[0-9a-f]{64}$/u,cas:/^[0-9a-f]{64}$/u,nx:/^[a-z0-9_-]{1,128}$/u};

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

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
 private reserved=0;
 private clock=0;
 private collecting:Promise<void>=Promise.resolve();
 private readonly writing=join(".tmp",`${process.pid}-${randomUUID()}`);
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
  for(const name of await readdir(join(store.root,".tmp")))if(!alive(Number.parseInt(name,10)))await rm(join(store.root,".tmp",name),{recursive:true,force:true});
  await mkdir(store.temporary,{recursive:true});
  await store.refresh();options.onEvent?.({stage:"ready",entries:store.index.size,bytes:store.bytes});return store;
 }

 /** 📁️ The versioned root below the store directory. */
 get root():string{return join(this.options.directory,"v1");}
 private get temporary():string{return join(this.root,this.writing);}
 private relative(namespace:string,kind:RemoteCacheKindV1,key:string):string{return `${namespace}/${kind}/${key.slice(0,2)}/${key}`;}
 private path(relative:string):string{return join(this.root,...relative.split("/"));}
 private checkpoint():void{this.options.signal?.throwIfAborted();}
 private tick():number{return this.clock=Math.max(Date.now(),this.clock+1);}


 private referenced(namespace:string,kind:RemoteCacheKindV1):boolean{return kind==="nx"&&this.options.host!==undefined;}
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
 async refresh():Promise<void>{
  const found=await scanRemoteCacheDirectoryV1(this.root,{...this.options.signal?{signal:this.options.signal}:{},onProgress:(_stage,entries,bytes)=>this.options.onEvent?.({stage:"scan",entries,bytes})});
  this.index.clear();this.slots.clear();this.bytes=0;
  for(const[relative,row]of found){this.index.set(relative,row);this.clock=Math.max(this.clock,row.accessed);this.bytes+=row.size;}
  for(const[relative,meta]of await scanRemoteCacheMetaV1(this.root,found,this.options.signal)){this.slots.set(relative,meta);this.storedClock=Math.max(this.storedClock,meta.storedAt);}
  await this.collect();
 }

 /** 📊️ Reports the current fill. */
 usage():RemoteCacheStoreUsageV1{return Object.freeze({entries:this.index.size,bytes:this.bytes,reservedBytes:this.reserved,maximumBytes:this.options.maximumBytes,maximumEntryBytes:this.options.maximumEntryBytes,slotted:this.slots.size,evictedEntries:this.evictedEntries,evictedBytes:this.evictedBytes});}

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
  this.slots.delete(relative);await removeRemoteCacheMetaV1(this.root,relative).catch(()=>undefined);
 }

 /** 🧹️ Plans and removes, in this order: unusable values, values a newer key of their slot superseded beyond the grace (never one an active claim holds or one stored within the grace), values without a slot idle longer than the authored age, then the least valuable values (superseded, without slot, current; least recently used first) until the fill is at most nine tenths of the bound; afterwards it offers the blobs no remaining entry references to the host store, which keeps every blob a live pin, a recent access or a workspace hard link protects. Calls queue behind each other. */
 collect(need:number=0):Promise<void>{
  const run=async()=>{
   const claimed=(relative:string):boolean=>{
    if(this.inflight.has(relative))return true;
    if(!this.options.claimed)return false;
    const [namespace,kind,,key]=relative.split("/");return this.options.claimed(namespace!,kind as RemoteCacheKindV1,key!);
   };
   const plan=planRemoteCacheCollectionV1([...this.index].map(([key,row])=>({key,...row})),{maximumBytes:this.options.maximumBytes,maximumAgeMilliseconds:this.options.maximumAgeMilliseconds,now:Date.now(),need,reserved:this.reserved},new Set(),{metas:this.slots,graceMilliseconds:this.grace,claimed});
   let removedEntries=0,removedBytes=0;const gone:string[]=[],refs=new Set<string>();
   for(const drop of plan){
    this.checkpoint();
    const meta=this.slots.get(drop.key);let outcome;
    try{outcome=await removeRemoteCacheEntryV1(this.root,drop,{...this.options.onStep?{onStep:this.options.onStep}:{},claimed,graceMilliseconds:this.grace});}catch(error){if(/^E[A-Z]+$/u.test(String((error as NodeJS.ErrnoException).code)))continue;throw error;}
    if(outcome==="kept")continue;
    const row=this.index.get(drop.key);
    if(row){this.index.delete(drop.key);this.bytes-=row.size;}
    this.slots.delete(drop.key);gone.push(drop.key);if(meta)for(const hash of meta.refs)refs.add(hash);
    removedEntries++;removedBytes+=drop.size;
   }
   if(removedEntries){this.evictedEntries+=removedEntries;this.evictedBytes+=removedBytes;this.options.onEvent?.({stage:"evict",entries:removedEntries,bytes:removedBytes});}
   if(this.options.host&&refs.size>0){
    const live=await liveRemoteCacheReferencesV1(this.root,this.index.keys(),this.slots,this.options.signal),orphaned=[...refs].filter(hash=>!live.has(hash));
    if(orphaned.length>0){
     const freed=await reclaimCasObjectsV1(this.options.host,orphaned,{...this.options.signal?{signal:this.options.signal}:{}});
     if(freed.entries>0)this.options.onEvent?.({stage:"reclaim",entries:freed.entries,bytes:freed.bytes});
    }
   }
   await this.pinLive().catch(()=>undefined);
  };
  this.collecting=this.collecting.then(run,run);return this.collecting;
 }

 /** 📌️ Pins the blobs of the synchronized state (every slotted entry that is current or still within its grace) in the host store under this process's holder, so the host store's own eviction by age, cap or partition pressure, which knows no entries, cannot take a blob a live entry needs; the pin lives one hour at most and is renewed by every collection (the maintenance timer), and a dead process loses it at once. Answers the number of pinned blobs. */
 async pinLive():Promise<number>{
  const host=this.options.host;if(!host)return 0;
  const verdicts=classifySlotsV1([...this.slots].map(([key,meta])=>({key,slot:meta.slot,storedAt:meta.storedAt,accessed:0,size:0})),{now:Date.now(),graceMilliseconds:this.grace}),hashes=new Set<string>();
  for(const[key,meta]of this.slots){const state=verdicts.get(key)?.state;if(state==="current"||state==="retained")for(const hash of meta.refs)hashes.add(hash);}
  await host.pin(`remote-cache-live-${process.pid}`,[...hashes],3600000);return hashes.size;
 }

 private async publish(relative:string,slot:RemoteCachePutSlotV1,kind:RemoteCacheKindV1,refs:readonly string[]):Promise<void>{
  const meta:RemoteCacheMetaV1={version:1,slot:slot.slot,storedAt:this.stamp(),refs:slot.refs??refs};
  try{await writeRemoteCacheMetaV1(this.root,this.temporary,relative,meta);this.slots.set(relative,meta);}
  catch(error){if(this.options.signal?.aborted)throw error;}
 }
 private async referencesOf(relative:string,kind:RemoteCacheKindV1):Promise<readonly string[]>{
  if(kind==="cas")return [];
  if(this.referenced(relative.split("/")[0]!,kind)){const hash=await this.reference(relative);return hash===undefined?[]:[hash];}
  return readRemoteCacheReferencesV1(this.path(relative),kind,parseRemoteCacheActionResultV1);
 }

 /** 🪢️ Makes an existing key current in its slot: its slot id and claim time are recorded (the key was stored again, or a client got a hit on it after a revert, or knows its slot only after the run); a read never does this. Answers `missing` for a key that is not stored. */
 async claim(namespace:string,kind:RemoteCacheKindV1,key:string,slot:string):Promise<RemoteCacheClaimOutcomeV1>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  if(kind==="cas")return "missing";
  const relative=this.relative(namespace,kind,key);
  if(!await this.has(namespace,kind,key))return "missing";
  await this.publish(relative,{slot},kind,await this.referencesOf(relative,kind));return "claimed";
 }

 /** ✍️ Streams one value into the store: the body is bounded, hashed when `verify` is set, written to a temporary file and then published atomically; nothing partial is ever visible. */
 async put(namespace:string,kind:RemoteCacheKindV1,key:string,body:AsyncIterable<Uint8Array>|null,input:Readonly<{mode:RemoteCachePutModeV1;verify?:boolean;declaredLength?:number|undefined;slot?:RemoteCachePutSlotV1|undefined}>):Promise<RemoteCachePutOutcomeV1>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  const relative=this.relative(namespace,kind,key),slotted=input.slot!==undefined&&kind!=="cas";
  if(slotted)this.inflight.add(relative);
  try{
   const outcome=await this.store(namespace,kind,key,body,input,relative);
   if(slotted&&(outcome==="created"||outcome==="replaced"||outcome==="exists"))await this.publish(relative,input.slot!,kind,await this.referencesOf(relative,kind));
   return outcome;
  }finally{if(slotted)this.inflight.delete(relative);}
 }

 private async store(namespace:string,kind:RemoteCacheKindV1,key:string,body:AsyncIterable<Uint8Array>|null,input:Readonly<{mode:RemoteCachePutModeV1;verify?:boolean;declaredLength?:number|undefined}>,relative:string):Promise<RemoteCachePutOutcomeV1>{
  const declared=input.declaredLength;
  if(kind==="cas"&&this.options.host){
   if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
   const result=await this.options.host.putChecked(body??[],{hash:key,maxBytes:this.options.maximumEntryBytes,...(declared===undefined?{}:{size:declared})},this.options.signal);
   return result.outcome==="exists"?"exists":result.outcome;
  }
  if(this.referenced(namespace,kind)){
   const known=input.mode!=="replace"&&await this.has(namespace,kind,key);
   if(known)return "exists";
   if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
   const result=await this.options.host!.putChecked(body??[],{maxBytes:this.options.maximumEntryBytes,...(declared===undefined?{}:{size:declared})},this.options.signal);
   if(!result.digest)return result.outcome as RemoteCachePutOutcomeV1;
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
