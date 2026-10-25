import {once} from "node:events";
import {readFileSync} from "node:fs";
import http2 from "node:http2";
import {gunzipSync,inflateSync} from "node:zlib";
import {decodeProtobufV1,encodeProtobufV1,PROTOBUF_SERVICES_V1,type ProtobufMessageV1,type ProtobufMethodV1} from "../📦️protobuf/🟦️.ts";

/** 🚦️ The gRPC status codes this implementation raises and understands. */
export const GRPC_CODES_V1=Object.freeze({OK:0,CANCELLED:1,UNKNOWN:2,INVALID_ARGUMENT:3,DEADLINE_EXCEEDED:4,NOT_FOUND:5,ALREADY_EXISTS:6,PERMISSION_DENIED:7,RESOURCE_EXHAUSTED:8,FAILED_PRECONDITION:9,ABORTED:10,OUT_OF_RANGE:11,UNIMPLEMENTED:12,INTERNAL:13,UNAVAILABLE:14,DATA_LOSS:15,UNAUTHENTICATED:16});

/** 🧯️ A call failure with a gRPC status; `details` is a serialized `google.rpc.Status` sent in `grpc-status-details-bin`. */
export class GrpcErrorV1 extends Error{
 constructor(readonly code:number,message:string,readonly details?:Uint8Array){super(message);}
}

/** 📞️ What a handler may know about the call it serves. */
export type GrpcCallV1=Readonly<{method:string;metadata:Readonly<Record<string,string>>;signal:AbortSignal;peer:string}>;

/** 🧩️ One handler per `package.Service/Method`: unary and client-streaming handlers answer a message, server-streaming handlers answer an async iterable of messages. */
export type GrpcHandlersV1=Readonly<Record<string,(input:any,call:GrpcCallV1)=>any>>;

/** 📜️ A finished call, published after the status is decided. */
export type GrpcCallEventV1=Readonly<{stage:"call";method:string;code:number;milliseconds:number}>;

/** ⚙️ Authors one gRPC listener. */
export type GrpcServerOptionsV1=Readonly<{host:string;port:number;handlers:GrpcHandlersV1;authorize?:(call:Readonly<{method:string;metadata:Readonly<Record<string,string>>}>)=>void;tls?:Readonly<{certificate:string;key:string}>|undefined;maximumMessageBytes?:number;onCall?:(event:GrpcCallEventV1)=>void}>;

/** 🛰️ A listening gRPC server. */
export interface GrpcServerV1{readonly url:string;readonly port:number;stop():Promise<void>}

const METHODS=new Map<string,ProtobufMethodV1>();
for(const [service,methods] of Object.entries(PROTOBUF_SERVICES_V1))for(const [name,method] of Object.entries(methods))METHODS.set(`${service}/${name}`,method);

const UNITS:Readonly<Record<string,number>>={H:3600000,M:60000,S:1000,m:1,u:0.001,n:0.000001};

class ByteQueue{
 private chunks:Uint8Array[]=[];length=0;
 push(chunk:Uint8Array):void{this.chunks.push(chunk);this.length+=chunk.length;}
 take(size:number):Uint8Array{
  const out=new Uint8Array(size);let offset=0;
  while(offset<size){const head=this.chunks[0]!,need=size-offset;if(head.length<=need){out.set(head,offset);offset+=head.length;this.chunks.shift();}else{out.set(head.subarray(0,need),offset);this.chunks[0]=head.subarray(need);offset=size;}}
  this.length-=size;return out;
 }
}

async function* chunksOf(stream:NodeJS.ReadableStream):AsyncGenerator<Uint8Array>{
 const queue:Uint8Array[]=[];let ended=false,failure:unknown,wake:(()=>void)|undefined;
 const notify=():void=>{const resume=wake;wake=undefined;resume?.();};
 stream.on("data",(chunk:Uint8Array)=>{queue.push(chunk);if(queue.length>8)stream.pause();notify();});
 stream.on("end",()=>{ended=true;notify();});
 stream.on("error",error=>{failure=error;notify();});
 stream.on("close",()=>{ended=true;notify();});
 for(;;){
  while(queue.length>0){const chunk=queue.shift()!;if(queue.length<4)stream.resume();yield chunk;}
  if(failure)throw failure;
  if(ended)return;
  await new Promise<void>(resolve=>{wake=resolve;});
 }
}

