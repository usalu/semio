import {repositoryCachePartitionRootDataV1} from "../📂️paths/🟦️.ts";
import {dirname,join} from "node:path";
import {lstatSync,mkdirSync} from "node:fs";
import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../🟦️.ts";
import {openNativePlatformStorageV1,requireNativeStoragePathV1,type NativeImageStorageV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,retireResourceLeasesV1,retireResourceQueueClaimsV1,requireResourceLeaseWriterProofV1,resourceLeasePathV1,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{requireIssuedResourceControlRefinementV1}from"../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import parentSchema from"./📂️parents/🧬️schema/🔣️.json"with{type:"json"};

/** 📦️ Names the closed authored generated-storage partitions. */
export type {RepositoryCachePartitionIdV1} from "../🟦️.ts";

/** 🗂️ Resolves only the authored canonical fixed backing and its compact mounted root. */
export function repositoryCachePartitionDirectoryV1(workspace:string,id:RepositoryCachePartitionIdV1):string{const partition=repositoryCacheBudgetV1(workspace).partitions[id];if(!partition)throw Error("Declared generated storage partition required");return partition.directory;}
/** 🧭️ Gives writer planners the same actual fixed mount namespace without provisioning. */
export function repositoryCachePartitionRootV1(workspace:string,id:RepositoryCachePartitionIdV1):string{return repositoryCachePartitionRootDataV1(workspace,id);}

/** 🗃️ Names queued lease writers inside their separate physically finite backing. */
export function repositoryLeaseDirectoryV1(workspace:string):string{return requireMaterializedPathV1(join(repositoryCachePartitionRootV1(workspace,"leases"),"data"));}

/** 📂️ Materializes only the two authored cache ancestors before the genuine fixed lease-image opener. */
function prepareRepositoryCacheParentsV1(workspace:string,control:ResourceLeaseControl):void{
 requireIssuedResourceControlRefinementV1(control,control);repositoryCachePartitionRootDataV1(workspace,"leases");const paths=parentSchema.definitions.Protocol.const.ancestors.map(path=>requireMaterializedPathV1(join(workspace,path))),retained=new Map<string,{dev:bigint;ino:bigint}>();
 const inspect=(path:string,optional=false):boolean=>{checkResourceLeaseControl(control);let row;try{row=lstatSync(path,{bigint:true});}catch(error){if(optional&&(error as NodeJS.ErrnoException|undefined)?.code==="ENOENT"){checkResourceLeaseControl(control);return false;}throw error;}checkResourceLeaseControl(control);const previous=retained.get(path);if(!row.isDirectory()||row.isSymbolicLink()||previous&&(row.dev!==previous.dev||row.ino!==previous.ino))throw Error("Exact physical cache ancestor required");if(!previous)retained.set(path,{dev:row.dev,ino:row.ino});return true;},recheck=()=>{for(const path of retained.keys())inspect(path);};
 for(let path=workspace;;path=dirname(path)){inspect(path);if(dirname(path)===path)break;}for(const path of paths)inspect(path,true);
 for(const path of paths){recheck();if(!inspect(path,true)){checkResourceLeaseControl(control);try{mkdirSync(path);}catch(error){if((error as NodeJS.ErrnoException|undefined)?.code!=="EEXIST")throw error;}checkResourceLeaseControl(control);}inspect(path);recheck();}
 requireIssuedResourceControlRefinementV1(control,control);
}

async function openRepositoryLeasePartitionV1(workspace:string,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{
 prepareRepositoryCacheParentsV1(workspace,control);
 const budget=repositoryCacheBudgetV1(workspace),partition=budget.partitions.leases,open=()=>openNativePlatformStorageV1({directory:partition.directory,leaseDirectory:budget.controlDirectory,filesystemBytes:partition.filesystemBytes,maximumPhysicalBytes:partition.capacityBytes,control,maximumLeaseDatabaseBytes:16384});let image=await open();
 try{const directory=repositoryLeaseDirectoryV1(workspace);await image.recheck();requireNativeStoragePathV1(image.root,directory);await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>{
  if(custody.kind!=="kernel-ex")throw Error("Actual exclusive lease image maintenance required");const resource="native-fixed-image:"+partition.directory;requireResourceLeaseWriterProofV1(custody.proof,{directory:budget.controlDirectory,resource,path:resourceLeasePathV1(budget.controlDirectory,resource)},custody.control);await image.recheck();const ordinary=await retireResourceLeasesV1(directory,custody.control,0),claims=await retireResourceQueueClaimsV1(directory,custody.control,0);await image.recheck();console.log("[DEBUG] Actual exclusive lease image retired authenticated idle databases "+JSON.stringify({ordinary,claims}));
 });image=await open();await image.recheck();return image;
 }catch(error){try{await image.close();}catch{}throw error;}
}

/** 🎟️ Opens the authored finite backing under original caller custody; unsupported enforcement remains a refusal. */
export async function openRepositoryCachePartitionV1(workspace:string,id:RepositoryCachePartitionIdV1,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{
 checkResourceLeaseControl(control);const budget=repositoryCacheBudgetV1(workspace),partition=budget.partitions[id];if(!partition||partition.capacityBytes%1048576||partition.capacityBytes<34603008)throw Error("Finite generated cache partition required");
 if(id==="leases")return openRepositoryLeasePartitionV1(workspace,control);
 const leases=await openRepositoryCachePartitionV1(workspace,"leases",control);let image:NativeImageStorageV1|undefined;try{const leaseDirectory=repositoryLeaseDirectoryV1(workspace);await leases.recheck();requireNativeStoragePathV1(leases.root,leaseDirectory);mkdirSync(leaseDirectory,{recursive:true});image=await openNativePlatformStorageV1({directory:partition.directory,leaseDirectory,filesystemBytes:partition.filesystemBytes,maximumPhysicalBytes:partition.capacityBytes,control});const admitted=image;return Object.freeze({...admitted,close:async()=>{try{await admitted.close();}finally{await leases.close();}}});}catch(error){try{await image?.close();}finally{await leases.close();}throw error;}
}
