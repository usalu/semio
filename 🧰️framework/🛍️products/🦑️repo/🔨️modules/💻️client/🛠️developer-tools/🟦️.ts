import {lstat,realpath} from "node:fs/promises";
import {isAbsolute,join,relative,resolve,sep,win32} from "node:path";
import requestSchema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🧬️ Schema-owned requests keep debugger options fixed and program arguments literal. */
export type DeveloperRequest = Readonly<{command:"open-source";file:string;line?:number;editor:"system"|"vscode"|"codium"|"sublime"}> | Readonly<{command:"git-status";path?:string;untracked?:"normal"|"all"|"no"}> | Readonly<{command:"git-diff";path?:string;revision?:string;area?:"worktree"|"staged"}> | Readonly<{command:"git-history";path?:string;revision?:string;count?:number}> | Readonly<{command:"git-show";path?:string;revision?:string;view?:"summary"|"patch"}> | Readonly<{command:"git-blame";file:string;revision?:string;line?:number;count?:number}> | Readonly<{command:"terminal";directory?:string;shell:"system"|"powershell"|"bash"|"zsh"|"cmd"}> | Readonly<{command:"debug";profile:"node-inspect"|"python-pdb"|"gdb"|"lldb";program:string;directory?:string;arguments?:readonly string[]}>;
/** 📝️ One process request contains an exact argument vector and workspace custody. */
export interface DeveloperPlan {readonly command:string;readonly args:string[];readonly cwd:string;readonly detached:boolean;}
/** 🎛️ Adapts only the repository-owned process interface, with the original cancellation signal. */
export interface DeveloperProcess {run(plan:DeveloperPlan,signal:AbortSignal):Promise<void>;open(plan:DeveloperPlan,signal:AbortSignal):Promise<void>;}

/** 🪲️ Describes installed system debuggers without granting an arbitrary command path. */
export interface DeveloperDebugger {readonly profile:string;readonly program:string;readonly title:Readonly<{en:string;de:string}>;readonly installed:boolean;readonly path?:string;readonly constraint?:"windows-ascii-paths";}

function debuggerProfile(profile:string,platform:string){
 if(platform!=="win32"&&platform!=="darwin"&&platform!=="linux")throw Error("developer.debug.platform-unsupported");
 const declaration=requestSchema.$defs.DebuggerProfiles.const.find(row=>row.id===profile);if(!declaration)throw Error("developer.debug.profile-invalid");return{...declaration,program:declaration.programs[platform]};
}

/** 🔎️ Reports each fixed profile against the caller's executable discovery port. */
export function developerDebuggers(platform:string,find:(program:string)=>string|null):DeveloperDebugger[]{
 return requestSchema.$defs.DebuggerProfiles.const.map(row=>{const profile=debuggerProfile(row.id,platform),path=find(profile.program);return{profile:profile.id,program:profile.program,title:profile.title,installed:path!==null,...(path?{path}:{}),...(platform==="win32"&&profile.id==="gdb"?{constraint:"windows-ascii-paths" as const}:{})};});
}

/** 🔐️ Refuses unknown schema fields before filesystem or process access. */
export function readDeveloperRequest(value:unknown):DeveloperRequest {
 if(validateJsonSchemaSubset(requestSchema,value).length)throw Error("developer.request.invalid");const request=value as DeveloperRequest;
 if(request.command==="debug"){const args=[...request.arguments??[]];if(args.reduce((sum,word)=>sum+word.length+1,0)>requestSchema.$defs.DebuggerLimits.const.maximumArgumentUnits)throw Error("developer.debug.arguments-too-large");return Object.freeze({...request,arguments:Object.freeze(args)});}return request;
}

