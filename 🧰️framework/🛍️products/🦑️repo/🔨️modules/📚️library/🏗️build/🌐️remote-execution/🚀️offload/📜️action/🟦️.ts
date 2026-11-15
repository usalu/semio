import {createHash} from "node:crypto";
import {readFileSync,statSync} from "node:fs";
import {encodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import type {RemoteExecutionPropertyV1} from "../../🟦️.ts";
import {OFFLOAD_POLICY_V1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
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

/** 🏷️ The platform properties of an offloaded rustc action: the machine kind of the requester, its exact compiler (`semio-target`, `semio-toolchain`), the target libraries the unit needs when it is not the host, and the two control properties of the scheduler (`semio-offload=if-idle`, `semio-origin`). Sorted, so equal units are equal actions. */
export function offloadPlatformV1(host:readonly RemoteExecutionPropertyV1[],toolchain:OffloadToolchainV1,target:string|undefined,origin:string|undefined):readonly RemoteExecutionPropertyV1[]{
 const p=policy.property,rows=[...host.filter(row=>row.name!==p.target&&row.name!==p.toolchain),{name:p.offload,value:p.offloadValue},{name:p.target,value:toolchain.host},{name:p.toolchain,value:toolchain.digest},...origin?[{name:p.origin,value:origin}]:[]];
 if(target!==undefined&&target!==toolchain.host)rows.push({name:p.targetPrefix+target,value:"1"});
 return rows.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0);
}

/** 🧱️ Reads the compiler identity out of `rustc -vV` text. */
export function parseRustcIdentityV1(text:string):Readonly<{commit:string;host:string;digest:string}>|undefined{
 const commit=/^commit-hash: ([0-9a-f]{40})$/mu.exec(text)?.[1],host=/^host: (\S+)$/mu.exec(text)?.[1];
 return commit&&host?{commit,host,digest:sha(text).slice(0,16)}:undefined;
}

type Node={files:Map<string,{digest:OffloadDigestV1;executable:boolean}>;directories:Map<string,Node>};

/** 🏗️ Builds the action of one planned unit and uploads what the store lacks: the input tree (files by content digest, so equal files are stored once for the whole fleet), the command and the action. Resolves to a decline when an input cannot be read or the policy bounds are exceeded. */
export async function buildOffloadActionV1(input:Readonly<{plan:OffloadPlanV1;content:RemoteExecutionContentV1;instance:string;platform:readonly RemoteExecutionPropertyV1[];signal?:AbortSignal;digest?:(path:string)=>string|null}>):Promise<OffloadActionV1|Readonly<{declined:OffloadReasonV1;detail:string}>>{
 const {plan,content,instance,platform,signal}=input,blobs=new Map<string,{digest:OffloadDigestV1;path?:string;bytes?:Uint8Array}>(),root:Node={files:new Map(),directories:new Map()};
 let bytes=0;
 for(const [path,real] of [...plan.files].sort((left,right)=>left[0]<right[0]?-1:1)){
  let size:number,mode:number,hash:string|null;
  try{const stat=statSync(real);size=stat.size;mode=stat.mode;hash=input.digest?.(real)??sha(readFileSync(real));}catch{return{declined:"unreadable-input",detail:real};}
  if(hash===null)return{declined:"unreadable-input",detail:real};
  bytes+=size;if(bytes>policy.limits.maximumInputBytes)return{declined:"inputs-too-large",detail:String(bytes)};
  const parts=path.split("/");let node=root;
  for(const part of parts.slice(0,-1)){let next=node.directories.get(part);if(!next){next={files:new Map(),directories:new Map()};node.directories.set(part,next);}node=next;}
  const digest={hash,size_bytes:size};node.files.set(parts.at(-1)!,{digest,executable:(mode&0o111)!==0});
  if(size>0&&!blobs.has(hash))blobs.set(hash,{digest,path:real});
 }
 const stage=(data:Uint8Array):OffloadDigestV1=>{const digest=digestOf(data);if(digest.size_bytes>0&&!blobs.has(digest.hash))blobs.set(digest.hash,{digest,bytes:data});return digest;};
 const build=(node:Node):OffloadDigestV1=>stage(encodeProtobufV1(REAPI_V1.directory,{
  files:[...node.files].sort((left,right)=>left[0]<right[0]?-1:1).map(([name,file])=>({name,digest:file.digest,is_executable:file.executable})),
  directories:[...node.directories].sort((left,right)=>left[0]<right[0]?-1:1).map(([name,child])=>({name,digest:build(child)})),
  symlinks:[],
 }));
 const rootDigest=build(root),properties=platform.map(row=>({name:row.name,value:row.value}));
 const argumentsList=["/bin/sh","-c",policy.script,"semio-rustc",...plan.arguments];
 const command=stage(encodeProtobufV1(REAPI_V1.command,{
  arguments:argumentsList,environment_variables:Object.entries(plan.environment).sort(([left],[right])=>left<right?-1:1).map(([name,value])=>({name,value})),
  output_files:[],output_directories:[],output_paths:[policy.virtual.out,policy.virtual.root],platform:{properties},working_directory:"",
 }));
 const action=stage(encodeProtobufV1(REAPI_V1.action,{command_digest:command,input_root_digest:rootDigest,timeout:{seconds:policy.timeouts.actionSeconds},platform:{properties}}));
 const absent=new Set((await content.missing(instance,[...blobs.values()].map(row=>({hash:row.digest.hash,size:row.digest.size_bytes})),signal)).map(row=>row.hash));
 const rows=[...blobs.values()].filter(row=>absent.has(row.digest.hash));let next=0;
 await Promise.all(Array.from({length:Math.min(8,rows.length)},async()=>{
  for(;;){const row=rows[next++];if(row===undefined)return;await content.write(instance,{hash:row.digest.hash,size:row.digest.size_bytes},row.bytes??readFileSync(row.path!),signal);}
 }));
 return{digest:action,files:plan.files.size,bytes,uploaded:rows.length};
}
