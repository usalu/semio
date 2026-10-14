import {openRepositoryCachePartitionV1} from "../📦️budget/📥️opening/🟦️.ts";
import {randomBytes} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,writeFileSync,openSync,fsyncSync,closeSync,renameSync,rmSync} from "node:fs";
import {join} from "node:path";
import {acquireQueuedResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {acquireNativeStorageV1,recoverNativeStorageReservationV1,nativeStorageStateV1,nativeStorageEventV1,type NativeStoragePolicyV1,type NativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1,type NativeImageStorageV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readBuildMode,type BuildMode} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import type {CargoSourceBoundaryPolicyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import {repositoryStrictStorageV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏪️ Retains one charged shared compiler working set rather than minting a new allocation per invocation. */
export interface RepositoryCompilerStoreV1 {readonly root:string;readonly buildDirectory:string;readonly captureDirectory:string;readonly storage:NativeStorageReservationV1;readonly sourceBoundary:CargoSourceBoundaryPolicyV1;readonly environment:Readonly<Record<string,string|undefined>>}
/** 🔄️ Recovers the exact still-charged store before another original compiler enters it. */
export async function repositoryCompilerReservationV1(image:NativeImageStorageV1,policy:NativeStoragePolicyV1,mode:BuildMode,control:ResourceLeaseControl):Promise<NativeStorageReservationV1>{
 checkResourceLeaseControl(control);mode=readBuildMode(mode);await image.recheck();const owner="repo:compiler:"+mode,receipt=requireNativeStoragePathV1(image.root,join(image.root,"compiler-"+mode+".json")),lease=await acquireQueuedResourceLease({directory:policy.leaseDirectory,resource:"repo:compiler-charge:"+image.root+":"+mode,mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:control.signal,control});
 try{
  let id:string|undefined;if(existsSync(receipt)){const status=lstatSync(receipt);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>2048)throw Error("Original compiler charge receipt required");const row=JSON.parse(readFileSync(receipt,"utf8"));if(validateJsonSchemaSubset(schema,row).length||row.mode!==mode||row.owner!==owner||row.bytes!==policy.maximumWorkspaceBytes)throw Error("Compiler charge receipt disagrees with authored policy");id=row.allocation;}
  else{const journal=join(image.root,"storage-events.jsonl");if(existsSync(journal)){const bytes=readFileSync(journal);if(bytes.length>1048576||!bytes.toString("utf8").endsWith("\n"))throw Error("Original compiler journal recovery required");let state=nativeStorageStateV1();for(const line of bytes.toString("utf8").split("\n").filter(Boolean))state=nativeStorageEventV1(state,JSON.parse(line),policy.capacityBytes);const candidates=state.allocations.filter(row=>row.owner===owner);if(candidates.length>1)throw Error("Original compiler allocation identity is ambiguous");id=candidates[0]?.id;}}
  const storage=id?await recoverNativeStorageReservationV1({policy,control,port:image.port,id}):await acquireNativeStorageV1({policy,control,port:image.port,owner,kind:"compiler-workspace",bytes:policy.maximumWorkspaceBytes});
  if(storage.allocation.owner!==owner||storage.allocation.bytes!==policy.maximumWorkspaceBytes||storage.allocation.kind!=="compiler-workspace"||storage.allocation.phase==="retiring")throw Error("Original shared compiler allocation refused");
  await storage.recheck();if(!existsSync(receipt)){const temporary=requireNativeStoragePathV1(image.root,receipt+"~"+randomBytes(8).toString("hex"));try{writeFileSync(temporary,JSON.stringify({version:1,mode,allocation:storage.allocation.id,owner,bytes:storage.allocation.bytes})+"\n",{flag:"wx"});const fd=openSync(temporary,"r+");try{fsyncSync(fd);}finally{closeSync(fd);}await storage.recheck();renameSync(temporary,receipt);}finally{rmSync(temporary,{force:true});}}if(storage.allocation.phase==="reserved")await storage.commit();return storage;
 }finally{lease.release();}
}

/** 🗜️ Holds the actual persistent backing and original store charge through every compiler output publication. */
export async function withRepositoryCompilerStoreV1<T>(workspace:string,mode:BuildMode,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>,body:(store:RepositoryCompilerStoreV1)=>Promise<T>):Promise<T>{
 mode=readBuildMode(mode);const declaration=repositoryStrictStorageV1(workspace),image=await openRepositoryCachePartitionV1(workspace,"rust",control),policy={version:1 as const,root:image.root,leaseDirectory:declaration.leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.maximumWorkspaceBytes,maximumArtifactBytes:declaration.maximumArtifactBytes};
 try{const storage=await repositoryCompilerReservationV1(image,policy,mode,control),base=join(image.root,"cargo",mode),buildDirectory=join(base,"build"),captureDirectory=join(base,"captures"),temporary=join(base,"tmp"),home=join(image.root,"cargo/home");for(const path of[buildDirectory,captureDirectory,temporary,home]){requireNativeStoragePathV1(image.root,path);await storage.recheck();mkdirSync(path,{recursive:true});}
  return await body(Object.freeze({root:image.root,buildDirectory,captureDirectory,storage,sourceBoundary:declaration.sourceBoundary,environment:Object.freeze({...environment,CARGO_HOME:home,CARGO_TARGET_DIR:join(base,"target"),CARGO_BUILD_BUILD_DIR:buildDirectory,TMPDIR:temporary,TMP:temporary,TEMP:temporary,SEMIO_TEST_ARTIFACT_DIR:captureDirectory})}));
 }finally{await image.close();}
}
