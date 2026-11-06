import type{RepositoryCommandContextV1}from"../../📥️context/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type{ScriptInvocation}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{loadBuildInvocationGraphV1,loadBuildGraphInChildV1,buildGraphPlanInChildV1,writeBuildGraphPlanV1,BUILD_GRAPH_PLAN_OUT_V1}from"../../../🧭️invocation/🕸️graph/🟦️.ts";
import{profileMarkV1,profilePhaseV1}from"../../../⏱️profile/🟨️.mjs";
import{markBuildCapacityPhaseV1}from"../../../🌡️host-capacity/🚏️marks/🟦️.ts";
/** 🧹️ Maintains the task stores under the current issued invocation. */
export async function maintainTaskStoresV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
    await profilePhaseV1("graph.regrow",async()=>{if(!await(await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts")).regrowElasticImageIfQuiescentV1(invocation,phaseControl,"graph"))await(await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchGraphCollectorIfPressedV1(root);});
    await profilePhaseV1("rust.regrow",async()=>{await(await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts")).regrowElasticImageIfQuiescentV1(invocation,phaseControl,"rust");});
    await profilePhaseV1("products.regrow",async()=>{await(await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts")).regrowElasticImageIfQuiescentV1(invocation,phaseControl,"products");});
    await profilePhaseV1("products.collect",async()=>{await(await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchProductsCollectorIfPressedV1(root);});
    await profilePhaseV1("budget.sweep",async()=>{await(await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchBudgetSweepIfDueV1(root);});
}
/** 🕸️ Executes the selected backend against its authenticated graph and original control. */
export async function runTaskGraphV1(root:string,args:readonly string[],parent:ScriptInvocation,phaseControl:ResourceLeaseControl,context:RepositoryCommandContextV1):Promise<void>{const{selected,resolved,invocation,previous}=context;const command=resolved.args[0];
    if(process.env.SEMIO_BUILD_GRAPH_MEMO_DERIVE!==undefined){await profilePhaseV1("graph.memo-derive",async()=>(await import("../../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts")).answerGraphMemoDerivationV1(root,process.env,phaseControl.signal));return;}
    const {prepareBuildToolStorageV1}=await profilePhaseV1("tools.storage-load",()=>import("../../../🔧️tools/💾️storage/🟦️.ts"));await profilePhaseV1("tools.storage",()=>prepareBuildToolStorageV1(root,selected.tool,invocation,phaseControl,0));
    const loadHere=async()=>loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH==="true",undefined,selected.graphPolicy,(await import("../../../🔌️backends/🟦️.ts")).buildBackendCompactGraphV1(selected.tool),await(await(await import("../../../🔌️backends/🟦️.ts")).loadBuildBackendV1(selected.tool)).graphExternalInputs?.()??[]),planFile=process.env[BUILD_GRAPH_PLAN_OUT_V1];
    if(planFile!==undefined){await profilePhaseV1("graph.plan",async()=>writeBuildGraphPlanV1(planFile,await loadHere(),phaseControl.signal));return;}
    await markBuildCapacityPhaseV1("plan");
    const baseline=await profilePhaseV1("graph.load",async()=>buildGraphPlanInChildV1(process.env,process.env.SEMIO_BUILD_REUSE_GRAPH==="true")?loadBuildGraphInChildV1(root,args,invocation,phaseControl):loadHere()),{withSelectedRustBuildGraphV1}=await profilePhaseV1("graph.rust-load",()=>import("../../../🕸️graph/🦀️rust/🟦️.ts"));await withSelectedRustBuildGraphV1(root,baseline,resolved.args,invocation,phaseControl,async(prepared,rustPhase)=>{profileMarkV1("graph.rust-ready");const graph=prepared.graph;for(const[key,value]of Object.entries(prepared.environment)){previous[key]=process.env[key];process.env[key]=value;}
    const {withRepositoryBuildGraphArtifactV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts"),{releaseBuildGraphV1:releaseGraph,sealedGenerationPinV1}=await import("../../../🕸️graph/🟦️.ts");
    profileMarkV1("graph.pinning");await withRepositoryBuildGraphArtifactV1(root,graph,invocation,rustPhase,async(pinned,artifactPhase)=>{profileMarkV1("graph.pinned");
    process.env.SEMIO_BUILD_GRAPH_PATH=pinned.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pinned.digest;process.env.SEMIO_BUILD_REUSE_GRAPH="true";
    const generationPin=command==="watch"?undefined:sealedGenerationPinV1(selected.tool);if(generationPin!==undefined){previous.SEMIO_BUILD_GENERATION_PIN=process.env.SEMIO_BUILD_GENERATION_PIN;process.env.SEMIO_BUILD_GENERATION_PIN=generationPin;}
    if(command==="watch"){const selection=await import("../../../🧭️selection/🟦️.ts"),{readOwnedBuildInvocationV1}=await import("../../../📥️caller/🟦️.ts"),{runOwnedBuildChildInvocationV1}=await import("../../../📥️caller/📤️child/🟦️.ts");await selection.watchBuildInvocationV1(root,graph,resolved.args,invocation,artifactPhase,{child:{readOwnedBuildInvocationV1,runOwnedBuildChildInvocationV1}});return;}
    await markBuildCapacityPhaseV1("execution");
    await(await(await import("../../../🔌️backends/🟦️.ts")).loadBuildBackendV1(selected.tool)).execute({root,invocation,phase:artifactPhase,args:resolved.args,tool:selected.tool,graph,release:()=>releaseGraph(graph,baseline),...(resolved.watch===undefined?{}:{watch:resolved.watch})});
    },additionalBytes=>prepareBuildToolStorageV1(root,selected.tool,invocation,rustPhase,additionalBytes));
    });}
/** 📦️ Admits the selected task dependency environment under its original invocation. */
export async function ensureTaskDependenciesV1(root:string,phaseControl:ResourceLeaseControl,context:RepositoryCommandContextV1):Promise<void>{const{selected,resolved,invocation}=context;const command=resolved.args[0]!;
    const {buildOperationRequiresV1}=await import("../../../⚙️operations/🟦️.ts");
    if(buildOperationRequiresV1(command)==="dependencies"&&process.env[BUILD_GRAPH_PLAN_OUT_V1]===undefined){const {ensureBuildEnvironmentV1,buildEnvironmentPlanV1}=await profilePhaseV1("dependencies.ensure-load",()=>import("../../../🚀️bootstrap/🟦️.ts")),environmentPlan=buildEnvironmentPlanV1(root,selected.tool,process.env);if(!environmentPlan.closure){const {requireJavascriptLocksInSyncV1}=await profilePhaseV1("dependencies.locks-load",()=>import("../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/🟦️.ts"));profilePhaseV1("dependencies.locks",()=>requireJavascriptLocksInSyncV1(root,undefined,process.env));}const published=await profilePhaseV1("dependencies.ensure",()=>ensureBuildEnvironmentV1(root,invocation,phaseControl,environmentPlan.install,undefined,environmentPlan.scope));if(published)throw(await import("../../../📥️source/🔏️closure/🔁️dependencies/🟦️.ts")).dependencyReadmissionV1();}
}