async function* messageFrames(source:AsyncIterable<Uint8Array>,limit:number,encoding:string|undefined):AsyncGenerator<Uint8Array>{
 const queue=new ByteQueue();let need:{compressed:boolean;size:number}|undefined;
 for await(const chunk of source){
  queue.push(chunk);
  for(;;){
   if(!need){
    if(queue.length<5)break;
    const head=queue.take(5),size=new DataView(head.buffer,head.byteOffset,5).getUint32(1);
    if(head[0]!==0&&head[0]!==1)throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Invalid message frame");
    if(size>limit)throw new GrpcErrorV1(GRPC_CODES_V1.RESOURCE_EXHAUSTED,`Message exceeds ${limit} bytes`);
    need={compressed:head[0]===1,size};
   }
   if(queue.length<need.size)break;
   let body=queue.take(need.size);
   if(need.compressed){
    if(encoding==="gzip")body=gunzipSync(body);else if(encoding==="deflate")body=inflateSync(body);else throw new GrpcErrorV1(GRPC_CODES_V1.UNIMPLEMENTED,`Message encoding not supported: ${encoding??"none"}`);
   }
   need=undefined;yield body;
  }
 }
 if(need||queue.length>0)throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Truncated message frame");
}

const frame=(body:Uint8Array):Uint8Array=>{const out=new Uint8Array(5+body.length);new DataView(out.buffer).setUint32(1,body.length);out.set(body,5);return out;};

const percent=(message:string):string=>encodeURIComponent(message).replace(/%20/gu," ");

function metadataOf(headers:http2.IncomingHttpHeaders):Record<string,string>{
 const out:Record<string,string>={};
 for(const [name,value] of Object.entries(headers))if(!name.startsWith(":")&&value!==undefined)out[name]=Array.isArray(value)?value.join(","):String(value);
 return out;
}

function failure(error:unknown):GrpcErrorV1{
 if(error instanceof GrpcErrorV1)return error;
 return new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,error instanceof Error?error.message:String(error));
}

async function writeFrame(stream:http2.ServerHttp2Stream,body:Uint8Array):Promise<void>{
 if(stream.destroyed||stream.closed)throw new GrpcErrorV1(GRPC_CODES_V1.CANCELLED,"Call closed");
 if(!stream.write(frame(body)))await once(stream,"drain").catch(()=>{});
}

