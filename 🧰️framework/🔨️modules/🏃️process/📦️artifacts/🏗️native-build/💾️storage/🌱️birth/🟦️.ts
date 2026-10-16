import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import exportedSchema from"./🧬️schema/🔣️.json";
const schema=JSON.parse(JSON.stringify(exportedSchema))as typeof exportedSchema,protocol=Object.freeze({...schema.definitions.Protocol.const,stages:Object.freeze([...schema.definitions.Protocol.const.stages]),paths:Object.freeze(schema.definitions.Protocol.const.paths.map(path=>Object.freeze([...path])))});
/** 📋️ Records finite birth order as DATA, never issuing Native or cleanup ownership. */
export type NativeStorageBirthStateV1=Readonly<{version:1;stage:string|null;accepted:number}>;
/** 🧬️ Captures immutable protocol primitives independently of mutable exported schema DATA. */
export function nativeStorageBirthProtocolV1():typeof protocol{return protocol;}
/** 🌱️ No physical effect or ownership is implied by the empty protocol projection. */
export function nativeStorageBirthStateV1():NativeStorageBirthStateV1{return Object.freeze({version:1,stage:null,accepted:0});}
/** 🔗️ Every parent and child obligation must follow its exact prior protocol stage. */
export function nextNativeStorageBirthStateV1(value:NativeStorageBirthStateV1,stage:string):NativeStorageBirthStateV1{if(validateJsonSchemaSubset(schema,value,schema).length||value.accepted>=protocol.maximumStages||!protocol.paths.some(path=>(value.accepted===0?value.stage===null:path[value.accepted-1]===value.stage)&&path[value.accepted]===stage))throw Error("Exact finite Native birth order required");return Object.freeze({version:1,stage,accepted:value.accepted+1});}
