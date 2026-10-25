import {createHash} from "node:crypto";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {connectGrpcV1,type GrpcClientV1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {startRemoteExecutionServerV1,type RemoteExecutionServerOptionsV1,type RemoteExecutionServerV1,type RemoteExecutionTokensV1} from "../../🖥️server/🟦️.ts";
import {runRemoteExecutionWorkerV1,type RemoteExecutionWorkerEventV1,type RemoteExecutionWorkerOptionsV1} from "../../👷️worker/🟦️.ts";

/** 🧰️ A complete in-process service for tests: a real cache server, the execution server with its scheduler and a gRPC client, plus any number of in-process workers. */
export const NAMESPACE_V1="build.bazel.remote.execution.v2";
const sha=(data:Uint8Array|string)=>createHash("sha256").update(data).digest("hex");

/** 📁️ A tree an action receives: file contents (optionally executable) and symbolic links, by relative path. */
export type ClusterTreeV1=Readonly<Record<string,string|Uint8Array|Readonly<{content?:string|Uint8Array;executable?:boolean;symlink?:string}>>>;

/** 📜️ What a client declares for one action. */
export type ClusterActionV1=Readonly<{arguments:readonly string[];files?:ClusterTreeV1;environment?:Readonly<Record<string,string>>;outputFiles?:readonly string[];outputDirectories?:readonly string[];outputPaths?:readonly string[];platform?:readonly {name:string;value:string}[];timeoutSeconds?:number;workingDirectory?:string;salt?:string;skipUpload?:boolean}>;

/** 🔢️ A digest in the protocol's own field names. */
export type ClusterDigestV1={hash:string;size_bytes:number};

/** 🎛️ The harness. */
export interface ClusterV1{
 readonly directory:string;readonly cache:RemoteCacheServerV1;readonly server:RemoteExecutionServerV1;readonly client:GrpcClientV1;readonly content:RemoteExecutionContentV1;readonly events:string[];readonly workerEvents:string[];
 startWorker(options?:Partial<RemoteExecutionWorkerOptionsV1>):{stop():Promise<void>;done:Promise<void>};
 upload(bytes:Uint8Array):Promise<ClusterDigestV1>;
 action(spec:ClusterActionV1):Promise<ClusterDigestV1>;
 execute(action:ClusterDigestV1,options?:{skipCacheLookup?:boolean;signal?:AbortSignal}):Promise<{stages:string[];response:ProtobufMessageV1;operation:string}>;
 stop():Promise<void>;
}

/** 🧱️ Uploads a tree and answers its root digest. */
async function treeDigest(upload:(bytes:Uint8Array)=>Promise<ClusterDigestV1>,tree:ClusterTreeV1):Promise<ClusterDigestV1>{
 interface Node{files:Record<string,{content:Uint8Array;executable:boolean}>;links:Record<string,string>;dirs:Record<string,Node>}
 const root:Node={files:{},links:{},dirs:{}};
 for(const [path,value] of Object.entries(tree)){
  const parts=path.split("/");let node=root;
  for(const part of parts.slice(0,-1))node=node.dirs[part]??={files:{},links:{},dirs:{}};
  const name=parts.at(-1)!;
  if(typeof value==="string"||value instanceof Uint8Array)node.files[name]={content:typeof value==="string"?new TextEncoder().encode(value):value,executable:false};
  else if(value.symlink!==undefined)node.links[name]=value.symlink;
  else node.files[name]={content:typeof value.content==="string"?new TextEncoder().encode(value.content):value.content??new Uint8Array(0),executable:value.executable===true};
 }
 const build=async(node:Node):Promise<ClusterDigestV1>=>{
  const files=[],directories=[],symlinks=[];
  for(const name of Object.keys(node.files).sort()){const file=node.files[name]!;files.push({name,digest:await upload(file.content),is_executable:file.executable});}
  for(const name of Object.keys(node.dirs).sort())directories.push({name,digest:await build(node.dirs[name]!)});
  for(const name of Object.keys(node.links).sort())symlinks.push({name,target:node.links[name]!});
  return upload(encodeProtobufV1(REAPI_V1.directory,{files,directories,symlinks}));
 };
 return build(root);
}

/** 🚀️ Starts the harness; `workers` in-process workers are started with the given options. */
export async function startClusterV1(options:Readonly<{workers?:number;bind?:string;advertise?:string;worker?:Partial<RemoteExecutionWorkerOptionsV1>;tokens?:RemoteExecutionTokensV1;scheduler?:Partial<RemoteExecutionServerOptionsV1["scheduler"]>;server?:Partial<RemoteExecutionServerOptionsV1>;root?:string}>={}):Promise<ClusterV1>{
 const directory=mkdtempSync(join(options.root??testScratchRootV1("rexec-inhouse"),"cluster-")),events:string[]=[],workerEvents:string[]=[],stoppers:(()=>Promise<void>)[]=[];
 mkdirSync(directory,{recursive:true});
 const token=options.tokens?.write??options.tokens?.read;
 const bind=options.bind??"127.0.0.1",cache=await startRemoteCacheServerV1({directory:join(directory,"cache"),host:bind,port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:30,defaultInstance:"main",tokens:{read:options.tokens?.read,write:options.tokens?.write}});
 const content=httpRemoteExecutionContentV1({endpoint:cache.url.replace("//0.0.0.0","//127.0.0.1"),token});
 const server=await startRemoteExecutionServerV1({host:bind,grpcPort:0,workerPort:0,content,cacheUrl:cache.url,advertise:options.advertise,tokens:options.tokens??{},maximumMessageBytes:1<<25,maximumBatchBytes:1<<22,
  scheduler:{unmatchedSeconds:4,lingerSeconds:1,leaseSeconds:6,maximumAttempts:3,defaultActionSeconds:60,maximumQueued:1000,heartbeatSeconds:1,pollSeconds:2,sweepMilliseconds:250,...options.scheduler},
  onEvent:event=>{events.push(`${event.stage} ${event.detail}`);},...options.server});
 const client=connectGrpcV1(server.grpcUrl),metadata:Record<string,string>=token?{authorization:`Bearer ${token}`}:{};
 const upload=async(bytes:Uint8Array):Promise<ClusterDigestV1>=>{
  const digest={hash:sha(bytes),size_bytes:bytes.length};
  if(bytes.length>0){const answer=await client.unary(`${NAMESPACE_V1}.ContentAddressableStorage/BatchUpdateBlobs`,{instance_name:"main",requests:[{digest,data:bytes}]},{metadata});if(answer.responses[0].status.code!==0)throw Error(answer.responses[0].status.message);}
  return digest;
 };
 const harness:ClusterV1={
  directory,cache,server,client,content,events,workerEvents,upload,
  startWorker:(extra={})=>{
   const abort=new AbortController(),name=extra.name??`worker-${stoppers.length+1}`,done=runRemoteExecutionWorkerV1({scheduler:server.workerUrl,token:options.tokens?.worker,cacheToken:token,concurrency:2,directory:join(directory,"worker-"+name),sandbox:"auto",materialize:"clone",maximumBlobBytes:1<<30,inlineOutputBytes:4096,signal:abort.signal,name,onEvent:(event:RemoteExecutionWorkerEventV1)=>{workerEvents.push(`${name} ${event.stage} ${event.detail}`);},...options.worker,...extra});
   const stop=async()=>{abort.abort();await done;};stoppers.push(stop);return {stop,done};
  },
  action:async spec=>{
   const input=await treeDigest(upload,spec.files??{}),command=await upload(encodeProtobufV1(REAPI_V1.command,{
    arguments:spec.arguments,environment_variables:Object.entries(spec.environment??{}).sort().map(([name,value])=>({name,value})),output_files:spec.outputFiles??[],output_directories:spec.outputDirectories??[],output_paths:spec.outputPaths??[],
    platform:{properties:spec.platform??[]},working_directory:spec.workingDirectory??"",
   }));
   return upload(encodeProtobufV1(REAPI_V1.action,{command_digest:command,input_root_digest:input,...(spec.timeoutSeconds?{timeout:{seconds:spec.timeoutSeconds}}:{}),...(spec.salt?{salt:new TextEncoder().encode(spec.salt)}:{}),platform:{properties:spec.platform??[]}}));
  },
  execute:async(action,run={})=>{
   const stages:string[]=[];let response:ProtobufMessageV1|undefined,operation="";
   for await(const row of client.stream(`${NAMESPACE_V1}.Execution/Execute`,{instance_name:"main",action_digest:action,skip_cache_lookup:run.skipCacheLookup===true},{metadata,...(run.signal?{signal:run.signal}:{})})){
    operation=row.name;stages.push((["UNKNOWN","CACHE_CHECK","QUEUED","EXECUTING","COMPLETED"] as const)[decodeProtobufV1(REAPI_V1.executeMetadata,row.metadata.value).stage]!);
    if(row.done)response=decodeProtobufV1(REAPI_V1.executeResponse,row.response.value);
   }
   if(!response)throw Error("Execution ended without a response");
   return {stages,response,operation};
  },
  stop:async()=>{
   for(const stop of stoppers)await stop().catch(()=>{});
   client.close();await server.stop().catch(()=>{});await cache.stop().catch(()=>{});
   rmSync(directory,{recursive:true,force:true});
  },
 };
 for(let index=0;index<(options.workers??0);index++)harness.startWorker({name:`worker-${index+1}`});
 return harness;
}

/** ⏳️ Polls until the condition holds. */
export async function eventuallyV1(condition:()=>boolean|Promise<boolean>,milliseconds=15000,message="Condition not reached"):Promise<void>{
 const deadline=Date.now()+milliseconds;
 while(Date.now()<deadline){if(await condition())return;await Bun.sleep(50);}
 throw Error(message);
}

/** 📄️ The text of an inline stream of an ActionResult. */
export const textOfV1=(bytes:Uint8Array|undefined):string=>new TextDecoder().decode(bytes??new Uint8Array(0));
