import {createHash} from "node:crypto";
export type RustBuildFallbackAdapterV1="foreign-source"|"binary"|"proc-macro"|"build-script"|"build-script-run"|"effective-profile";
const fallbackAdapters:readonly RustBuildFallbackAdapterV1[]=["foreign-source","binary","proc-macro","build-script","build-script-run","effective-profile"];
export interface RustBuildIntentV1 {
 version:1;
 manifest:string;
 package:string;
 scope:"package"|"library"|"binary";
 binary?:string;
 sourcesRecord?:string;
 features?:string[];
 defaultFeatures?:boolean;
 target?:string;
 fallback?:{kind:"bounded-cargo";adapters:RustBuildFallbackAdapterV1[]};
}

/** 🎟️ Admits bounded declarative compiler selection before any Cargo preparation or graph graft. */
export function readRustBuildIntentV1(value:unknown):RustBuildIntentV1{
 const fail=():never=>{throw Error("Invalid selected Cargo build intent");};
 if(!value||typeof value!=="object"||Array.isArray(value))return fail();const row=value as Record<string,unknown>,allowed=new Set(["version","manifest","package","scope","binary","sourcesRecord","features","defaultFeatures","target","fallback"]),name=(value:unknown)=>typeof value==="string"&&value.length>=1&&value.length<=128&&/^[A-Za-z0-9_][A-Za-z0-9_-]*$/u.test(value)&&!/[\u0000-\u0020]/u.test(value);
 if(Object.keys(row).some(key=>!allowed.has(key))||row.version!==1||typeof row.manifest!=="string"||row.manifest.length<10||row.manifest.length>256||/^(?:\/|[A-Za-z]:)/u.test(row.manifest)||/[\\:\u0000-\u0020]/u.test(row.manifest)||row.manifest.split("/").some(part=>!part||part==="."||part==="..")||row.manifest.split("/").at(-1)!=="Cargo.toml"||!name(row.package)||!["package","library","binary"].includes(row.scope as string))return fail();
 if(row.scope==="binary"?!name(row.binary):Object.hasOwn(row,"binary"))return fail();
 if(Object.hasOwn(row,"sourcesRecord")&&(row.scope!=="binary"||typeof row.sourcesRecord!=="string"||row.sourcesRecord.length>160||!/^[A-Za-z0-9_][A-Za-z0-9_-]*\.sources\.json$/u.test(row.sourcesRecord)))return fail();
 if(Object.hasOwn(row,"target")&&!name(row.target)||Object.hasOwn(row,"defaultFeatures")&&typeof row.defaultFeatures!=="boolean")return fail();
 if(Object.hasOwn(row,"features")&&(!Array.isArray(row.features)||row.features.length<1||row.features.length>128||new Set(row.features).size!==row.features.length||row.features.some(feature=>typeof feature!=="string"||feature.length<1||feature.length>128||!/^[A-Za-z0-9_][A-Za-z0-9_./?-]*$/u.test(feature)||/[\u0000-\u0020]/u.test(feature))))return fail();
 if(Object.hasOwn(row,"fallback")){const fallback=row.fallback as Record<string,unknown>;if(!fallback||typeof fallback!=="object"||Array.isArray(fallback)||Object.keys(fallback).sort().join(",")!=="adapters,kind"||fallback.kind!=="bounded-cargo"||!Array.isArray(fallback.adapters)||fallback.adapters.length<1||fallback.adapters.length>fallbackAdapters.length||new Set(fallback.adapters).size!==fallback.adapters.length||fallback.adapters.some(adapter=>!fallbackAdapters.includes(adapter)))return fail();}
 return {version:1,manifest:row.manifest,package:row.package as string,scope:row.scope as RustBuildIntentV1["scope"],...(Object.hasOwn(row,"binary")?{binary:row.binary as string}:{}),...(Object.hasOwn(row,"sourcesRecord")?{sourcesRecord:row.sourcesRecord as string}:{}),...(Object.hasOwn(row,"features")?{features:[...row.features as string[]]}:{}),...(Object.hasOwn(row,"defaultFeatures")?{defaultFeatures:row.defaultFeatures as boolean}:{}),...(Object.hasOwn(row,"target")?{target:row.target as string}:{}),...(Object.hasOwn(row,"fallback")?{fallback:{kind:"bounded-cargo" as const,adapters:[...(row.fallback as {adapters:RustBuildFallbackAdapterV1[]}).adapters]}}:{})};
}

/** 📤️ Projects admitted package, deliverable, feature and triple facts without granting Cargo storage controls. */
export function rustBuildIntentArgumentsV1(value:RustBuildIntentV1):string[]{
 const intent=readRustBuildIntentV1(value);
 return ["-p",intent.package,...(intent.scope==="library"?["--lib"]:intent.scope==="binary"?["--bin",intent.binary!]:[]),...(intent.features?["--features",intent.features.join(",")]:[]),...(intent.defaultFeatures===false?["--no-default-features"]:[]),...(intent.target?["--target",intent.target]:[])];
}

export interface RustBuildFallbackDecisionV1 {version:1;intentDigest:string;adapters:RustBuildFallbackAdapterV1[]}

/** 🛂️ Grants an uncached finite Cargo route only for every expressly declared missing compiler adapter. */
export function qualifyRustBuildFallbackV1(value:RustBuildIntentV1,unsupported:readonly {unit:string;adapters:readonly string[]}[]):RustBuildFallbackDecisionV1|undefined{
 if(!unsupported.length)return;
 if(!value.fallback)throw Error("Selected Rust bounded fallback was not declared");
 const intent=readRustBuildIntentV1(value),allowed=intent.fallback!.adapters,adapters=new Set<RustBuildFallbackAdapterV1>();
 for(const row of unsupported){if(!row.unit||!Array.isArray(row.adapters)||row.adapters.length<1||row.adapters.length>fallbackAdapters.length||new Set(row.adapters).size!==row.adapters.length)throw Error("Typed Rust bounded fallback adapters required");for(const adapter of row.adapters){if(!allowed.includes(adapter as RustBuildFallbackAdapterV1))throw Error("Selected Rust bounded fallback adapter refused: "+adapter+" (unit "+row.unit+")");adapters.add(adapter as RustBuildFallbackAdapterV1);}}
 return {version:1,intentDigest:createHash("sha256").update(JSON.stringify(intent)).digest("hex"),adapters:[...adapters].sort()};
}

/** 🔏️ Rejects stale or broadened graph-issued fallback decisions before native compiler storage is opened. */
export function readRustBuildFallbackDecisionV1(value:unknown,intent:RustBuildIntentV1):RustBuildFallbackDecisionV1{
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join(",")!=="adapters,intentDigest,version")throw Error("Closed Rust bounded fallback decision required");const row=value as RustBuildFallbackDecisionV1;if(row.version!==1||typeof row.intentDigest!=="string"||!Array.isArray(row.adapters))throw Error("Closed Rust bounded fallback decision required");const expected=qualifyRustBuildFallbackV1(intent,[{unit:"selected",adapters:row.adapters}]);if(!expected||expected.intentDigest!==row.intentDigest||JSON.stringify(expected.adapters)!==JSON.stringify(row.adapters))throw Error("Original Rust bounded fallback intent identity changed");return expected;
}
