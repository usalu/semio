import {closeSync,constants,lstatSync,openSync,readdirSync,rmSync,rmdirSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {runtimeNativeSystemV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🟦️.ts";
import {runtimeSupportsV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 📐️ The authored policy of the incremental session collector.
 * @see ./🧬️schema/🔣️.json */
export const INCREMENTAL_POLICY_V1=policy;
/** 🌱️ One finalized or working Cargo incremental session: its allocated share, last use, whether a running compiler holds its lock and whether it is the newest of its crate. */
export type IncrementalSessionV1=Readonly<{path:string;bytes:number;lastUsedMs:number;held:boolean;newest:boolean}>;
/** 🧭️ Why a session is removed. */
export type IncrementalReasonV1="idle"|"budget"|"pressure";
/** 🗑️ One planned removal. */
export type IncrementalRemovalV1=Readonly<{path:string;bytes:number;reason:IncrementalReasonV1}>;
/** 📋️ The measured state the pure planner decides on: `needBytes` is what the image must free to fall to its low-water mark. */
export type IncrementalInputV1=Readonly<{volumeBytes:number;needBytes:number;nowMs:number;sessions:readonly IncrementalSessionV1[]}>;
/** 🔒️ A lock this process holds on one session; released when the session is gone. */
export type IncrementalLockV1=Readonly<{release():void}>;
/** 🔌️ Probes the compiler's own lock file of a session without waiting: held by a running compiler, absent, unsupported on this platform, or taken by us. */
export type IncrementalLockPortV1=(lockPath:string)=>IncrementalLockV1|"held"|"missing"|"unsupported";

const byAge=(left:IncrementalSessionV1,right:IncrementalSessionV1)=>left.lastUsedMs-right.lastUsedMs||(left.path<right.path?-1:left.path>right.path?1:0);

/** 🧮️ Plans which sessions to remove, oldest first and never a held or recent one: sessions idle past the limit that a newer session of their crate supersedes, then the oldest until the total fits its share of the image, then the oldest until the image has freed `needBytes`. Pure and deterministic. */
export function planIncrementalV1(input:IncrementalInputV1,override:Partial<typeof policy>={}):IncrementalRemovalV1[]{
 const rules={...policy,...override},ordered=[...input.sessions].sort(byAge),removed=new Map<string,IncrementalRemovalV1>();
 const eligible=(row:IncrementalSessionV1)=>!row.held&&input.nowMs-row.lastUsedMs>=rules.minimumAgeMilliseconds;
 const take=(row:IncrementalSessionV1,reason:IncrementalReasonV1)=>{removed.set(row.path,{path:row.path,bytes:row.bytes,reason});};
 for(const row of ordered)if(eligible(row)&&!row.newest&&input.nowMs-row.lastUsedMs>rules.idleMilliseconds)take(row,"idle");
 let total=ordered.reduce((sum,row)=>removed.has(row.path)?sum:sum+row.bytes,0);const budget=Math.floor(input.volumeBytes*rules.budgetShare);
 for(const row of ordered){if(total<=budget)break;if(eligible(row)&&!removed.has(row.path)){take(row,"budget");total-=row.bytes;}}
 let freed=[...removed.values()].reduce((sum,row)=>sum+row.bytes,0);
 for(const row of ordered){if(freed>=input.needBytes)break;if(eligible(row)&&!removed.has(row.path)){take(row,"pressure");freed+=row.bytes;}}
 return ordered.filter(row=>removed.has(row.path)).map(row=>removed.get(row.path)!);
}

let native:ReturnType<typeof openNative>|undefined;
function openNative(){
 const system=runtimeNativeSystemV1(),library=system.dlopen(process.platform==="darwin"?"/usr/lib/libSystem.B.dylib":"libc.so.6",process.platform==="darwin"?{flock:{args:["i32","i32"],returns:"i32"}}:{fcntl:{args:["i32","i32","ptr"],returns:"i32"}});
 return {system,library};
}

/** 🔐️ Takes the compiler's own session lock without waiting: `flock` on macOS, a POSIX record lock on Linux (the primitives rustc itself locks with); Windows and runtimes without native binding report `unsupported`, so only the age rules apply there. */
export const tryLockSessionV1:IncrementalLockPortV1=lockPath=>{
 if(!["darwin","linux"].includes(process.platform)||!runtimeSupportsV1("ffi"))return "unsupported";
 let fd:number;try{fd=openSync(lockPath,constants.O_RDWR);}catch(error){return (error as NodeJS.ErrnoException).code==="ENOENT"?"missing":"held";}
 try{
  native??=openNative();const library=native.library as any;
  if(process.platform==="darwin"){
   if(library.symbols.flock(fd,2|4)!==0){closeSync(fd);return "held";}
   return {release:()=>{try{library.symbols.flock(fd,8);}finally{closeSync(fd);}}};
  }
  const record=Buffer.alloc(32);record.writeInt16LE(1,0);record.writeInt16LE(0,2);
  if(library.symbols.fcntl(fd,6,native.system.ptr(record))!==0){closeSync(fd);return "held";}
  return {release:()=>{const unlock=Buffer.alloc(32);unlock.writeInt16LE(2,0);try{library.symbols.fcntl(fd,6,native!.system.ptr(unlock));}finally{closeSync(fd);}}};
 }catch{try{closeSync(fd);}catch{}return "held";}
};

const directoriesOf=(path:string):string[]=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>join(path,entry.name));}catch{return [];}};
const isDirectory=(path:string):boolean=>{try{return lstatSync(path).isDirectory();}catch{return false;}};

