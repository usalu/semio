import{compileJsonSchemaSubset}from"../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import sourceSchema from"../🧬️schema/🔣️.json"with{type:"json"};
const contract=compileJsonSchemaSubset(schema.definitions.Path),parallel=schema.definitions.Policy.const.maximumParallelOperations;

/** 📚️ Drains each bounded parent layer and every accepted operation before completion or failure. */
export async function forEachNxSdkSourceLayerV1<T>(rows:readonly T[],path:(row:T)=>string,perform:(row:T)=>Promise<void>):Promise<void>{
 if(rows.length>sourceSchema.definitions.Policy.const.maximumEntries||!Number.isSafeInteger(parallel)||parallel<1)throw Error("Bounded SDK source layer required");const layers=new Map<number,T[]>(),seen=new Set<string>();
 for(const row of rows){const key=path(row);if(contract.validate(key).length||key.length>sourceSchema.definitions.Policy.const.maximumPath||seen.has(key))throw Error("Unique closed SDK source path required");seen.add(key);const depth=key===""?0:key.split("/").length,layer=layers.get(depth)??[];layer.push(row);layers.set(depth,layer);}
 for(const[,layer]of[...layers].sort(([a],[b])=>a-b)){let cursor=0,failed=false,failure:unknown;const worker=async()=>{while(!failed&&cursor<layer.length){const row=layer[cursor++]!;try{await perform(row);}catch(error){if(!failed){failed=true;failure=error;}}}};await Promise.all(Array.from({length:Math.min(parallel,layer.length)},worker));if(failed)throw failure;}
}
