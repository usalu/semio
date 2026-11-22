import {createHash} from "node:crypto";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join,relative} from "node:path";
import {offloadModeV1} from "../../../../../🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import {COMMAND_POLICY_V1} from "../../../../../🏗️build/🌐️remote-execution/🚀️offload/🧰️command/📐️policy/🟦️.ts";
import {leakedRootKindsV1} from "../../🛰️fleet/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import policySchema from "./🧬️schema/🔣️.json" with {type:"json"};

type Environment=Readonly<Record<string,string|undefined>>;

/** ☁️ The authored policy of a build script run on another machine: what a hermetic script may read, the markers of the ones that are not, and the layout of the worker's private root.
 * @see ./🧬️schema/🔣️.json */
export const BUILD_OFFLOAD_POLICY_V1=policySchema.policy;
const policy=BUILD_OFFLOAD_POLICY_V1,ignored=new Set<string>(UNIT_CACHE_POLICY_V1.buildScript.ignoredDirectories);

const sourceMarkers=policy.sourceMarkers.map(row=>({reason:row.reason,pattern:new RegExp(row.pattern,"u")}));
const binaryMarkers=policy.binaryMarkers.map(row=>({reason:row.reason,bytes:Buffer.from(row.text)}));
const declaredName=(name:string):boolean=>name==="OUT_DIR"||name==="CARGO_MANIFEST_DIR"||(policy.environment.names as readonly string[]).includes(name)||policy.environment.prefixes.some(prefix=>name.startsWith(prefix));
const SAFE_TEXT=/^[^\0\r\n]{0,16384}$/u,SAFE_PATH=/^[^\s"'\\`$;|&<>*?(){}\[\]!~#%^=:,\u0000-\u001f\u007f]{1,512}$/u;


/** 📜️ The source file of a package's build script, from its `Cargo.toml` (`build = "path"`, `build = false`), `build.rs` by default; `undefined` when the package has none. */
export function buildScriptSourcePathV1(manifest:string):string|undefined{
 const row=/^\s*build\s*=\s*(?:"([^"]+)"|(false))/mu.exec(manifest);
 if(row?.[2]!==undefined)return undefined;
 return row?.[1]??"build.rs";
}

/** 🔎️ Why the source text of a build script is not hermetic (it spawns programs, reaches the network or the host's libraries, reads a path outside its package, or reads an environment variable the command does not carry), `undefined` when it is. */
export function buildScriptSourceRefusalV1(source:string):string|undefined{
 const code=source.split("\n").filter(line=>!/^\s*\/\//u.test(line)).join("\n");
 for(const row of sourceMarkers)if(row.pattern.test(code))return row.reason;
 if(new RegExp(policy.allEnvironment,"u").test(code))return"reads-all-environment";
 for(const match of code.matchAll(new RegExp(policy.environmentRead,"gu")))if(!declaredName(match[1]!))return"undeclared-environment:"+match[1]!.slice(0,40);
 return undefined;
}

/** 🔎️ Why the bytes of a compiled build script are not hermetic (the names of the environment variables, programs and libraries of host probing it carries), `undefined` when none is. */
export function buildScriptBinaryRefusalV1(bytes:Uint8Array):string|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 for(const row of binaryMarkers)if(buffer.includes(row.bytes))return row.reason;
 return undefined;
}

/** 🌿️ The environment a hermetic run carries, as the worker's command policy takes it: the declared names and prefixes as they are (a value the policy cannot carry is dropped when the script never names the variable), `PATH` the policy's, `OUT_DIR` and `CARGO_MANIFEST_DIR` mapped into the worker's root. A variable of the unit key that the command cannot carry and a script could read (`DEP_*`, `CARGO_MANIFEST_LINKS`, the compiler paths) makes the run local. */
export function buildScriptEnvironmentV1(environment:Environment,source:string):Readonly<{variables:readonly (readonly [string,string])[]}|{refusal:string}>{
 const rows:Array<readonly [string,string]>=[];
 for(const name of Object.keys(environment).sort()){
  const value=environment[name];if(value===undefined)continue;
  if(policy.environment.refused.some(prefix=>name===prefix||name.startsWith(prefix)))return{refusal:"environment:"+name.slice(0,40)};
  if(name==="OUT_DIR"||name==="CARGO_MANIFEST_DIR"||!declaredName(name))continue;
  if(!SAFE_TEXT.test(value)||value.includes("@root")){if(source.includes(name))return{refusal:"unsafe-environment:"+name.slice(0,40)};continue;}
  rows.push([name,value]);
 }
 rows.push(["OUT_DIR",policy.layout.root+"/"+policy.layout.output+"/"+policy.layout.outDirectory],["CARGO_MANIFEST_DIR",policy.layout.root+"/"+policy.layout.input+"/"+policy.layout.manifest],["PATH",COMMAND_POLICY_V1.environment.path]);
 return{variables:rows};
}

