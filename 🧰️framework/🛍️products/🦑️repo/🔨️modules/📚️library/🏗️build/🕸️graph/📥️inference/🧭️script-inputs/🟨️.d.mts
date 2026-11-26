/** 🧭️ Types of the script-inputs component: command closures stay structural data (their rows are untrusted until a producer admits them). */
export type ScriptInputsValueV1 = any;
export const component: "script-inputs";
export function wire(registry: ScriptInputsValueV1): void;
export function createScriptInputCache(context?: ScriptInputsValueV1): ScriptInputsValueV1;
export function scriptProviderPortsV1(text: string): readonly string[];
export function designatedPortsV1(cache: ScriptInputsValueV1, ports: readonly string[]): readonly string[];
export function buildImportResourceV1(containing: string, specifier: string): { readonly specifier: string; readonly suffix: string; readonly source: boolean };
export function declaredInputPatternV1(input: ScriptInputsValueV1, projectRoot: string): ScriptInputsValueV1;
export function declaredModuleGlobsV1(inputs: ScriptInputsValueV1, workspaceRoot: string, projectRoot: string, environment: ScriptInputsValueV1): ScriptInputsValueV1;
export function declaredTypeScriptProjectsV1(inputs: ScriptInputsValueV1, workspaceRoot: string, projectRoot: string, environment: ScriptInputsValueV1): ScriptInputsValueV1;
export function relativeScriptInputs(entries: ScriptInputsValueV1, workspaceRoot: string, cache?: ScriptInputsValueV1, requiredSources?: boolean, resourceEntries?: ScriptInputsValueV1, commands?: ReadonlyMap<string, readonly string[]>): Promise<ScriptInputsValueV1>;
export function withoutSchedulingV1(closure: readonly ScriptInputsValueV1[], prefixes?: readonly string[]): ScriptInputsValueV1[];
export function nativeCommandInputs(workspaceRoot: string, scripts: ScriptInputsValueV1): Promise<ScriptInputsValueV1>;
export function targetScriptEntriesV1(target: ScriptInputsValueV1, workspaceRoot: string, projectRoot?: string, nativeManifest?: ScriptInputsValueV1): Promise<{resources: string[]; entries: Set<string>; words: Map<string, Set<string> | null>; fallback: boolean}>;
export function targetScriptSources(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string, nativeManifest?: ScriptInputsValueV1, cover?: "native" | "generic" | "queries"): Promise<ScriptInputsValueV1>;
export function ownerCommandChildV1(options: ScriptInputsValueV1, bound: ScriptInputsValueV1): { readonly cwd: string; readonly command: string } | undefined;
export function targetScriptClosure(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string, cover?: "native" | "generic" | "queries"): Promise<ScriptInputsValueV1>;
export function nativeLockInputs(command: ScriptInputsValueV1): ScriptInputsValueV1;
export function nativeTargetCommandInputs(target: ScriptInputsValueV1, workspaceRoot: string, fallback: ScriptInputsValueV1, scripts: ScriptInputsValueV1, projectRoot?: string): Promise<ScriptInputsValueV1>;
export function genericTargetCommandInputs(target: ScriptInputsValueV1, workspaceRoot: string, fallback: ScriptInputsValueV1, scripts?: ScriptInputsValueV1, projectRoot?: string): Promise<ScriptInputsValueV1>;
export function genericCommandFallbackInputs(workspaceRoot: string): ScriptInputsValueV1;
export function closureRecipeByKeyV1(key: string): ScriptInputsValueV1;
export function closureRecipeKeyV1(recipe: ScriptInputsValueV1): string;
export function closureHandleFromRecipeV1(recipe: ScriptInputsValueV1): ScriptInputsValueV1;
export function closureHandleOfV1(value: unknown): ScriptInputsValueV1;
export function registerClosureQueryV1(name: string, query: ScriptInputsValueV1): void;
export function bindClosureScriptsV1(workspaceRoot: string, scripts: ScriptInputsValueV1): void;
export function resetClosureEntriesV1(entryBytes?: number): void;
export function closureCollectionsV1(): { name: string; entries: number; bytes: number }[];
/** 🌊️ Forgets the verdicts of the rounds of finished waves. */
export function forgetRoundVerdictsV1(): void;
export function closureEntryCountsV1(): { derived: number; loaded: number; stale: number; checked: number; fetched: number; reasked: number; reaskFailed: number; reaskEqual: number; reaskDifferent: number };
export function closureStoreDirectoryV1(workspaceRoot: string): string | undefined;
export function flushClosureOffersV1(): Promise<void>;
export function bindClosureStoreV1(workspaceRoot: string, directory: string, writable?: boolean, limits?: Readonly<Record<string, number>>): void;
/** 🔑️ The name of one open closure store: workspace, directory, read-only flag and limits. */
export function closureStoreKeyV1(workspaceRoot: string, directory: string, bound?: { readonly writable: boolean; readonly limits?: Readonly<Record<string, number>> }): string;
export function closureApiV1(scripts: ScriptInputsValueV1, workspaceRoot: string): ScriptInputsValueV1;
export function derivedWalkAnswerV1(value: unknown): string;
export function expandedCommandInputsV1<T>(value: T, workspaceRoot: string, scripts?: ScriptInputsValueV1): Promise<T>;
export function resolveCommandWalkV1(workspaceRoot: string, entries: readonly string[], cache: ScriptInputsValueV1): Promise<{ readonly value: readonly string[]; readonly rows: readonly ScriptInputsValueV1[]; readonly derived: readonly { readonly row: ScriptInputsValueV1; readonly support: readonly ScriptInputsValueV1[] }[] }>;
export function yieldInferenceV1(context: ScriptInputsValueV1, phase: string, index: number, force?: boolean): Promise<void>;
export function throwIfInferenceCancelledV1(context: ScriptInputsValueV1 | undefined): void;
export function withClosurePassV1<T>(compute:()=>Promise<T>|T,options?:{verdicts?:string}):Promise<T>;
export function closureVerdictScopeV1():string|undefined;
export function closureVerdictCountsV1():{read:number;written:number;waited:number};

