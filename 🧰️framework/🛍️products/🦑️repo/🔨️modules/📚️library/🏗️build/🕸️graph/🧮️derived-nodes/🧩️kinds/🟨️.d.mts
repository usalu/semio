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
export function verifyObservedV1(ref:{rows:{dir:string;names:string[]}},options:{environment:{readdir(dir:string):Promise<readonly string[]>|readonly string[]}}):Promise<boolean>;
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
