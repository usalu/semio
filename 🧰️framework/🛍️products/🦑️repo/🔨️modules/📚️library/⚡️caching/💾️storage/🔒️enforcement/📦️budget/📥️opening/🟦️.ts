import {join} from "node:path";
import {mkdirSync} from "node:fs";
import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../🟦️.ts";
import {openNativePlatformStorageV1,nativeImageRootV1,requireNativeStoragePathV1,type NativeImageStorageV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

/** 📦️ Names the closed authored generated-storage partitions. */
export type {RepositoryCachePartitionIdV1} from "../🟦️.ts";

/** 🗂️ Resolves only the authored canonical fixed backing and its compact mounted root. */
export function repositoryCachePartitionDirectoryV1(workspace:string,id:RepositoryCachePartitionIdV1):string{const partition=repositoryCacheBudgetV1(workspace).partitions[id];if(!partition)throw Error("Declared generated storage partition required");return partition.directory;}
/** 🧭️ Gives writer planners the same actual fixed mount namespace without provisioning. */
export function repositoryCachePartitionRootV1(workspace:string,id:RepositoryCachePartitionIdV1):string{return nativeImageRootV1(repositoryCachePartitionDirectoryV1(workspace,id));}

/** 🗃️ Names queued lease writers inside their separate physically finite backing. */
export function repositoryLeaseDirectoryV1(workspace:string):string{return requireMaterializedPathV1(join(repositoryCachePartitionRootV1(workspace,"leases"),"data"));}

/** 🎟️ Opens the authored finite backing under original caller custody; unsupported enforcement remains a refusal. */
export async function openRepositoryCachePartitionV1(workspace:string,id:RepositoryCachePartitionIdV1,control:ResourceLeaseControl):Promise<NativeImageStorageV1>{
 checkResourceLeaseControl(control);const budget=repositoryCacheBudgetV1(workspace),partition=budget.partitions[id];if(!partition||partition.capacityBytes%1048576||partition.capacityBytes<34603008)throw Error("Finite generated cache partition required");
 if(id==="leases")return openNativePlatformStorageV1({directory:partition.directory,leaseDirectory:budget.controlDirectory,filesystemBytes:partition.filesystemBytes,maximumPhysicalBytes:partition.capacityBytes,control,maximumLeaseDatabaseBytes:16384});
 const leases=await openRepositoryCachePartitionV1(workspace,"leases",control);try{const leaseDirectory=repositoryLeaseDirectoryV1(workspace);await leases.recheck();requireNativeStoragePathV1(leases.root,leaseDirectory);mkdirSync(leaseDirectory,{recursive:true});const image=await openNativePlatformStorageV1({directory:partition.directory,leaseDirectory,filesystemBytes:partition.filesystemBytes,maximumPhysicalBytes:partition.capacityBytes,control});return Object.freeze({...image,close:async()=>{try{await image.close();}finally{await leases.close();}}});}catch(error){await leases.close();throw error;}
}
