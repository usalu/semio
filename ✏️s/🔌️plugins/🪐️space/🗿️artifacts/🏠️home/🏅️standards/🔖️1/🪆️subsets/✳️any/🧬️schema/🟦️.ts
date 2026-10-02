/** 🏠️ Home owns its schema and exact unsigned64 catalog generation. */
import {parseHomeCatalogGeneration} from "./🔢️catalog-generation/🟦️.ts";
export interface SHomeArtifact{
 /** 🧬️ @state artifact */
 schema:string;
 /** 🔢️ @state artifact */
 catalogGeneration:bigint;
}
/** 🚪️ Admit the actual persisted Home field domains. */
export function parseSHomeArtifact(value:unknown,at="$"):SHomeArtifact{
 if(value===null||typeof value!=="object"||Array.isArray(value))throw Error(at+": Home artifact is not an object");const row=value as Record<string,unknown>;
 if(typeof row.schema!=="string")throw Error(at+".schema: Home schema is not a string");return{schema:row.schema,catalogGeneration:parseHomeCatalogGeneration(row.catalogGeneration,at+".catalogGeneration")};
}
