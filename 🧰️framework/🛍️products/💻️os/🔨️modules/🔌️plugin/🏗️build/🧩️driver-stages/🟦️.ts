import {ensureNativeTranspileV1} from "../../🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts";
import {checkDescriptorEmitterCapabilitiesV1,descriptorEmitterReleaseReplyExistsV1,ensureFreshDescriptorEmitterV1,ensurePinnedDescriptorEmitterV1,withCompilerStoreDescriptorEmitterV1,withDescriptorEmitterTaskV1,withDescriptorEmitterV1,type DescriptorEmitterV1} from "../../🖨️describe/🛠️emitter/🟦️.ts";
import {createDescriptorEmitterReleaseJobV1,descriptorEmitterReleaseArtifactRequestV1} from "../../🖨️describe/🛠️emitter/🤝️release-handoff/🟦️.ts";
import{readNativeStorageReservationPolicyV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import type{CargoCompilerInputsPortV1}from"@semio-tech/process/artifacts/native-build/inputs";
import{DESCRIPTOR_EMITTER_MANIFEST_V1,type DescriptorEmitterPinV1}from"../../🖨️describe/🛠️emitter/📋️contract/🟦️.ts";
import { availableParallelism } from "node:os";
import { OwnedCommandFailure,runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import { retireArtifactDirectoriesV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { machinePathV1 } from "../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import { RELEASE_PROVENANCE_COMPLETION_V1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📦️partial-provenance/🟦️.ts";
import { nativeBuildIndependentJobPlanV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/📋️prerequisites/🟦️.ts";
import { withoutJobserverEnvironmentV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts";
import {dirname} from "node:path";
import {beginAcceptedPhysicalCleanupV1,checkAcceptedPhysicalCleanupV1,readAcceptedPhysicalCleanupV1,withAcceptedPhysicalCleanupV1,withIssuedResourceOperationV1,requireIssuedResourceControlInvocationV1,scriptInvocationEnvironment,scriptResourceProcessEnvironmentV1,withRequestedIssuedResourceControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { withRepositoryCargoPassInputsV1, repositoryCargoSourceMemoV1, repositoryCargoCompilerInputsV1, repositoryCargoConsumptionV1, type CargoDependencyNetworkV1, componentPassPlanV1, nativeBuildPipelinePlanV1, prepareCargoWorkspaceInvocation } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { repositoryCargoPreparationStorageV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { reclaimUnitStorageV1, relieveUnitStorageV1, repositoryCachePartitionRootDataV1, retryAfterOutOfSpaceV1, withUnitPressureRecheckV1, withRepositoryCompilerStoreV1, withRepositoryComponentLaneV1, withCompilerWorkspaceSharedWriteV1, withCompilerWorkspaceStageWriteV1, withRepositoryCacheStoreV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import {runtimeCatalogBuildInventoryV1} from "../../📇️registry/📖️catalog-view/🟦️.ts";
import { requireMaterializedPathV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { compileUnitsForHostV1, describeCompileUnitsV1, planFleetJobsV1, readCompileUnitsHostV1, requireGrantedCompileUnitsV1, withCompilerCapacityConsumptionV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { resolveLanguageV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { CATALOG_LANGUAGES_V1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { mkdirSync, rmSync } from "node:fs";
import { join,resolve } from "node:path";
import { nativeBuildGenerationKindV1, nativeBuildPassInputRequestsV1, nativeBuildSchedulerInputRequestV1, runNativeBuildDriverV1, type LinkedComponentV1, type NativeBuildOutcomeV1, type NativeBuildDriverOptionsV1, type NativeBuildStageContextV1, type NativeBuildStageV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import{nativeReleaseWorkUnitsV1,type NativeBuildGenerationKindV1,type NativeBuildGenerationV1,type NativeBuildJobV1,type NativeBuildPipelineV1,type NativeBuildPlanV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/📋️contract/🟦️.ts";
import { optimizeComponentCoresV1,publishComponentBrowserV1, transpileComponentModulesV1, type ComponentModuleWorkV1 } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
import { componentDeliverableV1, SHIPPED_COMPONENT_PROFILE, componentProfileV1, type ComponentReceiptV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { componentStageTargetV1, componentsDescribeArgumentsV1, settleComponentDescriptorV1, type ComponentDescribeModeV1 } from "../../🖨️describe/🧩️component-stage/🟦️.ts";
import type { SemanticDescriptorEmissionV1 } from "../../🖨️describe/🛂️descriptor-emission/🟦️.ts";


import {withComponentModelPublicationV1,describeComponentModelV1,publishComponentModelV1,type ComponentModelPublicationV1} from "../../📦️model/🟦️.ts";
import {captureComponentProductLeaseTaskV1} from "../../📦️model/🔒️lease-task/🟦️.ts";
import {componentStagePlanV1,type ComponentStageFacetV1} from "./📋️contract/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟨️.mjs";
import { captureCheckoutSnapshotV1, checkoutSnapshotOriginPathV1, checkoutSnapshotPathV1, forgetCheckoutSnapshotStepV1, requireCheckoutSnapshotRootV1 } from "../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🟦️.ts";
import { isConcurrentEditV1 } from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { stageFleetDecoratorV1, stageFleetEventTextV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/☁️stage-fleet/🟦️.ts";
import { describeComponentJobsV1, planComponentJobsV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts";
import { fleetIdleSlotsV1, fleetLinkedSlotsV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import { componentPassPeakV1, componentPeakProfilesV1 } from "./🪪️peak-profile/🟦️.ts";
import { offloadLinkerIdentityV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧰️toolchain/🟦️.ts";
import { componentFinalCratePeaksV1, readUnitHistoryV1, unitHistoryPathV1 } from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import { lightSlotsHeldV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🟦️.ts";

type ComponentBrowserStageOwnerV1=(work:ComponentModuleWorkV1,transpiled:string,optimized:string|null,receipt:ComponentReceiptV1)=>Promise<void>;
type ComponentDescribeStageOwnerV1=(context:NativeBuildStageContextV1,body:(emitter:DescriptorEmitterV1,control:CompilerAdmissionControl)=>Promise<void>)=>Promise<void>;

/** 🧩️ The post-link stages of one shipped plugin or extension component inside a build driver, each holding one jobserver token and cached by content:
 * `describe` (the semantic descriptor exactly these bytes answer, verified against the committed pair) and `transpile` start the moment the component is
 * staged and run beside the rest of the compilation; `optimize` (`wasm-opt -Oz` per core) follows transpilation and verification of the linked compiler source closure; `probe` runs the assembled module's own
 * `describe()`; `publish` waits for the pass's provenance, writes the receipt bound to the Merkle root of the component's unit closure and publishes the
 * module; the `discard` finalizer then removes an assembled module no publish took and the stage work directory, whatever failed or was cancelled. A component
 * without a deployment directory is described and receipted only. `describeMode` `write` replaces a differing committed pair
 * (`components describe`) instead of refusing it. A plan's `stageAfter` adds the jobs of the same driver a stage
 * consumes (the descriptor emitter before `describe`, browser support before `browser`).
 * @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
 * @see ../../🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json */
export function componentDriverStagesV1(repoRoot:string,invocation:ScriptInvocation,publisher:ComponentModelPublicationV1,facet:ComponentStageFacetV1,describeMode:ComponentDescribeModeV1,withDescribe:ComponentDescribeStageOwnerV1,withBrowser:ComponentBrowserStageOwnerV1):(component:LinkedComponentV1)=>readonly NativeBuildStageV1[]{
 return component=>{
  const target=componentStageTargetV1(repoRoot,component.package.manifest),deliverable=Object.freeze({profile:componentProfileV1(component.pass.profile),directory:dirname(component.staged),wasm:component.staged,receipt:join(dirname(component.staged),"🧾️component.json")});
  let browserPublished=false,receipt:ComponentReceiptV1|undefined,transpiled:string|undefined,optimized:string|null=null,work:ComponentModuleWorkV1|undefined;
  const workOf=(control:CompilerAdmissionControl,cache:ComponentModuleWorkV1["cache"],root:string,environment:Readonly<Record<string,string|undefined>>):ComponentModuleWorkV1=>{if(!work){const directory=requireMaterializedPathV1(join(root,"w",target.packageName));rmSync(directory,{recursive:true,force:true});mkdirSync(directory,{recursive:true});work={invocation,repoRoot,target,deliverable,cache,work:directory,control};}work={...work,control,cache,environment};return work;};
  const run:Readonly<Record<string,NativeBuildStageV1["run"]>>={
   describe:context=>withDescribe(context,async(emitter,control)=>{const emission=await describeComponentModelV1(publisher,component,{...context,control},emitter),drift=settleComponentDescriptorV1(repoRoot,target,deliverable,emission,describeMode);if(drift.length&&describeMode==="write")context.log(`[describe] ${target.pluginId}: rewrote ${drift.length} committed paths`);}),
   model:async context=>{receipt=await publishComponentModelV1(publisher,component,await context.provenance());},
   transpile:async context=>{transpiled=await transpileComponentModulesV1(workOf(context.control,context.cache,context.generation.root,context.environment));},
   optimize:async context=>{await context.provenance();optimized=await optimizeComponentCoresV1(workOf(context.control,context.cache,context.generation.root,context.environment),transpiled!,context.parallelism);},
   browser:async context=>{if(!receipt||!transpiled)throw Error("Actual model and transpile must complete");await withBrowser(workOf(context.control,context.cache,context.generation.root,context.environment),transpiled,optimized,receipt);browserPublished=true;},
   complete:async()=>{if(!receipt||facet==="browser"&&target.directoryName!==undefined&&!browserPublished)throw Error("Actual model and selected browser publication must complete");},
   discard:async()=>{if(work)rmSync(work.work,{recursive:true,force:true});}
  };
  return componentStagePlanV1({version:1,facet},target.directoryName!==undefined).map(row=>Object.freeze({name:row.name,after:row.name==="model"?[...row.after,`provenance:${component.pass.id}`,RELEASE_PROVENANCE_COMPLETION_V1]:row.name==="optimize"?[...row.after,`provenance:${component.pass.id}`]:row.after,capacity:row.name==="optimize"?"parallel":row.name==="discard"?"cleanup":row.name==="model"||row.name==="complete"?"io":"single",run:run[row.name]!}));
 };
}

/** 🛠️ Finds the exact declared descriptor emitter job that this plan's describe stage consumes. */
function planEmitterJobV1(repoRoot:string,plan:NativeBuildPlanV1):NativeBuildJobV1|undefined{
 const emitterRoot=dirname(resolve(repoRoot,DESCRIPTOR_EMITTER_MANIFEST_V1));
 return plan.jobs?.find(job=>job.cwd===emitterRoot&&job.command[0]==="repo"&&job.command[1]==="./📜️script.ts"&&job.command[2]==="build"&&plan.stageAfter?.describe?.includes(`job:${job.id}`));
}

type DescriptorEmitterReleaseJobV1=ReturnType<typeof createDescriptorEmitterReleaseJobV1>;

/** 📸️ Runs every release component build against ONE immutable source generation: unless the caller's authority already names a `sourceRoot`, the live workspace is captured into a content-addressed checkout snapshot, the Cargo passes, their configuration, lockfile, profile policy and provenance read that snapshot only, and authority, leases, caches and committed descriptor pairs stay on the live root. A concurrent edit that surfaces while the snapshot is taken or its lockfile is checked takes a new snapshot as the churn policy of the `build` layer declares. Prototype builds keep reading the live workspace.
 * @see ../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🟦️.ts */
async function runRepositoryComponentDriverV1(repoRoot: string, mode: BuildMode, plan: NativeBuildPlanV1, options: Parameters<typeof runRepositoryComponentDriverRootedV1>[4]): Promise<NativeBuildOutcomeV1> {
  if(mode!=="release")return runRepositoryComponentDriverRootedV1(repoRoot,repoRoot,mode,plan,options);
  const declared=readNativeCallerAuthorityV1(options.invocation.capabilities).sourceRoot,say=options.log??console.log;
  const step=options.environment.SEMIO_SOURCE_SNAPSHOT_STEP??process.env.SEMIO_SOURCE_SNAPSHOT_STEP,ownerText=options.environment.SEMIO_SOURCE_SNAPSHOT_OWNER??process.env.SEMIO_SOURCE_SNAPSHOT_OWNER,owner=ownerText===undefined||!/^[1-9][0-9]*$/u.test(ownerText)?undefined:Number(ownerText);
  const root=declared!==repoRoot?declared:(await captureCheckoutSnapshotV1({workspace:repoRoot,signal:options.control.signal,onEvent:text=>say("[snapshot] "+text),...step!==undefined&&owner!==undefined?{step,owner}:{}})).root;
  const proof=await requireCheckoutSnapshotRootV1(root,repoRoot);
  say(`[snapshot] source root ${root} digest ${proof.digest}`);
  try{return await runRepositoryComponentDriverRootedV1(repoRoot,root,mode,plan,options);}
  catch(error){
   if(step!==undefined&&(isConcurrentEditV1(error)||/lockfile|source snapshot/iu.test(error instanceof Error?error.message:String(error))))await forgetCheckoutSnapshotStepV1(repoRoot,step);
   throw error;
  }
}

/** 🏪️ Runs one build driver over `plan` inside the repository's charged compiler store of `mode`, the one owner every component driver run goes through:
 * the store issues the native storage reservation, the configured Source boundary, the Cargo home, target and build directories and the stage cache root,
 * and the bounded registry custody of every pass manifest is held under the caller's own network grant BEFORE the compiler writer custody is taken, so the
 * driver acquires in ascending lock order (cargo registry rank 2, then the image writer rank 7) like every other Cargo flow. The writer custody is SHARED, like an
 * ordinary Cargo command: the pass only grows its own lanes, stage cache and run directories (each under its own build, store or product lease) plus content-addressed
 * outputs, so component builds and Cargo commands run side by side and only maintenance asks for the exclusive custody.
 * @see ../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json
 * @see ../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
 * @see ../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts */
async function runRepositoryComponentDriverRootedV1(repoRoot: string, sourceRoot: string, mode: BuildMode, plan: NativeBuildPlanV1, options: Readonly<{ invocation:ScriptInvocation;facet:ComponentStageFacetV1;describeMode:ComponentDescribeModeV1;network: CargoDependencyNetworkV1; control: CompilerAdmissionControl; environment: Readonly<Record<string, string | undefined>>; generation: NativeBuildGenerationKindV1; emitterHeld?:true; emitterPin?:DescriptorEmitterPinV1; releaseEmitter?:DescriptorEmitterReleaseJobV1; independentJobs?:true; partialProvenance?:true; releaseWorkUnits?: number; consume?:NativeBuildDriverOptionsV1["consume"];maximumObservationBytes?:number;retainCompilationBytes?:number; jobEnvironment?: (job: NativeBuildJobV1, generation: NativeBuildGenerationV1) => Readonly<Record<string, string>>; log?: (line: string) => void }>): Promise<NativeBuildOutcomeV1> {
  if (!plan.passes.length) throw Error("Component build driver requires declared Cargo passes");
  const source=Object.freeze({root:sourceRoot,origin:repoRoot}),toSource=(path:string):string=>checkoutSnapshotPathV1(source,path),toOrigin=(path:string):string=>checkoutSnapshotOriginPathV1(source,path);
  const sourcePlan:NativeBuildPlanV1=sourceRoot===repoRoot?plan:Object.freeze({...plan,passes:plan.passes.map(pass=>Object.freeze({...pass,manifest:toSource(pass.manifest),packages:pass.packages.map(row=>Object.freeze({...row,manifest:toSource(row.manifest)}))}))});
  const originOf=(component:LinkedComponentV1):LinkedComponentV1=>sourceRoot===repoRoot?component:Object.freeze({...component,pass:Object.freeze({...component.pass,manifest:toOrigin(component.pass.manifest),packages:component.pass.packages.map(row=>Object.freeze({...row,manifest:toOrigin(row.manifest)}))}),package:Object.freeze({...component.package,manifest:toOrigin(component.package.manifest)})});
  const targets=plan.passes.flatMap(pass=>pass.packages.map(row=>componentStageTargetV1(repoRoot,row.manifest)));
  if(options.emitterPin){
   if(mode!=="prototype"||options.emitterHeld||options.releaseEmitter||planEmitterJobV1(repoRoot,plan))throw Error("Published descriptor emitter selection requires an independent prototype component build");
   await ensurePinnedDescriptorEmitterV1(repoRoot,options.invocation,options.control,options.emitterPin);
  }else if(!options.emitterHeld&&!options.releaseEmitter&&!planEmitterJobV1(repoRoot,plan))await ensureFreshDescriptorEmitterV1(repoRoot,options.invocation,options.control,options.log,undefined,sourceRoot);
  if(options.facet==="browser"&&targets.some(target=>target.directoryName!==undefined))await ensureNativeTranspileV1(repoRoot,options.invocation,options.control,options.environment,sourceRoot);
  const declarations=targets.map(target=>({manifest:target.manifestPath,intent:{version:1 as const,mode,pluginId:target.pluginId,cratePath:target.cratePath},packageId:target.packageId,packageName:target.packageName}));
  return withRepositoryCacheStoreV1(repoRoot,"dependencies",options.invocation,options.control,(_store,storePhase)=>{
   const environment=scriptInvocationEnvironment(options.invocation,options.environment);
   return withRepositoryCompilerStoreV1(repoRoot,mode,storePhase,environment,async(store,control)=>{
   const buildsEmitter=planEmitterJobV1(repoRoot,plan)!==undefined;
   if(!buildsEmitter&&!options.releaseEmitter)await checkDescriptorEmitterCapabilitiesV1(repoRoot,options.invocation,store,control,options.emitterPin);
   const withProductLeaseTask=captureComponentProductLeaseTaskV1();
   const jobAttempt:NativeBuildDriverOptionsV1["jobAttempt"]=(job,phase)=>{
    if(job.id!==options.releaseEmitter?.id)return;
    return options.releaseEmitter.attempt(phase,async request=>{
     if(!await descriptorEmitterReleaseReplyExistsV1(repoRoot,options.invocation,phase,request))return false;
     return withProductLeaseTask(()=>withCompilerStoreDescriptorEmitterV1(repoRoot,options.invocation,store,phase,emitter=>withDescriptorEmitterTaskV1(emitter,options.invocation,async()=>true),undefined,request));
    });
   };
   const leaseDirectory=readNativeStorageReservationPolicyV1(store.storage,store.control).leaseDirectory;
   const withDescribe:ComponentDescribeStageOwnerV1=(context,body)=>withProductLeaseTask(()=>
    withCompilerStoreDescriptorEmitterV1(repoRoot,options.invocation,store,context.control,(emitter,reader)=>
     withCompilerWorkspaceStageWriteV1(store.storage,leaseDirectory,reader,writer=>body(emitter,writer)),options.emitterPin,options.releaseEmitter?.selected()));

   const withBrowserLeaseTask:ComponentBrowserStageOwnerV1=(work,transpiled,optimized,receipt)=>withProductLeaseTask(()=>
    publishComponentBrowserV1(work,transpiled,optimized,receipt,(phase,body)=>withCompilerWorkspaceStageWriteV1(store.storage,leaseDirectory,phase,writer=>{
     requireNativeStoragePathV1(store.storage.backing.root,work.work);
     requireNativeStoragePathV1(store.storage.backing.root,transpiled);
     if(optimized!==null)requireNativeStoragePathV1(store.storage.backing.root,optimized);
     return body(writer);
    })));

   return withRepositoryComponentLaneV1({storage:store.storage,root:store.root,sharedBuild:store.buildDirectory,workspace:repoRoot,crates:declarations.map(row=>row.packageName),environment,control,policyControl:store.control},async lane=>{
   let fleetLight=0;
   const withCargoInputs:CargoCompilerInputsPortV1=(request,body)=>repositoryCargoCompilerInputsV1({workspace:repoRoot,sourceRoot,invocation:options.invocation,consumption:repositoryCargoConsumptionV1(request.arguments,resolve(request.manifest)===resolve(repoRoot,DESCRIPTOR_EMITTER_MANIFEST_V1)?"descriptor-emitter":"artifact"),network:options.network,control,storage:store.storage})(request,body);
   const peakProfiles=componentPeakProfilesV1(repoRoot,mode,plan.passes);
   const passInputs={workspace:repoRoot,sourceRoot,invocation:options.invocation,network:options.network,storage:store.storage},requestOptions={cwd:sourceRoot,build:lane.build,target:lane.target,environment:store.environment,control},requests=[...nativeBuildPassInputRequestsV1(sourcePlan,requestOptions),nativeBuildSchedulerInputRequestV1({...requestOptions,originCwd:repoRoot})];
   const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,say=(line:string)=>(options.log??console.log)(line);
   const linkedSlots=(peakMegabytes:number,signal:AbortSignal)=>{
    const linker=offloadLinkerIdentityV1(store.toolchain.root);
    return linker===undefined?Promise.resolve(null):fleetLinkedSlotsV1(store.environment,{signal,peakMegabytes},{linker,target:"wasm32-wasip2"});
   };
   return retryAfterOutOfSpaceV1({store:store.root,language,report:say},()=>withUnitPressureRecheckV1(()=>relieveUnitStorageV1(store.root,{environment:store.environment,language,report:say}),()=>withRepositoryCargoPassInputsV1({...passInputs,control},requests,inputs=>runNativeBuildDriverV1({storage:store.storage,sourceBoundary:store.sourceBoundary,sourceMemo:repositoryCargoSourceMemoV1(store),decorateStageCache:stageFleetDecoratorV1({workspaceRoot:repoRoot,environment:store.environment,report:event=>say(stageFleetEventTextV1(language,event))}),withCargoInputs,withPassInputs:(rows,body)=>withRepositoryCargoPassInputsV1({...passInputs,control:inputs},rows,body),plan:sourcePlan,...(options.partialProvenance?{partialProvenance:true as const}:{}),cwd:sourceRoot,originCwd:repoRoot,cacheRoot:join(store.root,"components"),shared:{target:lane.target,build:lane.build},generation:options.generation,leaseDirectory:leaseDirectory,environment:store.environment,control:inputs,withCapacity:(async compile=>{const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,report=options.log??console.log;await relieveUnitStorageV1(store.root,{environment:store.environment,language,report:line=>report(line)});const units=await compileUnitsForHostV1({environment:store.environment,signal:inputs.signal}),waitStart=Date.now(),operation="component build";report(describeCompileUnitsV1(language,"planned",{operation,preferred:units.decision.preferred,minimum:units.decision.minimum,cores:units.host.cores,reason:units.decision.reason}));return withCompilerWorkspaceSharedWriteV1(store.storage,leaseDirectory,inputs,writer=>withCompilerCapacityConsumptionV1({request:units.request,directory:leaseDirectory,control:writer,environment:store.environment,...(options.independentJobs?{independentJobs:true as const}:{})},async(facts,held)=>{requireGrantedCompileUnitsV1(units.decision,facts.parallelism);report(describeCompileUnitsV1(language,"granted",{operation,granted:facts.parallelism,cores:units.host.cores,seconds:Math.round((Date.now()-waitStart)/1000)}));return withComponentModelPublicationV1({workspace:repoRoot,invocation:options.invocation,storage:store.storage,control:held,toolchain:store.toolchain,declarations,withProductLeaseTask},publisher=>compile(facts,held,publisher));}));}) as NonNullable<NativeBuildDriverOptionsV1["withCapacity"]>,fleetJobs:async facts=>{const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,report=options.log??console.log,host=await readCompileUnitsHostV1({environment:store.environment,signal:facts.signal}),idle=await fleetIdleSlotsV1(store.environment,{signal:facts.signal}),plan=planFleetJobsV1({local:facts.local,fleetIdle:idle,grantedMegabytes:host.grantedMegabytes,availableMegabytes:host.availableMegabytes,pressure:host.pressure,unitMegabytes:facts.unitMegabytes});fleetLight=plan.remote;report(describeCompileUnitsV1(language,"fleet",{operation:"component build",jobs:plan.jobs,local:plan.local,remote:plan.remote,idle:idle??0,reason:plan.reason}));return plan;},componentJobs:async facts=>{const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,report=options.log??console.log,host=await readCompileUnitsHostV1({environment:store.environment,signal:facts.signal}),root=store.environment.SEMIO_UNIT_CACHE_ROOT,recorded=root===undefined?undefined:await readUnitHistoryV1(unitHistoryPathV1(join(store.root,"components"))).catch(()=>undefined);const peak=componentPassPeakV1({root,crates:facts.crates,profiles:peakProfiles,seeds:recorded===undefined?[]:componentFinalCratePeaksV1(recorded)}),slots=await linkedSlots(peak,facts.signal),plan=planComponentJobsV1({local:1,fleetSlots:slots,availableMegabytes:host.availableMegabytes,pressure:host.pressure,unitMegabytes:facts.unitMegabytes});report(describeComponentJobsV1(language,{operation:"component build",jobs:plan.jobs,local:plan.local,remote:plan.remote,slots:slots??0,peak,reason:plan.reason}));return plan;},fleetSlots:async facts=>{const root=store.environment.SEMIO_UNIT_CACHE_ROOT,peak=componentPassPeakV1({root,crates:facts.crates,profiles:peakProfiles,seeds:[]});return linkedSlots(peak,facts.signal);},fleetRunning:()=>store.environment.SEMIO_UNIT_CACHE_ROOT?lightSlotsHeldV1(store.environment.SEMIO_UNIT_CACHE_ROOT,fleetLight):0,withStageWriter:(phase,write)=>withCompilerWorkspaceStageWriteV1(store.storage,leaseDirectory,phase,write),reclaimStorage:({needBytes})=>reclaimUnitStorageV1(store.root,needBytes,{signal:inputs.signal,report:{language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,write:(line:string)=>(options.log??console.log)(line)}}).then(()=>undefined),stages:(component,publisher)=>componentDriverStagesV1(repoRoot,options.invocation,publisher as ComponentModelPublicationV1,options.facet,options.describeMode,withDescribe,withBrowserLeaseTask)(originOf(component)).map(stage=>stage.name==="describe"||stage.capacity==="cleanup"?stage:{...stage,run:context=>withIssuedResourceOperationV1(context.control,()=>withCompilerWorkspaceSharedWriteV1(store.storage,leaseDirectory,context.control,()=>stage.run(context)))}),...(options.consume?{consume:options.consume,maximumObservationBytes:options.maximumObservationBytes,retainCompilationBytes:options.retainCompilationBytes}:{}),...(options.jobEnvironment?{jobEnvironment:options.jobEnvironment}:{}),...(options.releaseEmitter?{jobAttempt}:{}),...(options.log?{log:options.log}:{})}))));
   });
   },options.releaseWorkUnits);
  });
}

/** 🚀️ Runs a product's whole release pipeline (`pipeline`, the `metadata.semio.release` of its release target) as one build driver in the compiler store of
 * the caller's build mode: one Cargo pass per component workspace restricted to `components` (every discovered component when omitted), the pipeline's
 * jobs with their owner policies, and the generation `SEMIO_NATIVE_BUILD_GENERATION` selects.
 * @see ../../../../../../../📜️script.ts
 * @see ../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts */
export async function buildReleasePipelineV1(repoRoot: string, pipeline: NativeBuildPipelineV1, options: Readonly<{ invocation:ScriptInvocation; components?: ReadonlySet<string>; facet:ComponentStageFacetV1; network: CargoDependencyNetworkV1; control: CompilerAdmissionControl }>): Promise<NativeBuildOutcomeV1> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);
  const authority=readNativeCallerAuthorityV1(options.invocation.capabilities),mode=authority.original.buildMode;if(repoRoot!==authority.repositoryRoot)throw Error("Actual component driver workspace required");
  const { plan, jobEnvironment } = nativeBuildPipelinePlanV1(repoRoot, pipeline, mode, options.components);
  const ordered=nativeBuildIndependentJobPlanV1(plan,availableParallelism());
  const emitter=mode==="release"?planEmitterJobV1(repoRoot,plan):undefined;
  const releaseEmitter=emitter===undefined?undefined:createDescriptorEmitterReleaseJobV1(options.invocation,emitter.id);
  const run=async(control:CompilerAdmissionControl)=>{
   for(const job of ordered.before){
    await withRequestedIssuedResourceControlV1(control,0,async phase=>{
     const carried=releaseEmitter?.id===job.id?releaseEmitter.attempt(phase,async request=>{
      if(!await descriptorEmitterReleaseReplyExistsV1(repoRoot,options.invocation,phase,request))return false;
      return withDescriptorEmitterV1(repoRoot,options.invocation,phase,emitter=>withDescriptorEmitterTaskV1(emitter,options.invocation,async()=>true),request);
     }):undefined;
     const environment=withoutJobserverEnvironmentV1({...job.environment,...jobEnvironment(job),...carried?.environment});
     delete environment.SEMIO_COMPILER_CAPACITY_HELD;
     delete environment.SEMIO_COMPILER_PARALLELISM;
     try{await runOwnedCommand(job.command[0]!,job.command.slice(1),job.cwd,job.id,0,{control:phase,signal:phase.signal,env:scriptResourceProcessEnvironmentV1(phase,environment),output:{maximumBytes:authority.transport.maximumBytes,maximumLines:authority.transport.maximumLines}});}
     catch(error){
      if(!carried||!(error instanceof OwnedCommandFailure)||error.outcome.status===null||await carried.settle(error.outcome.status)!==0)throw error;
      return;
     }
     await carried?.settle(0);
    });
   }
   return runRepositoryComponentDriverV1(repoRoot,mode,ordered.plan, { ...(options.facet==="browser"&&mode==="release"?{partialProvenance:true as const}:{}),...(ordered.independentJobs?{independentJobs:true as const}:{}), invocation:options.invocation,facet:options.facet,describeMode:mode==="release"?"verify":"local",network:options.network,control,environment:process.env,generation:nativeBuildGenerationKindV1(process.env),releaseWorkUnits:nativeReleaseWorkUnitsV1(plan),jobEnvironment,...(releaseEmitter?{releaseEmitter}:{}) });
  };
  if(!releaseEmitter)return run(options.control);
  return withAcceptedPhysicalCleanupV1(options.control,{version:1,kind:"artifact-scratch",resource:"descriptor-emitter-release-handoff",reserveMilliseconds:5000},async(enrollment,control)=>{
   let value:NativeBuildOutcomeV1|undefined;
   const failures:unknown[]=[];
   try{value=await run(control);}catch(error){failures.push(error);}
   try{
    await beginAcceptedPhysicalCleanupV1(enrollment);
    checkAcceptedPhysicalCleanupV1(enrollment);
    const products=repositoryCachePartitionRootDataV1(repoRoot,"products"),requests=releaseEmitter.requests();
    const retired=await retireArtifactDirectoriesV1(requests.map(request=>descriptorEmitterReleaseArtifactRequestV1(products,request).path),{leaseDirectory:machinePathV1(repoRoot,".cache/b/l/m/data"),signal:readAcceptedPhysicalCleanupV1(enrollment).cleanupSignal});
    if(retired.held.length)throw Error("Release descriptor emitter handoff still has unjoined artifact custody");
   }catch(error){if(!failures.includes(error))failures.push(error);}
   if(failures.length>1)throw new AggregateError(failures,"Native release and descriptor emitter handoff retirement failed");
   if(failures.length)throw failures[0];
   return value!;
  });
}

/** 🛠️ Retains the exact selected packages and profile of each component Cargo pass. */
export function componentOwnerPreparationArgumentsV1(pass: Readonly<{
  manifest: string;
  profile: string;
  target: string;
  packages: readonly Readonly<{ name: string }>[];
}>): readonly string[] {
  return ["build", "--manifest-path", pass.manifest, "--profile", pass.profile, "--target", pass.target,
    ...pass.packages.flatMap(pkg => ["-p", pkg.name])];
}

/** 🎯️ Builds exactly `pluginIds` — the closure one dev launch serves — through the one build driver over the warm stores: only the passes and packages of
 * those components, each streamed through its cached stages into the one plugin-modules root. A launch never pays for components it does not load.
 * @see ../../../🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts */
export async function buildComponentClosureV1(repoRoot: string, pluginIds: ReadonlySet<string>, options: Readonly<{invocation:ScriptInvocation;facet:ComponentStageFacetV1;network:CargoDependencyNetworkV1; control: CompilerAdmissionControl; prepareCargoOwners?:true; emitterHeld?:true; emitterPin?:DescriptorEmitterPinV1; generation?:NativeBuildGenerationKindV1;consume?:NativeBuildDriverOptionsV1["consume"];maximumObservationBytes?:number;retainCompilationBytes?:number;log?: (line: string) => void }>): Promise<NativeBuildOutcomeV1> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);
  const authority=readNativeCallerAuthorityV1(options.invocation.capabilities),mode=authority.original.buildMode;if(repoRoot!==authority.repositoryRoot)throw Error("Actual component driver workspace required");
  const plan = componentPassPlanV1(repoRoot, pluginIds, mode), environment = buildModeEnvironment(mode);
  if (options.prepareCargoOwners) {
    const storage = repositoryCargoPreparationStorageV1(repoRoot);
    for (const pass of plan.passes) {
      await prepareCargoWorkspaceInvocation(storage, repoRoot, componentOwnerPreparationArgumentsV1(pass), repoRoot,
        scriptInvocationEnvironment(options.invocation, environment), options.control);
    }
  }
  return runRepositoryComponentDriverV1(repoRoot,mode,plan, { invocation:options.invocation,facet:options.facet,describeMode:mode==="release"?"verify":"local",network: options.network, control: options.control, environment, generation:options.generation??"warm",...(options.emitterHeld?{emitterHeld:true as const}:{}),...(options.emitterPin?{emitterPin:options.emitterPin}:{}),...(options.consume?{consume:options.consume,maximumObservationBytes:options.maximumObservationBytes,retainCompilationBytes:options.retainCompilationBytes}:{}), ...(options.log ? { log: options.log } : {}) });
}

/** 🧭️ The components one playground variant loads: `boot` is the host closure a launch must serve before anything else (its app scope only), `full` is
 * every component the variant's session lists. */
export function playgroundComponentClosureV1(variant: string, scope: "boot" | "full"): ReadonlySet<string> {
  const row = runtimeCatalogBuildInventoryV1().playgrounds.find((target) => target.variant === variant);
  if (!row) throw new Error(`Unknown playground variant: ${variant}`);
  return new Set(runtimeComponentClosure([...runtimeCatalogBuildInventoryV1().plugins, ...runtimeCatalogBuildInventoryV1().extensions], [{ id: row.pluginId, appScoped: scope === "boot" || row.app !== undefined }]) as readonly string[]);
}

/** ✍️ `components describe <plugin-id…>|--all [--profile release|prototype]`: writes the committed semantic pairs from freshly built bytes. Shipped
 * bytes come from one build driver run over exactly the selected components, each described (writing a differing pair), receipted and materialized as it
 * links; iterate bytes are built and described one component at a time. */
export async function describeComponentsV1(repoRoot: string, invocation:ScriptInvocation, segments: readonly string[], control: CompilerAdmissionControl,network:CargoDependencyNetworkV1): Promise<void> {
  requireIssuedResourceControlInvocationV1(control,invocation);const authority=readNativeCallerAuthorityV1(invocation.capabilities),mode=authority.original.buildMode;
  const {all,pluginIds,profile,facet}=componentsDescribeArgumentsV1(segments);if(repoRoot!==authority.repositoryRoot||profile!==componentProfileV1(buildModePolicy(mode).cargoProfile))throw Error("Describe selection must match its actual Original workspace and mode");
  const plan=componentPassPlanV1(repoRoot,all?undefined:new Set(pluginIds),mode);
  const outcome=await runRepositoryComponentDriverV1(repoRoot,mode,plan,{invocation,facet,describeMode:"write",network,control,environment:buildModeEnvironment(mode),generation:"warm"});
  console.log(`components describe: ${outcome.staged.length} component(s) described from ${profile} bytes; report ${outcome.report}`);
}
