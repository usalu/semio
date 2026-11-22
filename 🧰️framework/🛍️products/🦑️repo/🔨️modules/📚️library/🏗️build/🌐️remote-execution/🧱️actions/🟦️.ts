import {isAbsolute,posix,win32} from "node:path";
import {OFFLOAD_POLICY_V1,offloadCrateTypeRefusalV1,offloadLinkArgumentV1,offloadLinkLibraryRefusalV1,offloadSearchDirectoryRefusalV1,offloadWasmLinkArgumentV1,offloadWasmTargetV1} from "../🚀️offload/📐️policy/🟦️.ts";
import {COMMAND_POLICY_V1,commandTextV1,commandToolKeyV1,commandToolSourceV1,parseCommandProgramV1} from "../🚀️offload/🧰️command/📐️policy/🟦️.ts";

/** 🧱️ Which actions a worker runs: `rustc` (only the declared rustc unit of the offload, with validated arguments and environment), `any` (every command; allowed only on a confining sandbox) or `none`. */
export type RemoteExecutionActionPolicyV1="rustc"|"any"|"none";

/** 📜️ The part of a decoded `Command` the validator reads. */
export type RemoteExecutionCommandViewV1=Readonly<{arguments:readonly string[];environment_variables:readonly {name:string;value:string}[];output_paths:readonly string[];output_files?:readonly string[];output_directories?:readonly string[];working_directory:string}>;

const policy=OFFLOAD_POLICY_V1,virtual=policy.virtual,rootMark=policy.root.placeholder,toolProgram=`${policy.tool.placeholder}:${policy.tool.name}`,SAFE_TEXT=/^[^\0\r\n]{0,16384}$/u,CHECK_CFG_TEXT=new RegExp(`^[^\\0\\r\\n]{0,${OFFLOAD_POLICY_V1.limits.maximumCheckConfigurationBytes}}$`,"u");
const MANIFEST_TEXT=/^[^\0]{0,65536}$/u;
const WITH_VALUE=new Set(["--crate-name","--crate-type","--edition","--emit","--out-dir","--target","--cfg","--check-cfg","--cap-lints","--error-format","--json","--diagnostic-width","--color","--extern","--remap-path-prefix","--remap-path-scope","--allow","--warn","--deny","--forbid","--force-warn","-L","-C","-A","-W","-D","-F"]);
const BARE=new Set(["--test","-g","-O","--verbose","-v"]);
const CODEGEN=new Set(["opt-level","debuginfo","debug-assertions","overflow-checks","lto","panic","codegen-units","metadata","extra-filename","embed-bitcode","strip","prefer-dynamic","target-cpu","target-feature","relocation-model","code-model","split-debuginfo","symbol-mangling-version","force-frame-pointers","inline-threshold","rpath","no-redzone","soft-float","link-dead-code","instrument-coverage","control-flow-guard","dwarf-version","force-unwind-tables","linker-plugin-lto","linker-features","link-self-contained"]);
const REMAP_SCOPES=new Set(["macro","diagnostics","debuginfo","coverage","object","all"]);
const EMIT=new Set(["dep-info","metadata","link","asm","llvm-ir","obj","llvm-bc","mir"]);
const DENIED_ENVIRONMENT_PREFIXES=["RUSTC","RUSTFLAGS","RUSTDOC","LD_","DYLD_","MALLOC","BASH_FUNC_"];
const DENIED_ENVIRONMENT=new Set(["BASH_ENV","ENV","IFS","SHELLOPTS","BASHOPTS","PS4","PROMPT_COMMAND","CDPATH","GLOBIGNORE","POSIXLY_CORRECT","SHELL","TMPDIR","HOME","CARGO_HOME","RUSTUP_HOME","RUSTUP_TOOLCHAIN"]);
const OSO_PREFIX=`-Wl,-oso_prefix,${rootMark}/${virtual.out}/`;
const ABSOLUTE_NAMES=new Set(["OUT_DIR","CARGO_MANIFEST_DIR","CARGO_MANIFEST_PATH"]);

