/** 🦀️ Declares the native-manifest inference component: Cargo and Go manifests, source files and native dependency closures. */
export const component: "native-manifests";
/** 🦀️ A parsed Cargo manifest: its package, workspace and per-target dependency tables. */
export type CargoManifestV1=Readonly<{package?:Readonly<{name?:string;workspace?:string}&Record<string,unknown>>;workspace?:Readonly<{dependencies?:Readonly<Record<string,unknown>>}&Record<string,unknown>>;target?:Readonly<Record<string,Readonly<Record<string,unknown>>>>}&Record<string,unknown>>;
/** 🏭️ A generator contract that a native consumer needs run first. */
export type NativeGeneratorContractV1=Readonly<{target?:string;previewTarget?:string;checkTarget?:string;ownership?:string;nativeConsumers?:readonly string[]}&Record<string,unknown>>;
export function wire(registry: Readonly<Record<string, Readonly<Record<string, unknown>>>>): void;
export function rustSourceFiles(entries: readonly string[], manifestRoot: string, cache?: Map<string, unknown>, snapshots?: Map<string, unknown>, compiled?: boolean, allowedFiles?:ReadonlySet<string>): string[];
export function assertedCargoBuildScriptInputsV1(cargo:unknown,script:string,workspaceRoot:string,operations:Readonly<{stat(path:string):Readonly<{isFile():boolean;isSymbolicLink():boolean;size:number}>;read(path:string):Uint8Array;sources(script:string,allowed:ReadonlySet<string>):string[]}>):string[]|undefined;
export function cargoSourceInputs(root: string, workspaceRoot: string, facts: Map<string, unknown> | undefined, includeTests?: boolean, compiled?: boolean): string[] | undefined;
export const TOML_RETRY_MILLISECONDS_V1: readonly number[];
export function readToml(path: string, options?: Readonly<{ milliseconds?: readonly number[]; sleep?(milliseconds: number): void }>): CargoManifestV1;
export function readCargoManifestRoleV1(path: string, role: "graph" | "graph-tests" | "sources" | "sources-tests"): CargoManifestV1;
export function nativeDependencies(manifest: CargoManifestV1, workspace: Readonly<{dependencies?:Readonly<Record<string,string|Readonly<{path?:string;package?:string}&Record<string,unknown>>|undefined>>|undefined}>): Readonly<{name:string;path:string;workspace:boolean;kind:"dependencies"|"build-dependencies"|"dev-dependencies"}>[];
export function nativeWorkspaceV1(directory: string, workspaceRoot: string, manifest?: CargoManifestV1): Readonly<{root:string;declaration:Readonly<Record<string,unknown>>}>;
export function nativeDependencyRoots(root: string, workspaceRoot: string, tests?: boolean, cache?: Map<string, unknown>, closures?: Map<string, unknown>): string[];
export function nativePreparation(root: string, workspaceRoot: string, contracts: Readonly<Record<string, NativeGeneratorContractV1>>, tests?: boolean, cache?: Map<string, unknown>, closures?: Map<string, unknown>): NativeGeneratorContractV1[];
export function withNativePreparation<T extends object>(project: T, workspaceRoot: string, contracts: Readonly<Record<string, NativeGeneratorContractV1>>, cache?: Map<string, unknown>, projectsByRoot?: Map<string, unknown>, closures?: Map<string, unknown>): T;
export function goDependencies(text: string): Readonly<{module:string;requires:string[];replacements:{module:string;path:string}[]}>;
export function goManifest(root: string, workspaceRoot: string): string|undefined;

/** 🦀️ The cargo source set of one crate: resolved with its source files and emitters, or refused with the reason. */
export type CargoSourceSetV1=Readonly<{version:1;root:string;resolved:true;files:readonly string[];emitters?:readonly string[]}|{version:1;root:string;resolved:false;reason:string;message?:string}>;
export function cargoSourceSetV1(root: string, workspaceRoot: string, facts: Map<string,unknown>|undefined, includeTests?: boolean, compiled?: boolean): CargoSourceSetV1;
export function cargoSourcesOrRefuseV1(root: string, workspaceRoot: string, facts: Map<string,unknown>|undefined, includeTests?: boolean, compiled?: boolean): string[]|undefined;
