import {createHash,randomBytes} from "node:crypto";
import {createReadStream} from "node:fs";
import {chmod,copyFile,lstat,mkdir,open,readdir,readFile,rename,rm,stat,type FileHandle} from "node:fs/promises";
import {dirname,isAbsolute,join,resolve} from "node:path";
import {blobPathV1,requireOriginIdV1,storePathsV1,type FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";
import {compileIgnoreV1,hashFileV1,scanSharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../../🚧️boundary/🟦️.ts";
import {platformV1,readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {foldStoreV1,requireReplicaShapeV1,storeReachableV1,type ReplicaEventV1,type ReplicaSinkV1} from "../🟦️.ts";

/** 🥾️ The inputs of the archive writer: the worktree to read, the store to write into and the origin that owns the staging names. */
export type BootstrapOptionsV1=Readonly<{worktree:string;storeRoot:string;originId:string}>;

/** 📦️ What one archive build wrote: the folded clock, the files and content bytes it holds, the files it skipped, the sha256 of the tar and the tar path. */
export type BootstrapResultV1=Readonly<{hlc:string;files:number;bytes:number;skipped:number;sha256:string;archive:string}>;

/** 🧾️ The document `manifest.json` next to the archive; `bytes` is the sum of the file contents, `sha256` is the hash of the whole tar. */
export type BootstrapManifestV1=Readonly<{version:1;hlc:string;files:number;bytes:number;sha256:string}>;

/** 🧹️ What a settle did: files that already matched the store, files moved aside and the folder that holds them. */
export type BootstrapSettleV1=Readonly<{kept:number;moved:number;folder:string}>;

/** 🧭️ The context a bootstrap command runs in: the repository root, cancellation, the stdout line sink, the message sink and the byte formatter. */
export type BootstrapContextV1=Readonly<{root:string;signal:AbortSignal;write:(line:string)=>void;say:(key:string,variables?:Readonly<Record<string,string|number>>)=>void;format:(bytes:number)=>string}>;

/** ⛔️ Raised when a bootstrap operation cannot start; `key` names the message `error.bootstrap.<key>`. */
export class BootstrapRefusedV1 extends Error{
 readonly key:"identity"|"empty"|"joined"|"store";
 readonly variables:Readonly<Record<string,string>>;
 constructor(key:"identity"|"empty"|"joined"|"store",variables:Readonly<Record<string,string>>={}){super(`Bootstrap refused: ${key}`);this.name="BootstrapRefusedV1";this.key=key;this.variables=variables;}
}

const BLOCK=512,LIMIT=8**11,CHUNK=1<<20,FIELD=/[^\x20-\x7e]/gu,windows=process.platform==="win32",staleMilliseconds=6*3_600_000,retryMinutes=2;
const code=(error:unknown):string|undefined=>(error as {code?:string}).code;
const gone=(error:unknown):boolean=>code(error)==="ENOENT"||code(error)==="ENOTDIR";
const busy=(error:unknown):boolean=>code(error)==="EBUSY"||code(error)==="EPERM"||code(error)==="EACCES";
const token=():string=>randomBytes(8).toString("hex");
const octal=(value:number,width:number):string=>`${value.toString(8).padStart(width-1,"0")}\0`;
const unsafe=(path:string):boolean=>path===""||path.startsWith("/")||path.includes("\\")||path.split("/").some(part=>part===""||part==="."||part===".."||part.includes("\0"));
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const where=(root:string,path:string):string=>isAbsolute(path)?path:resolve(root,path);

function ustar(name:string,size:number,mode:number,mtime:number,type:string):Buffer{
 const block=Buffer.alloc(BLOCK);
 block.write(name,0,100,"latin1");
 block.write(octal(mode,8),100,8,"latin1");
 block.write(octal(0,8),108,8,"latin1");
 block.write(octal(0,8),116,8,"latin1");
 block.write(octal(size<LIMIT?size:0,12),124,12,"latin1");
 block.write(octal(mtime,12),136,12,"latin1");
 block.fill(0x20,148,156);
 block.write(type,156,1,"latin1");
 block.write("ustar\0",257,6,"latin1");
 block.write("00",263,2,"latin1");
 let sum=0;
 for(const byte of block)sum+=byte;
 block.write(`${sum.toString(8).padStart(6,"0")}\0 `,148,8,"latin1");
 return block;
}

function paxRecord(key:string,value:string):Buffer{
 const body=Buffer.from(` ${key}=${value}\n`,"utf8");
 let digits=1;
 while(String(body.length+digits).length>digits)digits++;
 return Buffer.concat([Buffer.from(String(body.length+digits),"latin1"),body]);
}

/** 🏷️ Renders the header blocks of one regular file entry in POSIX pax format: a ustar header, preceded by an extended `x` header with `path` (UTF-8, whenever the name is not plain ASCII or exceeds 100 bytes) and `size` (whenever the size does not fit the 11 octal digits, that is from 8 GiB); the ustar `size` field then holds 0, as Python's `tarfile` writes it. The content follows, padded with zeros to a multiple of 512 bytes. */
export function tarEntryHeaderV1(path:string,size:number,executable:boolean,mtimeSeconds:number):Buffer{
 const plain=/^[\x20-\x7e]{1,100}$/u.test(path),time=Math.min(Math.max(0,Math.floor(mtimeSeconds)),LIMIT-1),records:Buffer[]=[];
 if(!plain)records.push(paxRecord("path",path));
 if(size>=LIMIT)records.push(paxRecord("size",String(size)));
 const entry=ustar(plain?path:path.replace(FIELD,"_").slice(0,100),size,executable?0o755:0o644,time,"0");
 if(records.length===0)return entry;
 const body=Buffer.concat(records);
 return Buffer.concat([ustar("PaxHeader/entry",body.length,0o644,time,"x"),body,Buffer.alloc((BLOCK-body.length%BLOCK)%BLOCK),entry]);
}

/** 🧱️ The zero bytes that pad a content of `size` bytes to a multiple of 512. */
export function tarPaddingV1(size:number):number{return (BLOCK-size%BLOCK)%BLOCK;}

/** 🏁️ The end of an archive: two zero blocks. */
export function tarEndV1():Buffer{return Buffer.alloc(2*BLOCK);}

/** 🗂️ Resolves the ASCII locations of the archive and its manifest below `<storeRoot>/v1/bootstrap`. */
export function bootstrapPathsV1(storeRoot:string):Readonly<{directory:string;archive:string;manifest:string}>{
 const directory=join(storePathsV1(storeRoot).root,"bootstrap");
 return {directory,archive:join(directory,"worktree.tar"),manifest:join(directory,"manifest.json")};
}

async function put(handle:FileHandle,data:Uint8Array,position:number):Promise<void>{
 let done=0;
 while(done<data.length){const {bytesWritten}=await handle.write(data,done,data.length-done,position+done);done+=bytesWritten;}
}

async function copyEntry(handle:FileHandle,file:string,path:string,wanted:FleetManifestEntryV1,start:number,signal:AbortSignal|undefined):Promise<number|string>{
 let info;
 try{info=await lstat(file);}catch(error){if(gone(error))return "missing";if(busy(error))return "busy";throw error;}
 if(!info.isFile()||info.size!==wanted.bytes)return "differs";
 const head=tarEntryHeaderV1(path,wanted.bytes!,wanted.executable!,info.mtimeMs/1000),hash=createHash("sha256");
 let position=start,seen=0;
 await put(handle,head,position);position+=head.length;
 try{
  for await(const chunk of createReadStream(file,signal?{highWaterMark:CHUNK,signal}:{highWaterMark:CHUNK})){
   const data=chunk as Buffer;
   seen+=data.length;
   if(seen>wanted.bytes!)return "changed";
   hash.update(data);
   await put(handle,data,position);position+=data.length;
  }
 }catch(error){if(gone(error))return "missing";if(busy(error))return "busy";throw error;}
 if(seen!==wanted.bytes||hash.digest("hex")!==wanted.sha256)return "changed";
 const pad=tarPaddingV1(seen);
 if(pad>0){await put(handle,Buffer.alloc(pad),position);position+=pad;}
 return position;
}

/** 🥾️ Writes `<storeRoot>/v1/bootstrap/worktree.tar` and `manifest.json` from the folded store manifest and the local worktree: one pax tar entry per shared file (links stay to `join`), streamed in 1 MiB chunks, every entry verified against its manifest hash while it is written (a file that is missing, busy, changed or different is skipped and the entry is rewound), staged in `<storeRoot>/v1/tmp` and renamed, tar first and manifest second. */
export async function writeBootstrapArchiveV1(options:BootstrapOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<BootstrapResultV1>{
 requireOriginIdV1(options.originId);
 const store=storePathsV1(options.storeRoot),paths=bootstrapPathsV1(options.storeRoot),folded=await foldStoreV1(options.storeRoot,signal,sink);
 const wanted=[...folded.manifest].filter(([path,entry])=>entry.operation==="put"&&!unsafe(path)).sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
 if(wanted.length===0)throw new BootstrapRefusedV1("empty");
 await mkdir(store.tmp,{recursive:true});await mkdir(paths.directory,{recursive:true});
 for(const name of await readdir(store.tmp).catch(()=>[] as string[])){
  if(!name.startsWith(`${options.originId}-bootstrap-`))continue;
  const file=join(store.tmp,name),info=await stat(file).catch(()=>undefined);
  if(info!==undefined&&Date.now()-info.mtimeMs>staleMilliseconds)await rm(file,{force:true});
 }
 const stem=join(store.tmp,`${options.originId}-bootstrap-${token()}`),archive=`${stem}.tar`,manifest=`${stem}.json`;
 let files=0,bytes=0,skipped=0,sha256="";
 try{
  const handle=await open(archive,"w");
  try{
   let position=0;
   for(const [index,[path,entry]] of wanted.entries()){
    signal?.throwIfAborted();
    const outcome=await copyEntry(handle,absolute(options.worktree,path),path,entry,position,signal);
    if(typeof outcome==="number"){position=outcome;files++;bytes+=entry.bytes!;}
    else{skipped++;sink({version:1,event:"skipped",stage:"bootstrap",path,reason:outcome});}
    if((index+1)%500===0)sink({version:1,event:"progress",stage:"bootstrap",done:index+1,total:wanted.length,bytes});
   }
   await put(handle,tarEndV1(),position);
   await handle.truncate(position+2*BLOCK);await handle.sync();
  }finally{await handle.close();}
  sha256=(await hashFileV1(archive)).sha256;
  const document:BootstrapManifestV1={version:1,hlc:folded.hlc,files,bytes,sha256};
  const out=await open(manifest,"w");
  try{await out.writeFile(JSON.stringify(document));await out.sync();}finally{await out.close();}
  await rename(archive,paths.archive);
  await rename(manifest,paths.manifest);
 }catch(error){await rm(archive,{force:true});await rm(manifest,{force:true});throw error;}
 sink({version:1,event:"progress",stage:"bootstrap",done:wanted.length,total:wanted.length,bytes});
 return {hlc:folded.hlc,files,bytes,skipped,sha256,archive:paths.archive};
}

/** 🔄️ Rebuilds the archive now and then every `minutes`, until `signal` aborts; a failed build is reported as an `error` event and retried after at most two minutes, so the relay of a group server never stops for it and a store that is still empty gets its archive soon after the first files arrive. Staging files of a crashed build (older than six hours) are removed by the next build. */
export async function refreshBootstrapV1(options:BootstrapOptionsV1&Readonly<{minutes:number}>,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 while(!signal.aborted){
  let minutes=options.minutes;
  try{await writeBootstrapArchiveV1(options,signal,sink);}
  catch(error){if(signal.aborted)break;minutes=Math.min(minutes,retryMinutes);sink({version:1,event:"error",stage:"bootstrap",message:error instanceof BootstrapRefusedV1?error.message:error instanceof Error?error.message:String(error)});}
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,minutes*60_000);signal.addEventListener("abort",finish,{once:true});
  });
 }
}

async function matches(worktree:string,entry:FleetScanEntryV1,wanted:FleetManifestEntryV1|undefined):Promise<boolean>{
 if(wanted?.operation==="link")return entry.kind==="link"&&entry.target===wanted.target;
 if(wanted?.operation==="put"&&entry.kind==="file"&&entry.bytes===wanted.bytes&&(windows||entry.executable===wanted.executable))return (await hashFileV1(absolute(worktree,entry.path))).sha256===wanted.sha256;
 return false;
}

async function healNamesV1(worktree:string,ignore:FleetIgnoreV1,signal:AbortSignal|undefined,sink:ReplicaSinkV1):Promise<void>{
 for(let round=0;round<64;round++){
  const todo=(await scanSharedV1(worktree,ignore,signal)).refused.filter(item=>item.reason==="not-nfc").map(item=>item.path);
  let renamed=0;
  for(const path of todo){
   const clean=path.normalize("NFC"),directory=dirname(absolute(worktree,path)),siblings=await readdir(directory).catch(()=>[] as string[]);
   if(siblings.includes(clean.slice(clean.lastIndexOf("/")+1)))continue;
   await rename(absolute(worktree,path),absolute(worktree,clean));
   renamed++;
   sink({version:1,event:"skipped",stage:"settle",path:clean,reason:"renamed-to-nfc"});
  }
  if(renamed===0)return;
 }
}

/** 🧹️ Prepares a worktree extracted from a bootstrap archive for `join`: it folds the store and moves every shared file that differs from it (changed since the archive was built, deleted since, rewritten by a tool) to `.cache/f/lost/bootstrap-<ms>/<path>`, so that `join` finds only matching files and materialises the rest. It refuses a worktree that is already joined and a store without files, and never deletes. */
export async function settleBootstrapV1(options:Readonly<{worktree:string;storeRoot:string;now?:()=>number}>,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<BootstrapSettleV1>{
 const machine=join(options.worktree,".cache","f","machine.json");
 if(await stat(machine).then(()=>true,()=>false))throw new BootstrapRefusedV1("joined",{file:machine});
 if(!await storeReachableV1(options.storeRoot))throw new BootstrapRefusedV1("store",{store:storePathsV1(options.storeRoot).root});
 const folded=await foldStoreV1(options.storeRoot,signal,sink);
 if(![...folded.manifest.values()].some(entry=>entry.operation==="put"))throw new BootstrapRefusedV1("empty");
 const rules=folded.manifest.get(".gitignore"),text=rules?.operation==="put"?await readFile(blobPathV1(options.storeRoot,rules.sha256!),"utf8"):await readFile(join(options.worktree,".gitignore"),"utf8").catch(()=>"");
 const compiled=compileIgnoreV1(text),ignore:FleetIgnoreV1={digest:compiled.digest,ignores:(path,isDirectory)=>path===".cache/f"||path.startsWith(".cache/f/")||compiled.ignores(path,isDirectory),child:(path,isDirectory)=>path===".cache/f"||path.startsWith(".cache/f/")||compiled.child!(path,isDirectory)};
 await healNamesV1(options.worktree,ignore,signal,sink);
 const scan=await scanSharedV1(options.worktree,ignore,signal),folder=join(options.worktree,".cache","f","lost",`bootstrap-${(options.now??Date.now)()}`);
 let kept=0,moved=0;
 for(const [index,entry] of scan.entries.entries()){
  signal?.throwIfAborted();
  if(await matches(options.worktree,entry,folded.manifest.get(entry.path)))kept++;
  else{
   const target=absolute(folder,entry.path);
   await mkdir(dirname(target),{recursive:true});
   await rename(absolute(options.worktree,entry.path),target);
   const wanted=folded.manifest.get(entry.path);
   if(wanted?.operation==="put"){
    const restored=absolute(options.worktree,entry.path);
    await copyFile(blobPathV1(options.storeRoot,wanted.sha256!),restored);
    if(!windows)await chmod(restored,wanted.executable?0o755:0o644);
   }
   moved++;
   sink({version:1,event:"skipped",stage:"settle",path:entry.path,reason:"moved-aside"});
  }
  if((index+1)%500===0)sink({version:1,event:"progress",stage:"settle",done:index+1,total:scan.entries.length});
 }
 sink({version:1,event:"progress",stage:"settle",done:scan.entries.length,total:scan.entries.length});
 return {kept,moved,folder};
}

async function machineOf(worktree:string):Promise<Readonly<{originId:string;groupId:string;storeRoot:string}>|undefined>{
 try{
  const value=JSON.parse(await readFile(join(worktree,".cache","f","machine.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Machine",value,"Closed replica machine required");
  return value as {originId:string;groupId:string;storeRoot:string};
 }catch(error){if(code(error)==="ENOENT")return undefined;throw error;}
}

/** 🎬️ Runs `bootstrap-archive` (needs the machine identity of the group server) or `bootstrap-settle` (needs none) with `--worktree` (default: the repository root), `--store` (default: the machine's store, else the topology mount of this OS) and returns the exit code; a refusal is reported through `say` and exits 1. */
export async function runBootstrapCommandV1(operation:"bootstrap-archive"|"bootstrap-settle",values:Readonly<Record<string,string>>,context:BootstrapContextV1):Promise<number>{
 const worktree=where(context.root,values["--worktree"]??context.root),machine=await machineOf(worktree),given=values["--store"];
 const storeRoot=given!==undefined?where(context.root,given):machine?.storeRoot??readTopologyV1().storeMount[platformV1()];
 const sink=(event:ReplicaEventV1):void=>{context.write(JSON.stringify(event));};
 try{
  if(operation==="bootstrap-archive"){
   if(machine===undefined)throw new BootstrapRefusedV1("identity");
   context.say("bootstrap.archive.start",{worktree,store:storeRoot});
   const result=await writeBootstrapArchiveV1({worktree,storeRoot,originId:machine.originId},context.signal,sink);
   context.say("bootstrap.archive.done",{files:result.files,bytes:context.format(result.bytes),skipped:result.skipped,sha256:result.sha256});
   return 0;
  }
  context.say("bootstrap.settle.start",{worktree,store:storeRoot});
  const result=await settleBootstrapV1({worktree,storeRoot},context.signal,sink);
  context.say("bootstrap.settle.done",{kept:result.kept,moved:result.moved,folder:result.folder});
  return 0;
 }catch(error){
  if(error instanceof BootstrapRefusedV1){context.say(`error.bootstrap.${error.key}`,error.variables);return 1;}
  throw error;
 }
}
