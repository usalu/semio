import {acquireResourceLease,checkResourceLeaseControl,ResourceLeaseTimeoutErrorV1,type ResourceLease,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

export const repositoryDependencyBindingsBusyV1 = Symbol("dependency bindings busy");

/** 🔒️ Tries the Source binding gate once while the protected dependency image is exclusive. */
export async function tryRepositoryDependencyBindingsV1(directory:string,resource:string,control:ResourceLeaseControl):Promise<ResourceLease|undefined>{
 try{return await acquireResourceLease({directory,resource,mode:"exclusive",control,signal:control.signal,timeoutMs:0});}
 catch(error){
  checkResourceLeaseControl(control);
  if(error instanceof ResourceLeaseTimeoutErrorV1&&error.resource===resource)return undefined;
  throw error;
 }
}

/** 🧵️ Projects contention only after the original image and infrastructure closures settle. */
export function settleRepositoryDependencyBindingsResultV1(result:unknown):unknown{
 if(result&&typeof result==="object"&&"acquired" in result&&result.acquired===true&&"value" in result&&result.value===repositoryDependencyBindingsBusyV1)return Object.freeze({acquired:false as const});
 return result;
}
