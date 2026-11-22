import {createHash} from "node:crypto";
import {mkdir,open,readdir,readFile,rename,unlink,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {requireSlotShapeV1} from "../../🪢️slots/🟦️.ts";
import {parseRemoteCacheTreeV1,type RemoteCacheDigestV1} from "../🔬️action-result/🟦️.ts";

/** 🏷️ The retention sidecar of one server entry (`Meta` in the slot schema): its slot, when it was stored or last claimed (server clock) and the blobs it references.
 * @see ../../🪢️slots/🧬️schema/🔣️.json */
export type RemoteCacheMetaV1=Readonly<{version:1;slot:string;storedAt:number;refs:readonly string[]}>;

/** 📁️ Where sidecars live below a store root: a dot directory, so the entry scan never sees them. */
export const REMOTE_CACHE_META_DIRECTORY_V1=".meta";

const META=/^[0-9a-f]{64}$/u;

/** 🧭️ The sidecar path of an entry given its slash-separated key below the root (`<namespace>/<kind>/<first two characters>/<name>`). */
export const remoteCacheMetaPathV1=(root:string,relative:string):string=>join(root,REMOTE_CACHE_META_DIRECTORY_V1,...relative.split("/"))+".json";

const valid=(value:unknown):value is RemoteCacheMetaV1=>{
 try{requireSlotShapeV1("Meta",value,"meta");return true;}catch{return false;}
};

/** ✍️ Publishes a sidecar atomically: written beside the staging area and renamed into place, so a reader sees the old or the new document, never a part of one; the entry must be in place first, so a crash leaves a value without a sidecar (kept by the age and size rules) and never a sidecar that condemns a value. */
export async function writeRemoteCacheMetaV1(root:string,staging:string,relative:string,meta:RemoteCacheMetaV1):Promise<void>{
 const target=remoteCacheMetaPathV1(root,relative),temporary=join(staging,`meta-${process.pid}-${Math.random().toString(36).slice(2)}`);
 await mkdir(dirname(target),{recursive:true});await writeFile(temporary,JSON.stringify(meta));
 try{await rename(temporary,target);}catch(error){await unlink(temporary).catch(()=>undefined);throw error;}
}

/** 📖️ Reads one sidecar; a missing or malformed one reads as `undefined` (a malformed one is removed, so the entry falls back to the age and size rules). */
export async function readRemoteCacheMetaV1(root:string,relative:string):Promise<RemoteCacheMetaV1|undefined>{
 const path=remoteCacheMetaPathV1(root,relative);let text;try{text=await readFile(path,"utf8");}catch{return undefined;}
 try{const value=JSON.parse(text);if(valid(value))return value;}catch{}
 await unlink(path).catch(()=>undefined);return undefined;
}

/** 🗑️ Removes one sidecar; a missing one is not an error. */
export async function removeRemoteCacheMetaV1(root:string,relative:string):Promise<void>{
 try{await unlink(remoteCacheMetaPathV1(root,relative));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
}

/** 🚶️ Reads every sidecar below a root, keyed by the entry key; names that are not 64-hex-or-key sidecars and malformed documents are skipped, and sidecars whose entry is not in `present` are removed (a crash between the removal of an entry and its sidecar leaves exactly that). */
export async function scanRemoteCacheMetaV1(root:string,present:Readonly<{has(relative:string):boolean}>,signal?:AbortSignal,width:number=64):Promise<Map<string,RemoteCacheMetaV1>>{
 const found=new Map<string,RemoteCacheMetaV1>(),base=join(root,REMOTE_CACHE_META_DIRECTORY_V1);
 const walk=async(directory:string,depth:number,prefix:readonly string[]):Promise<void>=>{
  signal?.throwIfAborted();let rows;try{rows=await readdir(directory,{withFileTypes:true});}catch{return;}
  const files:{relative:string;name:string}[]=[];
  for(const row of rows){
   if(depth<3){if(row.isDirectory())await walk(join(directory,row.name),depth+1,[...prefix,row.name]);continue;}
   if(!row.isFile()||!row.name.endsWith(".json"))continue;
   files.push({relative:[...prefix,row.name.slice(0,-5)].join("/"),name:row.name});
  }
  for(let at=0;at<files.length;at+=width){if(at>0&&width<=16)await new Promise<void>(done=>{setImmediate(done);});await Promise.all(files.slice(at,at+width).map(async({relative,name})=>{
   if(!present.has(relative)){await unlink(join(directory,name)).catch(()=>undefined);return;}
   const meta=await readRemoteCacheMetaV1(root,relative);if(meta)found.set(relative,meta);
  }));}
 };
 await walk(base,0,[]);return found;
}

/** 📏️ Reads one addressed tree within the same byte ceiling as server validation. */
export async function readRemoteCacheTreeBytesV1(path:string,size:number):Promise<Uint8Array|undefined>{
 if(!Number.isSafeInteger(size)||size<1||size>16*1048576)return undefined;
 const file=await open(path,"r").catch(()=>undefined);if(!file)return undefined;
 try{
  const stat=await file.stat();if(!stat.isFile()||stat.size!==size)return undefined;
  const bytes=Buffer.allocUnsafe(size);let at=0;
  while(at<size){const{bytesRead}=await file.read(bytes,at,size-at,at);if(bytesRead===0)return undefined;at+=bytesRead;}
  return bytes;
 }catch{return undefined;}finally{await file.close();}
}

/** 🔗️ Keeps the file digests inside every output Tree alongside its own CAS digest. */
export async function readRemoteCacheReferencesV1(path:string,kind:"ac"|"cas"|"nx",parse:(bytes:Uint8Array)=>Readonly<{references:readonly {hash:string}[];trees:readonly RemoteCacheDigestV1[]}>,readTree?:(tree:RemoteCacheDigestV1)=>Promise<Uint8Array|undefined>):Promise<readonly string[]>{
 if(kind==="cas")return [];
 let bytes;try{bytes=await readFile(path);}catch{return [];}
 if(kind==="nx"){try{const row=JSON.parse(new TextDecoder().decode(bytes)) as {hash?:unknown};return typeof row.hash==="string"&&META.test(row.hash)?[row.hash]:[];}catch{return [];}}
 let parsed;try{parsed=parse(bytes);}catch{return [];}
 const refs=new Set([...parsed.references,...parsed.trees].map(row=>row.hash)),namespace=dirname(dirname(dirname(path)));
 for(const tree of parsed.trees){
  try{
   const data=await readTree?.(tree)??await readRemoteCacheTreeBytesV1(join(namespace,"cas",tree.hash.slice(0,2),tree.hash),tree.size);
   if(data&&data.length===tree.size&&createHash("sha256").update(data).digest("hex")===tree.hash)for(const leaf of parseRemoteCacheTreeV1(data))refs.add(leaf.hash);
  }catch{}
 }
 return [...refs];
}

/** 🔍️ Audits a store directory against the keys the current worktree state needs: an entry with a slot that is not among them and neither the newest key of its slot nor within the grace is stray. An empty `stray` list means the store holds the synchronized state (plus what the grace keeps) only; entries without a slot are counted, not judged. The current keys come from outside (for Nx the task hashes of the worktree), so this is an oracle that does not use the supersession rule. */
export async function auditRemoteCacheDirectoryV1(root:string,currentKeys:ReadonlySet<string>,options:Readonly<{graceMilliseconds:number;now?:number;signal?:AbortSignal}>):Promise<Readonly<{version:1;cache:string;current:number;grace:number;stray:readonly string[];unslotted:number}>>{
 const {scanRemoteCacheDirectoryV1}=await import("../🧹️collector/🟦️.ts"),{classifySlotsV1}=await import("../../🪢️slots/🟦️.ts"),index=await scanRemoteCacheDirectoryV1(root,options.signal?{signal:options.signal}:{}),metas=await scanRemoteCacheMetaV1(root,index,options.signal),now=options.now??Date.now();
 const verdicts=classifySlotsV1([...index].map(([key,row])=>{const meta=metas.get(key);return {key,slot:meta?.slot??null,storedAt:meta?.storedAt??0,accessed:row.accessed,size:row.size};}),{now,graceMilliseconds:options.graceMilliseconds});
 let current=0,grace=0,unslotted=0;const stray:string[]=[];
 for(const[key]of index){
  const verdict=verdicts.get(key)!,name=key.split("/").pop()!,needed=currentKeys.has(name);
  if(verdict.state==="unslotted"){unslotted++;continue;}
  if(needed)current++;
  else if(verdict.state==="retained"||metas.get(key)!==undefined&&now-metas.get(key)!.storedAt<options.graceMilliseconds)grace++;
  else stray.push(key);
 }
 return Object.freeze({version:1,cache:root,current,grace,stray:Object.freeze(stray.sort()),unslotted});
}
