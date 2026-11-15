import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {dropSpillObjectsV1,newestEntryV1,emitUnitEventV1,readSpillObjectV1} from "../../💾️store/🟦️.ts";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {readCasObjectSyncV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../📡️client/🟦️.ts";
import {remoteCacheFetchV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {FLEET_POLICY_V1,portableRootsV1,portableTextV1,type PlatformV1} from "../🪪️identity/🟦️.ts";
import {adoptFleetUnitV1} from "../📥️adopt/🟦️.ts";
import {packUnitV1,type RemoteUnitV1} from "../📦️wire/🟦️.ts";
import {countUnitV1,writeBuildCountersV1} from "../📊️counters/🟦️.ts";

const fleet=FLEET_POLICY_V1,limits=fleet.limits,sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),decoder=new TextDecoder("utf-8",{fatal:true}),encoder=new TextEncoder();
type Environment=Readonly<Record<string,string|undefined>>;

/** 🧾️ One unit waiting for upload (`SpoolJob` of the schema). */
export type SpoolJobV1={version:1;key:string;localKey:string;slot:string;crate:string;cwd:string;workspace:string|null;laneRoot:string|null;owner:string;claimed:boolean;offer?:boolean;costMs:number;platform:PlatformV1;queuedMs:number;attempts?:number;notBeforeMs?:number};

const spoolDirectory=(root:string)=>join(root,fleet.layout.state,fleet.layout.spool);
const offeredPath=(root:string,key:string)=>join(root,fleet.layout.state,"offered",key.slice(0,2),key);

/** 🏷️ Whether this machine already settled the unit with the fleet cache (uploaded it, found it there, or installed it from there). */
export function offeredV1(root:string,key:string):boolean{try{statSync(offeredPath(root,key));return true;}catch{return false;}}
/** 🏷️ Remembers that the unit is settled, so later local hits do not offer it again. */
export function markOfferedV1(root:string,key:string):void{try{const path=offeredPath(root,key);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,"");}catch{}}

/** 📮️ Queues a unit for upload: one small file renamed into the spool, `false` (the unit is dropped) when the spool is full. Never waits for the network. */
export function enqueueUploadV1(root:string,job:SpoolJobV1):boolean{
 try{
  const directory=spoolDirectory(root);mkdirSync(directory,{recursive:true});
  if(readdirSync(directory).filter(name=>name.endsWith(".json")).length>=limits.maximumSpool)return false;
  const temporary=join(directory,"."+randomBytes(6).toString("hex")+".tmp"),name=(job.offer?"offer-":String(job.queuedMs).padStart(15,"0")+"-")+job.key.slice(0,16)+".json";
  writeFileSync(temporary,JSON.stringify(job));renameSync(temporary,join(directory,name));return true;
 }catch{return false;}
}

/** 🔢️ How many units wait in the spool. */
export function spoolLengthV1(root:string):number{try{return readdirSync(spoolDirectory(root)).filter(name=>name.endsWith(".json")).length;}catch{return 0;}}

const scriptOf=()=>{const path=fileURLToPath(new URL("../../📜️script.ts",import.meta.url));return existsSync(path)?path:undefined;};

/** 🚀️ Starts the upload worker when none is alive: one detached process per machine (a lock file with a heartbeat names it), without any inherited descriptor, so Cargo never waits for it and the compiler's exit is not delayed by uploads. A start storm is throttled. */
export function startUploaderV1(root:string,environment:Environment):void{
 try{
  const directory=join(root,fleet.layout.state);mkdirSync(directory,{recursive:true});
  const lock=join(directory,"uploader.lock"),stamp=join(directory,fleet.layout.spawned);
  try{if(Date.now()-statSync(lock).mtimeMs<fleet.uploader.staleMilliseconds)return;}catch{}
  try{if(Date.now()-statSync(stamp).mtimeMs<fleet.uploader.spawnThrottleMilliseconds)return;}catch{}
  writeFileSync(stamp,String(Date.now()));
  const script=scriptOf(),env:Record<string,string>={};
  for(const [name,value] of Object.entries(environment)){if(value!==undefined&&!/^(CARGO_MAKEFLAGS|MAKEFLAGS|MFLAGS)$/u.test(name))env[name]=value;}
  const child=spawn(process.execPath,[...(script?[script]:[]),"--semio-unit-cache","fleet-upload","--root",root],{env,detached:true,stdio:"ignore",windowsHide:true});
  child.on("error",()=>{});child.unref();
 }catch{}
}

