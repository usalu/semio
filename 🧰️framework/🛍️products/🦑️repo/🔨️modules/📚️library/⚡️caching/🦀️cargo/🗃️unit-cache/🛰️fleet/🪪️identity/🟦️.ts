import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {cpus,homedir,release} from "node:os";
import {isAbsolute,join,relative,resolve,win32} from "node:path";
import {mkdirSync,readFileSync,realpathSync,renameSync,statSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {canonicalMetadataV1,sha256V1,unitKeyV1,UNIT_CACHE_POLICY_V1,type RustcUnitV1} from "../../🔏️key/🟦️.ts";

/** 📐️ The authored fleet-tier policy: timeouts, breaker windows, claim and upload limits, tokens and state layout.
 * @see ../🧬️schema/🔣️.json */
export const FLEET_POLICY_V1=schema.definitions.Policy.const;
const fleet=FLEET_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;

/** 🖥️ What must be equal on two machines before one may use the other's compiler output: operating system, architecture, the exact compiler build (host triple and the digest of its full `-vV` text), the launcher's toolchain digest, the linker world (the Darwin major release, only for kinds that link) and the CPU model (only when the unit is compiled for the native CPU). */
export type PlatformV1=Readonly<{version:1;os:"darwin"|"linux"|"win32"|"any";arch:string;host:string;rustc:string;toolchain:string;linker:string;cpu:string}>;

/** 🔬️ The compiler's own `-vV` text, its host triple and the digest of the text. */
export type CompilerReleaseV1=Readonly<{text:string;host:string;digest:string;neutral?:string}>;

/** 🖥️ The machine a platform identity is read from; the running machine unless a test simulates another. */
export type MachineV1=Readonly<{os:"darwin"|"linux"|"win32";arch:string;darwinMajor:string;cpuModel:string|undefined;cpuInfo:string|undefined}>;

/** 🧬️ Reads the compiler build: probed once and kept beside the cache (for good when the launcher states the toolchain digest the build is a function of, else for one minute), so a unit pays no process start for it. */
export function compilerReleaseV1(root:string,inner:readonly string[],environment:Environment):CompilerReleaseV1{
 const pinned=environment.SEMIO_UNIT_CACHE_TOOLCHAIN??"",directory=join(root,fleet.layout.state),memo=join(directory,"tv-"+sha256V1(inner.join("\0")+"\0"+(environment.RUSTUP_TOOLCHAIN??"")+"\0"+pinned).slice(0,24));
 let text:string|undefined;
 try{if(pinned!==""||Date.now()-statSync(memo).mtimeMs<60000)text=readFileSync(memo,"utf8");}catch{}
 if(text===undefined||!text.includes("host: ")){
  const probe=spawnSync(inner[0]!,[...inner.slice(1),"-vV"],{windowsHide:true,encoding:"utf8",env:environment as NodeJS.ProcessEnv});
  if(probe.status!==0||!probe.stdout.includes("host: "))throw Error("Compiler identity probe failed");
  text=probe.stdout;
  try{mkdirSync(directory,{recursive:true});const temporary=memo+"."+randomBytes(3).toString("hex");writeFileSync(temporary,text);renameSync(temporary,memo);}catch{}
 }
 return Object.freeze({text,host:/^host: (\S+)$/mu.exec(text)![1]!,digest:sha256V1(text),neutral:sha256V1(text.split(/\r?\n/u).filter(line=>!line.startsWith("host: ")).join("\n"))});
}

/** 🧠️ The CPU a native-CPU compilation is bound to: the model name where the system gives one (Node reports `unknown` for every ARM Linux CPU, which would let two different ARM CPUs share code compiled for one of them), else implementer, architecture, variant and part plus a digest of the feature list from the first processor block of `/proc/cpuinfo`. */
export function cpuIdentityV1(model:string|undefined,cpuinfo:string|undefined):string{
 if(model!==undefined&&model!==""&&model!=="unknown")return model;
 const block=(cpuinfo??"").split(/\r?\n\s*\r?\n/u)[0]??"",field=(name:string)=>new RegExp("^"+name+"\\s*:\\s*(.+)$","mu").exec(block)?.[1]?.trim();
 const parts=["CPU implementer","CPU architecture","CPU variant","CPU part"].map(field).filter((value):value is string=>value!==undefined),features=field("Features")??field("flags");
 return parts.length===0&&features===undefined?"unknown":"cpuinfo:"+parts.join(".")+":"+sha256V1(features??"").slice(0,12);
}

const readCpuInfo=():string|undefined=>{try{return process.platform==="linux"?readFileSync("/proc/cpuinfo","utf8"):undefined;}catch{return undefined;}};

const runningMachine=():MachineV1=>({os:process.platform as MachineV1["os"],arch:process.arch,darwinMajor:release().split(".")[0]!,cpuModel:cpus()[0]?.model,cpuInfo:readCpuInfo()});

/** 🌍️ Whether the output of the unit is a function of its inputs and the compiler release alone, not of the machine that compiled it: a library (no linker, no proc-macro) built for a target of another family than the host, for instance WebAssembly. Such a unit is shared by machines of every operating system and architecture. A unit compiled for the native CPU or for the host itself stays bound to its platform. */
export function hostIndependentV1(input:Readonly<{release:CompilerReleaseV1;unit:RustcUnitV1;args:readonly string[]}>):boolean{
 const target=input.unit.target,rule=fleet.hostIndependent;
 if(target===undefined||target===input.release.host||!rule.targetPrefixes.some(prefix=>target.startsWith(prefix)))return false;
 if(input.unit.crateTypes.length===0||!input.unit.crateTypes.every(type=>(rule.crateTypes as readonly string[]).includes(type)))return false;
 return !input.args.some(argument=>/target-cpu=native/u.test(argument));
}

/** 🌍️ The compiler release as a host-independent unit sees it: the digest of the `-vV` text without its `host:` line, so the same compiler build on another operating system is the same release. */
export function neutralReleaseV1(release:CompilerReleaseV1):CompilerReleaseV1{return Object.freeze({text:release.text,host:fleet.hostIndependent.token,digest:release.neutral??release.digest,neutral:release.neutral??release.digest});}

/** 🪪️ The platform identity of one unit (see {@link PlatformV1}); a unit read through a neutral release names no operating system, architecture or linker. */
export function platformOfV1(input:Readonly<{release:CompilerReleaseV1;unit:RustcUnitV1;args:readonly string[];environment:Environment;toolchain:string;machine?:MachineV1}>):PlatformV1{
 const machine=input.machine??runningMachine(),any=fleet.hostIndependent.token;
 if(input.release.host===any)return Object.freeze({version:1,os:any as PlatformV1["os"],arch:any,host:any,rustc:input.release.digest,toolchain:"",linker:"",cpu:""});
 const linked=input.unit.crateTypes.length===0?true:input.unit.crateTypes.some(type=>(fleet.linkedCrateTypes as readonly string[]).includes(type)),native=input.args.some(argument=>/target-cpu=native/u.test(argument));
 return Object.freeze({version:1,os:machine.os,arch:machine.arch,host:input.release.host,rustc:input.release.digest,toolchain:input.toolchain,linker:linked&&machine.os==="darwin"?"darwin-"+machine.darwinMajor:"",cpu:native?cpuIdentityV1(machine.cpuModel,machine.cpuInfo):""});
}

/** 🧭️ One path root a unit sees and the token that stands for it in everything shared. */
export type PortableRootV1=Readonly<{path:string;token:string;preferred?:boolean}>;

const escape=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
const windowsPath=(path:string):boolean=>/^[A-Za-z]:[\\/]/u.test(path)||/^\\\\[^\\]/u.test(path);
const absolute=(path:string):string=>windowsPath(path)?win32.resolve(path):resolve(path);
const forms=(path:string):string[]=>[...new Set([path,path.replaceAll("\\","/"),path.replaceAll("\\","\\\\")])];

/** 🔗️ Every spelling of one directory the compiler and Cargo may use for it: the path, its real path and, on macOS, the form without the `/private` prefix that `/var`, `/tmp` and `/etc` links hide. */
function spellings(path:string):string[]{
 const rows=new Set<string>([absolute(path)]);
 try{rows.add(realpathSync(path));}catch{}
 for(const row of[...rows]){rows.add(row.replace(/^\/private(?=\/(?:var|tmp|etc)(?:\/|$))/u,""));if(/^\/(?:var|tmp|etc)(?:\/|$)/u.test(row)&&process.platform==="darwin")rows.add("/private"+row);}
 return[...rows];
}

/** 🔗️ The spelling of `directory` that a hint inside it uses (a path the environment names, such as the manifest directory), when it differs from every spelling derivable from the directory itself, for instance through a link of the user's own. */
function spellingOfHint(directory:string,hint:string):string|undefined{
 try{
  const base=realpathSync(directory),inside=relative(base,realpathSync(hint));
  if(inside.startsWith("..")||isAbsolute(inside))return undefined;
  if(inside==="")return resolve(hint);
  const normal=absolute(hint),tail=inside.split("\\").join("/");
  return normal.split("\\").join("/").endsWith("/"+tail)?normal.slice(0,normal.length-tail.length-1):undefined;
 }catch{return undefined;}
}

/** 🧰️ The directories of the Rust toolchain a machine keeps outside the checkout: the Cargo home (registry and git checkouts), the rustup home and the compiler's sysroot when it is not inside the rustup home. */
export type ToolRootsV1=Readonly<{cargoHome?:string|undefined;rustupHome?:string|undefined;sysroot?:string|undefined}>;

const inside=(path:string,parent:string):boolean=>{const rows=spellings(parent).flatMap(row=>forms(row));return forms(path).some(form=>rows.some(row=>form===row||form.startsWith(row+"/")||form.startsWith(row+"\\")));};

/** 🧰️ The tool roots a machine names for a compilation: `CARGO_HOME` and `RUSTUP_HOME` of the environment, else the defaults below the user's home (`~/.cargo`, `~/.rustup`), and the sysroot when it lies outside the rustup home (inside it the rustup token covers it, so two machines never tokenise one path two ways). */
export function toolRootsV1(environment:Readonly<Record<string,string|undefined>>,sysroot?:string|undefined):ToolRootsV1{
 const home=environment.HOME||environment.USERPROFILE||homedir(),named=(value:string|undefined,fallback:string)=>value!==undefined&&value!==""?value:home?join(home,fallback):undefined;
 const cargoHome=named(environment.CARGO_HOME,".cargo"),rustupHome=named(environment.RUSTUP_HOME,".rustup");
 return Object.freeze({...cargoHome===undefined?{}:{cargoHome},...rustupHome===undefined?{}:{rustupHome},...sysroot!==undefined&&sysroot!==""&&(rustupHome===undefined||!inside(sysroot,rustupHome))?{sysroot}:{}});
}

/** 🧭️ The roots of one unit: the working directory Cargo gave the compiler (the workspace root for a member, the package directory for a registry crate), the workspace root when the launcher states it, and the tool roots (Cargo home, rustup home, sysroot), each in every spelling. A path inside several roots takes the token of the deepest one, so a Cargo home below the checkout is the Cargo home token on every machine, wherever its Cargo home is. Roots shorter than four characters are never replaced. */
export function portableRootsV1(input:Readonly<{cwd:string;workspace:string|undefined;hints?:readonly (string|undefined)[]}&ToolRootsV1>):readonly PortableRootV1[]{
 const hints=(input.hints??[]).filter((hint):hint is string=>typeof hint==="string"&&hint!==""),first=(paths:string[])=>paths.find(path=>hints.some(hint=>hint===path||hint.startsWith(path+"/")||hint.startsWith(path+"\\")))??paths[0]!;
 const make=(path:string,token:string):PortableRootV1[]=>{const all=spellings(path);for(const hint of hints){const extra=spellingOfHint(path,hint);if(extra!==undefined&&!all.includes(extra))all.push(extra);}const chosen=first(all);return all.map(row=>({path:row,token,...(row===chosen?{preferred:true}:{})}));};
 const rows:PortableRootV1[]=make(input.cwd,fleet.tokens.cwd);
 const add=(path:string|undefined,token:string)=>{if(path&&absolute(path)!==absolute(input.cwd))for(const row of make(path,token))if(!rows.some(other=>other.path===row.path))rows.push(row);};
 add(input.workspace,fleet.tokens.workspace);add(input.cargoHome,fleet.tokens.cargoHome);add(input.rustupHome,fleet.tokens.rustupHome);add(input.sysroot,fleet.tokens.sysroot);
 return rows.filter(row=>row.path.length>=4).sort((left,right)=>right.path.length-left.path.length);
}

/** ✂️ Replaces every root by its token (only where the root ends: `/a/bc` is not under `/a/b`), so equal compilations in different checkouts produce equal text. */
export function portableTextV1(text:string,roots:readonly PortableRootV1[]):string{
 if(roots.length===0)return text;
 const lookup=new Map<string,string>(),alternatives:string[]=[];
 for(const root of roots)for(const form of forms(root.path)){if(!lookup.has(form)){lookup.set(form,root.token);alternatives.push(escape(form));}}
 alternatives.sort((left,right)=>right.length-left.length);
 const replaced=text.replace(new RegExp("(?:"+alternatives.join("|")+")(?![A-Za-z0-9._-])","gu"),match=>lookup.get(match)!);
 const tokens=[...new Set(lookup.values())].map(escape).join("|");
 return replaced.includes("\\")?replaced.replace(new RegExp("("+tokens+")((?:\\\\[^\\\\/\\s\"'<>|*?:,;]+)+)","gu"),(_,token:string,rest:string)=>token+rest.replaceAll("\\","/")):replaced;
}

/** 🏠️ Puts the roots of this machine back where the tokens stand. */
export function localTextV1(text:string,roots:readonly PortableRootV1[]):string{
 let result=text;
 for(const token of new Set(roots.map(root=>root.token))){const chosen=roots.find(root=>root.token===token&&root.preferred===true)??roots.find(root=>root.token===token)!;result=result.split(token).join(chosen.path);}
 return result;
}

/** 🔑️ The machine-independent first-level key of a unit: its inputs with every checkout root replaced by its token. The same on every machine, whoever compiles. */
export function portableKeyV1(input:Parameters<typeof unitKeyV1>[0],roots:readonly PortableRootV1[]):string{
 const source=(argument:string)=>argument.endsWith(".rs")&&!argument.startsWith("-")?argument.replaceAll("\\","/"):argument;
 return unitKeyV1({...input,normalized:input.normalized.map(argument=>source(portableTextV1(argument,roots))),environment:input.environment.map(([name,value])=>[name,portableTextV1(value,roots)] as [string,string])});
}

/** 🆔️ The `-C metadata` of one unit from its slot-level identity: the compiler, the portable command line (its extern names, never their bytes), the slot parts (target, profile, package name and version, member path, crate, kinds, features, cfg) and nothing of its content (sources, extern digests, environment values). Equal in every workspace and across edits, so the stable crate id and the symbol hashes survive a body edit and the early cutoff of metadata-only builds keeps working; two units that differ in package, crate, kind, features, flags, target or compiler never share one. A unit without slot parts is identified by its whole portable environment instead. */
export function slotMetadataV1(input:Parameters<typeof unitKeyV1>[0],roots:readonly PortableRootV1[],slotParts:readonly string[]|undefined):string{
 const source=(argument:string)=>argument.endsWith(".rs")&&!argument.startsWith("-")?argument.replaceAll("\\","/"):argument;
 const normalized=input.normalized.map((argument,index)=>{const text=source(portableTextV1(argument,roots));return input.normalized[index-1]==="--extern"?text.replace(/=#.*$/su,""):text;});
 const identity=slotParts===undefined?input.environment.map(([name,value])=>[name,portableTextV1(value,roots)]):slotParts.slice(1);
 return canonicalMetadataV1(JSON.stringify([input.toolchain,normalized,identity]));
}

/** 🔑️ The key of one unit in the fleet cache: the namespace, the platform and the unit's portable key. A unit whose compiled bytes contain a real path of its machine (`sensitive`: the paths, see `pathSensitiveV1`) is keyed on those paths as well, so two machines share it only when the paths are equal. */
export function remoteKeyV1(platform:PlatformV1,input:Parameters<typeof unitKeyV1>[0],roots:readonly PortableRootV1[],sensitive:readonly string[]=[]):string{
 return sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,fleet.namespace,platform,portableKeyV1(input,roots),...sensitive.length===0?[]:[["real-paths",...sensitive]]]));
}

/** 🏠️ The local first-level key of a unit: the portable key, so one crate under another checkout root, task view or machine is the same local entry; a unit whose bytes name a real path of this machine (`sensitive`) is keyed on those paths as well and shares only with equal paths. */
export function localKeyV1(input:Parameters<typeof unitKeyV1>[0],roots:readonly PortableRootV1[],sensitive:readonly string[]=[]):string{
 const portable=portableKeyV1(input,roots);
 return sensitive.length===0?portable:sha256V1(JSON.stringify([portable,["real-paths",...sensitive]]));
}

/** 🪢️ A slot id exactly as package C specified it (`🏗️build/🪢️slots`); the test compares both implementations. */
export function slotIdV1(parts:readonly string[]):string{return createHash("sha256").update(JSON.stringify(["semio-slot-v1","rust-unit",...parts])).digest("hex");}

function codegen(args:readonly string[]):Map<string,string>{
 const rows=new Map<string,string>();
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,value=argument==="-C"||argument==="--codegen"?args[++index]:/^-C./u.test(argument)?argument.slice(2):undefined;
  if(value===undefined)continue;const equals=value.indexOf("=");rows.set(equals<0?value:value.slice(0,equals),equals<0?"":value.slice(equals+1));
 }
 return rows;
}
function many(args:readonly string[],flag:string):string[]{
 const rows:string[]=[];
 for(let index=0;index<args.length;index++){const argument=args[index]!;if(argument===flag)rows.push(args[++index]??"");else if(argument.startsWith(flag+"="))rows.push(argument.slice(flag.length+1));}
 return rows;
}

