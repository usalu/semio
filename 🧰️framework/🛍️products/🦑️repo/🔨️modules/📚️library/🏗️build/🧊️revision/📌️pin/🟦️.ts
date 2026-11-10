import {lstat,open,readlink} from "node:fs/promises";
import {join,sep} from "node:path";
import type {CasDigestV1,CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {buildTreeV1,compareCodePointsV1,flattenTreeV1,manifestOfV1,parseRevisionV1,sealRevisionV1} from "../🌳️tree/🟦️.ts";
import {isSafeRevisionPathV1,REVISION_POLICY_V1,revisionErrorV1,type DigestV1,type InputManifestV1,type ManifestEntryV1,type PinV1,type RevisionPolicyV1,type RevisionV1,type StageReplyV1,type StageRequestV1} from "../🧬️schema/🟦️.ts";

/** 🔔️ Progress of staging; `done` of `total` inputs. */
export type RevisionProgressV1=(event:{phase:"stage";done:number;total:number})=>unknown;
/** 🧪️ Hooks around each read attempt: tests edit files at an exact phase. */
export type RevisionIoV1=Readonly<{beforeRead?(path:string,attempt:number):unknown;afterRead?(path:string,attempt:number):unknown}>;
/** 🧊️ A pinned revision: the revision, its sorted manifest, what staging did, and the pin that keeps it in the store. */
export type PinnedRevisionV1=Readonly<{revision:RevisionV1;manifest:InputManifestV1;pin:PinV1;absent:readonly string[];torn:readonly string[];staged:number;reused:number}>;
export type PinRevisionOptionsV1=Readonly<{
 cas:CasStoreV1;root:string;workspace:string;paths:readonly string[];holder:string;journalCursor:string;
 parent?:Readonly<{revision:RevisionV1;manifest:InputManifestV1}>;dirty?:Readonly<{paths:readonly string[];overflow?:boolean}>;
 policy?:RevisionPolicyV1;ttlMs?:number;now?():number;signal?:AbortSignal;progress?:RevisionProgressV1;io?:RevisionIoV1;
}>;

const holderOf=(base:string,revision:DigestV1):string=>`${base.replace(/[^A-Za-z0-9._-]/gu,"_").slice(0,60)}-${revision.slice(0,12)}-${Math.random().toString(36).slice(2,10)}`;
const nativePath=(root:string,path:string):string=>join(root,...path.split("/"));
const stableStat=(info:{size:number;mtimeMs:number;ino:number}):string=>`${info.size}:${info.mtimeMs}:${info.ino}`;
const missing=(error:unknown):boolean=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??"");

async function* chunksOf(path:string,chunkBytes:number,signal?:AbortSignal):AsyncGenerator<Uint8Array>{
 const handle=await open(path,"r");
 try{
  const buffer=Buffer.allocUnsafe(chunkBytes);
  for(;;){signal?.throwIfAborted();const {bytesRead}=await handle.read(buffer,0,chunkBytes,null);if(bytesRead===0)return;yield Buffer.from(buffer.subarray(0,bytesRead));}
 }finally{await handle.close();}
}

type StageIoV1=Readonly<{cas:CasStoreV1;root:string;signal?:AbortSignal;io?:RevisionIoV1}>;
type Staged=Readonly<{entry:ManifestEntryV1;torn:boolean}>|"absent";

async function stageOne(options:StageIoV1,policy:RevisionPolicyV1,path:string):Promise<Staged>{
 const native=nativePath(options.root,path);
 for(let attempt=1;;attempt++){
  options.signal?.throwIfAborted();
  await options.io?.beforeRead?.(path,attempt);
  let before;try{before=await lstat(native);}catch(error){if(missing(error))return "absent";throw error;}
  let stored:CasDigestV1,mode:ManifestEntryV1["mode"];
  try{
   if(before.isSymbolicLink()){mode="symlink";stored=await options.cas.put(new TextEncoder().encode((await readlink(native)).split(sep).join("/")),options.signal);}
   else if(before.isFile()){mode=process.platform!=="win32"&&(before.mode&0o111)!==0?"executable":"file";stored=await options.cas.put(chunksOf(native,policy.chunkBytes,options.signal),options.signal);}
   else throw revisionErrorV1("unsupported-entry",{path});
  }catch(error){if(missing(error))return "absent";throw error;}
  await options.io?.afterRead?.(path,attempt);
  let after;try{after=await lstat(native);}catch(error){if(missing(error))after=undefined;else throw error;}
  const steady=after!==undefined&&stableStat(before)===stableStat(after);
  if(steady||attempt>=policy.maximumAttempts)return{entry:{path,mode,digest:stored.hash,size:stored.size},torn:!steady};
 }
}

