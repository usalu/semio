import{readNativeArtifactCachePolicyV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/💾️cache/⚙️policy/🟦️.ts";
import rawPolicy from"../🔣️policy.json";
import rawSchema from"../🧬️schema/🔣️.json";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const schema=structuredClone(rawSchema),captured=structuredClone(rawPolicy);
if(validateJsonSchemaSubset(schema.definitions.Policy,captured,schema).length)throw Error("Closed authored compiler state policy required");
/** 🔐️ Retains only privately captured primitive policy facts, independent of public imported JSON objects. */
export const compilerStatePolicyV1=readNativeArtifactCachePolicyV1(captured);
/** 🛂️ Validates bounded owner data against the privately captured authored contract. */
export function compilerStateErrorsV1(definition:keyof typeof schema.definitions,value:unknown):readonly string[]{return validateJsonSchemaSubset(schema.definitions[definition],value,schema);}

/** 🌱️ Captures the closed direct-compiler environment admission contract without exporting mutable sets. */
export const compilerStateEnvironmentPolicyV1=Object.freeze({...schema.definitions.EnvironmentPolicy.const,machine:Object.freeze([...schema.definitions.EnvironmentPolicy.const.machine]),owned:Object.freeze([...schema.definitions.EnvironmentPolicy.const.owned])});
