import {spawn,spawnSync} from "node:child_process";
import {createHash,randomBytes,randomUUID} from "node:crypto";
import {accessSync,constants,createReadStream,createWriteStream,statSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {chmod,copyFile,lstat,mkdir,readdir,readlink,realpath,rename,rm,stat,utimes,writeFile} from "node:fs/promises";
import {availableParallelism,hostname,platform as osPlatform,totalmem} from "node:os";
import {delimiter,dirname,isAbsolute,join,normalize,sep} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../📡️wire/📦️protobuf/🟦️.ts";
import {httpRemoteExecutionContentV1,retryRemoteExecutionContentOutageV1,RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {placeRemoteExecutionLinkV1,remoteExecutionLinkEscapesV1,selectRemoteExecutionSandboxV1,type RemoteExecutionSandboxStrategyV1} from "../🔒️sandbox/🟦️.ts";
import {expandRemoteExecutionRootV1,remoteExecutionCommandProgramV1,remoteExecutionNativePathV1,remoteExecutionToolOfProgramV1,validateRemoteExecutionCommandActionV1,validateRemoteExecutionRustcActionV1,type RemoteExecutionActionPolicyV1} from "../🧱️actions/🟦️.ts";
import {COMMAND_POLICY_V1,commandTextV1,commandToolKeyV1,commandToolSourceV1,parseCommandProgramV1} from "../🚀️offload/🧰️command/📐️policy/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadCrateTypesOfV1} from "../🚀️offload/📐️policy/🟦️.ts";
import {createRemoteExecutionMeterV1,remoteExecutionProcessTableV1} from "./📏️task-memory/🟦️.ts";
import {remoteExecutionCodegenArgumentsV1,remoteExecutionCodegenSerialRequestedV1,withRemoteExecutionCodegenPoolV1} from "./🧵️codegen/🟦️.ts";
import type {RemoteExecutionAdmissionV1,RemoteExecutionHeldUnitV1,RemoteExecutionWorkerStateV1} from "../🌡️load/🟦️.ts";
import {remoteExecutionPeakEntryV1,remoteExecutionThreadsEntryV1,readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,remoteExecutionMatchPropertiesV1,remoteExecutionPlatformMatchesV1,remoteExecutionDigestV1,requireRemoteExecutionShapeV1,type RemoteExecutionConfinementV1,type RemoteExecutionDigestV1,type RemoteExecutionNetworkV1,type RemoteExecutionHeartbeatAnswerV1,type RemoteExecutionPollAnswerV1,type RemoteExecutionPropertyV1,type RemoteExecutionRefusalV1,type RemoteExecutionSandboxV1,type RemoteExecutionTaskResultV1,type RemoteExecutionTaskStatusV1,type RemoteExecutionTaskV1,type RemoteExecutionWorkerRegisteredV1} from "../🟦️.ts";
import { runtimeSleepV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";
import {runtimeIdentityV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {OFFLOAD_REFUSAL_PREFIX_V1} from "../🚀️offload/🩺️failure/🟦️.ts";
import {casObjectPathV1,casRootV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {casReserveBytesV1,measureCasPartitionV1} from "../../🗄️cas/🌡️partition/🟦️.ts";
import {CAS_POLICY_V1} from "../../🗄️cas/🧬️schema/🟦️.ts";
import {remoteExecutionDiskNeedBytesV1} from "../🛰️agent/💽️disk/🟦️.ts";

const EMPTY_HASH="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/** 📣️ Something a worker did, for logs and progress. */
export type RemoteExecutionWorkerEventV1=Readonly<{stage:"registered"|"reconnecting"|"started"|"finished"|"cancelled"|"stopped"|"sandbox"|"refused";detail:string}>;

/** ⚙️ Everything a worker needs: where the scheduler is, who it is, what it offers and where it works. */
/** 🙋️ What the worker tells its admission: `poll` (before it asks the scheduler for a unit, so it asks for the smallest) or `task` (a unit arrived, with the peak megabytes its requester declared, 0 when none), and the peaks of the units it already runs (0 for an undeclared one). */
export type RemoteExecutionUnitAskV1=Readonly<{stage:"poll";held:readonly RemoteExecutionHeldUnitV1[]}>|Readonly<{stage:"task";megabytes:number;held:readonly RemoteExecutionHeldUnitV1[]}>;

/** 🧵️ What a worker tells its host to size the code generation threads of one linked unit: the cores its other running actions hold (the grant of a linked unit, else one each), the peak this unit was admitted with (0: none declared) and the peaks of the other running units. */
export type RemoteExecutionThreadsAskV1=Readonly<{others:number;peakMegabytes:number;held:readonly RemoteExecutionHeldUnitV1[]}>;

export type RemoteExecutionWorkerOptionsV1=Readonly<{
 scheduler:string;token?:string|undefined;cacheToken?:string|undefined;fetch?:typeof fetch|undefined;cacheFetch?:typeof fetch|undefined;actions?:RemoteExecutionActionPolicyV1;unconfinedForTests?:boolean;name?:string;concurrency:number;directory:string;sandbox:"auto"|RemoteExecutionSandboxV1;materialize:"clone"|"copy";maximumBlobBytes:number;inlineOutputBytes:number;
 properties?:readonly RemoteExecutionPropertyV1[];platform?:string;signal:AbortSignal;onEvent?:(event:RemoteExecutionWorkerEventV1)=>void;content?:RemoteExecutionContentV1;keepWork?:boolean;
 origin?:string;capacity?:Readonly<{cores:number;memoryMegabytes:number}>;network?:"deny"|"allow";tools?:RemoteExecutionToolsV1;commands?:RemoteExecutionToolsV1;confinement?:"required"|"best-effort";admit?:(running:number,unit:RemoteExecutionUnitAskV1)=>Promise<RemoteExecutionAdmissionV1>;observe?:(state:RemoteExecutionWorkerStateV1)=>void;admitRetryMilliseconds?:number;threads?:(ask:RemoteExecutionThreadsAskV1)=>Promise<number>;
}>;

/** 🛠️ The tools a worker owns by name: an action whose program is `@tool:<name>` runs the tool of that name, and its confined process may read the listed directories. The toolchain itself is never shipped, a worker that lacks the tool refuses the action with a precondition failure. */
export type RemoteExecutionToolsV1=Readonly<Record<string,Readonly<{path:string;readable:readonly string[]}>>>;

/** 🧰️ The command tools a worker owns, keyed `<tool>/<sha-256 of the tool file>`: an action whose program is `@command:<tool>/<digest>` runs only the tool of that exact key (a shipped tool needs no row, the worker checks the digest of the file it received), and only on a confining sandbox without network. A worker without this table refuses every command action. */
export type RemoteExecutionCommandToolsV1=RemoteExecutionToolsV1;

/** 🎬️ What executing one action needs besides the store. */
export type RemoteExecutionExecutionV1=Readonly<{
 properties?:readonly RemoteExecutionPropertyV1[];instance:string;action:RemoteExecutionDigestV1;timeoutSeconds:number;content:RemoteExecutionContentV1;directory:string;work?:string;workerName:string;sandbox:RemoteExecutionSandboxStrategyV1;materialize:"clone"|"copy";inlineOutputBytes:number;signal:AbortSignal;keepWork?:boolean;blobs?:RemoteExecutionBlobCacheV1;network?:boolean;tools?:RemoteExecutionToolsV1;commands?:RemoteExecutionCommandToolsV1;actions?:RemoteExecutionActionPolicyV1;meter?:Readonly<{attach(pid:number):void;peak():number|undefined}>;threads?:()=>Promise<number>;wardenCancels?:(sinceMs:number)=>Promise<readonly Readonly<{pid:number;command:string}>[]>;
}>;

/** 🏁️ How executing one action ended. */
export type RemoteExecutionExecutionResultV1=Readonly<{outcome:"completed"|"failed"|"cancelled"|"refused";status:RemoteExecutionTaskStatusV1;actionResult:Uint8Array|null;refusal?:RemoteExecutionRefusalV1}>;

class MissingBlob extends Error{readonly digest:RemoteExecutionDigestV1; constructor(digest:RemoteExecutionDigestV1){super(`Missing blob ${digest.hash}/${digest.size}`);this.digest=digest;}}

const sha256=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const digestOf=(data:Uint8Array):RemoteExecutionDigestV1=>({hash:sha256(data),size:data.length});
const commandToolKey=(tool:Readonly<{name:string;digest:string}>):string=>commandToolKeyV1(tool.name,tool.digest);
const fileSha256=(path:string):Promise<string>=>new Promise((resolve,reject)=>{const hash=createHash("sha256");createReadStream(path).on("data",chunk=>hash.update(chunk)).on("error",reject).on("end",()=>{resolve(hash.digest("hex"));});});
const wire=(digest:RemoteExecutionDigestV1)=>({hash:digest.hash,size_bytes:digest.size});
const unwire=(digest:{hash:string;size_bytes:number}|undefined):RemoteExecutionDigestV1=>{
 if(!digest)throw Error("Digest required");
 return remoteExecutionDigestV1(digest.hash,digest.size_bytes);
};
const timestamp=(milliseconds:number)=>({seconds:Math.floor(milliseconds/1000),nanos:(milliseconds%1000)*1000000});

async function limited<T>(items:readonly T[],limit:number,work:(item:T)=>Promise<void>):Promise<void>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{
  while(failure===undefined){const index=next++;if(index>=items.length)return;try{await work(items[index]!);}catch(error){failure??=error;}}
 }));
 if(failure!==undefined)throw failure;
}

const BLOB_PROTECT_MILLISECONDS_V1=600000,FETCH_CONCURRENCY_V1=32;

/** 📥️ Where the input files of one action came from: the worker's blob cache (`cached`), the machine's own content store (`local`, a copy on this disk) or the network (`fetched`), in files and bytes. */
export type RemoteExecutionInputFetchV1={cachedFiles:number;cachedBytes:number;localFiles:number;localBytes:number;fetchedFiles:number;fetchedBytes:number};

/** 🏷️ The type of the auxiliary metadata entry that carries {@link RemoteExecutionInputFetchV1} in an action result (REAPI `ExecutedActionMetadata.auxiliary_metadata`; clients that do not know it ignore it). */
export const REMOTE_EXECUTION_INPUT_FETCH_TYPE_URL_V1="type.semio.dev/input-fetch";

/** 📦️ The auxiliary entry of an input fetch: its JSON text. */
export const remoteExecutionInputFetchEntryV1=(fetch:RemoteExecutionInputFetchV1):Readonly<{type_url:string;value:Uint8Array}>=>Object.freeze({type_url:REMOTE_EXECUTION_INPUT_FETCH_TYPE_URL_V1,value:new TextEncoder().encode(JSON.stringify(fetch))});

/** 📖️ The input fetch an action result reports, `undefined` when it carries none. */
export function remoteExecutionInputFetchOfV1(entries:readonly Readonly<{type_url:string;value:Uint8Array}>[]|undefined):RemoteExecutionInputFetchV1|undefined{
 for(const entry of entries??[])if(entry.type_url===REMOTE_EXECUTION_INPUT_FETCH_TYPE_URL_V1)try{return JSON.parse(new TextDecoder().decode(entry.value)) as RemoteExecutionInputFetchV1;}catch{return undefined;}
 return undefined;
}

/** 📐️ How large the blob cache may be on the disk it has: at most `maximumBytes`, and never so large that the volume of the work directory keeps less than what every action slot needs free (`keepWorkBytes`, the disk rule of the agent) or the volume of the machine's content store falls below the content store's hard reserve of the worker role (`hostReserveBytes`, CAS7); a volume that cannot be measured does not bound it. The cache counts its own bytes as room it may keep. Pure. */
export function remoteExecutionBlobBoundBytesV1(input:Readonly<{maximumBytes:number;cacheBytes:number;workFreeBytes?:number|undefined;keepWorkBytes:number;hostFreeBytes?:number|undefined;hostReserveBytes:number}>):number{
 const work=input.workFreeBytes===undefined?Infinity:input.cacheBytes+input.workFreeBytes-input.keepWorkBytes,host=input.hostFreeBytes===undefined?Infinity:input.cacheBytes+input.hostFreeBytes-input.hostReserveBytes;
 return Math.max(0,Math.min(input.maximumBytes,work,host));
}

/** 💽️ The worker's local, bounded, content-addressed blob cache: every blob is fetched once, verified against its digest and written atomically; least recently used blobs are evicted beyond the bound, but never one that was used within the last ten minutes (an action that runs or is being prepared holds its inputs, so a bound below one input closure cannot make every action fetch its whole closure again). */
export class RemoteExecutionBlobCacheV1{
 private readonly pending=new Map<string,Promise<string>>();
 readonly directory:string; private readonly content:RemoteExecutionContentV1; private readonly maximumBytes:number; private readonly room:((cacheBytes:number)=>number)|undefined; private readonly local:string|undefined;
 constructor(directory:string,content:RemoteExecutionContentV1,maximumBytes:number,options:Readonly<{room?:(cacheBytes:number)=>number;local?:string}>={}){this.directory=directory;this.content=content;this.maximumBytes=maximumBytes;this.room=options.room;this.local=options.local;}

 private location(hash:string):string{return join(this.directory,hash.slice(0,2),hash);}

 async path(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal,tally?:RemoteExecutionInputFetchV1):Promise<string>{
  const file=this.location(digest.hash),known=await stat(file).catch(()=>undefined);
  if(known&&known.size===digest.size){const now=new Date();await utimes(file,now,now).catch(()=>{});if(tally){tally.cachedFiles++;tally.cachedBytes+=digest.size;}return file;}
  const key=instance+"|"+digest.hash;let running=this.pending.get(key);
  if(!running){running=this.fetch(instance,digest,file,signal,tally).finally(()=>{this.pending.delete(key);});this.pending.set(key,running);}
  return running;
 }

 /** 🔎️ Whether the blob is in the local copy at its size (no request). */
 async cached(digest:RemoteExecutionDigestV1):Promise<boolean>{return(await stat(this.location(digest.hash)).catch(()=>undefined))?.size===digest.size;}

 /** 📖️ The bytes of a small blob (a directory of an input tree) from the local copy, fetched once. */
 async bytes(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<Uint8Array>{
  if(digest.size===0)return new Uint8Array(0);
  return readFile(await this.path(instance,digest,signal));
 }

 private async fetch(instance:string,digest:RemoteExecutionDigestV1,file:string,signal?:AbortSignal,tally?:RemoteExecutionInputFetchV1):Promise<string>{
  await mkdir(dirname(file),{recursive:true});
  const temporary=`${file}.${process.pid}.${randomUUID()}.tmp`;
  if(digest.size===0){await writeFile(temporary,"");await rename(temporary,file);return file;}
  const copy=async(source:AsyncIterable<Uint8Array>):Promise<boolean>=>{
   const hash=createHash("sha256"),handle=createWriteStream(temporary),endStream=():Promise<void>=>new Promise((resolve,reject)=>{handle.once("error",reject);handle.end(()=>resolve());});let size=0;
   try{
    for await(const chunk of source){hash.update(chunk);size+=chunk.length;if(!handle.write(chunk))await new Promise<void>(resolve=>handle.once("drain",resolve));}
    await endStream();
    if(size!==digest.size||hash.digest("hex")!==digest.hash){await rm(temporary,{force:true});return false;}
    await rename(temporary,file);return true;
   }catch(error){try{await endStream();}catch{}await rm(temporary,{force:true});throw error;}
  };
  const near=this.local===undefined?undefined:casObjectPathV1(this.local,digest.hash);
  if(near!==undefined&&(await stat(near).catch(()=>undefined))?.size===digest.size&&await copy(createReadStream(near) as unknown as AsyncIterable<Uint8Array>).catch(()=>false)){if(tally){tally.localFiles++;tally.localBytes+=digest.size;}return file;}
  const source=await this.content.stream(instance,digest,signal);
  if(!source)throw new MissingBlob(digest);
  if(!await copy(source))throw new RemoteExecutionContentErrorV1(15,`Blob ${digest.hash} does not match its digest`);
  if(tally){tally.fetchedFiles++;tally.fetchedBytes+=digest.size;}
  return file;
 }

 /** 🧹️ Removes the least recently used blobs until the cache fits its bound. */
 async prune(dryRun=false):Promise<number>{
  const rows:{path:string;size:number;time:number}[]=[];let total=0;
  for(const shard of await readdir(this.directory).catch(()=>[] as string[])){
   for(const name of await readdir(join(this.directory,shard)).catch(()=>[] as string[])){
    const path=join(this.directory,shard,name),info=await stat(path).catch(()=>undefined);
    if(!info)continue;
    if(name.endsWith(".tmp")){if(Date.now()-info.mtimeMs>3600000)await rm(path,{force:true});continue;}
    rows.push({path,size:info.size,time:info.mtimeMs});total+=info.size;
   }
  }
  rows.sort((left,right)=>left.time-right.time);let removed=0;
  const bound=Math.min(this.maximumBytes,this.room?.(total)??Infinity);
  if(total<=bound)return 0;
  const protectedAfter=Date.now()-BLOB_PROTECT_MILLISECONDS_V1;
  for(const row of rows){if(total<=bound*0.9||row.time>protectedAfter)break;if(!dryRun)await rm(row.path,{force:true});total-=row.size;removed++;}
  return removed;
 }
}

const WORK_OWNER_FILE_V1=".owner";
const processAliveV1=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🏷️ Marks the work folder of this worker process with its owner, the proof of liveness every later sweep reads: the folder of a process that is gone holds the private roots of actions that were running when it ended. */
export async function claimRemoteExecutionWorkV1(work:string,pid:number=process.pid,now:number=Date.now()):Promise<void>{
 await mkdir(work,{recursive:true});
 await writeFile(join(work,WORK_OWNER_FILE_V1),JSON.stringify({version:1,pid,startedMs:now}));
}

/** 🪦️ Retires the private action roots of dead workers: every folder below `work/` except `keep` whose owner record names a process that is gone is removed at once, one without a readable record once it was untouched for `ownerlessSeconds`; a folder of a live owner is never touched (a pid that answers `EPERM` is alive). Answers how many worker folders and action roots went; with `dryRun` nothing is removed. Event-driven: a worker runs it when it starts and whenever an action ended, `repo cache gc` runs it for a machine without a worker. @see 👷️worker */
export async function retireRemoteExecutionWorkV1(work:string,rules:Readonly<{keep?:string;ownerlessSeconds:number;alive?:(pid:number)=>boolean;now?:number;dryRun?:boolean;signal?:AbortSignal}>):Promise<Readonly<{workers:number;roots:number}>>{
 const alive=rules.alive??processAliveV1,now=rules.now??Date.now();let workers=0,roots=0;
 for(const name of await readdir(work).catch(()=>[] as string[])){
  if(rules.signal?.aborted===true)break;
  if(name===rules.keep)continue;
  const folder=join(work,name),info=await lstat(folder).catch(()=>undefined);
  if(info===undefined||!info.isDirectory())continue;
  const owner=await readFile(join(folder,WORK_OWNER_FILE_V1),"utf8").then(text=>{try{return JSON.parse(text) as {pid?:unknown};}catch{return undefined;}},()=>undefined);
  const pid=typeof owner?.pid==="number"&&Number.isSafeInteger(owner.pid)?owner.pid:undefined;
  if(pid!==undefined?alive(pid):now-info.mtimeMs<rules.ownerlessSeconds*1000)continue;
  const tasks=(await readdir(folder).catch(()=>[] as string[])).filter(entry=>entry!==WORK_OWNER_FILE_V1).length;
  if(rules.dryRun!==true)await rm(folder,{recursive:true,force:true,maxRetries:3}).catch(()=>{});
  workers++;roots+=tasks;
 }
 return Object.freeze({workers,roots});
}

/** 📏️ The directory name of a worker below `work/`: at most 16 characters (a long host name is cut and made unique by a hash), so the input roots stay inside the 256-character path budget of native Windows. */
export function remoteExecutionWorkNameV1(name:string):string{
 const clean=name.replace(/[^A-Za-z0-9._-]/gu,"_");
 return clean.length<=16?clean:`${clean.slice(0,9)}-${sha256(name).slice(0,6)}`;
}

function safeName(name:string):string{
 if(name===""||name==="."||name===".."||name.includes("/")||name.includes("\\")||name.includes("\0"))throw new RemoteExecutionContentErrorV1(3,`Valid path segment required: ${JSON.stringify(name)}`);
 return name;
}

function safeRelative(path:string):string{
 const clean=normalize(path.replace(/\/+$/u,""));
 if(clean===""||clean==="."||isAbsolute(clean)||clean===".."||clean.startsWith(".."+sep))throw new RemoteExecutionContentErrorV1(3,`Relative path inside the input root required: ${path}`);
 return clean;
}

async function placeFile(source:string,target:string,executable:boolean,mode:"clone"|"copy"):Promise<void>{
 await copyFile(source,target,mode==="clone"?constants.COPYFILE_FICLONE:0);
 await chmod(target,executable?0o755:0o644);
}

function killTree(pid:number|undefined):void{
 if(pid===undefined)return;
 try{
  if(process.platform==="win32")spawnSync("taskkill",["/pid",String(pid),"/T","/F"],{windowsHide:true,stdio:"ignore"});
  else process.kill(-pid,"SIGKILL");
 }catch{}
}

function resolveExecutable(first:string,env:Readonly<Record<string,string>>,root:string,cwd:string):string|undefined{
 const extensions=process.platform==="win32"?["",...(env.PATHEXT??".EXE;.CMD;.BAT").split(";")]:[""];
 const usable=(path:string):boolean=>{for(const extension of extensions){try{if(statSync(path+extension).isFile()){accessSync(path+extension,constants.X_OK);return true;}}catch{}}return false;};
 if(first.includes("/")||first.includes("\\")){const full=isAbsolute(first)?first:remoteExecutionNativePathV1(root,first,{join,sep});return usable(full)?full:undefined;}
 for(const directory of (env.PATH??"").split(delimiter)){if(directory==="")continue;const full=join(isAbsolute(directory)?directory:join(cwd,directory),first);if(usable(full))return full;}
 return undefined;
}

type Collected={exitCode:number;stdout:Uint8Array;stderr:Uint8Array;timedOut:boolean;cancelled:boolean};
const LIMIT=64*1048576;

function execute(argv:readonly string[],env:Readonly<Record<string,string>>,cwd:string,timeoutMilliseconds:number,signal:AbortSignal,onSpawn?:(pid:number)=>void):Promise<Collected>{
 return new Promise(resolve=>{
  const out:Buffer[]=[],err:Buffer[]=[];let outSize=0,errSize=0,timedOut=false,cancelled=false,settled=false;
  const finish=(exitCode:number,extra?:string):void=>{
   if(settled)return;settled=true;clearTimeout(timer);signal.removeEventListener("abort",abort);
   if(extra)err.push(Buffer.from(extra));
   resolve({exitCode,stdout:Buffer.concat(out),stderr:Buffer.concat(err),timedOut,cancelled});
  };
  let child;
  try{child=spawn(argv[0]!,argv.slice(1),{cwd,env:{...env},stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true});}
  catch(error){finish(127,`Cannot start ${argv[0]}: ${(error as Error).message}\n`);return;}
  if(child.pid!==undefined&&process.platform!=="win32")onSpawn?.(child.pid);
  const abort=():void=>{cancelled=true;killTree(child.pid);},timer=setTimeout(()=>{timedOut=true;killTree(child.pid);},timeoutMilliseconds);
  signal.addEventListener("abort",abort,{once:true});
  child.stdout.on("data",(chunk:Buffer)=>{if(outSize<LIMIT){out.push(chunk);outSize+=chunk.length;}});
  child.stderr.on("data",(chunk:Buffer)=>{if(errSize<LIMIT){err.push(chunk);errSize+=chunk.length;}});
  child.on("error",error=>{finish(127,`Cannot start ${argv[0]}: ${error.message}\n`);});
  child.on("close",(code,received)=>{killTree(child.pid);finish(code??(received?128+((({SIGHUP:1,SIGINT:2,SIGQUIT:3,SIGABRT:6,SIGKILL:9,SIGSEGV:11,SIGPIPE:13,SIGTERM:15}) as Record<string,number>)[received]??0):1));});
  if(signal.aborted)abort();
 });
}

type Directory=ProtobufMessageV1;

/** 🚧️ The variable with which the worker names the real private input root to the action: a source-reading compiler plug-in (the mutation derives) never looks above it, because everything above is another machine's tree or refused by the sandbox, and answers a missing input as `E0463` so the client compiles the unit locally. */
export const REMOTE_EXECUTION_INPUT_ROOT_ENV_V1="SEMIO_OFFLOAD_INPUT_ROOT";

/** 🛡️ Whether a process end that looks like the operating system's kill (exit 137) was the host warden cancelling the compiler of this task: a cancel record the warden wrote after the task started names the pid of the task process or carries a command line below the private work directory of the task (the warden cancels the compiler leaf, not the task root), so the worker reports the task `refused` (`warden-cancelled`) and the scheduler places it on another machine instead of failing it.
 * @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts */
export function remoteExecutionWardenCancelledV1(input:Readonly<{exitCode:number;cancels:readonly Readonly<{pid:number;command:string}>[];work:string;pid?:number}>):boolean{
 if(input.exitCode!==137)return false;
 const prefix=input.work.split(sep).join("/").replace(/\/+$/u,"")+"/";
 return input.cancels.some(record=>(input.pid!==undefined&&record.pid===input.pid)||record.command.includes(prefix));
}

const wardenCancelsV1=async(sinceMs:number):Promise<readonly Readonly<{pid:number;command:string}>[]>=>(await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")).readHostWardenCancelsSinceV1(sinceMs);
const ensureWardenV1=():void=>{void import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts").then(module=>module.ensureHostWardenV1()).catch(()=>undefined);};

/** 🧰️ Executes one action the way the protocol describes: fetch the action, its command and its input tree, build a private input root, run the command in the sandbox with exactly the declared environment, collect the declared outputs into the store and answer an ActionResult. Failures of the infrastructure are statuses, failures of the action are exit codes. */
export async function executeRemoteExecutionActionV1(run:RemoteExecutionExecutionV1):Promise<RemoteExecutionExecutionResultV1>{
 const started=Date.now(),task=randomBytes(6).toString("hex"),work=join(run.work??join(run.directory,"work"),task),root=join(work,"r"),links:{target:string;path:string}[]=[],blobs=run.blobs??new RemoteExecutionBlobCacheV1(join(run.directory,"blobs"),run.content,Number.MAX_SAFE_INTEGER);
 const fail=(code:number,message:string,missing?:RemoteExecutionDigestV1[]):RemoteExecutionExecutionResultV1=>({outcome:"failed",status:{code,message,...(missing?{missing}:{})},actionResult:null});
 const directories=new Map<string,Directory>(),inputs:RemoteExecutionInputFetchV1={cachedFiles:0,cachedBytes:0,localFiles:0,localBytes:0,fetchedFiles:0,fetchedBytes:0};
 let surveyed:RemoteExecutionDigestV1|undefined;
 let busy=0;const waiting:(()=>void)[]=[];
 const place=async(work:()=>Promise<void>):Promise<void>=>{
  while(busy>=FETCH_CONCURRENCY_V1)await new Promise<void>(resolve=>waiting.push(resolve));
  busy++;
  try{await work();}finally{busy--;waiting.shift()?.();}
 };
 const loadDirectory=async(digest:RemoteExecutionDigestV1):Promise<Directory>=>{
  const known=directories.get(digest.hash);if(known)return known;
  const bytes=await blobs.bytes(run.instance,digest,run.signal);
  if(sha256(bytes)!==digest.hash)throw new RemoteExecutionContentErrorV1(15,`Directory ${digest.hash} does not match its digest`);
  const directory=decodeProtobufV1(REAPI_V1.directory,bytes);directories.set(digest.hash,directory);return directory;
 };
 const materialize=async(digest:RemoteExecutionDigestV1,target:string,depth:number):Promise<void>=>{
  if(depth>256)throw new RemoteExecutionContentErrorV1(3,"Input tree too deep");
  const directory=await loadDirectory(digest);
  await mkdir(target,{recursive:true});
  const seen=new Set<string>();
  for(const name of [...directory.files,...directory.directories,...directory.symlinks].map((node:{name:string})=>safeName(node.name))){if(seen.has(name))throw new RemoteExecutionContentErrorV1(3,`Duplicate name in input tree: ${name}`);seen.add(name);}
  await limited(directory.files as {name:string;digest:{hash:string;size_bytes:number};is_executable:boolean}[],16,node=>place(async()=>{
   const source=await blobs.path(run.instance,unwire(node.digest),run.signal,inputs);
   await placeFile(source,join(target,node.name),node.is_executable,run.materialize);
  }));
  for(const node of directory.symlinks as {name:string;target:string}[])links.push({target:node.target,path:join(target,node.name)});
  await limited(directory.directories as {name:string;digest:{hash:string;size_bytes:number}}[],8,node=>materialize(unwire(node.digest),join(target,node.name),depth+1));
 };
 const survey=async(top:RemoteExecutionDigestV1):Promise<RemoteExecutionDigestV1[]>=>{
  const gone=new Map<string,RemoteExecutionDigestV1>(),files=new Map<string,RemoteExecutionDigestV1>(),seen=new Set<string>();
  let level=[top];
  while(level.length>0){
   const next:RemoteExecutionDigestV1[]=[];
   await limited(level,16,async digest=>{
    if(seen.has(digest.hash))return;seen.add(digest.hash);
    let node:Directory;
    try{node=await loadDirectory(digest);}catch(error){if(error instanceof MissingBlob){gone.set(digest.hash,digest);return;}throw error;}
    for(const file of node.files as {digest:{hash:string;size_bytes:number}}[]){const found=unwire(file.digest);if(found.size>0)files.set(found.hash,found);}
    for(const child of node.directories as {digest:{hash:string;size_bytes:number}}[])next.push(unwire(child.digest));
   });
   level=next;
  }
  const unknown:RemoteExecutionDigestV1[]=[];
  for(const digest of files.values())if(!await blobs.cached(digest))unknown.push(digest);
  for(const digest of unknown.length===0?[]:await run.content.missing(run.instance,unknown,run.signal))gone.set(digest.hash,digest);
  return[...gone.values()];
 };
 try{
  const actionBytes=await run.content.read(run.instance,run.action,run.signal);
  if(!actionBytes)return fail(9,`Missing blob ${run.action.hash}/${run.action.size}`,[run.action]);
  const action=decodeProtobufV1(REAPI_V1.action,actionBytes),commandDigest=unwire(action.command_digest),commandBytes=await run.content.read(run.instance,commandDigest,run.signal);
  if(!commandBytes)return fail(9,`Missing blob ${commandDigest.hash}/${commandDigest.size}`,[commandDigest]);
  const command=decodeProtobufV1(REAPI_V1.command,commandBytes) as {arguments:string[];environment_variables:{name:string;value:string}[];output_files:string[];output_directories:string[];output_paths:string[];working_directory:string};
  if(command.arguments.length===0)return fail(3,"Command without arguments");
  if(run.properties!==undefined){
   const wanted=remoteExecutionMatchPropertiesV1((action.platform?.properties?.length?action.platform.properties:(command as {platform?:{properties?:readonly RemoteExecutionPropertyV1[]}}).platform?.properties)??[]),lacking=wanted.filter(row=>!remoteExecutionPlatformMatchesV1([row],run.properties!));
   if(lacking.length>0)return fail(9,`No worker offers the platform ${lacking.map(row=>`${row.name}=${row.value}`).join(";")}: this worker (${run.workerName}) does not advertise it`);
  }
  if(remoteExecutionCommandProgramV1(command.arguments[0]!)){
   const refusal=run.actions==="none"?"this worker runs no actions":run.commands===undefined?"this worker offers no command actions":validateRemoteExecutionCommandActionV1(command,{tools:run.commands,confined:run.sandbox.name!=="isolated-root",network:run.network===true});
   if(refusal!==undefined)return fail(9,`${OFFLOAD_REFUSAL_PREFIX_V1}: ${refusal}`);
  }else if(run.actions!=="any"){
   const refusal=run.actions==="none"?"this worker runs no actions":validateRemoteExecutionRustcActionV1(command);
   if(refusal!==undefined)return fail(9,`${OFFLOAD_REFUSAL_PREFIX_V1}: ${refusal}`);
  }
  if(command.output_paths.length===0&&(command.output_files.length>0||command.output_directories.length>0))return fail(3,"Command declares only REAPI 2.0 output_files or output_directories; declare output_paths (REAPI 2.1)");
  const rootDigest=action.input_root_digest?unwire(action.input_root_digest):{hash:EMPTY_HASH,size:0},fetchStart=Date.now();
  surveyed=rootDigest;
  await materialize(rootDigest,root,0);
  for(const link of links){
   if(remoteExecutionLinkEscapesV1(root,link.path,link.target))throw new RemoteExecutionContentErrorV1(3,`Input link leaves the input root: ${link.target}`);
   await placeRemoteExecutionLinkV1(link.target,link.path,root);
  }
  const cwdRelative=command.working_directory===""?"":safeRelative(command.working_directory),cwd=cwdRelative===""?root:join(root,cwdRelative),declared=command.output_paths;
  await mkdir(cwd,{recursive:true});
  for(const path of declared){const clean=safeRelative(path),directory=dirname(join(cwd,clean));await mkdir(directory,{recursive:true});}
  const env:Record<string,string>={},report=OFFLOAD_POLICY_V1.root.report;
  for(const variable of command.environment_variables)env[variable.name]=variable.value;
  const reported=env[report];delete env[report];
  for(const name of Object.keys(env))if(!name.startsWith("CARGO_PKG_")&&(env[name]===OFFLOAD_POLICY_V1.root.placeholder||env[name]!.startsWith(OFFLOAD_POLICY_V1.root.placeholder+"/")))env[name]=expandRemoteExecutionRootV1(env[name]!,root,{join,sep});
  if(reported!==undefined){const file=join(root,...safeRelative(reported).split(sep));await mkdir(dirname(file),{recursive:true});await writeFile(file,root);}
  const scratch=join(root,".tmp");await mkdir(scratch,{recursive:true});
  if(reported!==undefined)for(const directory of OFFLOAD_POLICY_V1.root.directories)await mkdir(remoteExecutionNativePathV1(root,directory,{join,sep}),{recursive:true});
  if(env.TMPDIR===undefined&&process.platform!=="win32")env.TMPDIR=scratch;
  env[REMOTE_EXECUTION_INPUT_ROOT_ENV_V1]=await realpath(root);
  const readable:string[]=[],args=command.arguments.map(argument=>expandRemoteExecutionRootV1(argument,root,{join,sep})),toolName=remoteExecutionToolOfProgramV1(args[0]!);
  const commandTool=parseCommandProgramV1(command.arguments[0]!);
  if(commandTool!==undefined){
   if(commandToolSourceV1(commandTool.name)==="resolved"){const tool=run.commands![commandToolKey(commandTool)]!;args[0]=tool.path;readable.push(...tool.readable);}
   else{
    const path=join(root,COMMAND_POLICY_V1.root.tool,commandTool.name);
    if(await fileSha256(path).catch(()=>"")!==commandTool.digest)return fail(9,`${OFFLOAD_REFUSAL_PREFIX_V1}: ${commandTextV1("en","refusal.shipped")} / ${commandTextV1("de","refusal.shipped")}`);
    args[0]=path;
   }
  }else if(toolName!==undefined){
   const tool=run.tools?.[toolName];
   if(!tool)return fail(9,`This worker does not offer the tool ${toolName}`);
   args[0]=tool.path;readable.push(...tool.readable);
  }
  const first=args[0]!,executable=resolveExecutable(first,env,root,cwd),executionStart=Date.now();
  const linked=executable!==undefined&&toolName==="rustc"&&offloadCrateTypesOfV1(args).includes("cdylib"),asked=linked&&!remoteExecutionCodegenSerialRequestedV1(args)?Math.max(1,Math.floor(await run.threads?.().catch(()=>1)??1)):1;
  let granted=1,taskPid:number|undefined;
  const collected:Collected=executable===undefined?{exitCode:127,stdout:new Uint8Array(0),stderr:new TextEncoder().encode(`Executable not found: ${first}\n`),timedOut:false,cancelled:false}:await withRemoteExecutionCodegenPoolV1(linked?asked:1,scratch,async pool=>{
   granted=pool===undefined?1:asked;
   const wrapped=run.sandbox.wrap({argv:[executable,...(linked?remoteExecutionCodegenArgumentsV1(args.slice(1),granted):args.slice(1))],env:pool===undefined?env:{...env,...pool},cwd,root,network:run.network===true,...(readable.length>0?{readable}:{})});
   return execute(wrapped.argv,wrapped.env,wrapped.cwd,run.timeoutSeconds*1000,run.signal,pid=>{taskPid=pid;run.meter?.attach(pid);});
  }),executionEnd=Date.now();
  if(collected.cancelled||run.signal.aborted)return {outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null};
  if(collected.timedOut)return fail(4,`Action exceeded its timeout of ${run.timeoutSeconds}s`);
  if(collected.exitCode===137&&remoteExecutionWardenCancelledV1({exitCode:collected.exitCode,cancels:await(run.wardenCancels??wardenCancelsV1)(executionStart).catch(()=>[]),work,...taskPid===undefined?{}:{pid:taskPid}}))return {outcome:"refused",status:{code:8,message:`${commandTextV1("en","refusal.warden-cancelled")} / ${commandTextV1("de","refusal.warden-cancelled")}`},actionResult:null,refusal:"warden-cancelled"};
  const uploadStart=Date.now(),uploads=new Map<string,{digest:RemoteExecutionDigestV1;source:Uint8Array|string}>();
  const stage=(digest:RemoteExecutionDigestV1,source:Uint8Array|string)=>{if(digest.size>0&&!uploads.has(digest.hash))uploads.set(digest.hash,{digest,source});};
  const hashFile=(path:string):Promise<string>=>new Promise((resolve,reject)=>{const hash=createHash("sha256");createReadStream(path).on("data",chunk=>hash.update(chunk)).on("error",reject).on("end",()=>{resolve(hash.digest("hex"));});});
  const fileNode=async(path:string,name:string)=>{const info=await stat(path),digest={hash:await hashFile(path),size:info.size};stage(digest,path);return {name,digest:wire(digest),is_executable:(info.mode&0o111)!==0};};
  const walk=async(path:string,collect:Directory[]):Promise<RemoteExecutionDigestV1>=>{
   const files:ProtobufMessageV1[]=[],children:ProtobufMessageV1[]=[],links:ProtobufMessageV1[]=[],entries=(await readdir(path)).sort();
   for(const name of entries){
    const full=join(path,name),info=await lstat(full);
    if(info.isSymbolicLink())links.push({name,target:await readlink(full)});
    else if(info.isDirectory())children.push({name,digest:wire(await walk(full,collect))});
    else if(info.isFile())files.push(await fileNode(full,name));
   }
   const message={files,directories:children,symlinks:links},bytes=encodeProtobufV1(REAPI_V1.directory,message),digest=digestOf(bytes);collect.push(message);stage(digest,bytes);return digest;
  };
  const outputFiles:ProtobufMessageV1[]=[],outputDirectories:ProtobufMessageV1[]=[],outputSymlinks:ProtobufMessageV1[]=[];
  const realRoot=await realpath(root);
  for(const path of declared){
   const clean=safeRelative(path),shown=clean.split(sep).join("/"),full=join(cwd,clean),parent=await realpath(dirname(full)).catch(()=>undefined);
   if(parent!==undefined&&parent!==realRoot&&!parent.startsWith(realRoot+sep))return fail(3,`Output path leaves the input root: ${path}`);
   const info=await lstat(full).catch(()=>undefined);
   if(!info)continue;
   if(info.isSymbolicLink())outputSymlinks.push({path:shown,target:await readlink(full)});
   else if(info.isDirectory()){
    const collect:Directory[]=[],rootDirectory=await walk(full,collect),rootMessage=collect[collect.length-1]!,tree={root:rootMessage,children:collect.slice(0,-1)},treeBytes=encodeProtobufV1(REAPI_V1.tree,tree),treeDigest=digestOf(treeBytes);
    stage(treeDigest,treeBytes);outputDirectories.push({path:shown,tree_digest:wire(treeDigest),root_directory_digest:wire(rootDirectory)});
   }else if(info.isFile())outputFiles.push({path:shown,...(await fileNode(full,clean).then(({digest,is_executable})=>({digest,is_executable})))});
  }
  const measured=run.meter?.peak(),stdoutDigest=digestOf(collected.stdout),stderrDigest=digestOf(collected.stderr);
  const inlineOut=collected.stdout.length<=run.inlineOutputBytes,inlineErr=collected.stderr.length<=run.inlineOutputBytes;
  if(!inlineOut)stage(stdoutDigest,collected.stdout);
  if(!inlineErr)stage(stderrDigest,collected.stderr);
  const outage=readRemoteExecutionPolicyV1().worker.outage,rows=[...uploads.values()],absent=new Set((await retryRemoteExecutionContentOutageV1(()=>run.content.missing(run.instance,rows.map(row=>row.digest),run.signal),outage,run.signal)).map(digest=>digest.hash));
  await limited(rows.filter(row=>absent.has(row.digest.hash)),8,row=>retryRemoteExecutionContentOutageV1(()=>{
   if(typeof row.source!=="string")return run.content.write(run.instance,row.digest,row.source,run.signal);
   const stream=createReadStream(row.source);
   return run.content.write(run.instance,row.digest,(async function*(){for await(const chunk of stream as unknown as AsyncIterable<Uint8Array>)yield chunk;})(),run.signal);
  },outage,run.signal));
  const end=Date.now(),result=encodeProtobufV1(REAPI_V1.actionResult,{
   output_files:outputFiles.sort((left,right)=>left.path<right.path?-1:1),output_directories:outputDirectories,output_symlinks:outputSymlinks,
   exit_code:collected.exitCode,...(inlineOut?{stdout_raw:collected.stdout}:{stdout_digest:wire(stdoutDigest)}),...(inlineErr?{stderr_raw:collected.stderr}:{stderr_digest:wire(stderrDigest)}),
   execution_metadata:{worker:run.workerName,auxiliary_metadata:[...measured===undefined?[]:[remoteExecutionPeakEntryV1(measured)],...linked?[remoteExecutionThreadsEntryV1(granted)]:[],remoteExecutionInputFetchEntryV1(inputs)],worker_start_timestamp:timestamp(started),input_fetch_start_timestamp:timestamp(fetchStart),input_fetch_completed_timestamp:timestamp(executionStart),execution_start_timestamp:timestamp(executionStart),execution_completed_timestamp:timestamp(executionEnd),output_upload_start_timestamp:timestamp(uploadStart),output_upload_completed_timestamp:timestamp(end),worker_completed_timestamp:timestamp(end)},
  });
  return {outcome:"completed",status:{code:0,message:""},actionResult:result};
 }catch(error){
  if(run.signal.aborted)return {outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null};
  if(error instanceof MissingBlob){
   const named=surveyed===undefined?[error.digest]:await survey(surveyed).catch(()=>[error.digest]),all=[error.digest,...named.filter(digest=>digest.hash!==error.digest.hash)];
   return fail(9,all.length===1?error.message:`${error.message} and ${all.length-1} more`,all);
  }
  if(error instanceof RemoteExecutionContentErrorV1)return fail(error.code,error.message);
  return fail(14,`Executing the action failed: ${error instanceof Error?error.message:String(error)}`);
 }finally{
  if(run.keepWork!==true)await rm(work,{recursive:true,force:true,maxRetries:3}).catch(()=>{});
 }
}

/** 🧾️ Collects the workers' home for `repo cache gc`: trims the shared blob cache to its bound and retires the private input roots of workers that are gone (their owner record names a dead process; a folder without one after `ownerlessWorkSeconds`); answers one language-neutral line for the ledger report. The home of the area `remote-execution-worker` (the sidecar agents' home inside its partition image) is the one the agents write, the area `remote-execution` is the checkout directory of the policy; a home whose image is not attached answers `absent`. */
export async function collectRemoteExecutionForLedgerV1(workspace:string,dryRun:boolean,signal:AbortSignal,area:string="remote-execution"):Promise<string>{
 const policy=readRemoteExecutionPolicyV1().worker;
 let home:string;
 if(area==="remote-execution-worker"){
  try{home=(await import("../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts")).cacheAreaHomeV1(workspace,area).directory;}catch{return "absent";}
 }else home=isAbsolute(policy.directory)?policy.directory:join(workspace,policy.directory);
 const cache=new RemoteExecutionBlobCacheV1(join(home,"blobs"),undefined as never,policy.maximumBlobBytes);
 if(!await stat(home).then(info=>info.isDirectory(),()=>false))return "absent";
 const retired=await retireRemoteExecutionWorkV1(join(home,"work"),{ownerlessSeconds:policy.ownerlessWorkSeconds,dryRun,signal});
 if(signal.aborted)return "cancelled";
 const removed=await cache.prune(dryRun);
 return `${dryRun?"would remove":"removed"} ${removed} blobs and ${retired.roots} input roots of ${retired.workers} dead workers`;
}

/** 📮️ Talks to the scheduler's worker API. */
function schedulerClient(base:string,token:string|undefined,call:typeof fetch=fetch){
 const root=base.replace(/\/+$/u,"");
 return async<T>(method:string,path:string,body:unknown,signal:AbortSignal,timeout=30000):Promise<T|undefined>=>{
  const response=await call(root+path,{method,signal:AbortSignal.any([signal,AbortSignal.timeout(timeout)]),headers:{"content-type":"application/json",...(token?{authorization:`Bearer ${token}`}:{})},...(body===undefined?{}:{body:JSON.stringify(body)})});
  if(response.status===204){await response.body?.cancel();return undefined;}
  if(!response.ok){const text=await response.text().catch(()=>"");throw Error(`Scheduler answered ${response.status}: ${text.trim()}`);}
  return await response.json() as T;
 };
}

/** 👷️ Runs a worker agent until its signal aborts: registers with the scheduler, long-polls for work while it has free slots, executes each action in its own private root and sandbox, heartbeats, honours cancellations, reports results and deregisters on the way out. */
export async function runRemoteExecutionWorkerV1(options:RemoteExecutionWorkerOptionsV1):Promise<void>{
 const call=schedulerClient(options.scheduler,options.token,options.fetch),say=(stage:RemoteExecutionWorkerEventV1["stage"],detail:string)=>options.onEvent?.({stage,detail});
 const sandbox=selectRemoteExecutionSandboxV1(options.sandbox),name=options.name??`${hostname()}-${process.pid}`,properties=options.properties??remoteExecutionHostPropertiesV1(),platform=options.platform??`${osPlatform()}-${process.arch}`;
 const confinement:RemoteExecutionConfinementV1=sandbox.name==="isolated-root"?"none":"full",network:RemoteExecutionNetworkV1=options.network==="allow"?"allowed":confinement==="full"?"denied":"unenforced",origin=options.origin??(name.replace(/[^A-Za-z0-9._-]/gu,"-").replace(/^[^A-Za-z0-9]+/u,"").slice(0,64)||"worker");
 if(options.confinement==="required"&&confinement==="none")throw Error(`Confinement required but this host has no confining sandbox (only ${sandbox.name})`);
 const actions:RemoteExecutionActionPolicyV1=options.actions??"rustc";
 if(actions==="any"&&confinement==="none"&&options.unconfinedForTests!==true)throw Error(`Running arbitrary actions needs a confining sandbox; this host has only ${sandbox.name}. Use actions "rustc" or "none".`);
 say("sandbox",`${sandbox.name} ${confinement==="full"?"confined":"UNCONFINED"} network ${network}`);
 ensureWardenV1();
 let registered:RemoteExecutionWorkerRegisteredV1|undefined,content=options.content;
 const meter=createRemoteExecutionMeterV1(remoteExecutionProcessTableV1,1000),running=new Map<string,AbortController>(),demands=new Map<string,number>(),grants=new Map<string,number>(),resumable=new Map<string,Readonly<{instance:string;action:{hash:string;size:number}}>>();
 const heldUnits=(tasks:readonly string[]):readonly RemoteExecutionHeldUnitV1[]=>{const used=new Map(meter.usage().map(row=>[row.task,row.megabytes]));return tasks.map(task=>({megabytes:demands.get(task)??0,usedMegabytes:used.get(task)??0}));};
 let registering:Promise<void>|undefined;
 const registerOnce=async():Promise<void>=>{
  for(let delay=250;!options.signal.aborted;delay=Math.min(delay*2,5000)){
   try{
    const answer=await call<RemoteExecutionWorkerRegisteredV1>("POST","/v1/workers",{version:1,name,host:hostname(),platform,properties,concurrency:options.concurrency,sandbox:sandbox.name,runtime:`${runtimeIdentityV1().id} ${runtimeIdentityV1().version}`,origin,...resumable.size>0?{resumed:[...resumable].map(([task,row])=>({task,instance:row.instance,action:row.action}))}:{},capacity:options.capacity??{cores:Math.max(1,availableParallelism()),memoryMegabytes:Math.max(1,Math.floor(totalmem()/1048576))},confinement,network},options.signal);
    requireRemoteExecutionShapeV1("WorkerRegistered",answer,"Closed registration answer required");
    registered=answer!;
    if(!options.content){
     const scheduler=new URL(options.scheduler),url=answer!.cache.relative?`${scheduler.protocol}//${scheduler.hostname.includes(":")?`[${scheduler.hostname}]`:scheduler.hostname}:${new URL(answer!.cache.url).port}`:answer!.cache.url;
     content=httpRemoteExecutionContentV1({endpoint:url,token:options.cacheToken,...(options.cacheFetch===undefined?{}:{fetch:options.cacheFetch})});
    }
    say("registered",`${answer!.id} cache ${answer!.cache.url}`);return;
   }catch(error){
    if(options.signal.aborted)return;
    say("reconnecting",(error as Error).message);await runtimeSleepV1(delay);
   }
  }
 };
 const register=():Promise<void>=>{registering??=registerOnce().finally(()=>{registering=undefined;});return registering;};
 await register();
 if(!registered||!content)return;
 const local=casRootV1(),keepWorkBytes=remoteExecutionDiskNeedBytesV1(readRemoteExecutionPolicyV1().worker.disk,options.concurrency);
 const room=(cacheBytes:number):number=>{const work=measureCasPartitionV1(options.directory),host=measureCasPartitionV1(local);return remoteExecutionBlobBoundBytesV1({maximumBytes:options.maximumBlobBytes,cacheBytes,workFreeBytes:work?.freeBytes,keepWorkBytes,hostFreeBytes:host?.freeBytes,hostReserveBytes:host===undefined?0:casReserveBytesV1(CAS_POLICY_V1,{totalBytes:host.totalBytes,role:"worker"})});};
 const store=new RemoteExecutionBlobCacheV1(join(options.directory,"blobs"),content,options.maximumBlobBytes,{room,local}),workDirectory=join(options.directory,"work",remoteExecutionWorkNameV1(name));
 const ownerlessSeconds=readRemoteExecutionPolicyV1().worker.ownerlessWorkSeconds,retire=():void=>{void retireRemoteExecutionWorkV1(join(options.directory,"work"),{keep:remoteExecutionWorkNameV1(name),ownerlessSeconds}).catch(()=>undefined);};
 await rm(workDirectory,{recursive:true,force:true}).catch(()=>{});
 await claimRemoteExecutionWorkV1(workDirectory).catch(()=>{});retire();
 await store.prune().catch(()=>0);let pruned=Date.now();
 const cancel=(ids:readonly string[]):void=>{for(const id of ids)running.get(id)?.abort();};
 let beating=false,beatLost=false,reportsUsage=true;
 const heartbeat=setInterval(()=>{
  if(beating)return;
  beating=true;const id=registered!.id;
  void call<RemoteExecutionHeartbeatAnswerV1>("POST",`/v1/workers/${id}/heartbeat`,{version:1,running:[...running.keys()],...(reportsUsage?{usage:meter.usage().filter(row=>running.has(row.task)).slice(0,512)}:{})},options.signal,10000).then(answer=>{beatLost=false;if(answer){if(!answer.known)void register();cancel(answer.cancel);}}).catch(error=>{if(reportsUsage&&/Scheduler answered 400/u.test((error as Error).message)){reportsUsage=false;say("reconnecting","heartbeat without memory usage: the scheduler does not know it");return;}if(!beatLost&&!options.signal.aborted)say("reconnecting",`heartbeat lost: ${(error as Error).message}`);beatLost=true;}).finally(()=>{beating=false;});
 },registered.heartbeatSeconds*1000);
 heartbeat.unref();
 let wake:(()=>void)|undefined,refusing=false;
 const publish=():void=>{options.observe?.({state:refusing?"refusing":running.size>0?"busy":"idle",concurrency:options.concurrency,running:running.size,sandbox:sandbox.name,confinement,network});};
 const send=async(task:string,report:RemoteExecutionTaskResultV1):Promise<void>=>{
  const outage=readRemoteExecutionPolicyV1().worker.outage,until=Date.now()+outage.seconds*1000;let told=false;
  for(let attempt=0;!options.signal.aborted;attempt++){
   try{await call("POST",`/v1/workers/${registered!.id}/tasks/${task}/result`,report,AbortSignal.timeout(30000));return;}
   catch(error){
    const message=(error as Error).message,answered=/^Scheduler answered (\d+)/u.exec(message),status=answered===null?0:Number(answered[1]);
    if(answered!==null&&status!==404&&status<500&&status!==408&&status!==429){say("reconnecting",`report refused: ${message}`);return;}
    if(Date.now()>=until){say("reconnecting",`report lost after ${outage.seconds} s: ${message}`);return;}
    if(!told){told=true;say("reconnecting",`report of ${task} kept, retrying: ${message}`);}
    if(status===404)await register();
    await runtimeSleepV1(Math.min(outage.maximumMilliseconds,Math.max(200,outage.initialMilliseconds/4)*2**Math.min(attempt,10)));
   }
  }
 };
 const finishTask=async(task:RemoteExecutionTaskV1,controller:AbortController):Promise<void>=>{
  ensureWardenV1();
  say("started",`${task.id} ${task.action.hash.slice(0,12)}`);
  const limit=setTimeout(()=>{controller.abort();},(task.timeoutSeconds+readRemoteExecutionPolicyV1().server.cancelGraceSeconds)*1000);
  const result=await executeRemoteExecutionActionV1({properties,instance:task.instance,action:task.action,timeoutSeconds:task.timeoutSeconds,content:content!,directory:options.directory,work:workDirectory,workerName:name,sandbox,network:network==="allowed",actions,materialize:options.materialize,inlineOutputBytes:options.inlineOutputBytes,signal:controller.signal,meter:{attach:pid=>{meter.attach(task.id,pid);},peak:()=>meter.finish(task.id)},...(options.threads?{threads:async()=>{const others=[...running.keys()].filter(id=>id!==task.id),count=await options.threads!({others:others.reduce((sum,id)=>sum+(grants.get(id)??1),0),peakMegabytes:task.peakMegabytes??0,held:heldUnits(others)});grants.set(task.id,count);return count;}}:{}),...(options.keepWork?{keepWork:true}:{}),...(options.tools?{tools:options.tools}:{}),...(options.commands?{commands:options.commands}:{}),blobs:store}).finally(()=>{clearTimeout(limit);meter.finish(task.id);});
  const report:RemoteExecutionTaskResultV1={version:1,outcome:result.outcome,status:result.status,actionResult:result.actionResult?Buffer.from(result.actionResult).toString("base64"):null,...result.outcome==="refused"?{refusal:result.refusal??"warden-cancelled"}:{}};
  await send(task.id,report);
  say(result.outcome==="cancelled"?"cancelled":result.outcome==="refused"?"refused":"finished",`${task.id} ${result.outcome} ${result.status.code}`);
  running.delete(task.id);demands.delete(task.id);grants.delete(task.id);resumable.delete(task.id);publish();wake?.();
  if(Date.now()-pruned>30000){pruned=Date.now();void store.prune().catch(()=>{});}
  retire();
 };
 try{
  while(!options.signal.aborted){
   if(running.size>=options.concurrency){refusing=false;publish();await new Promise<void>(resolve=>{wake=resolve;options.signal.addEventListener("abort",()=>{resolve();},{once:true});});continue;}
   if(options.admit){
    const verdict=await options.admit(running.size,{stage:"poll",held:heldUnits([...demands.keys()])}).catch(()=>({admit:true as const}));
    refusing=!verdict.admit;publish();
    if(!verdict.admit){await new Promise<void>(resolve=>{const timer=setTimeout(resolve,options.admitRetryMilliseconds??2000);wake=()=>{clearTimeout(timer);resolve();};options.signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});});continue;}
   }
   let answer:RemoteExecutionPollAnswerV1|undefined;
   try{answer=await call<RemoteExecutionPollAnswerV1>("POST",`/v1/workers/${registered.id}/poll?wait=${registered.pollSeconds*1000}`,{version:1},options.signal,registered.pollSeconds*1000+15000);}
   catch(error){if(options.signal.aborted)break;say("reconnecting",(error as Error).message);await runtimeSleepV1(1000);continue;}
   if(!answer)continue;
   if(!answer.known){await register();continue;}
   cancel(answer.cancel);
   if(answer.task){
    const verdict=options.admit?await options.admit(running.size,{stage:"task",megabytes:answer.task.peakMegabytes??0,held:heldUnits([...demands.keys()])}).catch(()=>({admit:true as const})):{admit:true as const};
    if(!verdict.admit){
     refusing=true;publish();say("refused",`${answer.task.id} ${verdict.reason}`);
     await send(answer.task.id,{version:1,outcome:"refused",status:{code:8,message:`The machine has no headroom: ${verdict.reason}`},actionResult:null,refusal:verdict.reason});
     continue;
    }
    const controller=new AbortController();running.set(answer.task.id,controller);demands.set(answer.task.id,answer.task.peakMegabytes??0);resumable.set(answer.task.id,{instance:answer.task.instance,action:{hash:answer.task.action.hash,size:answer.task.action.size}});publish();void finishTask(answer.task,controller);
   }
  }
 }finally{
  clearInterval(heartbeat);meter.close();
  for(const controller of running.values())controller.abort();
  const settle=Date.now()+10000;while(running.size>0&&Date.now()<settle)await runtimeSleepV1(50);
  await call("DELETE",`/v1/workers/${registered.id}`,undefined,AbortSignal.timeout(5000)).catch(()=>{});
  say("stopped",name);
 }
}
