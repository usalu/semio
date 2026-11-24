import {createHash} from "node:crypto";
import {readFileSync,statSync} from "node:fs";
import {encodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import type {RemoteExecutionPropertyV1} from "../../🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadActionSecondsV1,offloadCrateTypesOfV1,offloadTargetOfV1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
import type {OffloadPlanV1} from "../🗺️paths/🟦️.ts";

const policy=OFFLOAD_POLICY_V1;
/** 🔢️ A digest in the protocol's own field names. */
export type OffloadDigestV1=Readonly<{hash:string;size_bytes:number}>;
/** 🎬️ One built and uploaded action: its digest and what it ships. */
export type OffloadActionV1=Readonly<{digest:OffloadDigestV1;files:number;bytes:number;uploaded:number}>;
/** 🖥️ The compiler identity a worker must already have: the commit hash and host triple of `rustc -vV`, the 16-hex toolchain digest the scheduler matches on (the first 16 hex of the SHA-256 of the exact `rustc -vV` text, the text the unit cache keys on) and the target triples of its sysroot. */
export type OffloadToolchainV1=Readonly<{commit:string;host:string;digest:string;targets:readonly string[]}>;

const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const digestOf=(data:Uint8Array):OffloadDigestV1=>({hash:sha(data),size_bytes:data.length});

/** 🏷️ The platform properties of an offloaded rustc action: the machine kind of the requester, its exact compiler (`semio-target`, `semio-toolchain`), the target libraries the unit needs when it is not the host, the machine it is pinned to (`semio-pin=<origin>`, which only that machine offers) and the control properties of the scheduler (`semio-offload=if-idle`, or `queue` when the unit waits for a machine instead of being declined, `semio-origin` and, for a unit with a predicted peak, `semio-peak-mb`). Sorted, so equal units are equal actions. */
export function offloadPlatformV1(host:readonly RemoteExecutionPropertyV1[],toolchain:OffloadToolchainV1,target:string|undefined,origin:string|undefined,peakMegabytes:number=0,placement:Readonly<{queue?:boolean;pin?:string;sdk?:string}>={}):readonly RemoteExecutionPropertyV1[]{
 const p=policy.property,rows=[...host.filter(row=>row.name!==p.target&&row.name!==p.toolchain),{name:p.offload,value:placement.queue===true||placement.pin!==undefined?p.queueValue:p.offloadValue},...placement.pin===undefined?[]:[{name:p.pin,value:placement.pin}],...placement.sdk===undefined?[]:[{name:p.sdk,value:placement.sdk}],{name:p.format,value:p.formatValue},{name:p.target,value:toolchain.host},{name:p.toolchain,value:toolchain.digest},...origin?[{name:p.origin,value:origin}]:[],...peakMegabytes>0?[{name:p.peak,value:String(Math.ceil(peakMegabytes))}]:[]];
 if(target!==undefined&&target!==toolchain.host)rows.push({name:p.targetPrefix+target,value:"1"});
 return rows.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0);
}

/** 🧱️ Reads the compiler identity out of `rustc -vV` text. */
export function parseRustcIdentityV1(text:string):Readonly<{commit:string;host:string;digest:string}>|undefined{
 const commit=/^commit-hash: ([0-9a-f]{40})$/mu.exec(text)?.[1],host=/^host: (\S+)$/mu.exec(text)?.[1];
 return commit&&host?{commit,host,digest:sha(text).slice(0,16)}:undefined;
}

/** 🪶️ The arguments of a `cdylib` final the way the build scheduler runs one locally: one front-end thread (`-Z threads=<n>` becomes `threads=1`). The LLVM back end is not decided here: the worker that runs the unit grants it code generation threads from its own free cores and memory (the schema's `worker.codegen`) and runs it serially (`-Z no-parallel-backend`) only when it grants one, so a final sent to an idle machine uses its cores. Output bytes do not depend on the thread count. Every other unit is returned unchanged. */
export function offloadFrontEndArgumentsV1(args:readonly string[]):readonly string[]{
 if(!offloadCrateTypesOfV1(args).includes("cdylib"))return args;
 const threads=(argument:string):string=>/^threads=[0-9]{1,3}$/u.test(argument)?"threads=1":argument;
 return args.map((argument,index)=>args[index-1]==="-Z"?threads(argument):argument.startsWith("-Zthreads=")?"-Z"+threads(argument.slice(2)):argument);
}

