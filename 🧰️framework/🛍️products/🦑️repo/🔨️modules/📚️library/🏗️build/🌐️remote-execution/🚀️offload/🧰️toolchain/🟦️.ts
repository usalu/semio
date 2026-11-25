import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import type {RemoteExecutionPropertyV1} from "../../🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../📐️policy/🟦️.ts";
import {parseRustcIdentityV1,type OffloadToolchainV1} from "../📜️action/🟦️.ts";
import {sdkIdentityV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";

const policy=OFFLOAD_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🛠️ A tool the worker owns and the action may name: where it lives and which directories a confined action may read to use it. */
export type OffloadToolV1=Readonly<{path:string;readable:readonly string[]}>;
/** 🖥️ What a worker offers for rustc offload: the tool table and the platform properties that say which compiler it is. */
export type OffloadWorkerOfferV1=Readonly<{tools:Readonly<Record<string,OffloadToolV1>>;properties:readonly RemoteExecutionPropertyV1[];toolchain:OffloadToolchainV1}>;

const sdkMemo=new Map<string,string|undefined>();
/** 🍎️ The identity of the macOS SDK that a link on this machine uses (its declared version, `macosx27.2`, from the one function the unit key uses): `SDKROOT` when set, else the SDK `xcrun` reports; `undefined` off macOS or when none is found. Memoised per `SDKROOT`. */
export function offloadSdkIdentityV1(environment:Environment=process.env,platform:string=process.platform):string|undefined{
 if(platform!=="darwin")return undefined;
 const given=environment.SDKROOT??"";
 if(sdkMemo.has(given))return sdkMemo.get(given);
 let path=given;
 if(path===""){const run=spawnSync("xcrun",["--sdk","macosx","--show-sdk-path"],{encoding:"utf8",timeout:20000});path=run.status===0?run.stdout.trim():"";}
 const identity=path===""?undefined:sdkIdentityV1(path);
 sdkMemo.set(given,identity);return identity;
}

const LINKERS=new Set(["rust-lld","rust-lld.exe","wasm-component-ld","wasm-component-ld.exe"]);
const linkerMemo=new Map<string,{signature:string;identity:string|undefined}>();

/** 🔗️ Every file of the linkers a compiler brings (`lib/rustlib/<host>/bin`: `rust-lld`, `wasm-component-ld` and the `gcc-ld` wrappers; the optional LLVM tools beside them do not count) with its size and modification time, sorted: the cheap stamp that tells whether their identity has to be read again. */
function linkerFilesV1(sysroot:string):readonly Readonly<{path:string;size:number;mtimeMs:number}>[]{
 const rows:{path:string;size:number;mtimeMs:number}[]=[],walkAll=(directory:string):void=>{
  let names:string[];try{names=readdirSync(directory).sort();}catch{return;}
  for(const name of names){const path=join(directory,name);let info;try{info=statSync(path);}catch{continue;}if(info.isFile())rows.push({path,size:info.size,mtimeMs:info.mtimeMs});}
 },walk=(directory:string):void=>{
  let names:string[];try{names=readdirSync(directory).sort();}catch{return;}
  for(const name of names){const path=join(directory,name);let info;try{info=statSync(path);}catch{continue;}if(info.isDirectory()){if(name==="gcc-ld")walkAll(path);}else if(info.isFile()&&LINKERS.has(name))rows.push({path,size:info.size,mtimeMs:info.mtimeMs});}
 };
 const base=join(sysroot,"lib","rustlib");
 let hosts:string[];try{hosts=readdirSync(base).sort();}catch{return rows;}
 for(const host of hosts)walk(join(base,host,"bin"));
 return rows;
}

/** 🔗️ The identity of the linkers that consume the bitcode and objects of this compiler: the first 16 hex digits of the SHA-256 over the relative path and content digest of its linker files below `lib/rustlib/<host>/bin` (so a toolchain whose `rust-lld` or `wasm-component-ld` differs from the compiler's own, or is damaged, has another identity although `rustc -vV` reads the same); `undefined` when the sysroot has none. Memoised by file size and time, in the process and, when `memoRoot` is given, in `offload-linker.json` there so a build of hundreds of units reads the files once.
 * @see ./🧪️tests/🟦️.ts */
export function offloadLinkerIdentityV1(sysroot:string,memoRoot?:string):string|undefined{
 const files=linkerFilesV1(sysroot);
 if(files.length===0)return undefined;
 const signature=files.map(row=>`${row.path.slice(sysroot.length)}:${row.size}:${row.mtimeMs}`).join("\n"),known=linkerMemo.get(sysroot);
 const memoPath=memoRoot===undefined?undefined:join(memoRoot,"offload-linker.json");
 if(known?.signature===signature&&memoPath===undefined)return known.identity;
 if(memoPath!==undefined)try{const memo=JSON.parse(readFileSync(memoPath,"utf8")) as {sysroot:string;signature:string;identity:string};if(memo.sysroot===sysroot&&memo.signature===signature&&/^[0-9a-f]{16}$/u.test(memo.identity)){linkerMemo.set(sysroot,{signature,identity:memo.identity});return memo.identity;}}catch{}
 const hash=createHash("sha256");
 for(const row of files)hash.update(`${row.path.slice(sysroot.length).split("\\").join("/")}\0${createHash("sha256").update(readFileSync(row.path)).digest("hex")}\n`);
 const identity=hash.digest("hex").slice(0,16);
 linkerMemo.set(sysroot,{signature,identity});
 if(memoPath!==undefined)try{mkdirSync(memoRoot!,{recursive:true});const temporary=memoPath+"."+process.pid;writeFileSync(temporary,JSON.stringify({sysroot,signature,identity}));renameSync(temporary,memoPath);}catch{}
 return identity;
}

/** 🧭️ The sysroot of the compiler behind `inner` (`rustc --print sysroot`), memoised for ten minutes in `offload-sysroot.json` of the unit cache root like the version text; `undefined` when it does not answer. */
export function offloadSysrootV1(root:string,inner:readonly string[],environment:Environment,now:number=Date.now()):string|undefined{
 const path=join(root,"offload-sysroot.json"),identity=JSON.stringify(inner);
 try{const memo=JSON.parse(readFileSync(path,"utf8")) as {atMs:number;inner:string;sysroot:string};if(memo.inner===identity&&now-memo.atMs<600000&&now>=memo.atMs)return memo.sysroot;}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"--print","sysroot"],{windowsHide:true,encoding:"utf8",env:environment as NodeJS.ProcessEnv,timeout:20000}),text=result.status===0?result.stdout.trim():"";
 if(text==="")return undefined;
 try{mkdirSync(root,{recursive:true});const temporary=path+"."+process.pid;writeFileSync(temporary,JSON.stringify({atMs:now,inner:identity,sysroot:text}));renameSync(temporary,path);}catch{}
 return text;
}