/** 🗺️ Every `incremental` directory of the compiler image: the declared bases (build and target directories of both modes, the lanes, fresh component generations), at most `maximumProfileDepth` levels of profile and target-triple directories below each. Bounded directory reads only, no recursive walk. */
export function incrementalRootsV1(store:string):string[]{
 const found=new Set<string>();
 for(const pattern of policy.bases){
  let current=[store];
  for(const segment of pattern.split("/"))current=current.flatMap(dir=>segment==="*"?directoriesOf(dir):isDirectory(join(dir,segment))?[join(dir,segment)]:[]);
  for(const base of current){
   let level=[base];
   for(let depth=0;depth<policy.maximumProfileDepth;depth++){level=level.flatMap(directoriesOf).filter(dir=>basename(dir)!==policy.directory);for(const dir of level)if(isDirectory(join(dir,policy.directory)))found.add(join(dir,policy.directory));}
  }
 }
 return [...found].sort();
}

/** 🔑️ The compiler's lock file of a session directory: `s-<stamp>-<id>` plus `.lock`, whatever suffix (`-working` or the content hash) the directory carries. */
export function incrementalLockPathV1(session:string):string{
 const key=basename(session).split("-").slice(0,3).join("-");return join(dirname(session),key+policy.lockSuffix);
}

function allocatedShare(path:string):number{
 let bytes=0,entries=0;const stack=[path];
 while(stack.length){
  const directory=stack.pop()!;let children;try{children=readdirSync(directory,{withFileTypes:true});}catch{continue;}
  for(const child of children){
   if(++entries>policy.maximumEntriesPerSession)return bytes;
   const full=join(directory,child.name);let info;try{info=lstatSync(full);}catch{continue;}
   if(info.isDirectory())stack.push(full);else if(!info.isSymbolicLink())bytes+=Math.round((info.blocks??Math.ceil(info.size/512))*512/Math.max(1,info.nlink));
  }
 }
 return bytes;
}

const turn=()=>new Promise<void>(done=>setImmediate(done));
/** 🔭️ Measures every session below the roots. A session is held when a running compiler holds its lock; without a lock file or a lock backend a recent session counts as held, so age alone never exposes one in use. */
export async function measureIncrementalV1(roots:readonly string[],options:Readonly<{nowMs:number;lock?:IncrementalLockPortV1;signal?:AbortSignal;progress?:(done:number,total:number)=>void}>):Promise<IncrementalSessionV1[]>{
 const lock=options.lock??tryLockSessionV1,rows:IncrementalSessionV1[]=[],crates=roots.flatMap(root=>directoriesOf(root));let done=0;
 for(const crate of crates){
  options.signal?.throwIfAborted();
  let names:string[];try{names=readdirSync(crate);}catch{continue;}
  const group:Omit<IncrementalSessionV1,"newest">[]=[];
  for(const name of names){
   const path=join(crate,name);if(!name.startsWith(policy.sessionPrefix)||name.endsWith(policy.lockSuffix)||!isDirectory(path))continue;
   let used=0;try{used=lstatSync(path).mtimeMs;}catch{continue;}
   const lockPath=incrementalLockPathV1(path);try{used=Math.max(used,lstatSync(lockPath).mtimeMs);}catch{}
   const probe=lock(lockPath),recent=options.nowMs-used<=policy.idleMilliseconds;
   if(typeof probe==="object")probe.release();
   group.push({path,bytes:allocatedShare(path),lastUsedMs:Math.floor(used),held:probe==="held"||(probe==="missing"||probe==="unsupported")&&recent&&name.endsWith(policy.workingSuffix)||probe==="unsupported"&&recent});
  }
  const finalized=group.filter(row=>!row.path.endsWith(policy.workingSuffix)).sort(byAge as never),newest=finalized.length?finalized[finalized.length-1]!.path:undefined;
  for(const row of group)rows.push({...row,newest:row.path===newest});
  options.progress?.(++done,crates.length);if(done%policy.yieldEvery===0)await turn();
 }
 return rows;
}

