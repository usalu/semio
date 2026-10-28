import {createHash} from "node:crypto";
import {constants} from "node:fs";
import {chmod,copyFile,lstat,open,readdir,rename,unlink,utimes,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";
import {cacheRecencyMsV1,cacheTextV1,formatCacheBytesV1,type CacheLanguageV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import type {CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";

/** 👯️ One regular file seen by the duplicate census; `identity` names the inode (`device:inode`), `protected` says a recent use or a live-writer name forbids replacing it. */
export type DedupeFileV1=Readonly<{path:string;size:number;hash:string;identity:string;links:number;protected:boolean}>;
/** 🔁️ One planned replacement: `path` becomes a copy-on-write clone of `canonical`. */
export type DedupeReplacementV1=Readonly<{canonical:string;path:string;bytes:number}>;
/** 📊️ What a census found: duplicate groups, the files in them, the bytes that would be freed at most and the replacements. */
export type DedupePlanV1=Readonly<{groups:number;files:number;reclaimableBytes:number;replacements:readonly DedupeReplacementV1[]}>;
/** 🎯️ One declared place where duplicates are looked for. */
export type DedupeRequestV1=Readonly<{base:string;minimumBytes:number;quietMs:number;maximumFiles:number;dryRun:boolean;language:CacheLanguageV1;write(line:string):void}>;
/** 🧮️ What a run found and did; `clones` is false when the volume cannot make copy-on-write clones, so zero replacements are explained and not silent. */
export type DedupeOutcomeV1=Readonly<{plan:DedupePlanV1;clones:boolean;replaced:number;freedBytes:number;skipped:number;failed:readonly string[];partial:boolean}>;

const byPath=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const VOLATILE=/(\.sqlite|\.sqlite3|\.db|\.log|\.jsonl|\.lock|\.pid|-wal|-shm|\.tmp)$/iu;

/**
 * 📐️ Pure duplicate plan over a census: files of equal size and sha256 form a group; the canonical file is the one with the most links (then the smallest path) among the unprotected ones; every other unprotected file of another inode is replaced by a clone of it.
 * Bytes count as freed when every path of a victim inode is replaced, once per inode, so the number is an upper bound (blocks may already be shared).
 */
export function planDedupeV1(files:readonly DedupeFileV1[]):DedupePlanV1{
 const groups=new Map<string,DedupeFileV1[]>();
 for(const file of files){const key=file.size+":"+file.hash,row=groups.get(key);if(row)row.push(file);else groups.set(key,[file]);}
 const replacements:DedupeReplacementV1[]=[];let count=0,members=0,reclaimable=0;
 for(const[,group]of[...groups].sort((left,right)=>byPath(left[0],right[0]))){
  if(new Set(group.map(file=>file.identity)).size<2)continue;
  const stable=group.filter(file=>!file.protected);if(!stable.length)continue;
  const canonical=[...stable].sort((left,right)=>right.links-left.links||byPath(left.path,right.path))[0]!,victims=group.filter(file=>!file.protected&&file.identity!==canonical.identity);
  if(!victims.length)continue;
  count++;members+=group.length;
  const identities=new Map<string,{all:number;replaced:number}>();
  for(const file of group.filter(file=>file.identity!==canonical.identity)){const row=identities.get(file.identity)??{all:0,replaced:0};row.all++;if(!file.protected)row.replaced++;identities.set(file.identity,row);}
  for(const row of identities.values())if(row.all===row.replaced)reclaimable+=group[0]!.size;
  for(const victim of [...victims].sort((left,right)=>byPath(left.path,right.path)))replacements.push({canonical:canonical.path,path:victim.path,bytes:victim.size});
 }
 return Object.freeze({groups:count,files:members,reclaimableBytes:reclaimable,replacements:Object.freeze(replacements)});
}

async function sha256V1(path:string,ports:CacheRunnerPortsV1):Promise<string|null>{
 let handle;try{handle=await open(path,"r");}catch{return null;}
 try{const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(1<<20);for(;;){ports.signal.throwIfAborted();const{bytesRead}=await handle.read(buffer,0,buffer.length,null);if(!bytesRead)break;hash.update(buffer.subarray(0,bytesRead));}return hash.digest("hex");}
 catch(error){if(ports.signal.aborted)throw error;return null;}
 finally{await handle.close().catch(()=>{});}
}

async function* filesV1(base:string,ports:CacheRunnerPortsV1):AsyncGenerator<{path:string;size:number;identity:string;links:number;recencyMs:number;mtimeMs:number;mode:number;ino:number}>{
 const stack=[base];
 while(stack.length){
  ports.signal.throwIfAborted();const directory=stack.pop()!;let children;
  try{children=await readdir(directory,{withFileTypes:true});}catch{continue;}
  for(const child of children){
   const path=join(directory,child.name);
   if(child.isDirectory()){stack.push(path);continue;}
   if(!child.isFile())continue;
   let info;try{info=await lstat(path,{bigint:false});}catch{continue;}
   if(!info.isFile())continue;
   yield{path,size:info.size,identity:info.dev+":"+info.ino,links:info.nlink,recencyMs:cacheRecencyMsV1(info,ports.now()),mtimeMs:info.mtimeMs,mode:info.mode&0o7777,ino:info.ino};
  }
 }
}

/** 🧬️ Clones a file copy-on-write or says it cannot (no clone support on the volume): never a real copy, never a hardlink, so the replacement can not write through to another path. */
export async function cloneFileV1(source:string,destination:string):Promise<boolean>{
 try{await copyFile(source,destination,constants.COPYFILE_FICLONE_FORCE);return true;}catch{await unlink(destination).catch(()=>{});return false;}
}

/** 🔬️ Whether the volume of `base` makes clones: a probe file is cloned once, so a volume without clones (ext4, NTFS) ends the run before the census hashes a single file. A directory that cannot be written reads as "no clones". */
async function volumeClonesV1(base:string,clone:(source:string,destination:string)=>Promise<boolean>):Promise<boolean>{
 const source=join(base,".dedupe-probe-"+process.pid),destination=source+".clone";
 try{await writeFile(source,"x");return await clone(source,destination);}catch{return false;}
 finally{await unlink(source).catch(()=>{});await unlink(destination).catch(()=>{});}
}

/**
 * 🧹️ Finds duplicates below `base` with a streaming census (one walk counting sizes, one walk hashing only files whose size occurs twice, hardlinks hashed once) and, unless asked to only plan, replaces each duplicate by a copy-on-write clone of the canonical file.
 * A file used within the quiet time, with a volatile name, or changed since it was hashed is left alone; the replacement is a temporary clone renamed over the file, so a reader sees either the old or the new, identical, bytes. Progress and cancellation per file; memory stays bounded by `maximumFiles` candidates.
 */
export async function dedupeTreeV1(request:DedupeRequestV1,ports:CacheRunnerPortsV1,clone:(source:string,destination:string)=>Promise<boolean>=cloneFileV1):Promise<DedupeOutcomeV1>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(request.language,key,variables),now=ports.now(),sizes=new Map<number,number>();
 if(!request.dryRun&&!await volumeClonesV1(request.base,clone)){request.write(text("dedupe.noClone",{directory:request.base}));return Object.freeze({plan:planDedupeV1([]),clones:false,replaced:0,freedBytes:0,skipped:0,failed:[],partial:false});}
 let seen=0,last=-Infinity;const tick=async(stage:string):Promise<void>=>{ports.signal.throwIfAborted();const at=ports.now();if(at-last>=CACHE_GC_LIMITS_V1.dedupeProgressMilliseconds){last=at;await ports.progress(stage);ports.signal.throwIfAborted();}};
 for await(const file of filesV1(request.base,ports)){if(file.size<request.minimumBytes)continue;sizes.set(file.size,(sizes.get(file.size)??0)+1);if(++seen%CACHE_GC_LIMITS_V1.dedupeCountProgressStride===0)await tick("dedupe:count");}
 const candidates:DedupeFileV1[]=[],stamps=new Map<string,{ino:number;mtimeMs:number;size:number;mode:number}>(),hashes=new Map<string,string|null>();let partial=false,hashed=0;
 for await(const file of filesV1(request.base,ports)){
  if(file.size<request.minimumBytes||(sizes.get(file.size)??0)<2)continue;
  if(candidates.length>=request.maximumFiles){partial=true;break;}
  await tick("dedupe:hash "+hashed);
  let hash=hashes.get(file.identity);if(hash===undefined){hash=await sha256V1(file.path,ports);hashed++;if(file.links>1)hashes.set(file.identity,hash);}
  if(hash===null)continue;
  candidates.push({path:file.path,size:file.size,hash,identity:file.identity,links:file.links,protected:now-file.recencyMs<request.quietMs||VOLATILE.test(file.path)});
  stamps.set(file.path,{ino:file.ino,mtimeMs:file.mtimeMs,size:file.size,mode:file.mode});
 }
 const plan=planDedupeV1(candidates);
 request.write(text(request.dryRun?"dedupe.plan":"dedupe.start",{groups:plan.groups,files:plan.files,bytes:formatCacheBytesV1(plan.reclaimableBytes),replacements:plan.replacements.length}));
 if(request.dryRun)return Object.freeze({plan,clones:true,replaced:0,freedBytes:0,skipped:0,failed:[],partial});
 let replaced=0,skipped=0,freed=0,counter=0;const failed:string[]=[],seenIdentity=new Set<string>(),candidateOf=new Map(candidates.map(file=>[file.path,file]));
 for(const row of plan.replacements){
  await tick("dedupe:replace "+replaced);
  const stamp=stamps.get(row.path)!,temporary=join(dirname(row.path),".dedupe-"+process.pid+"-"+(counter++));
  try{
   const before=await lstat(row.path);
   if(!before.isFile()||before.ino!==stamp.ino||before.mtimeMs!==stamp.mtimeMs||before.size!==stamp.size||ports.now()-Math.max(before.mtimeMs,before.atimeMs)<request.quietMs){skipped++;request.write(text("dedupe.changed",{path:row.path}));continue;}
   const origin=stamps.get(row.canonical)!,unchanged=async():Promise<boolean>=>{const source=await lstat(row.canonical);return source.isFile()&&source.ino===origin.ino&&source.mtimeMs===origin.mtimeMs&&source.size===origin.size&&source.size===row.bytes;};
   if(!await unchanged()){skipped++;request.write(text("dedupe.changed",{path:row.canonical}));continue;}
   if(!await clone(row.canonical,temporary)){skipped++;request.write(text("dedupe.unsupported",{path:row.path}));continue;}
   await chmod(temporary,stamp.mode);await utimes(temporary,before.atime,before.mtime);
   const again=await lstat(row.path);
   if(again.ino!==stamp.ino||again.mtimeMs!==stamp.mtimeMs||again.size!==stamp.size||!await unchanged()){await unlink(temporary).catch(()=>{});skipped++;request.write(text("dedupe.changed",{path:row.path}));continue;}
   await rename(temporary,row.path);replaced++;
   const identity=candidateOf.get(row.path)!.identity;if(!seenIdentity.has(identity)){seenIdentity.add(identity);freed+=row.bytes;}
   request.write(text("dedupe.replaced",{path:row.path,bytes:formatCacheBytesV1(row.bytes)}));
  }catch(error){await unlink(temporary).catch(()=>{});if(ports.signal.aborted)throw error;failed.push(row.path+": "+String((error as Error).message??error));}
 }
 return Object.freeze({plan,clones:true,replaced,freedBytes:freed,skipped,failed,partial});
}