/** 🧬️ The command of one planned unit in argv form, the same on every operating system and run without a shell: the program `@tool:rustc`, the compiler arguments (the working-directory remap and, for a macOS host, the object-prefix flag first; the remaps of the shipped directories and the source of an absolute path last) and the environment, where every path below the worker's private input root is spelled `@root/<relative posix path>` and mapped to native separators by the worker. The platform key decides who may run it, never its encoding: remapped names are written with `/`, and no input carries an executable bit, so the same unit described from a POSIX and from a Windows requester has one digest. */
export function offloadRustcCommandV1(plan:OffloadPlanV1,host:string):Readonly<{arguments:readonly string[];environment:readonly (readonly [string,string])[]}>{
 const mark=policy.root.placeholder,env=plan.environment,remaps=(env.SEMIO_REMAPS??"").split("\n").filter(line=>line!==""),source=env.SEMIO_SOURCE;
 const spelled=(text:string):string=>text.replace(/\\/gu,"/"),rooted=(path:string):string=>path==="."||path===""?mark:`${mark}/${path}`;
 const absolute=(env.SEMIO_ABSOLUTE??"").split(" ").filter(part=>part!=="");
 const variables:[string,string][]=Object.entries(env).filter(([name])=>!name.startsWith("SEMIO_")&&!absolute.includes(name)).map(([name,value])=>[name,value]);
 for(const name of absolute)variables.push([name,rooted(env[`SEMIO_REL_${name}`]??"")]);
 variables.push(["HOME",rooted(policy.root.home)],[policy.root.report,policy.virtual.root]);
 const argumentsList=[`${policy.tool.placeholder}:${policy.tool.name}`,`--remap-path-prefix=${mark}=${spelled(env.SEMIO_CWD_NAME??"")}`,...host.includes("apple-darwin")&&(plan.target===undefined||plan.target.includes("-apple-"))?[`-Clink-arg=-Wl,-oso_prefix,${rooted(policy.virtual.out)}/`]:[],...offloadFrontEndArgumentsV1(plan.arguments),...remaps.map(entry=>{const equals=entry.indexOf("=");return `--remap-path-prefix=${rooted(entry.slice(0,equals))}=${spelled(entry.slice(equals+1))}`;}),...source===undefined?[]:[rooted(source)]];
 return Object.freeze({arguments:Object.freeze(argumentsList),environment:Object.freeze(variables.sort(([left],[right])=>left<right?-1:left>right?1:0))});
}

type Node={files:Map<string,{digest:OffloadDigestV1;executable:boolean}>;directories:Map<string,Node>};

/** 📦️ What every offloaded action ships, whatever it runs: the input files by virtual path (to the real file that holds them), the virtual paths of the files that carry the executable bit, the program and arguments in argv form, the environment, the declared output paths and the seconds the action may run. */
export type OffloadActionSpecV1=Readonly<{files:ReadonlyMap<string,string>;executable?:ReadonlySet<string>;arguments:readonly string[];environment:readonly (readonly [string,string])[];outputPaths:readonly string[];seconds:number}>;

