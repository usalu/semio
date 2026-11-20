import {lstat,readdir,realpath,rm,stat} from "node:fs/promises";
import {join,sep} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The bounds of the writers that had none: waves, footprint logs, read verdicts, temporary directories and the files the replica overwrote (single source: the schema document of the recovery, definition `Bounds`). */
export type DiskBoundsV1=Readonly<{version:1;waves:Readonly<{maximumAgeMilliseconds:number;keepNewest:number}>;footprintLogs:Readonly<{maximumAgeMilliseconds:number;keepNewest:number}>;readVerdicts:Readonly<{maximumAgeMilliseconds:number;directoryPattern:string}>;temporaryDirectories:Readonly<{minimumAgeMilliseconds:number;maximumAgeMilliseconds:number;prefix:string}>;replicaLost:Readonly<{maximumAgeMilliseconds:number;keepNewest:number}>}>;
export const DISK_BOUNDS_V1:DiskBoundsV1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Bounds.const)));

/** 📄️ One entry of a directory a bound judges: its name, the newest write below it and the bytes it holds. */
export type BoundEntryV1=Readonly<{name:string;newestMs:number;bytes:number}>;
type BoundDirectoryEntryV1=BoundEntryV1&Readonly<{directory:boolean}>;

/** ⏳️ Selects the entries that are expired: older than the maximum age and not among the newest `keepNewest` (waves, replica files). Pure. */
export function selectExpiredV1(entries:readonly BoundEntryV1[],now:number,bound:Readonly<{maximumAgeMilliseconds:number;keepNewest:number}>):readonly string[]{
 const newest=[...entries].sort((first,second)=>second.newestMs-first.newestMs||(first.name<second.name?-1:1));
 return newest.filter((entry,index)=>index>=bound.keepNewest&&now-entry.newestMs>bound.maximumAgeMilliseconds).map(entry=>entry.name);
}

/** 🔁️ Selects the entries a rotation drops: older than the maximum age or beyond the newest `keepNewest`, so the count is bounded whatever the age (footprint logs). Pure. */
export function selectRotatedV1(entries:readonly BoundEntryV1[],now:number,bound:Readonly<{maximumAgeMilliseconds:number;keepNewest:number}>):readonly string[]{
 const newest=[...entries].sort((first,second)=>second.newestMs-first.newestMs||(first.name<second.name?-1:1));
 return newest.filter((entry,index)=>index>=bound.keepNewest||now-entry.newestMs>bound.maximumAgeMilliseconds).map(entry=>entry.name);
}

/** 🧬️ Selects finished verdict directories by the authored reader name, age and live process custody. */
export function finishedReadVerdictV1(now:number,bound:DiskBoundsV1["readVerdicts"],alive:(pid:number)=>boolean):(entry:BoundDirectoryEntryV1)=>boolean{
 const pattern=new RegExp(bound.directoryPattern,"u");
 return entry=>{
  if(!entry.directory||now-entry.newestMs<=bound.maximumAgeMilliseconds)return false;
  const pid=pattern.exec(entry.name)?.[1];
  return pid!==undefined&&!alive(Number(pid));
 };
}

/** 🗑️ Selects the temporary directories of the build system to remove: a name that carries the pid of its creator (`semio-<tag>-<pid>-<random>`) goes once that process is gone and the directory is older than the minimum age, every other one once it is older than the maximum age; sockets, files, names without the prefix and names without the random suffix of a made temporary directory (the runtime root `semio-repo`, `semio-footprint`, `semio-scratch`) never go. Pure. */
export function selectTemporaryV1(entries:readonly (BoundEntryV1&Readonly<{directory:boolean}>)[],now:number,bound:DiskBoundsV1["temporaryDirectories"],alive:(pid:number)=>boolean):readonly string[]{
 return entries.filter(entry=>{
  if(!entry.directory||!entry.name.startsWith(bound.prefix)||!/-[A-Za-z0-9]{6}$/u.test(entry.name))return false;
  const age=now-entry.newestMs,named=/-(\d{2,9})-[A-Za-z0-9]{6}$/u.exec(entry.name)?.[1];
  if(named!==undefined)return age>bound.minimumAgeMilliseconds&&!alive(Number(named));
  return age>bound.maximumAgeMilliseconds;
 }).map(entry=>entry.name);
}

