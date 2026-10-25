import {createHash,timingSafeEqual} from "node:crypto";
import {hostname} from "node:os";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../📡️wire/📦️protobuf/🟦️.ts";
import {GRPC_CODES_V1,GrpcErrorV1,startGrpcServerV1,type GrpcCallEventV1,type GrpcServerV1} from "../📡️wire/🚪️grpc/🟦️.ts";
import {RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {RemoteExecutionSchedulerV1,type RemoteExecutionSchedulerEventV1,type RemoteExecutionSchedulerOptionsV1,type RemoteExecutionSnapshotV1} from "../🗓️scheduler/🟦️.ts";
import {remoteExecutionDigestV1,requireRemoteExecutionShapeV1,type RemoteExecutionDigestV1} from "../🟦️.ts";
import {runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";

/** 🔐️ Bearer tokens: with none the service is open (loopback only), a read token allows lookups and downloads, a write token everything, a worker token the worker API. */
export type RemoteExecutionTokensV1=Readonly<{read?:string|undefined;write?:string|undefined;worker?:string|undefined}>;

/** ⚙️ One service: both listeners, the store, the scheduler bounds and the secrets. */
export type RemoteExecutionServerOptionsV1=Readonly<{
 host:string;grpcPort:number;workerPort:number;content:RemoteExecutionContentV1;cacheUrl:string|null;scheduler:Omit<RemoteExecutionSchedulerOptionsV1,"content"|"cache"|"language">&{cache?:RemoteExecutionSchedulerOptionsV1["cache"];language?:RemoteExecutionSchedulerOptionsV1["language"]};
 tokens:RemoteExecutionTokensV1;tls?:Readonly<{certificate:string;key:string}>|undefined;maximumMessageBytes:number;maximumBatchBytes:number;advertise?:string|undefined;onEvent?:(event:RemoteExecutionSchedulerEventV1)=>void;onCall?:(event:GrpcCallEventV1)=>void;
}>;

/** 🛰️ A running service. */
export interface RemoteExecutionServerV1{readonly grpcUrl:string;readonly workerUrl:string;readonly scheduler:RemoteExecutionSchedulerV1;status():Promise<Record<string,unknown>>;stop():Promise<void>}

const WILDCARD=new Set(["0.0.0.0","::","[::]"]),NS="build.bazel.remote.execution.v2",EXEC=`${NS}.Execution`,CACHE=`${NS}.ActionCache`,CAS=`${NS}.ContentAddressableStorage`,CAPS=`${NS}.Capabilities`,STREAM="google.bytestream.ByteStream",
 INSTANCE=/^([a-z0-9][a-z0-9._-]{0,63})?$/u,EMPTY="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]);
const READ_ONLY=new Set([`${CAPS}/GetCapabilities`,`${CACHE}/GetActionResult`,`${CAS}/FindMissingBlobs`,`${CAS}/BatchReadBlobs`,`${CAS}/GetTree`,`${STREAM}/Read`,`${STREAM}/QueryWriteStatus`]);

const compare=(expected:string,presented:string):boolean=>timingSafeEqual(createHash("sha256").update(expected).digest(),createHash("sha256").update(presented).digest());
const bearer=(header:string|null|undefined):string|undefined=>header?.startsWith("Bearer ")?header.slice(7):undefined;
const instanceOf=(name:string):string=>{if(!INSTANCE.test(name))throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Valid instance name required: ${name}`);return name;};
const digestOf=(message:{hash:string;size_bytes:number}|undefined):RemoteExecutionDigestV1=>{
 if(!message)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"Digest required");
 try{return remoteExecutionDigestV1(message.hash,message.size_bytes);}catch{throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Valid SHA-256 digest required: ${message.hash}/${message.size_bytes}`);}
};
const wire=(digest:RemoteExecutionDigestV1)=>({hash:digest.hash,size_bytes:digest.size});
const checkFunction=(value:number|string):void=>{if(value!==0&&value!==1&&value!=="UNKNOWN"&&value!=="SHA256")throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"Only SHA256 is supported");};

function mapStore(error:unknown):never{
 if(error instanceof RemoteExecutionContentErrorV1)throw new GrpcErrorV1(error.code,error.message);
 throw error;
}