function workspacePath(root:string,value:string):string {
 const normalized=value.replaceAll("\\","/");
 if(!normalized||normalized.length>256||/[\u0000-\u001f\u007f]/u.test(normalized)||isAbsolute(normalized)||win32.isAbsolute(normalized)||normalized.includes(":"))throw Error("developer.path.relative-required");
 const parts=normalized.split("/");if(parts.some(part=>part===".."||part===""))throw Error("developer.path.relative-required");
 const absolute=resolve(root,normalized),distance=relative(root,absolute);if(isAbsolute(distance)||distance===".."||distance.startsWith(`..${sep}`)||absolute.length>256)throw Error("developer.path.outside-workspace");return absolute;
}

async function physicalWorkspacePath(root:string,value:string,kind:"file"|"directory"):Promise<string> {
 const absolute=workspacePath(root,value),physicalRoot=await realpath(root);let physical:string;try{physical=await realpath(absolute);}catch(error){if(["ENOENT","ENOTDIR"].includes((error as {code:string}).code))throw Error(`developer.path.${kind}-required`);throw error;}const distance=relative(physicalRoot,physical);
 if(isAbsolute(distance)||distance===".."||distance.startsWith(`..${sep}`))throw Error("developer.path.symlink-escape");
 const stat=await lstat(absolute);if(kind==="file"?!stat.isFile():!stat.isDirectory())throw Error(`developer.path.${kind}-required`);return absolute;
}

/** 🧭️ Selects passive navigation, supervised debugging, a terminal or read-only Git through fixed portable argv. */
export async function planDeveloperRequest(root:string,value:unknown,platform:string,environment:Readonly<Record<string,string|undefined>>):Promise<DeveloperPlan> {
 const request=readDeveloperRequest(value),cwd=resolve(root);
 if(request.command==="debug"){
  const profile=debuggerProfile(request.profile,platform),program=await physicalWorkspacePath(cwd,request.program,"file"),directory=await physicalWorkspacePath(cwd,request.directory??".","directory"),target=relative(directory,program).replaceAll("\\","/");if(platform==="win32"&&request.profile==="gdb"&&/[^\u0020-\u007e]/u.test(program+directory))throw Error("developer.debug.windows-gdb-unicode");return{command:profile.program,args:[...profile.arguments,target.startsWith(".")?target:`./${target}`,...request.arguments??[]],cwd:directory,detached:false};
 }
 if(request.command==="terminal"){
  const directory=await physicalWorkspacePath(cwd,request.directory??".","directory"),shell=request.shell==="system"?(platform==="win32"?"powershell":platform==="darwin"?"zsh":"sh"):request.shell;
  const commands:Record<string,[string,string[]]>={powershell:[platform==="win32"?"powershell.exe":"pwsh",["-NoLogo","-NoProfile","-NoExit"]],bash:["bash",["--noprofile","--norc","-i"]],zsh:["zsh",["-f","-i"]],cmd:["cmd.exe",["/D","/Q","/K"]],sh:["/bin/sh",["-i"]]};
  const [command,args]=commands[shell]!;return{command,args,cwd:directory,detached:false};
 }
 if(request.command==="open-source"){
  const file=await physicalWorkspacePath(cwd,request.file,"file"),position=request.line===undefined?file:`${file}:${request.line}`;
  if(environment.SEMIO_SOURCE_OPENER){const words:unknown=JSON.parse(environment.SEMIO_SOURCE_OPENER);if(!Array.isArray(words)||words.length===0||words.some(word=>typeof word!=="string"||!word||word.includes("\0")))throw Error("developer.editor.opener-invalid");return{command:words[0],args:[...words.slice(1),position],cwd,detached:false};}
  if(request.editor==="system"&&request.line!==undefined)throw Error("developer.editor.line-requires-capability");
  if(request.editor==="system")return{command:platform==="win32"?"explorer.exe":platform==="darwin"?"open":"xdg-open",args:[file],cwd,detached:false};
  let command=request.editor==="vscode"?"code":request.editor==="codium"?"codium":"subl";
  if(platform==="win32"){
   const candidates=request.editor==="vscode"?[join(environment.LOCALAPPDATA??"","Programs/Microsoft VS Code/Code.exe"),join(environment.ProgramFiles??"","Microsoft VS Code/Code.exe")]:request.editor==="codium"?[join(environment.LOCALAPPDATA??"","Programs/VSCodium/VSCodium.exe"),join(environment.ProgramFiles??"","VSCodium/VSCodium.exe")]:[join(environment.ProgramFiles??"","Sublime Text/sublime_text.exe")];
   for(const candidate of candidates)if(isAbsolute(candidate)&&(await lstat(candidate).catch(()=>undefined))?.isFile()){command=candidate;break;}
  }
  return{command,args:request.editor==="sublime"?[position]:["--reuse-window",...(request.line===undefined?[]:["--goto"]),position],cwd,detached:true};
 }
 const path=request.command==="git-blame"?request.file:request.path??".";workspacePath(cwd,path);
 const revision="revision"in request?request.revision:undefined;if(revision!==undefined&&(!revision||! /^[A-Za-z0-9_][A-Za-z0-9_./~^:@{}+-]*$/u.test(revision)))throw Error("developer.git.revision-invalid");
 const args=["--no-pager","--no-optional-locks","-c","core.hooksPath=","-c","core.fsmonitor=false","-c","core.untrackedCache=false","-c","core.quotePath=false","-c","diff.external=","-c","pager.log=false"];
 if(request.command==="git-status")args.push("status","--short","--branch",`--untracked-files=${request.untracked??"normal"}`);
 if(request.command==="git-diff")args.push("diff","--no-ext-diff","--no-textconv","--color=never",...(request.area==="staged"?["--cached"]:[]),...(revision?[revision]:[]));
 if(request.command==="git-history")args.push("log","--no-ext-diff","--no-textconv","--date=iso-strict","--format=%h %ad %an %s",`--max-count=${request.count??50}`,...(revision?[revision]:[]));
 if(request.command==="git-show")args.push("show","--no-ext-diff","--no-textconv","--format=fuller","--stat",...(request.view==="patch"?["--patch"]:[]),"--color=never",revision??"HEAD");
 if(request.command==="git-blame")args.push("blame","--no-textconv","--date=short","-L",`${request.line??1},+${request.count??50}`,...(revision?[revision]:[]));
 args.push("--",`${request.command==="git-blame"?"":":(literal)"}${path.replaceAll("\\","/")}`);return{command:"git",args,cwd,detached:false};
}

