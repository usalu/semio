import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {isAbsolute} from "node:path";

export type CargoPreparationStorageV1=Readonly<{version:1;directory:string;program:string}>;
/** 📦️ Admits explicit absolute storage with capacity for the unchanged queued-lease suffix. */
export function parseCargoPreparationStorageV1(value:unknown):CargoPreparationStorageV1 {
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join(",")!=="directory,program,version")throw Error("Preparation storage required");const row=value as Record<string,unknown>,directory=row.directory;if(row.version!==1||typeof directory!=="string"||!directory.length||directory.length>schema.properties.directory.maxLength||!new RegExp(schema.properties.directory.pattern).test(directory)||directory.includes("\0"))throw Error("Preparation storage capacity refused");const program=row.program;if(typeof program!=="string"||!isAbsolute(program)||program.length>schema.properties.program.maxLength||program.includes("\0"))throw Error("Preparation program required");return{version:1,directory,program};
}
/** 🚪️ Receives the exact serialized storage object from the owning preparation launch. */
export function cargoPreparationStorageFromEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):CargoPreparationStorageV1 {const value=environment.SEMIO_CARGO_PREPARATION_STORAGE;if(typeof value!=="string")throw Error("Preparation launch lacks mandatory storage");return parseCargoPreparationStorageV1(JSON.parse(value));}