/** 🪢️ The slot parts of one unit: its identity without any content input (toolchain, target, profile, package, crate, kind, features and cfg). */
export function slotPartsV1(input:Readonly<{release:CompilerReleaseV1;unit:RustcUnitV1;args:readonly string[];environment:Environment;cwd:string}>):string[]{
 const flags=codegen(input.args),cfgs=many(input.args,"--cfg"),features=cfgs.filter(value=>value.startsWith("feature=")).map(value=>value.slice("feature=".length).replace(/^"|"$/gu,"")).sort(),others=cfgs.filter(value=>!value.startsWith("feature=")).sort();
 const manifest=input.environment.CARGO_MANIFEST_DIR,inside=manifest!==undefined&&manifest!==input.cwd&&(isAbsolute(manifest)||windowsPath(manifest))?(windowsPath(manifest)&&windowsPath(input.cwd)?win32.relative(input.cwd,manifest):relative(input.cwd,manifest)):"",member=inside!==""&&!inside.startsWith("..")&&!isAbsolute(inside)&&!windowsPath(inside)?"#"+inside.split("\\").join("/"):"";
 const profile=["opt="+(flags.get("opt-level")??""),"debug="+(flags.get("debuginfo")??""),"da="+(flags.get("debug-assertions")??""),"oc="+(flags.get("overflow-checks")??""),"lto="+(flags.get("lto")??""),"panic="+(flags.get("panic")??""),"cgu="+(flags.get("codegen-units")??""),"inc="+(input.unit.incremental===undefined?"0":"1")].join(";");
 const kinds=[...(input.unit.crateTypes.length?input.unit.crateTypes:["bin"])].sort().join(",")+"/"+[...new Set(input.unit.emit)].sort().join(",");
 return[input.release.digest,input.unit.target??input.release.host,profile,(input.environment.CARGO_PKG_NAME??"")+"@"+(input.environment.CARGO_PKG_VERSION??"")+member,input.unit.crateName,kinds,features.join(","),others.join(",")];
}
