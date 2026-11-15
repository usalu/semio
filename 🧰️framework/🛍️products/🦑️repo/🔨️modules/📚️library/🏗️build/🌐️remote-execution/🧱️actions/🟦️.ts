import {isAbsolute} from "node:path";
import {OFFLOAD_POLICY_V1} from "../🚀️offload/📐️policy/🟦️.ts";

/** 🧱️ Which actions a worker runs: `rustc` (only the declared rustc unit of the offload, with validated arguments and environment), `any` (every command; allowed only on a confining sandbox) or `none`. */
export type RemoteExecutionActionPolicyV1="rustc"|"any"|"none";

/** 📜️ The part of a decoded `Command` the validator reads. */
export type RemoteExecutionCommandViewV1=Readonly<{arguments:readonly string[];environment_variables:readonly {name:string;value:string}[];output_paths:readonly string[];output_files?:readonly string[];output_directories?:readonly string[];working_directory:string}>;

const policy=OFFLOAD_POLICY_V1,virtual=policy.virtual,SAFE_TEXT=/^[^\0\r\n]{0,16384}$/u;
const MANIFEST_TEXT=/^[^\0]{0,65536}$/u;
const WITH_VALUE=new Set(["--crate-name","--crate-type","--edition","--emit","--out-dir","--target","--cfg","--check-cfg","--cap-lints","--error-format","--json","--diagnostic-width","--color","--extern","--remap-path-prefix","--remap-path-scope","--allow","--warn","--deny","--forbid","--force-warn","-L","-C","-A","-W","-D","-F"]);
const BARE=new Set(["--test","-g","-O","--verbose","-v"]);
const CODEGEN=new Set(["opt-level","debuginfo","debug-assertions","overflow-checks","lto","panic","codegen-units","metadata","extra-filename","embed-bitcode","strip","prefer-dynamic","target-cpu","target-feature","relocation-model","code-model","split-debuginfo","symbol-mangling-version","force-frame-pointers","inline-threshold","rpath","no-redzone","soft-float","link-dead-code","instrument-coverage","control-flow-guard","dwarf-version","force-unwind-tables","linker-plugin-lto"]);
const REMAP_SCOPES=new Set(["macro","diagnostics","debuginfo","coverage","object","all"]);
const EMIT=new Set(["dep-info","metadata","link","asm","llvm-ir","obj","llvm-bc","mir"]);
const DENIED_ENVIRONMENT_PREFIXES=["RUSTC","RUSTFLAGS","RUSTDOC","LD_","DYLD_","MALLOC","BASH_FUNC_"];
const DENIED_ENVIRONMENT=new Set(["BASH_ENV","ENV","IFS","SHELLOPTS","BASHOPTS","PS4","PROMPT_COMMAND","CDPATH","GLOBIGNORE","POSIXLY_CORRECT","SHELL","TMPDIR","HOME","CARGO_HOME","RUSTUP_HOME","RUSTUP_TOOLCHAIN"]);
const ABSOLUTE_NAMES=new Set(["OUT_DIR","CARGO_MANIFEST_DIR","CARGO_MANIFEST_PATH"]);

