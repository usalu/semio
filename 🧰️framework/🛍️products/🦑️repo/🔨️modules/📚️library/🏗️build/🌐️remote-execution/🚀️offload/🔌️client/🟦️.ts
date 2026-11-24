import {chmodSync,mkdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {decodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import {connectGrpcV1,GRPC_CODES_V1,GrpcErrorV1,type GrpcClientV1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {remoteExecutionPeakOfMetadataV1,remoteExecutionPhasesOfMetadataV1,remoteExecutionThreadsOfMetadataV1,type RemoteExecutionPhasesV1} from "../../🟦️.ts";
import {classifyOffloadStatusV1} from "../🩺️failure/🟦️.ts";
import {OFFLOAD_POLICY_V1,type OffloadReasonV1} from "../📐️policy/🟦️.ts";
import type {OffloadDigestV1} from "../📜️action/🟦️.ts";

const policy=OFFLOAD_POLICY_V1,timeouts=policy.timeouts,deadlines=policy.deadline,NAMESPACE="build.bazel.remote.execution.v2";
const STAGES=["UNKNOWN","CACHE_CHECK","QUEUED","EXECUTING","COMPLETED"] as const;
/** 📣️ The stage an offloaded action reached, for progress. */
export type OffloadStageV1=typeof STAGES[number];
/** 🏁️ How one remote execution ended: a result, or a reason to stay local. */
export type OffloadExecutionV1=Readonly<{kind:"done";actionResult:ProtobufMessageV1;cached:boolean;origin:string}>|Readonly<{kind:"declined";reason:OffloadReasonV1;detail:string;transient?:true}>;

/** ⏳️ How long the client waits for one action overall, from submitting to the answer: the time the action itself may run (`actionSeconds`) plus a small margin (`executeMarginMilliseconds`); the wait is derived from the action timeout and can never outlast it by more than the margin. */
export function offloadExecuteBudgetMillisecondsV1(actionSeconds:number=timeouts.actionSeconds):number{return actionSeconds*1000+timeouts.executeMarginMilliseconds;}

const declined=(reason:OffloadReasonV1,detail:string,transient=false):OffloadExecutionV1=>({kind:"declined",reason,detail,...(transient?{transient:true as const}:{})});
/** 🔌️ Submits an action once with `semio-offload=if-idle` semantics and waits for it under per-phase timeouts: connecting, being taken by an idle machine, running. The wait for the answer never outlasts three limits: the overall wait (`executeMilliseconds`), the deadline of the unit (`deadlineMilliseconds`, counted from the moment it executes and never restarted when the scheduler hands it to another machine) and the silence of the server: the server repeats its operation every few seconds (`keepAliveSeconds`), so a stream that stays silent for `silenceMilliseconds` is closed and the client re-attaches to the operation by name (`WaitExecution`) on a new connection, as often as the server answers that attach and the other limits hold; an operation the server no longer knows or an attach that stays silent ends the wait. A refusal for lack of an idle machine, a timeout, a lost operation or any fault is a decline the caller answers by compiling here; a cancelled `signal` closes the call, which cancels the remote action. */
export async function executeOffloadV1(input:Readonly<{endpoint:string;token:string|undefined;sign?:(method:string,content:string)=>Record<string,string>;instance:string;action:OffloadDigestV1;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;queueMilliseconds?:number;executeMilliseconds?:number;connectMilliseconds?:number;deadlineMilliseconds?:number;silenceMilliseconds?:number;reattachMilliseconds?:number}>):Promise<OffloadExecutionV1>{
 const controller=new AbortController(),connect=():GrpcClientV1=>connectGrpcV1(input.endpoint,input.sign===undefined?{}:{sign:input.sign}),cancel=()=>controller.abort();
 let client=connect();
 input.signal?.addEventListener("abort",cancel,{once:true});
 let phase="connect" as "connect"|"queue"|"execute",expired=false,deadlineAt:number|undefined;
 const limits={connect:input.connectMilliseconds??timeouts.connectMilliseconds,queue:input.queueMilliseconds??timeouts.queueMilliseconds,execute:input.executeMilliseconds??offloadExecuteBudgetMillisecondsV1()},submitted=Date.now(),silenceLimit=input.silenceMilliseconds??deadlines.silenceMilliseconds,reattachLimit=input.reattachMilliseconds??deadlines.reattachMilliseconds;
 let timer:ReturnType<typeof setTimeout>|undefined;
 const arm=(next:typeof phase):void=>{
  phase=next;if(timer)clearTimeout(timer);
  let wait=Math.min(limits[next],submitted+limits.execute-Date.now());
  if(next==="execute"){deadlineAt??=input.deadlineMilliseconds===undefined?undefined:Date.now()+input.deadlineMilliseconds;if(deadlineAt!==undefined)wait=Math.min(wait,deadlineAt-Date.now());}
  timer=setTimeout(()=>{expired=true;controller.abort();},Math.max(1,wait));
 };
 arm("connect");
 const metadata:Record<string,string>=input.token?{authorization:`Bearer ${input.token}`}:{};
 try{
  let response:ProtobufMessageV1|undefined,name:string|undefined,seen:OffloadStageV1|undefined;
  for(let round=0;response===undefined;round++){
   const attempt=new AbortController(),link=()=>attempt.abort(),silenceFor=round===0?silenceLimit:reattachLimit;
   let lastRow=Date.now(),silent=false,heard=false;
   controller.signal.addEventListener("abort",link,{once:true});
   const watchdog=setInterval(()=>{if(Date.now()-lastRow>silenceFor){silent=true;attempt.abort();}},Math.max(25,Math.floor(silenceFor/4)));
   try{
    const stream=round===0?client.stream(`${NAMESPACE}.Execution/Execute`,{instance_name:input.instance,action_digest:input.action,skip_cache_lookup:false},{metadata,signal:attempt.signal}):client.stream(`${NAMESPACE}.Execution/WaitExecution`,{name},{metadata,signal:attempt.signal});
    for await(const row of stream){
     lastRow=Date.now();heard=true;
     if(typeof row.name==="string"&&row.name!=="")name=row.name;
     const stage=STAGES[decodeProtobufV1(REAPI_V1.executeMetadata,row.metadata.value).stage]??"UNKNOWN";
     if(stage!==seen){
      seen=stage;input.onStage?.(stage);
      if(stage==="CACHE_CHECK"||stage==="QUEUED"){if(phase==="connect")arm("queue");}
      else if(stage==="EXECUTING")arm("execute");
     }
     if(row.done)response=decodeProtobufV1(REAPI_V1.executeResponse,row.response.value);
    }
   }catch(error){
    if(!silent||controller.signal.aborted)throw error;
   }finally{clearInterval(watchdog);controller.signal.removeEventListener("abort",link);}
   if(response!==undefined)break;
   if(input.signal?.aborted)return declined("cancelled","");
   if(controller.signal.aborted)throw Error("aborted");
   if(!silent)break;
   if(name===undefined)return declined("infrastructure","silent");
   if(round>0&&!heard)return declined("infrastructure","silent after attach");
   client.close();client=connect();
  }
  if(input.signal?.aborted)return declined("cancelled","");
  if(!response)return declined("infrastructure","no response",true);
  const code=response.status?.code??0;
  if(code!==0){const message=String(response.status?.message??""),verdict=classifyOffloadStatusV1({code,message});return declined(verdict.reason,verdict.reason==="infrastructure"?`${code} ${message}`:message.slice(0,200));}
  const result=response.result;
  if(!result)return declined("infrastructure","no result");
  const worker=/^executed by (.+)$/u.exec(String(response.message??""))?.[1]??"",cached=response.cached_result===true;
  return{kind:"done",actionResult:result,cached,origin:cached?`${worker||"fleet"} (cache)`:worker||"fleet"};
 }catch(error){
  if(input.signal?.aborted)return declined("cancelled","");
  if(expired)return declined(phase==="execute"?"infrastructure":"queue-timeout",phase==="execute"&&deadlineAt!==undefined&&Date.now()>=deadlineAt-50?`deadline ${input.deadlineMilliseconds} ms`:phase);
  if(error instanceof GrpcErrorV1){
   if(error.code===GRPC_CODES_V1.NOT_FOUND)return declined("infrastructure","operation lost");
   const verdict=classifyOffloadStatusV1({code:error.code,message:error.message});return declined(verdict.reason,`${error.code} ${error.message}`,verdict.transient);
  }
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
export type OffloadOutputsV1=Readonly<{directory:ReadonlyMap<string,Readonly<{digest:OffloadDigestV1;executable:boolean}>>;files:ReadonlyMap<string,Uint8Array>;stdout:Buffer;stderr:Buffer;exitCode:number;peakMegabytes?:number;threads?:number;phases?:RemoteExecutionPhasesV1}>;

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
 const phases=remoteExecutionPhasesOfMetadataV1(actionResult.execution_metadata as Readonly<Record<string,unknown>>|undefined),auxiliary=(actionResult.execution_metadata as {auxiliary_metadata?:{type_url:string;value:Uint8Array}[]}|undefined)?.auxiliary_metadata,peak=remoteExecutionPeakOfMetadataV1(auxiliary),threads=remoteExecutionThreadsOfMetadataV1(auxiliary);
 return{directory,files,stdout:await stream(actionResult.stdout_raw,actionResult.stdout_digest),stderr:await stream(actionResult.stderr_raw,actionResult.stderr_digest),exitCode:actionResult.exit_code,...(peak===undefined?{}:{peakMegabytes:peak}),...(threads===undefined?{}:{threads}),...(phases===undefined?{}:{phases})};
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

/** 🛟️ Submits like `executeOffloadV1` and, when the call is lost to the transport (the scheduler restarts: connection refused or reset, the stream ends without an answer), submits the same action again with jittered back-off for `budgetMilliseconds`. The scheduler deduplicates by action digest (a worker that kept running the action re-joins and the scheduler adopts it) and answers an action that already finished from the action cache, so the retry never runs the unit a second time; once the budget is spent the caller compiles locally as before. A refusal, a timeout or a cancel is never retried, and the whole ride never outlasts the overall wait of one action (`offloadExecuteBudgetMillisecondsV1`, or `executeMilliseconds`): every new submission gets what is left of it. */
export async function executeOffloadRidingThroughV1(input:Parameters<typeof executeOffloadV1>[0],budgetMilliseconds:number,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise<void>(done=>{setTimeout(done,milliseconds);})):Promise<OffloadExecutionV1>{
 const began=Date.now(),overall=input.executeMilliseconds??offloadExecuteBudgetMillisecondsV1(),until=began+Math.min(budgetMilliseconds,overall);
 let execution=await executeOffloadV1(input);
 for(let attempt=0;execution.kind==="declined"&&execution.transient===true&&input.signal?.aborted!==true&&Date.now()<until;attempt++){
  await pause(Math.min(2000,250*2**attempt)*(0.5+Math.random()));
  const left=began+overall-Date.now();
  if(left<=0||Boolean(input.signal?.aborted))break;
  execution=await executeOffloadV1({...input,executeMilliseconds:left,...input.deadlineMilliseconds===undefined?{}:{deadlineMilliseconds:Math.max(1000,input.deadlineMilliseconds-(Date.now()-began))}});
 }
 return execution;
}

/** ⏱️ Runs one phase of an offload (the upload) under a time limit that really stops it: the work gets a signal that aborts when the time is spent or the build is cancelled, so the transport closes its requests; the phase ends at that moment even for work that ignores the signal. Answers the value, `timeout` or `cancelled`; a fault of the work itself is rethrown. */
export async function runBoundedOffloadPhaseV1<T>(run:(signal:AbortSignal)=>Promise<T>,input:Readonly<{cancelled:AbortSignal;milliseconds:number}>):Promise<Readonly<{kind:"done";value:T}>|Readonly<{kind:"timeout"}>|Readonly<{kind:"cancelled"}>>{
 const controller=new AbortController(),stop=()=>controller.abort(),timer=setTimeout(stop,input.milliseconds);
 input.cancelled.addEventListener("abort",stop,{once:true});
 if(input.cancelled.aborted)controller.abort();
 const ended=new Promise<"ended">(done=>{if(controller.signal.aborted)done("ended");else controller.signal.addEventListener("abort",()=>done("ended"),{once:true});});
 const work=(async()=>run(controller.signal))().then(value=>({value}),error=>({error}));
 try{
  const first=await Promise.race([work,ended]),over=():Readonly<{kind:"timeout"}>|Readonly<{kind:"cancelled"}>=>input.cancelled.aborted?{kind:"cancelled"}:{kind:"timeout"};
  if(first==="ended")return over();
  if("error" in first){if(controller.signal.aborted)return over();throw first.error;}
  return {kind:"done",value:first.value};
 }finally{clearTimeout(timer);input.cancelled.removeEventListener("abort",stop);}
}
