import {constants} from "node:fs";
import {copyFile,link,lstat,mkdir,open,readFile,readdir,readlink,rename,rm,stat,symlink,utimes,writeFile,chmod} from "node:fs/promises";
import {randomUUID} from "node:crypto";
import {createReadStream} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join,relative,resolve,sep,isAbsolute} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {GlobV1} from "../../../../../../../../🔨️modules/⚙️runtime/🌐️glob/🟦️.ts";
import {SEMIO_POLICY_V1 as policy,semioOutputsDigestV1,type SemioEntryV1,type SemioFileV1} from "../🟦️.ts";

const slash=(value:string)=>value.split(sep).join("/"),wait=(milliseconds:number,signal?:AbortSignal)=>new Promise<void>((accept,refuse)=>{const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);accept();},milliseconds),abort=()=>{clearTimeout(timer);refuse(signal!.reason??Error("Cancelled"));};if(signal?.aborted)abort();else signal?.addEventListener("abort",abort,{once:true});});

/** 🔏️ Hashes a file by streaming it. */
export function semioFileDigestV1(path:string):Promise<string>{
 return new Promise((accept,refuse)=>{const hash=createHash("sha256"),stream=createReadStream(path);stream.on("data",chunk=>hash.update(chunk));stream.once("error",refuse);stream.once("end",()=>accept(hash.digest("hex")));});
}

/** 🧭️ Keeps a workspace-relative path inside the workspace. */
export function semioInsideV1(workspace:string,path:string):string{
 const full=resolve(workspace,path),inner=relative(resolve(workspace),full);
 if(inner===""||inner.startsWith("..")||isAbsolute(inner))throw Error(`A workspace-relative path is required: ${path}`);
 return full;
}

const both=(english:string,german:string)=>`${english}\n${german}`;

/** ⏱️ The lock timing of a store; the defaults are the engine policy. */
export type SemioStoreTimingV1=Readonly<{lockStaleMilliseconds:number;lockHeartbeatMilliseconds:number;lockPollMilliseconds:number}>;
/** 🔌️ What a store may be handed: how a lock is refreshed (a test substitutes a failing one) and the lock timing. */
export type SemioStorePortsV1=Readonly<{touch?:(path:string)=>Promise<void>;timing?:Partial<SemioStoreTimingV1>}>;

