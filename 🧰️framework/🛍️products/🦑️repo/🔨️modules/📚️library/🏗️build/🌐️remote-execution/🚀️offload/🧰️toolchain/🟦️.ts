import {spawnSync} from "node:child_process";
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
/** 🍎️ The identity of the macOS SDK that a link on this machine uses (the name of the SDK directory, as the unit key sees it): `SDKROOT` when set, else the SDK `xcrun` reports; `undefined` off macOS or when none is found. Memoised per `SDKROOT`. */
export function offloadSdkIdentityV1(environment:Environment=process.env,platform:string=process.platform):string|undefined{
 if(platform!=="darwin")return undefined;
 const given=environment.SDKROOT??"";
 if(sdkMemo.has(given))return sdkMemo.get(given);
 let path=given;
 if(path===""){const run=spawnSync("xcrun",["--sdk","macosx","--show-sdk-path"],{encoding:"utf8",timeout:20000});path=run.status===0?run.stdout.trim():"";}
 const identity=path===""?undefined:sdkIdentityV1(path);
 sdkMemo.set(given,identity);return identity;
}

/** 🎯️ The target triples a sysroot carries libraries for. */
export function sysrootTargetsV1(sysroot:string):readonly string[]{
 const base=join(sysroot,"lib","rustlib");
 try{return readdirSync(base).filter(name=>{try{return statSync(join(base,name)).isDirectory()&&existsSync(join(base,name,"lib"));}catch{return false;}}).sort();}catch{return[];}
}

/** 🔭️ Probes the compiler pinned by `cwd` and offers the executable in the reported sysroot for its host triple. The target libraries determine the target properties. */
export function detectWorkerRustcV1(cwd:string,environment:Environment=process.env,compiler:string|readonly [string,...string[]]="rustc"):OffloadWorkerOfferV1|undefined{
 const [tool,...prefix]=typeof compiler==="string"?[compiler]:compiler;
 const run=(args:readonly string[])=>spawnSync(tool,[...prefix,...args],{cwd,encoding:"utf8",env:environment as NodeJS.ProcessEnv,timeout:60000});
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
  properties:Object.freeze([{name:p.format,value:p.formatValue},...offloadSdkIdentityV1(environment)===undefined?[]:[{name:p.sdk,value:offloadSdkIdentityV1(environment)!}],...targets.map(target=>({name:p.targetPrefix+target,value:"1"}))]),
 });
}

/** 🧬️ The exact `rustc -vV` text of the compiler behind `inner`, memoised for ten minutes in the unit cache root so a build of hundreds of units probes it once; `undefined` when the probe fails. */
export function rustcVersionTextV1(root:string,inner:readonly string[],environment:Environment,now:number=Date.now()):string|undefined{
 const path=join(root,"offload-rustc.json"),identity=JSON.stringify(inner);
 try{const memo=JSON.parse(readFileSync(path,"utf8")) as {atMs:number;inner:string;text:string};if(memo.inner===identity&&now-memo.atMs<600000&&now>=memo.atMs)return memo.text;}catch{}
 const result=spawnSync(inner[0]!,[...inner.slice(1),"-vV"],{encoding:"utf8",env:environment as NodeJS.ProcessEnv,timeout:20000});
 if(result.status!==0||!result.stdout)return undefined;
 try{mkdirSync(root,{recursive:true});const temporary=path+"."+process.pid;writeFileSync(temporary,JSON.stringify({atMs:now,inner:identity,text:result.stdout}));renameSync(temporary,path);}catch{}
 return result.stdout;
}