/** 📋️ The recipes every owner reads and none depends on its own target. */
export function sharedClosureRecipesV1(scripts: ScriptInputsValueV1): readonly Record<string, unknown>[];
/** 🧰️ Derives the shared closure entries once, in order, before the owners start. */
export function primeSharedClosuresV1(workspaceRoot: string, scripts: ScriptInputsValueV1, options?: { signal?: AbortSignal; progress?: (event: { done: number; total: number }) => unknown; recipes?: readonly Record<string, unknown>[] }): Promise<{ recipes: number; derived: number; loaded: number; stale: number }>;

/** 🚚️ The closure entries of a project record as portable bundles and back. */
export const closureTravelV1: Readonly<{ export(workspaceRoot: string, image: string): Promise<readonly Readonly<{ recipeKey: string; stamp?: string; bundle: import("../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts").ClosureBundleV1 }>[]>; import(workspaceRoot: string, bundles: readonly Readonly<{ recipeKey: string; bundle: unknown; source?: string; neutral?: string }>[]): Promise<number>; held(workspaceRoot: string, source: string, neutral?: string, refusedMilliseconds?: number): Promise<string | undefined>; claim(workspaceRoot: string, neutral: string, staleMilliseconds: number): Promise<(() => Promise<void>) | undefined>; claimed(workspaceRoot: string, neutral: string): Promise<boolean> }>;

/** 🫧️ Releases what one finished owner left in the shared script input cache. */
export function closureEntriesWithLayersV1(): number;
export function releaseOwnerStateV1(scripts: ScriptInputsValueV1, options?: { shrink?: boolean; now?: boolean; gate?: { grantBytes(): number | undefined; footprintBytes(): number } }): void;
/** 📈️ Whether a look releases the shared cache: the managed memory grew `lookReleaseGrowthBytes` beyond its lowest reading, or the host is short. */
export function lookReleaseDueV1(lowestBytes: number, managedBytes: number, options?: { shrink?: boolean; limits?: { lookReleaseGrowthBytes: number } }): boolean;
/** 🪤️ Runs a take look in which no closure entry is derived: a missing or stale entry ends it with `BUILD_TAKE_MISS`. */
export function runClosureTakeV1<T>(body: () => Promise<T>, signal?: AbortSignal, options?: { reask?: true | ((workspaceRoot: string, recipe: unknown) => Promise<{ value: unknown; rows: unknown[]; layers: unknown[][] } | undefined>) }): Promise<T>;
export function firstMovedRowOfEntryV1(entry: Readonly<{rows?: readonly unknown[]; layers?: readonly (readonly unknown[])[]; digests?: readonly string[]; store?: unknown}>, budgetMilliseconds?: number): Readonly<{kind: string; path: string; fact: unknown}> | undefined;
export function closureFailureWhyV1(error: unknown, key: string): string;
export const importRefusalsV1: Readonly<{get(id: string): Readonly<{kind: string; path: string; fact: unknown}> | null | undefined; has(id: string): boolean; set(id: string, row: Readonly<{kind: string; path: string; fact: unknown}> | null): boolean; delete(id: string): boolean; clear(): void; stats(): Readonly<{entries: number}>}>;
export function rememberImportRefusalV1(id: string, bundle: Readonly<{rows?: readonly unknown[]}>): void;
export function releaseAfterTakeLookV1(scripts: ScriptInputsValueV1 | undefined, options?: { shrink?: boolean; gate?: { grantBytes(): number | undefined; footprintBytes(): number } }): void;
/** 🛃️ Walks every declared tool closure and provider port once and returns the refusals. */
export function preflightToolClosuresV1(workspaceRoot: string, scripts: ScriptInputsValueV1, options?: { progress?: (event: { done: number; total: number }) => void | Promise<void> }): Promise<{ checked: number; refused: readonly { name: string; version: 1; code: string; key: string; values: Record<string, string | number> }[] }>;