/** 📊️ What a collection did or, in a dry run, would do. */
export type IncrementalResultV1=Readonly<{dryRun:boolean;total:number;removed:readonly IncrementalRemovalV1[];freedBytes:number;held:number;kept:number}>;
/** 💬️ One localized line with its placeholders filled. */
export function incrementalTextV1(key:keyof typeof messages,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
const gib=(bytes:number)=>bytes>=1073741824?(bytes/1073741824).toFixed(1)+" GiB":(bytes/1048576).toFixed(0)+" MiB";

/** 🗣️ Where and in which language the collection reports; absent, it reports nothing and needs no language. */
export type IncrementalReportV1=Readonly<{language:"en"|"de";write:(line:string)=>void}>;

/** 🧹️ Measures, plans and (unless `dryRun`) removes the sessions: each removal re-takes the compiler's own lock and holds it while the session goes, so a compiler that starts meanwhile finds it taken and uses another session; pressure removals stop as soon as `satisfied()` says the image is back at its low-water mark. Progress and cancellation are served between sessions. */
export async function collectIncrementalV1(store:string,options:Readonly<{dryRun:boolean;volumeBytes:number;needBytes:()=>number;satisfied?:()=>boolean;nowMs?:number;lock?:IncrementalLockPortV1;signal?:AbortSignal;progress?:(done:number,total:number)=>void;report?:IncrementalReportV1}>):Promise<IncrementalResultV1>{
 const nowMs=options.nowMs??Date.now(),lock=options.lock??tryLockSessionV1,report=options.report,say=(key:keyof typeof messages,variables:Readonly<Record<string,string|number>>={}):void=>{if(report)report.write(incrementalTextV1(key,report.language,variables));};
 say("scan",{root:store});
 const sessions=await measureIncrementalV1(incrementalRootsV1(store),{nowMs,lock,signal:options.signal,progress:options.progress}),plan=planIncrementalV1({volumeBytes:options.volumeBytes,needBytes:options.needBytes(),nowMs,sessions}),removed:IncrementalRemovalV1[]=[],crates=new Set<string>();
 const reason=(row:IncrementalRemovalV1)=>report?incrementalTextV1(`reason.${row.reason}` as keyof typeof messages,report.language,{minutes:Math.round(policy.idleMilliseconds/60000)}):"";
 let held=sessions.filter(row=>row.held).length,done=0;
 for(const row of plan){
  options.signal?.throwIfAborted();
  if(options.dryRun){removed.push(row);if(removed.length<=policy.reportedRemovals)say("would",{path:row.path,bytes:gib(row.bytes),reason:reason(row)});continue;}
  if(row.reason==="pressure"&&options.satisfied?.())break;
  const taken=lock(incrementalLockPathV1(row.path));
  if(taken==="held"){held++;say("held",{path:row.path});continue;}
  try{rmSync(row.path,{recursive:true,force:true});rmSync(incrementalLockPathV1(row.path),{force:true});removed.push(row);crates.add(dirname(row.path));say("removed",{path:row.path,bytes:gib(row.bytes),reason:reason(row)});}
  finally{if(typeof taken==="object")taken.release();}
  options.progress?.(++done,plan.length);if(done%policy.yieldEvery===0)await turn();
 }
 for(const crate of crates)try{if(!readdirSync(crate).length)rmdirSync(crate);}catch{}
 const freedBytes=removed.reduce((sum,row)=>sum+row.bytes,0);
 say(options.dryRun?"summaryPlan":"summaryApply",{count:removed.length,total:sessions.length,bytes:gib(freedBytes),held,kept:sessions.length-removed.length-held});
 return Object.freeze({dryRun:options.dryRun,total:sessions.length,removed:Object.freeze(removed),freedBytes,held,kept:sessions.length-removed.length-held});
}
