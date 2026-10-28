import racesSchema from "../🧬️schema/🌊️races.json" with {type:"json"};
import {lstatSync} from "node:fs";
import {posix,win32} from "node:path";

type RaceKindV1="vanished"|"shape"|"stale"|"busy";
type RaceTableV1=Readonly<{codes:Readonly<{all:Readonly<Record<string,RaceKindV1>>;win32:Readonly<Record<string,RaceKindV1>>}>;readSyscalls:readonly string[];reasons:Readonly<Record<RaceKindV1,RaceReasonV1>>;persistentReason:RaceReasonV1;witness:Readonly<{slackMilliseconds:number}>;ownedSegments:readonly string[];rootMarkers:readonly string[];text:Readonly<{codePattern:string;syscallPattern:string;pathPatterns:readonly string[];windowCharacters:number;propertyPatterns:Readonly<{code:string;syscall:string}>}>}>;
const table=Object.freeze(JSON.parse(JSON.stringify(racesSchema.definitions.Table.const)) as RaceTableV1);

/** 🏷️ The reason a race is announced with: the vocabulary of the churn policy, so the retry texts exist in every declared language. */
export type RaceReasonV1="vanished"|"identity"|"busy";
/** 🧾️ What is known when a filesystem error is classified: the error, the platform, the workspace root, the probe of the path now and the newest change time of the directories around it. */
export type RaceFactsV1=Readonly<{code:string;syscall:string|undefined;path:string;platform:string;root:string|undefined;startedAtMs:number;probe:"present"|"absent"|"unknown";changeMs:number|undefined}>;
/** ⚖️ The verdict: `transient` (read again), `persistent` (only a new snapshot or process cures it), `none` (not a race, the error stands); `cause` names the rule that decided. */
export type RaceDecisionV1=Readonly<{disposition:"transient"|"persistent"|"none";reason:RaceReasonV1|undefined;path:string;cause:string}>;
/** 🌍️ The ports the classification reads the world through: the clock of this process and one no-follow stat. */
export type RaceWorldV1=Readonly<{root:string|undefined;startedAtMs:number;platform:string;lstat:(path:string)=>Readonly<{directory:boolean;mtimeMs:number;ctimeMs:number}>|"absent"|"unknown"}>;

const none=(path:string,cause:string):RaceDecisionV1=>Object.freeze({disposition:"none",reason:undefined,path,cause});
const kindOf=(code:string,platform:string):RaceKindV1|undefined=>Object.hasOwn(table.codes.all,code)?table.codes.all[code]:platform==="win32"&&Object.hasOwn(table.codes.win32,code)?table.codes.win32[code]:undefined;

/** 📕️ The closed table of race codes, read syscalls, reasons and witness slack exactly as the schema declares it. */
export function filesystemRaceTableV1():RaceTableV1{return table;}

/** 🧮️ Decides, as a pure function of the facts, whether a filesystem error is a race with a peer: only a read of an absolute path inside the workspace (outside the roots this tooling owns) with a known code, and for a vanished or replaced path only when a directory around it changed since this process started. */
export function decideFilesystemRaceV1(facts:RaceFactsV1):RaceDecisionV1{
 const kind=kindOf(facts.code,facts.platform);if(kind===undefined)return none(facts.path,"code");
 if(facts.syscall!==undefined&&!table.readSyscalls.includes(facts.syscall))return none(facts.path,"write");
 const flavor=facts.platform==="win32"?win32:posix;
 if(!flavor.isAbsolute(facts.path))return none(facts.path,"relative");
 if(facts.root===undefined)return none(facts.path,"root");
 const relation=flavor.relative(facts.root,facts.path);
 if(relation===".."||relation.startsWith(".."+flavor.sep)||flavor.isAbsolute(relation))return none(facts.path,"outside");
 if(relation.split(flavor.sep).some(segment=>table.ownedSegments.includes(segment)))return none(facts.path,"owned");
 if(kind==="busy")return Object.freeze({disposition:"transient",reason:table.reasons.busy,path:facts.path,cause:"busy"});
 if(facts.changeMs===undefined||facts.changeMs<facts.startedAtMs-table.witness.slackMilliseconds)return none(facts.path,"unchanged");
 if(facts.probe==="unknown")return none(facts.path,"probe");
 if(facts.probe==="absent")return Object.freeze({disposition:"persistent",reason:table.persistentReason,path:facts.path,cause:"absent"});
 return kind==="shape"?Object.freeze({disposition:"persistent",reason:table.reasons.shape,path:facts.path,cause:"shape"}):Object.freeze({disposition:"transient",reason:table.reasons[kind],path:facts.path,cause:"present"});
}