export function closureLayerListV1(log: unknown, view: { key: unknown; digest?: string; store?: unknown; rows?: unknown }): any;
export function layerVerdictCountsV1(): { hits: number; joined: number; checked: number; indexed: number; milliseconds: number };
export const scriptProviderIdsV1: () => readonly string[];
/** 🌳️ One walk question of the module closure graph. */
export type ClosureGraphQuestionV1 = { roots: readonly string[]; requiredSources?: boolean; resources?: readonly string[]; selectors?: Iterable<readonly [string, readonly string[]]>; stops?: Iterable<readonly [string, string]> };
/** 🌳️ One pass of the module closure graph over walk questions. */
export function closureGraphPassV1(cache: any, workspaceRoot: string, questions: readonly ClosureGraphQuestionV1[], options?: { graph?: import("./🌳️closure-graph/🟨️.mjs").ClosureGraphV1; signal?: AbortSignal; progress?: (step: { done: number; total: number }) => unknown; witness?: boolean; prune?: boolean; trusted?: { has(key: string): boolean }; pass?: object }): Promise<{ digests: (string | undefined)[]; visited: number; built: number; pruned: number; graph: import("./🌳️closure-graph/🟨️.mjs").ClosureGraphV1 }>;
/** 🧭️ The walk question of one target's command entries. */
export function targetClosureQuestionV1(target: unknown, workspaceRoot: string, scripts: any, projectRoot?: string, nativeManifest?: string): Promise<ClosureGraphQuestionV1 | undefined>;
/** 🌳️ The closure digest the module graph answers for one walk question, once per validation round. */
export function closureGraphAnswerV1(cache: any, workspaceRoot: string, question: ClosureGraphQuestionV1, pass?: object): Promise<Readonly<{digest:string;answer:unknown}>>;
/** 🗄️ The directory the module graph of a workspace is kept in on this machine. */
export function closureGraphDirectoryV1(workspaceRoot: string): string;
/** 💾️ Saves now every module graph of this process that changed since its last save. */
export function flushClosureGraphsV1(): Promise<void>;
/** 🪪️ The keys of the stored nodes whose witness holds now. */
export function closureGraphWitnessedV1(graph: import("./🌳️closure-graph/🟨️.mjs").ClosureGraphV1, options?: { step?: () => unknown }): Promise<Set<string>>;

