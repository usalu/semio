import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {readScriptProcessJsonV1} from "@semio-tech/process/routing/invocation/process/json";
const admission=structuredClone(schema),policy=Object.freeze({...admission.definitions.Policy.const});
export const VITE_RUNTIME_CHANNELS_ENV_V1=policy.environment;
export type ViteRuntimeChannelV1=Readonly<{name:string;endpoint:string;secret:string}>;
/** 📨️ Captures finite service-channel DATA; no arbitrary environment field or runtime authority is issued. */
export function readViteRuntimeChannelsV1(value:unknown):readonly ViteRuntimeChannelV1[]{const data=readScriptProcessJsonV1({channels:value}).channels;if(!Array.isArray(data)||validateJsonSchemaSubset(admission,data,admission).length||Buffer.byteLength(JSON.stringify(data))>policy.maximumBytes)throw Error("Closed finite Vite runtime channels required");const rows=data.map((value:unknown)=>{if(!value||typeof value!=="object"||!("name"in value)||!("endpoint"in value)||!("secret"in value)||typeof value.name!=="string"||typeof value.endpoint!=="string"||typeof value.secret!=="string")throw Error("Captured Vite runtime channel fields required");return Object.freeze({name:value.name,endpoint:value.endpoint,secret:value.secret});});if(new Set(rows.map(row=>row.name)).size!==rows.length)throw Error("Unique Vite runtime channel names required");return Object.freeze(rows);}
/** 🌐️ Emits only the one authored runtime transport variable, after semantic compiler identity. */
export function viteRuntimeEnvironmentV1(value:readonly ViteRuntimeChannelV1[]):Readonly<Record<string,string>>{return Object.freeze({[policy.environment]:JSON.stringify(readViteRuntimeChannelsV1(value))});}

/** 🧹️ Removes only the authored transport variable from build/compiler environments. */
export function withoutViteRuntimeEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,platform:string):Readonly<Record<string,string|undefined>>{const key=(value:string)=>platform==="win32"?value.toUpperCase():value;return Object.freeze(Object.fromEntries(Object.entries(environment).filter(([name])=>key(name)!==key(policy.environment))));}
