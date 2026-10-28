import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {isAbsolute,join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored unit-cache policy.
 * @see ../🧬️schema/🔣️.json */
export const UNIT_CACHE_POLICY_V1=schema.definitions.Policy.const;
const policy=UNIT_CACHE_POLICY_V1;

/** 🔗️ One `--extern` the compiler resolves from the dependency directory. */
export type RustcExternV1=Readonly<{flags:string;name:string;path:string}>;
/** 🧾️ The parts of one rustc command line that decide whether and how its result can be reused. */
export type RustcUnitV1=Readonly<{crateName:string;crateTypes:readonly string[];emit:readonly string[];outDir:string;source:string;target:string|undefined;externs:readonly RustcExternV1[];links:readonly string[];nativeSearch:boolean;incremental:string|undefined;artifacts:boolean}>;
/** 🚫️ Why a command line is never cached, `undefined` when it can be. */
export type RustcBypassV1=string|undefined;

const valueFlags=new Set(["--crate-name","--crate-type","--emit","--out-dir","--target","--edition","--extern","-L","-l","-C","--cfg","--check-cfg","--cap-lints","--error-format","--json","--diagnostic-width","--remap-path-prefix","-A","-W","-D","-F","--force-warn","-Z","--codegen","--sysroot","--print","--explain","-o","--color","--allow","--warn","--deny","--forbid","--force-warn","--emit-dep-info"]);

/** 📖️ Expands `@file` response files exactly as rustc does: one argument per line. */
export function expandRustcArgumentsV1(args:readonly string[],cwd:string):string[]{
 return args.flatMap(argument=>{if(!argument.startsWith("@")||argument.length<2)return[argument];try{return readFileSync(resolve(cwd,argument.slice(1)),"utf8").split(/\r?\n/u).filter(line=>line.length>0);}catch{return[argument];}});
}

type Option=Readonly<{index:number;span:number;flag:string;value:string}>;
function options(args:readonly string[]):{options:Option[];positional:string[]}{
 const found:Option[]=[],positional:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(argument==="-"||!argument.startsWith("-")){positional.push(argument);continue;}
  const equals=argument.startsWith("--")?argument.indexOf("="):-1;
  if(equals>0){found.push({index,span:1,flag:argument.slice(0,equals),value:argument.slice(equals+1)});continue;}
  if(valueFlags.has(argument)){found.push({index,span:2,flag:argument,value:args[index+1]??""});index++;continue;}
  const short=/^-([LlCAWDFZo])(.+)$/u.exec(argument);
  if(short){found.push({index,span:1,flag:"-"+short[1],value:short[2]!});continue;}
  found.push({index,span:1,flag:argument,value:""});
 }
 return{options:found,positional};
}

/** 🔬️ Reads the compilation-relevant options of one rustc command line. */
export function parseRustcUnitV1(args:readonly string[]):RustcUnitV1{
 const parsed=options(args),all=(flag:string)=>parsed.options.filter(row=>row.flag===flag).map(row=>row.value),first=(flag:string)=>all(flag)[0];
 const codegen=parsed.options.filter(row=>row.flag==="-C").map(row=>row.value),incremental=codegen.find(value=>value.startsWith("incremental="))?.slice("incremental=".length);
 const externs=all("--extern").map(value=>{const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1),colon=spec.lastIndexOf(":");return Object.freeze({flags:colon<0?"":spec.slice(0,colon+1),name:colon<0?spec:spec.slice(colon+1),path});});
 const json=all("--json").join(",");
 return Object.freeze({crateName:first("--crate-name")??"",crateTypes:Object.freeze(all("--crate-type").flatMap(value=>value.split(","))),emit:Object.freeze(all("--emit").flatMap(value=>value.split(",").map(part=>part.split("=")[0]!))),outDir:first("--out-dir")??"",source:parsed.positional[0]??"",target:first("--target"),externs:Object.freeze(externs),links:Object.freeze(all("-l")),nativeSearch:all("-L").some(value=>/^(native|all)=/u.test(value)),incremental,artifacts:json.includes("artifacts")});
}

/** ⚡️ Decides whether a command line can be served from the cache. */
export function rustcBypassV1(unit:RustcUnitV1,args:readonly string[]):RustcBypassV1{
 if(unit.source===""||unit.source==="-"||!unit.source.endsWith(".rs"))return"no-source-file";
 if(unit.crateName==="")return"no-crate-name";
 if(unit.outDir==="")return"no-out-dir";
 if(args.some(argument=>argument==="--print"||argument.startsWith("--print=")||argument==="-vV"||argument==="--version"||argument==="-V"||argument==="--explain"))return"query";
 if(!unit.artifacts)return"no-artifact-messages";
 if(!unit.emit.includes("link")&&!unit.emit.includes("metadata"))return"no-emit";
 const types=unit.crateTypes.length?unit.crateTypes:["bin"],allowed=policy.key.cacheableCrateTypes;
 if(types.some(type=>!allowed.includes(type)))return"crate-type:"+types.join(",");
 if(types.some(type=>policy.key.linkedCrateTypes.includes(type))&&unit.emit.includes("link")&&(unit.links.length>0||unit.nativeSearch))return"native-link";
 return undefined;
}

