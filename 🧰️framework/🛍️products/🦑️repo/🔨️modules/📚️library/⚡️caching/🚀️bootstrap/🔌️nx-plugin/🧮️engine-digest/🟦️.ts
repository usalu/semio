import{refuseBuildV1}from"../../../../🏗️build/📖️texts/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import table from"./🔣️.json" with { type: "json" };

/** 🌱️ One file reference a module only uses to start a program, not to load its code. */
export interface NxEngineSpawnV1{readonly owner:string;readonly reference:string;readonly contract:{readonly en:string;readonly de:string}}

/** 🔖️ The declared task semantics: the revision number, the exact executor surface files every task is keyed by, and the directory prefixes of the Cargo argument builders that only the native family adds. See `./🔣️.json` and `./🧪️tests/🔖️task-semantics`. */
export interface NxEngineSemanticsV1{readonly revision:number;readonly contract:{readonly en:string;readonly de:string};readonly surface:readonly string[];readonly nativeSurface:readonly string[];readonly excludedSegments:readonly string[]}

/** 📚️ The closed policy of the Nx engine digest: the task semantics and the references the engine closure does not follow. */
export interface NxEnginePolicyV1{readonly version:2;readonly semantics:NxEngineSemanticsV1;readonly spawnOnly:readonly NxEngineSpawnV1[]}

/** 🧬️ The two engine families: `generic` keys every Nx command, `native` keys the commands that compile Rust and therefore also the Cargo argument builders. */
export type NxEngineFamilyV1="generic"|"native";

/** 🏷️ The engine families in the order their named inputs and digests are produced. */
export const nxEngineFamiliesV1:readonly NxEngineFamilyV1[]=Object.freeze(["generic","native"]);

let declared:NxEnginePolicyV1|undefined;

/** 🔐️ Admits a policy: closed by its schema, no duplicate or nested prefix, no surface file under a prefix twice, no parent segment; any other table is refused so nothing enters a task hash by accident. */
export function nxEnginePolicyV1(candidate:unknown=table):NxEnginePolicyV1{
 if(candidate===table&&declared)return declared;
 if(validateJsonSchemaSubset(schema,candidate).length)throw refuseBuildV1("invalid","nx-engine.table-required");
 const value=candidate as NxEnginePolicyV1,prefixes=value.semantics.nativeSurface;
 for(const name of [...value.semantics.surface,...prefixes])if(name.split("/").some(part=>part==="."||part===".."))throw refuseBuildV1("invalid","nx-engine.table-required");
 for(const [index,prefix] of prefixes.entries())if(prefixes.some((other,at)=>at!==index&&other.startsWith(prefix)))throw refuseBuildV1("invalid","nx-engine.table-required");
 if(value.semantics.surface.some(file=>prefixes.some(prefix=>file.startsWith(prefix))))throw refuseBuildV1("invalid","nx-engine.table-required");
 const frozen=Object.freeze(value);if(candidate===table)declared=frozen;return frozen;
}

/** ❓️ True when the reference as written in the owner module (workspace-relative) only starts a program, so the engine closure leaves it out. */
export function nxEngineSpawnOnlyV1(owner:string,reference:string,value:NxEnginePolicyV1=nxEnginePolicyV1()):boolean{return (value.spawnOnly??[]).some(row=>row.owner===owner&&row.reference===reference);}

/** 🚧️ Refuses a family name the engine does not know. */
export function nxEngineFamilyV1(name:unknown):NxEngineFamilyV1{
 if(name===undefined)return "generic";
 if(name==="generic"||name==="native")return name;
 throw refuseBuildV1("invalid","nx-engine.family-required",{family:String(name)});
}

/** 🎯️ The sorted workspace-relative files the digest of a family covers: the declared surface files and, for `native`, the closure files below a declared Cargo argument prefix outside the excluded segments. Every other file is in no task hash. */
export function nxEngineSurfaceV1(closureFiles:readonly string[],family:NxEngineFamilyV1="generic",value:NxEnginePolicyV1=nxEnginePolicyV1()):readonly string[]{
 const {surface,nativeSurface,excludedSegments}=value.semantics,files=new Set<string>(surface);
 if(family==="native")for(const file of closureFiles)if(nativeSurface.some(prefix=>file.startsWith(prefix))&&!file.split("/").some(part=>excludedSegments.includes(part)))files.add(file);
 return [...files].sort();
}

/** 🪪️ The rows the digest of a family carries beside the file bytes: the declared task semantics revision and the family name, so a bump or another family re-keys without a byte changing. */
export function nxEngineIdentityV1(family:NxEngineFamilyV1="generic",value:NxEnginePolicyV1=nxEnginePolicyV1()):readonly string[]{return [`semantics ${value.semantics.revision}`,`family ${family}`];}
