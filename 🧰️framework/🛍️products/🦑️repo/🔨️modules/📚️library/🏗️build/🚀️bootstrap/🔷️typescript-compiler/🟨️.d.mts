/** 🔷️ The TypeScript compiler API of the tooling as the tooling uses it: its version and the members named by the schema; the members are read by name and carry no type of the compiler package. */
export type TypescriptCompilerV1 = Readonly<{ version: string }> & Readonly<Record<string, any>>;
/** 📍️ Where to look: the workspace root and the directory of the parser package; both default to the checkout this module sits in. */
export type TypescriptCompilerOptionsV1 = Readonly<{ root?: string; parserDirectory?: string; load?: () => unknown }>;
/** 📋️ One directory the tooling looks in: its path, the version found there (`null` when absent) and the API names missing there. */
export type TypescriptCompilerCandidateV1 = Readonly<{ path: string; found: string | null; missing: readonly string[] }>;
/** 🚫️ The closed refusal record of the schema: English and German text, never retried. */
export type TypescriptCompilerRefusalRecordV1 = Readonly<{
  code: "typescript-compiler-declaration" | "typescript-compiler-absent" | "typescript-compiler-version" | "typescript-compiler-api" | "typescript-compiler-load";
  retry: "never"; wanted: string | null; resolved: string; found: string | null; missing: readonly string[]; cause: string | null; tried: readonly TypescriptCompilerCandidateV1[]; en: string; de: string;
}>;
/** 🚫️ The error thrown when the tooling compiler is not the declared one. */
export class TypescriptCompilerRefusalV1 extends Error { readonly code: TypescriptCompilerRefusalRecordV1["code"]; readonly retry: "never"; readonly refusal: TypescriptCompilerRefusalRecordV1; constructor(refusal: TypescriptCompilerRefusalRecordV1); }
/** 🔎️ Whether the declared compiler is installed where the tooling loads it, read from two manifests without loading the compiler. */
export function inspectTypescriptCompilerV1(options?: TypescriptCompilerOptionsV1): Readonly<{ ok: true; path: string; version: string; wanted: string }> | Readonly<{ ok: false; refusal: TypescriptCompilerRefusalRecordV1 }>;
/** 📍️ The main file of the declared compiler, read from manifests without loading it, or `TypescriptCompilerRefusalV1`. */
export function typescriptCompilerFileV1(options?: TypescriptCompilerOptionsV1): string;
/** 🔷️ Loads the declared TypeScript compiler API or throws `TypescriptCompilerRefusalV1`. */
export function typescriptCompilerV1(options?: TypescriptCompilerOptionsV1): TypescriptCompilerV1;
