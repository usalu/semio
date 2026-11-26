/** 🔎️ Declares the one owner-discovery member its component entry shares with the typed inference API. */
export const component: "owner-discovery";
export { discoverBuildOwnerAdmissionFactsV1 } from "../🟨️.d.mts";

/** 🏷️ The name under which what an owner admission read of the generator contracts travels as `global` rows. */
export const ADMISSION_GENERATOR_CONTRACTS_FACT_V1: "owner-admission-generator-contracts";
/** 📚️ A catalog provider: the owner root and the source path of its provider module. */
export type CatalogProviderV1=Readonly<{ownerRoot:string;sourcePath:string}>;
/** 📖️ Loads only the caller-declared optional providers and verifies current producer authority. */
export function catalogBuildContributionsV1(workspaceRoot: string, providers: readonly CatalogProviderV1[]): Promise<readonly import("../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]>;
/** 🦉️ Projects native catalog manifest candidates from canonical owner identities. */
export function catalogNativeManifestCandidatesV1(workspaceRoot: string, registry: import("../../🚪️admission/🟦️.ts").BuildOwnerRegistryV1, accept: (manifest:string)=>unknown, context: import("../🟨️.mjs").BuildInferenceContextV1): Promise<string[]>;
/** 📊️ How often the shared catalog was computed and how often an owner reused it (the counters of this process). */
export function catalogSharedStatsV1(): Readonly<{computed:number;reused:number}>;
/** 🛡️ Admits callback candidates through fresh raw physical workspace ancestry. */
export function inferenceCandidates(configFiles: Iterable<string>, workspaceRoot: string): string[];
export function projectAssemblyAuthorityV1(workspaceRoot: string, nativeRoot: string|undefined, projectsByRoot: ReadonlyMap<string,unknown>, componentRoots: unknown, nativeCache?: Map<string,unknown>, nativeClosures?: Map<string,unknown>): string;
/** 🏭️ The producer of a generated or produced compile output: its project, target, manifest, output path and sources. */
export type BuildCompileOutputV1=Readonly<{project:string;target:string;manifest:string;output:string;sources:readonly string[]}&Record<string,unknown>>;
/** 🏭️ Resolves only one exact ignored output contract to its physical authored task owner. */
export function readBuildGeneratedCompileOutputV1(path: string, workspaceRoot: string, contracts: Readonly<Record<string,unknown>>, registry: import("../../🚪️admission/🟦️.ts").BuildOwnerRegistryV1|undefined, context: import("../🟨️.mjs").BuildInferenceContextV1): BuildCompileOutputV1|undefined;
/** 🧾️ Reads the pinned registry for inference, or the current physical registry when checking a native owner failure. */
export function readBuildOwnerRegistrySourceV1(workspaceRoot: string, policy?: "current"): string;
export function readBuildProducedCompileOutputV1(path: string, workspaceRoot: string, contracts: Readonly<Record<string,unknown>>, registry: import("../../🚪️admission/🟦️.ts").BuildOwnerRegistryV1|undefined, context: import("../🟨️.mjs").BuildInferenceContextV1): BuildCompileOutputV1|undefined;
/** 🔎️ One scan of source input patterns, of a target and whether it is a compile input. */
export type BuildSourceScanV1=Readonly<{patterns:readonly string[];compile?:boolean;target:string}&Record<string,unknown>>;
/** 🧵️ Shares observed DFS ranges and identical source questions while keeping each target's input classification and bounded scan checkpoints. */
export function readBuildSourceInputMatchesV1(scans: readonly BuildSourceScanV1[], workspaceRoot: string, context: import("../🟨️.mjs").BuildInferenceContextV1): Promise<Readonly<{matched:Set<string>;compileMatched:Set<string>;compileByTarget:Map<string,Set<string>>}>>;
/** 🌫️ Reads text of a file that replicas may remove while inference runs: a vanished file (ENOENT/ENOTDIR) is absent, every other failure surfaces. */
export function readVanishableTextV1(path: string): string|undefined;
export function sharedCatalogV1(workspaceRoot: string, registry: import("../../🚪️admission/🟦️.ts").BuildOwnerRegistryV1, providers: readonly CatalogProviderV1[], context: import("../🟨️.mjs").BuildInferenceContextV1): Promise<Readonly<{contributions:readonly import("../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[];files:readonly string[]}>>;
/** 🦀️ Lists every workspace Cargo manifest through taxonomy-respecting discovery. */
export function walkCargoToml(workspaceRoot: string): string[];
/** 🧠️ Discovers the same complete canonical manifest scope through the caller's bounded continuation. */
export function walkCargoTomlControlledV1(workspaceRoot: string, control: Readonly<{check():void;advance(phase:string):Promise<void>}>): Promise<string[]>;
/** 🔌️ The coordinates of the sibling members this member is wired with. */
export type OwnerDiscoveryRegistryV1=Readonly<Record<"coordinates"|"scriptInputs"|"bootstrap"|"nativeManifests"|"componentTargets"|"sourceAnalyses"|"playgroundTargets"|"targetPolicy"|"projectAssembly",Readonly<Record<string,unknown>>>>;
/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry: OwnerDiscoveryRegistryV1): void;
/** 🧭️ Reads the explicit catalog options declared on this workspace plugin. */
export function workspaceCatalogBuildContributionsV1(workspaceRoot: string): Promise<readonly import("../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts").CatalogBuildContributionV1[]>;
