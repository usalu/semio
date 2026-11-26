/** ✳️ The regular expression of one glob. */
export function globRegexV1(pattern:string):RegExp;
/** ✳️ A matcher of relative paths for a pattern list. */
export function globMatcherV1(patterns:readonly string[]):(path:string)=>boolean;
/** 📜 The restricted TOML subset Cargo manifests use. */
export function parseTomlSubsetV1(text:string):Record<string,unknown>;
/** 🦀 The parsed dependency set of a Cargo.toml. */
export function cargoDependenciesV1(text:string):Record<string,unknown>;
/** 📦 The parsed dependency set of a package.json. */
export function packageDependenciesV1(text:string):Record<string,unknown>;
/** 📋 The parsed declaration of a project.json. */
export function projectDeclarationV1(text:string):Record<string,unknown>;
/** 🐹 The parsed dependency set of a go.mod. */
export function goDependenciesV1(text:string):Record<string,unknown>;
/** 🧾 The parsed dependency set of one manifest by its file name. */
export function parseManifestV1(path:string,text:string):Record<string,unknown>;
/** 🔎 Re-checks an observation recorded by the `directory-names` rule. */
export function verifyObservedV1(ref:import("../🟨️.mjs").RowRefV1,options:{signal?:AbortSignal;environment?:any}):Promise<boolean>;
/** 🧩 The rule of every node kind. */
export const KIND_RULES_V1:Readonly<Record<string,(ctx:any)=>Promise<unknown>>>;
/** 🧭 The compiled form of a pattern list below a root. */
export function globSpecV1(root:string,patterns:readonly unknown[]):{groups:{base:string;rest:string}[];bases:string[];key:string;has(path:string):boolean};
/** #️⃣ The additive multiset hash of one entry. */
export function entryHashV1(path:string,digest:string):bigint;
/** #️⃣ The additive multiset hash of one roster member. */
export function rosterHashV1(path:string):bigint;
/** #️⃣ A sum modulo 2^256 as 64 hex digits. */
export function hexOfV1(value:bigint):string;

/** 🏗️ The configuration files of one project root, in the order the assembly reads them. */
export const PROJECT_CONFIG_FILES_V1: readonly ["📋️project.json","project.json","package.json","Cargo.toml","go.mod"];
/** 🏗️ The glob patterns that list the project roots of a workspace. */
export const PROJECT_ROSTER_PATTERNS_V1: readonly string[];
/** 📋️ One observed project configuration file: unreadable, or parsed as JSON, TOML or the go.mod dependencies. */
export type ProjectConfigEntryV1=Readonly<{ok:false}|{ok:true;json?:Readonly<Record<string,unknown>>;toml?:Readonly<{package?:Readonly<{name?:unknown}>}&Record<string,unknown>>;go?:Readonly<Record<string,unknown>>}>;
/** 🏗️ The name and availability of the project of one root, as inference decides them: a root with an authored project file is named by that file (null when it declares no name, unavailable when it does not parse); every other root with a Cargo manifest is named by its package (null without one, unavailable when the manifest does not parse). */
export function projectOfConfigV1(config: Readonly<{files:Readonly<Record<string,ProjectConfigEntryV1|null|undefined>>}>, hasProjectFile: boolean): Readonly<{name:string|null;unavailable:boolean}>;