/** 🔤️ Hex SHA-256 of bytes or text. */
export function sha256V1(value:string|Uint8Array):string{return createHash("sha256").update(value).digest("hex");}

/** 🧭️ Replaces every occurrence of the lane root so equal compilations in different lanes produce equal text. */
export function laneNeutralV1(text:string,laneRoot:string|undefined):string{
 if(!laneRoot)return text;
 const forms=new Set([laneRoot,laneRoot.replaceAll("\\","/"),laneRoot.replaceAll("\\","\\\\")]);
 let result=text;for(const form of[...forms].sort((left,right)=>right.length-left.length))result=result.split(form).join(policy.key.laneToken);
 return result;
}

/** 🏠️ Restores the lane root into stored text. */
export function laneRestoreV1(text:string,laneRoot:string|undefined):string{
 if(!laneRoot)return text;
 return text.split(policy.key.laneToken).join(laneRoot);
}

/** ✂️ Rewrites a command line without the parts that differ between lanes: the output directory, the incremental directory, `-L dependency` search paths and extern paths. */
export function normalizeRustcArgumentsV1(args:readonly string[],externDigests:ReadonlyMap<string,string>,laneRoot:string|undefined):string[]{
 const result:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  const take=(flag:string):string|undefined=>{if(argument===flag){index++;return args[index]??"";}if(argument.startsWith(flag+"="))return argument.slice(flag.length+1);return undefined;};
  if(take("--out-dir")!==undefined)continue;
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental="))continue;
  if(argument==="-L"&&(args[index+1]??"").startsWith("dependency=")){index++;continue;}
  if(argument.startsWith("-Ldependency="))continue;
  if(argument==="--extern"||argument.startsWith("--extern=")){const value=take("--extern")!,equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1);result.push("--extern",path===""?spec:spec+"=#"+(externDigests.get(path)??"missing"));continue;}
  result.push(laneNeutralV1(argument,laneRoot));
 }
 return result;
}

/** 📥️ The selected environment the compilation can observe without declaring it in its dependency file. */
export function keyEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,laneRoot:string|undefined):Array<[string,string]>{
 const names=new Set(policy.key.environmentNames),rows:Array<[string,string]>=[];
 for(const name of Object.keys(environment).sort()){const value=environment[name];if(value===undefined)continue;if(names.has(name)||policy.key.environmentPrefixes.some(prefix=>name.startsWith(prefix)))rows.push([name,laneNeutralV1(value,laneRoot)]);}
 return rows;
}

/** 🔑️ The first-level key: toolchain, normalized command line, root source bytes, dependency artifact bytes and selected environment. */
export function unitKeyV1(input:Readonly<{toolchain:string;normalized:readonly string[];sourceSha256:string;externs:readonly Readonly<{name:string;sha256:string}>[];environment:readonly (readonly [string,string])[]}>):string{
 return sha256V1(JSON.stringify([policy.namespace,input.toolchain,input.normalized,input.sourceSha256,input.externs.map(row=>[row.name,row.sha256]),input.environment]));
}

/** 🧩️ Names the parts of a first-level key by short digests (toolchain, arguments, source, externs, environment), so two compilations of one crate that disagree on the key are explained by the part that differs.
 * @see ./🧪️tests/🧩️parts/🟦️.ts */
export function unitKeyPartsV1(input:Parameters<typeof unitKeyV1>[0]):string{
 const part=(value:unknown)=>sha256V1(JSON.stringify(value)).slice(0,8);
 return["toolchain:"+part(input.toolchain),"args:"+part(input.normalized),"source:"+part(input.sourceSha256),"externs:"+part(input.externs.map(row=>[row.name,row.sha256])),"environment:"+part(input.environment)].join(" ");
}

function unescapeDepPath(text:string):string{return text.replace(/\\ /gu," ");}

