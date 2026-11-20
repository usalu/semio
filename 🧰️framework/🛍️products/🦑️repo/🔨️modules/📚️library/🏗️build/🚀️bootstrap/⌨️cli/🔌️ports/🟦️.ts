import type {NativeRepoRecoveryPlanV1,NativeRepoRecoveryPortsV1} from "../🟦️.ts";

/** 🔌️ Composes the fixed owners of one native recovery plan; this composition root is the only place that imports the layers which run above the recovery frontend. */
export async function nativeRepoRecoveryPortsV1(plan:NativeRepoRecoveryPlanV1):Promise<NativeRepoRecoveryPortsV1> {
 if(plan.kind==="cache-image")return{image:async(context,target)=>{
  const owner=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts");
  if(target.operation==="reclaim")await owner.runRepositoryImageReclaimV1(context,target.profile);
  else if(target.operation==="reclaim-wait")await owner.runRepositoryRustImageReclaimWaitV1(context);
  else await owner.runRepositoryElasticRegrowV1(context,target.profile,true);
 }};
 if(plan.kind==="cache-reset")return{reset:async(context,profile)=>(await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📜️script.ts")).runInactiveRepositoryCacheResetV1(context,profile)};
 if(plan.kind==="maintenance")return{maintenance:async(context,operation)=>(await import("../../../../⚡️caching/🧹️pruning/🏪️budget/📜️script.ts")).runInactiveCompilerMaintenanceV1(context,operation)};
 const builder=await import("../../../../../💻️client/🔌️mcp/🟦️.ts");
 return{build:builder.runBuildScript,install:async context=>(await import("../../../../../💻️client/🔌️mcp/📥️installation/🟦️.ts")).installNativeRepoV1(context),child:async(workspace,request,parent,control)=>(await import("../../../📥️caller/📤️child/🟦️.ts")).runOwnedBuildChildInvocationV1(workspace,request,parent,control)};
}
