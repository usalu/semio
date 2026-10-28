import {constants,createReadStream,existsSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {chmod,copyFile,lstat,mkdir,readdir,readFile,rename,rm,rmdir,stat,utimes,writeFile} from "node:fs/promises";
import {createHash,randomBytes} from "node:crypto";
import {dirname,join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {waitingDelayV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import snapshotSchema from "../../📸️snapshot/🧬️schema/🔣️.json" with {type:"json"};
import {readBazelServerPidSyncV1,readBazelSlotStateSyncV1} from "../📖️status/🟦️.ts";
import {BAZEL_SESSION_POLICY_V1 as policy,bazelGrowthSlotIdsV1,bazelSessionMessageV1,bazelSlotIdsV1,nextBazelSlotStateV1,planBazelServerReapV1,planBazelSessionGarbageV1,requireBazelKeyV1,requireBazelSlotV1,type BazelGarbageV1,type BazelInventoryV1,type BazelLanguageV1,type BazelServerPlanV1,type BazelServerRowV1,type BazelSlotKindV1,type BazelSlotPortV1,type BazelSlotStateV1,type BazelSlotWaiterV1} from "../🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const temporaryName=snapshotSchema.definitions.Policy.const.temporary,slotPattern=new RegExp(policy.names.slotPattern),keyPattern=new RegExp(policy.names.keyPattern),missing=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""),busy=(error:unknown)=>error instanceof Error&&error.message.startsWith("Resource lease timed out: ");
export type BazelSlotDirectoriesV1=Readonly<{leases:string;server:string;workspaces:string;pool?:string}>;

const alivePid=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const waiterPattern=(kind:BazelSlotKindV1)=>new RegExp(`^waiter-${kind}-[0-9]{15}-([0-9]+)-[0-9a-f]{12}$`,"u");
/** 🚶️ The waiters of one slot class still alive, oldest first: one small file per waiter beside the slot states, named by class, arrival time, process and a random token (the inventory ignores plain files of other names); the file of a dead process is removed when seen. */
function liveWaiters(server:string,kind:BazelSlotKindV1):string[]{
 if(!existsSync(server))return [...localWaiters.get(server+"\0"+kind)??[]];
 const pattern=waiterPattern(kind);let names:string[];try{names=readdirSync(server).filter(name=>pattern.test(name)).sort();}catch{return [];}
 return names.filter(name=>{if(alivePid(Number(pattern.exec(name)![1])))return true;try{rmSync(join(server,name),{force:true});}catch{}return false;});
}
const localWaiters=new Map<string,string[]>();
/** ⏳️ Registers the caller as the newest slot waiter of its class; below an unmounted server partition nothing is written and the waiters of this process queue in memory. */
function registerWaiter(server:string,kind:BazelSlotKindV1):BazelSlotWaiterV1{
 if(!existsSync(server)){const queue=localWaiters.get(server+"\0"+kind)??[],name=randomBytes(6).toString("hex");queue.push(name);localWaiters.set(server+"\0"+kind,queue);return Object.freeze({first:()=>queue[0]===name,leave:()=>{const at=queue.indexOf(name);if(at>=0)queue.splice(at,1);}});}
 const name=`waiter-${kind}-${String(Date.now()).padStart(15,"0")}-${process.pid}-${randomBytes(6).toString("hex")}`;writeFileSync(join(server,name),"");
 return Object.freeze({first:()=>liveWaiters(server,kind)[0]===name,leave:()=>{try{rmSync(join(server,name),{force:true});}catch{}}});
}

/** 🔌️ Maps the session port onto repository resource leases (one exclusive lease per slot, one fair queued lease for the short picker critical section, one small file per slot waiter) and one remembered-state file per slot beside its output base; a state is valid only while the slot's output base exists, so a reset, a cache clear or an unmounted partition never leaves a stale affinity behind and nothing is ever written below an unmounted partition; the lease directory is the checkout's own, so sessions of different checkouts never meet. */
export function bazelSlotPortV1(directories:BazelSlotDirectoriesV1,options:Readonly<{control?:ResourceLeaseControl;write?:(line:string)=>void;spare?:()=>Promise<number>;floorSpare?:()=>Promise<number>;imageFree?:()=>number|undefined}>={}):BazelSlotPortV1{
 let sampled={at:0,value:0},sampledFloor={at:0,value:0};const control=options.control?{control:options.control}:{},file=(slot:string)=>join(directories.server,requireBazelSlotV1(slot)+policy.names.stateSuffix);
 return Object.freeze({
  async claim(slot,signal){try{const lease=await acquireResourceLease({directory:directories.leases,resource:policy.leases.slotPrefix+requireBazelSlotV1(slot),mode:"exclusive",signal,timeoutMs:0,...control});return()=>lease.release();}catch(error){if(busy(error))return undefined;throw error;}},
  async gate(signal,kind){const lease=await acquireQueuedResourceLease({directory:directories.leases,resource:kind==="build"?policy.leases.pickerResource:policy.leases.servicePickerResource,mode:"exclusive",owner:randomBytes(12).toString("base64url"),signal,...control});return()=>lease.release();},
  queued:kind=>liveWaiters(directories.server,kind).length,
  async spare(){if(Date.now()-sampled.at<policy.limits.spareSampleMilliseconds)return sampled.value;try{sampled={at:Date.now(),value:await options.spare?.()??0};}catch{sampled={at:Date.now(),value:0};}return sampled.value;},
  async floorSpare(){if(Date.now()-sampledFloor.at<policy.limits.spareSampleMilliseconds)return sampledFloor.value;try{sampledFloor={at:Date.now(),value:await options.floorSpare?.()??Infinity};}catch{sampledFloor={at:Date.now(),value:Infinity};}return sampledFloor.value;},
  async imageFree(){return options.imageFree?.()??Infinity;},
  enqueue:kind=>registerWaiter(directories.server,kind),
  async read(slot){
   if(!existsSync(join(directories.server,requireBazelSlotV1(slot))))return undefined;
   return readBazelSlotStateSyncV1(directories.server,slot);
  },
  async write(slot,state){if(!existsSync(join(directories.server,requireBazelSlotV1(slot))))return;const target=file(slot),temporary=`${target}.${process.pid}.tmp`;await writeFile(temporary,JSON.stringify(state)+"\n");await rename(temporary,target);},
  sleep:(milliseconds,signal)=>waitingDelayV1(milliseconds,undefined,{signal}).then(()=>undefined),
  now:()=>Date.now(),
  progress:line=>{(options.write??console.log)(line);},
 });
}

const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
/** 🛑️ Stops the persistent server of an output base the way Bazel itself notices a shutdown request, by removing its pid file, then waits for the process to end; it needs neither the Bazel binary nor a workspace and starts no JVM. An output base without a server is already stopped. */
export async function stopBazelServerV1(outputBase:string,signal:AbortSignal,rules:{timeoutMilliseconds:number;pollMilliseconds:number}=policy.stop):Promise<void>{
 const file=join(outputBase,policy.stop.pidFile);let pid:number;
 try{pid=Number((await readFile(file,"utf8")).trim());await rm(file,{force:true});}catch(error){if(missing(error))return;throw error;}
 if(!Number.isSafeInteger(pid)||pid<1)return;
 const deadline=Date.now()+rules.timeoutMilliseconds;
 while(alive(pid)){signal.throwIfAborted();if(Date.now()>=deadline)throw Error("A Bazel server did not stop after its pid file was removed");await delay(rules.pollMilliseconds,undefined,{signal});}
}

/** 🔎️ The pid of the server living in an output base: its pid file names a process that still exists; undefined when there is none or the file is stale. */
export async function readBazelServerPidV1(outputBase:string):Promise<number|undefined>{return readBazelServerPidSyncV1(outputBase);}

/** 🧹️ Keeps the live servers of the host within its budget: when a session starts a server, the idle servers beyond what the budget leaves next to the held slots are stopped, least recently used first, each one only while the reaper holds its slot's claim (so no session can connect to a server that is going away) and without the Bazel binary. A server that does not stop is reported and left to its idle exit; the build never fails because of it. Returns the stopped slots. */
export async function reapBazelServersV1(directories:BazelSlotDirectoriesV1,port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,signal:AbortSignal,stop:(outputBase:string)=>Promise<void>,options:Readonly<{budget:number;progress?:(line:string)=>void;language:BazelLanguageV1}>):Promise<readonly string[]>{
 const slots=[...bazelSlotIdsV1(plan,"build"),...bazelSlotIdsV1(plan,"service"),...bazelGrowthSlotIdsV1(plan)],live=new Set<string>();
 for(const slot of slots)if(await readBazelServerPidV1(join(directories.server,slot))!==undefined)live.add(slot);
 if(!live.size)return Object.freeze([]);
 const rows:BazelServerRowV1[]=[];
 for(const slot of slots){signal.throwIfAborted();const probe=await port.claim(slot,signal);probe?.();rows.push({slot,live:live.has(slot),held:probe===undefined,usedAt:(await port.read(slot))?.usedAt??0});}
 const doomed=planBazelServerReapV1(rows,options.budget,await port.floorSpare()),stopped:string[]=[];
 for(const slot of doomed){
  const claimed=await port.claim(slot,signal);if(claimed===undefined)continue;
  try{await stop(join(directories.server,slot));stopped.push(slot);options.progress?.(bazelSessionMessageV1("reaped",{slot,live:live.size,budget:options.budget},options.language));}
  catch(error){signal.throwIfAborted();options.progress?.(bazelSessionMessageV1("reap-failed",{slot,reason:(error as Error).message},options.language));}
  finally{claimed();}
 }
 return Object.freeze(stopped);
}

/** 🗑️ Removes a tree whose Bazel output directories may be read-only: a refused removal makes the tree writable and retries once. */
export async function removeBazelTreeV1(path:string):Promise<void>{
 try{await rm(path,{recursive:true,force:true});}
 catch(error){if(!["EACCES","EPERM","ENOTEMPTY"].includes((error as NodeJS.ErrnoException).code??""))throw error;await writable(path);await rm(path,{recursive:true,force:true});}
}
async function writable(path:string):Promise<void>{
 const facts=await lstat(path).catch(error=>{if(missing(error))return undefined;throw error;});if(facts===undefined||facts.isSymbolicLink())return;
 if(facts.isDirectory()){await chmod(path,0o700);for(const name of await readdir(path))await writable(join(path,name));}else await chmod(path,0o600);
}

/** 🧬️ Turns the identical large launcher files directly below the `bin` folders of one IDLE output base (one per test, each a copy of the pinned runtime, made before launchers were cloned) into clones of one file kept in the base's `launchers` folder: byte-identical files share their blocks (APFS, btrfs and XFS; any other volume refuses the forced clone and nothing changes), modes and modification times stay, and the next build re-runs only the cheap launcher action because its output file changed. A stamp keeps a repeated pass from touching files it already handled. The caller must hold the slot's claim. Returns the files replaced and their logical bytes. */
export async function dedupeBazelLaunchersV1(outputBase:string,signal:AbortSignal):Promise<Readonly<{files:number;bytes:number}>>{
 const rules=policy.dedupe,shared=join(outputBase,rules.directory),stampPath=join(shared,rules.stamp),since=await stat(stampPath).then(facts=>facts.mtimeMs,()=>0),started=Date.now(),candidates:{path:string;size:number;mode:number;atime:Date;mtime:Date}[]=[],roots=join(outputBase,"execroot","_main","bazel-out");
 for(const config of await readdir(roots,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;})){
  if(!config.isDirectory())continue;const bin=join(roots,config.name,"bin");
  for(const entry of await readdir(bin,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;})){
   signal.throwIfAborted();if(!entry.isFile())continue;const path=join(bin,entry.name),facts=await lstat(path).catch(error=>{if(missing(error))return undefined;throw error;});
   if(facts?.isFile()&&facts.size>=rules.minimumBytes&&facts.mtimeMs>since)candidates.push({path,size:facts.size,mode:facts.mode&0o777,atime:facts.atime,mtime:facts.mtime});
  }
 }
 const bySize=new Map<number,typeof candidates>();for(const row of candidates)bySize.set(row.size,[...bySize.get(row.size)??[],row]);
 const digest=(path:string)=>new Promise<string>((resolve,reject)=>{const hash=createHash("sha256"),stream=createReadStream(path);stream.on("data",chunk=>hash.update(chunk));stream.on("error",reject);stream.on("end",()=>resolve(hash.digest("hex")));});
 let files=0,bytes=0;
 for(const group of bySize.values()){
  if(group.length<2)continue;const byContent=new Map<string,typeof candidates>();
  for(const row of group){signal.throwIfAborted();const key=await digest(row.path).catch(error=>{if(missing(error))return undefined;throw error;});if(key!==undefined)byContent.set(key,[...byContent.get(key)??[],row]);}
  for(const[key,rows]of byContent){
   if(rows.length<2)continue;await mkdir(shared,{recursive:true});const original=join(shared,key.slice(0,32)),temporary=original+"."+randomBytes(6).toString("hex");
   if(!existsSync(original)){try{await copyFile(rows[0]!.path,temporary,constants.COPYFILE_FICLONE_FORCE);await rename(temporary,original);}catch{await rm(temporary,{force:true});continue;}}
   for(const row of rows){
    signal.throwIfAborted();const clone=row.path+"."+randomBytes(6).toString("hex");
    try{await copyFile(original,clone,constants.COPYFILE_FICLONE_FORCE);await chmod(clone,row.mode);await utimes(clone,row.atime,row.mtime);await rename(clone,row.path);files++;bytes+=row.size;}catch{await rm(clone,{force:true});}
   }
  }
 }
 if(files>0){await mkdir(shared,{recursive:true});await writeFile(stampPath,String(started));await utimes(stampPath,new Date(started),new Date(started));}
 return Object.freeze({files,bytes});
}

/** 🧺️ Removes the entries of the content pool that entered it at least `maximumAgeMs` ago (their change time, because a cloned file keeps the modification time of its source): pool files are only accelerators, packages keep their own clones, so an evicted entry costs one copy when its content is captured again. Returns the entries and the allocated bytes it found expired; a dry run removes nothing. */
export async function collectBazelPoolV1(pool:string,signal:AbortSignal,options:Readonly<{now:number;maximumAgeMs:number;dryRun?:boolean}>):Promise<Readonly<{entries:number;bytes:number}>>{
 let entries=0,bytes=0;
 for(const folder of await readdir(pool,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;})){
  signal.throwIfAborted();
  if(!folder.isDirectory())continue;
  const directory=join(pool,folder.name);
  for(const name of await readdir(directory).catch(error=>{if(missing(error))return [];throw error;})){
   const path=join(directory,name),facts=await lstat(path).catch(error=>{if(missing(error))return undefined;throw error;});
   if(!facts?.isFile()||options.now-facts.ctimeMs<options.maximumAgeMs)continue;
   entries++;bytes+=allocated(facts);if(!options.dryRun)await rm(path,{force:true});
  }
  if(!options.dryRun)await rmdir(directory).catch(()=>undefined);
 }
 return Object.freeze({entries,bytes});
}

