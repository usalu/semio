/** 🔺️ Sparse Home fields retain their exact canonical domains. */
import {parseHomeCatalogGeneration} from "../🔢️catalog-generation/🟦️.ts";
export interface SHomeDiff{schema?:string;catalogGeneration?:bigint}
/** 🚪️ Validate only fields present in a sparse Home change. */
export function parseSHomeDiff(value:unknown,at="$"):SHomeDiff{
 if(value===null||typeof value!=="object"||Array.isArray(value))throw Error(at+": Home diff is not an object");const row=value as Record<string,unknown>;
 if(row.schema!==undefined&&typeof row.schema!=="string")throw Error(at+".schema: Home schema is not a string");return{schema:row.schema as string|undefined,catalogGeneration:row.catalogGeneration===undefined?undefined:parseHomeCatalogGeneration(row.catalogGeneration,at+".catalogGeneration")};
}