const newest=(row:Readonly<{mtimeMs:number;ctimeMs:number}>):number=>Math.max(row.mtimeMs,row.ctimeMs);
/** 🔦️ The probe of the path and the witness of a change around it: for a present path the newest time of the path and its directory, for a missing one the newest time of the nearest existing directory above it. */
export function observeRaceWorldV1(path:string,world:RaceWorldV1):Readonly<{probe:"present"|"absent"|"unknown";changeMs:number|undefined}>{
 const flavor=world.platform==="win32"?win32:posix,own=world.lstat(path);
 if(own==="unknown")return{probe:"unknown",changeMs:undefined};
 for(let at=path,step=0;step<64;step++){
  const parent=flavor.dirname(at);if(parent===at)break;
  const row=world.lstat(parent);if(row==="unknown")return{probe:"unknown",changeMs:undefined};
  if(row==="absent"){at=parent;continue;}
  return{probe:own==="absent"?"absent":"present",changeMs:Math.max(newest(row),own==="absent"?0:newest(own))};
 }
 return{probe:own==="absent"?"absent":"present",changeMs:own==="absent"?undefined:newest(own)};
}

let rootCache:Readonly<{cwd:string;environment:string|undefined;root:string|undefined}>|undefined;
function findRoot(start:string,platform:string,exists:(path:string)=>boolean):string|undefined{
 const flavor=platform==="win32"?win32:posix;
 for(let at=start;;){if(table.rootMarkers.every(marker=>exists(flavor.join(at,marker))))return at;const parent=flavor.dirname(at);if(parent===at)return;at=parent;}
}
const present=(path:string):boolean=>{try{lstatSync(path);return true;}catch{return false;}};
/** 🏠️ The workspace root of this process: `SEMIO_WORKSPACE_ROOT` when it names a workspace, else the nearest directory above the working directory that carries the root markers. */
export function workspaceRootOfProcessV1(environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const cwd=process.cwd(),named=environment.SEMIO_WORKSPACE_ROOT;
 if(rootCache&&rootCache.cwd===cwd&&rootCache.environment===named)return rootCache.root;
 const root=findRoot(named&&table.rootMarkers.every(marker=>present(posix.join(named,marker))||present(win32.join(named,marker)))?named:cwd,process.platform,present);
 rootCache=Object.freeze({cwd,environment:named,root});return root;
}

const startedAtMsV1=Date.now()-process.uptime()*1000;
/** 🗺️ The real world of this process: its start time, its platform, the workspace root it runs in and a no-follow stat that maps a missing path (also a missing or non-directory ancestor) to `absent`. */
export function processRaceWorldV1(environment:Readonly<Record<string,string|undefined>>=process.env):RaceWorldV1{
 return Object.freeze({root:workspaceRootOfProcessV1(environment),startedAtMs:startedAtMsV1,platform:process.platform,lstat:(path:string)=>{
  try{const row=lstatSync(path);return{directory:row.isDirectory(),mtimeMs:row.mtimeMs,ctimeMs:row.ctimeMs};}
  catch(error){const code=(error as {code?:unknown}).code;return code==="ENOENT"||code==="ENOTDIR"?"absent":"unknown";}
 }});
}

