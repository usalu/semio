import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {parseDepInfoV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadCrateTypesOfV1,offloadLinkArgumentV1,offloadLinkLibraryRefusalV1,offloadSystemDirectoryV1,offloadWasmLinkArgumentV1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
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

const MINIMUM_SHIPPED_DEPTH=3,REACHED_FILES=policy.limits.maximumInputFiles;
const depthOf=(path:string):number=>path.split(/[\\/]/u).filter(part=>part!=="").length;
const SYSTEM=["/usr/","/bin/","/sbin/","/Library/","/System/","/Applications/","/opt/","/dev/"];
const within=(parent:string,path:string):boolean=>path===parent||path.startsWith(parent.endsWith(sep)?parent:parent+sep);
const commonAncestor=(left:string,right:string):string=>{let at=left;while(!within(at,right)){const up=dirname(at);if(up===at)return at;at=up;}return at;};
const INLINE_MODULES=/^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\{/gmu;
const INCLUDES=/#\s*\[\s*path\s*=\s*"([^"\n]{1,512})"\s*\]|\bcfg_attr\s*\([^\]"]*(?:"[^"\n]*"[^\]"]*)*,\s*path\s*=\s*"([^"\n]{1,512})"|include(?:_[A-Za-z0-9_]+)?!\s*\(\s*(?:r(#*)"([^"\n]{1,512})"\3|"([^"\n]{1,512})")|include(?:_[A-Za-z0-9_]+)?!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"\n]{1,512})"/gu,BINDINGS=/\b(?:bindgen|generate|from_witx|wasmtime_integration)!\s*\(\s*\{[\s\S]{0,8192}?\b(?:path|witx)\s*:\s*(\[[^\]]{0,2048}\]|"[^"\n]{1,512}")/gu,QUOTED=/"([^"\n]{1,512})"/gu,MODULES=/^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*;/gmu;
/** 🔗️ Every file that rustc reads for the root source, found without compiling: the files of `mod name;` declarations (`name.rs` or `name/mod.rs` below the directory the declaring file owns), the targets of `#[path = "..."]` and of the `path = "..."` of a `#[cfg_attr(..., path = "...")]` (their children live beside them) and the literal paths of `include!` and of any `include_<name>!` (`include_str!`, `include_bytes!`, `include_wgsl!`, `include_spirv!`, `include_glsl!`, including a `macro_rules!` of that shape), as a plain `"..."`, a raw `r#*"..."#*`, or `concat!(env!("CARGO_MANIFEST_DIR"), "...")` (the last relative to the manifest), the `path` of a `bindgen!` or `generate!` WIT binding macro and the `witx` of a `from_witx!` or `wasmtime_integration!` one (files or directories of files, relative to the manifest; the directory of a file whose `use` declarations name its siblings travels with it), transitively through Rust files, existing files only, bounded; `undefined` beyond the bound. Only `.rs` targets are queued. The module layout of this repository keeps the code of a crate in the module directory above its manifest and includes it from a one-line shim, so a directory walk would ship whole module trees (tests, generators, other crates) for a few source files. */
export function reachedFilesV1(entry:string,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}},maximumFiles:number=512,manifest?:string):readonly string[]|undefined{
 const seen=new Set<string>([entry]),queue:{file:string;children:string;outer:readonly string[]}[]=[{file:entry,children:dirname(entry),outer:[]}],found:string[]=[];
 const take=(target:string,children:string,outer:readonly string[]):void=>{if(seen.has(target)||!existsSync(target))return;seen.add(target);if(statSync(target).isDirectory()){for(const name of readdirSync(target).sort())if(seen.size<=maximumFiles)take(join(target,name),children,outer);return;}found.push(target);if(target.endsWith(".rs"))queue.push({file:target,children,outer});else if(target.endsWith(".witx"))for(const name of readdirSync(dirname(target)).sort())take(join(dirname(target),name),children,outer);};
 while(queue.length>0){
  if(seen.size>maximumFiles)return undefined;
  const {file,children,outer}=queue.shift()!,text=read(file),owned=[children,...outer];
  if(text===undefined)continue;
  for(const match of text.matchAll(INCLUDES)){
   if(match[6]!==undefined){if(manifest!==undefined){const target=resolve(manifest,"."+(match[6].startsWith("/")?"":"/")+match[6]);take(target,dirname(target),owned);}continue;}
   const literal=match[1]??match[2]??match[4]??match[5];if(literal===undefined)continue;
   const target=resolve(dirname(file),literal);take(target,dirname(target),owned);
  }
  if(manifest!==undefined)for(const match of text.matchAll(BINDINGS))for(const quoted of match[1]!.matchAll(QUOTED)){const target=resolve(manifest,"."+(quoted[1]!.startsWith("/")?"":"/")+quoted[1]!);take(target,dirname(target),owned);}
  const inline=[...text.matchAll(INLINE_MODULES)].map(match=>match[1]!);
  for(const match of text.matchAll(MODULES)){
   const homes=owned.flatMap(home=>[home,...inline.map(name=>join(home,name))]);
   for(const home of homes){const flat=join(home,match[1]!+".rs"),nested=join(home,match[1]!,"mod.rs");if(existsSync(flat)){take(flat,join(home,match[1]!),owned);break;}if(existsSync(nested)){take(nested,join(home,match[1]!),owned);break;}}
  }
 }
 return found;
}
/** 🪜️ How many directories above `shipped` a textual path of the reached sources climbs before it descends again (`#[path = "../../../module/lib.rs"]` of a shim climbs out of the module directory and back into it by name); the virtual root keeps that many real directory names so that such a path resolves on the other machine, where rustc reads the text, not the real file system. */
export function climbAboveV1(entry:string,reached:readonly string[],shipped:string,manifest?:string,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):number{
 let climb=0;
 const measure=(base:string,literal:string):void=>{let depth=relative(shipped,base).split(sep).filter(part=>part!=="").length;for(const part of literal.split("/")){if(part===".."){depth--;climb=Math.max(climb,-depth);}else if(part!==""&&part!==".")depth++;}};
 for(const file of [entry,...reached]){
  const text=file.endsWith(".rs")?read(file):undefined;
  if(text===undefined)continue;
  for(const match of text.matchAll(INCLUDES)){const literal=match[1]??match[2]??match[4]??match[5];if(literal!==undefined)measure(dirname(file),literal);else if(match[6]!==undefined&&manifest!==undefined)measure(manifest,match[6]);}
  if(manifest!==undefined)for(const match of text.matchAll(BINDINGS))for(const quoted of match[1]!.matchAll(QUOTED))measure(manifest,quoted[1]!);
 }
 return climb;
}
const DERIVES=/(?<![A-Za-z0-9_])derive\s*\(([^()]*)\)/gu,WORKSPACE_DERIVES=/(?<![A-Za-z0-9_])(?:MutationLeaf|Mutations)(?![A-Za-z0-9_])/u;
/** 🔖️ The workspace file that the mutation derive names itself in, the file name of its generated authority projection (`MUTATION_AUTHORITY_LOCATOR` of `🔖️mutation-source-authority`, checked against that source by a test). */
export const MUTATION_AUTHORITY_FILE_V1="🔣️mutation-authority.json";
/** 🔖️ The files of `files` that apply the mutation derive (`#[derive(MutationLeaf)]`, `#[derive(Mutations)]`, also inside a `cfg_attr`): the derive finds its workspace by the `package.json` and `📋️project.json` markers above each of them and reads the generated authority projection, the schema index, the leaf descriptors and the schema documents by workspace-relative locators; its output names only workspace-relative paths and the digest of the authority, so another root gives the same bytes. Empty when no file applies it. */
export function workspaceDeriveFilesV1(files:Iterable<string>,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):readonly string[]{
 const found:string[]=[];
 for(const file of files){
  const text=file.endsWith(".rs")?read(file):undefined;
  if(text===undefined)continue;
  for(const match of text.matchAll(DERIVES))if(WORKSPACE_DERIVES.test(match[1]!)){found.push(file);break;}
 }
 return found;
}
/** 🧭️ The workspace root the derive finds for `file`: the nearest ancestor whose `package.json` declares `workspaces` and that holds a `📋️project.json`, following the exact-ancestor delegation `semio.mutationSourceAuthority.workspaceRoot` (`..` segments) of a nested one; `undefined` when there is none. Its location is the one the unit must find again in the sandbox, because the derive names every path relative to it. */
export function deriveWorkspaceRootV1(file:string,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):string|undefined{
 const anchor=(at:string):string|null|undefined=>{
  const text=read(join(at,"package.json"));
  if(text===undefined)return null;
  let json:{workspaces?:unknown;semio?:{mutationSourceAuthority?:{workspaceRoot?:unknown}}}|null;
  try{json=JSON.parse(text);}catch{return undefined;}
  if(typeof json!=="object"||json===null||typeof json.workspaces!=="object"||json.workspaces===null)return null;
  const delegation=json.semio?.mutationSourceAuthority?.workspaceRoot;
  if(delegation!==undefined){
   if(typeof delegation!=="string"||delegation==="")return undefined;
   let target=at;for(const part of delegation.split("/")){if(part!=="..")return undefined;target=dirname(target);}
   const found=anchor(target);return found===null?undefined:found;
  }
  return read(join(at,"📋️project.json"))!==undefined?at:undefined;
 };
 for(let at=dirname(file);;at=dirname(at)){
  const found=anchor(at);
  if(found!==null)return found;
  if(dirname(at)===at)return undefined;
 }
}
/** 🧭️ The workspace markers the derive looks for on the way from `file` up to `root` (both included): the `package.json` and `📋️project.json` that exist there, whose content is deliberately not a compile input (an edit of either never invalidates a derive user). */
export function deriveMarkersV1(file:string,root:string):readonly string[]{
 const found:string[]=[];
 for(let at=dirname(file);within(root,at);at=dirname(at)){
  for(const name of["package.json","📋️project.json"]){const marker=join(at,name);if(existsSync(marker))found.push(marker);}
  if(at===root||dirname(at)===at)break;
 }
 return found;
}
/** 🔗️ The files that the root source reaches beyond itself (see `reachedFilesV1`); the empty list beyond the bound. */
export function includedSourcesV1(entry:string,read?:(path:string)=>string|undefined,maximumFiles?:number,manifest?:string):readonly string[]{return reachedFilesV1(entry,read,maximumFiles,manifest)??[];}
const deniedName=(name:string):boolean=>policy.environment.deniedNames.includes(name)||policy.environment.deniedPrefixes.some(prefix=>name.startsWith(prefix));
const carriable=(name:string,value:string):boolean=>/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(name)&&(name.startsWith("CARGO_PKG_")?!value.includes("\0")&&value.length<=65536:!/[\0\r\n]/u.test(value)&&value.length<=16384);
const decline=(declined:OffloadReasonV1,detail:string):OffloadDeclineV1=>({declined,detail});
const SEMIO_READS=/\b(?:option_)?env!\s*\(\s*"(SEMIO_[A-Za-z0-9_]*)"/gu;
/** 🔎️ The first `SEMIO_*` variable that the shipped sources read at compile time (`env!` or `option_env!`) and that the requester has set: the worker never receives a `SEMIO_*` name, so such a unit would compile differently there; it stays here instead of diverging silently. `undefined` when no shipped source reads a set one. */
export function semioEnvironmentReadV1(files:Iterable<string>,environment:Environment,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):string|undefined{
 for(const file of files){
  if(!file.endsWith(".rs"))continue;
  const text=read(file);
  if(text===undefined||!text.includes("SEMIO_"))continue;
  for(const match of text.matchAll(SEMIO_READS))if(environment[match[1]!]!==undefined)return match[1]!;
 }
 return undefined;
}

const READ_NAMES=/\b(?:option_)?env!\s*\(\s*"([A-Za-z_][A-Za-z0-9_]{0,255})"/gu,CARRIED_NAMES=new Set(["OUT_DIR","RUST_MIN_STACK","SDKROOT","MACOSX_DEPLOYMENT_TARGET","IPHONEOS_DEPLOYMENT_TARGET","RUSTC_BOOTSTRAP"]),CARRIED_PREFIXES=["CARGO_","RUSTC_","DEP_"];
/** 🌱️ The environment names a compilation can read: the names of `env!` and `option_env!` in the shipped Rust sources, those Cargo and the compiler hand every unit (`CARGO_*`, `RUSTC_*`, `OUT_DIR`, the SDK and deployment targets). Every other variable of the requester (`NATIVE_PLAY_OUT`, `__CARGO_TEST_ROOT`, a shell's own) is nothing the unit reads, so it neither travels nor blocks the offload. */
export function environmentReadNamesV1(files:Iterable<string>,read:(path:string)=>string|undefined=path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}}):ReadonlySet<string>{
 const names=new Set<string>();
 for(const file of files){
  if(!file.endsWith(".rs"))continue;
  const text=read(file);
  if(text===undefined||!text.includes("env!"))continue;
  for(const match of text.matchAll(READ_NAMES))names.add(match[1]!);
 }
 return names;
}
const environmentReadable=(name:string,scanned:ReadonlySet<string>):boolean=>CARRIED_NAMES.has(name)||scanned.has(name)||CARRIED_PREFIXES.some(prefix=>name.startsWith(prefix));

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
 const sourceReal=resolve(cwd,pass.source),reached=reachedFilesV1(sourceReal,undefined,REACHED_FILES,manifest);
 if(reached===undefined)return undefined;
 const shipped=[sourceReal,...reached].reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest);
 if(within(cwd,shipped)||shipped===manifest||depthOf(shipped)<MINIMUM_SHIPPED_DEPTH||!within(workspace,shipped)||remappedNameV1(remapsOfV1(args),shipped)!==shipped)return undefined;
 const first=args.findIndex(argument=>argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")),rewritten=args.map(argument=>argument===pass.source&&!isAbsolute(argument)?sourceReal:argument),at=first<0?rewritten.length:first;
 return Object.freeze([...rewritten.slice(0,at),`--remap-path-prefix=${shipped}=${shipped===workspace?".":relative(workspace,shipped).split(sep).join("/")}`,...rewritten.slice(at)]);
}

