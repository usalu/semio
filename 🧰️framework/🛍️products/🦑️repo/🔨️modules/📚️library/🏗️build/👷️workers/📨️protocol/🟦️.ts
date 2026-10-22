import {AsyncLocalStorage} from "node:async_hooks";
import {realpathSync,statSync} from "node:fs";
import {isAbsolute,resolve,sep} from "node:path";
import {format as formatText} from "node:util";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import { runtimeStdinStreamV1 } from "../../../../../../../🔨️modules/⚙️runtime/🚀️process/🟦️.ts";

const protocol=schema.definitions.Protocol.const;

/** 📨️ One declared input of a work request: a path relative to the execution root and the opaque digest the build tool computed for it. */
export type WorkInputV1=Readonly<{path:string;digest:string}>;

/** 📨️ One normalised request of the JSON worker protocol; unknown wire fields are dropped. */
export type WorkRequestV1=Readonly<{arguments:readonly string[];inputs:readonly WorkInputV1[];requestId:number;cancel:boolean;verbosity:number;sandboxDir?:string}>;

/** 📨️ One response of the JSON worker protocol. */
export type WorkResponseV1=Readonly<{exitCode:number;output:string;requestId:number;wasCancelled?:boolean}>;

/** 🎬️ What a handler returns for one request. */
export type WorkOutcomeV1=Readonly<{exitCode:number;output:string}>;

/** 🎬️ Runs one request; it must stop soon after `signal` aborts. */
export type WorkHandlerV1=(request:WorkRequestV1,context:Readonly<{signal:AbortSignal}>)=>Promise<WorkOutcomeV1>;

/** 🚫 A message that breaks the protocol; the worker can not answer it and must stop. */
export class WorkerProtocolErrorV1 extends Error{constructor(message:string){super(message);this.name="WorkerProtocolErrorV1";}}

/** 📖️ Parses one protocol line into a normalised request and refuses anything the closed schema rejects. */
export function parseWorkRequestV1(line:string):WorkRequestV1{
 let value:unknown;
 try{value=JSON.parse(line);}catch{throw new WorkerProtocolErrorV1("Work request is not JSON");}
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/WorkRequest"},value);
 if(errors.length)throw new WorkerProtocolErrorV1("Work request breaks the protocol: "+JSON.stringify(errors[0]));
 const row=value as Record<string,unknown>;
 return Object.freeze({arguments:Object.freeze([...(row.arguments as string[]|undefined)??[]]),inputs:Object.freeze(((row.inputs as WorkInputV1[]|undefined)??[]).map(input=>Object.freeze({path:input.path,digest:input.digest}))),requestId:(row.requestId as number|undefined)??0,cancel:row.cancel===true,verbosity:(row.verbosity as number|undefined)??0,...(typeof row.sandboxDir==="string"&&row.sandboxDir?{sandboxDir:row.sandboxDir}:{})});
}

/** 📨️ Encodes one response as a single compact line; output beyond the bound is cut on a character boundary and marked. */
export function encodeWorkResponseV1(response:WorkResponseV1,maximumOutputBytes:number=protocol.maximumOutputBytes):string{
 let output=response.output;
 if(Buffer.byteLength(output)>maximumOutputBytes){const marker=protocol.truncationMarker;let end=Math.max(0,maximumOutputBytes-Buffer.byteLength(marker));let cut=Buffer.from(output).subarray(0,end).toString("utf8");if(cut.endsWith("�"))cut=cut.slice(0,-1);output=cut+marker;}
 const row:WorkResponseV1={exitCode:response.exitCode,output,requestId:response.requestId,...(response.wasCancelled?{wasCancelled:true}:{})};
 return JSON.stringify(row)+"\n";
}

/** 📥️ Reads a byte stream chunk by chunk; the default is the process's standard input. */
export async function* readWorkerInputV1(stream:ReadableStream<Uint8Array>=runtimeStdinStreamV1()):AsyncGenerator<Uint8Array>{
 const reader=stream.getReader();
 try{for(;;){const row=await reader.read();if(row.done)return;yield row.value;}}finally{reader.releaseLock();}
}

