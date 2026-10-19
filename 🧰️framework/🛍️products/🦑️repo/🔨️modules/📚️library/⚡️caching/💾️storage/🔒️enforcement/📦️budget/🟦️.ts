import {join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {requireMaterializedPathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nativeImageUsableBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🟦️.ts";
import {repositoryStrictStorageV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};

/** 📦️ Identifies the complete closed set of managed generated-storage owners. */
export type RepositoryCachePartitionIdV1="rust"|"tools"|"graph"|"dependencies"|"products"|"diagnostics"|"nx"|"leases";
/** 🗜️ Declares physical funding, the payload bytes left after filesystem overhead, and independent logical publication observation limits. */
export interface RepositoryCachePartitionV1 {readonly directory:string;readonly capacityBytes:number;readonly filesystemBytes:number;readonly maximumArtifactBytes:number;readonly maximumLogicalBytes:number;readonly usableBytes:number}
/** 📏️ Accounts all fixed backing envelopes plus their finite bootstrap-control allowance. */
export interface RepositoryCacheBudgetV1 {readonly version:1;readonly maximumBytes:number;readonly maximumControlBytes:number;readonly partitionBytes:number;readonly headroomBytes:number;readonly partitions:Readonly<Record<RepositoryCachePartitionIdV1,RepositoryCachePartitionV1>>}
/** 📋️ Validates the entire partition ledger before any physical provisioning or cache write. */
export function readRepositoryCacheBudgetV1(value:unknown):RepositoryCacheBudgetV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed repository cache partition declaration required");
 const declaration=value as {version:1;maximumBytes:number;maximumControlBytes:number;partitions:Record<RepositoryCachePartitionIdV1,{directory:string;capacityBytes:number;maximumArtifactBytes?:number;maximumLogicalBytes:number}>},partitions={} as Record<RepositoryCachePartitionIdV1,RepositoryCachePartitionV1>,overhead=schema.$defs.filesystemOverhead.const,paths:string[]=[];let partitionBytes=0;
 for(const[id,partition]of Object.entries(declaration.partitions)){
  const directory=partition.directory,path=directory.toLowerCase();
  if(directory.startsWith("/")||/[\\:\u0000-\u001f]/u.test(directory)||directory.split("/").some(part=>!part||part==="."||part==="..")||partition.capacityBytes%1048576!==0)throw Error("Aligned physical cache partition and relative directory required");
  if(paths.some(previous=>previous===path||previous.startsWith(path+"/")||path.startsWith(previous+"/")))throw Error("Cache partition directories must be disjoint");
  if(path===".cache/b/c"||path.startsWith(".cache/b/c/")||".cache/b/c".startsWith(path+"/"))throw Error("Cache partition overlaps finite bootstrap control storage");
  paths.push(path);partitionBytes+=partition.capacityBytes;if(partition.maximumArtifactBytes!==undefined&&partition.maximumArtifactBytes>partition.capacityBytes-1048576)throw Error("Artifact charge must fit its partition filesystem");const usableBytes=nativeImageUsableBytesV1(partition.capacityBytes-1048576);if(partition.capacityBytes<=overhead.tinyCapacityBytes&&partition.maximumLogicalBytes>usableBytes)throw Error("Tiny cache partition logical limit must fit its filesystem after overhead");partitions[id as RepositoryCachePartitionIdV1]=Object.freeze({...partition,usableBytes,filesystemBytes:partition.capacityBytes-1048576,maximumArtifactBytes:partition.maximumArtifactBytes??partition.capacityBytes-1048576});
 }
 const headroomBytes=declaration.maximumBytes-partitionBytes-declaration.maximumControlBytes;if(headroomBytes<0)throw Error("Cache partition reservations exceed repository maximum");
 return Object.freeze({version:1,maximumBytes:declaration.maximumBytes,maximumControlBytes:declaration.maximumControlBytes,partitionBytes,headroomBytes,partitions:Object.freeze(partitions)});
}
/** 🚦️ Declares the one bounded queued whole-image maintenance wait that ordinary commands use instead of refusing concurrent invocations; callers start it before taking their own shared pin on that partition because later readers queue behind it. */
export function repositoryCacheOrdinaryMaintenanceV1():Readonly<{version:1;mode:"queued";maximumWaitMilliseconds:number}>{const row=schema.$defs.ordinaryMaintenance.const;return Object.freeze({version:1 as const,mode:"queued" as const,maximumWaitMilliseconds:row.maximumWaitMilliseconds});}
/** 🧭️ Resolves one finite repository budget without substituting an ordinary filesystem for admission. */
export function repositoryCacheBudgetV1(workspace:string):RepositoryCacheBudgetV1&{readonly controlDirectory:string}{
 const root=requireMaterializedPathV1(resolve(workspace)),budget=readRepositoryCacheBudgetV1(policy),partitions=Object.fromEntries(Object.entries(budget.partitions).map(([id,partition])=>[id,Object.freeze({...partition,directory:requireMaterializedPathV1(join(root,partition.directory))})])) as Record<RepositoryCachePartitionIdV1,RepositoryCachePartitionV1>,rust=repositoryStrictStorageV1(root);
 if(partitions.rust.directory!==rust.directory||partitions.rust.capacityBytes!==rust.capacityBytes||partitions.rust.filesystemBytes!==rust.filesystemBytes)throw Error("Rust cache envelope disagrees with repository total budget");
 return Object.freeze({...budget,partitions:Object.freeze(partitions),controlDirectory:requireMaterializedPathV1(join(root,".cache/b/c"))});
}
