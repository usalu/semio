import{lstat}from"node:fs/promises";
import type{BigIntStats}from"node:fs";
import{join,relative,sep}from"node:path";
import{withRepositoryToolingOperationV1,type RepositoryToolingStoreV1}from"../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{requireNativeStorageReservationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{requireNativeStoragePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};

/** 🪪️ Separates published device DATA from an exact inode in the fresh authenticated namespace. */
export function rustupGenerationDirectoryMatchesV1(published:unknown,current:Readonly<{device:string;inode:string;directory:boolean;alias:boolean}>,rootDevice:string):boolean{return validateJsonSchemaSubset(schema.definitions.GenerationReceipt.properties.directory,published,schema).length===0&&current.directory&&!current.alias&&current.device===rootDevice&&current.inode===(published as{inode:string}).inode;}

function sameRustupDirectoryFactsV1(before:BigIntStats,after:BigIntStats,leaf:boolean):boolean{return after.isDirectory()&&!after.isSymbolicLink()&&before.dev===after.dev&&before.ino===after.ino&&before.mode===after.mode&&(!leaf||before.size===after.size&&before.nlink===after.nlink&&before.mtimeNs===after.mtimeNs&&before.ctimeNs===after.ctimeNs);}

/** 🧷️ Retains live nofollow ancestry while immutable generation content is verified under its same Original. */
export function withRustupDirectoryIdentityV1<T>(workspace:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl,directory:string,published:unknown,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryToolingOperationV1(workspace,store,control,async actual=>{
 requireNativeStorageReservationV1(store.storage,store.control);await store.storage.recheck();directory=requireNativeStoragePathV1(store.root,directory);const parts=relative(store.root,directory).split(sep);if(directory===store.root||parts.length>schema.definitions.GenerationDirectoryPolicy.const.maximumAncestors)throw Error("Exact bounded Rustup generation ancestry required");
 const ancestors:{path:string;facts:BigIntStats}[]=[];let current=store.root;for(const part of["",...parts]){if(part)current=join(current,part);const facts=await lstat(requireNativeStoragePathV1(store.root,current),{bigint:true});if(!facts.isDirectory()||facts.isSymbolicLink()||ancestors.length&&facts.dev!==ancestors[0]!.facts.dev)throw Error("Original immutable Rustup generation identity changed");ancestors.push({path:current,facts});}const leaf=ancestors.at(-1)!.facts;if(!rustupGenerationDirectoryMatchesV1(published,{device:String(leaf.dev),inode:String(leaf.ino),directory:leaf.isDirectory(),alias:leaf.isSymbolicLink()},String(ancestors[0]!.facts.dev)))throw Error("Original immutable Rustup generation identity changed");
 const recheck=async()=>{checkResourceLeaseControl(actual);await store.storage.recheck();for(const[index,row]of ancestors.entries())if(!sameRustupDirectoryFactsV1(row.facts,await lstat(requireNativeStoragePathV1(store.root,row.path),{bigint:true}),index===ancestors.length-1))throw Error("Original immutable Rustup generation identity changed");checkResourceLeaseControl(actual);};await recheck();let failed=false;try{return await body(actual);}catch(error){failed=true;throw error;}finally{try{await recheck();}catch(error){if(!failed)throw error;}}
 });}
