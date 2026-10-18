import { type RustStructuralVisibility } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

export type RustStructuralFieldStyle = "unit" | "tuple" | "struct";

export interface RustModuleFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly inline: boolean;
  readonly pathTarget: string | null;
  readonly cfgTest: boolean;
}

/** 🪺️ Refuses module authority whose authored metadata can change its structure. */
export type RustModuleMetadataProblem = Readonly<{ code: "unsupported-attribute"; attributes: readonly string[] } | { code: "ambiguous-path"; paths: readonly (string | null)[] }>;

/** 🏘️ Retains the exact half-open UTF16 body of a physical root or inline module. */
export interface RustModuleScopeFact {
  readonly kind: "root" | "inline";
  readonly modulePath: readonly string[];
  readonly bodyStartOffset: number;
  readonly bodyEndOffset: number;
  readonly crateCapabilities?: readonly "associated_type_defaults"[];
  readonly unresolved?: RustModuleMetadataProblem;
}

/** 🧾️ Owns structural observations independently of admitted module membership. */
export interface RustModuleGraphFacts {
  readonly modules: readonly RustModuleGraphFact[];
  readonly uses: readonly RustModuleUseFact[];
  readonly includes: readonly RustModuleIncludeFact[];
  readonly scopes: readonly RustModuleScopeFact[];
}

/** 🛂️ Requires a unique proven physical root and every enclosing inline module body. */
export type RustModuleScopeProof = Readonly<{ state: "resolved"; scopes: readonly RustModuleScopeFact[] } | { state: "unresolved"; modulePath: readonly string[]; problem: RustModuleMetadataProblem | Readonly<{ code: "scope-not-unique"; count: number }> }>;

export interface RustModuleGraphFact {
  readonly name: string;
  readonly modulePath: readonly string[];
  readonly visibility: RustStructuralVisibility;
  readonly inline: boolean;
  readonly pathTarget: string | null;
  readonly declarationOffset: number;
  readonly macroUse?: true;
  readonly unresolved?: RustModuleMetadataProblem;
  readonly conditional?: true;
}

export interface RustModuleUseFact {
  readonly modulePath: readonly string[];
  readonly specifier: string;
  readonly relation: "import" | "reexport";
  readonly visibility: RustStructuralVisibility;
  readonly conditional?: true;
}

/** 📎️ One `include!("…")` expansion: the named file's tokens become part of THIS file's module, so
 * the file compiles without ever being a `mod` of its own. `path` is the literal as written, which
 * `rustc` resolves against the directory of the file the invocation sits in. */
export interface RustModuleIncludeFact {
  readonly modulePath: readonly string[];
  readonly path: string;
  readonly conditional?: true;
}

export interface RustEnumVariantFact {
  readonly name: string;
  readonly fieldStyle: RustStructuralFieldStyle;
  readonly fieldTypes: readonly string[];
  readonly wrappedTupleLeafType: string | null;
  readonly conditional?: true;
}

export interface RustEnumFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly variants: readonly RustEnumVariantFact[];
  readonly conditional?: true;
}

export interface RustPayloadFieldFact {
  readonly name: string | null;
  readonly type: string;
}

export interface RustInlinePayloadFact {
  readonly name: string;
  readonly visibility: RustStructuralVisibility;
  readonly fieldStyle: RustStructuralFieldStyle;
  readonly fields: readonly RustPayloadFieldFact[];
}

export interface RustImplFact {
  readonly traitPath: string | null;
  readonly selfType: string;
  readonly methods: readonly string[];
  readonly associatedConstants: readonly string[];
}

export interface RustConstIdentityFact {
  readonly owner: string | null;
  readonly name: string;
  readonly type: string | null;
  readonly value: string;
  readonly stringValue: string | null;
  readonly identityFields: Readonly<Record<string, string>>;
}

export interface RustMatchArmFact {
  readonly pattern: string;
  readonly variantPath: string | null;
  readonly expression: string;
}

export interface RustIncludeFact {
  readonly macro: "include" | "include_str" | "include_bytes";
  readonly expression: string;
  readonly usesOutDir: boolean;
}

export interface RustStructuralFacts {
  readonly schemaVersion: 1;
  readonly modules: readonly RustModuleFact[];
  readonly enums: readonly RustEnumFact[];
  readonly impls: readonly RustImplFact[];
  readonly inlinePayloads: readonly RustInlinePayloadFact[];
  readonly matchArms: readonly RustMatchArmFact[];
  readonly constants: readonly RustConstIdentityFact[];
  readonly includes: readonly RustIncludeFact[];
  readonly testModules: readonly RustModuleFact[];
}

export interface RustVirtualSourceFact {
  readonly path: string;
  readonly facts: RustStructuralFacts;
}

export interface RustTestModuleFact {
  readonly name: string;
  readonly modulePath: readonly string[];
  readonly mountBase: readonly string[];
  readonly pathTarget: string | null;
  readonly configuration: "enabled" | "disabled" | "ambiguous";
}

export interface RustRunnableTestFact {
  readonly name: string;
  readonly modulePath: readonly string[];
}

export interface RustTestFacts {
  readonly schemaVersion: 1;
  readonly runnableTests: readonly RustRunnableTestFact[];
  readonly mountedModules: readonly RustTestModuleFact[];
}

/** 🗨️ Exact unescaped message argument owned by a standard Rust assertion macro. */
export interface RustAssertionMessageSpan {
  readonly macroName: string;
  readonly start: number;
  readonly end: number;
  readonly value: string;
}