/** ▶️ Executes only a fully admitted plan using the supplied process and cancellation ports. */
export async function executeDeveloperRequest(root:string,value:unknown,process:DeveloperProcess,signal:AbortSignal,platform:string,environment:Readonly<Record<string,string|undefined>>):Promise<void> {
 signal.throwIfAborted();const request=readDeveloperRequest(value),plan=await planDeveloperRequest(root,request,platform,environment);signal.throwIfAborted();await(request.command==="open-source"?process.open(plan,signal):process.run(plan,signal));
}

/** ⌨️ Parses a closed typed flag vocabulary and refuses duplicate or unowned argv. */
export function parseDeveloperArguments(words:readonly string[]):DeveloperRequest {
 const command=words[0],request:Record<string,unknown>={command};for(let index=1;index<words.length;index+=2){const flag=words[index]!,value=words[index+1];if(!/^--(?:file|line|editor|path|untracked|revision|area|view|count|directory|shell|profile|program|arguments)$/u.test(flag)||value===undefined||Object.hasOwn(request,flag.slice(2)))throw Error("developer.arguments.invalid");const key=flag.slice(2);request[key]=key==="arguments"?JSON.parse(value):key==="line"||key==="count"?(/^\d+$/u.test(value)?Number(value):NaN):value;}
 if(command==="open-source"&&request.editor===undefined)request.editor="system";if(command==="terminal"&&request.shell===undefined)request.shell="system";return readDeveloperRequest(request);
}