/** 📏️ The allocated bytes below a path, counted with a bounded number of entries (`limit`); symbolic links are counted, not followed. */
export async function boundedBytesV1(path:string,limit:number,signal?:AbortSignal):Promise<Readonly<{bytes:number;newestMs:number;complete:boolean}>>{
 let bytes=0,newestMs=0,seen=0;
 const stack=[path];
 while(stack.length){
  signal?.throwIfAborted();
  const current=stack.pop()!,info=await lstat(current).catch(()=>undefined);
  if(info===undefined)continue;
  bytes+=info.blocks*512;newestMs=Math.max(newestMs,info.mtimeMs);
  if(++seen>=limit)return{bytes,newestMs,complete:false};
  if(info.isDirectory())for(const name of await readdir(current).catch(()=>[] as string[]))stack.push(join(current,name));
 }
 return{bytes,newestMs,complete:true};
}

/** 👀️ Reads the entries of a directory with the newest write below each and its bytes, never following links. */
export async function boundEntriesV1(directory:string,limit:number,signal?:AbortSignal,candidate?:(entry:BoundDirectoryEntryV1)=>boolean):Promise<readonly BoundDirectoryEntryV1[]>{
 const names=await readdir(directory).catch(()=>[] as string[]),rows:BoundDirectoryEntryV1[]=[];
 for(const name of names){
  signal?.throwIfAborted();
  const info=await lstat(join(directory,name)).catch(()=>undefined);
  if(info===undefined)continue;
  const entry={name,newestMs:info.mtimeMs,bytes:info.blocks*512,directory:info.isDirectory()&&!info.isSymbolicLink()};
  if(candidate!==undefined&&!candidate(entry))continue;
  const walked=await boundedBytesV1(join(directory,name),limit,signal);
  rows.push({...entry,newestMs:Math.max(walked.newestMs,entry.newestMs),bytes:walked.bytes});
 }
 return rows;
}

/** 🗑️ Removes the named entries of a directory; an entry whose real path leaves the directory, a link and a name with a separator are refused, so a bound can only delete what is inside its own root. Answers the bytes removed. */
export async function removeBoundEntriesV1(directory:string,names:readonly string[],entries:readonly BoundEntryV1[],dryRun:boolean):Promise<number>{
 const root=await realpath(directory).catch(()=>undefined);
 if(root===undefined)return 0;
 let freed=0;
 for(const name of names){
  if(name===""||name.includes(sep)||name==="."||name==="..")continue;
  const path=join(directory,name),info=await lstat(path).catch(()=>undefined);
  if(info===undefined||info.isSymbolicLink())continue;
  const real=await realpath(path).catch(()=>undefined);
  if(real===undefined||!real.startsWith(root+sep))continue;
  if(!dryRun)await rm(path,{recursive:true,force:true});
  freed+=entries.find(entry=>entry.name===name)?.bytes??0;
 }
 return freed;
}

/** 🧺️ One bound applied to a real directory: reads its entries, selects by the rule and removes them inside the directory only. */
export async function applyBoundV1(input:Readonly<{directory:string;limit:number;dryRun:boolean;signal?:AbortSignal;candidate?:(entry:BoundDirectoryEntryV1)=>boolean;select(entries:readonly BoundDirectoryEntryV1[]):readonly string[]}>):Promise<Readonly<{removed:readonly string[];freedBytes:number}>>{
 const entries=await boundEntriesV1(input.directory,input.limit,input.signal,input.candidate),names=input.select(entries);
 return{removed:names,freedBytes:await removeBoundEntriesV1(input.directory,names,entries,input.dryRun)};
}

/** 🧪️ Whether a path exists as a directory, for the callers that skip an absent root. */
export const boundRootPresentV1=async(path:string):Promise<boolean>=>(await stat(path).catch(()=>undefined))?.isDirectory()===true;