/** 🗄️ The local content-addressed action cache: output blobs by digest, entries by action key, locks per key. Every write is a temporary file renamed into place, so a reader never sees a partial entry. */
export class SemioStoreV1{
 private readonly touchLock:(path:string)=>Promise<void>;
 private readonly timing:SemioStoreTimingV1;
 constructor(readonly directory:string,ports:SemioStorePortsV1={}){
  this.touchLock=ports.touch??(async path=>{const now=new Date();await utimes(path,now,now);});
  this.timing={lockStaleMilliseconds:policy.lockStaleMilliseconds,lockHeartbeatMilliseconds:policy.lockHeartbeatMilliseconds,lockPollMilliseconds:policy.lockPollMilliseconds,...ports.timing};
 }
 blob(sha:string):string{return join(this.directory,"cas",sha.slice(0,2),sha);}
 entry(key:string):string{return join(this.directory,"ac",key.slice(0,2),key+".json");}
 private async place(target:string,write:(temporary:string)=>Promise<void>):Promise<void>{
  const temporary=join(this.directory,"tmp",randomUUID());
  await mkdir(dirname(temporary),{recursive:true});
  try{await write(temporary);await mkdir(dirname(target),{recursive:true});await rename(temporary,target);}finally{await rm(temporary,{force:true});}
 }
 /** 📥️ Stores a blob under its digest unless it is already there. */
 async putBlob(sha:string,bytes:Uint8Array):Promise<void>{
  try{await stat(this.blob(sha));return;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  await this.place(this.blob(sha),temporary=>writeFile(temporary,bytes));
 }
 /** 📤️ Reads a blob and checks its digest. */
 async readBlob(sha:string):Promise<Uint8Array>{
  const bytes=await readFile(this.blob(sha));
  if(createHash("sha256").update(bytes).digest("hex")!==sha)throw Error(`Corrupt cache blob ${sha}`);
  return bytes;
 }
 /** 🔎️ Reads the entry of a key when it exists, is well formed and all its blobs are present; otherwise it is a miss. */
 async lookup(key:string):Promise<SemioEntryV1|undefined>{
  let text:string;
  try{text=await readFile(this.entry(key),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
  let entry:SemioEntryV1;
  try{entry=JSON.parse(text);}catch{return undefined;}
  if(validateJsonSchemaSubset(schema.definitions.Entry,entry,schema).length||entry.key!==key||semioOutputsDigestV1(entry.files)!==entry.digest)return undefined;
  for(const file of entry.files)if(file.kind==="file"){try{await stat(this.blob(file.sha256!));}catch{return undefined;}}
  await this.touch(this.entry(key));
  return entry;
 }
 private async touch(path:string):Promise<void>{
  try{const facts=await stat(path);if(Date.now()-facts.atimeMs>policy.touchIntervalMilliseconds){const now=new Date();await utimes(path,now,facts.mtime);}}catch{}
 }
 /** 🧺️ Keeps the store within a byte bound: removes the least recently used entries until the entries and the blobs they reference fit, then the blobs no entry references and no writer can still be about to reference (younger than the grace). A dry run only counts. Entries and directories that cannot be read are counted in `unreadable`, so a quota is never silently under-counted. */
 async collect(options:Readonly<{maximumBytes:number;dryRun:boolean;now?:number}>):Promise<{removedEntries:number;removedBlobs:number;freedBytes:number;remainingBytes:number;unreadable:number}>{
  const now=options.now??Date.now();
  let unreadable=0;
  const list=async(directory:string):Promise<{path:string;name:string;size:number;used:number;changed:number}[]>=>{
   const rows:{path:string;name:string;size:number;used:number;changed:number}[]=[];
   let shards:string[];
   try{shards=await readdir(directory);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return rows;throw error;}
   for(const shard of shards)for(const name of await readdir(join(directory,shard)).catch(()=>{unreadable++;return [] as string[];})){
    try{const path=join(directory,shard,name),facts=await stat(path);rows.push({path,name,size:facts.size,used:facts.atimeMs,changed:facts.mtimeMs});}catch{unreadable++;}
   }
   return rows;
  };
  const entries=(await list(join(this.directory,"ac"))).filter(row=>row.name.endsWith(".json")),blobs=new Map((await list(join(this.directory,"cas"))).map(row=>[row.name,row]));
  const references=new Map<string,Set<string>>();
  for(const row of entries){
   try{const entry=JSON.parse(await readFile(row.path,"utf8")) as SemioEntryV1;references.set(row.path,new Set(entry.files.flatMap(file=>file.kind==="file"?[file.sha256!]:[])));}
   catch{references.set(row.path,new Set());}
  }
  const count=new Map<string,number>();
  for(const shas of references.values())for(const sha of shas)count.set(sha,(count.get(sha)??0)+1);
  let total=entries.reduce((sum,row)=>sum+row.size,0)+[...blobs.values()].reduce((sum,row)=>sum+row.size,0);
  const removedEntries:string[]=[],removedBlobs=new Set<string>();let freed=0;
  const young=(row:{changed:number})=>now-row.changed<policy.collectGraceMilliseconds;
  const orphan=(sha:string)=>{const blob=blobs.get(sha);return blob!==undefined&&!removedBlobs.has(sha)&&(count.get(sha)??0)===0&&!young(blob);};
  for(const [sha,blob] of blobs)if((count.get(sha)??0)===0&&!young(blob)&&total>options.maximumBytes){removedBlobs.add(sha);total-=blob.size;freed+=blob.size;}
  for(const row of [...entries].sort((left,right)=>left.used-right.used)){
   if(total<=options.maximumBytes)break;
   removedEntries.push(row.path);total-=row.size;freed+=row.size;
   for(const sha of references.get(row.path)!){const left=(count.get(sha)??1)-1;count.set(sha,left);if(left===0&&orphan(sha)){removedBlobs.add(sha);const blob=blobs.get(sha)!;total-=blob.size;freed+=blob.size;}}
  }
  if(!options.dryRun){
   for(const path of removedEntries)await rm(path,{force:true});
   for(const sha of removedBlobs)await rm(blobs.get(sha)!.path,{force:true});
  }
  return {removedEntries:removedEntries.length,removedBlobs:removedBlobs.size,freedBytes:freed,remainingBytes:total,unreadable};
 }
 /** 📝️ Publishes an entry atomically; a concurrent publisher of the same key writes identical content. */
 async publish(entry:SemioEntryV1):Promise<void>{await this.place(this.entry(entry.key),temporary=>writeFile(temporary,JSON.stringify(entry)));}
 /** 📸️ Reads the outputs a task declared from the workspace; with `copy` the file contents also enter the blob store. A declared output that was not produced is returned in `missing`. */
 async capture(workspace:string,task:string,key:string,patterns:readonly string[],copy:boolean,signal?:AbortSignal):Promise<{entry:SemioEntryV1;missing:string[]}>{
  const roots=new Set<string>(),missing:string[]=[],files:SemioFileV1[]=[];
  for(const pattern of patterns){
   const path=slash(relative(workspace,resolve(workspace,pattern)));
   semioInsideV1(workspace,path);
   if(/[?*{[]/u.test(path)){for await(const match of new GlobV1(path).scan({cwd:workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}
   else try{await lstat(join(workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;missing.push(path);}
  }
  const visit=async(path:string):Promise<void>=>{
   if(signal?.aborted)throw signal.reason??Error("Cancelled");
   if(files.length>policy.maximumOutputFiles)throw Error("Bounded output file count required");
   const source=join(workspace,path),facts=await lstat(source),mode=facts.mode&0o777;
   if(facts.isSymbolicLink())files.push({path,kind:"symlink",mode,link:await readlink(source)});
   else if(facts.isDirectory()){files.push({path,kind:"directory",mode});for(const child of (await readdir(source)).sort())await visit(path+"/"+child);}
   else{const sha=await semioFileDigestV1(source);files.push({path,kind:"file",mode,sha256:sha,size:facts.size});if(copy)try{await stat(this.blob(sha));}catch{await this.place(this.blob(sha),temporary=>copyFile(source,temporary,constants.COPYFILE_FICLONE));}}
  };
  const sorted=[...roots].sort();
  for(const path of sorted.filter(path=>!sorted.some(parent=>parent!==path&&path.startsWith(parent+"/"))))await visit(path);
  files.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
  return {entry:{version:1,key,task,files,digest:semioOutputsDigestV1(files)},missing};
 }
 /** ♻️ Replaces the workspace's outputs with an entry's: a matching tree is left alone, anything else is removed and written again from the blobs. */
 async restore(workspace:string,entry:SemioEntryV1,signal?:AbortSignal):Promise<void>{
  const roots=entry.files.map(file=>file.path).filter((path,_at,all)=>!all.some(parent=>parent!==path&&path.startsWith(parent+"/")));
  if(await this.matches(workspace,entry))return;
  for(const root of roots){if(signal?.aborted)throw signal.reason??Error("Cancelled");await rm(semioInsideV1(workspace,root),{recursive:true,force:true});}
  for(const file of entry.files){
   if(signal?.aborted)throw signal.reason??Error("Cancelled");
   const target=semioInsideV1(workspace,file.path);
   await mkdir(dirname(target),{recursive:true});
   if(file.kind==="directory")await mkdir(target,{recursive:true});
   else if(file.kind==="symlink")await symlink(file.link!,target);
   else{await copyFile(this.blob(file.sha256!),target,constants.COPYFILE_FICLONE);await chmod(target,file.mode);}
  }
  for(const file of [...entry.files].filter(file=>file.kind==="directory").sort((left,right)=>right.path.length-left.path.length))await chmod(semioInsideV1(workspace,file.path),file.mode);
 }
 private async matches(workspace:string,entry:SemioEntryV1):Promise<boolean>{
  try{return (await this.capture(workspace,entry.task,entry.key,entry.files.filter((file,_at,all)=>!all.some(parent=>parent!==file&&file.path.startsWith(parent.path+"/"))).map(file=>file.path),false)).entry.digest===entry.digest;}catch{return false;}
 }
 /** 🚦️ Runs `body` as the single owner of an action key across processes. A waiter polls until the owner releases the lock or the entry appears, then reports `owner: false` so the caller restores instead of executing. The lock file carries a token (pid, random nonce, time): the owner refreshes it and checks that it still holds the token, and a failed refresh or a lost token aborts `body` through its second argument and fails the call with the reason, never silently. A stale lock is broken by moving it aside and re-checking what was moved, and a lock is released only when it still carries the owner's own token. The leases module needs an issued invocation phase, which the engine's store does not have, so this lock stays private to the store. */
 async withFlight<T>(key:string,signal:AbortSignal,body:(owner:boolean,held:AbortSignal)=>Promise<T>):Promise<T>{
  const lock=join(this.directory,"locks",key+".lock");
  await mkdir(dirname(lock),{recursive:true});
  for(;;){
   if(signal.aborted)throw signal.reason??Error("Cancelled");
   const token=JSON.stringify({pid:process.pid,nonce:randomUUID(),at:Date.now()});
   let acquired=false;
   try{
    const handle=await open(lock,"wx");
    acquired=true;
    try{await handle.writeFile(token);}catch(error){await rm(lock,{force:true});throw error;}finally{await handle.close();}
   }catch(error){
    if(acquired||(error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   }
   if(acquired)return this.hold(lock,token,key,signal,body);
   if(await this.lookup(key))return body(false,signal);
   await this.breakStale(lock);
   await wait(this.timing.lockPollMilliseconds,signal);
  }
 }
 private async hold<T>(lock:string,token:string,key:string,signal:AbortSignal,body:(owner:boolean,held:AbortSignal)=>Promise<T>):Promise<T>{
  const lost=new AbortController(),held=AbortSignal.any([signal,lost.signal]);
  let failure:Error|undefined,busy=false;
  const lose=(english:string,german:string)=>{if(!failure){failure=Error(both(english,german));lost.abort(failure);}};
  const beat=setInterval(()=>{
   if(busy)return;
   busy=true;
   void this.refresh(lock,token).then(problem=>{if(problem)lose(problem[0],problem[1]);},error=>{const reason=String((error as Error).message??error);lose(`The build lock of ${key} could not be refreshed (${reason}); stopping so that two writers never run.`,`Die Build-Sperre von ${key} konnte nicht erneuert werden (${reason}); die Arbeit wird beendet, damit nie zwei Schreiber laufen.`);}).finally(()=>{busy=false;});
  },this.timing.lockHeartbeatMilliseconds);
  let result:T;
  try{
   const published=await this.lookup(key)!==undefined;
   result=await body(!published,held);
  }catch(error){
   clearInterval(beat);
   await this.release(lock,token);
   throw failure??error;
  }
  clearInterval(beat);
  const owned=await this.release(lock,token);
  if(failure)throw failure;
  if(!owned)throw Error(both(`The build lock of ${key} was taken over while its work ran.`,`Die Build-Sperre von ${key} wurde übernommen, während ihre Arbeit lief.`));
  return result;
 }
 private async refresh(lock:string,token:string):Promise<[string,string]|undefined>{
  let text:string;
  try{text=await readFile(lock,"utf8");}catch(error){
   if((error as NodeJS.ErrnoException).code==="ENOENT")return ["The build lock disappeared while its work ran.","Die Build-Sperre verschwand, während ihre Arbeit lief."];
   throw error;
  }
  if(text!==token)return ["The build lock was taken over while its work ran.","Die Build-Sperre wurde übernommen, während ihre Arbeit lief."];
  await this.touchLock(lock);
  return undefined;
 }
 private tombstone(lock:string,kind:string):string{return `${lock}.${randomUUID().slice(0,8)}.${kind}`;}
 private async release(lock:string,token:string):Promise<boolean>{
  const tomb=this.tombstone(lock,"release");
  try{await rename(lock,tomb);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return false;throw error;}
  let text:string|undefined;
  try{text=await readFile(tomb,"utf8");}catch{text=undefined;}
  if(text===token){await rm(tomb,{force:true});return true;}
  try{await link(tomb,lock);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
  await rm(tomb,{force:true});
  return false;
 }
 private stale(text:string,modified:number):boolean{
  let dead=false;
  try{const owner=JSON.parse(text) as {pid:number};try{process.kill(owner.pid,0);}catch(error){dead=(error as NodeJS.ErrnoException).code==="ESRCH";}}catch{dead=false;}
  return dead||Date.now()-modified>this.timing.lockStaleMilliseconds;
 }
 private async breakStale(lock:string):Promise<void>{
  let modified:number,text:string;
  try{modified=(await stat(lock)).mtimeMs;text=await readFile(lock,"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
  if(!this.stale(text,modified))return;
  const tomb=this.tombstone(lock,"stale");
  try{await rename(lock,tomb);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
  let moved:{text:string;modified:number}|undefined;
  try{moved={text:await readFile(tomb,"utf8"),modified:(await stat(tomb)).mtimeMs};}catch{moved=undefined;}
  if(moved&&moved.text===text&&this.stale(moved.text,moved.modified)){await rm(tomb,{force:true});return;}
  try{await link(tomb,lock);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
  await rm(tomb,{force:true});
 }
 /** 📊️ Counts the stored entries and the bytes of the store. */
 async status():Promise<{entries:number;bytes:number}>{
  let entries=0,bytes=0;
  const walk=async(directory:string,count:boolean):Promise<void>=>{
   let names:import("node:fs").Dirent[];
   try{names=await readdir(directory,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
   for(const name of names){const path=join(directory,name.name);if(name.isDirectory())await walk(path,count);else{bytes+=(await stat(path)).size;if(count)entries++;}}
  };
  await walk(join(this.directory,"ac"),true);await walk(join(this.directory,"cas"),false);
  return {entries,bytes};
 }
 /** 🧹️ Removes every entry, blob and stale temporary file. */
 async clear():Promise<void>{for(const name of ["ac","cas","tmp"])await rm(join(this.directory,name),{recursive:true,force:true});}
}