/** 🌡️ The free bytes of the volume below a directory, undefined while nothing of the path exists (an unmounted image counts as no pressure). */
export function bazelFreeBytesV1(directory:string):number|undefined{
 return nativeFreeSpaceSyncV1(directory,{nearest:true})?.availableBytes;
}

const allocated=(facts:{blocks:number;size:number})=>facts.blocks>0?facts.blocks*512:facts.size;
/** 📏️ Measures the bytes a tree occupies (allocated blocks of every regular file, symbolic links are leaves that are never followed); it stops between directories when the signal aborts. */
export async function measureBazelTreeBytesV1(path:string,signal:AbortSignal):Promise<number>{
 let bytes=0;const pending=[path];
 for(let next=pending.pop();next!==undefined;next=pending.pop()){
  signal.throwIfAborted();
  const entries=await readdir(next,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;});
  await Promise.all(entries.map(async entry=>{const child=join(next,entry.name);if(entry.isDirectory())pending.push(child);else{const facts=await lstat(child).catch(error=>{if(missing(error))return undefined;throw error;});if(facts?.isFile())bytes+=allocated(facts);}}));
 }
 return bytes;
}

/** 📋️ Lists what exists on disk below the server and workspace directories: per slot its output base, remembered state and generated packages with their leftover capture folder; every other directory is foreign (the hashed output bases and packages of the time before slots). With `measure` it also reads the bytes of every output base and package, the sizes the pressure rule frees by. */
export async function readBazelInventoryV1(directories:BazelSlotDirectoriesV1,port:BazelSlotPortV1,options:Readonly<{measure?:(path:string)=>Promise<number>;skip?:ReadonlySet<string>}>={}):Promise<BazelInventoryV1>{
 const listing=async(path:string)=>readdir(path,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;}),slots=new Map<string,{outputBase:boolean;packages:{key:string;modifiedAt:number;temporary?:boolean;bytes?:number}[]}>(),foreign:string[]=[],row=(slot:string)=>{const value=slots.get(slot)??{outputBase:false,packages:[]};slots.set(slot,value);return value;};
 for(const entry of await listing(directories.server))if(entry.isDirectory()){if(slotPattern.test(entry.name))row(entry.name).outputBase=true;else foreign.push("server/"+entry.name);}else{const match=entry.name.endsWith(policy.names.stateSuffix)?entry.name.slice(0,-policy.names.stateSuffix.length):undefined;if(match!==undefined&&slotPattern.test(match))row(match);}
 for(const entry of await listing(directories.workspaces))if(entry.isDirectory()){
  if(!slotPattern.test(entry.name)){foreign.push("workspaces/"+entry.name);continue;}
  const value=row(entry.name);for(const child of await listing(join(directories.workspaces,entry.name)))if(child.isDirectory()&&keyPattern.test(child.name)){const path=join(directories.workspaces,entry.name,child.name);value.packages.push({key:child.name,modifiedAt:Math.floor((await stat(path)).mtimeMs),...existsSync(join(path,temporaryName))?{temporary:true}:{}});}
 }
 const sizes=new Map<string,number>();
 if(options.measure){
  const jobs:{id:string;path:string}[]=[];for(const[slot,value]of slots){if(options.skip?.has(slot))continue;if(value.outputBase)jobs.push({id:"base/"+slot,path:join(directories.server,slot)});for(const entry of value.packages)jobs.push({id:slot+"/"+entry.key,path:join(directories.workspaces,slot,entry.key)});}
  let next=0;await Promise.all(Array.from({length:Math.min(policy.pressure.measureConcurrency,jobs.length)},async()=>{for(let job=jobs[next++];job!==undefined;job=jobs[next++])sizes.set(job.id,await options.measure!(job.path).catch(error=>{if(missing(error))return 0;throw error;}));}));
 }
 const rows=[];for(const [slot,value] of [...slots].sort((left,right)=>left[0]<right[0]?-1:1)){const state=await port.read(slot);rows.push({slot,outputBase:value.outputBase,...(sizes.has("base/"+slot)?{outputBaseBytes:sizes.get("base/"+slot)!}:{}),...(state?{state}:{}),packages:value.packages.map(entry=>sizes.has(slot+"/"+entry.key)?{...entry,bytes:sizes.get(slot+"/"+entry.key)!}:entry)});}
 const inventory={slots:rows,foreign:foreign.sort()};if(validateJsonSchemaSubset(schema.definitions.Inventory,inventory,schema).length)throw Error("Closed Bazel session inventory required");return inventory;
}