async function parallel<T,R>(items:readonly T[],limit:number,work:(item:T,index:number)=>Promise<R>,signal?:AbortSignal):Promise<R[]>{
 const results=new Array<R>(items.length);let next=0;
 const lane=async():Promise<void>=>{for(;;){signal?.throwIfAborted();const index=next++;if(index>=items.length)return;results[index]=await work(items[index]!,index);}};
 await Promise.all(Array.from({length:Math.min(limit,items.length)},lane));return results;
}

/** 📖️ Loads a revision by id from the store. */
export async function loadRevisionV1(cas:CasStoreV1,id:DigestV1):Promise<RevisionV1>{
 const bytes=await cas.read(id);if(bytes===undefined)throw revisionErrorV1("unknown-revision",{revision:id});
 return parseRevisionV1(id,bytes);
}

/** 🔎️ A revision with its sorted manifest and every store address it needs: file contents, tree objects and the revision object itself. */
export async function loadManifestV1(cas:CasStoreV1,id:DigestV1,signal?:AbortSignal):Promise<{revision:RevisionV1;manifest:InputManifestV1;digests:DigestV1[]}> {
 const revision=await loadRevisionV1(cas,id),trees=new Set<DigestV1>();
 const read=async(digest:DigestV1):Promise<Uint8Array|undefined>=>{trees.add(digest);return cas.read(digest);};
 const entries=(await Promise.all(revision.roots.map(root=>flattenTreeV1(root.tree,read,signal)))).flat();
 return{revision,manifest:manifestOfV1(revision,entries),digests:[...new Set([revision.id,...trees,...entries.map(entry=>entry.digest)])]};
}

/** 📌️ Pins a revision of exactly the given input closure. Each input is read once into the shared store (re-stat loop, bounded; a file that keeps changing is recorded with the content actually read); unchanged inputs of the parent revision are reused without reading. Edits after this call never change the result. */
export async function pinRevisionV1(options:PinRevisionOptionsV1):Promise<PinnedRevisionV1>{
 const policy=options.policy??REVISION_POLICY_V1,now=options.now??Date.now,cas=options.cas;
 if(options.paths.length>policy.maximumPaths)throw revisionErrorV1("too-many-paths",{count:options.paths.length,limit:policy.maximumPaths});
 for(const path of options.paths)if(!isSafeRevisionPathV1(path))throw revisionErrorV1("unsafe-path",{path});
 const paths=[...new Set(options.paths)].sort(compareCodePointsV1),dirty=new Set(options.dirty?.paths??[]),reusable=options.parent!==undefined&&options.dirty?.overflow!==true;
 const parentEntries=new Map(options.parent?.manifest.entries.map(entry=>[entry.path,entry]));
 const keep=paths.map(path=>reusable&&!dirty.has(path)?parentEntries.get(path):undefined);
 const present=await cas.has(keep.map(entry=>entry?.digest??"0".repeat(64)));
 let done=0,lastReport=0;
 const report=(force:boolean):void=>{const at=now();if(force||at-lastReport>=policy.progressMs){lastReport=at;options.progress?.({phase:"stage",done,total:paths.length});}};
 const staged=await parallel(paths,policy.concurrency,async(path,index):Promise<Staged|"reused">=>{
  const result:Staged|"reused"=keep[index]!==undefined&&present[index]?"reused":await stageOne(options,policy,path);
  done++;report(false);return result;
 },options.signal);
 report(true);
 const entries:ManifestEntryV1[]=[],absent:string[]=[],torn:string[]=[];let reused=0,count=0;
 staged.forEach((result,index)=>{
  if(result==="reused"){entries.push(keep[index]!);reused++;}
  else if(result==="absent")absent.push(paths[index]!);
  else{entries.push(result.entry);count++;if(result.torn)torn.push(paths[index]!);}
 });
 return sealEntriesV1({...options,policy,now,entries,absent,torn,staged:count,reused});
}

