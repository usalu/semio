import{existsSync,mkdirSync,readdirSync,readFileSync,rmSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{ownerPoolAliveV1,ownerPoolProcessStartV1}from"../🪦️reaper/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧹️ The numbers of the staging sweep (see `./🧬️schema/🔣️.json`). */
export const OWNER_POOL_STAGING_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
const P=OWNER_POOL_STAGING_POLICY_V1;

export type OwnerPoolStagingOwnerV1=Readonly<{version:1;pid:number;startTime?:string}>;
export type OwnerPoolStagingIoV1=Readonly<{
 list(parent:string):readonly string[];
 owner(directory:string):OwnerPoolStagingOwnerV1|undefined;
 ageMilliseconds(directory:string):number|undefined;
 alive(owner:OwnerPoolStagingOwnerV1):boolean;
 remove(directory:string):void;
}>;

/** 🧹️ Removes the staging directories of owner-pool processes that are gone from `parents`, the directory of the caller never: a directory with an owner file stays while that process lives with the same start time, a directory without one stays while it is younger than `graceMilliseconds`. Returns what it removed (a removal that fails is left for the next sweep). The count of directories one machine holds is therefore bounded by its live pool processes, not by its tries.
 * @see ./🧪️tests/🟦️.ts */
export function ownerPoolStagingSweepV1(input:Readonly<{parents:readonly string[];own:string;io:OwnerPoolStagingIoV1}>):readonly string[]{
 const removed:string[]=[];
 for(const parent of input.parents){
  let names:readonly string[]=[];
  try{names=input.io.list(parent);}catch{continue;}
  for(const name of names){
   if(!name.startsWith(P.prefix))continue;
   const directory=join(parent,name);
   if(directory===input.own)continue;
   const owner=input.io.owner(directory),age=input.io.ageMilliseconds(directory);
   const gone=owner===undefined?age!==undefined&&age>P.graceMilliseconds:!input.io.alive(owner);
   if(!gone)continue;
   try{input.io.remove(directory);removed.push(directory);}catch{}
  }
 }
 return removed;
}

/** 🏷️ Writes the owner file of this process into `directory` (its pid and start time), the first act of an owner after it made the directory. */
export function writeOwnerPoolStagingOwnerV1(directory:string,pid:number=process.pid,startTime:string|undefined=ownerPoolProcessStartV1(pid)):void{
 mkdirSync(directory,{recursive:true});
 writeFileSync(join(directory,P.ownerFile),JSON.stringify({version:1,pid,...startTime===undefined?{}:{startTime}}));
}

/** 🗑️ The real filesystem and process table for the sweep: the pid must live and, where the system reports a start time, have the recorded one. */
export const ownerPoolStagingIoV1:OwnerPoolStagingIoV1=Object.freeze({
 list:parent=>existsSync(parent)?readdirSync(parent):[],
 owner:directory=>{try{const row=JSON.parse(readFileSync(join(directory,P.ownerFile),"utf8")) as OwnerPoolStagingOwnerV1;return row.version===1&&Number.isInteger(row.pid)?row:undefined;}catch{return undefined;}},
 ageMilliseconds:directory=>{try{return Date.now()-statSync(directory).mtimeMs;}catch{return undefined;}},
 alive:owner=>{if(!ownerPoolAliveV1(owner.pid))return false;if(owner.startTime===undefined)return true;const now=ownerPoolProcessStartV1(owner.pid);return now===undefined||now===owner.startTime;},
 remove:directory=>rmSync(directory,{recursive:true,force:true}),
});
