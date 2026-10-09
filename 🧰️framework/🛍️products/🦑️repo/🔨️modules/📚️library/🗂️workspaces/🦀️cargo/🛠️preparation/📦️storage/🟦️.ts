import schema from "./🧬️schema/🔣️.json";
import {join,isAbsolute} from "node:path";

export type CargoPreparationStorageV1=Readonly<{version:1;directory:string}>;
/** 📦️ Admits explicit absolute storage with capacity for the unchanged queued-lease suffix. */
export function parseCargoPreparationStorageV1(value:unknown):CargoPreparationStorageV1 {
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join(",")!=="directory,version")throw Error("Preparation storage required");const row=value as Record<string,unknown>,directory=row.directory;if(row.version!==1||typeof directory!=="string"||!directory.length||directory.length>schema.properties.directory.maxLength||!new RegExp(schema.properties.directory.pattern).test(directory)||directory.includes("\0"))throw Error("Preparation storage capacity refused");return{version:1,directory};
}
/** 🏗️ Authors ordinary repository command storage under its selected repository root. */
export function repositoryCargoPreparationStorageV1(root:string):CargoPreparationStorageV1 {if(!isAbsolute(root))throw Error("Repository storage root must be absolute");return parseCargoPreparationStorageV1({version:1,directory:join(root,".🧬semio/🦑️repo/⚡️cache/cargo-preparation")});}
/** 🚪️ Receives the exact serialized storage object from the owning preparation launch. */
export function cargoPreparationStorageFromEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):CargoPreparationStorageV1 {const value=environment.SEMIO_CARGO_PREPARATION_STORAGE;if(typeof value!=="string")throw Error("Preparation launch lacks mandatory storage");return parseCargoPreparationStorageV1(JSON.parse(value));}
