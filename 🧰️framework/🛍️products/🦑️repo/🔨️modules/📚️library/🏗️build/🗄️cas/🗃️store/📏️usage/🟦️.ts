import {lstat,readdir} from "node:fs/promises";
import {join} from "node:path";

/** 🔢️ Counts objects and bytes of a store root without reading file contents; used by status. */
export async function casUsageV1(root:string):Promise<Readonly<{objects:number;bytes:number}>>{
 let objects=0,bytes=0;const base=join(root,"v1","o");let shards:string[];try{shards=await readdir(base);}catch{return{objects,bytes};}
 for(const shardName of shards){
  let names:string[];try{names=await readdir(join(base,shardName));}catch{continue;}
  for(const name of names)try{const info=await lstat(join(base,shardName,name));objects++;bytes+=info.size;}catch{}
 }
 return{objects,bytes};
}
