import {measureNativeDirectoryUsageAsyncV1,nativeDirectoryEntryBoundV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📏️usage/🟦️.ts";
import {requireCapturedBuildCommandV1,type CapturedBuildCommandProofV1} from "../../../../🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {withRepositoryRuntimeEntriesV1} from "../../../🟦️.ts";
import {openRepositoryCachePartitionV1} from "../../../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts";
import {withRepositoryPartitionOperationV1,repositoryLeaseDirectoryV1} from "../../../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {repositoryCacheBudgetV1} from "../../../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {observeExistingNativeStorageStateV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/👁️existing/🟦️.ts";
import {nativeStorageUsedBytesV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
/** ⏱️ Measures only existing public tools backing and journal observations under the captured receiving authority. */
export async function runCapturedBuildSourceV1(_args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{
 requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);
 await receiveScriptProcessInvocation(process.env,original=>{const control=scriptCompilerAdmissionControlV1(original,"test:tools-read-observation",120000),workspace=process.cwd();return withRepositoryRuntimeEntriesV1(workspace,original,control,async()=>{
  const image=await openRepositoryCachePartitionV1(workspace,"tools",control);let failed=false,failure:unknown;
  try{return await withRepositoryPartitionOperationV1(image,control,async(canonical,actual)=>{
   const declaration=repositoryCacheBudgetV1(workspace).partitions.tools,policy={version:1 as const,root:canonical.root,leaseDirectory:repositoryLeaseDirectoryV1(workspace),capacityBytes:declaration.capacityBytes,maximumWorkspaceBytes:declaration.filesystemBytes,maximumArtifactBytes:declaration.maximumArtifactBytes},timings:{operation:string;startedAtMilliseconds:number;endedAtMilliseconds:number;wallMilliseconds:number}[]=[];
   const timed=async<T>(operation:string,body:()=>Promise<T>)=>{const start=performance.now();try{return await body();}finally{const end=performance.now();timings.push({operation,startedAtMilliseconds:start,endedAtMilliseconds:end,wallMilliseconds:end-start});}};
   const backing=await timed("observe",()=>canonical.port.observe(policy,actual));if(!backing||!canonical.port.measure)throw Error("Actual existing tools observation and measurement interface required");
   await timed("recheck",()=>canonical.port.recheck(backing,actual));
   await timed("measure-first",()=>canonical.port.measure!(backing,{mode:"recheck",requestBytes:0},actual));
   await timed("measure-second",()=>canonical.port.measure!(backing,{mode:"recheck",requestBytes:0},actual));
   const usage=await timed("usage-row",()=>canonical.port.withOperation(actual,phase=>measureNativeDirectoryUsageAsyncV1(canonical.root,{maximumEntries:nativeDirectoryEntryBoundV1(canonical.maximumPhysicalBytes),signal:phase.signal})));
   const snapshot=await timed("journal-sh",()=>observeExistingNativeStorageStateV1({image:canonical,policy,control:actual}));
   console.log(JSON.stringify({version:1,mode:"tools-read-observation",timings,usage,backing:{maximumPhysicalBytes:canonical.maximumPhysicalBytes,effectiveCapacityBytes:snapshot.effectiveCapacityBytes},journal:snapshot.journal,sequence:snapshot.state.sequence,usedBytes:nativeStorageUsedBytesV1(snapshot.state),allocationBytes:snapshot.state.allocations.map(row=>row.bytes),measureRequest:{mode:"recheck",requestBytes:0}}));
  });}catch(error){failed=true;failure=error;throw error;}finally{try{await image.close();}catch(error){if(failed&&!Object.is(failure,error))throw new AggregateError([failure,error],"Tools read observation and image close failed");throw error;}}
 });});
}
