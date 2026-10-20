import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "../../../../../../📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
export type NativeHostV1=Readonly<{landingAppId:string;hostAppId:string}>;
export type NativePluginV1=Readonly<{pluginId:string;cratePath:string;wasmOut:string;role:"plugin"|"extension";extends:string|null;capabilities:readonly string[];activationEvents:readonly string[];host:NativeHostV1|null}>;
export type NativePlaygroundV1=Readonly<{variant:string;pluginId:string;app:string|null;aliases:readonly string[]}>;
export type NativeCatalogV1=Readonly<{version:1;plugins:readonly NativePluginV1[];playgrounds:readonly NativePlaygroundV1[]}>;
/** 📥️ No contributed native catalog owns an empty roster. */
export function emptyNativeCatalogV1():NativeCatalogV1{return {version:1,plugins:[],playgrounds:[]};}
/** 🔐️ Admits the schema and every referenced owner before booting a renderer. */
export function readNativeCatalogV1(value:unknown):NativeCatalogV1{
 if(new TextEncoder().encode(JSON.stringify(value)).byteLength>8388608)throw Error("Native catalog exceeds its byte authority");
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Native catalog schema refused");
 const catalog=value as NativeCatalogV1,ids=new Set<string>(),variants=new Set<string>(),claims=new Set<string>();
 for(const row of catalog.plugins){if([row.pluginId,row.cratePath,row.wasmOut].some(value=>value.includes("\0"))||ids.has(row.pluginId)||row.cratePath.split(/[\\/]/u).includes("..")||row.wasmOut.split(/[\\/]/u).includes(".."))throw Error("Native plugin identity or artifact path refused");ids.add(row.pluginId);for(const event of row.activationEvents)if(event.startsWith("on-artifact-kind:")){const kind=event.slice(17);if(!kind||claims.has(kind))throw Error("Native activation ownership refused");claims.add(kind);}}
 for(const row of catalog.plugins)if(row.extends!==null&&!ids.has(row.extends)||row.role==="extension"&&row.extends===null)throw Error("Native extension parent refused");
 for(const row of catalog.playgrounds){if(!ids.has(row.pluginId))throw Error("Native playground owner refused");for(const variant of [row.variant,...row.aliases]){if(variants.has(variant))throw Error("Native playground alias refused");variants.add(variant);}}
 return catalog;
}