/** 🌲️ The files of a package directory a hermetic script may read, as `in/manifest/<relative path>` to the real file, from a bounded sequential walk that skips the directories the package facts skip; `undefined` when a link is found (it could lead anywhere), a name is not portable or more than `maximumFiles` files (or `maximumBytes` bytes) are met. */
export function manifestInputsV1(directory:string,limits:Readonly<{maximumFiles:number;maximumBytes:number}>):ReadonlyMap<string,string>|{refusal:string}{
 const files=new Map<string,string>(),pending=[directory];let bytes=0;
 while(pending.length>0){
  const current=pending.pop()!;let entries;
  try{entries=readdirSync(current,{withFileTypes:true});}catch{return{refusal:"unreadable-input"};}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   const path=join(current,entry.name),name=relative(directory,path).split("\\").join("/");
   if(entry.isSymbolicLink())return{refusal:"symbolic-link:"+name.slice(-60)};
   if(entry.isDirectory()){if(!ignored.has(entry.name))pending.push(path);continue;}
   if(!entry.isFile())return{refusal:"special-file:"+name.slice(-60)};
   if(!SAFE_PATH.test(name))return{refusal:"file-name:"+name.slice(-60)};
   bytes+=lstatSync(path).size;
   if(files.size>=limits.maximumFiles||bytes>limits.maximumBytes)return{refusal:"inputs-too-large"};
   files.set(policy.layout.input+"/"+policy.layout.manifest+"/"+name,path);
  }
 }
 return files;
}

/** 🔁️ Writes a build script's streams as the requester's own run would have: the worker's `<root>/out/out-dir` and `<root>/in/manifest` become the real `OUT_DIR` and manifest directory (the paths of `cargo:rustc-link-search`, `rerun-if-changed`, `rustc-env` and every line a script prints); `undefined` when the text still names another directory of the worker (the run depends on the machine it ran on). */
export function translateBuildStreamV1(value:string,roots:readonly string[],real:Readonly<{outDir:string;manifest:string}>):string|undefined{
 let result=value;
 const ordered=[...new Set(roots)].filter(root=>root.length>0).sort((left,right)=>right.length-left.length),end="(?=$|[\\s/:;,\"'=)\\]}])",escape=(row:string)=>row.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
 for(const root of ordered){
  result=result.replace(new RegExp(escape(root+"/"+policy.layout.output+"/"+policy.layout.outDirectory)+end,"gu"),()=>real.outDir).replace(new RegExp(escape(root+"/"+policy.layout.input+"/"+policy.layout.manifest)+end,"gu"),()=>real.manifest);
 }
 return ordered.some(root=>result.includes(root))?undefined:result;
}

/** 🪞️ The text of the stand-in tool that runs on the worker: it records its private root (as the script sees it and resolved), makes `OUT_DIR` with a marker file (a directory the script left empty would not be an output), makes the real executable runnable and starts it in the package directory like Cargo does. */
export const buildScriptToolTextV1=():string=>policy.tool.lines.join("\n")+"\n";

const sha256=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");

/** 🏁️ How a build script run on another machine ended: the script's own answer (translated into the requester's paths, its `OUT_DIR` written beside where a local run writes it) or the reason it runs here. */
export type BuildScriptOffloadOutcomeV1=Readonly<{kind:"done";stdout:string;stderr:string;origin:string}>|Readonly<{kind:"local";reason:string;detail:string}>;

