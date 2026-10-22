import{spawnSync}from"node:child_process";
import{chmodSync,lstatSync,readdirSync}from"node:fs";
import{join}from"node:path";
import protocol from"./🔣️.json"with{type:"json"};
import { runtimeWhichV1 } from "../../../⚙️runtime/🚀️process/🟦️.ts";

type CandidateV1=Readonly<{command:string;arguments?:readonly string[];platforms?:readonly string[]}>;
type ToolRowV1=Readonly<{candidates:readonly CandidateV1[];probe:readonly string[];pattern:string;locate?:readonly string[]}>;
/** 🧰️ Names a tool the test suites may need from the host: an interpreter or a search program that is not part of every machine. */
export type TestToolNameV1=keyof typeof protocol.tools;
/** 🔌️ What resolving a tool touches, replaceable so every host is testable on any host. */
export type TestToolchainPortV1=Readonly<{platform:NodeJS.Platform;which(command:string):string|null;run(command:string,args:readonly string[]):Readonly<{status:number|null;stdout:string}>;environment:Readonly<Record<string,string|undefined>>;write(text:string):void}>;
/** 🔎️ The outcome: an absolute program path, or null with the candidates that were tried. */
export type TestToolResolutionV1=Readonly<{tool:TestToolNameV1;path:string|null;tried:readonly string[]}>;

const systemPort:TestToolchainPortV1=Object.freeze({platform:process.platform,which:command=>runtimeWhichV1(command),run:(command,args)=>{const result=spawnSync(command,[...args],{encoding:"utf8",timeout:10000,windowsHide:true});return{status:result.status,stdout:(result.stdout??"")+(result.stderr??"")};},environment:process.env,write:text=>{process.stderr.write(text+"\n");}});

/** 🔎️ Resolves one tool: the first declared candidate that exists on PATH and answers its probe like the wanted program (a Windows Store stub or a Python 2 answers differently and is skipped); a launcher such as `py -3` is turned into the interpreter it starts, so the answer is always one program path. */
export function resolveTestToolV1(tool:TestToolNameV1,port:TestToolchainPortV1=systemPort):TestToolResolutionV1{
 const row=protocol.tools[tool] as ToolRowV1,tried:string[]=[];
 for(const candidate of row.candidates){
  if(candidate.platforms&&!candidate.platforms.includes(port.platform))continue;
  const extra=candidate.arguments??[];tried.push([candidate.command,...extra].join(" "));
  const found=port.which(candidate.command);if(found===null)continue;
  const probe=port.run(found,[...extra,...row.probe]);
  if(probe.status!==0||!new RegExp(row.pattern,"u").test(probe.stdout.trim()))continue;
  if(extra.length&&row.locate){const located=port.run(found,[...extra,...row.locate]),path=located.stdout.trim();if(located.status===0&&path)return Object.freeze({tool,path,tried});continue;}
  return Object.freeze({tool,path:found,tried});
 }
 return Object.freeze({tool,path:null,tried});
}

/** 🗣️ Renders the missing-tool message in an explicit language: SEMIO_LANGUAGE, then the locale, then the first declared language. */
export function describeMissingTestToolV1(resolution:TestToolResolutionV1,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const language=[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG].map(value=>value?.toLowerCase().split(/[._@-]/u)[0]).find(tag=>tag!==undefined&&(protocol.languages as readonly string[]).includes(tag))??protocol.languages[0]!;
 return protocol.messages.missing[language as "en"].replaceAll("{tool}",resolution.tool).replaceAll("{tried}",resolution.tried.join(", "));
}

const resolved=new Map<TestToolNameV1,TestToolResolutionV1>();
/** 🧰️ Returns the program path of a test tool or null, resolving once per process; a missing tool is announced once with the reason, so a skipped comparison is never silent. */
export function testToolPathV1(tool:TestToolNameV1,port:TestToolchainPortV1=systemPort):string|null{
 const cached=port===systemPort?resolved.get(tool):undefined;if(cached)return cached.path;
 const resolution=resolveTestToolV1(tool,port);
 if(port===systemPort)resolved.set(tool,resolution);
 if(resolution.path===null)port.write(describeMissingTestToolV1(resolution,port.environment));
 return resolution.path;
}
/** 🐍️ The Python 3 interpreter for independent oracles: `python3`, `python` or `py -3`, whichever is a real Python 3, or null. */
export const testPythonV1=():string|null=>testToolPathV1("python");
/** 🔎️ The ripgrep program for scan oracles, or null. */
export const testRipgrepV1=():string|null=>testToolPathV1("ripgrep");
/** 🚧️ Returns the tool path or throws the bilingual message, for a test that must not pass without it. */
export function requireTestToolV1(tool:TestToolNameV1,port:TestToolchainPortV1=systemPort):string{
 const resolution=resolveTestToolV1(tool,port);if(resolution.path===null)throw Error(describeMissingTestToolV1(resolution,port.environment));return resolution.path;
}

const errorCode=(error:unknown)=>(error as NodeJS.ErrnoException)?.code;
/** 🔓️ Makes a tree writable for its owner without any external program (`chmod -R u+w` does not exist on Windows): links are skipped, a vanished entry is ignored, any other failure is thrown. */
export function makeTestTreeWritableV1(directory:string):void{
 const stack=[directory];
 for(let path=stack.pop();path!==undefined;path=stack.pop()){
  let row;try{row=lstatSync(path);}catch(error){if(errorCode(error)==="ENOENT")continue;throw error;}
  if(row.isSymbolicLink())continue;
  try{chmodSync(path,row.mode|0o200);}catch(error){if(errorCode(error)!=="ENOENT")throw error;}
  if(row.isDirectory())for(const name of readdirSync(path))stack.push(join(path,name));
 }
}