async function bounded<T>(items:readonly T[],concurrency:number,work:(item:T)=>Promise<boolean>):Promise<boolean>{
 let next=0,ok=true;
 await Promise.all(Array.from({length:Math.min(concurrency,items.length)},async()=>{for(;;){const index=next++;if(index>=items.length)return;if(!await work(items[index]!))ok=false;}}));
 return ok;
}

/** 📤️ Builds and uploads one spooled unit (`"diverged"`: the fleet already holds this key with other bytes, a determinism bug or an attack; the caller adopts the fleet's copy): reads the local entry and its objects, turns every path into its token, uploads the blobs the server lacks and then the action result (which makes the server wake every waiter). `"retry"` when the unit was built and the fleet cache did not take it (network or server fault: the drain queues it again, see `retryJobV1`), `false` on any other fault: the unit is simply not shared. */
export async function uploadUnitV1(client:FleetClientV1,root:string,job:SpoolJobV1):Promise<boolean|"diverged"|"retry">{
 const entry=newestEntryV1(root,job.localKey);if(!entry)return false;
 const roots=portableRootsV1({cwd:job.cwd,workspace:job.workspace??undefined}),contents=new Map<string,Uint8Array>(),cas=casRootV1();
 let total=0;
 const convert=(sha256:string,text:boolean):string|undefined=>{
  const bytes=readCasObjectSyncV1(cas,sha256)??readSpillObjectV1(root,sha256);if(!bytes)return undefined;
  const out=text?encoder.encode(portableTextV1(decoder.decode(bytes),roots)):new Uint8Array(bytes);total+=out.length;
  if(out.length>limits.maximumBlobBytes||total>limits.maximumEntryBytes)return undefined;
  const hash=sha(out);contents.set(hash,out);return hash;
 };
 try{
  const outputs:RemoteUnitV1["outputs"]=[];
  for(const output of entry.outputs){
   const hash=convert(output.sha256,output.template===true);if(hash===undefined)return false;
   outputs.push({name:output.name,sha256:hash,bytes:contents.get(hash)!.length,executable:output.executable,...(output.template?{template:true}:{})});
  }
  const stream=(value:string|null|undefined):string|null|undefined=>value==null?null:convert(value,true);
  const stdout=stream(entry.stdout),stderr=stream(entry.stderr);if(stdout===undefined||stderr===undefined)return false;
  const unit:RemoteUnitV1={version:1,key:job.key,localKey:job.localKey,slot:job.slot,crate:job.crate,platform:job.platform,createdMs:entry.createdMs,costMs:job.costMs,dependencies:entry.dependencies.map(row=>({path:portableTextV1(row.path,roots),sha256:row.sha256})),environment:entry.environment.map(row=>({name:row.name,value:row.value===null?null:portableTextV1(row.value,roots)})),outputs,stdout,stderr};
  const packed=packUnitV1(unit,contents),hashes=[...packed.blobs.keys()];
  const renewing=job.claimed?setInterval(()=>{void client.renew(job.key,job.owner,fleet.claim.ttlMilliseconds);},fleet.claim.heartbeatMilliseconds):undefined;renewing?.unref?.();
  try{
   const sent=await bounded(hashes,limits.maximumUploadConcurrency,async hash=>{
    if(await client.hasBlob(hash)===true)return true;
    return client.putBlob(hash,packed.blobs.get(hash)!);
   });
   if(!sent)return "retry";
   const stored=await client.putResult(job.key,packed.result,job.slot);
   return stored===false?"retry":stored;
  }finally{if(renewing)clearInterval(renewing);}
 }catch{return false;}
}

/** 🔁️ The job to queue again after a failed upload, `undefined` when it has used its attempts or is too old: a settled offer (the next drain first asks the server whether another machine stored the unit meanwhile) due after an exponentially growing delay, without the claim (a waiting machine takes the unit over). */
export function retryJobV1(job:SpoolJobV1,now:number):SpoolJobV1|undefined{
 const attempts=(job.attempts??0)+1,policy=fleet.uploader;
 if(attempts>=policy.maximumAttempts||now-job.queuedMs>policy.maximumJobAgeMilliseconds)return undefined;
 return{...job,claimed:false,offer:true,attempts,notBeforeMs:now+policy.retryBackoffMilliseconds*2**(attempts-1)};
}

/** 🚰️ The summary of one drain. */
export type DrainResultV1=Readonly<{uploaded:number;failed:number;dropped:number;retried?:number}>;

