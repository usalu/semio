import {nativeImageVolumeUsedBytesSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/⚖️availability/🟦️.ts";
import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {statSync,statfsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {effectiveCapacityV1,elasticBloatedV1,elasticCapacityV1,elasticGrowthV1,pinnedCapacityV1,writeElasticGrowthV1,type ElasticPartitionIdV1,type ElasticRuleV1} from "./📐️elastic/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {requireMaterializedPathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nativeImageUsableBytesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🟦️.ts";
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
export {effectiveCapacityV1,elasticBloatedV1,elasticCapacityV1,elasticGrowthV1,pinnedCapacityV1,writeElasticGrowthV1,type ElasticPartitionIdV1,type ElasticRuleV1} from "./📐️elastic/🟦️.ts";
/** 🌱️ Whether an elastic image should be recreated now: its rule is automatic, the mounted image has less free space than the pressure floor and the elastic size the host allows exceeds its capacity. Bloat is never a reason: it is reclaimed in place. */
export function repositoryPartitionRegrowDueV1(workspace:string,partition:ElasticPartitionIdV1):boolean{
 const rule=policy.elastic?.[partition];if(!rule?.automatic)return false;
 try{const declared=policy.partitions[partition],directory=machinePathV1(requireMaterializedPathV1(resolve(workspace)),declared.directory),mount=join(directory,"m"),image=statfsSync(mount);if(statSync(mount).dev===statSync(directory).dev)return false;const hostNow=statfsSync(dirname(directory));if(image.bavail*image.bsize>=rule.pressureFreeBytes)return false;const host=hostNow;const capacity=effectiveCapacityV1(directory,rule.minimumBytes,rule);return elasticGrowthV1(rule,capacity,host.bavail*host.bsize,statSync(join(directory,"fixed.dmg")).blocks*512)>capacity;}catch{return false;}
}
/** ♻️ The bloat rule of a partition: its elastic rule when it has one, else the policy's common `reclaim` rule. */
export function repositoryPartitionReclaimRuleV1(partition:RepositoryCachePartitionIdV1):Readonly<{reclaimFloorBytes:number;reclaimRatioPercent:number;reclaimTightHostFreeBytes:number}>{
 return (policy.elastic as Record<string,ElasticRuleV1>|undefined)?.[partition]??policy.reclaim;
}
/** 🧲️ Whether the mounted image of a partition holds host blocks it no longer uses (host allocation of its file against the bytes used inside by the partition's bloat rule); an image that is not mounted cannot say what it uses and is not due. */
export function repositoryPartitionReclaimDueV1(workspace:string,partition:RepositoryCachePartitionIdV1):boolean{
 try{
  const declared=policy.partitions[partition],directory=machinePathV1(requireMaterializedPathV1(resolve(workspace)),declared.directory),mount=join(directory,"m");
  if(statSync(mount).dev===statSync(directory).dev)return false;
  const used=nativeImageVolumeUsedBytesSyncV1(mount,statfsSync(mount)),host=statfsSync(dirname(directory));
  return used!==undefined&&elasticBloatedV1(repositoryPartitionReclaimRuleV1(partition),statSync(join(directory,"fixed.dmg")).blocks*512,used,host.bavail*host.bsize);
 }catch{return false;}
}
/** 📝️ Records the size the next image of a pressed elastic partition is born with (the image that is about to be recreated has less free space than the pressure floor), so the recreation grows instead of repeating the same size; answers the recorded bytes, or 0 when the image is not pressed or cannot grow. */
export function repositoryPartitionGrowthRecordV1(workspace:string,partition:ElasticPartitionIdV1):number{
 const rule=policy.elastic?.[partition];if(!rule?.automatic)return 0;
 try{const declared=policy.partitions[partition],directory=machinePathV1(requireMaterializedPathV1(resolve(workspace)),declared.directory),image=statfsSync(join(directory,"m"));if(image.bavail*image.bsize>=rule.pressureFreeBytes)return 0;const host=statfsSync(dirname(directory)),capacity=effectiveCapacityV1(directory,rule.minimumBytes,rule),grown=elasticGrowthV1(rule,capacity,host.bavail*host.bsize,statSync(join(directory,"fixed.dmg")).blocks*512);if(grown<=capacity)return 0;writeElasticGrowthV1(directory,grown);return grown;}catch{return 0;}
}
/** 🧭️ Resolves one finite repository budget without substituting an ordinary filesystem for admission; every partition keeps the capacity its image was born with (a changed policy target applies only to the next image born by the explicit reset), and a new elastic one is sized from need and host; a receipt whose capacity the budget schema refuses on its own (too small, corrupt or foreign) is ignored for that partition instead of failing the budget of every command. */
export function repositoryCacheBudgetV1(workspace:string):RepositoryCacheBudgetV1&{readonly controlDirectory:string}{
 const root=requireMaterializedPathV1(resolve(workspace));let maximumBytes=policy.maximumBytes;const partitions:Record<string,unknown>={...policy.partitions};
 for(const[id,declared]of Object.entries(policy.partitions) as [keyof typeof policy.partitions,{directory:string;capacityBytes:number;maximumLogicalBytes:number}][]){
  const rule=(policy.elastic as Record<string,ElasticRuleV1>|undefined)?.[id],capacity=rule?effectiveCapacityV1(machinePathV1(root,declared.directory),rule.minimumBytes,rule):pinnedCapacityV1(machinePathV1(root,declared.directory),declared.capacityBytes);
  if(capacity!==declared.capacityBytes){
   const logical=declared.maximumLogicalBytes+capacity-declared.capacityBytes,candidate={...declared,capacityBytes:capacity,maximumLogicalBytes:logical};
   try{readRepositoryCacheBudgetV1({...policy,maximumBytes:policy.maximumBytes+capacity-declared.capacityBytes,partitions:{...policy.partitions,[id]:candidate}});maximumBytes+=capacity-declared.capacityBytes;partitions[id]=candidate;}catch{}
  }
 }
 const budget=readRepositoryCacheBudgetV1(maximumBytes===policy.maximumBytes?policy:{...policy,maximumBytes,partitions}),resolved=Object.fromEntries(Object.entries(budget.partitions).map(([id,partition])=>[id,Object.freeze({...partition,directory:requireMaterializedPathV1(machinePathV1(root,partition.directory))})])) as Record<RepositoryCachePartitionIdV1,RepositoryCachePartitionV1>,rust=repositoryStrictStorageV1(root);
 if(resolved.rust.directory!==rust.directory||resolved.rust.capacityBytes!==rust.capacityBytes||resolved.rust.filesystemBytes!==rust.filesystemBytes)throw Error("Rust cache envelope disagrees with repository total budget");
 return Object.freeze({...budget,partitions:Object.freeze(resolved),controlDirectory:requireMaterializedPathV1(machinePathV1(root,".cache/b/c"))});
}
