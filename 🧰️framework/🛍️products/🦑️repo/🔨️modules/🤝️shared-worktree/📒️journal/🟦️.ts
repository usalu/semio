import {createHash,randomBytes} from "node:crypto";
import {constants,createReadStream} from "node:fs";
import {chmod,copyFile,lstat,mkdir,open,readFile,readdir,rename,stat,unlink,utimes} from "node:fs/promises";
import {dirname,join,sep} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {yieldingV1} from "../🛡️resilience/⏳️yielding/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {ownProcessIdentityV1,processHolderAliveV1} from "../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

/** 🧾️ One journal line: a put, delete or link of one shared path, ordered by its hybrid logical clock. */
export type FleetEventV1=Readonly<{version:1;hlc:string;operation:"put"|"delete"|"link";path:string;sha256?:string;bytes?:number;executable?:boolean;target?:string;base:string|null}>;

/** 🧭️ The published extent of one origin: readers trust the first `bytes` bytes of `segment` and every earlier segment. */
export type FleetHeadV1=Readonly<{version:1;segment:number;bytes:number;events:number;hlc:string}>;

/** 📍️ The next unread position inside one origin's segments. */
export type FleetCursorV1=Readonly<{segment:number;bytes:number}>;

/** ⏱️ The last issued or observed hybrid logical clock value. */
export type FleetClockV1=Readonly<{ms:number;counter:number}>;

/** 🗃️ The winning state of one path in a manifest; a delete is a tombstone. */
export type FleetManifestEntryV1=Readonly<{hlc:string;operation:"put"|"delete"|"link";sha256?:string;bytes?:number;executable?:boolean;target?:string}>;

/** 🗂️ The fixed directories of a store below `<storeRoot>/v1`. */
export type StorePathsV1=Readonly<{root:string;origins:string;blobs:string;tmp:string;snapshots:string;heads:string}>;

const snapshotChunk=1<<20,headRevalidateMilliseconds=5000,segmentBytes=4*1024*1024,segmentEvents=4096,futureMilliseconds=60_000,maximumCounter=999_999,maximumMilliseconds=9_999_999_999_999;
const hlcPattern=/^([0-9]{13})-([0-9]{6})-([a-z0-9][a-z0-9-]{0,39})$/u,originPattern=/^[a-z0-9][a-z0-9-]{0,39}$/u,hexPattern=/^[0-9a-f]{64}$/u;
const verifiedBlobs=new Set<string>(),validators=new Map<string,CompiledJsonSchemaSubset>(),resolvedStores=new Map<string,StorePathsV1>(),knownHeads=new Map<string,Readonly<{signature:string;at:number;text:string;head:FleetHeadV1}>>();
const check=(definition:string,value:unknown,message:string):void=>{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
};
const digit=(code:number):boolean=>code>=48&&code<=57;
const lowerAlnum=(code:number):boolean=>digit(code)||(code>=97&&code<=122);
const isShaV1=(value:unknown):boolean=>{
 if(typeof value!=="string"||value.length!==64)return false;
 for(let at=0;at<64;at++){const code=value.charCodeAt(at);if(!digit(code)&&!(code>=97&&code<=102))return false;}
 return true;
};
const isHlcV1=(value:unknown):boolean=>{
 if(typeof value!=="string"||value.length<22||value.length>61||value.charCodeAt(13)!==45||value.charCodeAt(20)!==45)return false;
 for(let at=0;at<13;at++)if(!digit(value.charCodeAt(at)))return false;
 for(let at=14;at<20;at++)if(!digit(value.charCodeAt(at)))return false;
 if(!lowerAlnum(value.charCodeAt(21)))return false;
 for(let at=22;at<value.length;at++){const code=value.charCodeAt(at);if(!lowerAlnum(code)&&code!==45)return false;}
 return true;
};
const isCountV1=(value:unknown):boolean=>typeof value==="number"&&Number.isInteger(value)&&value>=0&&value<=Number.MAX_SAFE_INTEGER;
const isBaseV1=(value:unknown):boolean=>value===null||isShaV1(value);
const characters=(value:string):number=>{
 let length=value.length;
 for(let at=0;at+1<value.length;at++){const high=value.charCodeAt(at);if(high>=0xd800&&high<=0xdbff){const low=value.charCodeAt(at+1);if(low>=0xdc00&&low<=0xdfff){length--;at++;}}}
 return length;
};
/** 🔡️ Hand-written twin of the `Path` definition. */
export const isPathV1=(value:unknown):boolean=>{
 if(typeof value!=="string"||value.length===0)return false;
 let part=0;
 for(let at=0;at<value.length;at++){
  const code=value.charCodeAt(at);
  if(code===47){if(at===part)return false;part=at+1;}
  else if(code===92||code===0)return false;
 }
 return part<value.length&&(value.length<=1024||characters(value)<=1024);
};
const isTargetV1=(value:unknown):boolean=>typeof value==="string"&&value.length>0&&(value.length<=4096||characters(value)<=4096);
const isRecord=(value:unknown):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value);
const sizeOf=(value:Record<string,unknown>):number=>Object.keys(value).length;

/** ☑️ Hand-written twin of the `Event` definition of the schema (same verdict for every value, proven against the generic validator and the Python oracle); the schema stays the source of truth and the slow generic check only names the error. */
export function isEventV1(value:unknown):boolean{
 if(!isRecord(value)||value.version!==1||!isHlcV1(value.hlc)||!isPathV1(value.path)||!isBaseV1(value.base))return false;
 const size=sizeOf(value);
 switch(value.operation){
  case "put":return size===8&&isShaV1(value.sha256)&&isCountV1(value.bytes)&&typeof value.executable==="boolean";
  case "delete":return size===5;
  case "link":return size===6&&isTargetV1(value.target);
  default:return false;
 }
}

/** ✔️ Hand-written twin of the `ManifestEntry` definition. */
export function isManifestEntryV1(value:unknown):boolean{
 if(!isRecord(value)||!isHlcV1(value.hlc))return false;
 const size=sizeOf(value);
 switch(value.operation){
  case "put":return size===5&&isShaV1(value.sha256)&&isCountV1(value.bytes)&&typeof value.executable==="boolean";
  case "delete":return size===2;
  case "link":return size===3&&isTargetV1(value.target);
  default:return false;
 }
}

/** 🔖️ Hand-written twin of the `Head` definition. */
export function isHeadV1(value:unknown):boolean{
 return isRecord(value)&&value.version===1&&isCountV1(value.segment)&&isCountV1(value.bytes)&&isCountV1(value.events)&&isHlcV1(value.hlc)&&sizeOf(value)===5;
}

const dotted=/(?:^|\/)\.\.?(?:\/|$)/u;
const missing=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";
const exists=async(file:string):Promise<boolean>=>{try{await stat(file);return true;}catch(error){if(missing(error))return false;throw error;}};

