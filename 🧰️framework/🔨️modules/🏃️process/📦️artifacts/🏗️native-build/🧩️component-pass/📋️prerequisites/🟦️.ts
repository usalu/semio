import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
const policy=structuredClone(schema);
/** 🗺️ Orders finite job-only prerequisites in declaration-stable dependency waves before retained pass planning. */
export function nativeBuildPrerequisiteOrderV1(jobs:readonly Readonly<{id:string;after:readonly string[]}>[]):readonly string[]{
 const rows=jobs.map(({id,after})=>({id,after:[...after]}));if(validateJsonSchemaSubset(policy,rows).length)throw Error("Finite job-only prerequisite graph required");const ids=new Set(rows.map(row=>row.id));if(ids.size!==rows.length||rows.some(row=>row.after.some(dependency=>!ids.has(dependency.slice(4)))))throw Error("Exact unique prerequisite job dependencies required");const remaining=new Map(rows.map(row=>[row.id,new Set(row.after.map(value=>value.slice(4)))])),ordered:string[]=[];
 while(remaining.size){const ready=rows.filter(row=>remaining.get(row.id)?.size===0);if(!ready.length)throw Error("Acyclic prerequisite job graph required");for(const row of ready){remaining.delete(row.id);ordered.push(row.id);}for(const dependencies of remaining.values())for(const row of ready)dependencies.delete(row.id);}
 return Object.freeze(ordered);
}
