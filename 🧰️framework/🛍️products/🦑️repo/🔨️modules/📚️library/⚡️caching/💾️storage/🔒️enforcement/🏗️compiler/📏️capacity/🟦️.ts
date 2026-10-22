import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withRetainedOperationsV1,withRetainedOperationV1,requireRetainedOperationLifetimeV1,requireRetainedOperationFrameV1,type RetainedOperationLifetimeV1}from"../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🪢️lifetime/🟦️.ts";
import{withNativeImageTerminalCloseV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts";
import{AsyncLocalStorage}from"node:async_hooks";
import{nativeStatfsV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{resolve}from"node:path";
import{randomBytes}from"node:crypto";
import{acquireQueuedResourceLease,acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type{NativeImageStorageV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{setTimeout as sleepV1}from"node:timers/promises";
import{resolveLanguageV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import{CATALOG_LANGUAGES_V1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ Physical initial headroom is independent from persistent logical compiler charges. */
export function compilerHeadroomAdmittedV1(value:unknown):boolean{if(validateJsonSchemaSubset(schema.definitions.Observation,value,schema).length)throw Error("Closed physical compiler headroom required");const row=value as{availableBytes:number;requiredBytes:number};return row.availableBytes>=row.requiredBytes;}
type WriterScope={root:string;control:ResourceLeaseControl;lifetime:RetainedOperationLifetimeV1;live:boolean;recover?:()=>Promise<void>};
const writers=new AsyncLocalStorage<WriterScope>(),owners=new WeakMap<NativeStorageReservationV1,WriterScope>();
function writerOperation<T>(scope:WriterScope,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{if(!scope.live)throw Error("Original compiler writer scope retired");return withRetainedOperationV1(scope.lifetime,control,body);}
function workspaceOwner(storage:NativeStorageReservationV1,control:ResourceLeaseControl):WriterScope{requireIssuedResourceControlV1(control);requireNativeStorageReservationV1(storage,control);const owner=owners.get(storage);if(!owner?.live||storage.allocation.kind!=="compiler-workspace")throw Error("Original compiler workspace writer lifetime retired");requireNativeStorageReservationV1(storage,owner.control);requireIssuedResourceControlRefinementV1(owner.control,control);return owner;}
type HeadroomAdmissionV1<T>=Readonly<{admitted:true;value:T}>|Readonly<{admitted:false;availableBytes:number}>;
/** ⏳️ Queues a short image with progress and cancellation instead of failing while other compiles or maintenance still free it; only an impossible or long-expired requirement refuses. */
async function admitCompilerHeadroomV1<T>(image:Pick<NativeImageStorageV1,"maximumPhysicalBytes">,control:ResourceLeaseControl,requiredBytes:number,request:Readonly<{wait:boolean}>,attempt:()=>Promise<HeadroomAdmissionV1<T>>):Promise<T>{
 const pace=schema.definitions.Wait.const,language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,start=Date.now(),gib=(bytes:number)=>(bytes/1073741824).toFixed(1),format=(key:"waiting"|"refused",availableBytes:number,seconds:number)=>pace.messages[language][key].replace("{seconds}",String(seconds)).replace("{available}",gib(availableBytes)).replace("{required}",gib(requiredBytes));
 let interval=pace.minimumMilliseconds,reported=-pace.reportMilliseconds;
 for(;;){
  const outcome=await attempt();if(outcome.admitted)return outcome.value;
  const elapsed=Date.now()-start;
  if(!request.wait||requiredBytes>image.maximumPhysicalBytes||elapsed>=pace.maximumMilliseconds)throw Error(format("refused",outcome.availableBytes,Math.round(elapsed/1000)));
  const remaining=checkResourceLeaseControl(control);await control.publish({resource:schema.definitions.Protocol.const.resource+":headroom",mode:"shared",elapsedMs:elapsed});
  if(elapsed-reported>=pace.reportMilliseconds){reported=elapsed;process.stderr.write(format("waiting",outcome.availableBytes,Math.round(elapsed/1000))+"\n");}
  await sleepV1(Math.min(interval,remaining??interval),undefined,{signal:control.signal}).catch(error=>{control.signal.throwIfAborted();throw error;});interval=Math.min(interval*2,pace.maximumIntervalMilliseconds);
 }
}
/** 🔒️ Serializes cold compiler growth and maintenance while artifact readers retain their separate custody. */
export async function withCompilerWriterCustodyV1<T>(image:Pick<NativeImageStorageV1,"root"|"maximumPhysicalBytes"|"recheck">,leaseDirectory:string,control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean;mode?:"exclusive"|"shared"}>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlV1(control);if(validateJsonSchemaSubset(schema.definitions.WriterRequest,request,schema).length)throw Error("Closed compiler writer request required");checkResourceLeaseControl(control);const root=resolve(image.root),current=writers.getStore();if(current?.root===root)return writerOperation(current,control,body);await image.recheck();const options={directory:leaseDirectory,resource:schema.definitions.Protocol.const.resource+":"+root,mode:(request.mode??"exclusive") as "exclusive"|"shared",...(request.mode==="shared"?{sharedCohort:true}:{}),owner:randomBytes(16).toString("base64url"),control,signal:control.signal},lease=request.wait?await acquireQueuedResourceLease(options):await acquireResourceLease({...options,timeoutMs:0});
 let scope:WriterScope|undefined;return withNativeImageTerminalCloseV1(async()=>{await image.recheck();const result=await withRetainedOperationsV1(control,schema.definitions.Protocol.const.maximumOperations,(lifetime,actual)=>{scope={root,control,lifetime,live:true};return writers.run(scope,()=>body(actual));});checkResourceLeaseControl(control);await image.recheck();return result;},async()=>{if(scope)scope.live=false;lease.release();});
}
/** 🏪️ Retains writer exclusion through the actual callback and refuses before compiler-side writes. */
export function withCompilerInitialHeadroomV1<T>(image:Pick<NativeImageStorageV1,"root"|"maximumPhysicalBytes"|"recheck">,leaseDirectory:string,control:ResourceLeaseControl,requiredBytes:number,request:Readonly<{version:1;wait:boolean}>,body:(control:ResourceLeaseControl)=>Promise<T>,recover?:()=>Promise<void>):Promise<T>{
 return admitCompilerHeadroomV1(image,control,requiredBytes,request,()=>withCompilerWriterCustodyV1(image,leaseDirectory,control,request,async(actual):Promise<HeadroomAdmissionV1<T>>=>{let fs=await nativeStatfsV1(image.root,{bigint:true}),availableBytes=Number(fs.bavail*fs.bsize);checkResourceLeaseControl(actual);if(!compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes})&&recover){await recover();await image.recheck();fs=await nativeStatfsV1(image.root,{bigint:true});availableBytes=Number(fs.bavail*fs.bsize);}checkResourceLeaseControl(actual);if(!compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes}))return{admitted:false,availableBytes};return{admitted:true,value:await body(actual)};}));
}

import{requireNativeStorageReservationV1,type NativeStorageReservationV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
/** 🧷️ Retains the authenticated Native allocation through every accepted workspace operation. */
export async function withCompilerWorkspaceLifetimeV1<T>(storage:NativeStorageReservationV1,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>,recover?:()=>Promise<void>):Promise<T>{
 requireIssuedResourceControlV1(control);requireNativeStorageReservationV1(storage,control);if(typeof body!=="function")throw Error("Actual compiler workspace callback required");if(storage.allocation.kind!=="compiler-workspace"||owners.get(storage)?.live)throw Error("Original compiler workspace lifetime required");let scope:WriterScope|undefined;return withNativeImageTerminalCloseV1(()=>withRetainedOperationsV1(control,schema.definitions.Protocol.const.maximumOperations,async(lifetime,actual)=>{scope={root:resolve(storage.backing.root),control,lifetime,live:true,recover};owners.set(storage,scope);await storage.recheck();return body(actual);}),async()=>{try{await storage.recheck();}finally{if(scope)scope.live=false;}});
}
/** 🪡️ Retains only the actual compiler allocation while independent context producers own their writers. */
export function withCompilerWorkspaceBorrowV1<T>(storage:NativeStorageReservationV1,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const owner=workspaceOwner(storage,control);return withRetainedOperationV1(owner.lifetime,control,async actual=>{await storage.recheck();checkResourceLeaseControl(actual);return body(actual);},async()=>{await storage.recheck();});}
/** 🎟️ Enrolls an existing writer before waiting, then checks real headroom for the exact retained allocation. */
export function withCompilerWorkspaceWriteV1<T>(storage:NativeStorageReservationV1,leaseDirectory:string,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const owner=workspaceOwner(storage,control),current=writers.getStore(),write=(actual:ResourceLeaseControl)=>withCompilerWorkspaceBorrowV1(storage,actual,phase=>withCompilerInitialHeadroomV1({root:storage.backing.root,maximumPhysicalBytes:storage.backing.maximumPhysicalBytes,recheck:storage.recheck},leaseDirectory,phase,storage.allocation.bytes,{version:1,wait:true},body,owner.recover));return current?.root===owner.root?writerOperation(current,control,write):write(control);}

/** 🏪️ Admits free headroom under a SHARED writer custody, so concurrent compiles never wait for each other, and escalates to an exclusive custody only to recover missing headroom; the body then runs shared. Cargo compiles use this; maintenance keeps the exclusive custody of `withCompilerInitialHeadroomV1`. */
export async function withCompilerSharedHeadroomV1<T>(image:Pick<NativeImageStorageV1,"root"|"maximumPhysicalBytes"|"recheck">,leaseDirectory:string,control:ResourceLeaseControl,requiredBytes:number,request:Readonly<{version:1;wait:boolean}>,body:(control:ResourceLeaseControl)=>Promise<T>,recover?:()=>Promise<void>,measure?:()=>Promise<number>):Promise<T>{
 const available=measure??(async():Promise<number>=>{const fs=await nativeStatfsV1(image.root,{bigint:true});return Number(fs.bavail*fs.bsize);});
 await admitCompilerHeadroomV1(image,control,requiredBytes,request,async():Promise<HeadroomAdmissionV1<void>>=>{
  let availableBytes=0;const enough=await withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"shared"},async actual=>{checkResourceLeaseControl(actual);availableBytes=await available();return compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes});});
  if(enough)return{admitted:true,value:undefined};
  const recovered=await withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"exclusive"},async actual=>{checkResourceLeaseControl(actual);availableBytes=await available();if(!compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes})&&recover){await recover();await image.recheck();availableBytes=await available();}checkResourceLeaseControl(actual);return compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes});});
  return recovered?{admitted:true,value:undefined}:{admitted:false,availableBytes};
 });
 return withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"shared"},body);
}
/** 🎟️ Like `withCompilerWorkspaceWriteV1` for compilers that only grow private lanes and the content-addressed cache: concurrent compiles of one image hold the writer custody together. */
export function withCompilerWorkspaceSharedWriteV1<T>(storage:NativeStorageReservationV1,leaseDirectory:string,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const owner=workspaceOwner(storage,control),current=writers.getStore(),write=(actual:ResourceLeaseControl)=>withCompilerWorkspaceBorrowV1(storage,actual,phase=>withCompilerSharedHeadroomV1({root:storage.backing.root,maximumPhysicalBytes:storage.backing.maximumPhysicalBytes,recheck:storage.recheck},leaseDirectory,phase,storage.allocation.bytes,{version:1,wait:true},body,owner.recover));return current?.root===owner.root?writerOperation(current,control,write):write(control);}

