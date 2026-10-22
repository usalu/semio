import type{ImplementationRowV1,InferenceProtocolV1}from"./🔖️stamps/🟨️.mjs";
import type{ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
/** 🕸️ Declares the actual JavaScript inference API while leaving dynamic result DATA untrusted. */
export function prepareSourceAnalysesV1():Readonly<Record<string,unknown>>;
/** 🚦️ Separates erased runtime edges from the complete compiler and production import projections. */
export interface ModuleSourceFactsV1{readonly imports:readonly string[];readonly runtimeImports:readonly string[];readonly productionImports:readonly string[]}
export function createScriptInputCache(context?:ScriptInputContextV1):ScriptInputCacheV1;
export function nativeSourceWatchPlanV1(root:string,workspaceRoot:string):Promise<unknown>;
export function inferReadOnlyBuildProjectSourcesV1(json:unknown,root:string,workspaceRoot:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<GeneratedProjectV1&{root:string;namedInputs:Record<string,unknown[]>}>;
export function discoverNativeBuildOwnersV1(configFiles:readonly string[],workspaceRoot:string,context:BuildInferenceContextV1):Promise<{files:string[];nativeCache:Map<string,unknown>;nativeClosures:Map<string,unknown>}>;
export function discoverBuildOwnerAdmissionFactsV1(owner:import("../🚪️admission/🟦️.ts").BuildOwnerRegistryV1["owners"][number],targets:readonly string[],registry:import("../🚪️admission/🟦️.ts").BuildOwnerRegistryV1,context:BuildInferenceContextV1,inputNames?:readonly string[]):Promise<import("../🚪️admission/🟦️.ts").BuildOwnerAdmissionFactsV1&{project:GeneratedProjectV1&{name:string}}>;
export function readBuildOwnerRegistrySourceV1(workspaceRoot:string):string;
/** 🔏️ Names the revision of the Source this module has on disk right now. */
export function currentImplementationRevisionV1():string;
/** 🔁️ Resolves the implementation of the Source this module has on disk right now, loading its revision graph when the Source changed. */
export function loadCurrentImplementationV1():Promise<BuildInferenceImplementationV1>;
export function dependencyResolutionAuthority(...args:readonly unknown[]):unknown;
export function createDependencies(...args:readonly unknown[]):unknown;
export function createMetadata(...args:readonly unknown[]):unknown;
declare const plugin:BuildInferencePluginV1;
export default plugin;
export const libraryBootstrap:Promise<unknown>;
export const cacheInternals:BuildInferenceInternalsV1;
export function discoverBuildProjectsV1(files:readonly string[],options:Readonly<Record<string,unknown>>,context:BuildInferenceContextV1):Promise<GeneratedNodesV1>;
export function discoverBuildDependenciesV1(options:Readonly<Record<string,unknown>>,context:BuildDependencyContextV1):Promise<readonly BuildInferenceEdgeV1[]>;

/** 📋️ Describes owned generated target DATA independently of any task runner package. */
export interface GeneratedTargetV1{readonly cache?:boolean;readonly continuous?:boolean;readonly dependsOn?:readonly (string|Readonly<Record<string,unknown>>)[];readonly outputs?:readonly string[];readonly inputs?:readonly unknown[];readonly options?:Readonly<Record<string,unknown>>;readonly metadata?:Readonly<Record<string,unknown>>;readonly [field:string]:unknown}
export interface GeneratedProjectV1{readonly targets:Readonly<Record<string,GeneratedTargetV1>>;readonly root?:string;readonly name?:string;readonly projectType?:string;readonly tags?:readonly string[];readonly sourceRoot?:string;readonly namedInputs?:Readonly<Record<string,readonly unknown[]>>;readonly metadata?:Readonly<Record<string,unknown>>}
export type GeneratedNodesV1=readonly(readonly[string,Readonly<{projects:Readonly<Record<string,GeneratedProjectV1>>;externalNodes?:Readonly<Record<string,unknown>>;unavailable?:readonly import("../🟦️.ts").BuildUnavailableProjectV1[]}>])[];
export type ScriptInputContextV1=Readonly<Record<string,unknown>>&({readonly invocation?:undefined;readonly phaseControl?:undefined}|{readonly invocation:ScriptInvocation;readonly phaseControl:ResourceLeaseControl});
export type BuildInferenceContextV1=ScriptInputContextV1&Readonly<{workspaceRoot:string}>;
/** 🔁️ Names the callable members of one revision of the inference module. */
export interface BuildInferenceImplementationV1{readonly revision:string;readonly cacheInternals:BuildInferenceInternalsV1;readonly libraryBootstrap:Promise<unknown>;createScriptInputCache(context?:ScriptInputContextV1):ScriptInputCacheV1}
export interface ScriptInputCacheV1{readonly files:Map<string,unknown>;readonly closures:Map<string,unknown>;readonly commandDependencies:Map<string,unknown>;readonly modules:Set<string>;readonly resources:Set<string>;readonly context:ScriptInputContextV1;readonly providerDesignation?:string}
export interface BuildInferenceEdgeV1{readonly source:string;readonly target:string;readonly type:string;readonly sourceFile?:string}
export interface BuildProjectFileV1{readonly file:string;readonly hash?:string}
export interface BuildFileMapV1{readonly projectFileMap:Readonly<Record<string,readonly BuildProjectFileV1[]>>;readonly nonProjectFiles?:readonly BuildProjectFileV1[]}
/** 🌳️ Names a project by its root; its generated targets are optional because dependency inference reads only roots. */
export type BuildProjectRootV1=Readonly<{root:string}>&Partial<GeneratedProjectV1>;
export type BuildDependencyContextV1=BuildInferenceContextV1&Readonly<{projects:Readonly<Record<string,BuildProjectRootV1>>;fileMap?:BuildFileMapV1;filesToProcess?:BuildFileMapV1;externalNodes?:Readonly<Record<string,unknown>>;nxJsonConfiguration?:Readonly<Record<string,unknown>>}>;
export type InferencePolicyV1=typeof import("../../../⚡️caching/🔣️policy.json").default;
export interface BuildInferencePluginV1{readonly name:string;readonly createNodesV2:readonly[string,(files:readonly string[],options:Readonly<Record<string,unknown>>,context:BuildInferenceContextV1)=>Promise<GeneratedNodesV1>];readonly createDependencies:(options:Readonly<Record<string,unknown>>,context:BuildDependencyContextV1)=>Promise<readonly BuildInferenceEdgeV1[]>;readonly createMetadata:(graph:unknown,options:unknown,context:BuildDependencyContextV1)=>Promise<unknown>}
/** 🔒️ Names the locked external package graph a Bun lockfile resolves to. */
export interface BunLockGraphV1{readonly externalNodes:Readonly<Record<string,{readonly type:"npm";readonly name:string;readonly data:Readonly<{packageName:string;version:string;hash:string}>}>>;readonly dependencies:readonly BuildInferenceEdgeV1[];readonly workspacePackages:ReadonlyMap<string,string>;resolve(from:string,name:string):string|undefined;resolveImport(file:string,name:string):string|undefined}
/** 🎡️ Names one playground row of a component crate together with its owning catalog contribution. */
export type PlaygroundCatalogRowV1=Readonly<Record<string,unknown>>&Readonly<{pluginId:string;cratePath:string;buildCatalog:import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1}>;
/** 🦀️ Names the bounded Rust source reference cache one inference pass shares. */
export interface RustSourceCacheV1{readonly entries:Map<string,unknown>;bytes:number;readonly limit:number;hits:number;misses:number}
/** 📊️ Names the project graph the module metadata of one build run is read from. */
export interface BuildMetadataGraphV1{readonly nodes:Readonly<Record<string,Readonly<{type:string;data:Readonly<{root:string;metadata?:Readonly<{semio?:Readonly<Record<string,unknown>>}>}>}>>>}
/** 🕸️ Names the module graph one project publishes. */
export interface BuildModuleGraphV1{readonly version:number;readonly files:{file:string;imports:string[];productionImports:string[]}[]}
/** 🗒️ Names the metadata one project contributes to the graph. */
export interface BuildMetadataEntryV1{readonly metadata:Readonly<{semio:Readonly<Record<string,unknown>>&Readonly<{moduleGraph:BuildModuleGraphV1}>}>}
/** 🧰️ Names real owned callable members while leaving unrelated dynamic DATA untrusted. */
/** 🧩️ The serial, cached `component` task of a plugin or extension crate: one store receipt over its exact inputs. */
export interface ComponentBuildTargetV1{readonly cache:boolean;readonly parallelism:boolean;readonly dependsOn:readonly string[];readonly inputs:readonly unknown[];readonly outputs:readonly string[];readonly options:Readonly<{command:string}>;readonly metadata:Readonly<{semio:Readonly<{execution:unknown;cache:Readonly<{class:string;reason:string}>}>}>}
export interface BuildInferenceInternalsV1 extends Readonly<Record<string,unknown>>{
 targetPolicy<T extends object>(name:string,target:T,policy?:InferencePolicyV1):T&{cache:boolean;continuous?:boolean};
 withWasmTooling<T extends Readonly<Record<string,GeneratedTargetV1>>>(targets:T,required:boolean):T;
 componentBuildTargets(root:string,workspaceRoot:string):Readonly<Record<string,ComponentBuildTargetV1>>;
 relativeScriptInputs(entries:readonly string[],workspaceRoot:string,cache?:ScriptInputCacheV1,requiredSources?:boolean):Promise<string[]>;
 projectInputs(json:Readonly<Record<string,unknown>>,root:string,workspaceRoot:string,facts:Map<string,unknown>,cache?:ScriptInputCacheV1,representation?:"source"|"hash",sourceGroups?:unknown,inputNames?:readonly string[],commandGroups?:Readonly<Record<string,unknown[]>>):Promise<Record<string,unknown[]>>;
 declaredSourceInputs(json:Readonly<Record<string,unknown>>,workspaceRoot:string,context?:ScriptInputContextV1,inputNames?:readonly string[]):Promise<Record<string,unknown[]>>;
 projectWithDefaults<T extends Readonly<Record<string,unknown>>>(json:T,root:string,projectDir:string,workspaceRoot:string,contracts?:Readonly<Record<string,unknown>>,facts?:Map<string,unknown>,commandInputs?:readonly string[],cache?:ScriptInputCacheV1,sourceGroups?:Readonly<Record<string,unknown[]>>,representation?:"source"|"hash",selectedTargets?:readonly string[],inputNames?:readonly string[]):Promise<T&GeneratedProjectV1>;
 workspaceCatalogBuildContributionsV1(workspaceRoot:string):Promise<readonly import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]>;
 playgroundPreparationTargets(configFiles:readonly string[],workspaceRoot:string,projectRoot:string,contracts:Readonly<Record<string,unknown>>,contributions:readonly import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[],context:BuildInferenceContextV1):Promise<Record<string,GeneratedTargetV1>>;
 createDependenciesImplementation(options:Readonly<{analyzeLockfile?:boolean}>,context:BuildDependencyContextV1):Promise<BuildInferenceEdgeV1[]>;
 collectImportEdges(workspaceRoot:string,files:Readonly<Record<string,readonly BuildProjectFileV1[]>>,projects:Readonly<Record<string,BuildProjectRootV1>>,byPackage:ReadonlyMap<string,string>,locked:unknown,add:(source:string,target:string|undefined,sourceFile:string)=>void,publication?:Record<string,{version:1;files:unknown[]}>,scheduledFiles?:ReadonlySet<string>,context?:ScriptInputContextV1):Promise<void>;
 withNativePreparation<T extends object>(project:T,workspaceRoot:string,contracts:Readonly<Record<string,unknown>>,cache?:Map<string,unknown>,projectsByRoot?:Map<string,unknown>,closures?:Map<string,unknown>):T;
 nativeLockInputs(command:string|GeneratedTargetV1):string[];
 genericTargetCommandInputs(target:GeneratedTargetV1,workspaceRoot:string,fallback:readonly string[],cache?:ScriptInputCacheV1,projectRoot?:string):Promise<string[]>;
 nativeTargetCommandInputs(target:GeneratedTargetV1,workspaceRoot:string,fallback:readonly string[]|undefined,cache:ScriptInputCacheV1|undefined,projectRoot?:string):Promise<string[]>;
 targetScriptClosure(target:GeneratedTargetV1,workspaceRoot:string,cache?:ScriptInputCacheV1,projectRoot?:string):Promise<string[]|undefined>;
 nativeDependencyRoots(root:string,workspaceRoot:string,tests?:boolean,cache?:Map<string,unknown>,closures?:Map<string,unknown>):string[];
 catalogNativeManifestCandidatesV1(workspaceRoot:string,registry:import("../🚪️admission/🟦️.ts").BuildOwnerRegistryV1,select:(manifest:string)=>unknown,context:BuildInferenceContextV1):Promise<string[]>;
 bunLockGraph(lock:Readonly<Record<string,unknown>>,patches?:Readonly<Record<string,Uint8Array|string>>):BunLockGraphV1;
 cargoTargets(root:string,workspaceRoot:string,commandInputs?:readonly string[],scripts?:ScriptInputCacheV1):Promise<Record<string,GeneratedTargetV1>>;
 collectPlaygroundCatalog(configFiles:readonly string[],workspaceRoot:string,catalogContributions?:readonly import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]):readonly PlaygroundCatalogRowV1[];
 cacheDecisionV1(decisionClass:string,reason:string):Readonly<{version:1;class:string;reason:string}>;
 targetClassV1(name:string,rows?:InferencePolicyV1["targetClasses"]):Readonly<{version:1;class:string;reason:string}>|undefined;
 normalizedTargetName(name:string):string;
 cachedOutputsAdmittedV1<T extends object>(target:T):T;
 nativeTargetRunsTestsV1(name:string,execution:Readonly<{kind:string;scope?:string}>|undefined,commandSources:readonly string[]):boolean;
 nativeAuthoredInputsV1(policyInputs:readonly unknown[],commandInputs:readonly unknown[]|undefined):unknown[];
 nativeTargetInputsV1(runsTests:boolean,generated:readonly unknown[],declared:readonly unknown[]):unknown[];
 pluginSiteTargetsForCrate(root:string,allPlaygrounds:readonly PlaygroundCatalogRowV1[],stagingProjectRoot:string|undefined,workspaceRoot:string):Record<string,GeneratedTargetV1>;
 walkCargoTomlControlledV1(workspaceRoot:string,control:Readonly<{check():void;advance(phase:string):Promise<void>}>):Promise<string[]>;
 createMetadataImplementation(graph:BuildMetadataGraphV1,options:Readonly<Record<string,unknown>>,context:BuildInferenceContextV1):Readonly<Record<string,BuildMetadataEntryV1>>;
 createRustSourceCache(limit?:number):RustSourceCacheV1;
 inferenceCandidates(configFiles:readonly string[],workspaceRoot:string):string[];
 playgroundSessionTargets(configFiles:readonly string[],workspaceRoot:string,catalogContributions?:readonly import("../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]):Promise<Record<string,GeneratedTargetV1>>;
 rustSourceFiles(entries:readonly string[],manifestRoot:string,cache?:RustSourceCacheV1,snapshots?:Map<string,unknown>):string[];
 targetScriptSources(target:GeneratedTargetV1,workspaceRoot:string,scripts?:ScriptInputCacheV1,projectRoot?:string,nativeManifest?:string):Promise<{closure:string[]|undefined;fallback:boolean}>;
 moduleSourceRecordCacheRootV1(workspaceRoot:string):string|undefined;
 sourceAnalysisOwnerIdentityV1(url:string,revision:string):string;
 targetExecutionKindV1(target:Readonly<{metadata?:Readonly<{semio?:Readonly<Record<string,unknown>>}>}>,identity?:string):string|undefined;
 validateGeneratorOutputProducersV1(contracts:Readonly<Record<string,unknown>>,projects:Readonly<Record<string,unknown>>,workspaceRoot:string,selection:import("../🚪️admission/🟦️.ts").BuildGraphSelectionV1,registry:import("../🚪️admission/🟦️.ts").BuildOwnerRegistryV1|undefined,context:BuildInferenceContextV1):Promise<void>;
}
/** 📋️ The authored files of the implementation this module loaded with the bytes they had at load. */
/** 🧱️ Classifies the loaded implementation rows by the declared stamp of every inferrer: data no inferrer covers stays a content fact, the code of an inferrer is named by its stamp. */
export function classifyImplementationRowsV1(rows:readonly ImplementationRowV1[],document?:InferenceProtocolV1):ImplementationRowV1[];
/** 🚧️ Names the executable files of the loaded implementation that no inferrer and no infrastructure fragment of the protocol claims. */
export function unclassifiedImplementationCodeV1(rows:readonly ImplementationRowV1[],document?:InferenceProtocolV1):string[];
/** 📜️ The declared inference protocol of the loaded revision. */
export function inferenceProtocolV1():InferenceProtocolV1;
export const loadedImplementationRowsV1:readonly {readonly kind:"digest";readonly path:string;readonly fact:{readonly sha256:string;readonly size:number;readonly mtimeMs:number}}[];
