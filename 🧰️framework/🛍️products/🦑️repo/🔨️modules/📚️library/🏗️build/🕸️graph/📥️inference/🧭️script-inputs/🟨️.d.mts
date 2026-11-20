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
export function mergeWalkMembersV1(table: { get(key: string): string[] | undefined; set(key: string, value: string[]): unknown }, digest: string, members: string[]): boolean;
export function closureCollectionsV1(): { name: string; entries: number; bytes: number }[];
export function closureEntryCountsV1(): { derived: number; loaded: number; stale: number; checked: number };
export function closureStoreDirectoryV1(workspaceRoot: string): string | undefined;
export function bindClosureStoreV1(workspaceRoot: string, directory: string, writable?: boolean): void;
export function closureApiV1(scripts: ScriptInputsValueV1, workspaceRoot: string): ScriptInputsValueV1;
export function derivedWalkAnswerV1(value: unknown): string;
export function expandedCommandInputsV1<T>(value: T, workspaceRoot: string, scripts?: ScriptInputsValueV1): Promise<T>;
export function resolveCommandWalkV1(workspaceRoot: string, entries: readonly string[], cache: ScriptInputsValueV1): Promise<{ readonly value: readonly string[]; readonly rows: readonly ScriptInputsValueV1[]; readonly derived: readonly { readonly row: ScriptInputsValueV1; readonly support: readonly ScriptInputsValueV1[] }[] }>;
export function yieldInferenceV1(context: ScriptInputsValueV1, phase: string, index: number, force?: boolean): Promise<void>;
export function withClosurePassV1<T>(compute:()=>Promise<T>|T,options?:{verdicts?:string}):Promise<T>;
export function closureVerdictScopeV1():string|undefined;
export function closureVerdictCountsV1():{read:number;written:number;waited:number};

/** 📋️ The recipes every owner reads and none depends on its own target. */
export function sharedClosureRecipesV1(scripts: ScriptInputsValueV1): readonly Record<string, unknown>[];
/** 🧰️ Derives the shared closure entries once, in order, before the owners start. */
export function primeSharedClosuresV1(workspaceRoot: string, scripts: ScriptInputsValueV1, options?: { signal?: AbortSignal; progress?: (event: { done: number; total: number }) => unknown; recipes?: readonly Record<string, unknown>[] }): Promise<{ recipes: number; derived: number; loaded: number; stale: number }>;

/** 🚚️ The closure entries of a project record as portable bundles and back. */
export const closureTravelV1: Readonly<{ export(workspaceRoot: string, image: string): Promise<readonly Readonly<{ recipeKey: string; stamp?: string; bundle: unknown }>[]>; import(workspaceRoot: string, bundles: readonly Readonly<{ recipeKey: string; bundle: unknown; source?: string; neutral?: string }>[]): Promise<number>; held(workspaceRoot: string, source: string, neutral?: string, refusedMilliseconds?: number): Promise<string | undefined>; claim(workspaceRoot: string, neutral: string, staleMilliseconds: number): Promise<(() => Promise<void>) | undefined>; claimed(workspaceRoot: string, neutral: string): Promise<boolean> }>;

/** 🫧️ Releases what one finished owner left in the shared script input cache. */
export function closureEntriesWithLayersV1(): number;
export function chargedToSharedV1<T>(cache: object, work: () => Promise<T>): Promise<T>;
export function releaseOwnerStateV1(scripts: ScriptInputsValueV1, options?: { shrink?: boolean; now?: boolean; gate?: { grantBytes(): number | undefined; footprintBytes(): number } }): void;
/** 📈️ Whether a look releases the shared cache: the managed memory grew `lookReleaseGrowthBytes` beyond its lowest reading, or the host is short. */
export function lookReleaseDueV1(lowestBytes: number, managedBytes: number, options?: { shrink?: boolean; limits?: { lookReleaseGrowthBytes: number } }): boolean;
/** 🪤️ Runs a take look in which no closure entry is derived: a missing or stale entry ends it with `BUILD_TAKE_MISS`. */
export function runClosureTakeV1<T>(body: () => Promise<T>, signal?: AbortSignal): Promise<T>;
export function releaseAfterTakeLookV1(scripts: ScriptInputsValueV1 | undefined, options?: { shrink?: boolean; gate?: { grantBytes(): number | undefined; footprintBytes(): number } }): void;
/** 🛃️ Walks every declared tool closure and provider port once and returns the refusals. */
export function preflightToolClosuresV1(workspaceRoot: string, scripts: ScriptInputsValueV1): Promise<{ checked: number; refused: readonly { name: string; version: 1; code: string; key: string; values: Record<string, string | number> }[] }>;

export function closureLayerListV1(log: unknown, view: { key: unknown; digest?: string; store?: unknown; rows?: unknown }): any;
export function layerVerdictCountsV1(): { hits: number; joined: number; checked: number; indexed: number; milliseconds: number };