/** 📜️ Reads a Makefile-style dependency file: the files the compilation read and the environment variables it observed. */
export function parseDepInfoV1(text:string):{dependencies:string[];environment:Array<{name:string;value:string|null}>}{
 const dependencies:string[]=[],environment:Array<{name:string;value:string|null}>=[];
 let ruleDone=false;
 for(const raw of text.split(/\r?\n/u)){
  if(raw.startsWith("# env-dep:")){const body=raw.slice("# env-dep:".length),equals=body.indexOf("=");environment.push(equals<0?{name:body,value:null}:{name:body.slice(0,equals),value:body.slice(equals+1).replace(/\\n/gu,"\n").replace(/\\r/gu,"\r").replace(/\\\\/gu,"\\")});continue;}
  if(raw.startsWith("#")||raw.trim()==="")continue;
  if(ruleDone)continue;
  const colon=raw.search(/(?<!\\): |(?<!\\):$/u);
  if(colon<0)continue;
  const rest=raw.slice(colon+1).trim();
  for(const token of rest.split(/(?<!\\) /u)){if(token!=="")dependencies.push(unescapeDepPath(token));}
  ruleDone=true;
 }
 return{dependencies,environment};
}

/** 🗂️ Resolves a dependency path against the compiler's working directory. */
export function resolveDependencyV1(cwd:string,path:string):string{return isAbsolute(path)?path:resolve(cwd,path);}

/** 🧬️ The identity of the compiler behind `inner`, from its `-vV` text: `SEMIO_UNIT_CACHE_TOOLCHAIN` when the launcher states it, else a probe memoised for one minute. */
export function toolchainIdentityV1(root:string,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string{
 if(environment.SEMIO_UNIT_CACHE_TOOLCHAIN)return environment.SEMIO_UNIT_CACHE_TOOLCHAIN;
 const memo=join(root,"tc-"+sha256V1(inner.join("\0")+"\0"+(environment.RUSTUP_TOOLCHAIN??"")).slice(0,16));
 try{if(Date.now()-statSync(memo).mtimeMs<60000)return readFileSync(memo,"utf8");}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"-vV"],{encoding:"utf8",env:environment as NodeJS.ProcessEnv});
 if(result.status!==0)throw Error("Compiler identity probe failed");
 mkdirSync(root,{recursive:true});writeFileSync(memo,result.stdout);return result.stdout;
}

/** 📨️ What the build broker is told about a unit: exactly the fields of the scheduler's `AdmitRequestV1` (the Rust `parse_unit` is the reference), `undefined` for probes. */
export type AdmitFactsV1=Readonly<{crate:string;package:string;version:string;target:string;crateTypes:readonly string[];outDir:string;extraFilename:string;source:string;manifestDir:string;primary:boolean;buildScriptOutDir:string|null;externs:readonly string[]}>;
export function admitFactsV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>,cwd:string):AdmitFactsV1|undefined{
 const unit=parseRustcUnitV1(args);
 if(unit.crateName===""||args.some(argument=>argument==="-vV"||argument==="-V"||argument==="--print"||argument.startsWith("--print=")||argument==="-"))return undefined;
 let extraFilename="";
 for(let index=0;index<args.length;index++){const argument=args[index]!;const value=argument==="-C"?args[index+1]:argument.startsWith("-C")?argument.slice(2):undefined;if(value?.startsWith("extra-filename="))extraFilename=value.slice("extra-filename=".length);}
 const absolute=(path:string)=>path===""||isAbsolute(path)?path:resolve(cwd,path);
 return Object.freeze({crate:unit.crateName,package:environment.CARGO_PKG_NAME??"",version:environment.CARGO_PKG_VERSION??"",target:unit.target??"host",crateTypes:Object.freeze(unit.crateTypes.length?[...unit.crateTypes]:["lib"]),outDir:absolute(unit.outDir),extraFilename,source:absolute(unit.source),manifestDir:environment.CARGO_MANIFEST_DIR??"",primary:environment.CARGO_PRIMARY_PACKAGE!==undefined,buildScriptOutDir:environment.OUT_DIR??null,externs:Object.freeze(unit.externs.filter(row=>row.path!=="").map(row=>absolute(row.path)))});
}

/** 🧪️ The environment a build script run is keyed by: Cargo's per-package, per-feature, per-cfg and dependency-metadata variables with the lane neutralised; never `NUM_JOBS` or the job server. */
export function buildKeyEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,laneRoot:string|undefined):Array<[string,string]>{
 const names=new Set(policy.buildScript.environmentNames),rows:Array<[string,string]>=[];
 for(const name of Object.keys(environment).sort()){const value=environment[name];if(value===undefined)continue;if(names.has(name)||policy.buildScript.environmentPrefixes.some(prefix=>name.startsWith(prefix)))rows.push([name,laneNeutralV1(value,laneRoot)]);}
 return rows;
}

/** 📋️ The `cargo:rerun-if-*` directives of a build script's standard output. */
export function parseBuildDirectivesV1(stdout:string):{files:string[];variables:string[]}{
 const files:string[]=[],variables:string[]=[];
 for(const line of stdout.split(/\r?\n/u)){
  const match=/^cargo::?(rerun-if-changed|rerun-if-env-changed)=(.*)$/u.exec(line);if(!match)continue;
  (match[1]==="rerun-if-changed"?files:variables).push(match[2]!);
 }
 return{files,variables};
}