/** 🔏️ Seals entries whose contents are already in the store into a revision and pins it: the tree objects and the revision object are stored, every address is held by one pin. */
export async function sealEntriesV1(options:Readonly<{cas:CasStoreV1;workspace:string;journalCursor:string;holder:string;entries:readonly ManifestEntryV1[];absent?:readonly string[];torn?:readonly string[];staged?:number;reused?:number;parent?:Readonly<{revision:RevisionV1;manifest:InputManifestV1}>;policy?:RevisionPolicyV1;ttlMs?:number;now?():number;signal?:AbortSignal}>):Promise<PinnedRevisionV1>{
 const policy=options.policy??REVISION_POLICY_V1,now=options.now??Date.now,cas=options.cas,entries=[...options.entries].sort((left,right)=>compareCodePointsV1(left.path,right.path));
 const tree=buildTreeV1(entries);
 const {revision,bytes}=sealRevisionV1({journalCursor:options.journalCursor,...options.parent===undefined?{}:{parent:options.parent.revision.id},roots:[{path:"",size:tree.size,tree:tree.root}],workspace:options.workspace});
 await parallel([...tree.objects.entries()],policy.concurrency,async([digest,object])=>{if(!(await cas.has([digest]))[0])await cas.put(object,options.signal);},options.signal);
 await cas.put(bytes,options.signal);
 const holder=holderOf(options.holder,revision.id),ttl=options.ttlMs??policy.pinTtlMs;
 await cas.pin(holder,[revision.id,...tree.objects.keys(),...entries.map(entry=>entry.digest)],ttl);
 return Object.freeze({revision,manifest:manifestOfV1(revision,entries),pin:Object.freeze({version:1 as const,revision:revision.id,holder,expiresMs:now()+ttl}),absent:options.absent??[],torn:options.torn??[],staged:options.staged??entries.length,reused:options.reused??0});
}

/** 🔓️ Releases one pin. A revision stays in the store while any other holder pins it; with none left its objects are plain least-recently-accessed candidates. */
export const releasePinV1=(cas:CasStoreV1,pin:PinV1):Promise<void>=>cas.release(pin.holder);

/** 🔁️ Renews a pin for another lease period; fails with `unknown-revision` when an object already left the store. */
export async function renewPinV1(cas:CasStoreV1,pin:PinV1,options:{ttlMs?:number;now?():number;signal?:AbortSignal}={}):Promise<PinV1>{
 const ttl=options.ttlMs??REVISION_POLICY_V1.pinTtlMs,now=options.now??Date.now;
 const {digests}=await loadManifestV1(cas,pin.revision,options.signal);
 if(!(await cas.has(digests)).every(Boolean))throw revisionErrorV1("unknown-revision",{revision:pin.revision});
 await cas.pin(pin.holder,digests,ttl);
 return Object.freeze({...pin,expiresMs:now()+ttl});
}

/** 📌️ Pins a revision, runs `work` with it, renews the pin while `work` runs and releases it afterwards, also on failure or cancellation. */
export async function withPinnedRevisionV1<T>(options:PinRevisionOptionsV1,work:(pinned:PinnedRevisionV1)=>Promise<T>):Promise<T>{
 const pinned=await pinRevisionV1(options),policy=options.policy??REVISION_POLICY_V1;
 let current=pinned.pin;
 const timer=setInterval(()=>{renewPinV1(options.cas,current,options.now===undefined?{}:{now:options.now}).then(next=>{current=next;},()=>undefined);},policy.renewIntervalMs);
 timer.unref?.();
 try{return await work(pinned);}finally{clearInterval(timer);await releasePinV1(options.cas,current);}
}

/** 📥️ Ensures the content of the requested paths of a revision is in the store and returns their addresses; content that is gone is restaged only if the workspace still holds exactly that digest. */
export async function stageRevisionV1(cas:CasStoreV1,request:StageRequestV1,options:{root:string;signal?:AbortSignal;policy?:RevisionPolicyV1}):Promise<StageReplyV1>{
 const {manifest}=await loadManifestV1(cas,request.revision,options.signal),byPath=new Map(manifest.entries.map(entry=>[entry.path,entry]));
 const wanted=request.paths.map(path=>{const entry=byPath.get(path);if(entry===undefined)throw revisionErrorV1("unsafe-path",{path});return entry;});
 const present=await cas.has(wanted.map(entry=>entry.digest)),policy=options.policy??REVISION_POLICY_V1;
 const refs=await parallel(wanted,policy.concurrency,async(entry,index)=>{
  if(!present[index]){
   const again=await stageOne({cas,root:options.root,...options.signal===undefined?{}:{signal:options.signal}},policy,entry.path);
   if(again==="absent"||again.entry.digest!==entry.digest)throw revisionErrorV1("content-evicted",{path:entry.path});
  }
  return{hash:entry.digest,size:entry.size};
 },options.signal);
 return{revision:request.revision,refs};
}