const SEARCH_KINDS=OFFLOAD_POLICY_V1.link.searchKinds;
const searchSplit=(text:string):{kind:string;dir:string}|undefined=>{
 const equals=text.indexOf("="),kind=equals>0?text.slice(0,equals):"";
 if(SEARCH_KINDS.includes(kind))return{kind,dir:text.slice(equals+1)};
 return text===""||kind==="crate"||kind==="dependency"?undefined:{kind:"all",dir:text};
};
const linkArgumentPortable=(value:string):boolean=>offloadWasmLinkArgumentV1(value)||offloadLinkArgumentV1(value)||value.startsWith("-Wl,-install_name,@rpath/");
const NATIVE_FILE=/(?:^|\/)(?:[^/]+\.(?:a|dylib|tbd|lib)|[^/]+\.so(?:\.[0-9]+)*|[^/]+\.framework\/.+)$/u;

/** 📚️ The native libraries of one search directory that the linker of another machine may need, by path relative to the directory: static and dynamic libraries, import libraries, text stubs and the files of frameworks, never the objects, sources and logs of the build that made them; `undefined` beyond the bounds of the policy. The final link of a unit does not name its libraries (the compiled dependencies carry them), so every library of the directory travels. */
export function nativeLibraryFilesV1(directory:string):ReadonlyMap<string,string>|undefined{
 const all=walkFilesV1(directory,[".git","node_modules"],policy.limits.maximumInputFiles*8,Number.MAX_SAFE_INTEGER);
 if(all===undefined)return undefined;
 const kept=new Map<string,string>();let bytes=0;
 for(const [path,real] of all){
  if(!NATIVE_FILE.test(path))continue;
  try{bytes+=statSync(real).size;}catch{continue;}
  if(kept.size>=policy.limits.maximumInputFiles||bytes>policy.limits.maximumInputBytes)return undefined;
  kept.set(path,real);
 }
 return kept;
}

