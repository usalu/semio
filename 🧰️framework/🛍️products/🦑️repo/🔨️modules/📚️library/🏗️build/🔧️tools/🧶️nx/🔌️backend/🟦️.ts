import {join} from "node:path";
import {profilePhaseV1} from "../../../⏱️profile/🟨️.mjs";
import {readBuildMode} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {BackendContextV1,BackendExecutionContextV1,BuildBackendV1} from "../../../🔌️backends/🟦️.ts";

/** 🔌️ nx behind the neutral interface: its setup, reset, storage, cache tools and execution, moved here from the neutral CLI so that neutral code never names it. */
export const backend:BuildBackendV1=Object.freeze({
 id:"nx",
 async graphExternalInputs(){return(await import("../../../../🟨️.mjs")).nxBuildEngineExternalInputsV1();},
 async setup(context:BackendContextV1){const {root,invocation,phase:phaseControl}=context;
  const adapter=await import("../../../../⚡️caching/🚀️bootstrap/📜️script.ts");await adapter.runOwnedNxInvocationV1(root,["--version"],invocation,phaseControl);
 },
 async reset(context:BackendContextV1){const {root,invocation,phase:phaseControl}=context;
  const{invalidateBuildGraphV1}=await import("../../../🕸️graph/🟦️.ts");await invalidateBuildGraphV1(root,invocation,phaseControl);const {runOwnedNxInvocationV1}=await import("../../../../⚡️caching/🚀️bootstrap/📜️script.ts");await runOwnedNxInvocationV1(root,["reset"],invocation,phaseControl);
 },
 async prepareStorage(context:BackendContextV1,snapshotBytes:number){const {root,invocation,phase:phaseControl}=context;
  const {prepareNxGraphCacheStorageV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts");await prepareNxGraphCacheStorageV1(root,invocation,phaseControl,snapshotBytes);
  await (await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchNxToolingCollectorIfPressedV1(root);
 },
 async cache(){return (await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts")).nxBuildCacheToolV1();},
 async cacheProbe(context:Readonly<{root:string;invocation:import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation}>){const {root,invocation}=context;await (await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts")).prepareNxToolingStorageV1(root,invocation,createIssuedResourceControlV1(invocation,"cache-verify:nx",0));return (await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts")).nxCacheProbeV1(root);},
 async remoteCache(){return (await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts")).nxRemoteCacheToolV1();},
 async execute(context:BackendExecutionContextV1){const {root,invocation,graph}=context,resolved={args:[...context.args],watch:context.watch},artifactPhase=context.phase,command=context.args[0]!;
    if(command==="exec"&&resolved.args.slice(0,resolved.args.indexOf("--")<0?undefined:resolved.args.indexOf("--")).some(argument=>argument==="--dry-run"||argument==="--dry-run=true"||argument==="--graph"||argument.startsWith("--graph="))){
      const selection=await import("../../../🧭️selection/🟦️.ts"),{buildExecPlanV1}=await import("../../../⚙️operations/🟦️.ts"),listed=(names:string[])=>selection.buildOptionsV1(resolved.args,names).flatMap(value=>value.split(",").filter(Boolean));
      console.log(JSON.stringify(buildExecPlanV1(selection.selectBuildProjectsV1(graph,listed(["projects","p"]),listed(["exclude"]))),null,2));
      return;
    }
    const {runOwnedNxInvocationV1,publishBootstrapSources}=await import("../../../../⚡️caching/🚀️bootstrap/📜️script.ts"),{nxOperationArgumentsV1}=await import("../../../../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts");
    publishBootstrapSources(root);
    const {storeReceiptTasksV1,withStoreReceiptsV1}=await import("../../../🧾️store-receipt/🟦️.ts"),{storeRepairNxArgumentsV1}=await import("../../../🧾️store-receipt/🟨️.mjs"),boundary=resolved.args.indexOf("--"),forwarded=boundary<0?[]:resolved.args.slice(boundary+1);
    const nx=async(nxArguments:string[])=>{const prior=process.exitCode;process.exitCode=undefined;try{await runOwnedNxInvocationV1(root,nxArguments,invocation,artifactPhase);if(process.exitCode)throw new Error(`Build invocation failed (${process.exitCode})`);}finally{process.exitCode=prior;}};
    await withStoreReceiptsV1({root,tasks:await profilePhaseV1("receipts.tasks",()=>storeReceiptTasksV1(root,graph,resolved.args,invocation,readBuildMode(process.env.SEMIO_BUILD_MODE))),run:()=>profilePhaseV1("tool.nx",async()=>{await context.release();return nx(nxOperationArgumentsV1(resolved.args)!);}),repair:task=>nx(storeRepairNxArgumentsV1(task.id,task.id===resolved.args[1]?forwarded:[])),restore:task=>restoreStoreReceiptFromFleetV1(root,task)});
    return;
 }
});

/** ☁️ Brings the store content of a task's receipt back from the fleet store before the task is re-executed: the receipt came out of the Nx cache (often from another machine) and its payload lives in the fleet's content-addressed store. Silent without a remote cache; reports in both languages when it restored or could not. */
async function restoreStoreReceiptFromFleetV1(root:string,task:{readonly id:string;readonly receipt:string}):Promise<void>{
 const {nxRemoteCacheEnvironmentV1}=await import("../../../🌐️remote-cache/🧶️nx/🟦️.ts"),{fleetContentFromEnvironmentV1,fleetContentTextV1,restoreStoreReceiptContentV1}=await import("../../../🧾️store-receipt/☁️fleet-content/🟦️.ts"),{parseStoreReceiptV1}=await import("../../../🧾️store-receipt/🟨️.mjs"),{readFile}=await import("node:fs/promises");
 const environment={...process.env,...await nxRemoteCacheEnvironmentV1(process.env,{warn:()=>{}})},store=fleetContentFromEnvironmentV1(environment);
 if(store===undefined)return;
 const result=await restoreStoreReceiptContentV1({workspaceRoot:root,receipt:parseStoreReceiptV1(await readFile(task.receipt,"utf8")),store}),texts=(key:"restored"|"unavailable")=>(["en","de"] as const).map(language=>fleetContentTextV1(language,key,{files:result.files,bytes:result.bytes,task:task.id,reason:result.reason??""})).join("\n");
 if(result.state==="restored"||result.state==="partial")console.log("[store-receipt] "+texts("restored").split("\n").join("\n[store-receipt] "));
 if(result.state==="partial"||result.state==="unavailable")console.log("[store-receipt] "+texts("unavailable").split("\n").join("\n[store-receipt] "));
}