async function serve(stream:http2.ServerHttp2Stream,headers:http2.IncomingHttpHeaders,options:GrpcServerOptionsV1,limit:number):Promise<void>{
 const started=performance.now(),path=String(headers[":path"]??""),key=path.startsWith("/")?path.slice(1):path,metadata=metadataOf(headers),controller=new AbortController();
 let sent=false,code:number=GRPC_CODES_V1.OK,timer:ReturnType<typeof setTimeout>|undefined;
 const finish=(status:number,message:string,details?:Uint8Array):void=>{
  code=status;
  if(stream.destroyed||stream.closed)return;
  const trailers:http2.OutgoingHttpHeaders={"grpc-status":String(status),...(message?{"grpc-message":percent(message)}:{}),...(details?{"grpc-status-details-bin":Buffer.from(details).toString("base64").replace(/=+$/u,"")}:{})};
  if(!sent){sent=true;stream.respond({":status":200,"content-type":"application/grpc",...trailers},{endStream:true});return;}
  stream.once("wantTrailers",()=>{stream.sendTrailers(trailers);});stream.end();
 };
 const respond=():void=>{if(sent)return;sent=true;stream.respond({":status":200,"content-type":"application/grpc","grpc-accept-encoding":"identity,gzip"},{waitForTrailers:true});};
 stream.on("close",()=>{controller.abort(new GrpcErrorV1(GRPC_CODES_V1.CANCELLED,"Call cancelled"));});
 stream.on("error",()=>{});
 try{
  const method=METHODS.get(key),handler=options.handlers[key];
  if(headers[":method"]!=="POST"||!String(headers["content-type"]??"").startsWith("application/grpc"))throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"gRPC request required");
  if(!method||!handler)throw new GrpcErrorV1(GRPC_CODES_V1.UNIMPLEMENTED,`Method not implemented: ${key}`);
  options.authorize?.({method:key,metadata});
  const timeout=/^(\d{1,8})([HMSmun])$/u.exec(String(headers["grpc-timeout"]??""));
  if(timeout){timer=setTimeout(()=>{controller.abort(new GrpcErrorV1(GRPC_CODES_V1.DEADLINE_EXCEEDED,"Deadline exceeded"));},Number(timeout[1])*UNITS[timeout[2]!]!);}
  const call:GrpcCallV1=Object.freeze({method:key,metadata,signal:controller.signal,peer:`${stream.session?.socket?.remoteAddress??"unknown"}`}),
   requests=(async function*(){for await(const body of messageFrames(chunksOf(stream),limit,headers["grpc-encoding"] as string|undefined))yield decodeProtobufV1(method.request,body);})();
  const aborted=new Promise<never>((_,reject)=>{controller.signal.addEventListener("abort",()=>{reject(controller.signal.reason);},{once:true});});
  void aborted.catch(()=>{});
  const run=async():Promise<void>=>{
   if(method.clientStreaming){const result=await handler(requests,call) as ProtobufMessageV1;respond();await writeFrame(stream,encodeProtobufV1(method.response,result));return;}
   const iterator=requests[Symbol.asyncIterator](),first=await iterator.next();
   if(first.done)throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Request message required");
   const second=await iterator.next();
   if(!second.done)throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Exactly one request message required");
   if(!method.serverStreaming){const result=await handler(first.value,call) as ProtobufMessageV1;respond();await writeFrame(stream,encodeProtobufV1(method.response,result));return;}
   const output=handler(first.value,call) as AsyncIterable<ProtobufMessageV1>,source=output[Symbol.asyncIterator]();
   try{
    for(;;){
     const next=await Promise.race([source.next(),aborted]);
     if(next.done)break;
     respond();await writeFrame(stream,encodeProtobufV1(method.response,next.value));
    }
   }finally{await source.return?.(undefined)?.catch(()=>{});}
  };
  const running=run();void running.catch(()=>{});
  await Promise.race([running,aborted]);
  finish(GRPC_CODES_V1.OK,"");
 }catch(error){
  const problem=failure(error);
  finish(problem.code,problem.message,problem.details);
 }finally{
  if(timer)clearTimeout(timer);
  options.onCall?.({stage:"call",method:key,code,milliseconds:Math.round(performance.now()-started)});
 }
}

/** 🚀️ Serves gRPC over HTTP/2 (cleartext, or TLS when a certificate is given) with Node's own `http2` module (the 1 MiB per-stream flow-control window is announced per session after it opens: declared in the server options it makes Bun's in-process client RST_STREAM unseen by the server from the second session on, so a cancelled call would run on): unary, server-streaming and client-streaming calls for every method the wire schema declares, request framing with gzip/deflate decompression, deadlines, cancellation, status trailers, flow control and per-call authorization. */
export async function startGrpcServerV1(options:GrpcServerOptionsV1):Promise<GrpcServerV1>{
 for(const key of Object.keys(options.handlers))if(!METHODS.has(key))throw Error(`Declared method required: ${key}`);
 const limit=options.maximumMessageBytes??33554432,settings={maxConcurrentStreams:1024},
  server=options.tls?http2.createSecureServer({cert:readFileSync(options.tls.certificate),key:readFileSync(options.tls.key),settings}):http2.createServer({settings});
 server.on("stream",(stream,headers)=>{void serve(stream,headers,options,limit).catch(()=>{stream.destroy();});});
 server.on("sessionError",()=>{});
 await new Promise<void>((resolve,reject)=>{server.once("error",reject);server.listen(options.port,options.host,()=>{server.off("error",reject);resolve();});});
 const port=(server.address() as {port:number}).port,host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host,sessions=new Set<http2.ServerHttp2Session>();
 server.on("session",session=>{session.settings({initialWindowSize:1048576});sessions.add(session);session.once("close",()=>{sessions.delete(session);});});
 let closing:Promise<void>|undefined;
 return Object.freeze({url:`${options.tls?"grpcs":"grpc"}://${host}:${port}`,port,stop:()=>closing??=new Promise<void>(resolve=>{for(const session of sessions)session.destroy();server.close(()=>{resolve();});})});
}