/** 🔑️ The native inputs of a linking unit as unit-key components, `undefined` when they cannot be keyed: one `{name,sha256}` row per library file of every native search directory that is not a system directory (`native:<kind>:<path in the directory>`, sorted), so a unit that links a static library of a build script has another key when the library changes, which is what lets such a unit be cached, shared with the fleet and offloaded. Every `-l` must be one that a worker resolves (static, or a library or framework of the schema), every directory must exist below no system prefix and all its libraries must be readable; the SDK itself is part of the key through `SDKROOT` and the platform's linker identity. */
export function nativeKeyInputsV1(input:Readonly<{args:readonly string[];cwd:string;digest:(path:string)=>string|null}>):readonly Readonly<{name:string;sha256:string}>[]|undefined{
 const pass=scan(input.args);
 if("declined" in pass)return undefined;
 const rows:{name:string;sha256:string}[]=[],seen=new Set<string>();
 for(const {kind,dir} of pass.native){
  const real=resolve(input.cwd,dir);
  if(seen.has(kind+"="+real)||offloadSystemDirectoryV1(real)||!existsSync(real))continue;
  seen.add(kind+"="+real);
  if(SYSTEM.some(prefix=>real.startsWith(prefix)))return undefined;
  const files=nativeLibraryFilesV1(real);
  if(files===undefined)return undefined;
  for(const [path,file] of files){const sha256=input.digest(file);if(sha256===null)return undefined;rows.push({name:`native:${kind}:${path}`,sha256});}
 }
 return rows.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:left.sha256<right.sha256?-1:left.sha256>right.sha256?1:0);
}