const REL_PATH=/^[^\s"'\\`$;|&<>*?(){}\[\]!~#%^=:,\u0000-\u001f\u007f]{1,512}$/u;
const relative=(value:string):boolean=>value!==""&&!isAbsolute(value)&&!value.startsWith("/")&&!/^[A-Za-z]:/u.test(value)&&!value.split(/[\\/]/u).includes("..")&&SAFE_TEXT.test(value);
const lint=/^[A-Za-z0-9_:-]{1,128}$/u,token=/^[A-Za-z0-9_.,=+:@-]{1,512}$/u;
const INSTALL_NAME=/^-Wl,-install_name,@rpath\/lib[A-Za-z0-9_]{1,200}(?:-[0-9a-f]{1,32})?\.dylib$/u;
const NAME_TEXT=/^[^\0\r\n]{0,1024}$/u;

function flag(name:string,value:string):string|undefined{
 if(!SAFE_TEXT.test(value))return `${name} value is not plain text`;
 switch(name){
  case "--crate-name":return /^[A-Za-z_][A-Za-z0-9_]{0,200}$/u.test(value)?undefined:"--crate-name";
  case "--crate-type":return (policy.crateTypes as readonly string[]).includes(value)?undefined:`--crate-type ${value}`;
  case "--edition":return /^20[0-9]{2}$/u.test(value)?undefined:"--edition";
  case "--emit":return value.split(",").every(part=>EMIT.has(part))?undefined:`--emit ${value}`;
  case "--out-dir":return value===virtual.out?undefined:`--out-dir ${value}`;
  case "--target":return /^[A-Za-z0-9_.-]{1,128}$/u.test(value)&&!value.endsWith(".json")?undefined:"--target";
  case "--cfg":case "--check-cfg":return /^[A-Za-z0-9_="'(),:. *+\-]{1,1024}$/u.test(value)?undefined:name;
  case "--cap-lints":return ["allow","warn","deny","forbid"].includes(value)?undefined:name;
  case "--error-format":case "--json":case "--color":return token.test(value)?undefined:name;
  case "--diagnostic-width":return /^[0-9]{1,4}$/u.test(value)?undefined:name;
  case "--extern":{const equals=value.indexOf("="),spec=equals<0?value:value.slice(0,equals),path=equals<0?"":value.slice(equals+1);return /^(?:(?:priv|noprelude|nounused):)*[A-Za-z_][A-Za-z0-9_]{0,200}$/u.test(spec)&&(equals<0||relative(path))?undefined:"--extern";}
  case "--remap-path-prefix":return value.includes("=")?undefined:name;
  case "--remap-path-scope":return value.split(",").every(part=>REMAP_SCOPES.has(part))?undefined:name;
  case "-Z":return /^(?:threads=[0-9]{1,3}|embed-metadata=(?:yes|no)|checksum-hash-algorithm=(?:md5|sha1|sha256|blake3))$/u.test(value)?undefined:`-Z ${value.slice(0,40)}`;
  case "--allow":case "--warn":case "--deny":case "--forbid":case "--force-warn":case "-A":case "-W":case "-D":case "-F":return lint.test(value)?undefined:name;
  case "-L":return value.startsWith("dependency=")&&relative(value.slice("dependency=".length))?undefined:"-L";
  case "-C":{const equals=value.indexOf("="),key=equals<0?value:value.slice(0,equals);if(key==="link-arg")return INSTALL_NAME.test(value.slice(equals+1))?undefined:"-C link-arg";return CODEGEN.has(key)&&token.test(value.slice(equals+1))?undefined:`-C ${key}`;}
 }
 return `${name} is not allowed`;
}

/** 🔬️ Checks that a command is exactly what the rustc offload sends: `/bin/sh -c <the fixed script> semio-rustc <rustc arguments>`, with rustc arguments from a closed allow-list (no linker, no `-Z` but `threads=<n>`, response file, `--sysroot`, `--print`, `-o`, absolute or `..` path, `.json` target), an environment without loader, shell-startup or rustc-control variables, only the declared `SEMIO_` variables, and the two virtual output paths. Answers `undefined` when the command is acceptable, otherwise the first reason. The compiler may still run the repository's own proc-macro and build-script code that the requester shipped as inputs: that is code of the group's own repository, inside the fleet's trust boundary (whoever writes the group store already changes the code every machine runs), and on a confining sandbox it runs without network and without access to the host. */
export function validateRemoteExecutionRustcActionV1(command:RemoteExecutionCommandViewV1):string|undefined{
 const [shell,option,script,name,...args]=command.arguments;
 if(shell!=="/bin/sh"||option!=="-c"||script!==policy.script||name!=="semio-rustc")return "only the declared rustc action may run on this worker";
 if(command.working_directory!=="")return "working directory not allowed";
 if((command.output_files?.length??0)>0||(command.output_directories?.length??0)>0)return "only output_paths are allowed";
 if(command.output_paths.length!==2||!command.output_paths.includes(virtual.out)||!command.output_paths.includes(virtual.root))return "output paths differ from the declared ones";
 if(args.length>20000)return "too many arguments";
 let sources=0;
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(!SAFE_TEXT.test(argument))return "argument is not plain text";
  if(argument.startsWith("@"))return "response files are not allowed";
  if(argument.endsWith(".rs")&&!argument.startsWith("-")){if(!relative(argument))return `source path ${argument.slice(0,80)}`;sources++;continue;}
  if(BARE.has(argument))continue;
  let reason:string|undefined;
  if(argument.startsWith("--")){
   const equals=argument.indexOf("="),flagName=equals<0?argument:argument.slice(0,equals);
   if(!WITH_VALUE.has(flagName))return `${flagName.slice(0,60)} is not allowed`;
   reason=flag(flagName,equals<0?args[++index]??"":argument.slice(equals+1));
  }else if(argument==="-C"||argument==="-L"||argument==="-Z"||argument==="-A"||argument==="-W"||argument==="-D"||argument==="-F")reason=flag(argument,args[++index]??"");
  else if(argument.startsWith("-C")||argument.startsWith("-L")||argument.startsWith("-Z"))reason=flag(argument.slice(0,2),argument.slice(2));
  else if(/^-[AWDF]/u.test(argument))reason=flag(argument.slice(0,2),argument.slice(2));
  else return `${argument.slice(0,60)} is not allowed`;
  if(reason!==undefined)return reason;
 }
 const names=new Set<string>();
 let absolute:string[]=[],tool=false,named=0;
 for(const {name:variable,value} of command.environment_variables){
  if(!/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(variable)||names.has(variable))return `environment name ${variable.slice(0,40)}`;
  names.add(variable);
  if(variable==="SEMIO_REMAPS"){
   const lines=value===""?[]:value.split("\n");
   if(lines.length>64||value.length>16384||lines.some(line=>{const equals=line.indexOf("=");return equals<1||!relative(line.slice(0,equals))||!REL_PATH.test(line.slice(0,equals))||!NAME_TEXT.test(line.slice(equals+1))||line.includes("\r")||line.includes("\0");}))return "SEMIO_REMAPS entries";
   continue;
  }
  if(variable.startsWith("CARGO_PKG_")){if(!MANIFEST_TEXT.test(value))return `environment value of ${variable}`;continue;}
  if(!SAFE_TEXT.test(value)||value.length>16384)return `environment value of ${variable}`;
  if(variable==="PATH"){if(value!==policy.environment.path)return "PATH differs from the declared one";continue;}
  if(variable==="SEMIO_TOOL_RUSTC"){if(value!==policy.tool.placeholder)return "tool placeholder";tool=true;continue;}
  if(variable==="SEMIO_CWD_NAME"){if(!NAME_TEXT.test(value))return "SEMIO_CWD_NAME";continue;}
  if(variable==="SEMIO_SOURCE"){if(!relative(value)||!value.endsWith(".rs")||!NAME_TEXT.test(value))return "SEMIO_SOURCE path";named++;continue;}
  if(variable==="SEMIO_ABSOLUTE"){absolute=value.split(" ").filter(part=>part!=="");if(!absolute.every(part=>ABSOLUTE_NAMES.has(part)))return "SEMIO_ABSOLUTE names";continue;}
  if(variable.startsWith("SEMIO_REL_")){if(!ABSOLUTE_NAMES.has(variable.slice("SEMIO_REL_".length))||!relative(value)||!REL_PATH.test(value))return `${variable} path`;continue;}
  if(variable.startsWith("SEMIO_"))return `${variable} is not allowed`;
  if(DENIED_ENVIRONMENT.has(variable)||DENIED_ENVIRONMENT_PREFIXES.some(prefix=>variable.startsWith(prefix))||policy.environment.deniedNames.includes(variable)&&!ABSOLUTE_NAMES.has(variable))return `${variable} is not allowed`;
  if(ABSOLUTE_NAMES.has(variable))return `${variable} must travel as SEMIO_REL_${variable}`;
 }
 if(sources+named!==1)return "exactly one source file required";
 if(!tool)return "tool placeholder missing";
 for(const part of absolute)if(!names.has(`SEMIO_REL_${part}`))return `SEMIO_REL_${part} missing`;
 return undefined;
}
