/** 📋️ Contract: module exporting `STYLING_PYTHON_POLICY_V1`, `StylingPythonRequestV1`, `readStylingPythonRequestV1`, `stylingPythonOutputV1` and more. */
import{types}from"node:util";
import{isAbsolute,join,resolve}from"node:path";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schema from"../🧬️schema/🔣️.json";
const requestSchema=structuredClone(schema.definitions.Request),wheelSchema=structuredClone(schema.definitions.WheelName);
export const STYLING_PYTHON_POLICY_V1=Object.freeze({...schema.definitions.Policy.const,environmentKeys:Object.freeze([...schema.definitions.Policy.const.environmentKeys])});
/** 🐍️ Defines only the authored Python operation and actual build intent. */
export type StylingPythonRequestV1=Readonly<{version:1;operation:"deps"|"build"|"test";mode:"prototype"|"release"}>;
/** 🧬️ Admits immutable DATA without creating any compiler or storage authority. */
export function readStylingPythonRequestV1(value:unknown):StylingPythonRequestV1{if(value===null||typeof value!=="object"||Array.isArray(value)||types.isProxy(value))throw Error("Closed styling Python request required");const fields=Object.getOwnPropertyDescriptors(value);if(Object.keys(fields).length!==3)throw Error("Closed styling Python request required");const version=fields.version,operation=fields.operation,mode=fields.mode;if(!version||!operation||!mode||!("value"in version)||!("value"in operation)||!("value"in mode)||version.value!==1||operation.value!=="deps"&&operation.value!=="build"&&operation.value!=="test"||mode.value!=="prototype"&&mode.value!=="release")throw Error("Closed styling Python request required");const captured:StylingPythonRequestV1=Object.freeze({version:1,operation:operation.value,mode:mode.value});if(validateJsonSchemaSubset(requestSchema,captured).length)throw Error("Closed styling Python request required");return captured;}
/** 📦️ Projects both intents to the one actual products publication owner. */
export function stylingPythonOutputV1(root:string,mode:StylingPythonRequestV1["mode"]):string{readStylingPythonRequestV1({version:1,operation:"build",mode});if(!isAbsolute(root)||resolve(root)!==root)throw Error("Absolute products root DATA required");return join(root,STYLING_PYTHON_POLICY_V1.outputNamespace,mode);}
/** 🛡️ Closes inherited tool configuration before any state or cache wait. */
export function stylingPythonEnvironmentV1(value:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string|undefined>>{const output:Record<string,string|undefined>={};for(const[key,item]of Object.entries(value)){const normalized=process.platform==="win32"?key.toUpperCase():key;if(!STYLING_PYTHON_POLICY_V1.environmentKeys.includes(normalized))continue;if(item!==undefined&&typeof item!=="string")throw Error("Primitive Python environment DATA required");if(output[normalized]!==undefined&&output[normalized]!==item)throw Error("Conflicting Python environment keys required");output[normalized]=item;}return Object.freeze(output);}
/** 🎡️ Refuses ambiguous, escaping or oversized wheel filenames. */
export function requireStylingPythonWheelNameV1(name:string):string{if(validateJsonSchemaSubset(wheelSchema,name).length||Buffer.byteLength(name)>STYLING_PYTHON_POLICY_V1.maximumWheelNameBytes)throw Error("One bounded portable Python wheel name required");return name;}