/** 🚰️ Drains the spool until it has been empty for `idleMilliseconds` (or once, with `idleMilliseconds` 0): one drainer per machine holds the lock, takes units oldest first, uploads a few at a time, deletes each job file whatever the outcome (a failed unit is dropped, never retried) and releases the claim of a unit it could not upload so a waiter can take over. */
export async function drainUploadsV1(root:string,environment:Environment,options:Readonly<{idleMilliseconds?:number;signal?:AbortSignal}>={}):Promise<DrainResultV1|"busy">{
 const settings=readFleetSettingsV1(root,environment);if(!settings)return "busy";
 const directory=join(root,fleet.layout.state);mkdirSync(directory,{recursive:true});
 const lock=tryClaimV1(directory,"uploader");if(!lock)return "busy";
 const client=createFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||root})}),idle=options.idleMilliseconds??fleet.uploader.idleMilliseconds,spool=spoolDirectory(root),result:{uploaded:number;failed:number;dropped:number;retried?:number}={uploaded:0,failed:0,dropped:0};
 let quietSince=Date.now();
 try{
  for(;;){
   if(options.signal?.aborted)break;
   let names:string[]=[];try{names=readdirSync(spool).filter(name=>name.endsWith(".json")).sort();}catch{}
   if(names.length===0){if(Date.now()-quietSince>=idle)break;await new Promise(done=>setTimeout(done,fleet.uploader.pollMilliseconds));continue;}
   quietSince=Date.now();
   const due=names.filter(name=>{try{return((JSON.parse(readFileSync(join(spool,name),"utf8")) as SpoolJobV1).notBeforeMs??0)<=Date.now();}catch{return true;}});
   if(due.length===0){if(options.idleMilliseconds===0)break;await new Promise(done=>setTimeout(done,fleet.uploader.pollMilliseconds));continue;}
   names=due;
   await bounded(names.slice(0,limits.maximumUploadConcurrency*2),limits.maximumUploadConcurrency,async name=>{
    const path=join(spool,name);let job:SpoolJobV1;
    try{job=JSON.parse(readFileSync(path,"utf8")) as SpoolJobV1;}catch{rmSync(path,{force:true});result.dropped++;return false;}
    rmSync(path,{force:true});
    const started=Date.now(),event=(kind:"remote-publish"|"remote-skip",reason:string)=>emitUnitEventV1(root,{lane:"uploader",kind,crate:job.crate,key:job.key,milliseconds:Date.now()-started,waitedMs:Math.max(0,started-job.queuedMs),reason});
    if(started-job.queuedMs>fleet.uploader.maximumJobAgeMilliseconds||!client.usable()){result.dropped++;event("remote-skip","upload-dropped");if(job.claimed)await client.release(job.key,job.owner);return false;}
    if(job.offer===true&&await client.hasResult(job.key)===true){markOfferedV1(root,job.key);result.dropped++;event("remote-skip","already-stored");return true;}
    const outcome=await Promise.race([uploadUnitV1(client,root,job),new Promise<"retry">(done=>{const timer=setTimeout(()=>done("retry"),fleet.uploader.jobMilliseconds);timer.unref?.();})]),ok=outcome===true;
    if(outcome==="diverged"){
     const adopted=await adoptFleetUnitV1(client,root,job);
     result.failed++;countUnitV1(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT,"skipped");event("remote-skip",adopted?"diverged: the fleet holds other bytes for this key, its copy replaces the local entry":"diverged: the fleet holds other bytes for this key and its copy could not be fetched");
     if(job.claimed)await client.release(job.key,job.owner);
    }else if(ok){markOfferedV1(root,job.key);result.uploaded++;countUnitV1(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT,"stored");event("remote-publish","uploaded");}
    else{
     const again=outcome==="retry"?retryJobV1(job,Date.now()):undefined,queued=again!==undefined&&enqueueUploadV1(root,again);
     if(queued){result.retried=(result.retried??0)+1;event("remote-skip","upload-retry "+again.attempts+"/"+fleet.uploader.maximumAttempts);}
     else{result.failed++;event("remote-skip",outcome==="retry"?"upload-failed after "+((job.attempts??0)+1)+" attempts":"upload-failed");}
     if(job.claimed)await client.release(job.key,job.owner);
     if(queued)return false;
    }
    {const spilled=newestEntryV1(root,job.localKey);if(spilled)dropSpillObjectsV1(root,spilled);}
    return ok;
   });
  }
 }finally{lock.release();if(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT)writeBuildCountersV1(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT);}
 return result;
}

