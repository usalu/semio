import {existsSync,readFileSync} from "node:fs";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {parseDepInfoV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {OFFLOAD_POLICY_V1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
import {dependencyClosureV1,walkFilesV1} from "../🌳️inputs/🟦️.ts";
import {installNameArgumentsV1,remappedNameV1,remapsOfV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";

const policy=OFFLOAD_POLICY_V1,virtual=policy.virtual;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🗺️ One directory the other machine sees under a virtual name. */
export type OffloadMappingV1=Readonly<{real:string;virtual:string}>;
/** 🚫️ A unit that stays here, and the detail that says why. */
export type OffloadDeclineV1=Readonly<{declined:OffloadReasonV1;detail:string}>;
/** 📦️ Everything an offloaded compilation needs, path-independent: the compiler arguments and environment with every requester path replaced by a virtual one, the input files by virtual path, and the mappings that translate the other machine's answer back. */
export type OffloadPlanV1=Readonly<{arguments:readonly string[];environment:Readonly<Record<string,string>>;files:ReadonlyMap<string,string>;mappings:readonly OffloadMappingV1[];outDir:string;crateName:string;target:string|undefined;extraFilename:string}>;

const MINIMUM_SHIPPED_DEPTH=3;
const depthOf=(path:string):number=>path.split(/[\\/]/u).filter(part=>part!=="").length;
const SYSTEM=["/usr/","/bin/","/sbin/","/Library/","/System/","/Applications/","/opt/","/dev/"];
const within=(parent:string,path:string):boolean=>path===parent||path.startsWith(parent.endsWith(sep)?parent:parent+sep);
const commonAncestor=(left:string,right:string):string=>{let at=left;while(!within(at,right)){const up=dirname(at);if(up===at)return at;at=up;}return at;};
const INLINE_MODULES=/^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\{/gmu;
const INCLUDES=/#\s*\[\s*path\s*=\s*"([^"\n]{1,512})"\s*\]|\bcfg_attr\s*\([^\]"]*(?:"[^"\n]*"[^\]"]*)*,\s*path\s*=\s*"([^"\n]{1,512})"|include(?:_str|_bytes)?!\s*\(\s*"([^"\n]{1,512})"|include(?:_str|_bytes)?!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"\n]{1,512})"/gu,MODULES=/^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*;/gmu;
/** 🔗️ Every file that rustc reads for the root source, found without compiling: the files of `mod name;` declarations (`name.rs` or `name/mod.rs` below the directory the declaring file owns), the targets of `#[path = "..."]` and of the `path = "..."` of a `#[cfg_attr(..., path = "...")]` (their children live beside them) and the literal paths of `include!`, `include_str!` and `include_bytes!` (relative to the file that names them), transitively through Rust files, existing files only, bounded; `undefined` beyond the bound. The module layout of this repository keeps the code of a crate in the module directory above its manifest and includes it from a one-line shim, so a directory walk would ship whole module trees (tests, generators, other crates) for a few source files. */
export function reachedFilesV1(entry:string,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}},maximumFiles:number=512,manifest?:string):readonly string[]|undefined{
 const seen=new Set<string>([entry]),queue:{file:string;children:string}[]=[{file:entry,children:dirname(entry)}],found:string[]=[];
 const take=(target:string,children:string):void=>{if(seen.has(target)||!existsSync(target))return;seen.add(target);found.push(target);if(target.endsWith(".rs"))queue.push({file:target,children});};
 while(queue.length>0){
  if(seen.size>maximumFiles)return undefined;
  const {file,children}=queue.shift()!,text=read(file);
  if(text===undefined)continue;
  for(const match of text.matchAll(INCLUDES)){
   if(match[4]!==undefined){if(manifest!==undefined){const target=resolve(manifest,"."+(match[4].startsWith("/")?"":"/")+match[4]);take(target,dirname(target));}continue;}
   const target=resolve(dirname(file),match[1]??match[2]??match[3]!);take(target,dirname(target));
  }
  const inline=[...text.matchAll(INLINE_MODULES)].map(match=>match[1]!);
  for(const match of text.matchAll(MODULES)){
   const homes=[children,...inline.map(name=>join(children,name))];
   for(const home of homes){const flat=join(home,match[1]!+".rs"),nested=join(home,match[1]!,"mod.rs");if(existsSync(flat)){take(flat,join(home,match[1]!));break;}if(existsSync(nested)){take(nested,join(home,match[1]!));break;}}
  }
 }
 return found;
}
/** 🔗️ The files that the root source reaches beyond itself (see `reachedFilesV1`); the empty list beyond the bound. */
export function includedSourcesV1(entry:string,read?:(path:string)=>string|undefined,maximumFiles?:number,manifest?:string):readonly string[]{return reachedFilesV1(entry,read,maximumFiles,manifest)??[];}
const deniedName=(name:string):boolean=>policy.environment.deniedNames.includes(name)||policy.environment.deniedPrefixes.some(prefix=>name.startsWith(prefix));
const carriable=(name:string,value:string):boolean=>/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(name)&&(name.startsWith("CARGO_PKG_")?!value.includes("\0")&&value.length<=65536:!/[\0\r\n]/u.test(value)&&value.length<=16384);
const decline=(declined:OffloadReasonV1,detail:string):OffloadDeclineV1=>({declined,detail});

/** 🪞️ Translates text the other machine produced (compiler messages, dependency files) back into the requester's paths: the other machine's root becomes the requester's working directory and every virtual directory its real one. One pass, longest names first, so `d1` never reads as `d`. */
export function offloadRealTextV1(text:string,mappings:readonly OffloadMappingV1[],root:string|undefined,cwd:string):string{
 const forms=new Map<string,string>();
 for(const row of mappings){forms.set(row.virtual,row.real);if(root!==undefined)forms.set(join(root,row.virtual),row.real);}
 if(root!==undefined){forms.set(root,cwd);forms.set("/semio/root",cwd);}
 const names=[...forms.keys()].sort((left,right)=>right.length-left.length);
 if(names.length===0)return text;
 const pattern=new RegExp("(?:"+names.map(name=>name.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|")+")(?![A-Za-z0-9_.-])","gu");
 return text.replace(pattern,found=>forms.get(found)!);
}

/** 🧭️ The compiler arguments of a unit that Cargo compiles in its package directory while its code lives in the module directory above the manifest (a one-line shim includes it), when no remap covers that directory: the directory gets a remap of its own, named by its path below the checkout, placed before Cargo's remaps (the last matching one wins, so the package directory keeps Cargo's name), and a relative source becomes absolute so that every name the compiler records is remapped. A local compile and an offloaded one then record the same names and give the same crate hash on every machine and at every checkout path. `undefined` when the unit needs nothing. */
export function moduleRemapArgumentsV1(args:readonly string[],environment:Environment,cwd:string,workspace:string|undefined):readonly string[]|undefined{
 const manifest=environment.CARGO_MANIFEST_DIR,pass=scan(args);
 if(workspace===undefined||manifest===undefined||!isAbsolute(manifest)||"declined" in pass||pass.source===undefined||!within(manifest,cwd))return undefined;
 const sourceReal=resolve(cwd,pass.source),reached=reachedFilesV1(sourceReal,undefined,undefined,manifest);
 if(reached===undefined)return undefined;
 const shipped=[sourceReal,...reached].reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest);
 if(within(cwd,shipped)||shipped===manifest||depthOf(shipped)<MINIMUM_SHIPPED_DEPTH||!within(workspace,shipped)||workspace===shipped||remappedNameV1(remapsOfV1(args),shipped)!==shipped)return undefined;
 const first=args.findIndex(argument=>argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")),rewritten=args.map(argument=>argument===pass.source&&!isAbsolute(argument)?sourceReal:argument),at=first<0?rewritten.length:first;
 return Object.freeze([...rewritten.slice(0,at),`--remap-path-prefix=${shipped}=${relative(workspace,shipped).split(sep).join("/")}`,...rewritten.slice(at)]);
}

type Pass=Readonly<{outDir:string|undefined;depsDirs:readonly string[];source:string|undefined;extra:string;crate:string;target:string|undefined}>;
function scan(args:readonly string[]):Pass|OffloadDeclineV1{
 let outDir:string|undefined,source:string|undefined,extra="",crate="",target:string|undefined;const depsDirs:string[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,take=(flag:string):string|undefined=>argument===flag?args[++index]??"":argument.startsWith(flag+"=")?argument.slice(flag.length+1):undefined;
  let value:string|undefined;
  if((value=take("--out-dir"))!==undefined){outDir=value;continue;}
  if((value=take("--crate-name"))!==undefined){crate=value;continue;}
  if((value=take("--target"))!==undefined){if(value.endsWith(".json"))return decline("unmapped-path","--target "+value);target=value;continue;}
  if(argument==="-L"||argument.startsWith("-L")){
   const text=argument==="-L"?args[++index]??"":argument.slice(2);
   if(!text.startsWith("dependency="))return decline("not-portable","-L "+text.split("=")[0]);
   depsDirs.push(text.slice("dependency=".length));continue;
  }
  if(argument==="-l"||argument.startsWith("-l"))return decline("not-portable","-l");
  if(argument==="-o"||argument.startsWith("--sysroot")||argument==="--emit-dep-info")return decline("unmapped-path",argument);
  if((value=take("--emit"))!==undefined){if(value.includes("="))return decline("unmapped-path","--emit "+value);continue;}
  const codegen=argument==="-C"?args[index+1]??"":argument.startsWith("-C")?argument.slice(2):undefined;
  if(codegen!==undefined){if(codegen.startsWith("extra-filename="))extra=codegen.slice("extra-filename=".length);if(argument==="-C")index++;continue;}
  if(argument.endsWith(".rs")&&!argument.startsWith("-"))source=argument;
 }
 return{outDir,depsDirs,source,extra,crate,target};
}

/** 📐️ Plans one offloaded compilation from the compiler arguments (response files already expanded), the environment of the call and its working directory; a unit that cannot be made path-independent is declined with the reason, never offloaded half-mapped. */
export function planOffloadV1(input:Readonly<{args:readonly string[];environment:Environment;cwd:string;before?:(plan:Pass)=>void}>):OffloadPlanV1|OffloadDeclineV1{
 const {args,environment,cwd}=input,pass=scan(args);
 if("declined" in pass)return pass;
 if(pass.outDir===undefined||pass.source===undefined)return decline("not-portable","out-dir or source");
 const manifest=environment.CARGO_MANIFEST_DIR,remaps=remapsOfV1(args),nameOf=(path:string):string=>remappedNameV1(remaps,path);
 if(manifest===undefined||!isAbsolute(manifest))return decline("unmapped-path","CARGO_MANIFEST_DIR");
 const sourceReal=resolve(cwd,pass.source),reached=[sourceReal,...includedSourcesV1(sourceReal,undefined,undefined,manifest)];
 const shipped=reached.reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest),anchored=within(cwd,shipped);
 if(!anchored&&shipped!==manifest&&(depthOf(shipped)<MINIMUM_SHIPPED_DEPTH||reachedFilesV1(sourceReal,undefined,undefined,manifest)===undefined))return decline("unmapped-path","source outside CARGO_MANIFEST_DIR: working directory "+cwd+", manifest "+manifest+", sources up to "+shipped);
 const packageVirtual=anchored?relative(cwd,manifest).split(sep).join("/"):shipped===manifest?virtual.package:virtual.package+"/"+relative(shipped,manifest).split(sep).join("/"),shippedVirtual=anchored?relative(cwd,shipped).split(sep).join("/"):virtual.package,files=new Map<string,string>(),mappings:OffloadMappingV1[]=[];
 const ignored=["target",".git","node_modules",".cache",virtual.base];
 const reachedAll=reachedFilesV1(sourceReal,undefined,undefined,manifest),closure=new Map<string,string>();
 if(reachedAll!==undefined&&[sourceReal,...reachedAll].every(path=>within(shipped,path))){
  for(const path of [sourceReal,...reachedAll,join(manifest,"Cargo.toml")])if(existsSync(path))closure.set(relative(shipped,path).split(sep).join("/"),path);
 }
 const walked=closure.size>0?closure:walkFilesV1(shipped,ignored);
 if(walked===undefined)return decline("inputs-too-large","package "+shipped);
 const place=(path:string,real:string):void=>{files.set(path,real);};
 for(const [path,real] of walked)place(shippedVirtual===""?path:shippedVirtual+"/"+path,real);
 if(!anchored)mappings.push({real:shipped,virtual:virtual.package});
 const cwdName=nameOf(cwd);
 if(!anchored&&shipped!==manifest&&nameOf(shipped)===shipped)return decline("unmapped-path","no --remap-path-prefix covers the module directory "+shipped+": the compiler would record this machine's paths");
 if(cwdName===cwd)return decline("unmapped-path","no --remap-path-prefix covers the working directory: the compiler would record this machine's paths");
 const outDirReal=resolve(cwd,pass.outDir),dependencies=[...new Set(pass.depsDirs.map(directory=>resolve(cwd,directory)))],directoryNames=new Map<string,string>();
 dependencies.forEach((directory,index)=>{directoryNames.set(directory,index===0?virtual.deps:virtual.deps+index);});
 let external=0;
 const virtualOf=(directory:string):string=>{let name=directoryNames.get(directory);if(name===undefined){name=virtual.external+external++;directoryNames.set(directory,name);}return name;};
 const rewritten:string[]=[],direct=new Map<string,string[]>(),inside:string[]=[];let sourceName:string|undefined;
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,take=(flag:string):string|undefined=>argument===flag?args[++index]??"":argument.startsWith(flag+"=")?argument.slice(flag.length+1):undefined,joined=argument.startsWith("--")&&argument.includes("=");
  let value:string|undefined;
  if((value=take("--out-dir"))!==undefined){rewritten.push(joined?"--out-dir="+virtual.out:"--out-dir",...joined?[]:[virtual.out]);continue;}
  if(argument==="-L"||argument.startsWith("-L")){
   const text=argument==="-L"?args[++index]!:argument.slice(2),mapped="dependency="+directoryNames.get(resolve(cwd,text.slice("dependency=".length)))!;
   if(argument==="-L")rewritten.push("-L",mapped);else rewritten.push("-L"+mapped);continue;
  }
  if((value=take("--extern"))!==undefined){
   const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1);
   if(path===""){rewritten.push(...joined?["--extern="+spec]:["--extern",spec]);continue;}
   const real=resolve(cwd,path),directory=dirname(real),name=virtualOf(directory),mapped=spec+"="+name+"/"+basename(real);
   if(!existsSync(real))return decline("unreadable-input","extern "+path);
   const sibling=real.replace(/\.(?:rlib|rmeta)$/u,real.endsWith(".rlib")?".rmeta":".rlib");direct.set(directory,[...direct.get(directory)??[],real,...sibling!==real&&existsSync(sibling)?[sibling]:[]]);rewritten.push(...joined?["--extern="+mapped]:["--extern",mapped]);continue;
  }
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental="))continue;
  if(argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")){
   const joinedRemap=argument.includes("="),spec=joinedRemap?argument.slice(argument.indexOf("=")+1):args[++index]??"",from=spec.slice(0,Math.max(0,spec.lastIndexOf("=")));
   if(isAbsolute(from)&&!SYSTEM.some(prefix=>from.startsWith(prefix))){if(!anchored&&within(shipped,from)){const rest=relative(shipped,from).split(sep).join("/");inside.push(`${virtual.package}${rest===""?"":"/"+rest}=${spec.slice(from.length+1)}`);}continue;}
   rewritten.push(...joinedRemap?[argument]:[argument,spec]);continue;
  }
  if(argument===pass.source){const shown=anchored?relative(cwd,sourceReal).split(sep).join("/"):virtual.package+"/"+relative(shipped,sourceReal).split(sep).join("/");if(isAbsolute(pass.source))sourceName=shown;else rewritten.push(shown);continue;}
  rewritten.push(argument);
 }
 const names:string[]=[];
 if(!anchored)names.push(`${virtual.package}=${nameOf(shipped)}`,...inside);
 const dependencyInputs=dependencyClosureV1([...directoryNames.keys()],[...direct.values()].flat());
 if(dependencyInputs===undefined)return decline("inputs-too-large","dependency closure");
 for(const [directory,name] of directoryNames){
  mappings.push({real:directory,virtual:name});
  for(const file of dependencyInputs.get(directory)??[])place(name+"/"+file,join(directory,file));
  for(const real of direct.get(directory)??[])place(name+"/"+basename(real),real);
 }
 mappings.push({real:pass.outDir,virtual:virtual.out});
 const absolute:string[]=[],environmentOut:Record<string,string>={};
 for(const [name,value] of Object.entries(environment)){
  if(value===undefined||deniedName(name))continue;
  if(!carriable(name,value))continue;
  if(value.includes(cwd))return decline("unmapped-path","environment "+name);
  environmentOut[name]=value;
 }
 const generated=environment.OUT_DIR;
 if(generated!==undefined){
  for(const [name,value] of Object.entries(environmentOut)){
   if(name==="OUT_DIR"||!value.includes(generated))continue;
   if(value!==generated&&!value.startsWith(generated+sep))return decline("unmapped-path","environment "+name);
   absolute.push(name);environmentOut["SEMIO_REL_"+name]=virtual.generated+value.slice(generated.length).split(sep).join("/");
  }
  const walkedOut=walkFilesV1(generated,[]);
  if(walkedOut===undefined)return decline("inputs-too-large","OUT_DIR");
  for(const [path,real] of walkedOut)place(virtual.generated+"/"+path,real);
  mappings.push({real:generated,virtual:virtual.generated});names.push(`${virtual.generated}=${nameOf(generated)}`);
  absolute.push("OUT_DIR");environmentOut.SEMIO_REL_OUT_DIR=virtual.generated;
 }
 absolute.push("CARGO_MANIFEST_DIR");environmentOut.SEMIO_REL_CARGO_MANIFEST_DIR=packageVirtual===""?".":packageVirtual;
 const manifestPath=environment.CARGO_MANIFEST_PATH;
 if(manifestPath!==undefined){
  if(manifestPath!==join(manifest,"Cargo.toml"))return decline("unmapped-path","CARGO_MANIFEST_PATH");
  absolute.push("CARGO_MANIFEST_PATH");environmentOut.SEMIO_REL_CARGO_MANIFEST_PATH=(packageVirtual===""?"":packageVirtual+"/")+"Cargo.toml";
 }
 environmentOut.SEMIO_CWD_NAME=cwdName;if(sourceName!==undefined)environmentOut.SEMIO_SOURCE=sourceName;environmentOut.SEMIO_REMAPS=names.join("\n");
 environmentOut.SEMIO_ABSOLUTE=absolute.join(" ");environmentOut.PATH=policy.environment.path;
 const infoName=join(outDirReal,pass.crate+pass.extra+".d");
 if(existsSync(infoName)){
  try{
   for(const dependency of parseDepInfoV1(readFileSync(infoName,"utf8")).dependencies){
    const real=resolve(cwd,dependency);
    if(within(shipped,real))continue;
    if(!within(cwd,real))return decline("unmapped-path","dependency "+dependency);
    const path=relative(cwd,real).split(sep).join("/");
    if(!files.has(path)&&existsSync(real))place(path,real);
   }
  }catch{return decline("unreadable-input","dependency file");}
 }
 for(const argument of rewritten){
  if(argument.includes(cwd))return decline("unmapped-path",argument.slice(0,80));
  const value=argument.includes("=")?argument.slice(argument.indexOf("=")+1):argument;
  if(isAbsolute(value)&&!SYSTEM.some(prefix=>value.startsWith(prefix)))return decline("unmapped-path",argument.slice(0,80));
 }
 const finalArguments=[...rewritten,...installNameArgumentsV1(args)];
 return Object.freeze({arguments:Object.freeze(finalArguments),environment:Object.freeze(environmentOut),files,mappings:Object.freeze(mappings),outDir:pass.outDir,crateName:pass.crate,target:pass.target,extraFilename:pass.extra});
}
