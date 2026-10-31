/** 📥️ Declares the cached module import facts the inference owner exposes to its TypeScript consumers. */
export const sourceHash: string;
/** 🔗️ Projects canonical runtime module facts onto relative command source imports. */
export function commandSourceImports(path: string, source: string): string[];
/** 🧭️ Literal module requests grouped by the syntax-selected resolution mode. */
export interface ModuleSourceReferences { readonly version: 1; readonly imports: readonly string[]; readonly requires: readonly string[]; }
/** 🧭️ Retains import and require modes without exposing compiler types. */
export function runtimeModuleSourceReferences(path: string, source: string): ModuleSourceReferences;
/** 🚀️ Retains runtime package and relative imports without erased declaration dependencies. */
export function runtimeModuleSourceImports(path: string, source: string): string[];
/** 🔗️ Projects runtime and optional test module facts for project dependency ownership. */
export function moduleSourceImports(path: string, source: string): string[];
/** 🏗️ Keeps typed and runtime production imports outside command and test ownership. */
export function productionModuleSourceImports(path: string, source: string): string[];
/** 🔗️ Retains the literal paths a script resolves against itself with `new URL("…", import.meta.url)`, which name the test files it runs and the data it reads. */
export function commandUrlReferences(path: string, source: string): string[];
/** 📊️ Reports bounded immutable parser fact storage without source authority. */
export function commandSourceParseStats(): Readonly<{ hits: number; misses: number; bytes: number; limit: number }>;