/** 🏁 How the serve loop ended. */
export type WorkerServeResultV1=Readonly<{requests:number;cancelled:number}>;

/** 👷️ Serves the JSON worker protocol: newline-framed requests are handled with at most `concurrency` in flight, a cancel request aborts the matching handler (or answers a queued request as cancelled), a handler failure becomes exit code 1 with its message, and closing the input lets every accepted request finish. Responses are written one line at a time in completion order. */
export async function serveWorkerV1(options:{input:AsyncIterable<Uint8Array|string>;write:(text:string)=>void|Promise<void>;handler:WorkHandlerV1;concurrency?:number;afterRequest?:(served:number)=>void|Promise<void>;maximumRequestBytes?:number;maximumOutputBytes?:number}):Promise<WorkerServeResultV1>{
 const concurrency=options.concurrency??1,maximumRequestBytes=options.maximumRequestBytes??protocol.maximumRequestBytes,decoder=new TextDecoder(),queue:WorkRequestV1[]=[],active=new Map<number,{controller:AbortController;done:Promise<void>}>();
 let buffer="",served=0,cancelled=0,written:Promise<void>=Promise.resolve(),failure:unknown;
 const send=(response:WorkResponseV1)=>written=written.then(()=>options.write(encodeWorkResponseV1(response,options.maximumOutputBytes)));
 const start=(request:WorkRequestV1)=>{
  const controller=new AbortController(),done=(async()=>{
   let response:WorkResponseV1;
   try{const outcome=await options.handler(request,{signal:controller.signal});response={exitCode:outcome.exitCode,output:outcome.output,requestId:request.requestId};}
   catch(error){response={exitCode:1,output:error instanceof Error?error.message:String(error),requestId:request.requestId};}
   if(controller.signal.aborted){response={...response,wasCancelled:true};cancelled++;}
   await send(response);served++;
   try{await options.afterRequest?.(served);}catch(error){failure??=error;}
  })().finally(()=>{active.delete(request.requestId);pump();});
  active.set(request.requestId,{controller,done});
 };
 const pump=()=>{while(active.size<concurrency&&queue.length)start(queue.shift()!);};
 const dispatch=(request:WorkRequestV1)=>{
  if(request.cancel){
   const running=active.get(request.requestId);
   if(running){running.controller.abort(Error("Work request cancelled"));return;}
   const at=queue.findIndex(row=>row.requestId===request.requestId);
   if(at>=0){queue.splice(at,1);cancelled++;void send({exitCode:0,output:"",requestId:request.requestId,wasCancelled:true});}
   return;
  }
  queue.push(request);pump();
 };
 try{
  for await(const chunk of options.input){
   buffer+=typeof chunk==="string"?chunk:decoder.decode(chunk,{stream:true});
   if(buffer.length>maximumRequestBytes)throw new WorkerProtocolErrorV1("Work request exceeds its bound");
   for(let end=buffer.indexOf("\n");end>=0;end=buffer.indexOf("\n")){const line=buffer.slice(0,end).replace(/\r$/u,"");buffer=buffer.slice(end+1);if(line.trim())dispatch(parseWorkRequestV1(line));}
   if(failure)throw failure;
  }
  const rest=(buffer+decoder.decode()).trim();if(rest)dispatch(parseWorkRequestV1(rest));
 }catch(error){
  for(const row of active.values())row.controller.abort(error);
  queue.length=0;throw error;
 }finally{
  while(active.size||queue.length)await Promise.allSettled([...active.values()].map(row=>row.done));
  await written.catch(()=>{});
 }
 if(failure)throw failure;
 return Object.freeze({requests:served,cancelled});
}

