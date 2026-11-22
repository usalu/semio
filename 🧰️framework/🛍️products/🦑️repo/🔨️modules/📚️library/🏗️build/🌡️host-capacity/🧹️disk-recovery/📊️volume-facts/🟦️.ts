import {existsSync,readdirSync,statSync,statfsSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📊️ The authored layout of the volume facts: the partition folder, its mount name, the nested mounts and the header line. */
export const VOLUME_FACTS_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
/** 📋️ One volume: its path and its size and free bytes. */
export type VolumeFactV1=Readonly<{path:string;totalBytes:number;freeBytes:number}>;

function rowOf(path:string):VolumeFactV1|undefined{
 try{const space=statfsSync(path);return{path,totalBytes:Number(space.blocks)*Number(space.bsize),freeBytes:Number(space.bavail)*Number(space.bsize)};}catch{return undefined;}
}

/** 📖️ Reads the host volume of the workspace and every mount below `.cache/b/*` (partition images and the nested native Nx shards of the nx partition); a folder that is no own volume is left out. */
export function readVolumeFactsV1(workspace:string,policy=VOLUME_FACTS_POLICY_V1):readonly VolumeFactV1[]{
 const rows:VolumeFactV1[]=[],host=rowOf(workspace);
 if(host!==undefined)rows.push(host);
 const base=join(workspace,...policy.partitions.split("/"));
 if(!existsSync(base))return rows;
 const own=(path:string,parent:number)=>{try{return statSync(path).dev!==parent;}catch{return false;}};
 for(const name of readdirSync(base).sort()){
  const mount=join(base,name,policy.mount);
  if(!own(mount,statSync(base).dev))continue;
  const row=rowOf(mount);if(row!==undefined)rows.push(row);
  for(const child of readdirSync(mount).sort())for(const nested of policy.nested){
   const path=join(mount,child,...nested.split("/"));
   if(own(path,statSync(mount).dev)){const found=rowOf(path);if(found!==undefined)rows.push(found);}
  }
 }
 return rows.slice(0,policy.maximumRows);
}

/** 🧾️ The lines that tell where the space is: the header, then one line per volume with free and total megabytes. Pure. */
export function formatVolumeFactsV1(rows:readonly VolumeFactV1[],policy=VOLUME_FACTS_POLICY_V1):string{
 return[policy.header,...rows.map(row=>`${row.path}: ${Math.floor(row.freeBytes/1048576)} of ${Math.floor(row.totalBytes/1048576)}`)].join("\n");
}

/** 🩺️ The facts of this machine as text; empty when nothing can be read, never throws. */
export function volumeFactsTextV1(workspace:string):string{
 try{return formatVolumeFactsV1(readVolumeFactsV1(workspace));}catch{return"";}
}
