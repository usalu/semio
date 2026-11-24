import {createHash,randomBytes} from "node:crypto";
import {createReadStream} from "node:fs";
import {chmod,lstat,mkdir,open,readdir,readFile,rename,rm,stat,statfs,writeFile,type FileHandle} from "node:fs/promises";
import {dirname,isAbsolute,join,resolve} from "node:path";
import {reclaimLitterV1} from "../../🗜️compaction/🟦️.ts";
import {blobPathV1,copyVerifiedV1,reclaimStagingV1,requireOriginIdV1,storePathsV1,type FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";
import {compileIgnoreV1,gitPrivateV1,hashFileV1,scanSharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../../🚧️boundary/🟦️.ts";
import {platformV1,readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {discardReachablePackV1,packExcludedV1,planPacksV1,writeReachablePackV1,type PackPlanV1} from "./🧺️packs/🟦️.ts";
import {foldStoreV1,requireReplicaShapeV1,storeReachableV1,type ReplicaEventV1,type ReplicaSinkV1} from "../🟦️.ts";

/** 🥾️ The inputs of the archive writer: the worktree to read, the store to write into and the origin that owns the staging names. */
export type BootstrapOptionsV1=Readonly<{worktree:string;storeRoot:string;originId:string}>;

/** 🎚️ How a build decides: `reachable` (default true) packs the git objects reachable from the refs into one new pack instead of copying the existing packs, `force` builds although little changed (never although the disk is short), `driftBytes` is the changed content that justifies a rebuild (default: the larger of 1 GiB and a tenth of the current archive), `maxAgeMilliseconds` rebuilds an archive that old anyway (default 24 h), `graceMilliseconds` is how long a superseded archive stays for readers that are copying it (schema `Retention`, default 1 h) and `pressureGraceMilliseconds` the shorter grace while the volume is short (default 10 min), `reserveBytes` is what must stay free after the write (default: 3 % of the volume, the disk guard, plus 2 GiB) and `space` reports the free and total bytes of the volume of the store (default: `statfs`). */
export type BootstrapPolicyV1=Readonly<{graceMilliseconds?:number;pressureGraceMilliseconds?:number;reachable?:boolean;force?:boolean;driftBytes?:number;maxAgeMilliseconds?:number;reserveBytes?:number;space?:(directory:string)=>Promise<Readonly<{free:number;total:number}>|undefined>;now?:()=>number}>;

/** 📦️ What one archive build wrote: the folded clock, the files and content bytes it holds, the files it skipped, the sha256 of the tar and the tar path. */
export type BootstrapResultV1=Readonly<{hlc:string;files:number;bytes:number;skipped:number;sha256:string;archive:string;outcome:"built"|"unchanged";packs?:PackPlanV1}>;

/** 🧾️ The document `manifest.json` next to the archive; `bytes` is the sum of the file contents, `sha256` is the hash of the whole tar. */
export type BootstrapManifestV1=Readonly<{version:1;hlc:string;files:number;bytes:number;sha256:string;archive:string}>;

/** 🧹️ What a settle did: files that already matched the store, files moved aside and the folder that holds them. */
export type BootstrapSettleV1=Readonly<{kept:number;moved:number;folder:string}>;

/** 🧭️ The context a bootstrap command runs in: the repository root, cancellation, the stdout line sink, the message sink and the byte formatter. */
export type BootstrapContextV1=Readonly<{root:string;signal:AbortSignal;write:(line:string)=>void;say:(key:string,variables?:Readonly<Record<string,string|number>>)=>void;format:(bytes:number)=>string}>;

/** ⛔️ Raised when a bootstrap operation cannot start; `key` names the message `error.bootstrap.<key>`. */
export class BootstrapRefusedV1 extends Error{
 readonly key:"identity"|"empty"|"joined"|"store"|"blob"|"space";
 readonly variables:Readonly<Record<string,string>>;
 constructor(key:"identity"|"empty"|"joined"|"store"|"blob"|"space",variables:Readonly<Record<string,string>>={}){super(`Bootstrap refused: ${key}`);this.name="BootstrapRefusedV1";this.key=key;this.variables=variables;}
}

const BLOCK=512,LIMIT=8**11,CHUNK=1<<20,FIELD=/[^\x20-\x7e]/gu,windows=process.platform==="win32",retryMinutes=2,spaceRetryMinutes=10,keepArchiveMilliseconds=3_600_000,pressureKeepMilliseconds=600_000,minimumWakeMilliseconds=60_000,litterMilliseconds=3_600_000,strayStagingMilliseconds=2*3_600_000,maximumAgeMilliseconds=24*3_600_000,guardFraction=0.03,guardExtraBytes=2**31,minimumDriftBytes=2**30,tarOverhead=1536,archivePattern=/^worktree(-[0-9a-f]{16})?\.tar$/u,partPattern=/^worktree(-[0-9a-f]{16})?\.(?:tar|entries)$/u;
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
export function bootstrapPathsV1(storeRoot:string):Readonly<{directory:string;manifest:string}>{
 const directory=join(storePathsV1(storeRoot).root,"bootstrap");
 return {directory,manifest:join(directory,"manifest.json")};
}

/** 🏷️ The file name of an archive: `worktree-` and the first 16 hex digits of its sha256, so that the name tells the content and a rebuilt archive never overwrites the one a reader is copying. */
export const bootstrapArchiveNameV1=(sha256:string):string=>`worktree-${sha256.slice(0,16)}.tar`;

/** 🔖️ Reads the manifest of the bootstrap archive and the path of the archive it names; `undefined` when there is none. */
export async function readBootstrapV1(storeRoot:string):Promise<Readonly<{manifest:BootstrapManifestV1;archive:string}>|undefined>{
 const paths=bootstrapPathsV1(storeRoot);
 try{
  const manifest=JSON.parse(await readFile(paths.manifest,"utf8")) as BootstrapManifestV1;
  return typeof manifest.archive==="string"&&archivePattern.test(manifest.archive)?{manifest,archive:join(paths.directory,manifest.archive)}:undefined;
 }catch{return undefined;}
}

const entriesOf=(archive:string):string=>archive.replace(/\.tar$/u,".entries");
const entriesLine=(path:string,entry:FleetManifestEntryV1):string=>`${path}\t${entry.sha256}\t${entry.bytes}\t${entry.executable===true?"x":"-"}\n`;

/** 📊️ What the retention sweep left behind: the superseded archive files still inside their grace (`held`) and the bytes it retired. */
export type BootstrapRetentionV1=Readonly<{retired:Readonly<{files:number;bytes:number}>;held:Readonly<{files:number;bytes:number;nextMilliseconds:number|null}>}>;

/** 🧹️ Retires every archive file (`worktree-<16 hex>.tar` or `.entries`) that the manifest does not name once `grace` has passed since it was superseded: the age counts from the later of the file's mtime and the manifest's mtime (the switch), so a crash between the manifest rename and any touch never shortens the grace and an orphan of a crashed build leaks nothing. A file a reader still holds open (Windows) stays and is retried by the next sweep. Schema: `./🧬️schema/🔣️.json#/definitions/Retention`. */
export async function retireSupersededV1(storeRoot:string,options:Readonly<{graceMilliseconds?:number;now?:number}>={}):Promise<BootstrapRetentionV1>{
 const paths=bootstrapPathsV1(storeRoot),grace=options.graceMilliseconds??keepArchiveMilliseconds,now=options.now??Date.now(),current=await readBootstrapV1(storeRoot),keep=new Set(current===undefined?[]:[current.manifest.archive,entriesOf(current.manifest.archive)]),switched=(await stat(paths.manifest).catch(()=>undefined))?.mtimeMs??0;
 let retired={files:0,bytes:0},held={files:0,bytes:0,nextMilliseconds:null as number|null};
 for(const name of await readdir(paths.directory).catch(()=>[] as string[])){
  if(!partPattern.test(name)||keep.has(name))continue;
  const file=join(paths.directory,name),info=await stat(file).catch(()=>undefined);
  if(info===undefined)continue;
  const left=Math.max(info.mtimeMs,switched)+grace-now;
  if(left>0){held={files:held.files+1,bytes:held.bytes+info.size,nextMilliseconds:held.nextMilliseconds===null?left:Math.min(held.nextMilliseconds,left)};continue;}
  if(await rm(file,{force:true}).then(()=>true,()=>false))retired={files:retired.files+1,bytes:retired.bytes+info.size};
  else held={files:held.files+1,bytes:held.bytes+info.size,nextMilliseconds:held.nextMilliseconds};
 }
 return {retired,held};
}

/** 📝️ What the group server reports about its archive in `.cache/f/bootstrap.json` (and `status`): the archive, what the sweep retired and still holds for readers, and the last refusal for lack of space. Schema: `./🧬️schema/🔣️.json#/definitions/BootstrapRecord`. */
export type BootstrapRecordV1=Readonly<{version:1;wallMs:number;archive:string|null;bytes:number;held:Readonly<{files:number;bytes:number}>;retiredBytes:number;refusal:Readonly<{need:string;free:string;reserve:string;wallMs:number}>|null}>;

async function recordBootstrapV1(worktree:string,storeRoot:string,now:number,retired:number,refusal:BootstrapRecordV1["refusal"]):Promise<void>{
 try{
  const current=await readBootstrapV1(storeRoot),swept=await retireSupersededV1(storeRoot,{graceMilliseconds:Number.MAX_SAFE_INTEGER,now}),folder=join(worktree,".cache","f"),file=join(folder,"bootstrap.json"),temp=`${file}.${token()}`,size=current===undefined?0:(await stat(current.archive).catch(()=>undefined))?.size??0;
  const record:BootstrapRecordV1={version:1,wallMs:now,archive:current?.manifest.archive??null,bytes:size,held:{files:swept.held.files,bytes:swept.held.bytes},retiredBytes:retired,refusal};
  await mkdir(folder,{recursive:true});await writeFile(temp,JSON.stringify(record));await rename(temp,file);
 }catch{}
}

/** 🩺️ Reads `.cache/f/bootstrap.json` of a worktree; `undefined` when the machine builds no archive. */
export async function readBootstrapRecordV1(worktree:string):Promise<BootstrapRecordV1|undefined>{
 try{return JSON.parse(await readFile(join(worktree,".cache","f","bootstrap.json"),"utf8")) as BootstrapRecordV1;}catch{return undefined;}
}

async function readEntriesV1(file:string):Promise<Map<string,string>|undefined>{
 try{
  const found=new Map<string,string>();
  for(const line of (await readFile(file,"utf8")).split("\n")){
   const cut=line.indexOf("\t");
   if(cut>0)found.set(line.slice(0,cut),line.slice(cut+1));
  }
  return found;
 }catch{return undefined;}
}

async function volumeV1(directory:string):Promise<Readonly<{free:number;total:number}>|undefined>{
 try{const info=await statfs(directory);return {free:Number(info.bavail)*Number(info.bsize),total:Number(info.blocks)*Number(info.bsize)};}catch{return undefined;}
}

const gib=(bytes:number):string=>(bytes/2**30).toFixed(1);

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

/** 🥾️ Writes `<storeRoot>/v1/bootstrap/worktree-<sha256 prefix>.tar`, `worktree-<same prefix>.entries` and `manifest.json` from the folded store manifest and the local worktree: one pax tar entry per shared file (links stay to `join`), streamed in 1 MiB chunks, every entry verified against its manifest hash while it is written (a file that is missing, busy, changed or different is skipped and the entry is rewound), staged in `<storeRoot>/v1/tmp`. The archive carries no dead or redundant git files: no `tmp_*` or lock file, no `gc.log`, no `multi-pack-index`, and of `.git/objects/pack` one new pack of the objects reachable from the refs (`writeReachablePackV1`, staged in `tmp` and removed after the build; a joiner fetches whatever else it needs from the store), or, when git is missing or fails or `reachable` is off, only the packs of `planPacksV1` (a pack whose objects all lie in a larger kept pack is left out). The extracted tree is a complete repository, `git fsck` confirms. Before it writes anything it (1) sweeps staging files of dead writers (silent for two hours) and archives no manifest names, (2) skips the build and reports `skipped` with reason `unchanged` while the non-pack content that differs from the current archive (its `.entries` sidecar) is below the drift limit and the archive is younger than 24 h (`force` overrides), and (3) estimates the size and refuses with `BootstrapRefusedV1("space")` (event `skipped`, reason `space`) when the volume would keep less than the reserve free; when only the archive that is now being replaced stands in the way (and is older than an hour) it is retired first, so the store never holds more than one archive then. The tar is renamed to its content name first, where no reader looks for it, then the sidecar, and the manifest, which names the tar, is renamed last: that one rename is the only step a reader can observe, so a reader never sees a manifest and an archive that disagree. The archive of the previous manifest stays for an hour for readers that are copying it. */
export async function writeBootstrapArchiveV1(options:BootstrapOptionsV1&BootstrapPolicyV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<BootstrapResultV1>{
 requireOriginIdV1(options.originId);
 const clock=options.now??Date.now,grace=options.graceMilliseconds??keepArchiveMilliseconds;
 const swept=await retireSupersededV1(options.storeRoot,{graceMilliseconds:grace,now:clock()});
 let retiredBytes=swept.retired.bytes;
 if(swept.retired.files>0)sink({version:1,event:"retired",stage:"bootstrap",reason:"superseded",bytes:swept.retired.bytes});
 await reclaimLitterV1(options.storeRoot,{olderThanMilliseconds:litterMilliseconds,now:clock(),signal}).catch(error=>{if(signal?.aborted===true)throw error;});
 const store=storePathsV1(options.storeRoot),paths=bootstrapPathsV1(options.storeRoot),folded=await foldStoreV1(options.storeRoot,signal,sink);
 const plan=await planPacksV1(join(options.worktree,".git","objects","pack"),signal),packed=(path:string):boolean=>path.startsWith(".git/objects/pack/");
 const eligible=[...folded.manifest].filter(([path,entry])=>entry.operation==="put"&&!unsafe(path)&&!gitPrivateV1(path)&&!packExcludedV1(path,plan)).sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
 const plain=eligible.filter(([path])=>!packed(path));
 if(plain.length===0)throw new BootstrapRefusedV1("empty");
 await mkdir(store.tmp,{recursive:true});await mkdir(paths.directory,{recursive:true});
 await reclaimStagingV1(options.storeRoot,{olderThanMilliseconds:strayStagingMilliseconds,now:clock()});
 let previous=await readBootstrapV1(options.storeRoot);
 if(previous!==undefined){
  const info=await stat(previous.archive).catch(()=>undefined),before=info===undefined?undefined:await readEntriesV1(join(paths.directory,entriesOf(previous.manifest.archive))),age=clock()-((await stat(paths.manifest).catch(()=>undefined))?.mtimeMs??0);
  if(info!==undefined&&before!==undefined){
   let drift=0;
   for(const [path,entry] of plain)if(before.get(path)!==`${entry.sha256}\t${entry.bytes}\t${entry.executable===true?"x":"-"}`)drift+=entry.bytes!;
   const limit=options.driftBytes??Math.max(minimumDriftBytes,previous.manifest.bytes/10);
   if(options.force!==true&&drift<limit&&age<(options.maxAgeMilliseconds??maximumAgeMilliseconds)){
    sink({version:1,event:"skipped",stage:"bootstrap",reason:"unchanged",bytes:drift});
    return {hlc:previous.manifest.hlc,files:previous.manifest.files,bytes:previous.manifest.bytes,skipped:0,sha256:previous.manifest.sha256,archive:previous.archive,outcome:"unchanged",packs:plan};
   }
  }
 }
 const guard=async(needed:number):Promise<void>=>{
  const volume=await (options.space??volumeV1)(store.tmp);
  if(volume===undefined)return;
  const reserve=options.reserveBytes??Math.ceil(volume.total*guardFraction)+guardExtraBytes;
  let free=volume.free;
  if(free-needed<reserve){
   const hurried=await retireSupersededV1(options.storeRoot,{graceMilliseconds:options.pressureGraceMilliseconds??pressureKeepMilliseconds,now:clock()});
   if(hurried.retired.files>0){free+=hurried.retired.bytes;retiredBytes+=hurried.retired.bytes;sink({version:1,event:"retired",stage:"bootstrap",reason:"pressure",bytes:hurried.retired.bytes});}
  }
  if(free-needed<reserve&&previous!==undefined){
   const info=await stat(previous.archive).catch(()=>undefined),manifestInfo=await stat(paths.manifest).catch(()=>undefined),side=await stat(join(paths.directory,entriesOf(previous.manifest.archive))).catch(()=>undefined);
   if(info!==undefined&&manifestInfo!==undefined&&clock()-manifestInfo.mtimeMs>grace&&free+info.size+(side?.size??0)-needed>=reserve){
    await rm(paths.manifest,{force:true});await rm(previous.archive,{force:true});await rm(join(paths.directory,entriesOf(previous.manifest.archive)),{force:true});
    free+=info.size+(side?.size??0);
    sink({version:1,event:"retired",stage:"bootstrap",reason:"single-slot",bytes:info.size});
    previous=undefined;
   }
  }
  if(free-needed<reserve){
   sink({version:1,event:"skipped",stage:"bootstrap",reason:"space",bytes:needed});
   await recordBootstrapV1(options.worktree,options.storeRoot,clock(),retiredBytes,{need:gib(needed),free:gib(free),reserve:gib(reserve),wallMs:clock()});
   throw new BootstrapRefusedV1("space",{need:gib(needed),free:gib(free),reserve:gib(reserve)});
  }
 };
 const size=(list:readonly (readonly [string,FleetManifestEntryV1])[]):number=>list.reduce((sum,[,entry])=>sum+entry.bytes!+tarOverhead,2*BLOCK);
 const stem=join(store.tmp,`${options.originId}-bootstrap-${token()}`),archive=`${stem}.tar`,manifest=`${stem}.json`,side=`${stem}.entries`,staged=`${stem}.pk`;
 const sources=new Map<string,string>();
 let wanted=eligible,reach:Awaited<ReturnType<typeof writeReachablePackV1>>;
 try{
  if(options.reachable!==false&&await stat(join(options.worktree,".git","HEAD")).then(()=>true,()=>false)){
   await guard(size(plain)+2**30);
   sink({version:1,event:"progress",stage:"bootstrap-pack",done:0,total:1});
   reach=await writeReachablePackV1(options.worktree,staged,signal);
   sink({version:1,event:"progress",stage:"bootstrap-pack",done:1,total:1,...reach===undefined?{}:{bytes:reach.bytes}});
  }
  if(reach!==undefined){
   const extra:[string,FleetManifestEntryV1][]=[];
   for(const [suffix,file] of [["pack",reach.pack],["idx",reach.idx]] as const){
    const path=`.git/objects/pack/pack-${reach.hash}.${suffix}`,info=await stat(file),hashed=await hashFileV1(file);
    sources.set(path,file);
    extra.push([path,{operation:"put",bytes:info.size,sha256:hashed.sha256,executable:false} as unknown as FleetManifestEntryV1]);
   }
   wanted=[...plain,...extra].sort((left,right)=>left[0]<right[0]?-1:left[0]>right[0]?1:0);
   await guard(size(wanted)+reach.bytes);
  }else await guard(size(wanted));
  let files=0,bytes=0,skipped=0,sha256="";
  const lines:string[]=[];
  try{
   const handle=await open(archive,"w");
   try{
    let position=0;
    for(const [index,[path,entry]] of wanted.entries()){
     signal?.throwIfAborted();
     const outcome=await copyEntry(handle,sources.get(path)??absolute(options.worktree,path),path,entry,position,signal);
     if(typeof outcome==="number"){position=outcome;files++;bytes+=entry.bytes!;if(!packed(path))lines.push(entriesLine(path,entry));}
     else{skipped++;sink({version:1,event:"skipped",stage:"bootstrap",path,reason:outcome});}
     if((index+1)%500===0)sink({version:1,event:"progress",stage:"bootstrap",done:index+1,total:wanted.length,bytes});
    }
    await put(handle,tarEndV1(),position);
    await handle.truncate(position+2*BLOCK);await handle.sync();
   }finally{await handle.close();}
   sha256=(await hashFileV1(archive)).sha256;
   const name=bootstrapArchiveNameV1(sha256),document:BootstrapManifestV1={version:1,hlc:folded.hlc,files,bytes,sha256,archive:name};
   const sidecar=await open(side,"w");
   try{await sidecar.writeFile(lines.join(""));await sidecar.sync();}finally{await sidecar.close();}
   const out=await open(manifest,"w");
   try{await out.writeFile(JSON.stringify(document));await out.sync();}finally{await out.close();}
   await rename(archive,join(paths.directory,name));
   await rename(side,join(paths.directory,entriesOf(name)));
   await rename(manifest,paths.manifest);
   const after=await retireSupersededV1(options.storeRoot,{graceMilliseconds:grace,now:clock()});
   retiredBytes+=after.retired.bytes;
  }catch(error){await rm(archive,{force:true});await rm(manifest,{force:true});await rm(side,{force:true});throw error;}
  sink({version:1,event:"progress",stage:"bootstrap",done:wanted.length,total:wanted.length,bytes});
  await recordBootstrapV1(options.worktree,options.storeRoot,clock(),retiredBytes,null);
  return {hlc:folded.hlc,files,bytes,skipped,sha256,archive:join(paths.directory,bootstrapArchiveNameV1(sha256)),outcome:"built",packs:plan};
 }finally{await discardReachablePackV1(staged);}
}

/** 🔄️ Rebuilds the archive now and then every `minutes` unless `writeBootstrapArchiveV1` finds it unchanged, until `signal` aborts; a failed build is reported as an `error` event and retried after at most two minutes (ten when the disk is too full), so the relay of a group server never stops for it and a store that is still empty gets its archive soon after the first files arrive. Staging files of dead writers (silent for two hours) are removed by every round, and every round and the wake-up at the end of each grace retire the archives the manifest no longer names (`retireSupersededV1`), so a crash can never leak an archive. */
export async function refreshBootstrapV1(options:BootstrapOptionsV1&BootstrapPolicyV1&Readonly<{minutes:number}>,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 while(!signal.aborted){
  let minutes=options.minutes;
  try{await writeBootstrapArchiveV1({...options,force:false},signal,sink);}
  catch(error){
   if(signal.aborted)break;
   minutes=Math.min(minutes,error instanceof BootstrapRefusedV1&&error.key==="space"?spaceRetryMinutes:retryMinutes);
   sink({version:1,event:"error",stage:"bootstrap",message:error instanceof BootstrapRefusedV1?`${error.message} ${JSON.stringify(error.variables)}`:error instanceof Error?error.message:String(error)});
  }
  const clock=options.now??Date.now,swept=await retireSupersededV1(options.storeRoot,{graceMilliseconds:options.graceMilliseconds,now:clock()}),wait=Math.min(minutes*60_000,Math.max(minimumWakeMilliseconds,(swept.held.nextMilliseconds??Infinity)+1000));
  if(swept.retired.files>0)sink({version:1,event:"retired",stage:"bootstrap",reason:"superseded",bytes:swept.retired.bytes});
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,wait);signal.addEventListener("abort",finish,{once:true});
  });
 }
}

async function matches(worktree:string,entry:FleetScanEntryV1,wanted:FleetManifestEntryV1|undefined):Promise<boolean>{
 if(wanted?.operation==="link")return entry.kind==="link"&&entry.target===wanted.target;
 if(wanted?.operation==="put"&&entry.kind==="file"&&entry.bytes===wanted.bytes&&(windows||entry.executable===wanted.executable))return (await hashFileV1(absolute(worktree,entry.path))).sha256===wanted.sha256;
 return false;
}

/** 🔤️ Renames every shared name that is not NFC (macOS `tar` extracts decomposed names) to its NFC spelling, parents before their children, until none is left; a name whose NFC spelling already exists beside it stays and remains refused by the path policy. Every rename is reported as a `skipped` event of `stage` with the reason `renamed-to-nfc` and the new path, and the new paths are returned. `bootstrap-settle` and `join --adopt` both start with it. */
export async function normaliseNamesV1(worktree:string,ignore:FleetIgnoreV1,signal:AbortSignal|undefined,sink:ReplicaSinkV1,stage:"settle"|"adopt"):Promise<readonly string[]>{
 const done:string[]=[];
 for(let round=0;round<64;round++){
  const todo=(await scanSharedV1(worktree,ignore,signal)).refused.filter(item=>item.reason==="not-nfc").map(item=>item.path);
  let renamed=0;
  for(const path of todo){
   const clean=path.normalize("NFC"),directory=dirname(absolute(worktree,path)),siblings=await readdir(directory).catch(()=>[] as string[]);
   if(siblings.includes(clean.slice(clean.lastIndexOf("/")+1)))continue;
   await rename(absolute(worktree,path),absolute(worktree,clean));
   renamed++;done.push(clean);
   sink({version:1,event:"skipped",stage,path:clean,reason:"renamed-to-nfc"});
  }
  if(renamed===0)break;
 }
 return done;
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
 await normaliseNamesV1(options.worktree,ignore,signal,sink,"settle");
 const scan=await scanSharedV1(options.worktree,ignore,signal),folder=join(options.worktree,".cache","f","lost",`bootstrap-${(options.now??Date.now)()}`);
 let kept=0,moved=0;
 for(const [index,entry] of scan.entries.entries()){
  signal?.throwIfAborted();
  if(await matches(options.worktree,entry,folded.manifest.get(entry.path)))kept++;
  else{
   const target=absolute(folder,entry.path),wanted=folded.manifest.get(entry.path),restored=absolute(options.worktree,entry.path),staged=wanted?.operation==="put"?join(options.worktree,".cache","f","tmp",`settle-${token()}`):undefined;
   if(staged!==undefined){
    try{
     await mkdir(dirname(staged),{recursive:true});
     if(await copyVerifiedV1(blobPathV1(options.storeRoot,wanted!.sha256!),staged,wanted!.sha256!)==="corrupt")throw new BootstrapRefusedV1("blob",{path:entry.path});
     if(!windows)await chmod(staged,wanted!.executable?0o755:0o644);
    }catch(error){await rm(staged,{force:true});throw error;}
   }
   await mkdir(dirname(target),{recursive:true});
   await rename(restored,target);
   if(staged!==undefined)await rename(staged,restored);
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
   const result=await writeBootstrapArchiveV1({worktree,storeRoot,originId:machine.originId,force:values["--force"]!==undefined},context.signal,sink);
   if(result.outcome==="unchanged"){context.say("bootstrap.archive.unchanged",{files:result.files,bytes:context.format(result.bytes),sha256:result.sha256});return 0;}
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