/** 🔎️ Classifies one error object that carries a filesystem `code`, `syscall` and `path` (the shape Node and Bun give); anything else is no race, and the classification itself never throws (a machine out of descriptors answers "no race" and the original error stands). What the table decides without looking at the disk (unknown code, a write, a relative path, outside the workspace, a root this tooling owns, a busy file) is answered before any stat. */
export function filesystemRaceOfErrorV1(error:unknown,given?:RaceWorldV1):RaceDecisionV1{
 const row=error as {code?:unknown;syscall?:unknown;path?:unknown}|null;
 if(row===null||typeof row!=="object"||typeof row.code!=="string"||typeof row.path!=="string")return none("","shape");
 if(kindOf(row.code,given?.platform??process.platform)===undefined)return none(row.path,"code");
 let world:RaceWorldV1;try{world=given??processRaceWorldV1();}catch{return none(row.path,"probe");}
 const known={code:row.code,syscall:typeof row.syscall==="string"?row.syscall:undefined,path:row.path,platform:world.platform,root:world.root,startedAtMs:world.startedAtMs};
 const early=decideFilesystemRaceV1({...known,probe:"unknown",changeMs:undefined});
 if(early.cause!=="unchanged")return early;
 try{const seen=observeRaceWorldV1(row.path,world);return decideFilesystemRaceV1({...known,probe:seen.probe,changeMs:seen.changeMs});}
 catch{return none(row.path,"probe");}
}

const patterns=Object.freeze({code:new RegExp(table.text.codePattern,"u"),syscall:new RegExp(table.text.syscallPattern,"u"),paths:table.text.pathPatterns.map(source=>new RegExp(source,"gu")),propertyCode:new RegExp(table.text.propertyPatterns.code,"u"),propertySyscall:new RegExp(table.text.propertyPatterns.syscall,"u")});
/** 📄️ Reads the code, the syscall and the first absolute path out of the printed form of a filesystem error that ends a captured output (only its last `windowCharacters`, where the error that killed a process is printed) (`ENOENT: no such file or directory, lstat '/p'` or the property lines Bun prints). */
export function filesystemErrorFromTextV1(text:string,platform:string=process.platform):Readonly<{code:string;syscall:string|undefined;path:string}>|undefined{
 const flavor=platform==="win32"?win32:posix,window=text.length>table.text.windowCharacters?text.slice(-table.text.windowCharacters):text;text=window;const code=patterns.propertyCode.exec(text)?.[1]??patterns.code.exec(text)?.[1];if(code===undefined)return;
 const syscall=patterns.propertySyscall.exec(text)?.[1]??patterns.syscall.exec(text)?.[1];
 for(const pattern of patterns.paths)for(const found of text.matchAll(pattern)){const path=found[1];if(path!==undefined&&flavor.isAbsolute(path))return{code,syscall,path};}
 return;
}

/** 📜️ Classifies the printed form of a filesystem error captured from a failed child's output, with the same rules as an error object. */
export function filesystemRaceOfTextV1(text:string,given?:RaceWorldV1):RaceDecisionV1{
 const error=filesystemErrorFromTextV1(text,given?.platform??process.platform);return error===undefined?none("","shape"):filesystemRaceOfErrorV1(error,given);
}

/** 🧱️ The error a reader raises when the path it expects to be a regular file is something else: a directory or a symlink carries the code the operating system gives for it (`EISDIR`, `ELOOP`), so the same classification decides whether a peer replaced the file; any other shape stays a plain error unless the caller names the code (an ancestor that became a file is `ENOTDIR`). */
export function filesystemShapeErrorV1(shape:Readonly<{isDirectory():boolean;isSymbolicLink():boolean}>,path:string,message:string,fallbackCode?:string,syscall:string="lstat"):Error{
 const code=shape.isDirectory()?"EISDIR":shape.isSymbolicLink()?"ELOOP":fallbackCode;
 return code===undefined?Error(message):Object.assign(Error(message),{code,syscall,path});
}
