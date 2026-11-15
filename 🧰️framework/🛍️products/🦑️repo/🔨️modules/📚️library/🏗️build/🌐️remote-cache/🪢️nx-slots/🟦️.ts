import {readdirSync,rmSync,statSync} from "node:fs";
import {join} from "node:path";
import {classifySlotsV1,SLOT_GRACE_MILLISECONDS_V1,slotIdV1,type SlotRowV1} from "../../🪢️slots/🟦️.ts";

/** 🧶️ One Nx task as the Nx workspace database records it (`task_details`) with the times of its cache output (`cache_outputs`, UTC milliseconds since the epoch; `null` when it has none). */
export type NxTaskRowV1=Readonly<{hash:string;project:string;target:string;configuration:string|null;createdAt:number|null;accessedAt:number|null}>;

/** 🔌️ Reads the task rows of an Nx workspace database; the default reads `task_details` and `cache_outputs` read-only through the runtime's SQLite binding, and tests inject their own reader. */
export type NxTaskReaderV1=(databasePath:string)=>Promise<readonly NxTaskRowV1[]>;

/** 🪪️ The slot of one Nx task on one platform: project, target, configuration (empty when none) and the platform of the machine that ran it (`<os>-<arch>`), because Nx hashes may differ per platform and machines of two platforms must not supersede each other. */
export function nxTaskSlotV1(row:Pick<NxTaskRowV1,"project"|"target"|"configuration">,platform:string):string{return slotIdV1("nx-task",[row.project,row.target,row.configuration??"",platform]);}

/** 🖥️ The platform key of this machine. */
export const nxPlatformV1=():string=>`${process.platform}-${process.arch}`;

const parseTime=(value:unknown):number|null=>{
 if(typeof value==="number")return value;
 if(typeof value!=="string")return null;
 const at=Date.parse(value.includes("T")?value:value.replace(" ","T")+"Z");return Number.isFinite(at)?at:null;
};

/** 📖️ The default reader: `task_details` joined with `cache_outputs` of the database, opened read-only (a real Nx database in WAL mode without its `-shm` file after Nx exited cannot be opened read-only, so it is then opened for reading and writing, which creates the shared-memory file only). */
export const readNxTaskRowsV1:NxTaskReaderV1=async databasePath=>{
 const {Database}=await import("bun:sqlite"),sql="SELECT d.hash AS hash, d.project AS project, d.target AS target, d.configuration AS configuration, c.created_at AS created, c.accessed_at AS accessed FROM task_details d LEFT JOIN cache_outputs c ON c.hash = d.hash";
 type Row={hash:string;project:string;target:string;configuration:string|null;created:unknown;accessed:unknown};
 const read=(flags:{readonly:true}|{readwrite:true}):Row[]=>{const database=new Database(databasePath,flags);try{return database.query(sql).all() as Row[];}finally{database.close();}};
 let rows:Row[];try{rows=read({readonly:true});}catch{rows=read({readwrite:true});}
 return rows.map(row=>({hash:String(row.hash),project:row.project,target:row.target,configuration:row.configuration===""?null:row.configuration,createdAt:parseTime(row.created),accessedAt:parseTime(row.accessed)}));
};

/** 📤️ What an Nx cache pass did or would do: tasks with a cache entry, how many are current, retained within the grace and removed (hash list). */
export type NxSupersessionV1=Readonly<{dryRun:boolean;scanned:number;current:number;retained:number;removed:readonly string[];bytes:number}>;

const directoryBytes=(path:string):number=>{
 let bytes=0;
 try{for(const row of readdirSync(path,{withFileTypes:true})){const child=join(path,row.name);if(row.isDirectory())bytes+=directoryBytes(child);else if(row.isFile())bytes+=statSync(child).size;}}catch{}
 return bytes;
};

