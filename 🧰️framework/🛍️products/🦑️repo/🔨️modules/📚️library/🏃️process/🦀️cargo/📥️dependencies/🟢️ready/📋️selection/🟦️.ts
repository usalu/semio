import {isAbsolute,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schemaDocument from "./🧬️schema/🔣️.json";
const schema=structuredClone(schemaDocument);
/** 🧾️ Identifies exact declared full-resolution DATA; this key cannot issue a registry reader. */
export function cargoRegistryReadSelectionV1(value:unknown):string{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed finite registry selection DATA required");const row=value as{manifest:string;cwd:string;offline:boolean};for(const path of[row.manifest,row.cwd])if(!isAbsolute(path)||resolve(path)!==path||path.includes("\0"))throw Error("Canonical registry selection DATA required");return JSON.stringify([row.manifest,row.cwd,row.offline]);}
