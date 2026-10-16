import type{CargoCompilerInputsPortV1}from"../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import{DESCRIPTOR_EMITTER_MANIFEST_V1}from"../../🖨️describe/🛠️emitter/📋️contract/🟦️.ts";
import {dirname} from "node:path";
import {withIssuedResourceOperationV1,requireIssuedResourceControlInvocationV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {type CargoDependencyNetworkV1,repositoryCargoCompilerInputsV1,repositoryCargoConsumptionV1} from "./../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {withRepositoryCompilerStoreV1} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {runtimeCatalogBuildInventoryV1} from "../../📇️registry/📖️catalog-view/🟦️.ts";
import { requireMaterializedPathV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type { CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { mkdirSync, rmSync } from "node:fs";
import { availableParallelism } from "node:os";
import { join,resolve } from "node:path";
import { nativeBuildGenerationKindV1, runNativeBuildDriverV1, type LinkedComponentV1, type NativeBuildGenerationKindV1, type NativeBuildGenerationV1, type NativeBuildJobV1, type NativeBuildOutcomeV1, type NativeBuildDriverOptionsV1, type NativeBuildPipelineV1, type NativeBuildPlanV1, type NativeBuildStageV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { componentPassPlanV1, nativeBuildPipelinePlanV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { optimizeComponentCoresV1,publishComponentBrowserV1, transpileComponentModulesV1, type ComponentModuleWorkV1 } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
import { componentDeliverableV1, SHIPPED_COMPONENT_PROFILE, componentProfileV1, type ComponentReceiptV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { componentStageTargetV1, componentsDescribeArgumentsV1, settleComponentDescriptorV1, type ComponentDescribeModeV1 } from "../../🖨️describe/🧩️component-stage/🟦️.ts";
import type { SemanticDescriptorEmissionV1 } from "../../🖨️describe/🛂️descriptor-emission/🟦️.ts";


import {withCompilerWorkspaceWriteV1} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {withComponentModelPublicationV1,describeComponentModelV1,publishComponentModelV1,type ComponentModelPublicationV1} from "../../📦️model/🟦️.ts";
import {componentStagePlanV1,type ComponentStageFacetV1} from "./📋️contract/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";

/** 🧩️ The post-link stages of one shipped plugin or extension component inside a build driver, each holding one jobserver token and cached by content:
 * `describe` (the semantic descriptor exactly these bytes answer, verified against the committed pair) and `transpile` start the moment the component is
 * staged and run beside the rest of the compilation; `optimize` (`wasm-opt -Oz` per core) follows the transpile; `probe` runs the assembled module's own
 * `describe()`; `publish` waits for the pass's provenance, writes the receipt bound to the Merkle root of the component's unit closure and publishes the
 * module; the `discard` finalizer then removes an assembled module no publish took and the stage work directory, whatever failed or was cancelled. A component
 * without a deployment directory is described and receipted only. `describeMode` `write` replaces a differing committed pair
 * (`components describe`) instead of refusing it. A plan's `stageAfter` adds the jobs of the same driver a stage
 * consumes (the descriptor emitter before `describe`, browser support before `probe`).
 * @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
 * @see ../../🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json */
export function componentDriverStagesV1(repoRoot:string,invocation:ScriptInvocation,publisher:ComponentModelPublicationV1,facet:ComponentStageFacetV1,describeMode:ComponentDescribeModeV1):(component:LinkedComponentV1)=>readonly NativeBuildStageV1[]{
 return component=>{
  const target=componentStageTargetV1(repoRoot,component.package.manifest),deliverable=Object.freeze({profile:componentProfileV1(component.pass.profile),directory:dirname(component.staged),wasm:component.staged,receipt:join(dirname(component.staged),"🧾️component.json")});
  let browserPublished=false,receipt:ComponentReceiptV1|undefined,transpiled:string|undefined,optimized:string|null=null,work:ComponentModuleWorkV1|undefined;
  const workOf=(control:CompilerAdmissionControl,cache:ComponentModuleWorkV1["cache"],root:string,environment:Readonly<Record<string,string|undefined>>):ComponentModuleWorkV1=>{if(!work){const directory=requireMaterializedPathV1(join(root,"w",target.packageName));rmSync(directory,{recursive:true,force:true});mkdirSync(directory,{recursive:true});work={invocation,repoRoot,target,deliverable,cache,work:directory,control};}work={...work,control,cache,environment};return work;};
  const run:Readonly<Record<string,NativeBuildStageV1["run"]>>={
   describe:async context=>{const emission=await describeComponentModelV1(publisher,component,context),drift=settleComponentDescriptorV1(repoRoot,target,deliverable,emission,describeMode);if(drift.length&&describeMode==="write")context.log(`[describe] ${target.pluginId}: rewrote ${drift.length} committed paths`);},
   model:async context=>{receipt=await publishComponentModelV1(publisher,component,await context.provenance());},
   transpile:async context=>{transpiled=await transpileComponentModulesV1(workOf(context.control,context.cache,context.generation.root,context.environment));},
   optimize:async context=>{optimized=await optimizeComponentCoresV1(workOf(context.control,context.cache,context.generation.root,context.environment),transpiled!,context.widen(availableParallelism()));},
   browser:async context=>{if(!receipt||!transpiled)throw Error("Actual model and transpile must complete");await publishComponentBrowserV1(workOf(context.control,context.cache,context.generation.root,context.environment),transpiled,optimized,receipt);browserPublished=true;},
   complete:async()=>{if(!receipt||facet==="browser"&&target.directoryName!==undefined&&!browserPublished)throw Error("Actual model and selected browser publication must complete");},
   discard:async()=>{if(work)rmSync(work.work,{recursive:true,force:true});}
  };
  return componentStagePlanV1({version:1,facet},target.directoryName!==undefined).map(row=>Object.freeze({name:row.name,after:row.name==="model"?[...row.after,`provenance:${component.pass.id}`]:row.after,...(row.name==="discard"?{finalizer:true}:{}),run:run[row.name]!}));
 };
}

/** 🏪️ Runs one build driver over `plan` inside the repository's charged compiler store of `mode`, the one owner every component driver run goes through:
 * the store issues the native storage reservation, the configured Source boundary, the Cargo home, target and build directories and the stage cache root,
 * and the bounded registry custody of every pass manifest is held under the caller's own network grant.
 * @see ../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts
 * @see ../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts */
function runRepositoryComponentDriverV1(repoRoot: string, mode: BuildMode, plan: NativeBuildPlanV1, options: Readonly<{ invocation:ScriptInvocation;facet:ComponentStageFacetV1;describeMode:ComponentDescribeModeV1;network: CargoDependencyNetworkV1; control: CompilerAdmissionControl; environment: Readonly<Record<string, string | undefined>>; generation: NativeBuildGenerationKindV1; consume?:NativeBuildDriverOptionsV1["consume"];maximumObservationBytes?:number; jobEnvironment?: (job: NativeBuildJobV1, generation: NativeBuildGenerationV1) => Readonly<Record<string, string>>; log?: (line: string) => void }>): Promise<NativeBuildOutcomeV1> {
  if (!plan.passes.length) throw Error("Component build driver requires declared Cargo passes");
  const declarations=plan.passes.flatMap(pass=>pass.packages.map(row=>{const target=componentStageTargetV1(repoRoot,row.manifest);return{manifest:resolve(repoRoot,row.manifest),intent:{version:1 as const,mode,pluginId:target.pluginId,cratePath:target.cratePath},packageId:target.packageId,packageName:target.packageName};}));
  return withRepositoryCacheStoreV1(repoRoot,"dependencies",options.invocation,options.control,(_store,storePhase)=>withRepositoryCompilerStoreV1(repoRoot,mode,storePhase,options.environment,(store,control)=>{
   const withCargoInputs:CargoCompilerInputsPortV1=(request,body)=>repositoryCargoCompilerInputsV1({workspace:repoRoot,invocation:options.invocation,consumption:repositoryCargoConsumptionV1(request.arguments,resolve(request.manifest)===resolve(repoRoot,DESCRIPTOR_EMITTER_MANIFEST_V1)?"descriptor-emitter":"artifact"),network:options.network,control,storage:store.storage})(request,body);
   return withCompilerWorkspaceWriteV1(store.storage,join(repoRoot,".cache/b/l/m/data"),control,()=>withComponentModelPublicationV1({workspace:repoRoot,invocation:options.invocation,storage:store.storage,control,toolchain:store.toolchain,declarations},publisher=>runNativeBuildDriverV1({storage:store.storage,sourceBoundary:store.sourceBoundary,withCargoInputs,plan,cwd:repoRoot,cacheRoot:join(store.root,"components"),shared:{target:store.environment.CARGO_TARGET_DIR!,build:store.buildDirectory},generation:options.generation,leaseDirectory:join(repoRoot,".cache/b/l/m/data"),environment:store.environment,control,stages:component=>componentDriverStagesV1(repoRoot,options.invocation,publisher,options.facet,options.describeMode)(component).map(stage=>stage.finalizer?stage:{...stage,run:context=>withIssuedResourceOperationV1(context.control,()=>withCompilerWorkspaceWriteV1(store.storage,join(repoRoot,".cache/b/l/m/data"),context.control,()=>stage.run(context)))}),...(options.consume?{consume:options.consume,maximumObservationBytes:options.maximumObservationBytes}:{}),...(options.jobEnvironment?{jobEnvironment:options.jobEnvironment}:{}),...(options.log?{log:options.log}:{})})));
  }));
}

/** 🚀️ Runs a product's whole release pipeline (`pipeline`, the `metadata.semio.release` of its release target) as one build driver in the compiler store of
 * the caller's build mode: one Cargo pass per component workspace restricted to `components` (every discovered component when omitted), the pipeline's
 * jobs with their owner policies, and the generation `SEMIO_NATIVE_BUILD_GENERATION` selects.
 * @see ../../../../../../../📜️script.ts
 * @see ../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts */
export function buildReleasePipelineV1(repoRoot: string, pipeline: NativeBuildPipelineV1, options: Readonly<{ invocation:ScriptInvocation; components?: ReadonlySet<string>; facet:ComponentStageFacetV1; network: CargoDependencyNetworkV1; control: CompilerAdmissionControl }>): Promise<NativeBuildOutcomeV1> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);
  const authority=readNativeCallerAuthorityV1(options.invocation.capabilities),mode=authority.original.buildMode;if(repoRoot!==authority.repositoryRoot)throw Error("Actual component driver workspace required");
  const { plan, jobEnvironment } = nativeBuildPipelinePlanV1(repoRoot, pipeline, mode, options.components);
  return runRepositoryComponentDriverV1(repoRoot,mode, plan, { invocation:options.invocation,facet:options.facet,describeMode:mode==="release"?"verify":"local",network: options.network, control: options.control, environment: process.env, generation: nativeBuildGenerationKindV1(process.env), jobEnvironment });
}

/** 🎯️ Builds exactly `pluginIds` — the closure one dev launch serves — through the one build driver over the warm stores: only the passes and packages of
 * those components, each streamed through its cached stages into the one plugin-modules root. A launch never pays for components it does not load.
 * @see ../../../🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts */
export function buildComponentClosureV1(repoRoot: string, pluginIds: ReadonlySet<string>, options: Readonly<{invocation:ScriptInvocation;facet:ComponentStageFacetV1;network:CargoDependencyNetworkV1; control: CompilerAdmissionControl; generation?:NativeBuildGenerationKindV1;consume?:NativeBuildDriverOptionsV1["consume"];maximumObservationBytes?:number;log?: (line: string) => void }>): Promise<NativeBuildOutcomeV1> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);
  const authority=readNativeCallerAuthorityV1(options.invocation.capabilities),mode=authority.original.buildMode;if(repoRoot!==authority.repositoryRoot)throw Error("Actual component driver workspace required");
  return runRepositoryComponentDriverV1(repoRoot,mode,componentPassPlanV1(repoRoot,pluginIds,mode), { invocation:options.invocation,facet:options.facet,describeMode:mode==="release"?"verify":"local",network: options.network, control: options.control, environment: process.env, generation:options.generation??"warm",...(options.consume?{consume:options.consume,maximumObservationBytes:options.maximumObservationBytes}:{}), ...(options.log ? { log: options.log } : {}) });
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
  const {all,pluginIds,profile}=componentsDescribeArgumentsV1(segments);if(repoRoot!==authority.repositoryRoot||profile!==componentProfileV1(buildModePolicy(mode).cargoProfile))throw Error("Describe selection must match its actual Original workspace and mode");
  const outcome=await runRepositoryComponentDriverV1(repoRoot,mode,componentPassPlanV1(repoRoot,all?undefined:new Set(pluginIds),mode),{invocation,facet:"model",describeMode:"write",network,control,environment:buildModeEnvironment(mode),generation:"warm"});
  console.log(`components describe: ${outcome.staged.length} component(s) described from ${profile} bytes; report ${outcome.report}`);
}
