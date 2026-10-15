import type{BigIntStats}from"node:fs";
import{lstat}from"node:fs/promises";
import{join}from"node:path";
import{checkResourceLeaseControl}from"../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{requireNativeStoragePathV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type{RepositoryToolingStoreV1}from"../../../../../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
const policy=schema.definitions.Policy.const;
function path(value:string):void{if(value!=="node_modules"&&!value.startsWith("node_modules/")||value.includes("\\")||value.includes("\0")||value.length>policy.maximumPath||value.split("/").some(part=>!part||part==="."||part===".."))throw Error("Closed SDK source path required");}
/** 🌳️ Projects exact portable parent identities without granting filesystem authority. */
export function nxSdkSourceAncestorsV1(value:string):readonly string[]{path(value);const parts=value.split("/");return parts.slice(0,-1).map((_,index)=>parts.slice(0,index+1).join("/"));}
const same=(a:BigIntStats,b:BigIntStats)=>a.dev===b.dev&&a.ino===b.ino&&a.mode===b.mode&&a.mtimeMs===b.mtimeMs&&a.ctimeMs===b.ctimeMs&&a.mtimeNs===b.mtimeNs&&a.ctimeNs===b.ctimeNs;
export interface NxSdkSourceTreeV1{parent(path:string):Promise<string>;directory(path:string,facts:BigIntStats):void;recheck():Promise<void>}
/** 🧷️ Retains every unique no-follow directory identity across one complete rooted read operation. */
export async function withNxSdkSourceTreeV1<T>(directory:string,store:RepositoryToolingStoreV1,body:(tree:NxSdkSourceTreeV1)=>Promise<T>):Promise<T>{
 requireNativeStoragePathV1(store.root,directory);const root=await lstat(directory,{bigint:true});if(!root.isDirectory()||root.isSymbolicLink())throw Error("Original SDK source root required");const directories=new Map<string,BigIntStats>([["",root]]);let live=true,accepting=true;const pending=new Set<Promise<unknown>>(),failures:unknown[]=[];const track=<R>(run:()=>Promise<R>):Promise<R>=>{if(!live||!accepting)return Promise.reject(Error("SDK source traversal released"));if(pending.size>=policy.maximumEntries)return Promise.reject(Error("SDK source traversal continuation ceiling exceeded"));const work=run();pending.add(work);void work.then(()=>pending.delete(work),error=>{pending.delete(work);if(!failures.length)failures.push(error);});return work;};
 const absolute=(value:string)=>{checkResourceLeaseControl(store.control);if(!live)throw Error("SDK source traversal released");path(value);const absolute=join(directory,value);if(absolute.length>policy.maximumPath)throw Error("SDK source traversal path ceiling exceeded");return absolute;};
 const check=async(value:string)=>{checkResourceLeaseControl(store.control);const facts=directories.get(value);if(!facts)throw Error("Anchored SDK source parent required");const next=await lstat(value?join(directory,value):directory,{bigint:true});checkResourceLeaseControl(store.control);if(!next.isDirectory()||next.isSymbolicLink()||!same(facts,next))throw Error("Anchored SDK source directory changed");};
 const recheck=async()=>{requireNativeStoragePathV1(store.root,directory);for(const value of directories.keys())await check(value);};
 const tree:NxSdkSourceTreeV1=Object.freeze({parent:(value:string)=>track(async()=>{const result=absolute(value),parent=nxSdkSourceAncestorsV1(value).at(-1)??"";await check(parent);return result;}),directory:(value:string,facts:BigIntStats)=>{if(!accepting)throw Error("SDK source traversal released");absolute(value);if(directories.size>=policy.maximumEntries||directories.has(value)||!facts.isDirectory()||facts.isSymbolicLink()||facts.dev!==root.dev)throw Error("Original bounded SDK directory required");directories.set(value,Object.freeze(facts));},recheck:()=>track(recheck)});
 let value:T|undefined,failed=false,failure:unknown;try{try{value=await body(tree);}catch(error){failed=true;failure=error;}accepting=false;while(pending.size)await Promise.allSettled([...pending]);if(failed)throw failure;if(failures.length)throw failures[0];await recheck();checkResourceLeaseControl(store.control);return value as T;}finally{accepting=false;live=false;}
}