/** 🔄️ Whether a read failed because the file was replaced under the reader: a store client still holds the handle of the previous file while another machine renames a new one into place, which a POSIX client reports as `ESTALE` and the Windows NFS client as `EBADF`. */
export const replacedUnderReaderV1=(error:unknown):boolean=>{const code=(error as {code?:string}).code;return code==="ESTALE"||code==="EBADF";};

/** ⏳️ How often a read that met a replacement is repeated and how long it waits in between: the Windows NFS client answers a replaced file with its old handle for a few seconds. */
export const REPLACED_READ_V1=Object.freeze({attempts:12,delayMilliseconds:500});

/** 🪢️ Reads a store file that other machines replace as a whole: a read that meets a replacement is repeated; `undefined` when the file is missing or every attempt met a replacement (the caller reads again at its next poll), and every other failure is thrown. */
export async function readThroughReplacementV1<T>(read:()=>Promise<T>,wait:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))):Promise<T|undefined>{
 for(let attempt=1;;attempt++){
  try{return await read();}catch(error){
   if(missing(error))return undefined;
   if(!replacedUnderReaderV1(error))throw error;
   if(attempt>=REPLACED_READ_V1.attempts)return undefined;
   await wait(REPLACED_READ_V1.delayMilliseconds);
  }
 }
}
const segmentName=(segment:number):string=>`${String(segment).padStart(12,"0")}.jsonl`;
const byKey=(left:readonly [string,unknown],right:readonly [string,unknown]):number=>left[0]<right[0]?-1:left[0]>right[0]?1:0;
const stage=(tmp:string,originId:string):string=>join(tmp,`${originId}-${process.pid}-${ownProcessIdentityV1().startMs}-${randomBytes(8).toString("hex")}`);

/** 🧹️ Reclaims staging files that a crash (`kill -9`) left in `<store>/v1/tmp`; returns how many files and bytes were removed. With `originId` it removes that origin's own leftovers at once: names `<originId>-<pid>-<hex>` whose process is dead and the older `<originId>-<hex>` names; a name of a live process (a `compact` running beside the daemon) stays. With `olderThanMilliseconds` it removes any file whose newest timestamp (modification or inode change) is older than that, which also covers the relay's files and the ones of other origins, and must be far beyond the longest write; `.nfs*` placeholders stay. A store without `tmp` yields nothing and is never created. */
export async function reclaimStagingV1(storeRoot:string,options:Readonly<{originId?:string;olderThanMilliseconds?:number;now?:number}>={}):Promise<Readonly<{files:number;bytes:number}>>{
 const {tmp}=storePathsV1(storeRoot),own=options.originId===undefined?undefined:new RegExp(`^${options.originId}-([0-9]+)-([0-9]+)-[0-9a-f]{16}$`,"u"),now=options.now??Date.now();
 let files=0,bytes=0;
 for(const name of await readdir(tmp).catch(()=>[] as string[])){
  if(name.startsWith(".nfs"))continue;
  const found=own?.exec(name),file=join(tmp,name),info=await lstat(file).catch(()=>undefined);
  if(info===undefined||!info.isFile())continue;
  const dead=found!==undefined&&found!==null&&!processHolderAliveV1({pid:Number(found[1]),startMs:Number(found[2])});
  const old=options.olderThanMilliseconds!==undefined&&now-Math.max(info.mtimeMs,info.ctimeMs)>options.olderThanMilliseconds;
  if(!dead&&!old)continue;
  await unlink(file).then(()=>{files++;bytes+=info.size;},()=>{});
 }
 return {files,bytes};
}

/** ⏰️ Issues the next clock value for a local event: `max(wall, last.ms)`, counter +1 when the millisecond did not advance. */
export function tickClockV1(last:FleetClockV1,wallMs:number):FleetClockV1{
 if(!Number.isSafeInteger(wallMs)||wallMs<0)throw Error("Fleet wall clock must be a non-negative integer");
 const ms=Math.max(wallMs,last.ms),counter=ms>last.ms?0:last.counter+1;
 if(ms>maximumMilliseconds||counter>maximumCounter)throw Error("Fleet clock exhausted");
 return {ms,counter};
}

/** 🔀️ Folds a received hlc into the clock (`max`) and refuses one more than 60 seconds ahead of the wall clock. */
export function mergeClockV1(last:FleetClockV1,receivedHlc:string,wallMs:number):FleetClockV1{
 const received=parseHlcV1(receivedHlc);
 if(received.ms>wallMs+futureMilliseconds)throw Error(`Fleet hlc is ${received.ms-wallMs} ms ahead of the wall clock: ${receivedHlc}`);
 return received.ms>last.ms||(received.ms===last.ms&&received.counter>last.counter)?{ms:received.ms,counter:received.counter}:{ms:last.ms,counter:last.counter};
}

/** 🖨️ Renders a clock value as the fixed-width hlc string that sorts like the total order. */
export function formatHlcV1(clock:FleetClockV1,originId:string):string{
 requireOriginIdV1(originId);
 if(!Number.isSafeInteger(clock.ms)||clock.ms<0||clock.ms>maximumMilliseconds||!Number.isSafeInteger(clock.counter)||clock.counter<0||clock.counter>maximumCounter)throw Error("Fleet clock out of range");
 return `${String(clock.ms).padStart(13,"0")}-${String(clock.counter).padStart(6,"0")}-${originId}`;
}

/** 🔬️ Splits an hlc string into its milliseconds, counter and origin. */
export function parseHlcV1(hlc:string):Readonly<{ms:number;counter:number;originId:string}>{
 const match=typeof hlc==="string"?hlcPattern.exec(hlc):null;
 if(!match)throw Error(`Closed fleet hlc required: ${String(hlc)}`);
 return {ms:Number(match[1]),counter:Number(match[2]),originId:match[3]!};
}

/** 🏆️ Decides whether a candidate beats the current state of a path: the greater hlc string wins, equal never wins. */
export function winsV1(candidateHlc:string,currentHlc:string|undefined):boolean{
 return currentHlc===undefined||candidateHlc>currentHlc;
}

/** 🛡️ Validates one event against the closed schema and refuses paths with `.` or `..` components. */
export function requireEventV1(value:unknown):FleetEventV1{
 if(!isEventV1(value)){check("Event",value,"Closed fleet event required");throw Error("Closed fleet event required");}
 const event=value as FleetEventV1;
 if(dotted.test(event.path))throw Error("Closed fleet event required: path must stay relative");
 return event;
}

/** 🆔️ Admits only a lowercase ASCII origin id of at most 40 characters. */
export function requireOriginIdV1(value:string):string{
 if(typeof value!=="string"||!originPattern.test(value))throw Error(`Closed fleet origin id required: ${String(value)}`);
 return value;
}

/** 🗂️ Resolves the fixed directories of a store below `<storeRoot>/v1`. */
export function storePathsV1(storeRoot:string):StorePathsV1{
 const known=resolvedStores.get(storeRoot);
 if(known!==undefined)return known;
 const root=join(storeRoot,"v1"),paths=Object.freeze({root,origins:join(root,"origins"),blobs:join(root,"blobs"),tmp:join(root,"tmp"),snapshots:join(root,"snapshots"),heads:join(root,"heads")});
 if(resolvedStores.size>=64)resolvedStores.clear();
 resolvedStores.set(storeRoot,paths);
 return paths;
}

