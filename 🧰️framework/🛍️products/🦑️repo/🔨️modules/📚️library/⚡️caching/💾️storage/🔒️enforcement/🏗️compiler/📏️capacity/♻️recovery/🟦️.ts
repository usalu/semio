import{retireInactiveCargoStoreV1}from"../../../../../🧹️pruning/🏪️budget/⚙️execution/🟦️.ts";
import{withRepositoryCacheStoreV1}from"../../../📦️budget/📥️opening/🏪️store/🟦️.ts";
import{repositoryCompilerLayoutV1}from"../../📍️layout/🟦️.ts";
import{requireCompilerWriterCustodyV1}from"../🟦️.ts";
import{lstat,mkdtemp,rm}from"node:fs/promises";
import{nativeStatfsV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{join}from"node:path";
import{requireNativeStorageReservationV1,type NativeStorageReservationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import{checkScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{readBuildMode,type BuildMode}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import{requireNativeStoragePathV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{collectGarbageSingleFlightV1}from"../../../../../🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts";
import{unitCacheCapacityBytesV1,unitCacheRootV1}from"../../../../../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
/** ♻️ Reclaims only finalized compact units under the retained real writer without queueing behind a caller's own readers. */
export async function recoverCompactCompilerHeadroomV1(workspace:string,mode:BuildMode,original:ScriptInvocation,control:ResourceLeaseControl,storage:NativeStorageReservationV1):Promise<void>{
 checkScriptInvocation(original);checkResourceLeaseControl(control);requireIssuedResourceControlInvocationV1(control,original);requireNativeStorageReservationV1(storage,control);requireCompilerWriterCustodyV1(storage.backing.root,control);mode=readBuildMode(mode);if(storage.allocation.kind!=="compiler-workspace")throw Error("Original compiler pressure owner required");await storage.recheck();const protocol=schema.definitions.Protocol.const,requiredBytes=storage.allocation.bytes,available=async()=>{await storage.recheck();const fs=await nativeStatfsV1(storage.backing.root,{bigint:true}),bytes=Number(fs.bavail*fs.bsize);if(!Number.isSafeInteger(bytes)||bytes<0)throw Error("Actual compact compiler headroom required");return bytes;};if(await available()>=requiredBytes)return;
 return withRepositoryCacheStoreV1(workspace,"diagnostics",original,control,async diagnostics=>{const artifactDirectory=await mkdtemp(join(diagnostics.root,"tmp","cr-"));let remaining=Math.min(protocol.maximumDeletionBytes,storage.backing.maximumPhysicalBytes);try{for(const selected of[mode,mode==="prototype"?"release":"prototype"] as const){const layout=repositoryCompilerLayoutV1(storage.backing.root,selected);requireNativeStoragePathV1(storage.backing.root,layout.buildDirectory);try{const status=await lstat(layout.buildDirectory);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Original compact compiler root required");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}for(let pass=0;pass<protocol.maximumMaintenancePasses&&remaining>0;pass++){const before=await available();if(before>=requiredBytes)return;const result=await retireInactiveCargoStoreV1({workspace,buildDirectory:layout.buildDirectory,relatedDirectories:[layout.targetDirectory],leaseDirectory:join(workspace,".cache/b/l/m/data"),artifactDirectory,invocation:original,control,policy:{version:1,retainedBytes:Math.min(protocol.maintenanceRetainedBytes,storage.backing.maximumPhysicalBytes),targetAvailableBytes:requiredBytes,maximumDeletionBytes:remaining},preview:false,wait:false,compilerStorage:storage,compilerMode:selected});remaining-=result.logicalBytes;await diagnostics.recheck();if(result.availableBytes<=before||!result.units)break;}}}finally{await diagnostics.recheck();await rm(artifactDirectory,{recursive:true});}});
}

/** 🧹️ Frees idle unit-cache lanes, incremental sets and objects for the bytes a refused compile still misses, through the single-flight collector (live lanes are never touched; a running collector is left alone). Never throws for a busy or missing cache. */
export async function relieveUnitCacheHeadroomV1(storage:NativeStorageReservationV1,demandBytes:number,collect:typeof collectGarbageSingleFlightV1=collectGarbageSingleFlightV1):Promise<void>{
 if(storage.allocation.kind!=="compiler-workspace"||!Number.isSafeInteger(demandBytes)||demandBytes<=0)return;
 await collect(unitCacheRootV1(storage.backing.root),unitCacheCapacityBytesV1({capacityBytes:storage.backing.maximumPhysicalBytes,maximumWorkspaceBytes:storage.allocation.bytes}),{demandBytes});
}
