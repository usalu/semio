import {availableParallelism,setPriority} from "node:os";
import {spawn,spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {appendFileSync,chmodSync,closeSync,existsSync,mkdirSync,openSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {createHash,randomBytes} from "node:crypto";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {sealModulesV1,type SealedModulesV1} from "../🏛️seal/🟦️.ts";
import {jobserverProbeShellV1,shellQuoteV1} from "../🎰️jobserver/🟦️.ts";
import {collectGarbageSingleFlightV1,gcOutOfSpaceV1,gcPressureV1,measureVolumeV1,type GcPressureV1} from "../🧹️gc/🟦️.ts";
import {releaseBallastV1} from "../🧹️gc/🛟️ballast/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {fleetCacheRootV1,readLocalFleetCacheFactsV1,resolveRemoteCacheV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
/** 📍️ The wrapper script Cargo runs in place of rustc. */
export const UNIT_CACHE_SCRIPT_V1=fileURLToPath(new URL("../📜️script.ts",import.meta.url));
/** 🏁️ The module a native compiler wrapper is built from. */
export const UNIT_CACHE_ENTRY_V1=fileURLToPath(new URL("../🏁️entry.ts",import.meta.url));

/** 🎯️ The package names a Cargo command line selects with `-p` or `--package`; their units always compile incrementally in the lane and are never published, while every other unit is served from the shared cache first (a workspace unit that misses still compiles with its incremental session in the lane). */
export function focusFromCargoArgumentsV1(args:readonly string[]):string[]{
 const names:string[]=[],end=args.indexOf("--"),head=end<0?args:args.slice(0,end);
 for(let index=0;index<head.length;index++){
  const argument=head[index]!;
  if(argument==="-p"||argument==="--package"){const value=head[++index];if(value)names.push(value.replace(/@.*$/u,""));}
  else if(argument.startsWith("--package="))names.push(argument.slice("--package=".length).replace(/@.*$/u,""));
  else if(/^-p.+/u.test(argument))names.push(argument.slice(2).replace(/@.*$/u,""));
 }
 return[...new Set(names)].sort();
}

/** 🏛️ Seals the module set the compiler wrapper runs (script and native entry, with every file they import) into the cache root and answers the sealed copies: every process of one build session runs this snapshot, so a concurrent edit of the live tooling reaches the next session only, and a tree that is mid-edit is retried and never sealed (see {@link sealModulesV1}). */
export function sealedUnitCacheV1(cacheRoot:string,runtime:string=process.execPath):SealedModulesV1{return sealModulesV1(cacheRoot,[UNIT_CACHE_SCRIPT_V1,UNIT_CACHE_ENTRY_V1],{checked:[UNIT_CACHE_SCRIPT_V1],runtime});}

/** 📜️ The executable compiler wrapper of a process that names it in its environment (`SEMIO_UNIT_CACHE_WRAPPER`, the one `RUSTC_WRAPPER` convention of {@link unitCacheEnvironmentV1}), or `undefined` after a bilingual notice when no wrapper could be made (the unit cache is then off for that process, never half-working). `RUSTC` is never the wrapper. */
export function unitCacheWrapperV1(cacheRoot:string,report:(line:string)=>void=line=>{process.stderr.write(line+"\n");}):string|undefined{
 try{return ensureUnitCacheShimV1(cacheRoot);}catch(error){report(unitCacheWrapperRefusalV1(String((error as Error)?.message??error).split("\n")[0]!));return undefined;}
}

/** 🪞️ Where the executable stand-in for the compiler wrapper lives: Cargo runs it as `<wrapper> <rustc> <arguments>`. Its name carries the digest of the code it runs (`rustc-wrapper-<digest>`; on Windows a native executable `….exe`, see {@link ensureUnitCacheShimV1}, because Cargo passes arguments with quotes (`--cfg feature="std"`) and Rust refuses to hand those to a batch file), so sessions of different tooling never share a stand-in. */
export function unitCacheShimPathV1(cacheRoot:string,platform:NodeJS.Platform,digest:string):string{return join(cacheRoot,"bin",`rustc-wrapper-${digest}${platform==="win32"?".exe":""}`);}

/** 📜️ The text of the POSIX stand-in: it names the runtime and the wrapper script by absolute path, so no `PATH` entry and no file mode of the tracked script matters. Before the runtime starts (and opens descriptors of its own at the lowest free numbers) it runs {@link jobserverProbeShellV1}, which exports the jobserver pair `CARGO_MAKEFLAGS` names only when both descriptors are open: a build script does not inherit them unless a stand-in hands them on, and the wrapper must never pass a descriptor that is not Cargo's. */
export function unitCacheShimSourceV1(runtime:string,script:string):string{
 return["#!/bin/sh",...jobserverProbeShellV1(),`exec ${shellQuoteV1(runtime)} ${shellQuoteV1(script)} "$@"`,""].join("\n");
}

/** 🏗️ How a native wrapper is made: `bundle` answers the code the executable would embed (its digest names the executable), `compile` writes the executable. Injectable so a test can count and fail the steps. */
export type UnitCacheNativeBuildV1=Readonly<{bundle:(runtime:string,entry:string)=>string;compile:(runtime:string,entry:string,outfile:string)=>void}>;

/** ⚙️ The real build: the runtime's own bundler and `--compile`, so the executable embeds the runtime and needs no `PATH` entry, no batch file and no interpreter lookup at run time. */
export const UNIT_CACHE_NATIVE_BUILD_V1:UnitCacheNativeBuildV1=Object.freeze({
 bundle(runtime,entry){
  const run=spawnSync(runtime,["build","--target=bun",entry],{encoding:"utf8",maxBuffer:1<<28,windowsHide:true});
  if(run.status!==0||!run.stdout)throw Error(String(run.error?.message??run.stderr??"bundler failed").trim()||"bundler failed");
  return run.stdout;
 },
 compile(runtime,entry,outfile){
  const run=spawnSync(runtime,["build","--compile",entry,"--outfile",outfile],{encoding:"utf8",maxBuffer:1<<26,windowsHide:true});
  if(run.status!==0)throw Error(String(run.error?.message??run.stderr??"compiler failed").trim()||"compiler failed");
 },
});

/** 🚫️ Why no native wrapper could be made, in every language: the unit cache is switched off for that Cargo process, never half-working. */
export function unitCacheWrapperRefusalV1(reason:string):string{return (["en","de"] as const).map(language=>laneMessageV1("wrapperUnavailable",{reason},language)).join("\n");}

/** 🪟️ Publishes a digest-addressed executable from its own stage and retains every peer-owned generation. */
function ensureNativeWrapperV1(cacheRoot:string,runtime:string,entry:string,platform:NodeJS.Platform,build:UnitCacheNativeBuildV1):string{
 const digest=createHash("sha256").update(build.bundle(runtime,entry)).digest("hex").slice(0,16),path=unitCacheShimPathV1(cacheRoot,platform,digest);
 if(existsSync(path))return path;
 mkdirSync(dirname(path),{recursive:true});
 const temporary=path+"."+randomBytes(4).toString("hex")+".tmp.exe";
 try{build.compile(runtime,entry,temporary);chmodSync(temporary,0o755);renameSync(temporary,path);}
 catch(error){rmSync(temporary,{force:true});if(!existsSync(path))throw error;}
 return path;
}

/** 🛠️ Writes the stand-in once (atomically, executable) and returns its path. Unless the caller names `script` and `entry` itself, both are the sealed copies of this session ({@link sealedUnitCacheV1}), so neither the stand-in nor the native executable ever runs or embeds a live file that a peer is editing. POSIX: a `/bin/sh` script named by the snapshot digest. Windows: a native executable built from the sealed `🏁️entry.ts`; a build or seal that fails throws, and {@link unitCacheEnvironmentV1} turns that into a bilingual refusal. Build scripts that run `$RUSTC` reach the real compiler; only Cargo's own compiler calls go through the wrapper. */
export function ensureUnitCacheShimV1(cacheRoot:string,runtime:string=process.execPath,script?:string,platform:NodeJS.Platform=process.platform,build:UnitCacheNativeBuildV1=UNIT_CACHE_NATIVE_BUILD_V1,entry?:string):string{
 const sealed=(platform==="win32"?entry:script)===undefined?sealedUnitCacheV1(cacheRoot,runtime):undefined;
 script??=sealed?.entries[0];entry??=sealed?.entries[1];
 for(const digest of sealed?.removed??[])rmSync(unitCacheShimPathV1(cacheRoot,platform,digest),{force:true});
 if(platform==="win32")return ensureNativeWrapperV1(cacheRoot,runtime,entry!,platform,build);
 const path=unitCacheShimPathV1(cacheRoot,platform,sealed?.digest??createHash("sha256").update(runtime+"\0"+script!).digest("hex").slice(0,UNIT_CACHE_POLICY_V1.seal.digestCharacters)),text=unitCacheShimSourceV1(runtime,script!);
 let current:string|undefined;try{current=readFileSync(path,"utf8");}catch{}
 if(current!==text){mkdirSync(dirname(path),{recursive:true});const temporary=path+"."+randomBytes(4).toString("hex");writeFileSync(temporary,text);chmodSync(temporary,0o755);renameSync(temporary,path);}
 return path;
}

/** 🛰️ The fleet cache as the rustc wrapper reads it (`SEMIO_REMOTE_CACHE_*`), resolved once per Cargo process by the one resolver of the remote cache (the environment first, field by field, then the fleet facts of this machine's local copy; the copy is read, never refreshed, so a launch waits for no network). Empty when the tier is switched off (`SEMIO_UNIT_CACHE_REMOTE=off`) or nothing names a cache. The token travels in the environment like the Nx cache token does: every process of the Cargo tree can read it. */
export function fleetRemoteEnvironmentV1(environment:Readonly<Record<string,string|undefined>>=process.env,workspaceRoot?:string):Record<string,string>{
 try{
  if(environment.SEMIO_UNIT_CACHE_REMOTE==="off")return{};
  const root=workspaceRoot??fleetCacheRootV1(environment),copy=root===undefined?undefined:readLocalFleetCacheFactsV1(root);
  const resolved=resolveRemoteCacheV1(environment,copy===undefined||root===undefined?{}:{root,copy});
  if(resolved===undefined)return{};
  const contract=resolved.contract;
  return{SEMIO_REMOTE_CACHE_ENDPOINT:contract.endpoint,SEMIO_REMOTE_CACHE_INSTANCE:contract.instance,SEMIO_REMOTE_CACHE_ACCESS:contract.access,SEMIO_REMOTE_CACHE_UPLOAD:String(contract.upload),SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:String(contract.timeoutSeconds),SEMIO_REMOTE_CACHE_RETRIES:String(contract.retries),...(resolved.token===undefined?{}:{SEMIO_REMOTE_CACHE_TOKEN:resolved.token}),...(resolved.generation===null?{}:{SEMIO_REMOTE_CACHE_GENERATION:String(resolved.generation)}),...(resolved.environment.SEMIO_REMOTE_CACHE_SIGNED==="1"&&root!==undefined?{SEMIO_REMOTE_CACHE_SIGNED:"1",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:root}:{})};
 }catch{return{};}
}

/** 🧩️ The one assembly of the environment block that routes every compiler call of a Cargo process through the unit cache, whoever launches it (`repo cargo` through {@link unitCacheEnvironmentV1}, the native-build driver for its passes): the wrapper, the cache and lane, the compiler chain (`inner`), the slots and the fleet tier ({@link fleetRemoteEnvironmentV1}, empty when `SEMIO_UNIT_CACHE_REMOTE=off`). Nothing else builds these variables, so no entry point can leave the fleet out. */
export function unitCacheChainEnvironmentV1(input:Readonly<{wrapper:string;cacheRoot:string;lane:string;laneRoot:string;inner:readonly string[];toolchain?:string;focus:readonly string[];slots:number;lightSlots?:number;workspaceRoot?:string;environment?:Readonly<Record<string,string|undefined>>;features?:Readonly<{buildScripts?:boolean;brokerReplay?:boolean}>}>):Record<string,string>{
 const workspace=input.workspaceRoot??fleetCacheRootV1(input.environment??process.env);
 return{
  RUSTC_WRAPPER:input.wrapper,
  SEMIO_UNIT_CACHE_ROOT:input.cacheRoot,
  SEMIO_UNIT_CACHE_LANE:input.lane,
  SEMIO_UNIT_CACHE_LANE_ROOT:input.laneRoot,
  SEMIO_UNIT_CACHE_INNER:JSON.stringify(input.inner),
  ...(input.toolchain===undefined?{}:{SEMIO_UNIT_CACHE_TOOLCHAIN:input.toolchain}),
  SEMIO_UNIT_CACHE_SLOTS:String(input.slots),
  SEMIO_UNIT_CACHE_FOCUS:input.focus.join(","),
  ...(input.lightSlots?{SEMIO_UNIT_CACHE_LIGHT_SLOTS:String(input.lightSlots)}:{}),
  ...(input.features?.buildScripts?{SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"}:{}),
  ...(input.features?.brokerReplay?{SEMIO_UNIT_CACHE_BROKER_REPLAY:"on"}:{}),
  ...(workspace?{SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace}:{}),
  ...fleetRemoteEnvironmentV1(input.environment,workspace),
 };
}

/** 🧩️ The environment block of a launcher that makes its own wrapper: it ensures the sealed stand-in, then {@link unitCacheChainEnvironmentV1} assembles the block. Cargo runs the stand-in as `<wrapper> <rustc> <arguments>`, which serves a unit from the cache or falls back to the memory-admitting scheduler for every miss. `RUSTC` stays the real compiler, so build scripts that exec it work whatever the file mode of the script or `PATH`. A launcher adds this after its storage admission and never before; without a `scheduler` a miss runs the compiler directly. */
export function unitCacheEnvironmentV1(input:Readonly<{cacheRoot:string;lane:string;laneRoot:string;scheduler?:string;compiler:string;toolchain:string;focus:readonly string[];slots?:number;lightSlots?:number;workspaceRoot?:string;environment?:Readonly<Record<string,string|undefined>>;runtime?:string;platform?:NodeJS.Platform;build?:UnitCacheNativeBuildV1;features?:Readonly<{buildScripts?:boolean;brokerReplay?:boolean}>}>):Record<string,string>{
 let wrapper:string;
 try{wrapper=ensureUnitCacheShimV1(input.cacheRoot,input.runtime,undefined,input.platform,input.build);}
 catch(error){return{RUSTC_WRAPPER:"",SEMIO_UNIT_CACHE:"off",SEMIO_UNIT_CACHE_REFUSAL:unitCacheWrapperRefusalV1(String((error as Error)?.message??error).split("\n")[0]!)};}
 return unitCacheChainEnvironmentV1({wrapper,cacheRoot:input.cacheRoot,lane:input.lane,laneRoot:input.laneRoot,inner:input.scheduler===undefined?[input.compiler]:[input.scheduler,input.compiler],toolchain:input.toolchain,focus:input.focus,slots:input.slots??(Number((input.environment??process.env).SEMIO_UNIT_CACHE_SLOTS)||availableParallelism()),...(input.lightSlots?{lightSlots:input.lightSlots}:{}),...(input.workspaceRoot===undefined?{}:{workspaceRoot:input.workspaceRoot}),...(input.environment===undefined?{}:{environment:input.environment}),...(input.features===undefined?{}:{features:input.features})});
}

/** 🧮️ The unit cache may fill the image only up to what the compiler workspace leaves free, so a cold writer is always admitted. */
export const unitCacheCapacityBytesV1=(policy:Readonly<{capacityBytes:number;maximumWorkspaceBytes:number}>):number=>Math.max(0,policy.capacityBytes-policy.maximumWorkspaceBytes);

/** 🗺️ Where the cache root lives beside the compiler store's own directories. */
export function unitCacheRootV1(storeRoot:string):string{return join(storeRoot,"u");}

/** 📏️ The share of the image each part of the cache may use. */
export const UNIT_CACHE_SHARES_V1=Object.freeze({lanes:policy.gc.laneShare,incremental:policy.gc.incrementalShare});

/** 💬️ One operator message with its placeholders filled. */
export function laneMessageV1(kind:keyof typeof schema.definitions.Messages.const,parameters:Readonly<Record<string,string|number>>,language:"en"|"de"):string{
 return schema.definitions.Messages.const[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(parameters[name]??""));
}

/** 🔁️ How often a command that ran out of space is repeated after the cleanup. */
export const UNIT_CACHE_OUT_OF_SPACE_RETRIES_V1=policy.gc.outOfSpaceRetries;
const GIB=1024**3,collectorEnvironmentKeys=Object.freeze(["PATH","HOME","USERPROFILE","TMPDIR","TEMP","TMP","SystemRoot","ComSpec","LANG","LC_ALL","LC_MESSAGES","SEMIO_LANGUAGE"]);
const gib=(bytes:number):string=>(bytes/GIB).toFixed(1);

/** 🚀️ Starts the background collector: detached, one priority step below its caller, its output in `gc.log` of the cache root; a start that fails is returned, never thrown (on a full image the ballast is given back and the start is tried once more, so the collector that has to free space can still write its stamp and log). The throttle stamp is written first, so a storm of writers starts one process, not many. */
export function startUnitCacheCollectorV1(options:Readonly<{root:string;capacityBytes:number;environment:Readonly<Record<string,string|undefined>>;runtime?:string;script?:string;watch?:boolean}>,fullImage:boolean=false):Readonly<{state:"started"}|{state:"failed";reason:string}>{
 try{
  const stamp=join(options.root,"gc.stamp"),log=join(options.root,"gc.log"),env:Record<string,string>={SEMIO_CACHE_BACKGROUND:"1"};
  for(const key of collectorEnvironmentKeys){const value=options.environment[key];if(value!==undefined)env[key]=value;}
  mkdirSync(options.root,{recursive:true});writeFileSync(stamp,String(Date.now()));
  try{if(statSync(log).size>UNIT_CACHE_POLICY_V1.gc.eventLogMaximumBytes)renameSync(log,log+".1");}catch{}
  const fd=openSync(log,"a");
  try{
   const child=spawn(options.runtime??process.execPath,[options.script??sealedUnitCacheV1(options.root,options.runtime).entries[0]!,"--semio-unit-cache","gc","--single-flight",...options.watch?["--watch"]:[],"--root",options.root,"--capacity",String(options.capacityBytes)],{env,detached:true,stdio:["ignore",fd,fd],windowsHide:true});
   child.on("error",error=>{try{appendFileSync(log,"launch failed: "+error.message+"\n");}catch{}});child.unref();
   if(child.pid!==undefined)try{setPriority(child.pid,10);}catch(error){appendFileSync(log,"priority not lowered: "+String((error as Error).message)+"\n");}
  }finally{closeSync(fd);}
  return{state:"started"};
 }catch(error){if(!fullImage&&(error as NodeJS.ErrnoException).code==="ENOSPC"&&releaseBallastV1(options.root))return startUnitCacheCollectorV1(options,true);return{state:"failed",reason:String((error as Error).message)};}
}

/** 🌊️ Looks at the image once and does what its pressure asks: nothing, the quiet periodic maintenance, the background collector, or (below what one compile must find free) a collection right here before the compile starts. Never throws: a cache that cannot be relieved only costs the next compile its headroom check. */
export async function relieveUnitCachePressureV1(options:Readonly<{root:string;capacityBytes:number;emergencyBytes:number;environment:Readonly<Record<string,string|undefined>>;language:"en"|"de";report:(line:string)=>void;nowMs?:number;measure?:typeof measureVolumeV1;start?:typeof startUnitCacheCollectorV1}>):Promise<GcPressureV1>{
 try{
  const nowMs=options.nowMs??Date.now(),volume=(options.measure??measureVolumeV1)(options.root,options.capacityBytes),start=options.start??startUnitCacheCollectorV1;let last:number|null=null;try{last=statSync(join(options.root,"gc.stamp")).mtimeMs;}catch{}
  const decision=gcPressureV1({...volume,emergencyBytes:options.emergencyBytes,nowMs,lastLaunchMs:last});
  if(decision==="maintain"){const started=start(options);if(started.state==="failed")options.report(laneMessageV1("collectorFailed",{reason:started.reason},options.language));}
  else if(decision==="watch"){const started=start({...options,watch:true});if(started.state==="failed")options.report(laneMessageV1("collectorFailed",{reason:started.reason},options.language));}
  else if(decision==="launch"){
   const started=start({...options,watch:true});
   options.report(started.state==="started"?laneMessageV1("collector",{usedPercent:Math.round(100*(volume.volumeBytes-volume.freeBytes)/volume.volumeBytes),highPercent:Math.round(100*policy.gc.highWaterShare)},options.language):laneMessageV1("collectorFailed",{reason:started.reason},options.language));
  }else if(decision==="collect-now"){
   const result=await collectGarbageSingleFlightV1(options.root,options.capacityBytes,{nowMs});if(!result.running&&result.applied.lanes+result.applied.incremental+result.applied.sessions+result.applied.units>0)options.report(laneMessageV1("collected",result.applied,options.language));
  }
  return decision;
 }catch{return "enough";}
}

/** 💧️ The pressure relief every compile start runs for an issued compiler store: the cache root is the store's `u` directory, the image is the store's own volume, below a floor of free space the single-flight background collector starts (and below `emergencyBytes` the compile collects first), so no launch needs its own copy of the rule. Never throws. */
export function relieveUnitStorageV1(store:string,options:Readonly<{environment:Readonly<Record<string,string|undefined>>;language:"en"|"de";report:(line:string)=>void;emergencyBytes?:number}>):Promise<GcPressureV1>{
 const root=unitCacheRootV1(store);return relieveUnitCachePressureV1({root,capacityBytes:measureVolumeV1(root,1).volumeBytes,emergencyBytes:options.emergencyBytes??policy.gc.emergencyFreeBytes,environment:options.environment,language:options.language,report:options.report});
}

/** ⏱️ Runs a long compile and, every `recheckMilliseconds`, looks at the image again with `recheck` (never awaited, never failing the compile): a Cargo process can write gigabytes after the launch decision was taken, so the watching collector is (re)started while the compile runs, not only before it. */
export async function withUnitPressureRecheckV1<T>(recheck:()=>Promise<unknown>,body:()=>Promise<T>,intervalMilliseconds:number=policy.gc.recheckMilliseconds,every:UnitRecheckTimerV1=realRecheckTimerV1):Promise<T>{
 const stop=every(()=>{void recheck().catch(()=>undefined);},intervalMilliseconds);
 try{return await body();}finally{stop();}
}

/** ⏲️ A repeating timer: starts calling `tick` every `milliseconds` and answers the function that stops it. Injectable so a test drives the ticks itself. */
export type UnitRecheckTimerV1=(tick:()=>void,milliseconds:number)=>()=>void;
const realRecheckTimerV1:UnitRecheckTimerV1=(tick,milliseconds)=>{const timer=setInterval(tick,milliseconds);timer.unref?.();return()=>clearInterval(timer);};

/** 🪐️ Runs a build once more after it failed with the operating system's out-of-space error: the unit cache of the store is collected right now (single-flight) and the failure that came back from the first run is only rethrown when the second run fails too or the error was something else. */
export async function retryAfterOutOfSpaceV1<T>(options:Readonly<{store:string;language:"en"|"de";report:(line:string)=>void}>,run:()=>Promise<T>):Promise<T>{
 try{return await run();}
 catch(error){
  const text=[(error as Error)?.message,(error as {cause?:{message?:string}})?.cause?.message,String(error)].filter(Boolean).join("\n");
  if(!gcOutOfSpaceV1(text))throw error;
  const root=unitCacheRootV1(options.store);await collectAfterOutOfSpaceV1({root,capacityBytes:measureVolumeV1(root,1).volumeBytes,language:options.language,report:options.report});
  return await run();
 }
}

/** 🛑️ After a compile ran out of space: collects once, right now, and says so in the selected language. The caller retries its command once and calls `reportUnitCacheStillFullV1` if that fails the same way. */
export async function collectAfterOutOfSpaceV1(options:Readonly<{root:string;capacityBytes:number;language:"en"|"de";report:(line:string)=>void;nowMs?:number}>):Promise<Readonly<{lanes:number;incremental:number;sessions:number;units:number}>>{
 const volume=measureVolumeV1(options.root,options.capacityBytes);let applied={lanes:0,incremental:0,sessions:0,units:0};
 try{const result=await collectGarbageSingleFlightV1(options.root,options.capacityBytes,{nowMs:options.nowMs});if(!result.running)applied=result.applied;}catch{}
 options.report(laneMessageV1("outOfSpace",{freeGib:gib(volume.freeBytes),volumeGib:gib(volume.volumeBytes),...applied},options.language));
 return applied;
}

/** 🔄️ Runs a command; while it reports a full image and retries remain, collects and runs it again. The command that is returned is the last one run; `stillFull` is called when the last retry failed the same way. */
export async function retryWhenFullV1<E extends Readonly<{full:boolean}>>(run:()=>Promise<E>,ports:Readonly<{collect:()=>Promise<unknown>;stillFull:()=>void}>,retries:number=UNIT_CACHE_OUT_OF_SPACE_RETRIES_V1):Promise<E>{
 let execution=await run();
 for(let attempt=0;execution.full&&attempt<retries;attempt++){await ports.collect();execution=await run();if(execution.full)ports.stillFull();}
 return execution;
}

/** 🧾️ The retry failed the same way: the space is held by running lanes, not by anything collectable. */
export function reportUnitCacheStillFullV1(options:Readonly<{root:string;capacityBytes:number;language:"en"|"de";report:(line:string)=>void}>):void{
 const volume=measureVolumeV1(options.root,options.capacityBytes);options.report(laneMessageV1("stillFull",{freeGib:gib(volume.freeBytes),volumeGib:gib(volume.volumeBytes)},options.language));
}