type Pass=Readonly<{outDir:string|undefined;depsDirs:readonly string[];native:readonly Readonly<{kind:string;dir:string}>[];source:string|undefined;extra:string;crate:string;target:string|undefined}>;
function scan(args:readonly string[]):Pass|OffloadDeclineV1{
 let outDir:string|undefined,source:string|undefined,extra="",crate="",target:string|undefined;const depsDirs:string[]=[],native:{kind:string;dir:string}[]=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,take=(flag:string):string|undefined=>argument===flag?args[++index]??"":argument.startsWith(flag+"=")?argument.slice(flag.length+1):undefined;
  let value:string|undefined;
  if((value=take("--out-dir"))!==undefined){outDir=value;continue;}
  if((value=take("--crate-name"))!==undefined){crate=value;continue;}
  if((value=take("--target"))!==undefined){if(value.endsWith(".json"))return decline("unmapped-path","--target "+value);target=value;continue;}
  if(argument==="-L"||argument.startsWith("-L")){
   const text=argument==="-L"?args[++index]??"":argument.slice(2);
   if(text.startsWith("dependency=")){depsDirs.push(text.slice("dependency=".length));continue;}
   const split=searchSplit(text);
   if(split===undefined)return decline("not-portable","-L "+text.split("=")[0]);
   native.push(split);continue;
  }
  if(argument==="-l"||argument.startsWith("-l")){
   const spec=argument==="-l"?args[++index]??"":argument.slice(2),refusal=offloadLinkLibraryRefusalV1(spec);
   if(refusal!==undefined)return decline("not-portable",refusal);
   continue;
  }
  if(argument==="-o"||argument.startsWith("--sysroot")||argument==="--emit-dep-info")return decline("unmapped-path",argument);
  if((value=take("--emit"))!==undefined){if(value.includes("="))return decline("unmapped-path","--emit "+value);continue;}
  const codegen=argument==="-C"?args[index+1]??"":argument.startsWith("-C")?argument.slice(2):undefined;
  if(codegen!==undefined){
   if(codegen.startsWith("extra-filename="))extra=codegen.slice("extra-filename=".length);
   if(codegen.startsWith("link-arg=")&&!linkArgumentPortable(codegen.slice(9)))return decline("not-portable","-C link-arg "+codegen.slice(9,60));
   if(argument==="-C")index++;continue;
  }
  if(argument.endsWith(".rs")&&!argument.startsWith("-"))source=argument;
 }
 return{outDir,depsDirs,native,source,extra,crate,target};
}