/** 🔏️ Requires the actual live shared compiler writer before automatic physical maintenance can start. */
export function requireCompilerWriterCustodyV1(root:string,control:ResourceLeaseControl):void{checkResourceLeaseControl(control);const current=writers.getStore();if(!current?.live||current.root!==resolve(root))throw Error("Original live compiler writer required");requireRetainedOperationLifetimeV1(current.lifetime,control);}
/** 🪪️ Authenticates the exact retained workspace owner separately from a genuine narrower writer phase. */
export function requireCompilerWorkspaceCustodyV1(storage:NativeStorageReservationV1,base:ResourceLeaseControl,phase:ResourceLeaseControl):void{const owner=workspaceOwner(storage,phase);if(owner.control!==base)throw Error("Exact live compiler workspace owner required");requireRetainedOperationLifetimeV1(owner.lifetime,phase);requireCompilerWriterCustodyV1(storage.backing.root,phase);}
/** 🧬️ Requires a real accepted borrow operation in the privately authenticated Native workspace lifetime. */
export function requireCompilerWorkspaceBorrowCustodyV1(storage:NativeStorageReservationV1,base:ResourceLeaseControl,phase:ResourceLeaseControl):void{const owner=workspaceOwner(storage,phase);if(owner.control!==base)throw Error("Exact live compiler workspace owner required");requireRetainedOperationFrameV1(owner.lifetime,phase);}
/** 🧶️ Retains an already held authentic writer and workspace without repeating cold headroom admission. */
export function withCompilerWorkspaceWriterOperationV1<T>(storage:NativeStorageReservationV1,base:ResourceLeaseControl,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const owner=workspaceOwner(storage,control);if(owner.control!==base)throw Error("Exact live compiler workspace owner required");requireCompilerWriterCustodyV1(owner.root,control);const writer=writers.getStore()!;return writerOperation(writer,control,actual=>withCompilerWorkspaceBorrowV1(storage,actual,phase=>{requireCompilerWorkspaceCustodyV1(storage,base,phase);return body(phase);}));}
