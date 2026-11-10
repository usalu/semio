import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import registry from "../📋️registry/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {BackendRowV1} from "../🟦️.ts";

/** 📜️ Reads the authored registry once and refuses a row set that breaks its schema, repeats an id or names a missing default. */
export function readBackendRegistryV1(value:unknown=registry):readonly BackendRowV1[]{
 const issues=validateJsonSchemaSubset(schema,value,schema);
 if(issues.length)throw Error(`Closed build backend registry required: ${issues.slice(0,3).join("; ")}`);
 const rows=(value as {backends:BackendRowV1[]}).backends,ids=rows.map(row=>row.id);
 if(new Set(ids).size!==ids.length)throw Error("Unique build backend ids required");
 if(rows.filter(row=>row.default===true).length!==1)throw Error("Exactly one default build backend required");
 return Object.freeze(rows.map(row=>Object.freeze(row)));
}