/** 🎯️ The target triples a sysroot carries libraries for. */
export function sysrootTargetsV1(sysroot:string):readonly string[]{
 const base=join(sysroot,"lib","rustlib");
 try{return readdirSync(base).filter(name=>{try{return statSync(join(base,name)).isDirectory()&&existsSync(join(base,name,"lib"));}catch{return false;}}).sort();}catch{return[];}
}

/** 🔭️ Probes the compiler pinned by `cwd` and offers the executable in the reported sysroot for its host triple. The target libraries determine the target properties. */
export function detectWorkerRustcV1(cwd:string,environment:Environment=process.env,compiler:string|readonly [string,...string[]]="rustc"):OffloadWorkerOfferV1|undefined{
 const [tool,...prefix]=typeof compiler==="string"?[compiler]:compiler;
 const run=(args:readonly string[])=>spawnSync(tool,[...prefix,...args],{windowsHide:true,cwd,encoding:"utf8",env:environment as NodeJS.ProcessEnv,timeout:60000});
 const identity=run(["-vV"]),sysroot=run(["--print","sysroot"]);
 if(identity.status!==0||sysroot.status!==0)return undefined;
 const parsed=parseRustcIdentityV1(identity.stdout),root=sysroot.stdout.trim();
 if(!parsed||!root)return undefined;
 const executable=parsed.host.split("-").includes("windows")?policy.tool.windowsExecutable:policy.tool.name,path=join(root,"bin",executable);
 if(!existsSync(path))return undefined;
 const targets=sysrootTargetsV1(root),p=policy.property;
 return Object.freeze({
  tools:Object.freeze({[policy.tool.name]:Object.freeze({path,readable:Object.freeze([root])})}),
  toolchain:Object.freeze({...parsed,targets}),
  properties:Object.freeze([{name:p.format,value:p.formatValue},...offloadLinkerIdentityV1(root)===undefined?[]:[{name:p.linker,value:offloadLinkerIdentityV1(root)!}],...offloadSdkIdentityV1(environment)===undefined?[]:[{name:p.sdk,value:offloadSdkIdentityV1(environment)!}],...targets.map(target=>({name:p.targetPrefix+target,value:"1"}))]),
 });
}

/** 🧬️ The exact `rustc -vV` text of the compiler behind `inner`, memoised for ten minutes in the unit cache root so a build of hundreds of units probes it once; `undefined` when the probe fails. */
export function rustcVersionTextV1(root:string,inner:readonly string[],environment:Environment,now:number=Date.now()):string|undefined{
 const path=join(root,"offload-rustc.json"),identity=JSON.stringify(inner);
 try{const memo=JSON.parse(readFileSync(path,"utf8")) as {atMs:number;inner:string;text:string};if(memo.inner===identity&&now-memo.atMs<600000&&now>=memo.atMs)return memo.text;}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"-vV"],{windowsHide:true,encoding:"utf8",env:environment as NodeJS.ProcessEnv,timeout:20000});
 if(result.status!==0||!result.stdout)return undefined;
 try{mkdirSync(root,{recursive:true});const temporary=path+"."+process.pid;writeFileSync(temporary,JSON.stringify({atMs:now,inner:identity,text:result.stdout}));renameSync(temporary,path);}catch{}
 return result.stdout;
}