/** 🧾️ Formats console arguments the way the console would, so a captured line equals the line it replaces. */
const formatWorkerLineV1=(values:readonly unknown[]):string=>formatText(...(values as [unknown,...unknown[]]));
/** 🎙️ Routes the standard output and error writes of the code a request runs into that request's output, so the protocol stream carries nothing but responses. */
export function installWorkerOutputCaptureV1(stdout:NodeJS.WriteStream=process.stdout,stderr:NodeJS.WriteStream=process.stderr):{run<T>(body:()=>Promise<T>):Promise<{value:T;output:string}>;writeProtocol(text:string):Promise<void>;restore():void}{
 const store=new AsyncLocalStorage<{chunks:string[]}>(),savedOut=stdout.write,savedErr=stderr.write,originalOut=savedOut.bind(stdout),originalErr=savedErr.bind(stderr);
 const capture=(fallback:typeof originalErr)=>(chunk:unknown,encoding?:unknown,callback?:unknown):boolean=>{
  const text=typeof chunk==="string"?chunk:Buffer.from(chunk as Uint8Array).toString("utf8"),done=typeof encoding==="function"?encoding:typeof callback==="function"?callback:undefined,row=store.getStore();
  if(row)row.chunks.push(text);else return fallback(text,(typeof encoding==="string"?encoding:"utf8") as BufferEncoding,done as never);
  if(done)queueMicrotask(()=>(done as ()=>void)());return true;
 };
 stdout.write=capture(originalErr) as never;stderr.write=capture(originalErr) as never;
 const savedConsole={log:console.log,info:console.info,debug:console.debug,warn:console.warn,error:console.error};
 console.log=console.info=console.debug=(...values:unknown[])=>{stdout.write(`${formatWorkerLineV1(values)}\n`);};console.warn=console.error=(...values:unknown[])=>{stderr.write(`${formatWorkerLineV1(values)}\n`);};
 return{
  async run(body){const row={chunks:[] as string[]};const value=await store.run(row,body);return{value,output:row.chunks.join("")};},
  writeProtocol:text=>new Promise<void>((accept,refuse)=>{originalOut(text,error=>error?refuse(error):accept());}),
  restore(){stdout.write=savedOut;stderr.write=savedErr;Object.assign(console,savedConsole);}
 };
}

/** ♻️ The result of preparing one module load. */
export type WorkerModuleLoadV1<T>=Readonly<{module:T;reset:boolean;retained:number}>;

/** ♻️ Reuses the modules a load brought in across requests while every one of them is unchanged, and drops all of them the moment one changed. A module is unchanged when the digest the build tool declared for it matches the digest recorded at load; where either side has none, when its size and modification time match. Only modules this cache loaded are ever dropped: the worker's own modules, `node_modules` and runtime built-ins stay. */
export class WorkerModuleCacheV1{
 readonly #root:string;readonly #owned=new Map<string,{stat:string;digest?:string}>();
 constructor(root:string){this.#root=resolve(root);}
 #real(path:string){try{return realpathSync(path);}catch{return path;}}
 #stat(path:string){try{const row=statSync(path);return `${row.size}:${row.mtimeMs}`;}catch{return "absent";}}
 #keys(){return Object.keys(require.cache).filter(path=>isAbsolute(path));}
 /** 🧹️ Drops every module this cache loaded and forgets its stamp. */
 reset():number{const count=this.#owned.size;for(const path of this.#owned.keys())delete require.cache[path];this.#owned.clear();return count;}
 /** 📥️ Loads `specifier` (an absolute path) after checking that nothing this cache loaded earlier changed. */
 async load<T>(specifier:string,inputs:readonly WorkInputV1[],base:string=this.#root):Promise<WorkerModuleLoadV1<T>>{
  const declared=new Map(inputs.map(input=>[this.#real(resolve(base,input.path)),input.digest] as const));
  let reset=false;
  for(const[path,stamp]of this.#owned){const digest=declared.get(path);if(digest!==undefined&&stamp.digest!==undefined?digest!==stamp.digest:this.#stat(path)!==stamp.stat){reset=true;break;}}
  if(reset)this.reset();
  const before=new Set(this.#keys()),module=await import(resolve(specifier)) as T;
  for(const path of this.#keys())if(!before.has(path)&&!path.split(sep).includes("node_modules")){const digest=declared.get(path);this.#owned.set(path,{stat:this.#stat(path),...(digest!==undefined?{digest}:{})});}
  for(const[path,stamp]of this.#owned){const digest=declared.get(path);if(stamp.digest===undefined&&digest!==undefined)this.#owned.set(path,{...stamp,digest});}
  return Object.freeze({module,reset,retained:this.#owned.size});
 }
}