/** 🧲️ Collects session garbage: for every slot that has something to collect it takes the slot's claim (a slot in use is skipped, except the one the caller already holds), stops its server before removing the output base, removes old packages, leftover capture folders and the remembered state of a retired slot, and removes foreign entries after stopping their servers. When `freeBytes` reports less free space than the soft threshold it measures the trees first and the plan frees the oldest unprotected packages and the longest idle output bases down to the target. Returns what was removed. */
export async function collectBazelSessionsV1(directories:BazelSlotDirectoriesV1,port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots"|"services">,signal:AbortSignal,stop:(outputBase:string)=>Promise<void>,options:Readonly<{dryRun?:boolean;holding?:string;keep?:string;progress?:(line:string)=>void;freeBytes?:()=>number;reclaim?:Readonly<{softAvailableBytes:number;targetAvailableBytes:number}>;language:BazelLanguageV1}>):Promise<BazelGarbageV1>{
 const language=options.language,free=options.freeBytes?.(),reclaim=options.reclaim,pressured=free!==undefined&&reclaim!==undefined&&free<reclaim.softAvailableBytes;
 if(pressured)options.progress?.(bazelSessionMessageV1("pressure",{free:Math.floor(free/1048576),soft:Math.floor(reclaim.softAvailableBytes/1048576),target:Math.floor(reclaim.targetAvailableBytes/1048576)},language));
 const occupied=new Set<string>();
 if(pressured||options.dryRun)for(const row of(await readBazelInventoryV1(directories,port)).slots){if(row.slot===options.holding)continue;const probe=await port.claim(row.slot,signal);if(probe===undefined)occupied.add(row.slot);else probe();}
 const measured=await readBazelInventoryV1(directories,port,pressured?{measure:path=>measureBazelTreeBytesV1(path,signal),skip:occupied}:{}),inventory:BazelInventoryV1=options.holding===undefined||options.keep===undefined?measured:{...measured,slots:measured.slots.map(row=>row.slot===options.holding?{...row,state:nextBazelSlotStateV1(row.slot,row.state,options.keep!,port.now())}:row)},garbage=planBazelSessionGarbageV1(inventory,plan,port.now(),occupied,policy,free===undefined||reclaim===undefined?undefined:{freeBytes:free,softBytes:reclaim.softAvailableBytes,targetBytes:reclaim.targetAvailableBytes});if(options.dryRun)return garbage;
 const removedPackages:BazelGarbageV1["packages"][number][]=[],removedBases:BazelGarbageV1["outputBases"][number][]=[],removedTemporaries:BazelGarbageV1["temporaries"][number][]=[],removedForeign:string[]=[],slots=[...new Set([...garbage.packages.map(row=>row.slot),...garbage.outputBases.map(row=>row.slot),...garbage.temporaries.map(row=>row.slot)])],sized=new Map(inventory.slots.flatMap(row=>[["base/"+row.slot,row.outputBaseBytes??0] as const,...row.packages.map(entry=>[row.slot+"/"+entry.key,entry.bytes??0] as const)]));let freed=0;
 for(const slot of slots){
  const claimed=slot===options.holding?()=>undefined:await port.claim(slot,signal);if(claimed===undefined)continue;
  try{
   for(const row of garbage.outputBases.filter(value=>value.slot===slot)){const base=join(directories.server,slot);await stop(base);await removeBazelTreeV1(base);await rm(join(directories.server,slot+policy.names.stateSuffix),{force:true});await rm(base+".events.json",{force:true});removedBases.push(row);freed+=sized.get("base/"+slot)??0;options.progress?.(`[bazel] removed the output base of slot ${slot} (${row.reason})`);}
   for(const row of garbage.packages.filter(value=>value.slot===slot)){await removeBazelTreeV1(join(directories.workspaces,slot,requireBazelKeyV1(row.key)));removedPackages.push(row);freed+=sized.get(slot+"/"+row.key)??0;if(row.reason==="pressure")options.progress?.(bazelSessionMessageV1("removed-package",{key:row.key,slot,reason:row.reason},language));}
   for(const row of garbage.temporaries.filter(value=>value.slot===slot)){await removeBazelTreeV1(join(directories.workspaces,slot,requireBazelKeyV1(row.key),temporaryName));removedTemporaries.push(row);options.progress?.(bazelSessionMessageV1("removed-temporary",{key:row.key,slot},language));}
  }finally{claimed();}
 }
 if(pressured)for(const row of measured.slots){
  if(!row.outputBase||occupied.has(row.slot)||removedBases.some(value=>value.slot===row.slot))continue;
  const claimed=row.slot===options.holding?()=>undefined:await port.claim(row.slot,signal);if(claimed===undefined)continue;
  try{const merged=await dedupeBazelLaunchersV1(join(directories.server,row.slot),signal);if(merged.files>0){freed+=merged.bytes;options.progress?.(bazelSessionMessageV1("deduped",{slot:row.slot,files:merged.files,megabytes:Math.floor(merged.bytes/1048576)},language));}}finally{claimed();}
 }
 for(const name of garbage.foreign){
  signal.throwIfAborted();const [area,entry]=[name.slice(0,name.indexOf("/")),name.slice(name.indexOf("/")+1)],path=join(area==="server"?directories.server:directories.workspaces,entry);
  if(area==="server")await stop(path).catch(()=>undefined);await removeBazelTreeV1(path);if(area==="server")await rm(path+".events.json",{force:true});removedForeign.push(name);options.progress?.(`[bazel] removed ${name}`);
 }
 if(pressured&&directories.pool!==undefined&&options.freeBytes!()<reclaim.targetAvailableBytes){
  const pooled=await collectBazelPoolV1(directories.pool,signal,{now:port.now(),maximumAgeMs:policy.pressure.packageGraceMinutes*60000});freed+=pooled.bytes;
  options.progress?.(bazelSessionMessageV1("pool-removed",{entries:pooled.entries,megabytes:Math.floor(pooled.bytes/1048576)},language));
 }
 if(pressured)options.progress?.(bazelSessionMessageV1("freed",{freed:Math.floor(freed/1048576)},language));
 return Object.freeze({packages:removedPackages,outputBases:removedBases,temporaries:removedTemporaries,foreign:removedForeign});
}