const REL_PATH=/^[^\s"'\\`$;|&<>*?(){}\[\]!~#%^=:,\u0000-\u001f\u007f]{1,512}$/u;
const relative=(value:string):boolean=>value!==""&&!isAbsolute(value)&&!value.startsWith("/")&&!value.startsWith("\\")&&!value.includes("\\")&&!/^[A-Za-z]:/u.test(value)&&!value.split(/[\\/]/u).includes("..")&&SAFE_TEXT.test(value);
const strict=(value:string):boolean=>relative(value)&&REL_PATH.test(value);
const lint=/^[A-Za-z0-9_:-]{1,128}$/u,token=/^[A-Za-z0-9_.,=+:@-]{1,512}$/u;
const INSTALL_NAME=/^-Wl,-install_name,@rpath\/lib[A-Za-z0-9_]{1,200}(?:-[0-9a-f]{1,32})?\.dylib$/u;
const NAME_TEXT=/^[^\0\r\n]{0,1024}$/u;

function flag(name:string,value:string):string|undefined{
 if(!(name==="--check-cfg"?CHECK_CFG_TEXT:SAFE_TEXT).test(value))return `${name} value is not plain text`;
 switch(name){
  case "--crate-name":return /^[A-Za-z_][A-Za-z0-9_]{0,200}$/u.test(value)?undefined:"--crate-name";
  case "--crate-type":return (policy.crateTypes as readonly string[]).includes(value)||(policy.wasm.crateTypes as readonly string[]).includes(value)||(policy.link.crateTypes as readonly string[]).includes(value)?undefined:`--crate-type ${value}`;
  case "--edition":return /^20[0-9]{2}$/u.test(value)?undefined:"--edition";
  case "--emit":return value.split(",").every(part=>EMIT.has(part))?undefined:`--emit ${value}`;
  case "--out-dir":return value===virtual.out?undefined:`--out-dir ${value}`;
  case "--target":return /^[A-Za-z0-9_.-]{1,128}$/u.test(value)&&!value.endsWith(".json")?undefined:"--target";
  case "--cfg":return /^[A-Za-z0-9_="'(),:. *+\-]{1,1024}$/u.test(value)?undefined:name;
  case "--check-cfg":return /^[\p{L}\p{N}_="'(),:. *+\-]+$/u.test(value)?undefined:name;
  case "--cap-lints":return ["allow","warn","deny","forbid"].includes(value)?undefined:name;
  case "--error-format":case "--json":case "--color":return token.test(value)?undefined:name;
  case "--diagnostic-width":return /^[0-9]{1,4}$/u.test(value)?undefined:name;
  case "--extern":{const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1);return /^(?:(?:priv|noprelude|nounused):)*[A-Za-z_][A-Za-z0-9_]{0,200}$/u.test(spec)&&(equals<0||strict(path))?undefined:"--extern";}
  case "--remap-path-prefix":{const equals=value.indexOf("="),from=equals<0?"":value.slice(0,equals);return equals<1||!NAME_TEXT.test(value.slice(equals+1))?name:from===rootMark||!from.startsWith(rootMark+"/")||strict(from.slice(rootMark.length+1))?undefined:name;}
  case "--remap-path-scope":return value.split(",").every(part=>REMAP_SCOPES.has(part))?undefined:name;
  case "-Z":return /^(?:threads=[0-9]{1,3}|no-parallel-backend|unstable-options|embed-metadata=(?:yes|no)|checksum-hash-algorithm=(?:md5|sha1|sha256|blake3))$/u.test(value)?undefined:`-Z ${value.slice(0,40)}`;
  case "--allow":case "--warn":case "--deny":case "--forbid":case "--force-warn":case "-A":case "-W":case "-D":case "-F":return lint.test(value)?undefined:name;
  case "-L":return value.startsWith("dependency=")?strict(value.slice("dependency=".length))?undefined:"-L":offloadSearchDirectoryRefusalV1(value);
  case "-l":return offloadLinkLibraryRefusalV1(value);
  case "-C":{const equals=value.indexOf("="),key=equals<0?value:value.slice(0,equals);if(key==="link-arg"){const link=value.slice(equals+1);return INSTALL_NAME.test(link)||link===OSO_PREFIX||offloadWasmLinkArgumentV1(link)||offloadLinkArgumentV1(link)?undefined:"-C link-arg";}return CODEGEN.has(key)&&token.test(value.slice(equals+1))?undefined:`-C ${key}`;}
 }
 return `${name} is not allowed`;
}

/** 🧭️ The path flavours an action path is mapped to: only `join` and `sep` are used, so `path.posix` and `path.win32` both qualify and a test can run either on any host. */
export type RemoteExecutionPathFlavourV1=Pick<typeof posix,"join"|"sep">;

const ROOT_TEXT=new RegExp(`(^|[=,])${rootMark}(/[^=,]*)?`,"gu");

/** 🗺️ Maps a relative POSIX action path (`a/b/c`, `.` and empty segments dropped, a trailing `/` kept) below `root` to the separators of one path flavour; the encoding of an action never changes, only this mapping does. */
export function remoteExecutionNativePathV1(root:string,path:string,flavour:RemoteExecutionPathFlavourV1):string{
 const mapped=flavour.join(root,...path.split("/").filter(part=>part!==""&&part!=="."));
 return path.endsWith("/")&&!mapped.endsWith(flavour.sep)?mapped+flavour.sep:mapped;
}

/** 🪟️ The same mapping for native Windows, as a pure function. */
export const remoteExecutionWindowsPathV1=(root:string,path:string):string=>remoteExecutionNativePathV1(root,path,win32);

/** 🧩️ Replaces the input-root placeholder of one action argument or environment value (`@root`, or `@root/<relative posix path>` at the start, after `=` or after `,`; the path ends at the next `=` or `,`) by the native path below `root`. */
export function expandRemoteExecutionRootV1(text:string,root:string,flavour:RemoteExecutionPathFlavourV1):string{
 if(!text.includes(rootMark))return text;
 return text.replace(ROOT_TEXT,(_,lead:string,path:string|undefined)=>lead+remoteExecutionNativePathV1(root,path===undefined?"":path.slice(1),flavour));
}

/** 🛠️ The tool name of an action program (`@tool:rustc` is `rustc`), `undefined` for every other program. */
export function remoteExecutionToolOfProgramV1(program:string):string|undefined{
 const mark=policy.tool.placeholder+":";
 return program.startsWith(mark)&&program.length>mark.length?program.slice(mark.length):undefined;
}

/** 🧷️ Whether every root placeholder in a text names a clean relative path below the root (no `..`, no absolute or drive form, no shell or path-special character). */
function rootPathsValid(text:string):boolean{
 if(!text.includes(rootMark))return true;
 for(const match of text.matchAll(ROOT_TEXT)){
  const path=match[2]===undefined?"":match[2].slice(1).replace(/\/+$/u,"");
  if(path!==""&&!strict(path))return false;
 }
 return true;
}

/** 🔬️ Checks that a command is exactly what the rustc offload sends: the program `@tool:rustc` (no shell, on every operating system) with <rustc arguments>, with rustc arguments from a closed allow-list (no linker but the declared link arguments: the macOS install name and object prefix, for a native target the schema's `link` arguments, for a WebAssembly target only the numeric `--max-memory` and `-zstack-size` and the `-Wl,--export=<name>` of wasm-ld; a `cdylib` links for a policy WebAssembly target or a native one, `dylib` and `staticlib` for a native one; `-l` only as a static library or a system library or framework that the schema lists and the worker resolves in its own SDK, `-L native|all|framework` only as a directory shipped with the action; no `-Z` but `threads=<n>`, `no-parallel-backend` and `unstable-options`, response file, `--sysroot`, `--print`, `-o`, absolute or `..` path, `.json` target), an environment without loader, shell-startup or rustc-control variables, only the declared `SEMIO_` variables, and the two virtual output paths. Answers `undefined` when the command is acceptable, otherwise the first reason. The compiler may still run the repository's own proc-macro and build-script code that the requester shipped as inputs: that is code of the group's own repository, inside the fleet's trust boundary (whoever writes the group store already changes the code every machine runs), and on a confining sandbox it runs without network and without access to the host. */
export function validateRemoteExecutionRustcActionV1(command:RemoteExecutionCommandViewV1):string|undefined{
 const [program,...args]=command.arguments;
 if(program!==toolProgram)return "only the declared rustc action may run on this worker";
 if(command.working_directory!=="")return "working directory not allowed";
 if((command.output_files?.length??0)>0||(command.output_directories?.length??0)>0)return "only output_paths are allowed";
 if(command.output_paths.length!==2||!command.output_paths.includes(virtual.out)||!command.output_paths.includes(virtual.root))return "output paths differ from the declared ones";
 if(args.length>20000)return "too many arguments";
 let sources=0,remaps=0,libraries=0,target:string|undefined,wasmLink=false,nativeLink=false;const types:string[]=[];
 const noteLink=(value:string):void=>{if(offloadWasmLinkArgumentV1(value))wasmLink=true;else if(offloadLinkArgumentV1(value))nativeLink=true;};
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(!SAFE_TEXT.test(argument)&&!(argument.startsWith("--check-cfg=")&&CHECK_CFG_TEXT.test(argument)))return "argument is not plain text";
  if(argument.startsWith("@")&&!argument.startsWith(rootMark+"/"))return "response files are not allowed";
  if(!rootPathsValid(argument))return `root path in ${argument.slice(0,60)}`;
  if(argument.endsWith(".rs")&&!argument.startsWith("-")){if(!strict(argument.startsWith(rootMark+"/")?argument.slice(rootMark.length+1):argument))return `source path ${argument.slice(0,80)}`;sources++;continue;}
  if(BARE.has(argument))continue;
  let reason:string|undefined;
  if(argument.startsWith("--")){
   const equals=argument.indexOf("="),flagName=equals<0?argument:argument.slice(0,equals);
   if(!WITH_VALUE.has(flagName))return `${flagName.slice(0,60)} is not allowed`;
   if(flagName==="--remap-path-prefix"&&++remaps>64)return "too many remaps";
   const flagValue=equals<0?args[++index]??"":argument.slice(equals+1);
   reason=flag(flagName,flagValue);
   if(flagName==="--crate-type")types.push(flagValue);else if(flagName==="--target")target=flagValue;
  }else if(argument==="-C"||argument==="-L"||argument==="-l"||argument==="-Z"||argument==="-A"||argument==="-W"||argument==="-D"||argument==="-F"){const flagValue=args[++index]??"";if(argument==="-l"&&++libraries>policy.link.maximumLibraries)return "too many libraries";reason=flag(argument,flagValue);if(argument==="-C"&&flagValue.startsWith("link-arg="))noteLink(flagValue.slice(9));}
  else if(argument.startsWith("-C")||argument.startsWith("-L")||argument.startsWith("-l")||argument.startsWith("-Z")){if(argument.startsWith("-l")&&++libraries>policy.link.maximumLibraries)return "too many libraries";reason=flag(argument.slice(0,2),argument.slice(2));if(argument.startsWith("-Clink-arg="))noteLink(argument.slice(11));}
  else if(/^-[AWDF]/u.test(argument))reason=flag(argument.slice(0,2),argument.slice(2));
  else return `${argument.slice(0,60)} is not allowed`;
  if(reason!==undefined)return reason;
 }
 const names=new Set<string>();
 let reported=false;
 for(const {name:variable,value} of command.environment_variables){
  if(!/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(variable)||names.has(variable))return `environment name ${variable.slice(0,40)}`;
  names.add(variable);
  if(variable.startsWith("CARGO_PKG_")){if(!MANIFEST_TEXT.test(value))return `environment value of ${variable}`;continue;}
  if(!SAFE_TEXT.test(value)||value.length>16384)return `environment value of ${variable}`;
  if(value.includes(rootMark)&&(!value.startsWith(rootMark)||value.indexOf(rootMark,1)>=0||!rootPathsValid(value)||value!==rootMark&&!value.startsWith(rootMark+"/")))return `${variable} path`;
  if(variable==="PATH"){if(value!==policy.environment.path)return "PATH differs from the declared one";continue;}
  if(variable==="HOME"){if(value!==`${rootMark}/${policy.root.home}`)return "HOME differs from the declared one";continue;}
  if(variable===policy.root.report){if(value!==virtual.root)return `${variable} path`;reported=true;continue;}
  if(ABSOLUTE_NAMES.has(variable)){const path=value.startsWith(rootMark+"/")?value.slice(rootMark.length+1):"";if(value!==rootMark&&(!relative(path)||!REL_PATH.test(path)))return `${variable} path`;continue;}
  if(variable.startsWith("SEMIO_"))return `${variable} is not allowed`;
  if(DENIED_ENVIRONMENT.has(variable)||DENIED_ENVIRONMENT_PREFIXES.some(prefix=>variable.startsWith(prefix))||policy.environment.deniedNames.includes(variable))return `${variable} is not allowed`;
 }
 const kind=offloadCrateTypeRefusalV1(types,target);
 if(kind!==undefined)return kind;
 if(wasmLink&&!offloadWasmTargetV1(target))return "wasm linker argument for a target that is not WebAssembly";
 if(nativeLink&&offloadWasmTargetV1(target))return "native linker argument for a WebAssembly target";
 if(sources!==1)return "exactly one source file required";
 if(!reported)return "root report missing";
 return undefined;
}

/** 🖥️ What a worker offers for command actions: the resolved tools it owns by `<tool>/<digest>`, whether its sandbox confines, and whether its actions may reach the network. */
export type RemoteExecutionCommandOfferV1=Readonly<{tools:Readonly<Record<string,unknown>>;confined:boolean;network:boolean}>;

/** 🧾️ Whether a program is a command program (`@command:<tool>/<digest>`). */
export const remoteExecutionCommandProgramV1=(program:string):boolean=>parseCommandProgramV1(program)!==undefined;

const commandEnvironment=COMMAND_POLICY_V1.environment;

/** 🔬️ Checks that a command is a declared command offload action: the program `@command:<tool>/<digest>` of the closed allow-list (a `resolved` tool only when this worker owns it by exactly that digest, a `shipped` tool whose digest the worker verifies after placing the inputs), only on a confining sandbox that allows no network, no working directory, only relative `out/` output paths, arguments that are plain text and name no path outside the input root (`@root/<relative path>` only), and an environment of declared names. Answers `undefined` when acceptable, otherwise the first reason in English and German. The tool may still run any code of its own: that code is the group's own repository code or an allow-listed binary, inside the confining sandbox without network and without access to the host. */
export function validateRemoteExecutionCommandActionV1(command:RemoteExecutionCommandViewV1,offer:RemoteExecutionCommandOfferV1):string|undefined{
 const [program,...args]=command.arguments,parsed=program===undefined?undefined:parseCommandProgramV1(program),say=(key:string,variables:Readonly<Record<string,string|number>>={})=>`${commandTextV1("en",key,variables)} / ${commandTextV1("de",key,variables)}`;
 if(parsed===undefined)return say("refusal.shape",{detail:"program"});
 const source=commandToolSourceV1(parsed.name);
 if(source===undefined)return say("refusal.tool",{tool:parsed.name});
 if(!offer.confined||offer.network)return say("refusal.confinement");
 if(source==="resolved"&&offer.tools[commandToolKeyV1(parsed.name,parsed.digest)]===undefined)return say("refusal.digest",{tool:parsed.name});
 if(command.working_directory!=="")return say("refusal.shape",{detail:"working directory"});
 if((command.output_files?.length??0)>0||(command.output_directories?.length??0)>0)return say("refusal.shape",{detail:"output_files"});
 const limits=COMMAND_POLICY_V1.limits,outputs=command.output_paths;
 if(outputs.length===0||outputs.length>limits.maximumOutputFiles)return say("refusal.output",{detail:String(outputs.length)});
 for(const path of outputs)if(!path.startsWith(COMMAND_POLICY_V1.root.output+"/")||!strict(path))return say("refusal.output",{detail:path.slice(0,80)});
 if(args.length>limits.maximumArguments)return say("refusal.argument",{detail:"count"});
 for(const argument of args){
  if(!SAFE_TEXT.test(argument))return say("refusal.argument",{detail:"not plain text"});
  if(argument.startsWith("@")&&!argument.startsWith(rootMark+"/"))return say("refusal.argument",{detail:argument.slice(0,60)});
  if(!rootPathsValid(argument))return say("refusal.argument",{detail:`root path in ${argument.slice(0,60)}`});
  if(!/\s/u.test(argument)&&(/^(?:\/|~|[A-Za-z]:[\\/])/u.test(argument)||argument.split(/[\\/=]/u).includes("..")))return say("refusal.argument",{detail:argument.slice(0,60)});
 }
 const names=new Set<string>();
 for(const {name,value} of command.environment_variables){
  if(!/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(name)||names.has(name))return say("refusal.environment",{detail:name.slice(0,40)});
  names.add(name);
  if(!SAFE_TEXT.test(value)||value.length>16384)return say("refusal.environment",{detail:name});
  if(name==="PATH"){if(value!==commandEnvironment.path)return say("refusal.environment",{detail:"PATH"});continue;}
  if((commandEnvironment.pathNames as readonly string[]).includes(name)){const path=value.startsWith(rootMark+"/")?value.slice(rootMark.length+1):"";if(value!==rootMark&&!strict(path))return say("refusal.environment",{detail:`${name} path`});continue;}
  if(value.includes(rootMark))return say("refusal.environment",{detail:`${name} path`});
  if((commandEnvironment.allowedNames as readonly string[]).includes(name)||commandEnvironment.allowedPrefixes.some(prefix=>name.startsWith(prefix)))continue;
  return say("refusal.environment",{detail:name});
 }
 return undefined;
}