/** 🪢️ Drops the entries of the local Nx cache that a newer entry of the same task slot superseded beyond the grace (`<hash>` directory and `<hash>.commit` file of the cache directory), so the cache of a machine holds the current state of its tasks. A task counts as stored or claimed at the later of its output creation and last access, because a local hit proves the worktree needs the entry. Entries Nx lists without a directory and directories Nx does not list are left alone. `forget` receives the removed hashes so the caller can delete their `cache_outputs` rows. */
export async function supersedeNxCacheV1(options:Readonly<{cacheDirectory:string;rows:readonly NxTaskRowV1[];platform?:string;nowMs?:number;graceMilliseconds?:number;dryRun?:boolean;forget?:(hashes:readonly string[])=>Promise<void>}>):Promise<NxSupersessionV1>{
 const nowMs=options.nowMs??Date.now(),platform=options.platform??nxPlatformV1(),present=new Set<string>();
 try{for(const name of readdirSync(options.cacheDirectory))present.add(name.endsWith(".commit")?name.slice(0,-7):name);}catch{}
 const rows:SlotRowV1[]=[];
 for(const row of options.rows){
  if(!present.has(row.hash))continue;
  const stored=Math.max(row.createdAt??0,row.accessedAt??0);
  rows.push({key:row.hash,slot:nxTaskSlotV1(row,platform),storedAt:stored,accessed:stored,size:0});
 }
 const verdicts=classifySlotsV1(rows,{now:nowMs,graceMilliseconds:options.graceMilliseconds??SLOT_GRACE_MILLISECONDS_V1});
 let current=0,retained=0,bytes=0;const removed:string[]=[];
 for(const row of rows){
  const verdict=verdicts.get(row.key)!;
  if(verdict.state==="current")current++;else if(verdict.state==="retained")retained++;
  else if(verdict.state==="expired"){
   const directory=join(options.cacheDirectory,row.key);bytes+=directoryBytes(directory);removed.push(row.key);
   if(options.dryRun!==true){rmSync(directory,{recursive:true,force:true});rmSync(directory+".commit",{force:true});}
  }
 }
 if(options.dryRun!==true&&removed.length>0)await options.forget?.(removed);
 return Object.freeze({dryRun:options.dryRun===true,scanned:rows.length,current,retained,removed:Object.freeze(removed),bytes});
}

/** 📮️ Registers the slots of Nx tasks at the fleet cache after a run (`POST /<instance>/v1/slots`), because Nx's own client cannot send the slot header; the server makes each key that it stores current in its slot. Fails soft: any error, timeout or non-200 answer returns `undefined` and never fails a build. */
export async function registerNxSlotsV1(options:Readonly<{endpoint:string;instance:string;token?:string|undefined;rows:readonly Pick<NxTaskRowV1,"hash"|"project"|"target"|"configuration">[];platform?:string;timeoutMilliseconds?:number;fetch?:typeof fetch}>):Promise<Readonly<{claimed:number;missing:number}>|undefined>{
 const platform=options.platform??nxPlatformV1(),entries=options.rows.filter(row=>/^[a-z0-9_-]{1,128}$/u.test(row.hash)).map(row=>({kind:"nx" as const,key:row.hash,slot:nxTaskSlotV1(row,platform)}));
 if(entries.length===0)return {claimed:0,missing:0};
 let claimed=0,missing=0;
 try{
  for(let offset=0;offset<entries.length;offset+=1024){
   const answer=await(options.fetch??fetch)(`${options.endpoint}/${options.instance}/v1/slots`,{method:"POST",headers:{"content-type":"application/json",...options.token===undefined?{}:{authorization:`Bearer ${options.token}`}},body:JSON.stringify({entries:entries.slice(offset,offset+1024)}),signal:AbortSignal.timeout(options.timeoutMilliseconds??5000)});
   if(answer.status!==200){await answer.body?.cancel();return undefined;}
   const value=await answer.json() as {claimed?:number;missing?:number};claimed+=value.claimed??0;missing+=value.missing??0;
  }
  return {claimed,missing};
 }catch{return undefined;}
}