/** 🎯️ The files an earlier compilation of this unit read that exist now and that the scan may have missed: the paths of its dependency file `info` (resolved against `cwd`) and the `recorded` gaps of its slot (absolute); the exact read set of rustc itself, empty when there was no earlier compilation. */
export function exactReadsV1(info:string,cwd:string,recorded:readonly string[]):readonly string[]{
 const found=new Set<string>();
 const add=(path:string):void=>{try{if(statSync(path).isFile())found.add(path);}catch{}};
 for(const path of recorded)add(path);
 try{if(existsSync(info))for(const dependency of parseDepInfoV1(readFileSync(info,"utf8")).dependencies)add(resolve(cwd,dependency));}catch{}
 return[...found];
}
/** 📐️ Plans one offloaded compilation from the compiler arguments (response files already expanded), the environment of the call and its working directory; a unit that cannot be made path-independent is declined with the reason, never offloaded half-mapped. A self-contained manifest also ships non-Rust files under its `src`, and an extern of `semio_os_kernel_dsl_derive` stays local. */
export function planOffloadV1(input:Readonly<{args:readonly string[];environment:Environment;cwd:string;recorded?:readonly string[];before?:(plan:Pass)=>void}>):OffloadPlanV1|OffloadDeclineV1{
 const {args,environment,cwd}=input,pass=scan(args);
 if("declined" in pass)return pass;
 if(pass.outDir===undefined||pass.source===undefined)return decline("not-portable","out-dir or source");
 const manifest=environment.CARGO_MANIFEST_DIR,remaps=remapsOfV1(args),nameOf=(path:string):string=>remappedNameV1(remaps,path);
 if(manifest===undefined||!isAbsolute(manifest))return decline("unmapped-path","CARGO_MANIFEST_DIR");
 const sourceReal=resolve(cwd,pass.source),reached=[sourceReal,...includedSourcesV1(sourceReal,undefined,REACHED_FILES,manifest)];
 const infoPath=join(resolve(cwd,pass.outDir),pass.crate+pass.extra+".d"),textual=workspaceDeriveFilesV1(reached);
 const applied=textual.length>0&&exactReadsV1(infoPath,cwd,input.recorded??[]).some(path=>basename(path)===MUTATION_AUTHORITY_FILE_V1),deriving=applied||textual.length>0&&!existsSync(infoPath)?textual:[];
 const roots=deriving.map(file=>deriveWorkspaceRootV1(file));
 const where=deriving.length===0?"":relative(cwd,deriving[0]!).split(sep).join("/");
 if(roots.some(root=>root===undefined))return decline("not-portable","the mutation derive in "+where+" finds no workspace (a package.json declaring workspaces and a 📋️project.json side by side) above its source");
 const shipped=[...reached,...roots as string[]].reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest),anchored=within(cwd,shipped);
 if(!anchored&&shipped!==manifest&&(depthOf(shipped)<MINIMUM_SHIPPED_DEPTH||reachedFilesV1(sourceReal,undefined,REACHED_FILES,manifest)===undefined))return decline("unmapped-path","source outside CARGO_MANIFEST_DIR: working directory "+cwd+", manifest "+manifest+", sources up to "+shipped);
 if(deriving.length>0&&!applied)return decline("not-portable","the mutation derive in "+where+" reads workspace files that no compile of this unit has recorded yet (no dependency file, no input manifest): it compiles here first, then it offloads");
 const climb=anchored?0:climbAboveV1(sourceReal,reached.slice(1),shipped,manifest),above=climb>0?shipped.split(/[\\/]/u).filter(part=>part!=="").slice(-climb):[],root=[virtual.package,...above].join("/");
 const packageVirtual=anchored?relative(cwd,manifest).split(sep).join("/"):shipped===manifest?root:root+"/"+relative(shipped,manifest).split(sep).join("/"),shippedVirtual=anchored?relative(cwd,shipped).split(sep).join("/"):root,files=new Map<string,string>(),mappings:OffloadMappingV1[]=[];
 const ignored=["target",".git","node_modules",".cache",virtual.base];
 const reachedAll=reachedFilesV1(sourceReal,undefined,REACHED_FILES,manifest),closure=new Map<string,string>();
 if(reachedAll!==undefined&&[sourceReal,...reachedAll].every(path=>within(shipped,path))){
  for(const path of [sourceReal,...reachedAll,join(manifest,"Cargo.toml")])if(existsSync(path))closure.set(relative(shipped,path).split(sep).join("/"),path);
  for(const real of exactReadsV1(join(resolve(cwd,pass.outDir),pass.crate+pass.extra+".d"),cwd,input.recorded??[]))if(within(shipped,real))closure.set(relative(shipped,real).split(sep).join("/"),real);
  if(shipped===manifest&&existsSync(join(manifest,"src"))){const embedded=walkFilesV1(join(manifest,"src"),ignored);if(embedded!==undefined)for(const real of embedded.values())if(!real.endsWith(".rs"))closure.set(relative(shipped,real).split(sep).join("/"),real);}
 }
 const walked=closure.size>0?closure:walkFilesV1(shipped,ignored);
 if(walked===undefined)return decline("inputs-too-large","package "+shipped);
 const place=(path:string,real:string):void=>{files.set(path,real);};
 for(const [path,real] of walked)place(shippedVirtual===""?path:shippedVirtual+"/"+path,real);
 for(const [index,file] of deriving.entries())for(const marker of deriveMarkersV1(file,roots[index]!))place(shippedVirtual===""?relative(shipped,marker).split(sep).join("/"):shippedVirtual+"/"+relative(shipped,marker).split(sep).join("/"),marker);
 if(!anchored)mappings.push({real:shipped,virtual:root});
 const cwdName=nameOf(cwd);
 if(!anchored&&shipped!==manifest&&nameOf(shipped)===shipped)return decline("unmapped-path","no --remap-path-prefix covers the module directory "+shipped+": the compiler would record this machine's paths");
 if(cwdName===cwd)return decline("unmapped-path","no --remap-path-prefix covers the working directory: the compiler would record this machine's paths");
 const outDirReal=resolve(cwd,pass.outDir),dependencies=[...new Set(pass.depsDirs.map(directory=>resolve(cwd,directory)))],directoryNames=new Map<string,string>();
 dependencies.forEach((directory,index)=>{directoryNames.set(directory,index===0?virtual.deps:virtual.deps+index);});
 const nativeNames=new Map<string,string|null>(),nativeInputs:[string,string][]=[];
 for(const {dir} of pass.native){
  const real=resolve(cwd,dir);
  if(nativeNames.has(real))continue;
  if(offloadSystemDirectoryV1(real)){nativeNames.set(real,null);continue;}
  if(SYSTEM.some(prefix=>real.startsWith(prefix)))return decline("not-portable","native search directory "+real);
  if(!existsSync(real)){nativeNames.set(real,null);continue;}
  const found=nativeLibraryFilesV1(real);
  if(found===undefined)return decline("inputs-too-large","native search directory "+real);
  const name=virtual.native+[...nativeNames.values()].filter(row=>row!==null).length;
  nativeNames.set(real,name);mappings.push({real,virtual:name});
  for(const [path,file] of found)nativeInputs.push([name+"/"+path,file]);
 }
 for(const [path,file] of nativeInputs)place(path,file);
 let external=0;
 const virtualOf=(directory:string):string=>{let name=directoryNames.get(directory);if(name===undefined){name=virtual.external+external++;directoryNames.set(directory,name);}return name;};
 const rewritten:string[]=[],direct=new Map<string,string[]>(),inside:string[]=[];let sourceName:string|undefined;
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,take=(flag:string):string|undefined=>argument===flag?args[++index]??"":argument.startsWith(flag+"=")?argument.slice(flag.length+1):undefined,joined=argument.startsWith("--")&&argument.includes("=");
  let value:string|undefined;
  if((value=take("--out-dir"))!==undefined){rewritten.push(joined?"--out-dir="+virtual.out:"--out-dir",...joined?[]:[virtual.out]);continue;}
  if(argument==="-L"||argument.startsWith("-L")){
   const text=argument==="-L"?args[++index]!:argument.slice(2),dependency=text.startsWith("dependency="),split=dependency?undefined:searchSplit(text),named=split===undefined?undefined:nativeNames.get(resolve(cwd,split.dir));
   if(!dependency&&(split===undefined||named===null||named===undefined))continue;
   const mapped=dependency?"dependency="+directoryNames.get(resolve(cwd,text.slice("dependency=".length)))!:`${split!.kind}=${named}`;
   if(argument==="-L")rewritten.push("-L",mapped);else rewritten.push("-L"+mapped);continue;
  }
  if((value=take("--extern"))!==undefined){
   const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1),real=path===""?"":resolve(cwd,path);
   if(path===""){rewritten.push(...joined?["--extern="+spec]:["--extern",spec]);continue;}
   const directory=dirname(real),name=virtualOf(directory),mapped=spec+"="+name+"/"+basename(real);
   if(!existsSync(real))return decline("unreadable-input","extern "+path);
   const sibling=real.replace(/\.(?:rlib|rmeta)$/u,real.endsWith(".rlib")?".rmeta":".rlib");direct.set(directory,[...direct.get(directory)??[],real,...sibling!==real&&existsSync(sibling)?[sibling]:[]]);rewritten.push(...joined?["--extern="+mapped]:["--extern",mapped]);continue;
  }
  if(argument==="-C"&&(args[index+1]??"").startsWith("incremental=")){index++;continue;}
  if(argument.startsWith("-Cincremental="))continue;
  if(argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")){
   const joinedRemap=argument.includes("="),spec=joinedRemap?argument.slice(argument.indexOf("=")+1):args[++index]??"",from=spec.slice(0,Math.max(0,spec.lastIndexOf("=")));
   if(isAbsolute(from)&&!SYSTEM.some(prefix=>from.startsWith(prefix))){if(!anchored&&within(shipped,from)){const rest=relative(shipped,from).split(sep).join("/");inside.push(`${root}${rest===""?"":"/"+rest}=${spec.slice(from.length+1)}`);}continue;}
   rewritten.push(...joinedRemap?[argument]:[argument,spec]);continue;
  }
  if(argument===pass.source){const shown=anchored?relative(cwd,sourceReal).split(sep).join("/"):root+"/"+relative(shipped,sourceReal).split(sep).join("/");if(isAbsolute(pass.source))sourceName=shown;else rewritten.push(shown);continue;}
  rewritten.push(argument);
 }
 const names:string[]=[];
 if(!anchored)names.push(`${root}=${nameOf(shipped)}`,...inside);
 const crateTypes=offloadCrateTypesOfV1(args),dependencyInputs=dependencyClosureV1([...directoryNames.keys()],[...direct.values()].flat(),undefined,(crateTypes.length>0?crateTypes:["bin"]).some(type=>type!=="lib"&&type!=="rlib"));
 if(dependencyInputs===undefined)return decline("inputs-too-large","dependency closure: more than "+policy.limits.maximumClosureFiles+" artifacts");
 for(const [directory,name] of directoryNames){
  mappings.push({real:directory,virtual:name});
  for(const file of dependencyInputs.get(directory)??[])place(name+"/"+file,join(directory,file));
  for(const real of direct.get(directory)??[])place(name+"/"+basename(real),real);
 }
 mappings.push({real:pass.outDir,virtual:virtual.out});
 const read=semioEnvironmentReadV1(walked.values(),environment);
 if(read!==undefined)return decline("not-portable","the sources read "+read+", which no worker receives");
 const absolute:string[]=[],environmentOut:Record<string,string>={},scanned=environmentReadNamesV1(walked.values());
 for(const [name,value] of Object.entries(environment)){
  if(value===undefined||deniedName(name))continue;
  if(!environmentReadable(name,scanned)&&!(environment.OUT_DIR!==undefined&&value.includes(environment.OUT_DIR)))continue;
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
    if(within(shipped,real)||mappings.some(row=>row.virtual!==virtual.out&&within(row.real,real)))continue;
    if(!within(cwd,real))return decline("unmapped-path","dependency "+dependency);
    const path=relative(cwd,real).split(sep).join("/");
    if(!files.has(path)&&existsSync(real))place(path,real);
   }
  }catch{return decline("unreadable-input","dependency file");}
 }
 for(const real of input.recorded??[]){
  if(within(shipped,real)||!within(cwd,real)||mappings.some(row=>row.virtual!==virtual.out&&within(row.real,real)))continue;
  const path=relative(cwd,real).split(sep).join("/");
  if(!files.has(path)&&existsSync(real))place(path,real);
 }
 for(const argument of rewritten){
  if(argument.includes(cwd))return decline("unmapped-path",argument.slice(0,80));
  const value=argument.includes("=")?argument.slice(argument.indexOf("=")+1):argument;
  if(isAbsolute(value)&&!SYSTEM.some(prefix=>value.startsWith(prefix)))return decline("unmapped-path",argument.slice(0,80));
 }
 const finalArguments=[...rewritten,...installNameArgumentsV1(args)];
 return Object.freeze({arguments:Object.freeze(finalArguments),environment:Object.freeze(environmentOut),files,mappings:Object.freeze(mappings),outDir:pass.outDir,crateName:pass.crate,target:pass.target,extraFilename:pass.extra});
}
