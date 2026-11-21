import {spawn} from "node:child_process";
import {open,readdir,rm,stat} from "node:fs/promises";
import {basename,dirname,join} from "node:path";
import {GIT_PINS_V1,resolveGitV1} from "../../../🌿️git/🟦️.ts";

/** 🧺️ What the bootstrap archive carries of `.git/objects/pack`: `keep` stems (packs holding an object no larger kept pack holds, and every pack whose index cannot be read), `covered` stems (every object lies in a kept pack), `orphans` (pack without index or index without pack), `garbage` names (`tmp_*`, locks, `multi-pack-index*`, unknown files) and the object and byte totals. Schema: `./🧬️schema/🔣️.json#/$defs/PackPlan`. */
export type PackPlanV1=Readonly<{keep:readonly string[];covered:readonly string[];orphans:readonly string[];garbage:readonly string[];objects:number;bytesKept:number;bytesDropped:number}>;

type Names=Readonly<{count:number;fanout:Uint32Array;names:Buffer}>;

const HASH=20,HEADER=Buffer.from([0xff,0x74,0x4f,0x63,0,0,0,2]),companions=new Set(["idx","rev","mtimes","bitmap","keep","promisor"]),yieldEvery=100_000;
const later=():Promise<void>=>new Promise(done=>setImmediate(done));

/** 🔬️ Reads the sorted object-name table of a version 2 pack index (SHA-1 names); `undefined` when the file is not a complete index of that shape. */
export async function readPackNamesV1(idx:string,options:Readonly<{maximumBytes?:number;expectedIdentity?:string;signal?:AbortSignal}>={}):Promise<Names|undefined>{
 const {maximumBytes,expectedIdentity,signal}=options;
 signal?.throwIfAborted();
 const handle=await open(idx,"r").catch(()=>undefined);
 if(handle===undefined)return undefined;
 try{
  const identity=async()=>{const value=await handle.stat({bigint:true});return [value.dev,value.ino,value.size,value.mtimeNs,value.ctimeNs].join(":");};
  if(expectedIdentity!==undefined&&await identity()!==expectedIdentity)return undefined;
  const size=(await handle.stat()).size,head=Buffer.alloc(8+1024);
  if((maximumBytes!==undefined&&size>maximumBytes)||size<head.length+2*HASH||(await handle.read(head,0,head.length,0)).bytesRead!==head.length||!head.subarray(0,8).equals(HEADER))return undefined;
  const fanout=new Uint32Array(256);
  for(let index=0;index<256;index++)fanout[index]=head.readUInt32BE(8+4*index);
  const count=fanout[255]!;
  if(size<head.length+count*(HASH+8)+2*HASH)return undefined;
  const names=Buffer.alloc(count*HASH);
  let done=0;
  while(done<names.length){signal?.throwIfAborted();const {bytesRead}=await handle.read(names,done,Math.min(names.length-done,1<<24),head.length+done);if(bytesRead===0)return undefined;done+=bytesRead;}
  if(expectedIdentity!==undefined&&await identity()!==expectedIdentity)return undefined;
  return {count,fanout,names};
 }catch{signal?.throwIfAborted();return undefined;}finally{await handle.close();}
}

function holds(table:Names,name:Buffer,offset:number):boolean{
 const first=name[offset]!;
 let low=first===0?0:table.fanout[first-1]!,high=table.fanout[first]!;
 while(low<high){
  const middle=(low+high)>>>1,order=table.names.compare(name,offset,offset+HASH,middle*HASH,middle*HASH+HASH);
  if(order===0)return true;
  if(order>0)high=middle;else low=middle+1;
 }
 return false;
}

async function coveredBy(pack:Names,kept:readonly Names[]):Promise<boolean>{
 for(let index=0;index<pack.count;index++){
  let found=false;
  for(const table of kept)if(holds(table,pack.names,index*HASH)){found=true;break;}
  if(!found)return false;
  if(index%yieldEvery===yieldEvery-1)await later();
 }
 return true;
}

const split=(name:string):readonly [string,string]|undefined=>{
 const dot=name.lastIndexOf(".");
 return dot>0?[name.slice(0,dot),name.slice(dot+1)]:undefined;
};