/** ☎️ Options of one client call. */
export type GrpcClientCallV1=Readonly<{metadata?:Readonly<Record<string,string>>;signal?:AbortSignal;timeoutMilliseconds?:number}>;

/** 🔌️ A small gRPC client over one HTTP/2 connection, for health probes, tests and tools. */
export interface GrpcClientV1{
 unary(method:string,request:ProtobufMessageV1,call?:GrpcClientCallV1):Promise<ProtobufMessageV1>;
 stream(method:string,request:ProtobufMessageV1,call?:GrpcClientCallV1):AsyncGenerator<ProtobufMessageV1>;
 upload(method:string,requests:Iterable<ProtobufMessageV1>,call?:GrpcClientCallV1):Promise<ProtobufMessageV1>;
 close():void;
}

/** 🔭️ Connects a client to `grpc://host:port` or `grpcs://host:port` (an optional `ca` file trusts a private certificate authority). */
export function connectGrpcV1(url:string,options:Readonly<{certificateAuthority?:string}>={}):GrpcClientV1{
 const parsed=new URL(url.replace(/^grpc(s?):/u,"http$1:")),session=http2.connect(parsed.origin,options.certificateAuthority?{ca:readFileSync(options.certificateAuthority)}:{});
 session.on("error",()=>{});
 const open=(method:string,call:GrpcClientCallV1|undefined)=>{
  const definition=METHODS.get(method);
  if(!definition)throw Error(`Declared method required: ${method}`);
  const headers:http2.OutgoingHttpHeaders={":method":"POST",":path":"/"+method,"content-type":"application/grpc",te:"trailers","grpc-accept-encoding":"identity",...(call?.timeoutMilliseconds?{"grpc-timeout":`${call.timeoutMilliseconds}m`}:{}),...(call?.metadata??{})},
   request=session.request(headers);
  call?.signal?.addEventListener("abort",()=>{request.close(http2.constants.NGHTTP2_CANCEL);},{once:true});
  return {definition,request};
 };
 async function* responses(method:string,send:(request:http2.ClientHttp2Stream,definition:ProtobufMethodV1)=>void,call?:GrpcClientCallV1):AsyncGenerator<ProtobufMessageV1>{
  const {definition,request}=open(method,call);let status:string|undefined,message="",details:string|undefined;
  request.on("response",headers=>{if(headers["grpc-status"]!==undefined){status=String(headers["grpc-status"]);message=decodeURIComponent(String(headers["grpc-message"]??""));details=headers["grpc-status-details-bin"] as string|undefined;}});
  request.on("trailers",headers=>{status=String(headers["grpc-status"]??"");message=decodeURIComponent(String(headers["grpc-message"]??""));details=headers["grpc-status-details-bin"] as string|undefined;});
  request.on("error",()=>{});
  send(request,definition);
  for await(const body of messageFrames(chunksOf(request),33554432,undefined))yield decodeProtobufV1(definition.response,body);
  if(status===undefined)throw new GrpcErrorV1(GRPC_CODES_V1.UNAVAILABLE,"Call closed without status");
  if(Number(status)!==0)throw new GrpcErrorV1(Number(status),message,details===undefined?undefined:Buffer.from(details,"base64"));
 }
 const sendAll=(request:http2.ClientHttp2Stream,definition:ProtobufMethodV1,messages:Iterable<ProtobufMessageV1>)=>{for(const item of messages)request.write(frame(encodeProtobufV1(definition.request,item)));request.end();};
 const client:GrpcClientV1={
  async unary(method,request,call){for await(const value of responses(method,(stream,definition)=>sendAll(stream,definition,[request]),call))return value;throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Response message required");},
  stream:(method,request,call)=>responses(method,(stream,definition)=>sendAll(stream,definition,[request]),call),
  async upload(method,requests,call){for await(const value of responses(method,(stream,definition)=>sendAll(stream,definition,requests),call))return value;throw new GrpcErrorV1(GRPC_CODES_V1.INTERNAL,"Response message required");},
  close:()=>{session.close();},
 };
 return Object.freeze(client);
}
