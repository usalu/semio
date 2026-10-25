import { physicalSourceOriginV1, type PhysicalSourceModuleMetaV1, captureBoundedSourceRevisionV1, implementationPinnedV1 } from "../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { fileURLToPath } from "node:url";
import type importedSchema from "../🧬️schema/🔣️.json";
import { type RepositoryCachePartitionIdV1, repositoryCacheBudgetV1 } from "../../../🟦️.ts";
import { type NativeImageStorageV1, requireNativeStoragePathV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { type ResourceLeaseControl, checkResourceLeaseControl, acquireQueuedResourceLease } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { type NativeStorageReservationV1, observeNativeStorageStateV1, recoverNativeStorageReservationV1, acquireNativeStorageV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import { join } from "node:path";
import { persistentStateArtifactBudgetV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/💾️state/📋️layout/🟦️.ts";
import { randomBytes } from "node:crypto";
import { existsSync, lstatSync, readFileSync, writeFileSync, openSync, fsyncSync, closeSync, renameSync, rmSync } from "node:fs";
import { staleImplementationV1 } from "../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const reservationSourceURLV1=new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),reservationSourceRevisionV1=captureBoundedSourceRevisionV1(fileURLToPath(reservationSourceURLV1),8388608).sha256;
const storeSchemaURLV1=new URL("../🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),storeSchemaCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(storeSchemaURLV1),65536,true),storeSchemaRevisionV1=storeSchemaCaptureV1.sha256;
/** 📐️ The closed Store schema exactly as pinned by the revisions above; the Store and the charge reservation share this one capture. */
export const schema:typeof importedSchema=JSON.parse(storeSchemaCaptureV1.text??"");
/** 🧷️ Refuses a stale loaded charge reservation or Store schema before retained enrollment or physical opening. */
export function requireCurrentRepositoryCacheStoreSchemaV1():void{if(implementationPinnedV1())return;if(captureBoundedSourceRevisionV1(fileURLToPath(reservationSourceURLV1),8388608).sha256!==reservationSourceRevisionV1||captureBoundedSourceRevisionV1(fileURLToPath(storeSchemaURLV1),65536).sha256!==storeSchemaRevisionV1)throw staleImplementationV1("Stale loaded repository Store implementation; restart required");}
/** 🎟️ Recovers the original shared partition charge under unchanged enforcing image custody. */
export async function repositoryCacheStoreReservationV1(workspace:string,id:RepositoryCachePartitionIdV1,image:NativeImageStorageV1,control:ResourceLeaseControl):Promise<NativeStorageReservationV1>{
 checkResourceLeaseControl(control);await image.recheck();const declaration=repositoryCacheBudgetV1(workspace).partitions[id],label=id==="tools"?"tooling":id,owner="repository:"+label+"-store",leaseDirectory=join(workspace,".cache/b/l/m/data"),policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.filesystemBytes,maximumArtifactBytes:id==="products"?persistentStateArtifactBudgetV1(declaration.filesystemBytes):declaration.maximumArtifactBytes},receipt=requireNativeStoragePathV1(image.root,join(image.root,label+".json"));
 const lease=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"repository-cache-charge:"+image.root,mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:control.signal,control});let storage:NativeStorageReservationV1;
  try{let id:string|undefined;if(existsSync(receipt)){const file=lstatSync(receipt);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>2048)throw Error("Original cache charge receipt required");const row=JSON.parse(readFileSync(receipt,"utf8"));if(validateJsonSchemaSubset(schema,row).length||row.owner!==owner||row.bytes!==policy.maximumArtifactBytes)throw Error("Original cache charge disagrees with global policy");id=row.allocation;}else{const state=await observeNativeStorageStateV1({policy,control,port:image.port}),candidates=state.allocations.filter(row=>row.owner===owner);if(candidates.length>1)throw Error("Original cache allocation is ambiguous");id=candidates[0]?.id;}
   storage=id?await recoverNativeStorageReservationV1({policy,control,port:image.port,id}):await acquireNativeStorageV1({policy,control,port:image.port,owner,kind:"artifact-publication",bytes:policy.maximumArtifactBytes});if(storage.allocation.owner!==owner||storage.allocation.kind!=="artifact-publication"||storage.allocation.bytes!==policy.maximumArtifactBytes||storage.allocation.phase==="retiring")throw Error("Original cache allocation refused");await storage.recheck();
   if(!existsSync(receipt)){const temporary=requireNativeStoragePathV1(image.root,receipt+"~"+randomBytes(8).toString("hex"));try{writeFileSync(temporary,JSON.stringify({version:1,allocation:storage.allocation.id,owner,bytes:storage.allocation.bytes})+"\n",{flag:"wx"});const fd=openSync(temporary,"r+");try{fsyncSync(fd);}finally{closeSync(fd);}await storage.recheck();renameSync(temporary,receipt);const directory=openSync(image.root,"r");try{fsyncSync(directory);}finally{closeSync(directory);}}finally{rmSync(temporary,{force:true});}}if(storage.allocation.phase==="reserved")await storage.commit();
  }finally{lease.release();}
 return storage;
}
