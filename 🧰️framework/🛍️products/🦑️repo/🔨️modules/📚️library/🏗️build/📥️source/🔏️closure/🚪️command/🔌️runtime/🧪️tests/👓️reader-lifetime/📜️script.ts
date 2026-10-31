import {join} from "node:path";
import {once} from "node:events";
import {requireCapturedBuildCommandV1,type CapturedBuildCommandProofV1} from "../../../🟦️.ts";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {openNativePlatformStorageV1,openPreparedNativePlatformStorageV1,retireNativePlatformStorageV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {acquirePreparedNativeStorageOwnerV1,recoverNativeStorageReservationV1,observeNativeStorageStateV1,nativeStorageRetirementPortV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {withRepositoryRuntimeEntriesV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

/** 👓️ Exercises real image and paid charge ownership; the existing preparation-owned path is the causal RED. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{
 requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);
 const [area,phase,id]=args,row=fixture.cases.find(row=>row.id===id);if(!area||!row||!['exercise','cleanup'].includes(phase!))throw Error("Declared Native reader lifecycle case required");
 await receiveScriptProcessInvocation(process.env,async original=>{
  const control=createIssuedResourceControlV1(original,"test:native-reader-lifetime",60000),options={directory:join(area,"i"),leaseDirectory:join(area,"l"),filesystemBytes:fixture.filesystemBytes,maximumPhysicalBytes:fixture.maximumPhysicalBytes,control};
  await withRepositoryRuntimeEntriesV1(process.cwd(),original,control,async()=>{
   if(phase==="cleanup"){
    const image=await openNativePlatformStorageV1(options),port=nativeStorageRetirementPortV1(image.port,async()=>{}),policy={version:1 as const,root:image.root,leaseDirectory:options.leaseDirectory,capacityBytes:fixture.maximumPhysicalBytes,maximumWorkspaceBytes:fixture.chargeBytes,maximumArtifactBytes:fixture.chargeBytes};let used=0;
    try{const state=await observeNativeStorageStateV1({policy,control,port});for(const allocation of state.allocations){if(allocation.owner!=="test:native-reader"||allocation.bytes!==fixture.chargeBytes||allocation.kind!=="compiler-workspace")throw Error("Exact owned lifecycle charge required");await(await recoverNativeStorageReservationV1({policy,control,port,id:allocation.id})).retire();}used=(await observeNativeStorageStateV1({policy,control,port})).allocations.reduce((sum,row)=>sum+row.bytes,0);}finally{await image.close();}
    const retired=await retireNativePlatformStorageV1(options,{version:1,wait:true});console.log(JSON.stringify({phase,id,used,retired}));return;
   }
   const preparedAt=performance.now(),consumerStop=new AbortController(),preparationStop=new AbortController(),consumer=createRequestedIssuedResourceControlV1(control,0,consumerStop.signal),preparation=createRequestedIssuedResourceControlV1(consumer,typeof row.preparationMilliseconds==="number"?row.preparationMilliseconds:fixture.preparationMilliseconds,preparationStop.signal),image=await openPreparedNativePlatformStorageV1({...options,control:consumer},preparation),port=nativeStorageRetirementPortV1(image.port,async()=>{}),policy={version:1 as const,root:image.root,leaseDirectory:options.leaseDirectory,capacityBytes:fixture.maximumPhysicalBytes,maximumWorkspaceBytes:fixture.chargeBytes,maximumArtifactBytes:fixture.chargeBytes};
   try{
    const charge=await acquirePreparedNativeStorageOwnerV1({policy,control:consumer,port,owner:"test:native-reader",kind:"compiler-workspace",bytes:fixture.chargeBytes},preparation);const committed=(await observeNativeStorageStateV1({policy,control:preparation,port})).allocations.find(row=>row.id===charge.allocation.id);if(committed?.phase!=="committed")throw Error("Actual committed reader charge required");console.log(JSON.stringify({phase:"prepared",id,policy,allocation:committed}));
    if(row.preparationCancelled)preparationStop.abort(Error("Owned preparation stopped"));if(row.consumerCancelled)consumerStop.abort(Error("Owned consumer stopped"));
    if("preparationExpired" in row&&row.preparationExpired){if(!preparation.signal.aborted)await once(preparation.signal,"abort",{signal:consumer.signal});console.log(JSON.stringify({phase:"expired",id,aborted:preparation.signal.aborted,manualAbort:preparationStop.signal.aborted,elapsedMilliseconds:performance.now()-preparedAt,message:preparation.signal.reason instanceof Error?preparation.signal.reason.message:String(preparation.signal.reason)}));}
    try{await image.port.withOperation(consumer,()=>charge.recheck());console.log(JSON.stringify({phase:"result",id,usable:true,bytes:charge.allocation.bytes}));}catch(error){console.log(JSON.stringify({phase:"result",id,usable:false,bytes:charge.allocation.bytes,message:error instanceof Error?error.message:String(error)}));throw error;}
   }finally{await image.close();}
  });
 });
}
