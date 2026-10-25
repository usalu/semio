import {existsSync,lstatSync,readdirSync,realpathSync,rmSync} from "node:fs";
import {join,sep} from "node:path";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {cacheLedgerV1,cacheRootDirectoryV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {undeclaredEntriesV1} from "../../../🧹️cache-gc/🏃️runner/🟦️.ts";
import type {DoctorPolicyV1} from "../../🟦️.ts";
import type {DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../📋️contract/🟦️.ts";

type Declaration=DoctorPolicyV1["caches"]["deadPublications"][number];
type Candidate=Readonly<{path:string;pid:number;partition:string}>;

const realOf=(path:string):string|undefined=>{try{return realpathSync(path);}catch{return undefined;}};
const directories=(path:string):string[]=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>join(path,entry.name));}catch{return [];}};

/** 🔭️ Expands a declared publication folder pattern (`*` stands for every subfolder) below one partition. */
function folders(base:string,pattern:readonly string[]):readonly string[]{
 let current=[base];
 for(const segment of pattern)current=segment==="*"?current.flatMap(directories):current.map(path=>join(path,segment));
 return current.filter(path=>existsSync(path));
}

/** 🧟️ Finds the per-process publications of processes that do not exist and that were quiet for the declared time; a process that cannot be signalled counts as alive. */
export function deadPublicationsV1(ports:DoctorPortsV1,policy:DoctorPolicyV1["caches"]):readonly Candidate[]{
 const found:Candidate[]=[];
 for(const declaration of policy.deadPublications as readonly Declaration[]){
  const partition=(budget.partitions as Record<string,{directory:string}>)[declaration.partition];if(!partition)continue;
  const expression=new RegExp("^"+declaration.prefix+"(\\d+)$","u"),base=join(ports.root,...partition.directory.split("/")),boundary=realOf(base);
  if(boundary===undefined)continue;
  for(const folder of folders(base,declaration.path)){
   const real=realOf(folder);if(real===undefined||real!==boundary&&!real.startsWith(boundary+sep))continue;
   let names:import("node:fs").Dirent[];try{names=readdirSync(folder,{withFileTypes:true});}catch{continue;}
   for(const entry of names){
    const match=expression.exec(entry.name);if(!match||!entry.isDirectory())continue;
    const pid=Number(match[1]),path=join(folder,entry.name);
    let info;try{info=lstatSync(path);}catch{continue;}
    if(info.isSymbolicLink()||ports.alive(pid)||ports.now()-info.mtimeMs<policy.deadPublicationQuietSeconds*1000)continue;
    found.push({path,pid,partition:declaration.partition});
   }
  }
 }
 return found;
}

/** 📏️ Sums the allocated bytes of a tree without following links, up to a bounded number of entries. */
export function allocatedBytesV1(path:string,limit:number):Readonly<{bytes:number;partial:boolean;entries:number}>{
 let bytes=0,seen=0;const stack=[path];
 while(stack.length){
  const directory=stack.pop()!;let entries:import("node:fs").Dirent[];
  try{entries=readdirSync(directory,{withFileTypes:true});}catch{continue;}
  for(const entry of entries){
   if(++seen>limit)return {bytes,partial:true,entries:seen-1};
   const child=join(directory,entry.name);
   if(entry.isDirectory()){stack.push(child);continue;}
   try{const info=lstatSync(child);bytes+=info.blocks?info.blocks*512:info.size;}catch{}
  }
 }
 return {bytes,partial:false,entries:seen};
}

/** 🧹️ Removes the dead publications; each target is re-derived and re-checked right before it goes, links are never followed and nothing outside its partition is touched. */
async function repair(ports:DoctorPortsV1,context:Parameters<DoctorProbeV1["read"]>[1],dryRun:boolean):Promise<DoctorRepairRunV1>{
 let removed=0,bytes=0,budgetLeft=context.policy.caches.walkEntries;
 for(const target of deadPublicationsV1(ports,context.policy.caches)){
  ports.signal.throwIfAborted();
  const partition=(budget.partitions as Record<string,{directory:string}>)[target.partition]!,boundary=realpathSync(join(ports.root,...partition.directory.split("/")));
  let real:string;try{real=realpathSync(target.path);}catch{continue;}
  if(!real.startsWith(boundary+sep)||ports.alive(target.pid)||lstatSync(target.path).isSymbolicLink())continue;
  const size=allocatedBytesV1(target.path,Math.max(1,budgetLeft));bytes+=size.bytes;budgetLeft-=size.entries;removed++;
  if(!dryRun)rmSync(target.path,{recursive:true,force:true});
 }
 return removed===0?{removed:0,bytes:0,status:"nothing"}:{removed,bytes};
}

/** ⚡️ The cache facts that need no tree walk: dead per-process publications with their size, retired roots that still exist and entries nobody declared. */
export const cachesProbeV1:DoctorProbeV1=Object.freeze({
 area:"caches",phase:1,
 repairs:[Object.freeze({id:"caches-dead-publications",area:"caches" as const,run:repair})],
 async read(ports,context){
  const dead=deadPublicationsV1(ports,context.policy.caches);let bytes=0,partial=false,budgetLeft=context.policy.caches.walkEntries;
  for(const target of dead){
   ports.signal.throwIfAborted();
   const size=allocatedBytesV1(target.path,Math.max(1,budgetLeft));bytes+=size.bytes;partial=partial||size.partial;budgetLeft-=size.entries;
  }
  const ledger=cacheLedgerV1(),retired:string[]=[],undeclared:string[]=[];
  for(const row of ledger.retirements)if(existsSync(join(cacheRootDirectoryV1(ports.root,ledger,row.root),...row.directory.split("/"))))retired.push(row.root+":"+row.directory);
  for(const root of ["repository","native"] as const){
   ports.signal.throwIfAborted();
   for(const name of await undeclaredEntriesV1(ports.root,ledger,root))undeclared.push(root+":"+name);
  }
  return {sections:{caches:{deadPublications:dead.length,deadPublicationBytes:bytes,deadPublicationPartial:partial,retired,undeclared}}};
 },
});