/** 🧱️ Resolves the immutable blob file of a content hash. */
export function blobPathV1(storeRoot:string,sha256:string):string{
 if(typeof sha256!=="string"||!hexPattern.test(sha256))throw Error(`Closed fleet sha256 required: ${String(sha256)}`);
 return join(storePathsV1(storeRoot).blobs,sha256.slice(0,2),sha256);
}

async function renameWithRetry(from:string,to:string):Promise<void>{
 for(let attempt=0;;attempt++){
  try{await rename(from,to);return;}
  catch(error){
   const name=(error as {code?:string}).code;
   if((name!=="EPERM"&&name!=="EBUSY"&&name!=="EACCES")||attempt>=4)throw error;
   await new Promise<void>(done=>{setTimeout(done,25*2**attempt);});
  }
 }
}

const seenRoots=new Set<string>(),knownOrigins=new Map<string,{signature:string;at:number;names:readonly string[]}>();

/** 👁️ Remembers that this process has seen the store root `v1` exist; from then on a missing `v1` is an unmounted share, not a fresh store. */
export function markStoreSeenV1(storeRoot:string):void{
 if(seenRoots.size>=64)seenRoots.clear();
 seenRoots.add(storePathsV1(storeRoot).root);
}

/** 📂️ Creates a directory below the store root `v1` one level at a time. A store root that this process has seen is never created again: when the share has vanished (unmounted, mount point left as an empty local directory) the write fails with ENOENT instead of rebuilding the store as a local copy. The first write into a store this process has never seen creates it. */
export async function ensureStoreDirectoryV1(storeRoot:string,directory:string):Promise<void>{
 const root=storePathsV1(storeRoot).root;
 const make=async (path:string):Promise<void>=>{
  if(path===root){if(seenRoots.has(root))await stat(root);else{await mkdir(root,{recursive:true});markStoreSeenV1(storeRoot);}return;}
  try{await mkdir(path);}
  catch(error){
   const kind=(error as {code?:string}).code;
   if(kind==="EEXIST")return;
   if(kind==="ENOENT"&&path.startsWith(root+sep)){await make(dirname(path));await make(path);return;}
   throw error;
  }
 };
 await make(directory);
}

