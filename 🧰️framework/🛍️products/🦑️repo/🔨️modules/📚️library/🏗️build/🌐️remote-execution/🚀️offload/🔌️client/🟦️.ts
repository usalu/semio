import {chmodSync,mkdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {decodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import {connectGrpcV1,GrpcErrorV1,GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {OFFLOAD_POLICY_V1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
import type {OffloadDigestV1} from "../📜️action/🟦️.ts";

const policy=OFFLOAD_POLICY_V1,timeouts=policy.timeouts,NAMESPACE="build.bazel.remote.execution.v2";
const STAGES=["UNKNOWN","CACHE_CHECK","QUEUED","EXECUTING","COMPLETED"] as const;
/** 📣️ The stage an offloaded action reached, for progress. */
export type OffloadStageV1=typeof STAGES[number];
/** 🏁️ How one remote execution ended: a result, or a reason to stay local. */
export type OffloadExecutionV1=Readonly<{kind:"done";actionResult:ProtobufMessageV1;cached:boolean;origin:string}>|Readonly<{kind:"declined";reason:OffloadReasonV1;detail:string;transient?:true}>;

const declined=(reason:OffloadReasonV1,detail:string,transient=false):OffloadExecutionV1=>({kind:"declined",reason,detail,...(transient?{transient:true as const}:{})});
/** 🔌️ Submits an action once with `semio-offload=if-idle` semantics and waits for it under per-phase timeouts: connecting, being taken by an idle machine, running. A refusal for lack of an idle machine, a timeout or any fault is a decline the caller answers by compiling here; a cancelled `signal` closes the call, which cancels the remote action. */
export async function executeOffloadV1(input:Readonly<{endpoint:string;token:string|undefined;sign?:(method:string,content:string)=>Record<string,string>;instance:string;action:OffloadDigestV1;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;queueMilliseconds?:number;executeMilliseconds?:number;connectMilliseconds?:number}>):Promise<OffloadExecutionV1>{
 const controller=new AbortController(),client=connectGrpcV1(input.endpoint,input.sign===undefined?{}:{sign:input.sign}),cancel=()=>controller.abort();
 input.signal?.addEventListener("abort",cancel,{once:true});
 let phase:"connect"|"queue"|"execute"="connect",expired=false;
 const limits={connect:input.connectMilliseconds??timeouts.connectMilliseconds,queue:input.queueMilliseconds??timeouts.queueMilliseconds,execute:input.executeMilliseconds??timeouts.executeMilliseconds};
 let timer:ReturnType<typeof setTimeout>|undefined;
 const arm=(next:typeof phase):void=>{phase=next;if(timer)clearTimeout(timer);timer=setTimeout(()=>{expired=true;controller.abort();},limits[next]);};
 arm("connect");
 try{
  let response:ProtobufMessageV1|undefined;
  const metadata:Record<string,string>=input.token?{authorization:`Bearer ${input.token}`}:{};
  for await(const row of client.stream(`${NAMESPACE}.Execution/Execute`,{instance_name:input.instance,action_digest:input.action,skip_cache_lookup:false},{metadata,signal:controller.signal})){
   const stage=STAGES[decodeProtobufV1(REAPI_V1.executeMetadata,row.metadata.value).stage]??"UNKNOWN";
   input.onStage?.(stage);
   if(stage==="CACHE_CHECK"||stage==="QUEUED"){if(phase==="connect")arm("queue");}
   else if(stage==="EXECUTING")arm("execute");
   if(row.done)response=decodeProtobufV1(REAPI_V1.executeResponse,row.response.value);
  }
  if(input.signal?.aborted)return declined("cancelled","");
  if(!response)return declined("infrastructure","no response",true);
  const code=response.status?.code??0;
  if(code!==0&&String(response.status?.message??"").includes("Action refused by the worker policy"))return declined("worker-policy",String(response.status?.message).slice(0,200));
  if(code===GRPC_CODES_V1.RESOURCE_EXHAUSTED||code===GRPC_CODES_V1.FAILED_PRECONDITION)return declined("no-idle-machine",response.status?.message??"");
  if(code!==0)return declined("infrastructure",`${code} ${response.status?.message??""}`);
  const result=response.result;
  if(!result)return declined("infrastructure","no result");
  const worker=/^executed by (.+)$/u.exec(String(response.message??""))?.[1]??"",cached=response.cached_result===true;
  return{kind:"done",actionResult:result,cached,origin:cached?`${worker||"fleet"} (cache)`:worker||"fleet"};
 }catch(error){
  if(input.signal?.aborted)return declined("cancelled","");
  if(expired)return declined(phase==="execute"?"infrastructure":"queue-timeout",phase);
  if(error instanceof GrpcErrorV1)return declined(error.code===GRPC_CODES_V1.RESOURCE_EXHAUSTED||error.code===GRPC_CODES_V1.FAILED_PRECONDITION?"no-idle-machine":"infrastructure",`${error.code} ${error.message}`,error.code===GRPC_CODES_V1.UNAVAILABLE);
  return declined("infrastructure",String((error as Error)?.message??error),true);
 }finally{
  if(timer)clearTimeout(timer);
  input.signal?.removeEventListener("abort",cancel);
  controller.abort();
  client.close();
 }
}

const wired=(digest:{hash:string;size_bytes:number}):{hash:string;size:number}=>({hash:digest.hash,size:digest.size_bytes});
/** 📥️ What an action wrote: the files below one output directory by relative path, the content of single output files, and the compiler's streams. */
export type OffloadOutputsV1=Readonly<{directory:ReadonlyMap<string,Readonly<{digest:OffloadDigestV1;executable:boolean}>>;files:ReadonlyMap<string,Uint8Array>;stdout:Buffer;stderr:Buffer;exitCode:number}>;

/** 📚️ Reads an action result into memory-light descriptors: the tree of the output directory (directories are blobs of the store), small single files and both streams. */
export async function readOffloadOutputsV1(content:RemoteExecutionContentV1,instance:string,actionResult:ProtobufMessageV1,outputDirectory:string,singleFiles:readonly string[],signal?:AbortSignal):Promise<OffloadOutputsV1>{
 const directory=new Map<string,{digest:OffloadDigestV1;executable:boolean}>(),files=new Map<string,Uint8Array>();
 const walk=async(digest:OffloadDigestV1,prefix:string):Promise<void>=>{
  const bytes=digest.size_bytes===0?new Uint8Array(0):await content.read(instance,wired(digest),signal);
  if(!bytes)throw Error("Output directory blob missing");
  const node=decodeProtobufV1(REAPI_V1.directory,bytes);
  for(const file of node.files as {name:string;digest:OffloadDigestV1;is_executable:boolean}[])directory.set(prefix+file.name,{digest:file.digest,executable:file.is_executable});
  for(const child of node.directories as {name:string;digest:OffloadDigestV1}[])await walk(child.digest,prefix+child.name+"/");
 };
 for(const row of actionResult.output_directories as {path:string;root_directory_digest:OffloadDigestV1}[])if(row.path===outputDirectory)await walk(row.root_directory_digest,"");
 for(const row of actionResult.output_files as {path:string;digest:OffloadDigestV1}[])if(singleFiles.includes(row.path))files.set(row.path,row.digest.size_bytes===0?new Uint8Array(0):await content.read(instance,wired(row.digest),signal)??new Uint8Array(0));
 const stream=async(raw:Uint8Array,digest:OffloadDigestV1|undefined):Promise<Buffer>=>raw.length>0||!digest||digest.size_bytes===0?Buffer.from(raw):Buffer.from(await content.read(instance,wired(digest),signal)??new Uint8Array(0));
 return{directory,files,stdout:await stream(actionResult.stdout_raw,actionResult.stdout_digest),stderr:await stream(actionResult.stderr_raw,actionResult.stderr_digest),exitCode:actionResult.exit_code};
}

/** 💾️ Writes the output files into a directory the way a local compilation leaves them: atomically per file, executable bit kept. Bytes already read are checked against their digest by the store client. */
export async function writeOffloadOutputsV1(content:RemoteExecutionContentV1,instance:string,outputs:OffloadOutputsV1,destination:string,transform:(name:string,bytes:Buffer)=>Buffer,signal?:AbortSignal):Promise<readonly string[]>{
 const written:string[]=[];let next=0;const rows=[...outputs.directory];
 mkdirSync(destination,{recursive:true});
 await Promise.all(Array.from({length:Math.min(8,rows.length)},async()=>{
  for(;;){
   const row=rows[next++];if(row===undefined)return;
   const [name,file]=row,bytes=file.digest.size_bytes===0?Buffer.alloc(0):Buffer.from(await content.read(instance,wired(file.digest),signal)??(()=>{throw Error("Output blob missing: "+name);})()),target=join(destination,name),temporary=target+"."+randomBytes(4).toString("hex")+".tmp";
   mkdirSync(dirname(target),{recursive:true});
   try{writeFileSync(temporary,transform(name,bytes));chmodSync(temporary,file.executable?0o755:0o644);rmSync(target,{force:true});renameSync(temporary,target);}catch(error){rmSync(temporary,{force:true});throw error;}
   written.push(target);
  }
 }));
 return written.sort();
}

/** 🛟️ Submits like `executeOffloadV1` and, when the call is lost to the transport (the scheduler restarts: connection refused or reset, the stream ends without an answer), submits the same action again with jittered back-off for `budgetMilliseconds`. The scheduler deduplicates by action digest (a worker that kept running the action re-joins and the scheduler adopts it) and answers an action that already finished from the action cache, so the retry never runs the unit a second time; once the budget is spent the caller compiles locally as before. A refusal, a timeout or a cancel is never retried. */
export async function executeOffloadRidingThroughV1(input:Parameters<typeof executeOffloadV1>[0],budgetMilliseconds:number,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise<void>(done=>{setTimeout(done,milliseconds);})):Promise<OffloadExecutionV1>{
 const until=Date.now()+budgetMilliseconds;
 let execution=await executeOffloadV1(input);
 for(let attempt=0;execution.kind==="declined"&&execution.transient===true&&input.signal?.aborted!==true&&Date.now()<until;attempt++){
  await pause(Math.min(2000,250*2**attempt)*(0.5+Math.random()));
  execution=await executeOffloadV1(input);
 }
 return execution;
}