/** 🧮️ Plans the archive content of one pack directory by comparing the index files (no object is read): stems without a complete pair are orphans, a pack whose index cannot be read is kept, the others are visited from the largest down (the stems of `seed` first) and kept only when one of their objects is in no kept pack. Everything else is `covered`. */
export async function planPacksV1(directory:string,signal?:AbortSignal,seed:readonly string[]=[]):Promise<PackPlanV1>{
 const files=new Map<string,Map<string,number>>(),garbage:string[]=[];
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  const parts=split(name),info=await stat(join(directory,name)).catch(()=>undefined);
  if(info===undefined||!info.isFile())continue;
  if(parts===undefined||name.startsWith("tmp_")||!companions.has(parts[1])&&parts[1]!=="pack"){garbage.push(name);continue;}
  const group=files.get(parts[0])??new Map<string,number>();
  group.set(parts[1],info.size);files.set(parts[0],group);
 }
 const orphans:string[]=[],candidates:{stem:string;size:number}[]=[];
 for(const [stem,group] of files){
  if(group.has("pack")&&group.has("idx"))candidates.push({stem,size:group.get("pack")!});else orphans.push(stem);
 }
 candidates.sort((left,right)=>Number(seed.includes(right.stem))-Number(seed.includes(left.stem))||right.size-left.size||(left.stem<right.stem?-1:1));
 const keep:string[]=[],covered:string[]=[],tables:Names[]=[];
 let objects=0;
 for(const {stem} of candidates){
  signal?.throwIfAborted();
  const table=await readPackNamesV1(join(directory,`${stem}.idx`));
  if(table===undefined){keep.push(stem);continue;}
  if(tables.length>0&&await coveredBy(table,tables)){covered.push(stem);continue;}
  keep.push(stem);tables.push(table);objects+=table.count;
 }
 const total=(stems:readonly string[]):number=>stems.reduce((sum,stem)=>sum+[...files.get(stem)!.values()].reduce((left,right)=>left+right,0),0);
 return {keep:keep.sort(),covered:covered.sort(),orphans:orphans.sort(),garbage:garbage.sort(),objects,bytesKept:total(keep),bytesDropped:total(covered)+total(orphans)};
}

/** 🚫️ Tells whether a worktree path (`/`-separated, relative) stays out of the archive: temporary files and locks anywhere below `.git/objects`, `gc.log`, `gc.pid`, and in `.git/objects/pack` everything but the files of the kept stems of `plan` (the `multi-pack-index` always stays out: it names packs the archive may not carry, and git rebuilds it on demand). Paths elsewhere are not excluded. */
export function packExcludedV1(path:string,plan:PackPlanV1|undefined):boolean{
 const parts=path.split("/");
 if(parts[0]!==".git")return false;
 const name=parts.at(-1)!;
 if(parts.length===2&&(name==="gc.log"||name==="gc.pid"))return true;
 if(parts[1]!=="objects")return false;
 if(name.startsWith("tmp_")||name.endsWith(".lock"))return true;
 if(parts.length!==4||parts[2]!=="pack"||plan===undefined)return false;
 const pieces=split(name);
 return pieces===undefined||!companions.has(pieces[1])&&pieces[1]!=="pack"||!plan.keep.includes(pieces[0]);
}

/** 🎯️ The one pack of every object reachable from the refs: `git pack-objects --revs --all` with `HEAD` on stdin (a detached HEAD counts), deltas reused from the existing packs (`--window=0`, no search), written beside `stage` as `<stage>-<hash>.pack` and `.idx`; `hash` names the pack (`pack-<hash>` in an archive). `undefined` when git is missing, fails (a non-zero exit, no output) or `signal` aborts; the partial files are removed then. Nothing in the repository is changed. */
export async function writeReachablePackV1(worktree:string,stage:string,signal?:AbortSignal):Promise<Readonly<{hash:string;pack:string;idx:string;bytes:number}>|undefined>{
 const executable=await resolveGitV1();
 if(executable===null)return undefined;
 const args=["-C",worktree,...GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]),"pack-objects","--revs","--all","--delta-base-offset","--window=0","--threads=2","--quiet",stage];
 const hash=await new Promise<string|undefined>(done=>{
  const child=spawn(executable,args,{stdio:["pipe","pipe","ignore"],env:{...process.env,GIT_OPTIONAL_LOCKS:"0"},...signal===undefined?{}:{signal}});
  let out="";
  child.stdout.on("data",chunk=>{out+=String(chunk);});
  child.on("error",()=>done(undefined));
  child.on("close",code=>done(code===0&&/^[0-9a-f]{40}$/u.test(out.trim())?out.trim():undefined));
  child.stdin.on("error",()=>{});
  child.stdin.end("HEAD\n");
 });
 const pack=`${stage}-${hash}.pack`,idx=`${stage}-${hash}.idx`;
 if(hash===undefined){await discardReachablePackV1(stage);return undefined;}
 const info=await stat(pack).catch(()=>undefined);
 if(info===undefined)return undefined;
 return {hash,pack,idx,bytes:info.size};
}

/** 🗑️ Removes every file `writeReachablePackV1` wrote for `stage`. */
export async function discardReachablePackV1(stage:string):Promise<void>{
 for(const name of await readdir(dirname(stage)).catch(()=>[] as string[]))if(name.startsWith(`${basename(stage)}-`))await rm(join(dirname(stage),name),{force:true});
}