/** 🧹️ Replaces a file of the own origin through tmp + fsync + rename. A target that keeps refusing to be replaced (`EPERM`/`EACCES`: a read-only file on Windows, a stale file node of the Windows NFS client that refused `attest.json` for hours) is first moved into `tmp` and the rename is tried once more, so one stuck file cannot silence the origin; readers already treat a missing file as not yet written. See https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-movefileexw */
async function replaceFile(tmp:string,originId:string,target:string,text:string):Promise<void>{
 await ensureStoreDirectoryV1(dirname(dirname(tmp)),tmp);
 const temporary=stage(tmp,originId);
 try{
  const handle=await open(temporary,"w");
  try{await handle.writeFile(text);await handle.sync();}finally{await handle.close();}
  try{await renameWithRetry(temporary,target);}
  catch(error){
   const code=(error as {code?:string}).code;
   if(code!=="EPERM"&&code!=="EACCES")throw error;
   const displaced=`${stage(tmp,originId)}-displaced`;
   await rename(target,displaced);
   await rename(temporary,target);
   await chmod(displaced,0o666).catch(()=>{});
   await unlink(displaced).catch(()=>{});
  }
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** 🪪️ Atomically replaces one small file `<store>/v1/origins/<originId>/<name>` of the own origin (tmp + fsync + rename, creating the origin directory); a vanished store that this process has seen is not recreated. */
export async function replaceOriginFileV1(storeRoot:string,originId:string,name:string,text:string):Promise<void>{
 requireOriginIdV1(originId);
 const {origins,tmp}=storePathsV1(storeRoot),directory=join(origins,originId);
 await ensureStoreDirectoryV1(storeRoot,directory);
 await replaceFile(tmp,originId,join(directory,name),text);
}

const copyChunk=1<<20,cloneUnsupported=new Set(["ENOTSUP","EOPNOTSUPP","EXDEV","ENOSYS","EINVAL","ENOTTY","EPERM","EACCES","UNKNOWN"]);

/** 🐘️ From this size on a copy first tries a copy-on-write clone (`COPYFILE_FICLONE_FORCE`, only on one volume) and falls back to a streamed copy; below it a copy is always streamed. */
export const largeBlobBytesV1=64*1024*1024;

/** 🪛️ Options of one copy: `signal` cancels between chunks, `progress` reports bytes done of total, `sync` fsyncs the copy before it is closed (a function decides from the sha256 of the copied bytes, so a copy that is going to be dropped is not synced), `mode` is the permission of the copy, `clone` allows the clone fast path for files of at least `largeBlobBytesV1` that share a volume with the target directory. */
export type FleetCopyOptionsV1=Readonly<{signal?:AbortSignal;progress?:(done:number,total:number)=>void;sync?:boolean|((sha256:string)=>Promise<boolean>);mode?:number;clone?:boolean;flushBytes?:number}>;

/** 🔣️ What one pass saw: the sha256 of the bytes that were copied, their count, and whether the source stayed unchanged (size and modification time equal before and after, bytes read equal to the size). */
export type FleetCopyResultV1=Readonly<{sha256:string;bytes:number;stable:boolean}>;

const ensured=new Set<string>();
async function ensureOnce(storeRoot:string,directory:string):Promise<void>{
 if(ensured.has(directory))return;
 await ensureStoreDirectoryV1(storeRoot,directory);
 if(ensured.size>=4096)ensured.clear();
 ensured.add(directory);
}
async function inDirectory<T>(storeRoot:string,directory:string,work:()=>Promise<T>):Promise<T>{
 await ensureOnce(storeRoot,directory);
 try{return await work();}
 catch(error){
  if(!missing(error))throw error;
  ensured.delete(directory);
  await ensureStoreDirectoryV1(storeRoot,directory);
  return work();
 }
}

async function writeFully(handle:Awaited<ReturnType<typeof open>>,buffer:Buffer,length:number):Promise<void>{
 let done=0;
 while(done<length){const {bytesWritten}=await handle.write(buffer,done,length-done);done+=bytesWritten;}
}

async function cloneFile(source:string,target:string,size:number):Promise<boolean>{
 if(size<largeBlobBytesV1)return false;
 try{
  if((await stat(source)).dev!==(await stat(dirname(target))).dev)return false;
  await copyFile(source,target,constants.COPYFILE_FICLONE_FORCE);
  return true;
 }catch(error){
  await unlink(target).catch(()=>{});
  if(cloneUnsupported.has((error as {code?:string}).code??""))return false;
  throw error;
 }
}

/** 🧫️ Copies `source` to the new file `target` (every 16 MiB the copy yields to the event loop with a macrotask, so that a multi-gigabyte copy never starves timers, the heartbeat or the watchdog's wake-ups) (created exclusively; a failed or unstable copy removes it) in one pass: the bytes are read once in 1 MiB chunks, hashed while they are written (reads and writes overlap), and the sha256 of exactly what was written comes back with the stability verdict. A file that is missing throws `ENOENT`. Memory is two chunks whatever the size. */
export async function copyHashedV1(source:string,target:string,options:FleetCopyOptionsV1={}):Promise<FleetCopyResultV1>{
 const {signal,progress,sync=false,mode=0o644,flushBytes}=options,input=await open(source,"r");
 try{
  const before=await input.stat(),hash=createHash("sha256");
  let bytes=0,digest="",turn=0;
  const breathe=async():Promise<void>=>{if(++turn%16===0)await new Promise<void>(done=>setImmediate(done));};
  const settle=async(handle:Awaited<ReturnType<typeof open>>):Promise<void>=>{digest=hash.copy().digest("hex");if(typeof sync==="function"?await sync(digest):sync)await handle.sync();};
  try{
   signal?.throwIfAborted();
   if(options.clone===true&&await cloneFile(source,target,before.size)){
    await chmod(target,0o600);
    const copy=await open(target,"r+");
    try{
     const chunk=Buffer.allocUnsafe(copyChunk);
     for(;;){
      signal?.throwIfAborted();
      const {bytesRead}=await copy.read(chunk,0,copyChunk,null);
      if(bytesRead===0)break;
      hash.update(chunk.subarray(0,bytesRead));bytes+=bytesRead;progress?.(bytes,before.size);await breathe();
     }
     await copy.chmod(mode);
     await settle(copy);
    }finally{await copy.close();}
    const now=new Date();await utimes(target,now,now);
   }else{
    const output=await open(target,"wx",mode);
    try{
     const buffers=[Buffer.allocUnsafe(copyChunk),Buffer.allocUnsafe(copyChunk)];
     let flip=0,pending:Promise<void>=Promise.resolve(),unflushed=0;
     try{
      for(;;){
       signal?.throwIfAborted();
       const buffer=buffers[flip]!,{bytesRead}=await input.read(buffer,0,copyChunk,null);
       if(bytesRead===0)break;
       hash.update(buffer.subarray(0,bytesRead));
       await pending;
       pending=writeFully(output,buffer,bytesRead);pending.catch(()=>{});
       flip^=1;bytes+=bytesRead;unflushed+=bytesRead;progress?.(bytes,before.size);await breathe();
       if(flushBytes!==undefined&&unflushed>=flushBytes){await pending;await output.datasync();unflushed=0;progress?.(bytes,before.size);}
      }
      await pending;
     }catch(error){await pending.catch(()=>{});throw error;}
     await output.chmod(mode);
     await settle(output);
    }finally{await output.close();}
   }
  }catch(error){await unlink(target).catch(()=>{});throw error;}
  const after=await input.stat(),stable=before.size===after.size&&before.mtimeMs===after.mtimeMs&&bytes===after.size;
  if(!stable)await unlink(target).catch(()=>{});
  return {sha256:digest,bytes,stable};
 }finally{await input.close();}
}

/** 🔏️ Copies like `copyHashedV1` and compares the sha256 of the copied bytes with `expected` in the same pass: `"corrupt"` (the copy is removed) when they differ, so a file whose content does not match its name never stays at `target`; a missing source throws `ENOENT`. */
export async function copyVerifiedV1(source:string,target:string,expected:string,options:FleetCopyOptionsV1={}):Promise<"ok"|"corrupt">{
 const result=await copyHashedV1(source,target,options);
 if(result.sha256===expected)return "ok";
 await unlink(target).catch(()=>{});
 return "corrupt";
}

async function intact(target:string,sha256:string,alive?:()=>void):Promise<boolean>{
 if(verifiedBlobs.has(target))return true;
 const hash=createHash("sha256");
 try{for await(const chunk of createReadStream(target)){hash.update(chunk);alive?.();}}catch(error){if(missing(error))return false;throw error;}
 if(hash.digest("hex")!==sha256)return false;
 if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();
 verifiedBlobs.add(target);
 return true;
}

/** 🤚️ Marks an existing blob as in use by setting its modification time to now and tells whether it was still there: a blob reclaim moves unreferenced blobs aside and puts one back when its modification time changed, so a publisher that finds its blob already stored must touch it before it appends the event, and must store the blob again when the touch finds it gone. */
export async function touchBlobV1(target:string):Promise<boolean>{
 const now=new Date();
 try{await utimes(target,now,now);return true;}catch(error){return !missing(error);}
}

/** 🚰️ How many bytes a blob upload writes before it flushes them to the store (32 MiB): a multi-gigabyte pack is paced by the store instead of ending in one flush of minutes that blocks every other call of the mount, and the progress and `alive` callbacks of a long upload never stay silent for longer than one such flush. */
export const storeFlushBytesV1=32*1024*1024;

/** 🔍️ Hashes a local file in one read and tells whether it stayed the same while it was read; `alive` is called at every chunk. */
async function hashLocalV1(source:string,signal?:AbortSignal,alive?:()=>void):Promise<Readonly<{sha256:string;bytes:number;stable:boolean}>>{
 const before=await stat(source),hash=createHash("sha256");
 let bytes=0;
 for await(const chunk of createReadStream(source,{highWaterMark:1<<20})){signal?.throwIfAborted();hash.update(chunk);bytes+=(chunk as Buffer).length;alive?.();await new Promise<void>(done=>setImmediate(done));}
 const after=await stat(source);
 return {sha256:hash.digest("hex"),bytes,stable:before.size===after.size&&before.mtimeMs===after.mtimeMs&&bytes===after.size};
}

/** 🛢️ Stores a file as an immutable blob in one pass and returns its sha256 (the name comes from the bytes that were written, so name and content cannot differ): the source is read once, staged in `tmp` while hashing, fsynced, then renamed to its name, or dropped when that blob already exists. With `known` (the hash a caller already holds) a blob that exists is not read at all, and a source whose bytes hash differently is reported unstable. A read-only source is published too. `stable` false means the source changed while it was read and nothing was stored. `alive` is called at every chunk that is copied, flushed or verified, so a caller can tell a slow upload from a stalled store. A source of at least `largeBlobBytesV1` without `known` is hashed locally first: when the store already holds that blob (a pack another machine uploaded and this one republishes after a state reset), nothing is staged or copied. */
export async function storeBlobV1(storeRoot:string,originId:string,source:string,options:Readonly<{known?:Readonly<{sha256:string;bytes:number}>;signal?:AbortSignal;progress?:(done:number,total:number)=>void;alive?:()=>void}>={}):Promise<Readonly<{sha256:string;bytes:number;stable:boolean;created:boolean}>>{
 requireOriginIdV1(originId);
 const {tmp}=storePathsV1(storeRoot),{known}=options;
 if(known!==undefined){
  const target=blobPathV1(storeRoot,known.sha256);
  if(await exists(target)&&await intact(target,known.sha256,options.alive)&&await touchBlobV1(target))return {...known,stable:true,created:false};
 }
 if(known===undefined){
  const info=await stat(source).catch(()=>undefined);
  if(info!==undefined&&info.isFile()&&info.size>=largeBlobBytesV1){
   const local=await hashLocalV1(source,options.signal,options.alive);
   if(local.stable){
    const target=blobPathV1(storeRoot,local.sha256);
    if(await exists(target)&&await intact(target,local.sha256,options.alive)&&await touchBlobV1(target))return {sha256:local.sha256,bytes:local.bytes,stable:true,created:false};
   }
  }
 }
 const temporary=stage(tmp,originId);
 try{
  let present:boolean|undefined;
  const {alive,progress}=options,beat=alive===undefined?progress:(done:number,total:number)=>{alive();progress?.(done,total);};
  const copied=await inDirectory(storeRoot,tmp,()=>copyHashedV1(source,temporary,{signal:options.signal,progress:beat,flushBytes:storeFlushBytesV1,sync:async hash=>{present=await exists(blobPathV1(storeRoot,hash));return !present;},clone:true}));
  if(!copied.stable||(known!==undefined&&copied.sha256!==known.sha256)){await unlink(temporary).catch(()=>{});return {sha256:copied.sha256,bytes:copied.bytes,stable:false,created:false};}
  const target=blobPathV1(storeRoot,copied.sha256);
  if((present??await exists(target))&&await intact(target,copied.sha256,alive)&&await touchBlobV1(target)){await unlink(temporary).catch(()=>{});return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:false};}
  if(present===true){const handle=await open(temporary,"r+");try{await handle.sync();}finally{await handle.close();}}
  try{await inDirectory(storeRoot,dirname(target),()=>rename(temporary,target));}
  catch(error){
   if((error as {code?:string}).code!=="EEXIST"&&!await exists(target))throw error;
   await unlink(temporary).catch(()=>{});return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:false};
  }
  verifiedBlobs.add(target);
  return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:true};
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** 📥️ Stores a file as an immutable blob whose hash the caller already knows, verified against the bytes of the single read; an existing blob is rehashed (once per process) before it is trusted and one that does not hash to its name is replaced by the new copy; `false` means it was already there and intact, a source that hashes differently or changed while publishing throws. */
export async function writeBlobV1(storeRoot:string,originId:string,sha256:string,source:string):Promise<boolean>{
 const stored=await storeBlobV1(storeRoot,originId,source,{known:{sha256,bytes:0}});
 if(!stored.stable)throw Error(`Fleet blob source changed while publishing: ${sha256}`);
 return stored.created;
}

/** 🫗️ Puts the right bytes under a blob name: the existing blob is hashed afresh (the per-process memory of verified blobs is not trusted here) and kept when it is intact; otherwise `source`, which must hash to `sha256` while it is copied, replaces it through a staged copy and a rename, so a reader sees the old file or the new one and never a mix. Returns `intact`, `repaired`, or `unstable` (the source changed or hashes differently, nothing was written). */
export async function repairBlobV1(storeRoot:string,originId:string,sha256:string,source:string):Promise<"intact"|"repaired"|"unstable">{
 requireOriginIdV1(originId);
 const target=blobPathV1(storeRoot,sha256),{tmp}=storePathsV1(storeRoot);
 verifiedBlobs.delete(target);
 if(await intact(target,sha256)){await touchBlobV1(target);return "intact";}
 const temporary=stage(tmp,originId);
 try{
  const copied=await inDirectory(storeRoot,tmp,()=>copyHashedV1(source,temporary,{sync:true}));
  if(!copied.stable||copied.sha256!==sha256){await unlink(temporary).catch(()=>{});return "unstable";}
  await inDirectory(storeRoot,dirname(target),()=>renameWithRetry(temporary,target));
  verifiedBlobs.add(target);
  return "repaired";
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** ⌛️ How long a delete tombstone is kept in the snapshots, in milliseconds (90 days). A tombstone is what makes an older put lose; once no journal segment can hold such a put and no follower can still lack the delete (a follower further behind than the segment retention must rejoin from a snapshot), it only costs space. A snapshot written at hlc `S` therefore leaves out the deletes older than `S - lifetime`, and a follower that rejoins from it treats a live entry older than that horizon which the snapshot lacks as deleted. */
export const tombstoneLifetimeMillisecondsV1=90*86_400_000;

/** 🕯️ Tells whether a manifest entry is a delete tombstone that a snapshot written at `snapshotHlc` no longer needs. */
export function expiredTombstoneV1(entry:FleetManifestEntryV1,snapshotHlc:string):boolean{
 return entry.operation==="delete"&&parseHlcV1(entry.hlc).ms<parseHlcV1(snapshotHlc).ms-tombstoneLifetimeMillisecondsV1;
}

/** 🦴️ Tells whether an entry hlc is older than the tombstone horizon of a snapshot written at `snapshotHlc`: a live entry that old which the snapshot lacks was deleted, its tombstone has expired. */
export function expiredHorizonV1(entryHlc:string,snapshotHlc:string):boolean{
 return parseHlcV1(entryHlc).ms<parseHlcV1(snapshotHlc).ms-tombstoneLifetimeMillisecondsV1;
}

/** 🩹️ What a repair of a segment that was shorter than its head reports: the segment that was sealed short, its real size in bytes and how many complete lines it holds. */
export type FleetJournalRepairV1=Readonly<{segment:number;bytes:number;found:number}>;

/** ✍️ Validates every event, encodes and fsyncs one segment at a time, then publishes the new head through tmp + rename. A segment that is shorter than its head (a store host that crashed before the data reached the disk, a removed file) is sealed at the size it has and the append continues in the next segment: peers read a sealed segment to its end and then the next one from its start, so no cursor ends up in front of new events; `onRepair` is told. */
export async function appendEventsV1(storeRoot:string,originId:string,events:readonly FleetEventV1[],onRepair?:(repair:FleetJournalRepairV1)=>void):Promise<FleetHeadV1>{
 requireOriginIdV1(originId);
 for(const event of events){
  requireEventV1(event);
  if(parseHlcV1(event.hlc).originId!==originId)throw Error(`Fleet event hlc belongs to another origin than ${originId}: ${event.hlc}`);
 }
 const prior=await readHeadV1(storeRoot,originId);
 if(events.length===0){
  if(prior)return prior;
  throw Error("Fleet append requires at least one event");
 }
 const {origins,tmp}=storePathsV1(storeRoot),directory=join(origins,originId),segments=join(directory,"segments");
 await ensureStoreDirectoryV1(storeRoot,segments);
 let segment=prior?.segment??0,bytes=prior?.bytes??0,count=prior?.events??0,hlc=prior?.hlc??"",index=0;
 let lookahead:Buffer|undefined;
 while(index<events.length){
  if(count>=segmentEvents||bytes>=segmentBytes){segment++;bytes=0;count=0;}
  const file=join(segments,segmentName(segment)),handle=await open(file,await exists(file)?"r+":"w+");
  try{
   const current=(await handle.stat()).size;
   if(current<bytes){
    const kept=await readRange(file,0,current),complete=kept.lastIndexOf(10)+1;
    if(complete<current)await handle.truncate(complete);
    onRepair?.({segment,bytes:complete,found:kept.subarray(0,complete).toString("utf8").split("\n").length-1});
    segment++;bytes=0;count=0;
    continue;
   }
   const lines:Buffer[]=[];
   let end=index,length=0;
   while(end<events.length&&count+(end-index)<segmentEvents){
    lookahead??=Buffer.from(`${JSON.stringify(events[end])}\n`,"utf8");
    if(end>index&&bytes+length+lookahead.length>segmentBytes)break;
    lines.push(lookahead);length+=lookahead.length;end++;lookahead=undefined;
   }
   const data=Buffer.concat(lines,length);
   if(current>bytes)await handle.truncate(bytes);
   let done=0;
   while(done<data.length){const {bytesWritten}=await handle.write(data,done,data.length-done,bytes+done);done+=bytesWritten;}
   await handle.sync();
   for(let at=index;at<end;at++)if(events[at]!.hlc>hlc)hlc=events[at]!.hlc;
   bytes+=length;count+=end-index;index=end;
  }finally{await handle.close();}
 }
 const head:FleetHeadV1={version:1,segment,bytes,events:count,hlc};
 if(!isHeadV1(head))check("Head",head,"Closed fleet head required");
 await replaceFile(tmp,originId,join(directory,"head.json"),JSON.stringify(head));
 await publishFlatHeadV1(storeRoot,originId,head).catch(()=>{});
 return head;
}

const flatPattern=/^([a-z0-9][a-z0-9-]{0,39})_([0-9]{1,16})_([0-9]{1,16})_([0-9]{1,16})_([0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9-]{0,39})$/u,flatSettleMilliseconds=[30,1100],flatRefreshMilliseconds=10_000;
const flatStates=new Map<string,{signature:string;readAt:number;stage:number;heads:ReadonlyMap<string,FleetHeadV1>}>(),flatWritten=new Map<string,string>(),noHeads:ReadonlyMap<string,FleetHeadV1>=new Map();
const flatName=(originId:string,head:FleetHeadV1):string=>`${originId}_${head.segment}_${head.bytes}_${head.events}_${head.hlc}`;
const flatHead=(name:string):Readonly<{originId:string;head:FleetHeadV1}>|undefined=>{
 const found=flatPattern.exec(name);
 if(found===null)return undefined;
 const head:FleetHeadV1={version:1,segment:Number(found[2]),bytes:Number(found[3]),events:Number(found[4]),hlc:found[5]!};
 return isHeadV1(head)?{originId:found[1]!,head}:undefined;
};
const later=(left:FleetHeadV1,right:FleetHeadV1):boolean=>left.segment>right.segment||(left.segment===right.segment&&left.bytes>right.bytes);

/** 🚩️ Publishes the flat copy of an origin's head: an empty file `<store>/v1/heads/<originId>_<segment>_<bytes>_<events>_<hlc>` whose name is the head, created before the previous name of that origin is removed (a reader sees both or the new one, never none, and takes the later). Every create and remove changes the mtime of the one directory `v1/heads`, so a follower learns that any origin published from one `stat` and then reads all heads with one `readdir`, without opening a file. It is only a hint written after `head.json`: a failure is harmless (followers audit `head.json` every 10 seconds), nothing reads it as the truth about the journal bytes, and code that does not know it ignores the directory. The first call of a process removes the leftover names of that origin. */
export async function publishFlatHeadV1(storeRoot:string,originId:string,head:FleetHeadV1):Promise<void>{
 requireOriginIdV1(originId);
 const {heads}=storePathsV1(storeRoot),name=flatName(originId,head),key=`${storeRoot}\0${originId}`,previous=flatWritten.get(key);
 await ensureStoreDirectoryV1(storeRoot,heads);
 if(previous!==name){
  try{await (await open(join(heads,name),"wx",0o644)).close();}catch(error){if((error as {code?:string}).code!=="EEXIST")throw error;}
 }
 if(flatWritten.size>=4096)flatWritten.clear();
 flatWritten.set(key,name);
 const stale=previous!==undefined?[previous]:(await readdir(heads)).filter(entry=>entry!==name&&entry.startsWith(`${originId}_`)&&flatHead(entry)?.originId===originId);
 for(const entry of stale)if(entry!==name)await unlink(join(heads,entry)).catch(()=>{});
}

/** 🪧️ Reads the flat heads of a store: every origin with an entry in `v1/heads` mapped to its published head (the later one when a writer is between create and remove). One `stat` of the directory per call; the directory is listed again only when its signature changed, then once more after 30 ms and after 1.1 seconds without a further change (a coarse file system timestamp can hide a second change within the same tick), and every 10 seconds for a stale client attribute cache. A store without the directory (nobody writes it yet) has no flat heads, so callers poll the heads of the origins that are missing here the old way. */
export async function readFlatHeadsV1(storeRoot:string):Promise<ReadonlyMap<string,FleetHeadV1>>{
 const directory=storePathsV1(storeRoot).heads,known=flatStates.get(storeRoot),now=Date.now();
 let signature:string;
 try{const info=await stat(directory);signature=`${info.ino}:${info.mtimeMs}:${info.size}:${info.nlink}`;}
 catch(error){if(missing(error)||(error as {code?:string}).code==="ENOTDIR"){flatStates.delete(storeRoot);return noHeads;}throw error;}
 const same=known!==undefined&&known.signature===signature;
 if(same){
  const wait=known.stage<flatSettleMilliseconds.length?flatSettleMilliseconds[known.stage]!:flatRefreshMilliseconds;
  if(now-known.readAt<wait)return known.heads;
 }
 let names:string[];
 try{names=await readdir(directory);}catch(error){if(missing(error)||(error as {code?:string}).code==="ENOTDIR"){flatStates.delete(storeRoot);return noHeads;}throw error;}
 const heads=new Map<string,FleetHeadV1>();
 for(const name of names){
  const found=flatHead(name);
  if(found===undefined)continue;
  const current=heads.get(found.originId);
  if(current===undefined||later(found.head,current))heads.set(found.originId,found.head);
 }
 if(flatStates.size>=64)flatStates.clear();
 flatStates.set(storeRoot,{signature,readAt:now,stage:same?Math.min(known.stage+1,flatSettleMilliseconds.length):0,heads});
 return heads;
}

/** 👀️ Reads the published head of an origin; `undefined` when the origin has none yet. */
export async function readHeadV1(storeRoot:string,originId:string):Promise<FleetHeadV1|undefined>{
 requireOriginIdV1(originId);
 const file=join(storePathsV1(storeRoot).origins,originId,"head.json"),key=`${storeRoot}\0${originId}`,known=knownHeads.get(key),now=Date.now();
 const read=await readThroughReplacementV1(async()=>{
  const info=await stat(file),signature=`${info.ino}:${info.size}:${info.mtimeMs}`;
  return known!==undefined&&known.signature===signature&&now-known.at<headRevalidateMilliseconds?{signature,text:undefined}:{signature,text:await readFile(file,"utf8")};
 });
 if(read===undefined)return known!==undefined&&await exists(file)?known.head:undefined;
 if(read.text===undefined)return known!.head;
 const {signature,text}=read;
 if(known!==undefined&&known.text===text){knownHeads.set(key,{signature,at:now,text,head:known.head});return known.head;}
 const value=JSON.parse(text) as unknown;
 if(!isHeadV1(value))check("Head",value,"Closed fleet head required");
 if(knownHeads.size>=4096)knownHeads.clear();
 knownHeads.set(key,{signature,at:now,text,head:value as FleetHeadV1});
 return value as FleetHeadV1;
}

/** 📇️ Lists the origin directories of a store in sorted order; a missing store has none. */
export async function listOriginsV1(storeRoot:string):Promise<readonly string[]>{
 try{
  const directory=storePathsV1(storeRoot).origins,info=await stat(directory),signature=`${info.ino}:${info.mtimeMs}:${info.nlink}`,known=knownOrigins.get(storeRoot),now=Date.now();
  if(known!==undefined&&known.signature===signature&&now-known.at<headRevalidateMilliseconds){markStoreSeenV1(storeRoot);return known.names;}
  const entries=await readdir(directory,{withFileTypes:true});
  markStoreSeenV1(storeRoot);
  const names=entries.filter(entry=>entry.isDirectory()&&originPattern.test(entry.name)).map(entry=>entry.name).sort();
  if(knownOrigins.size>=64)knownOrigins.clear();
  knownOrigins.set(storeRoot,{signature,at:now,names});
  return names;
 }catch(error){if(missing(error))return [];throw error;}
}

async function readRange(file:string,from:number,to:number):Promise<Buffer>{
 const handle=await open(file,"r");
 try{
  const length=Math.max(0,Math.min(to,(await handle.stat()).size)-from),data=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(data,done,length-done,from+done);if(bytesRead===0)break;done+=bytesRead;}
  return data.subarray(0,done);
 }finally{await handle.close();}
}

/** 📏️ Like `readEventsV1` and additionally returns `ends`, the cursor just behind each event (`ends[i]` belongs to `events[i]`), so a caller can consume only a prefix of the batch and keep a byte cursor in front of the first unconsumed event. With `segments` it reads at most that many segments (from the cursor's), so a long journal can be folded in bounded memory by calling it again with the returned cursor until the cursor stops advancing. With `maxEvents` it stops after that many events inside a segment, with the cursor right behind the last one, so one call costs a bounded amount of parsing and memory whatever the backlog. With `known` (a head the caller already read, for instance from `readFlatHeadsV1`) it does not read `head.json` again. */
export async function readEventSpansV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined,segments?:number,known?:FleetHeadV1,maxEvents?:number):Promise<Readonly<{events:readonly FleetEventV1[];ends:readonly FleetCursorV1[];cursor:FleetCursorV1}>>{
 const start=after??{segment:0,bytes:0},head=known??await readHeadV1(storeRoot,originId);
 if(!head||start.segment>head.segment||(start.segment===head.segment&&start.bytes>=head.bytes))return {events:[],ends:[],cursor:start};
 const directory=join(storePathsV1(storeRoot).origins,originId,"segments"),events:FleetEventV1[]=[],ends:FleetCursorV1[]=[],turn=yieldingV1();
 let cursor:FleetCursorV1=start;
 const last=segments===undefined?head.segment:Math.min(head.segment,start.segment+segments-1);
 for(let segment=start.segment;segment<=last;segment++){
  const from=segment===start.segment?start.bytes:0,limit=segment===head.segment?head.bytes:Infinity;
  const data=limit<=from?Buffer.alloc(0):await readRange(join(directory,segmentName(segment)),from,limit);
  let local=0;
  for(;;){
   await turn();
   const newline=data.indexOf(10,local);
   if(newline<0)break;
   const line=data.toString("utf8",local,newline);
   if(line!==""){
    try{events.push(requireEventV1(JSON.parse(line)));}
    catch(error){
     const version=(()=>{try{return (JSON.parse(line) as {version?:unknown}).version;}catch{return 1;}})();
     if(version!==1&&version!==undefined)throw Error(`Fleet segment ${segmentName(segment)} of ${originId} holds an unsupported event version ${String(version)} at byte ${from+local}: this daemon cannot read it and waits for newer code`);
     throw Error(`Fleet segment ${segmentName(segment)} of ${originId} is corrupt at byte ${from+local}: ${(error as Error).message}`);
    }
    ends.push({segment,bytes:from+newline+1});
    if(maxEvents!==undefined&&events.length>=maxEvents)return {events,ends,cursor:{segment,bytes:from+newline+1}};
   }
   local=newline+1;
  }
  cursor={segment,bytes:from+local};
  if(segment<head.segment&&local<data.length)break;
  if(segment===last&&last<head.segment)cursor={segment:segment+1,bytes:0};
 }
 return {events,ends,cursor};
}

/** 📖️ Reads the complete event lines after a cursor across segments, trusting only `head.bytes` of the head segment; returns the advanced cursor. */
export async function readEventsV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined):Promise<Readonly<{events:readonly FleetEventV1[];cursor:FleetCursorV1}>>{
 const {events,cursor}=await readEventSpansV1(storeRoot,originId,after);
 return {events,cursor};
}