/** 🎞️ Converts a scheduler snapshot to a `google.longrunning.Operation` carrying the protocol's metadata and, once done, its `ExecuteResponse`. */
export function remoteExecutionOperationV1(snapshot:RemoteExecutionSnapshotV1):ProtobufMessageV1{
 const stage=snapshot.stage==="CACHE_CHECK"?"CACHE_CHECK":snapshot.stage,metadata=encodeProtobufV1(REAPI_V1.executeMetadata,{stage,action_digest:wire(snapshot.action)}),done=snapshot.stage==="COMPLETED";
 const operation:ProtobufMessageV1={name:snapshot.name,metadata:{type_url:`type.googleapis.com/${REAPI_V1.executeMetadata}`,value:metadata},done};
 if(done&&snapshot.outcome){
  const outcome=snapshot.outcome,details=outcome.status.missing?.length?[{type_url:`type.googleapis.com/${REAPI_V1.preconditionFailure}`,value:encodeProtobufV1(REAPI_V1.preconditionFailure,{violations:outcome.status.missing.map(digest=>({type:"MISSING",subject:`blobs/${digest.hash}/${digest.size}`}))})}]:[];
  const response=encodeProtobufV1(REAPI_V1.executeResponse,{...(outcome.actionResult?{result:decodeProtobufV1(REAPI_V1.actionResult,outcome.actionResult)}:{}),cached_result:outcome.cached,status:{code:outcome.status.code,message:outcome.status.message,details},message:outcome.worker===""?"":`executed by ${outcome.worker}`});
  operation.response={type_url:`type.googleapis.com/${REAPI_V1.executeResponse}`,value:response};
 }
 return operation;
}

class Queue<T>{
 private items:T[]=[];private waiting?:()=>void;private space?:()=>void;private closed=false;
 constructor(private readonly bound:number){}
 async push(item:T):Promise<void>{while(this.items.length>=this.bound)await new Promise<void>(resolve=>{this.space=resolve;});this.items.push(item);this.waiting?.();}
 end():void{this.closed=true;this.waiting?.();}
 async *drain():AsyncGenerator<T>{
  for(;;){
   while(this.items.length>0){const item=this.items.shift()!;this.space?.();yield item;}
   if(this.closed)return;
   await new Promise<void>(resolve=>{this.waiting=resolve;});
  }
 }
}