/** 🚚️ Builds the action of one declared spec and uploads what the store lacks: the input tree (files by content digest, so equal files are stored once for the whole fleet), the command and the action. The one shipping path of the rustc offload and of the command offload. Resolves to a decline when an input cannot be read or the policy bounds are exceeded. */
export async function shipOffloadActionV1(input:Readonly<{spec:OffloadActionSpecV1;content:RemoteExecutionContentV1;instance:string;platform:readonly RemoteExecutionPropertyV1[];signal?:AbortSignal;digest?:(path:string)=>string|null}>):Promise<OffloadActionV1|Readonly<{declined:OffloadReasonV1;detail:string}>>{
 const {spec,content,instance,platform,signal}=input,blobs=new Map<string,{digest:OffloadDigestV1;path?:string;bytes?:Uint8Array}>(),root:Node={files:new Map(),directories:new Map()};
 let bytes=0;
 for(const [path,real] of [...spec.files].sort((left,right)=>left[0]<right[0]?-1:1)){
  let size:number,hash:string|null;
  try{const stat=statSync(real);size=stat.size;hash=input.digest?.(real)??sha(readFileSync(real));}catch{return{declined:"unreadable-input",detail:real};}
  if(hash===null)return{declined:"unreadable-input",detail:real};
  bytes+=size;
  const parts=path.split("/");let node=root;
  for(const part of parts.slice(0,-1)){let next=node.directories.get(part);if(!next){next={files:new Map(),directories:new Map()};node.directories.set(part,next);}node=next;}
  const digest={hash,size_bytes:size};node.files.set(parts.at(-1)!,{digest,executable:spec.executable?.has(path)===true});
  if(size>0&&!blobs.has(hash))blobs.set(hash,{digest,path:real});
 }
 const stage=(data:Uint8Array):OffloadDigestV1=>{const digest=digestOf(data);if(digest.size_bytes>0&&!blobs.has(digest.hash))blobs.set(digest.hash,{digest,bytes:data});return digest;};
 const build=(node:Node):OffloadDigestV1=>stage(encodeProtobufV1(REAPI_V1.directory,{
  files:[...node.files].sort((left,right)=>left[0]<right[0]?-1:1).map(([name,file])=>({name,digest:file.digest,is_executable:file.executable})),
  directories:[...node.directories].sort((left,right)=>left[0]<right[0]?-1:1).map(([name,child])=>({name,digest:build(child)})),
  symlinks:[],
 }));
 const rootDigest=build(root),properties=platform.map(row=>({name:row.name,value:row.value}));
 const command=stage(encodeProtobufV1(REAPI_V1.command,{
  arguments:[...spec.arguments],environment_variables:spec.environment.map(([name,value])=>({name,value})),
  output_files:[],output_directories:[],output_paths:[...spec.outputPaths],platform:{properties},working_directory:"",
 }));
 const action=stage(encodeProtobufV1(REAPI_V1.action,{command_digest:command,input_root_digest:rootDigest,timeout:{seconds:spec.seconds},platform:{properties}}));
 const absent=new Set((await content.missing(instance,[...blobs.values()].map(row=>({hash:row.digest.hash,size:row.digest.size_bytes})),signal,"input")).map(row=>row.hash));
 const rows=[...blobs.values()].filter(row=>absent.has(row.digest.hash));let next=0;
  const travelling=rows.reduce((sum,row)=>sum+row.digest.size_bytes,0);
 if(travelling>policy.limits.maximumUploadBytes)return{declined:"inputs-too-large",detail:`upload ${travelling}`};
 await Promise.all(Array.from({length:Math.min(8,rows.length)},async()=>{
  for(;;){const row=rows[next++];if(row===undefined)return;await content.write(instance,{hash:row.digest.hash,size:row.digest.size_bytes},row.bytes??readFileSync(row.path!),signal,"input");}
 }));
 return{digest:action,files:spec.files.size,bytes,uploaded:rows.length};
}

/** 🏗️ Builds the action of one planned rustc unit: its compiler command in argv form, the two virtual output paths and the time the unit may run, shipped like every offloaded action. */
export async function buildOffloadActionV1(input:Readonly<{plan:OffloadPlanV1;content:RemoteExecutionContentV1;instance:string;platform:readonly RemoteExecutionPropertyV1[];signal?:AbortSignal;digest?:(path:string)=>string|null}>):Promise<OffloadActionV1|Readonly<{declined:OffloadReasonV1;detail:string}>>{
 const {plan,platform}=input,built=offloadRustcCommandV1(plan,platform.find(row=>row.name===policy.property.target)?.value??"");
 return shipOffloadActionV1({spec:{files:plan.files,arguments:built.arguments,environment:built.environment,outputPaths:[policy.virtual.out,policy.virtual.root],seconds:offloadActionSecondsV1(offloadCrateTypesOfV1(plan.arguments),plan.target??offloadTargetOfV1(plan.arguments))},content:input.content,instance:input.instance,platform,...input.signal?{signal:input.signal}:{},...input.digest?{digest:input.digest}:{}});
}