const entryOf=(event:FleetEventV1):FleetManifestEntryV1=>event.operation==="put"?{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:event.executable!}
 :event.operation==="link"?{hlc:event.hlc,operation:"link",target:event.target!}:{hlc:event.hlc,operation:"delete"};

/** 🎯️ What a fold needs of a manifest: lookup and store of one path; a `Map` and the compact tables of the replica both qualify. */
export type FleetManifestTargetV1=Readonly<{get(path:string):FleetManifestEntryV1|undefined;set(path:string,entry:FleetManifestEntryV1):unknown}>;

/** 🧮️ Folds events into the manifest in place and returns the events that won and still stand, ordered by hlc then path. */
export function foldEventsV1(manifest:FleetManifestTargetV1,events:readonly FleetEventV1[]):readonly FleetEventV1[]{
 const won=new Map<string,FleetEventV1>();
 for(const event of events){
  if(!winsV1(event.hlc,manifest.get(event.path)?.hlc))continue;
  manifest.set(event.path,entryOf(event));won.set(event.path,event);
 }
 return [...won.values()].sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:left.path<right.path?-1:left.path>right.path?1:0);
}

/** 📸️ Writes a full manifest and the cursors it covers as `snapshots/<originId>/<hlc>.json` and returns that file; the document is the same single JSON object as ever but is streamed in sorted order through a temporary file, so the manifest never exists as one string. */
export async function writeSnapshotV1(storeRoot:string,originId:string,hlc:string,manifest:Readonly<{size:number;keys():Iterable<string>;get(path:string):FleetManifestEntryV1|undefined}>,cursors:Readonly<Record<string,FleetCursorV1>>):Promise<string>{
 requireOriginIdV1(originId);
 const sortedCursors=Object.fromEntries(Object.entries(cursors).sort(byKey));
 check("Snapshot",{version:1,hlc,manifest:{},cursors:sortedCursors},"Closed fleet snapshot required");
 const {snapshots,tmp}=storePathsV1(storeRoot),directory=join(snapshots,originId),target=join(directory,`${hlc}.json`);
 await ensureStoreDirectoryV1(storeRoot,directory);await ensureStoreDirectoryV1(storeRoot,tmp);
 const temporary=stage(tmp,originId),keys=[...manifest.keys()].sort((left,right)=>left<right?-1:left>right?1:0);
 try{
  const handle=await open(temporary,"w");
  try{
   let text=`{"version":1,"hlc":${JSON.stringify(hlc)},"manifest":{`,first=true;
   for(const path of keys){
    const entry=manifest.get(path);
    if(entry===undefined)continue;
    if(!isManifestEntryV1(entry))check("ManifestEntry",entry,"Closed fleet snapshot required");
    if(expiredTombstoneV1(entry,hlc))continue;
    text+=`${first?"":","}${JSON.stringify(path)}:${JSON.stringify(entry)}`;first=false;
    if(text.length>=snapshotChunk){await handle.write(text);text="";}
   }
   await handle.write(`${text}},"cursors":${JSON.stringify(sortedCursors)}}`);
   await handle.sync();
  }finally{await handle.close();}
  await renameWithRetry(temporary,target);
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
 return target;
}

const blank=(code:number):boolean=>code===32||code===10||code===13||code===9;

/** 🌊️ Streams the members of a big JSON file without holding it: for every top-level member whose key is in `streamed` and whose value is an object, `visit(outer,inner,text)` is called once per inner member with the raw JSON text of its value; every other top-level member is delivered once as `visit(key,undefined,text)`; `opened(outer)` is called when a streamed member starts, so an empty one is still noticed; `chunk` is the read size in bytes. Any valid JSON formatting is accepted; a malformed or non-object document throws. */
export async function streamMembersV1(file:string,streamed:ReadonlySet<string>,visit:(outer:string,inner:string|undefined,text:string)=>void,opened:(outer:string)=>void=()=>{},chunk:number=1<<14):Promise<void>{
 const fail=():never=>{throw Error(`Closed JSON object required: ${file}`);};
 let started=false,done=false,inside=false,inString=false,escaped=false,keying=false,active=false,nest=0,from=0,part="",outer="",inner="",expect:"key"|"colon"|"value"|"after"="key";
 for await(const piece of createReadStream(file,{encoding:"utf8",highWaterMark:chunk})){
  const chunk=piece as string;
  if(active)from=0;
  for(let at=0;at<chunk.length;at++){
   const code=chunk.charCodeAt(at);
   if(inString){
    if(escaped)escaped=false;
    else if(code===92)escaped=true;
    else if(code===34){
     inString=false;
     if(keying){
      keying=false;active=false;
      const key=JSON.parse(part+chunk.slice(from,at+1)) as string;
      part="";
      if(inside)inner=key;else outer=key;
      expect="colon";
     }
    }
    continue;
   }
   if(done){if(!blank(code))fail();continue;}
   if(!started){if(blank(code))continue;if(code!==123)fail();started=true;expect="key";continue;}
   if(expect==="key"){
    if(blank(code))continue;
    if(code===34){inString=true;keying=true;active=true;from=at;}
    else if(code===125){if(inside){inside=false;expect="after";}else done=true;}
    else fail();
    continue;
   }
   if(expect==="colon"){if(blank(code))continue;if(code!==58)fail();expect="value";continue;}
   if(expect==="after"){
    if(blank(code))continue;
    if(code===44)expect="key";else if(code===125)done=true;else fail();
    continue;
   }
   if(!active){
    if(blank(code))continue;
    if(!inside&&code===123&&streamed.has(outer)){inside=true;expect="key";opened(outer);continue;}
    active=true;from=at;nest=0;
   }
   if(code===34){inString=true;continue;}
   if(code===123||code===91){nest++;continue;}
   if((code===125||code===93)&&nest>0){nest--;continue;}
   if(nest===0&&(code===44||code===125)){
    const text=(part+chunk.slice(from,at)).trim();
    part="";active=false;
    visit(outer,inside?inner:undefined,text);
    if(code===44)expect="key";
    else if(inside){inside=false;expect="after";}
    else done=true;
   }
  }
  if(active){part+=chunk.slice(from);}
 }
 if(!done)fail();
}

/** 🎞️ Reads the snapshot with the greatest hlc across all origins (or only the snapshots written by `onlyOrigin`), skipping unreadable ones; `undefined` when there is none. The manifest is streamed entry by entry into `create()` (a fresh `Map` by default), so a big snapshot needs no more memory than the manifest it fills. */
export async function readNewestSnapshotV1<M extends FleetManifestTargetV1=Map<string,FleetManifestEntryV1>>(storeRoot:string,onlyOrigin?:string,create:()=>M=()=>new Map<string,FleetManifestEntryV1>() as unknown as M):Promise<Readonly<{hlc:string;manifest:M;cursors:Record<string,FleetCursorV1>}>|undefined>{
 const {snapshots}=storePathsV1(storeRoot),found:{hlc:string;file:string}[]=[];
 let origins:string[];
 try{origins=await readdir(snapshots);}catch(error){if(missing(error))return undefined;throw error;}
 for(const origin of origins){
  if(!originPattern.test(origin)||(onlyOrigin!==undefined&&origin!==onlyOrigin))continue;
  let names:string[];
  try{names=await readdir(join(snapshots,origin));}catch(error){if(missing(error))continue;throw error;}
  for(const name of names)if(name.endsWith(".json")&&hlcPattern.test(name.slice(0,-5)))found.push({hlc:name.slice(0,-5),file:join(snapshots,origin,name)});
 }
 found.sort((left,right)=>left.hlc<right.hlc?1:left.hlc>right.hlc?-1:0);
 for(const {hlc,file} of found){
  try{
   const manifest=create(),top:Record<string,unknown>={};
   await streamMembersV1(file,new Set(["manifest"]),(outer,inner,text)=>{
    if(outer!=="manifest"){top[outer]=JSON.parse(text);return;}
    const entry=JSON.parse(text) as FleetManifestEntryV1;
    if(!isPathV1(inner))check("Path",inner,"Closed fleet snapshot required");
    if(!isManifestEntryV1(entry))check("ManifestEntry",entry,"Closed fleet snapshot required");
    manifest.set(inner!,entry);
   });
   check("Snapshot",{...top,manifest:{}},"Closed fleet snapshot required");
   if(top.hlc!==hlc)continue;
   return {hlc,manifest,cursors:top.cursors as Record<string,FleetCursorV1>};
  }catch{continue;}
 }
 return undefined;
}
