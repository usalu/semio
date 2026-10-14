import {AsyncLocalStorage} from "node:async_hooks";
import {randomBytes} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readFileSync,writeFileSync,openSync,closeSync,fsyncSync,renameSync,rmSync} from "node:fs";
import {join} from "node:path";
import {openRepositoryCachePartitionV1} from "../🟦️.ts";
import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../../🟦️.ts";
import {acquireNativeStorageV1,recoverNativeStorageReservationV1,observeNativeStorageStateV1,type NativeStorageReservationV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1,type NativeImageStorageV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {acquireQueuedResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🛠️ Holds one genuinely issued cache charge and the original image throughout loaded child lifetime. */
export interface RepositoryCacheStoreV1{readonly root:string;readonly storage:NativeStorageReservationV1;readonly control:ResourceLeaseControl;readonly invocation:ScriptInvocation;readonly image:NativeImageStorageV1;readonly recheck:()=>Promise<void>;readonly environment:Readonly<Record<string,string|undefined>>}

const originalCacheStores=new AsyncLocalStorage<ReadonlyMap<string,{live:boolean;store:RepositoryCacheStoreV1}>>();

/** 🎟️ Recovers the original shared partition charge under unchanged enforcing image custody. */
export async function repositoryCacheStoreReservationV1(workspace:string,id:RepositoryCachePartitionIdV1,image:NativeImageStorageV1,control:ResourceLeaseControl):Promise<NativeStorageReservationV1>{
 checkResourceLeaseControl(control);await image.recheck();const declaration=repositoryCacheBudgetV1(workspace).partitions[id],label=id==="tools"?"tooling":id,owner="repository:"+label+"-store",leaseDirectory=join(workspace,".cache/b/l/m/data"),policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.filesystemBytes,maximumArtifactBytes:declaration.filesystemBytes},receipt=requireNativeStoragePathV1(image.root,join(image.root,label+".json"));
 const lease=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"repository-cache-charge:"+image.root,mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:control.signal,control});let storage:NativeStorageReservationV1;
  try{let id:string|undefined;if(existsSync(receipt)){const file=lstatSync(receipt);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>2048)throw Error("Original cache charge receipt required");const row=JSON.parse(readFileSync(receipt,"utf8"));if(validateJsonSchemaSubset(schema,row).length||row.owner!==owner||row.bytes!==policy.maximumArtifactBytes)throw Error("Original cache charge disagrees with global policy");id=row.allocation;}else{const state=await observeNativeStorageStateV1({policy,control,port:image.port}),candidates=state.allocations.filter(row=>row.owner===owner);if(candidates.length>1)throw Error("Original cache allocation is ambiguous");id=candidates[0]?.id;}
   storage=id?await recoverNativeStorageReservationV1({policy,control,port:image.port,id}):await acquireNativeStorageV1({policy,control,port:image.port,owner,kind:"artifact-publication",bytes:policy.maximumArtifactBytes});if(storage.allocation.owner!==owner||storage.allocation.kind!=="artifact-publication"||storage.allocation.bytes!==policy.maximumArtifactBytes||storage.allocation.phase==="retiring")throw Error("Original cache allocation refused");await storage.recheck();
   if(!existsSync(receipt)){const temporary=requireNativeStoragePathV1(image.root,receipt+"~"+randomBytes(8).toString("hex"));try{writeFileSync(temporary,JSON.stringify({version:1,allocation:storage.allocation.id,owner,bytes:storage.allocation.bytes})+"\n",{flag:"wx"});const fd=openSync(temporary,"r+");try{fsyncSync(fd);}finally{closeSync(fd);}await storage.recheck();renameSync(temporary,receipt);const directory=openSync(image.root,"r");try{fsyncSync(directory);}finally{closeSync(directory);}}finally{rmSync(temporary,{force:true});}}if(storage.allocation.phase==="reserved")await storage.commit();
  }finally{lease.release();}
 return storage;
}

/** 🏪️ Opens and recovers a single conservative shared cache allocation rather than accumulating recipe workspaces. */
export async function withRepositoryCacheStoreV1<T>(workspace:string,id:RepositoryCachePartitionIdV1,invocation:ScriptInvocation,body:(store:RepositoryCacheStoreV1)=>Promise<T>):Promise<T>{
 const key=repositoryCacheBudgetV1(workspace).partitions[id].directory,held=originalCacheStores.getStore()?.get(key);if(held&&held.store.invocation===invocation){if(validateJsonSchemaSubset(schema.definitions.CallbackScope,{version:1,live:held.live}).length)throw Error("Original cache callback scope released");await held.store.recheck();const value=await body(held.store);await held.store.recheck();return value;}
 const control=scriptCompilerAdmissionControlV1(invocation,"cache:"+id+"-store",0),image=await openRepositoryCachePartitionV1(workspace,id,control);
 try{const storage=await repositoryCacheStoreReservationV1(workspace,id,image,control);
  const temporary=requireNativeStoragePathV1(image.root,join(image.root,"tmp"));await storage.recheck();mkdirSync(temporary,{recursive:true});checkResourceLeaseControl(control);const store=Object.freeze({root:image.root,storage,control,invocation,image,recheck:()=>storage.recheck(),environment:Object.freeze({...process.env,...(id==="diagnostics"?{SEMIO_DIAGNOSTIC_DIRECTORY:temporary}:{}),TMPDIR:temporary,TMP:temporary,TEMP:temporary})}),active=new Map(originalCacheStores.getStore());const scope={live:true,store};active.set(key,scope);try{return await originalCacheStores.run(active,()=>body(store));}finally{scope.live=false;}
 }finally{await image.close();}
}