/** 🧩️ The gRPC methods of the Remote Execution API this service implements, over a store and a scheduler. */
export function remoteExecutionHandlersV1(content:RemoteExecutionContentV1,scheduler:RemoteExecutionSchedulerV1,limits:Readonly<{maximumBatchBytes:number}>):Record<string,(input:any,call:any)=>any>{
 const blobName=(name:string,marker:"blobs"|"uploads")=>{
  const parts=name.split("/"),index=marker==="blobs"?parts.indexOf("blobs"):parts.indexOf("uploads");
  if(parts.includes("compressed-blobs"))throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"Compressed blobs are not supported");
  if(marker==="blobs"){if(index<0||parts.length!==index+3)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Invalid resource name: ${name}`);return {instance:instanceOf(parts.slice(0,index).join("/")),digest:digestOf({hash:parts[index+1]!,size_bytes:Number(parts[index+2])})};}
  if(index<0||parts[index+2]!=="blobs"||parts.length!==index+5)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Invalid resource name: ${name}`);
  return {instance:instanceOf(parts.slice(0,index).join("/")),digest:digestOf({hash:parts[index+3]!,size_bytes:Number(parts[index+4])})};
 };
 return {
  [`${CAPS}/GetCapabilities`]:async()=>({
   cache_capabilities:{digest_functions:["SHA256"],action_cache_update_capabilities:{update_enabled:true},max_batch_total_size_bytes:limits.maximumBatchBytes,symlink_absolute_path_strategy:"ALLOWED"},
   execution_capabilities:{digest_function:"SHA256",digest_functions:["SHA256"],exec_enabled:true},
   low_api_version:{major:2,minor:0},high_api_version:{major:2,minor:3},
  }),
  [`${EXEC}/Execute`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal}){
   checkFunction(request.digest_function);
   const instance=instanceOf(request.instance_name),action=digestOf(request.action_digest);
   for await(const snapshot of scheduler.execute({instance,action,skipCacheLookup:request.skip_cache_lookup===true},call.signal))yield remoteExecutionOperationV1(snapshot);
  },
  [`${EXEC}/WaitExecution`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal}){
   const stream=scheduler.wait(request.name,call.signal);
   if(!stream)throw new GrpcErrorV1(GRPC_CODES_V1.NOT_FOUND,`Unknown operation: ${request.name}`);
   for await(const snapshot of stream)yield remoteExecutionOperationV1(snapshot);
  },
  [`${CACHE}/GetActionResult`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   checkFunction(request.digest_function);
   const bytes=await content.readActionResult(instanceOf(request.instance_name),digestOf(request.action_digest).hash,call.signal).catch(mapStore);
   if(!bytes)throw new GrpcErrorV1(GRPC_CODES_V1.NOT_FOUND,"Action result not found");
   return decodeProtobufV1(REAPI_V1.actionResult,bytes);
  },
  [`${CACHE}/UpdateActionResult`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   checkFunction(request.digest_function);
   if(!request.action_result)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"Action result required");
   await content.writeActionResult(instanceOf(request.instance_name),digestOf(request.action_digest).hash,encodeProtobufV1(REAPI_V1.actionResult,request.action_result),call.signal).catch(mapStore);
   return request.action_result;
  },
  [`${CAS}/FindMissingBlobs`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   checkFunction(request.digest_function);
   const digests=(request.blob_digests as {hash:string;size_bytes:number}[]).map(digestOf),missing=await content.missing(instanceOf(request.instance_name),digests.filter(digest=>digest.size>0||digest.hash!==EMPTY),call.signal).catch(mapStore);
   return {missing_blob_digests:missing.map(wire)};
  },
  [`${CAS}/BatchUpdateBlobs`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   checkFunction(request.digest_function);
   const instance=instanceOf(request.instance_name),rows=request.requests as {digest:{hash:string;size_bytes:number};data:Uint8Array;compressor:number}[];
   let total=0;for(const row of rows)total+=row.data.length;
   if(total>limits.maximumBatchBytes)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Batch exceeds ${limits.maximumBatchBytes} bytes`);
   const responses:ProtobufMessageV1[]=[];
   await Promise.all(rows.map(async(row,index)=>{
    let status={code:0,message:""};
    try{
     const digest=digestOf(row.digest);
     if(row.compressor!==0)status={code:3,message:"Compressed blobs are not supported"};
     else if(row.data.length!==digest.size||createHash("sha256").update(row.data).digest("hex")!==digest.hash)status={code:3,message:"Blob does not match its digest"};
     else if(digest.size>0)await content.write(instance,digest,row.data,call.signal);
    }catch(error){status={code:error instanceof GrpcErrorV1?error.code:error instanceof RemoteExecutionContentErrorV1?error.code:13,message:(error as Error).message};}
    responses[index]={digest:row.digest,status};
   }));
   return {responses};
  },
  [`${CAS}/BatchReadBlobs`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   checkFunction(request.digest_function);
   const instance=instanceOf(request.instance_name),rows=request.digests as {hash:string;size_bytes:number}[],responses:ProtobufMessageV1[]=[];
   let total=0;for(const row of rows)total+=row.size_bytes;
   if(total>limits.maximumBatchBytes)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Batch exceeds ${limits.maximumBatchBytes} bytes`);
   await Promise.all(rows.map(async(row,index)=>{
    try{
     const digest=digestOf(row),data=digest.size===0?new Uint8Array(0):await content.read(instance,digest,call.signal);
     responses[index]=data?{digest:row,data,status:{code:0,message:""}}:{digest:row,status:{code:5,message:"Blob not found"}};
    }catch(error){responses[index]={digest:row,status:{code:error instanceof RemoteExecutionContentErrorV1?error.code:13,message:(error as Error).message}};}
   }));
   return {responses};
  },
  [`${CAS}/GetTree`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal}){
   checkFunction(request.digest_function);
   const instance=instanceOf(request.instance_name),seen=new Set<string>(),pending=[digestOf(request.root_digest)],directories:ProtobufMessageV1[]=[];
   while(pending.length>0){
    const digest=pending.shift()!;
    if(seen.has(digest.hash))continue;seen.add(digest.hash);
    const bytes=digest.size===0?new Uint8Array(0):await content.read(instance,digest,call.signal).catch(mapStore);
    if(!bytes)throw new GrpcErrorV1(GRPC_CODES_V1.NOT_FOUND,`Missing directory ${digest.hash}`);
    const directory=decodeProtobufV1(REAPI_V1.directory,bytes);directories.push(directory);
    for(const node of directory.directories as {digest:{hash:string;size_bytes:number}}[])pending.push(digestOf(node.digest));
   }
   yield {directories,next_page_token:""};
  },
  [`${STREAM}/Read`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal}){
   const {instance,digest}=blobName(request.resource_name,"blobs");let skip=Number(request.read_offset),remaining=request.read_limit>0?Number(request.read_limit):Infinity;
   if(skip<0||skip>digest.size)throw new GrpcErrorV1(GRPC_CODES_V1.OUT_OF_RANGE,"Read offset out of range");
   if(digest.size===0)return;
   const source=await content.stream(instance,digest,call.signal).catch(mapStore);
   if(!source)throw new GrpcErrorV1(GRPC_CODES_V1.NOT_FOUND,`Blob not found: ${digest.hash}/${digest.size}`);
   for await(let chunk of source){
    if(skip>0){if(skip>=chunk.length){skip-=chunk.length;continue;}chunk=chunk.subarray(skip);skip=0;}
    if(chunk.length>remaining)chunk=chunk.subarray(0,remaining);
    for(let offset=0;offset<chunk.length;offset+=1048576){yield {data:chunk.subarray(offset,offset+1048576)};}
    remaining-=chunk.length;if(remaining<=0)break;
   }
  },
  [`${STREAM}/Write`]:async(requests:AsyncIterable<ProtobufMessageV1>,call:{signal:AbortSignal})=>{
   let target:ReturnType<typeof blobName>|undefined,committed=0,queue:Queue<Uint8Array>|undefined,upload:Promise<void>|undefined,finished=false,skipped=false;
   try{
    for await(const message of requests){
     if(!target){
      target=blobName(message.resource_name,"uploads");
      if(target.digest.size===0)skipped=true;
      else if(await content.has(target.instance,target.digest.hash,call.signal).catch(mapStore))skipped=true;
      else{queue=new Queue<Uint8Array>(8);upload=content.write(target.instance,target.digest,queue.drain(),call.signal);upload.catch(()=>{});}
     }
     if(Number(message.write_offset)!==committed&&!skipped)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Write offset ${message.write_offset} but ${committed} bytes were committed`);
     const data=message.data as Uint8Array;
     if(!skipped&&data.length>0){if(committed+data.length>target.digest.size)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"More data than the digest declares");await queue!.push(data);}
     committed+=data.length;
     if(message.finish_write){finished=true;break;}
    }
    if(!target||!finished)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,"Write must finish");
    if(skipped)return {committed_size:target.digest.size};
    if(committed!==target.digest.size)throw new GrpcErrorV1(GRPC_CODES_V1.INVALID_ARGUMENT,`Blob has ${committed} bytes but the digest declares ${target.digest.size}`);
    queue!.end();await upload!.catch(mapStore);
    return {committed_size:committed};
   }catch(error){queue?.end();await upload?.catch(()=>{});throw error;}
  },
  [`${STREAM}/QueryWriteStatus`]:async(request:ProtobufMessageV1,call:{signal:AbortSignal})=>{
   const {instance,digest}=blobName(request.resource_name,"uploads"),exists=digest.size===0||await content.has(instance,digest.hash,call.signal).catch(mapStore);
   return exists?{committed_size:digest.size,complete:true}:{committed_size:0,complete:false};
  },
 };
}

/** 🚀️ Starts the service: the gRPC listener for build tools (capabilities, execution, action cache, CAS, ByteStream) and the HTTP listener for workers and status, both over one scheduler. Refuses to listen beyond loopback without tokens. */
export async function startRemoteExecutionServerV1(options:RemoteExecutionServerOptionsV1):Promise<RemoteExecutionServerV1>{
 const open=options.tokens.read===undefined&&options.tokens.write===undefined,openWorkers=options.tokens.worker===undefined;
 if(!LOOPBACK.has(options.host)&&(open||openWorkers))throw Error("Tokens are required to listen beyond loopback");
 const cacheUrl=options.cacheUrl;
 let workerUrl="";
 const scheduler=new RemoteExecutionSchedulerV1({...options.scheduler,content:options.content,cache:options.scheduler.cache??(cacheUrl?{url:cacheUrl,relative:LOOPBACK.has(new URL(cacheUrl).hostname)||WILDCARD.has(new URL(cacheUrl).hostname)}:{url:"",relative:false}),language:options.scheduler.language??"none",...(options.onEvent?{onEvent:options.onEvent}:{})});
 const authorize=(call:{method:string;metadata:Readonly<Record<string,string>>}):void=>{
  if(open)return;
  const presented=bearer(call.metadata.authorization);
  if(!presented)throw new GrpcErrorV1(GRPC_CODES_V1.UNAUTHENTICATED,"Unauthorized");
  const writer=options.tokens.write!==undefined&&compare(options.tokens.write,presented),reader=options.tokens.read!==undefined&&compare(options.tokens.read,presented);
  if(!writer&&!reader)throw new GrpcErrorV1(GRPC_CODES_V1.UNAUTHENTICATED,"Unauthorized");
  if(!writer&&!READ_ONLY.has(call.method))throw new GrpcErrorV1(GRPC_CODES_V1.PERMISSION_DENIED,"Read-only token");
 };
 const grpc:GrpcServerV1=await startGrpcServerV1({host:options.host,port:options.grpcPort,handlers:remoteExecutionHandlersV1(options.content,scheduler,{maximumBatchBytes:options.maximumBatchBytes}),authorize,tls:options.tls,maximumMessageBytes:options.maximumMessageBytes,...(options.onCall?{onCall:options.onCall}:{})});
 const text=(status:number,message:string)=>new Response(message+"\n",{status,headers:{"content-type":"text/plain"}});
 const allowed=(request:Request,kind:"worker"|"status"):Response|undefined=>{
  const presented=bearer(request.headers.get("authorization"));
  if(kind==="worker"){
   if(openWorkers)return undefined;
   return presented&&compare(options.tokens.worker!,presented)?undefined:text(401,"Unauthorized");
  }
  if(open&&openWorkers)return undefined;
  const matches=[options.tokens.read,options.tokens.write,options.tokens.worker].some(token=>token!==undefined&&presented!==undefined&&compare(token,presented));
  return matches?undefined:text(401,"Unauthorized");
 };
 const status=async():Promise<Record<string,unknown>>=>{
  const view=scheduler.status();let answers=false;
  if(cacheUrl)answers=await fetch(new URL("/healthz",cacheUrl),{signal:AbortSignal.timeout(2000)}).then(response=>{void response.body?.cancel();return response.ok;},()=>false);
  const document={version:1,service:"semio-remote-execution",startedAt:scheduler.startedAt,grpc:grpc.url,workerApi:workerUrl,cache:{url:cacheUrl,answers},counters:view.counters,workers:view.workers,platforms:view.platforms};
  requireRemoteExecutionShapeV1("ServerStatus",document,"Closed remote execution status required");return document;
 };
 const http=await runtimeServeV1({
  hostname:options.host,port:options.workerPort,idleTimeoutSeconds:120,maximumRequestBodyBytes:64*1048576,
  ...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),
  fetch:async request=>{
   const url=new URL(request.url),path=url.pathname,method=request.method;
   try{
    if(path==="/healthz")return Response.json({status:"ok"});
    if(path==="/status"&&method==="GET"){const denied=allowed(request,"status");return denied??Response.json(await status());}
    const route=/^\/v1\/workers(?:\/([A-Za-z0-9-]+)(?:\/(poll|heartbeat)|\/tasks\/([A-Za-z0-9.-]+)\/result)?)?$/u.exec(path);
    if(!route)return text(404,"Not found");
    const denied=allowed(request,"worker");if(denied)return denied;
    const [,id,verb,task]=route;
    if(!id&&method==="POST"){
     const body=await request.json().catch(()=>undefined);
     try{requireRemoteExecutionShapeV1("WorkerRegistration",body,"Closed worker registration required");}catch(error){return text(400,(error as Error).message);}
     return Response.json(scheduler.register(body));
    }
    if(id&&!verb&&!task&&method==="DELETE"){scheduler.deregister(id);return Response.json({ok:true});}
    if(id&&verb==="poll"&&method==="POST"){const wait=Math.min(60000,Math.max(0,Number(url.searchParams.get("wait")??"20000")));return Response.json(await scheduler.poll(id,wait,request.signal));}
    if(id&&verb==="heartbeat"&&method==="POST"){
     const body=await request.json().catch(()=>undefined);
     try{requireRemoteExecutionShapeV1("Heartbeat",body,"Closed heartbeat required");}catch(error){return text(400,(error as Error).message);}
     return Response.json(scheduler.heartbeat(id,(body as {running:string[]}).running));
    }
    if(id&&task&&method==="POST"){
     const body=await request.json().catch(()=>undefined);
     try{requireRemoteExecutionShapeV1("TaskResult",body,"Closed task result required");}catch(error){return text(400,(error as Error).message);}
     return Response.json({accepted:scheduler.report(id,task,body as never)});
    }
    return text(405,"Method not allowed");
   }catch(error){return text(500,(error as Error).message);}
  },
 });
 const host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host;
 workerUrl=`${options.tls?"https":"http"}://${options.advertise??(WILDCARD.has(options.host)?hostname():host)}:${http.port}`;
 let closing:Promise<void>|undefined;
 return Object.freeze({grpcUrl:grpc.url,workerUrl,scheduler,status,stop:()=>closing??=(async()=>{scheduler.close();await http.stop(true);await grpc.stop();})()});
}