/** ☁️ Runs one build script on another machine of the fleet through the command offload (the tool `build-script`), or says why it runs here. Hermetic only: no network (workers deny it), the package directory and `OUT_DIR` are the only paths it reads, its environment is the declared one and it spawns nothing; anything else is refused before a byte leaves (`not-portable`, with the reason), and an output that names the worker's directories is withdrawn (`path-dependent-output`). Never throws; progress, cancellation, timeouts and the decline memo are the command offload's. */
export async function offloadBuildScriptRunV1(input:Readonly<{real:string;args:readonly string[];environment:Environment;cwd:string;root:string;lane:string;crate:string;key:string;outDir:string;signal?:AbortSignal;digest?:(path:string)=>string|null;ports?:unknown}>):Promise<BuildScriptOffloadOutcomeV1>{
 const {environment,root}=input,manifestDir=environment.CARGO_MANIFEST_DIR;
 const local=(reason:string,detail:string):BuildScriptOffloadOutcomeV1=>({kind:"local",reason,detail});
 if(offloadModeV1(environment)==="never")return local("never","");
 let offload:typeof import("../../../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts");
 try{offload=await import("../../../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts");}catch(error){return local("infrastructure",String((error as Error)?.message??error).slice(0,120));}
 const refuse=(detail:string):BuildScriptOffloadOutcomeV1=>{try{offload.declineOffloadV1(root,environment,{lane:input.lane,crate:input.crate,key:input.key,reason:"not-portable",detail:"build-script: "+detail});}catch{}return local("not-portable",detail);};
 if(manifestDir===undefined||manifestDir===""||input.cwd!==manifestDir)return refuse("working-directory");
 if(input.args.some(argument=>!/^[A-Za-z0-9_.,=+:-]{1,256}$/u.test(argument)||argument.startsWith("..")))return refuse("arguments");
 let manifest:string,source="";
 try{
  manifest=readFileSync(join(manifestDir,"Cargo.toml"),"utf8");
  const path=buildScriptSourcePathV1(manifest);
  if(path===undefined||relative(manifestDir,join(manifestDir,path)).startsWith(".."))return refuse("source");
  source=readFileSync(join(manifestDir,path),"utf8");
 }catch{return refuse("unreadable-input");}
 const dependencies=/^\s*\[(?:[\w.-]+\.)?build-dependencies\][^[]*/gmu,declared=[...manifest.matchAll(dependencies)].map(match=>match[0]).join("\n");
 for(const row of policy.buildDependencies)if(new RegExp(`^\\s*"?${row.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")}"?\\s*[=.]`,"mu").test(declared))return refuse("build-dependency:"+row);
 const sourceRefusal=buildScriptSourceRefusalV1(source);if(sourceRefusal!==undefined)return refuse(sourceRefusal);
 let binary:Buffer;try{binary=readFileSync(input.real);}catch{return refuse("unreadable-input");}
 const binaryRefusal=buildScriptBinaryRefusalV1(binary);if(binaryRefusal!==undefined)return refuse(binaryRefusal);
 const variables=buildScriptEnvironmentV1(environment,source);if("refusal" in variables)return refuse(variables.refusal);
 const limits=offload.COMMAND_POLICY_V1.limits,files=manifestInputsV1(manifestDir,{maximumFiles:limits.maximumInputFiles-4,maximumBytes:limits.maximumInputBytes});
 if(!(files instanceof Map))return refuse(files.refusal);
 const tools=join(root,policy.layout.tools),toolText=buildScriptToolTextV1(),toolDigest=sha256(toolText),tool=join(tools,toolDigest.slice(0,32));
 let scratch:string|undefined;
 try{
  if(!existsSync(tool)||sha256(readFileSync(tool))!==toolDigest){mkdirSync(tools,{recursive:true});const temporary=`${tool}.${process.pid}.tmp`;writeFileSync(temporary,toolText);chmodSync(temporary,0o755);renameSync(temporary,tool);}
  mkdirSync(join(root,policy.layout.scratch),{recursive:true});scratch=mkdtempSync(join(root,policy.layout.scratch,"run-"));
  const rootFile=join(scratch,"root"),inputs=new Map(files);inputs.set(policy.layout.input+"/"+policy.layout.executable,input.real);
  const before=new Set(existsSync(input.outDir)?readdirSync(input.outDir):[]);
  const answer=await offload.offloadCommandV1({tool:{name:"build-script",digest:toolDigest,file:tool},arguments:[...input.args],inputs,outputs:new Map([[policy.layout.output+"/"+policy.layout.rootFile,rootFile]]),directories:new Map([[policy.layout.output+"/"+policy.layout.outDirectory,input.outDir]]),variables:variables.variables,root,environment,lane:input.lane,label:input.crate+" build script",key:input.key,...input.digest?{digest:input.digest}:{},...input.signal?{signal:input.signal}:{},...input.ports?{ports:input.ports as never}:{}});
  if(answer.kind==="local")return local(answer.reason,answer.detail);
  const discard=()=>{for(const entry of readdirSync(input.outDir))if(!before.has(entry))rmSync(join(input.outDir,entry),{recursive:true,force:true});};
  const marker=join(input.outDir,policy.layout.marker);
  rmSync(marker,{force:true});
  const roots=readFileSync(rootFile,"utf8").split("\n").filter(row=>row.startsWith("/")),named=Object.fromEntries(roots.map((row,index)=>["worker"+index,row]));
  const leaked=leakedRootKindsV1(listFilesV1(input.outDir,limits.maximumOutputFiles),named),stdout=translateBuildStreamV1(answer.stdout,roots,{outDir:input.outDir,manifest:manifestDir}),stderr=translateBuildStreamV1(answer.stderr,roots,{outDir:input.outDir,manifest:manifestDir});
  if(roots.length===0||leaked!==undefined||stdout===undefined||stderr===undefined){
   discard();
   const detail=leaked?.file??(roots.length===0?"worker root unknown":"stream names the worker");
   try{offload.declineOffloadV1(root,environment,{lane:input.lane,crate:input.crate,key:input.key,reason:"path-dependent-output",detail:"build-script: "+detail});}catch{}
   return local("path-dependent-output",detail);
  }
  return{kind:"done",stdout,stderr,origin:answer.origin};
 }catch(error){return local("infrastructure",String((error as Error)?.message??error).slice(0,120));}
 finally{if(scratch!==undefined)rmSync(scratch,{recursive:true,force:true});}
}

function listFilesV1(directory:string,maximum:number):string[]{
 const files:string[]=[],pending=[directory];
 while(pending.length>0&&files.length<=maximum){
  const current=pending.pop()!;
  for(const entry of readdirSync(current,{withFileTypes:true})){const path=join(current,entry.name);if(entry.isDirectory())pending.push(path);else if(entry.isFile())files.push(path);}
 }
 return files;
}