export {askHeavyQuestionV1, isHeavyQuestionV1, preflightToolClosuresInChildV1, primeSharedClosuresInChildV1} from "./🪆️heavy-questions/🟨️.mjs";
/** 🔧️ The TypeScript compiler module the closure walkers resolve modules with. */
export type ScriptCompilerV1=ReturnType<typeof import("../../../🚀️bootstrap/📦️parsers/🟨️.mjs").parserTypescriptV1>;
/** 🗂️ The neutral file system of the compiler: what the walkers read through it. */
export type ScriptHostV1=import("../../../👁️observation/🟨️.mjs").SourceTypeScriptSystemV1;
/** 📋️ One closure recipe: the function that derives it and the target, entries, role or port it derives for. */
export type ClosureRecipeV1=Readonly<{fn?:string;projectRoot?:string;target?:Readonly<{options?:Readonly<{command?:string;cwd?:string}>}>;role?:string;name?:string;view?:string;port?:string;entries?:readonly string[]}&Record<string,unknown>>;
/** 🧭️ The ambient the walkers read their owner context and cancellation from. */
export type ClosureScriptsV1=Readonly<{context?:Readonly<{ownerSignal?:AbortSignal;phaseControl?:Readonly<{signal?:AbortSignal}>;invocation?:Readonly<{control?:Readonly<{signal?:AbortSignal}>}>}>}&Record<string,unknown>>;
/** 🏷️ What one walk observed beside its value: the rows it read and the layers of stored rows. */
export type ClosureWalkV1<T=unknown>=Readonly<{value:T;rows:unknown[];layers:(readonly unknown[])[]}>;
export function answeredAgainstFreshProvidersV1<T>(answer: () => T|undefined, refresh: () => Promise<unknown>|unknown, attempts?: number): Promise<T|undefined>;
export function answeredAgainstLiveLayersV1<T>(workspaceRoot: string, body: () => Promise<T>|T, options?: Readonly<{attempts?:number;say?:(line:string)=>void}>): Promise<T>;
/** 🪧️ The claim on the first derivation of one shared closure entry: the process that takes it derives the entry, the others wait for the stored entry and reuse it; undefined when the read has no verdict directory or no store. A claim is released when the derivation ends, so a later derivation of a stale entry is claimed anew. */
export function claimDerivationV1(id: string, store: Readonly<{load(id:string):Promise<Readonly<Record<string,unknown>>|undefined>}>|undefined, scripts: ClosureScriptsV1): Promise<Readonly<{release():void}>|Readonly<{entry:Readonly<Record<string,unknown>>}>|undefined>;
/** 🧶️ The fleet lane of closure entries of one workspace (see `🪪️graph-state/📦️project-records/🚚️entry-lane`), or undefined when no fleet cache is configured here or `SEMIO_CLOSURE_FLEET=off`; opened once. */
export function closureFleetLaneV1(workspaceRoot: string): Promise<import("../../../🪪️graph-state/📦️project-records/🚚️entry-lane/🟦️.ts").ClosureEntryLaneV1|undefined>;
/** 🔁️ Asks a derived fact of the command closure again against the live files (a repository walk or a TypeScript program): its answer digest and every observation the computation read, for a generation that decides whether the projects holding the fact stay valid. */
export function deriveClosureFactV1(row: Readonly<{path:string;kind?:string}>, cache: Record<string,unknown>, pass?: Record<string,unknown>): Promise<Readonly<{fact:string;rows:unknown[]}>>;
/** 🏭️ The walk of a child with the derived facts its walks recorded for the layers (identity data of the process that walked them, so they travel beside the layers). */
export function deriveClosureRecipeForChildV1(workspaceRoot: string, recipe: ClosureRecipeV1, scripts: ClosureScriptsV1): Promise<ClosureWalkV1&Readonly<{sidecar:Readonly<{facts:readonly unknown[]}>}>>;
/** 🏭️ The detached walk of one recipe, `{ value, rows, layers }`, as a walk child runs it (see `offloadClosureRecipeV1`). */
export function deriveClosureRecipeV1(workspaceRoot: string, recipe: ClosureRecipeV1, scripts: ClosureScriptsV1): Promise<ClosureWalkV1>;
/** 🧶️ Takes the entries of `recipes` that this workspace's store lacks from the fleet in one batch, before any of them is derived; answers how many were taken. */
export function prefetchClosureEntriesV1(workspaceRoot: string, recipes: readonly ClosureRecipeV1[], signal?: AbortSignal): Promise<number>;
/** 🪪️ The name of the providers project read as two fields of its configuration (its name and the designation it declares), so an edit of any other line of the configuration moves no walk of any project. */
export function providerProjectOfConfigV1(config: string, designation: string|undefined): string|undefined;
/** 🌍️ The module resolution configuration bun reads for every module of a command closure (checked with the kernel read oracle: `undeclared-g`): the nearest `package.json` and the nearest `tsconfig.json` above each module directory, and the `tsconfig.json` files an `extends` names; the walk stops below the workspace root, whose own files are shared inputs of every target. Answers through the observed host, so a config that appears or changes re-keys the walk. */
export function resolutionConfigsV1(compiler: ScriptCompilerV1, host: ScriptHostV1, workspaceRoot: string, modules: Iterable<string>): string[];
export function tsconfigCompilerOptionsV1(compiler: ScriptCompilerV1, host: ScriptHostV1, workspaceRoot: string, file: string): import("typescript").CompilerOptions;
/** 🧭️ The Bundler resolution options of a module of a command closure, with the `baseUrl` and `paths` of its tsconfig. */
export function tsconfigResolutionOptionsV1(compiler: ScriptCompilerV1, host: ScriptHostV1, workspaceRoot: string, file: string): import("typescript").CompilerOptions;
/** 📦️ Resolves a package specifier to the workspace module it names, so imports through `@scope/package` subpaths join a command closure like relative ones; installed registry packages stay covered by their locked external dependency. */
export function workspacePackageModuleV1(containing: string, specifier: string, workspaceRoot: string, compiler?: ScriptCompilerV1, host?: ScriptHostV1, mode?: import("typescript").ResolutionMode, declarations?: boolean, producedOutput?: (specifier:string)=>Readonly<{output:string;declarations?:string}>|undefined): string|undefined;
