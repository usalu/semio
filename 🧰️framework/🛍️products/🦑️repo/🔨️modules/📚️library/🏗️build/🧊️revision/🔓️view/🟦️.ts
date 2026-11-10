import {readFileSync} from "node:fs";
import {chmod,mkdir,rm,symlink} from "node:fs/promises";
import {dirname,resolve,sep} from "node:path";
import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {compareCodePointsV1,revisionDigestV1} from "../🌳️tree/🟦️.ts";
import {loadManifestV1} from "../📌️pin/🟦️.ts";
import {REVISION_POLICY_V1,revisionErrorV1,type DigestV1,type InputManifestV1,type RevisionPolicyV1} from "../🧬️schema/🟦️.ts";

/** 🔭️ The read side of a pinned revision, shaped like the derived-node `WorldPortV1` (`read`, `files`) so planning and executors see the frozen content, never the live workspace. */
export type PinnedWorldV1=Readonly<{manifest:InputManifestV1;read(path:string):Promise<Uint8Array|undefined>;files(root:string,signal?:AbortSignal):Promise<readonly string[]>}>;

/** 🔭️ Opens the frozen view of a revision from the store. With `verify` every read is re-hashed against the manifest. */
export async function openPinnedWorldV1(cas:CasStoreV1,revision:DigestV1,options:{verify?:boolean;signal?:AbortSignal}={}):Promise<PinnedWorldV1>{
 const {manifest}=await loadManifestV1(cas,revision,options.signal),byPath=new Map(manifest.entries.map(entry=>[entry.path,entry]));
 return Object.freeze({
  manifest,
  async read(path){
   const entry=byPath.get(path);if(entry===undefined)return undefined;
   const bytes=await cas.read(entry.digest);if(bytes===undefined)throw revisionErrorV1("content-evicted",{path});
   if(options.verify&&revisionDigestV1(bytes)!==entry.digest)throw revisionErrorV1("content-evicted",{path});
   return bytes;
  },
  async files(root,signal){
   signal?.throwIfAborted();
   const prefix=root===""||root==="."?"":root.replace(/\/$/u,"")+"/";
   return manifest.entries.filter(entry=>entry.path.startsWith(prefix)).map(entry=>entry.path);
  },
 });
}

/** 🔭️ A synchronous read-only view of a pinned revision for the walkers of inference, which read synchronously: bytes, kind and listings come from the manifest and the store objects, never from the live workspace. */
export type PinnedWorldSyncV1=Readonly<{manifest:InputManifestV1;readSync(path:string):Uint8Array|undefined;kindSync(path:string):"file"|"executable"|"symlink"|"directory"|undefined;listSync(directory:string):readonly Readonly<{name:string;directory:boolean}>[]|undefined;filesSync(root:string):readonly string[]}>;

/** 🔭️ Opens the synchronous frozen view of an already loaded manifest. With `verify` every read is re-hashed against the manifest. */
export function openPinnedWorldSyncV1(cas:CasStoreV1,manifest:InputManifestV1,options:{verify?:boolean}={}):PinnedWorldSyncV1{
 const byPath=new Map(manifest.entries.map(entry=>[entry.path,entry])),children=new Map<string,Map<string,boolean>>();
 for(const entry of manifest.entries){
  const parts=entry.path.split("/");
  for(let index=0;index<parts.length;index++){
   const parent=parts.slice(0,index).join("/"),map=children.get(parent)??children.set(parent,new Map()).get(parent)!;
   map.set(parts[index]!,index<parts.length-1||map.get(parts[index]!)===true);
  }
 }
 const clean=(path:string):string=>path===""||path==="."?"":path.replace(/\/$/u,"");
 return Object.freeze({
  manifest,
  readSync(path){
   const entry=byPath.get(path);if(entry===undefined)return undefined;
   let bytes:Uint8Array;try{bytes=readFileSync(cas.objectPath(entry.digest));}catch{throw revisionErrorV1("content-evicted",{path});}
   if(options.verify&&revisionDigestV1(bytes)!==entry.digest)throw revisionErrorV1("content-evicted",{path});
   return bytes;
  },
  kindSync(path){const clear=clean(path),entry=byPath.get(clear);return entry!==undefined?(entry.mode as "file"|"executable"|"symlink"):children.has(clear)?"directory":undefined;},
  listSync(directory){const map=children.get(clean(directory));return map===undefined?undefined:[...map].map(([name,isDirectory])=>({name,directory:isDirectory})).sort((left,right)=>compareCodePointsV1(left.name,right.name));},
  filesSync(root){const prefix=clean(root)===""?"":clean(root)+"/";return manifest.entries.filter(entry=>entry.path.startsWith(prefix)).map(entry=>entry.path);},
 });
}

/** 🏗️ Materialises a revision below `destination` so an executor reads the frozen inputs from a directory: reflink where the filesystem has it, copy otherwise (never a writable hardlink into the store). Returns the number of files placed. */
export async function materializeRevisionV1(cas:CasStoreV1,revision:DigestV1,destination:string,options:{paths?:readonly string[];signal?:AbortSignal;progress?:(event:{phase:"materialize";done:number;total:number})=>unknown;policy?:RevisionPolicyV1}={}):Promise<number>{
 const {manifest}=await loadManifestV1(cas,revision,options.signal),policy=options.policy??REVISION_POLICY_V1,wanted=options.paths===undefined?undefined:new Set(options.paths);
 const entries=manifest.entries.filter(entry=>wanted===undefined||wanted.has(entry.path)),base=resolve(destination);
 let done=0,next=0;
 const lane=async():Promise<void>=>{
  for(;;){
   options.signal?.throwIfAborted();
   const entry=entries[next++];if(entry===undefined)return;
   const target=resolve(base,...entry.path.split("/"));
   if(target!==base&&!target.startsWith(base+sep))throw revisionErrorV1("unsafe-path",{path:entry.path});
   await mkdir(dirname(target),{recursive:true});
   if(entry.mode==="symlink"){
    const text=await cas.read(entry.digest);if(text===undefined)throw revisionErrorV1("content-evicted",{path:entry.path});
    await rm(target,{force:true});await symlink(Buffer.from(text).toString("utf8"),target);
   }else{
    if(await cas.link(entry.digest,target,"auto")===undefined)throw revisionErrorV1("content-evicted",{path:entry.path});
    await chmod(target,entry.mode==="executable"?0o555:0o444).catch(()=>undefined);
   }
   done++;options.progress?.({phase:"materialize",done,total:entries.length});
  }
 };
 await mkdir(base,{recursive:true});
 await Promise.all(Array.from({length:Math.min(policy.concurrency,entries.length)},lane));
 return done;
}

