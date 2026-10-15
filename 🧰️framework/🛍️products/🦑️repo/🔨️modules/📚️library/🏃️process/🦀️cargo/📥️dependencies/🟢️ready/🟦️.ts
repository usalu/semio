import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
export interface CargoRegistryReadyResolutionV1 {readonly version:1;readonly workspaceManifest:string;readonly lock:string;readonly members:readonly string[];readonly packages:readonly Readonly<{id:string;manifest:string;source:string|null}>[];readonly resolved:readonly string[]}
/** 🛤️ Requires exact repository-relative input names before physical ready facts are observed. */
export function cargoRegistryReadyPathV1(value:string):boolean{return Boolean(value)&&value.length<=256&&!/[:\\\u0000-\u001f\u007f]/u.test(value)&&!value.startsWith("/")&&value.split("/").every(part=>Boolean(part)&&part!=="."&&part!=="..");}
/** 🧾️ Classifies only a complete first-party dependency resolution for shared ready admission. */
export function cargoRegistryReadyResolutionV1(value:unknown):boolean{
 if(validateJsonSchemaSubset(schema.definitions.Resolution,value,schema).length)return false;const row=value as CargoRegistryReadyResolutionV1;
 if(!cargoRegistryReadyPathV1(row.workspaceManifest)||!cargoRegistryReadyPathV1(row.lock)||row.members.some(value=>/[?*\[\]{}()!\\\u0000-\u001f\u007f]/u.test(value)))return false;
 const ids=new Set(row.packages.map(value=>value.id)),manifests=new Set(row.packages.map(value=>value.manifest));
 return ids.size===row.packages.length&&manifests.size===row.packages.length&&row.packages.every(value=>value.source===null&&cargoRegistryReadyPathV1(value.manifest)&&row.resolved.includes(value.id))&&row.resolved.every(value=>ids.has(value));
}
