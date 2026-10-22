import {availableParallelism,setPriority} from "node:os";
import {spawn} from "node:child_process";
import {fileURLToPath} from "node:url";
import {appendFileSync,chmodSync,closeSync,mkdirSync,openSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {collectGarbageSingleFlightV1,gcPressureV1,measureVolumeV1,type GcPressureV1} from "../🧹️gc/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=UNIT_CACHE_POLICY_V1;
/** 📍️ The wrapper script Cargo runs in place of rustc. */
export const UNIT_CACHE_SCRIPT_V1=fileURLToPath(new URL("../📜️script.ts",import.meta.url));

/** 🎯️ The package names a Cargo command line selects with `-p` or `--package`; their units compile incrementally in the lane instead of through the shared cache. */
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

/** 🪞️ Where the executable stand-in for the compiler wrapper lives: Cargo runs it as `<wrapper> <rustc> <arguments>`. */
export function unitCacheShimPathV1(cacheRoot:string,platform:NodeJS.Platform=process.platform):string{return join(cacheRoot,"bin",platform==="win32"?"rustc-wrapper.cmd":"rustc-wrapper");}

/** 📜️ The text of the stand-in: it names the runtime and the wrapper script by absolute path, so no `PATH` entry and no file mode of the tracked script matters. */
export function unitCacheShimSourceV1(runtime:string,script:string,platform:NodeJS.Platform=process.platform):string{
 if(platform==="win32")return `@"${runtime}" "${script}" %*\r\n`;
 const quote=(text:string)=>"'"+text.replaceAll("'","'\\''")+"'";
 return `#!/bin/sh\nexec ${quote(runtime)} ${quote(script)} "$@"\n`;
}

/** 🛠️ Writes the stand-in once (atomically, executable) and returns its path. Build scripts that run `$RUSTC` reach the real compiler; only Cargo's own compiler calls go through the wrapper. */
export function ensureUnitCacheShimV1(cacheRoot:string,runtime:string=process.execPath,script:string=UNIT_CACHE_SCRIPT_V1,platform:NodeJS.Platform=process.platform):string{
 const path=unitCacheShimPathV1(cacheRoot,platform),text=unitCacheShimSourceV1(runtime,script,platform);
 let current:string|undefined;try{current=readFileSync(path,"utf8");}catch{}
 if(current!==text){mkdirSync(dirname(path),{recursive:true});const temporary=path+"."+randomBytes(4).toString("hex");writeFileSync(temporary,text);chmodSync(temporary,0o755);renameSync(temporary,path);}
 return path;
}

/** 🧩️ The environment block that routes every compiler call of one Cargo process through the unit cache: Cargo runs the stand-in as `<wrapper> <rustc> <arguments>`, the wrapper script serves a unit from the cache or falls back to the memory-admitting scheduler for every miss. `RUSTC` stays the real compiler, so build scripts that exec it work whatever the file mode of the script or `PATH`. A launcher adds this after its storage admission and never before. */
export function unitCacheEnvironmentV1(input:Readonly<{cacheRoot:string;lane:string;laneRoot:string;scheduler:string;compiler:string;toolchain:string;focus:readonly string[];slots?:number;runtime?:string;features?:Readonly<{buildScripts?:boolean;brokerReplay?:boolean}>}>):Record<string,string>{
 return{
  RUSTC_WRAPPER:ensureUnitCacheShimV1(input.cacheRoot,input.runtime),
  SEMIO_UNIT_CACHE_ROOT:input.cacheRoot,
  SEMIO_UNIT_CACHE_LANE:input.lane,
  SEMIO_UNIT_CACHE_LANE_ROOT:input.laneRoot,
  SEMIO_UNIT_CACHE_INNER:JSON.stringify([input.scheduler,input.compiler]),
  SEMIO_UNIT_CACHE_TOOLCHAIN:input.toolchain,
  SEMIO_UNIT_CACHE_SLOTS:String(input.slots??availableParallelism()),
  SEMIO_UNIT_CACHE_FOCUS:input.focus.join(","),
  ...(input.features?.buildScripts?{SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"}:{}),
  ...(input.features?.brokerReplay?{SEMIO_UNIT_CACHE_BROKER_REPLAY:"on"}:{}),
 };
}

/** 🗺️ Where the cache root lives beside the compiler store's own directories. */
export function unitCacheRootV1(storeRoot:string):string{return join(storeRoot,"u");}

/** 📏️ The share of the image each part of the cache may use. */
export const UNIT_CACHE_SHARES_V1=Object.freeze({objects:policy.gc.casShare,lanes:policy.gc.laneShare,incremental:policy.gc.incrementalShare});

/** 💬️ One operator message with its placeholders filled. */
export function laneMessageV1(kind:keyof typeof schema.definitions.Messages.const,parameters:Readonly<Record<string,string|number>>,language:"en"|"de"):string{
 return schema.definitions.Messages.const[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(parameters[name]??""));
}

/** 🔁️ How often a command that ran out of space is repeated after the cleanup. */
export const UNIT_CACHE_OUT_OF_SPACE_RETRIES_V1=policy.gc.outOfSpaceRetries;
const GIB=1024**3,collectorEnvironmentKeys=Object.freeze(["PATH","HOME","USERPROFILE","TMPDIR","TEMP","TMP","SystemRoot","ComSpec","LANG","LC_ALL","LC_MESSAGES","SEMIO_LANGUAGE"]);
const gib=(bytes:number):string=>(bytes/GIB).toFixed(1);

/** 🚀️ Starts the background collector: detached, one priority step below its caller, its output in `gc.log` of the cache root; a start that fails is returned, never thrown. The throttle stamp is written first, so a storm of writers starts one process, not many. */
export function startUnitCacheCollectorV1(options:Readonly<{root:string;capacityBytes:number;environment:Readonly<Record<string,string|undefined>>;runtime?:string;script?:string}>):Readonly<{state:"started"}|{state:"failed";reason:string}>{
 try{
  const stamp=join(options.root,"gc.stamp"),log=join(options.root,"gc.log"),env:Record<string,string>={SEMIO_CACHE_BACKGROUND:"1"};
  for(const key of collectorEnvironmentKeys){const value=options.environment[key];if(value!==undefined)env[key]=value;}
  mkdirSync(options.root,{recursive:true});writeFileSync(stamp,String(Date.now()));
  const fd=openSync(log,"w");
  try{
   const child=spawn(options.runtime??process.execPath,[options.script??UNIT_CACHE_SCRIPT_V1,"--semio-unit-cache","gc","--single-flight","--root",options.root,"--capacity",String(options.capacityBytes)],{env,detached:true,stdio:["ignore",fd,fd],windowsHide:true});
   child.on("error",error=>{try{appendFileSync(log,"launch failed: "+error.message+"\n");}catch{}});child.unref();
   if(child.pid!==undefined)try{setPriority(child.pid,10);}catch(error){appendFileSync(log,"priority not lowered: "+String((error as Error).message)+"\n");}
  }finally{closeSync(fd);}
  return{state:"started"};
 }catch(error){return{state:"failed",reason:String((error as Error).message)};}
}

/** 🌊️ Looks at the image once and does what its pressure asks: nothing, the quiet periodic maintenance, the background collector, or (below what one compile must find free) a collection right here before the compile starts. Never throws: a cache that cannot be relieved only costs the next compile its headroom check. */
export async function relieveUnitCachePressureV1(options:Readonly<{root:string;capacityBytes:number;emergencyBytes:number;environment:Readonly<Record<string,string|undefined>>;language:"en"|"de";report:(line:string)=>void;nowMs?:number;measure?:typeof measureVolumeV1;start?:typeof startUnitCacheCollectorV1}>):Promise<GcPressureV1>{
 try{
  const nowMs=options.nowMs??Date.now(),volume=(options.measure??measureVolumeV1)(options.root,options.capacityBytes),start=options.start??startUnitCacheCollectorV1;let last:number|null=null;try{last=statSync(join(options.root,"gc.stamp")).mtimeMs;}catch{}
  const decision=gcPressureV1({...volume,emergencyBytes:options.emergencyBytes,nowMs,lastLaunchMs:last});
  if(decision==="maintain"){const started=start(options);if(started.state==="failed")options.report(laneMessageV1("collectorFailed",{reason:started.reason},options.language));}
  else if(decision==="launch"){
   const started=start(options);
   options.report(started.state==="started"?laneMessageV1("collector",{usedPercent:Math.round(100*(volume.volumeBytes-volume.freeBytes)/volume.volumeBytes),highPercent:Math.round(100*policy.gc.highWaterShare)},options.language):laneMessageV1("collectorFailed",{reason:started.reason},options.language));
  }else if(decision==="collect-now"){
   const result=await collectGarbageSingleFlightV1(options.root,options.capacityBytes,{nowMs});if(!result.running&&result.applied.lanes+result.applied.incremental+result.applied.objects>0)options.report(laneMessageV1("collected",result.applied,options.language));
  }
  return decision;
 }catch{return "enough";}
}

/** 🛑️ After a compile ran out of space: collects once, right now, and says so in the selected language. The caller retries its command once and calls `reportUnitCacheStillFullV1` if that fails the same way. */
export async function collectAfterOutOfSpaceV1(options:Readonly<{root:string;capacityBytes:number;language:"en"|"de";report:(line:string)=>void;nowMs?:number}>):Promise<Readonly<{lanes:number;incremental:number;objects:number}>>{
 const volume=measureVolumeV1(options.root,options.capacityBytes);let applied={lanes:0,incremental:0,objects:0};
 try{const result=await collectGarbageSingleFlightV1(options.root,options.capacityBytes,{nowMs:options.nowMs});if(!result.running)applied=result.applied;}catch{}
 options.report(laneMessageV1("outOfSpace",{freeGib:gib(volume.freeBytes),volumeGib:gib(volume.volumeBytes),...applied},options.language));
 return applied;
}

/** 🔁️ Runs a command; while it reports a full image and retries remain, collects and runs it again. The command that is returned is the last one run; `stillFull` is called when the last retry failed the same way. */
export async function retryWhenFullV1<E extends Readonly<{full:boolean}>>(run:()=>Promise<E>,ports:Readonly<{collect:()=>Promise<unknown>;stillFull:()=>void}>,retries:number=UNIT_CACHE_OUT_OF_SPACE_RETRIES_V1):Promise<E>{
 let execution=await run();
 for(let attempt=0;execution.full&&attempt<retries;attempt++){await ports.collect();execution=await run();if(execution.full)ports.stillFull();}
 return execution;
}

/** 🛑️ The retry failed the same way: the space is held by running lanes, not by anything collectable. */
export function reportUnitCacheStillFullV1(options:Readonly<{root:string;capacityBytes:number;language:"en"|"de";report:(line:string)=>void}>):void{
 const volume=measureVolumeV1(options.root,options.capacityBytes);options.report(laneMessageV1("stillFull",{freeGib:gib(volume.freeBytes),volumeGib:gib(volume.volumeBytes)},options.language));
}
