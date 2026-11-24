import type {RemoteExecutionDigestV1} from "../🟦️.ts";
import {CAS_LEASE_HEADER_V1} from "../../🗄️cas/📌️pins/🟦️.ts";
import { runtimeSleepV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";

/** 📥️ Why a blob is asked for or sent: `input` marks a blob that only feeds one action, which the store leases for the time of the action instead of keeping it like a published output; `input:<action>` names the action that holds the lease, so the lease of one action can end (`release`) while another action still uses the blob. */
export type RemoteExecutionContentIntentV1="input"|`input:${string}`;

/** 🗄️ Where the protocol's blobs and action results live: the shared content-addressed store. Scheduler and workers use this interface only, so a store can be swapped without touching either. */
export interface RemoteExecutionContentV1{
 has(instance:string,hash:string,signal?:AbortSignal,intent?:RemoteExecutionContentIntentV1):Promise<boolean>;
 missing(instance:string,digests:readonly RemoteExecutionDigestV1[],signal?:AbortSignal,intent?:RemoteExecutionContentIntentV1):Promise<RemoteExecutionDigestV1[]>;
 read(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<Uint8Array|undefined>;
 stream(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<AsyncGenerator<Uint8Array>|undefined>;
 write(instance:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>,signal?:AbortSignal,intent?:RemoteExecutionContentIntentV1):Promise<void>;
 readActionResult(instance:string,hash:string,signal?:AbortSignal):Promise<Uint8Array|undefined>;
 writeActionResult(instance:string,hash:string,bytes:Uint8Array,signal?:AbortSignal):Promise<void>;
 release?(instance:string,hashes:readonly string[],intent:RemoteExecutionContentIntentV1,signal?:AbortSignal):Promise<void>;
}

/** 🚧️ The store refused or could not take the request. `code` is a gRPC status code and `missingOutput` names a blob explicitly rejected during action-result validation. */
export class RemoteExecutionContentErrorV1 extends Error{
 readonly code:number;
 readonly missingOutput?:string|undefined;
 readonly status?:number|undefined;
 constructor(code:number,message:string,missingOutput?:string,status?:number){
  super(message);
  this.code=code;
  this.missingOutput=missingOutput;
  this.status=status;
 }
}

/** ⚙️ How to reach the shared store over its HTTP protocol (`/<instance>/cas/<sha256>`, `/<instance>/ac/<sha256>`, bearer token). */
export type RemoteExecutionHttpContentOptionsV1=Readonly<{endpoint:string;token?:string|undefined;fetch?:typeof fetch;concurrency?:number;timeoutSeconds?:number;retries?:number;retryAfterCapMilliseconds?:number}>;

const EMPTY="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,TRANSIENT=new Set([408,429,500,502,503,504]);

async function* chunksOf(body:ReadableStream<Uint8Array>):AsyncGenerator<Uint8Array>{
 const reader=body.getReader();
 try{for(;;){const {done,value}=await reader.read();if(done)return;yield value;}}finally{await reader.cancel().catch(()=>{});}
}

/** 🌐️ The store of the repository's own cache server (and any server speaking Bazel's HTTP cache protocol), as `RemoteExecutionContentV1`. Retries transient failures, bounds parallel existence checks, sends the declared length of every blob and streams large blobs in both directions. */
export function httpRemoteExecutionContentV1(options:RemoteExecutionHttpContentOptionsV1):RemoteExecutionContentV1{
 const base=options.endpoint.replace(/\/+$/u,""),limit=options.concurrency??32,retries=options.retries??3,retryAfterCap=options.retryAfterCapMilliseconds??5000,timeout=(options.timeoutSeconds??60)*1000,headers=(extra:Record<string,string>={}):Record<string,string>=>({...(options.token?{authorization:`Bearer ${options.token}`}:{}),...extra});
 const url=(instance:string,kind:"ac"|"cas",hash:string):string=>{
  if(instance!==""&&!INSTANCE.test(instance))throw new RemoteExecutionContentErrorV1(3,`Valid instance name required: ${instance}`);
  return `${base}${instance===""?"":"/"+instance}/${kind}/${hash}`;
 };
 const send=async(method:string,target:string,signal:AbortSignal|undefined,init:Partial<RequestInit>={},retry=true):Promise<Response>=>{
  let last:unknown;
  for(let attempt=0;attempt<=(retry?retries:0);attempt++){
   try{
    const response=await(options.fetch??fetch)(target,{method,...init,headers:{...headers(),...(init.headers as Record<string,string>|undefined)},signal:signal?AbortSignal.any([signal,AbortSignal.timeout(timeout)]):AbortSignal.timeout(timeout)});
    if(TRANSIENT.has(response.status)&&attempt<retries){const hint=Number(response.headers.get("retry-after"));await response.body?.cancel();await runtimeSleepV1(Math.max(50*2**attempt,Number.isFinite(hint)&&hint>0?Math.min(hint*1000,retryAfterCap):0));continue;}
    return response;
   }catch(error){
    if(signal?.aborted)throw error;
    last=error;if(attempt<retries)await runtimeSleepV1(50*2**attempt);
   }
  }
  throw new RemoteExecutionContentErrorV1(14,`Content store unreachable: ${base}: ${last instanceof Error?last.message:String(last)}`);
 };
 const reasonOf=async(response:Response):Promise<string>=>{try{return (await response.text()).replace(/\s+/gu," ").trim().slice(0,200);}catch{return "";}};
 const settle=async(response:Response,what:string):Promise<void>=>{if(response.status===200){await response.body?.cancel();return;}refuse(response,what,await reasonOf(response));};
 const refuse=(response:Response,what:string,reason=""):never=>{
  const code=response.status===401?16:response.status===403?7:response.status===507?8:response.status===413?8:response.status===400?3:14;
  const missingOutput=response.status===400&&what.startsWith("ac/")?/^Invalid action result: (?:blob|tree) ([0-9a-f]{64})(?: of tree [0-9a-f]{64})? is not in the content store$/u.exec(reason)?.[1]:undefined;
  throw new RemoteExecutionContentErrorV1(code,`Content store answered ${response.status} for ${what}${reason===""?"":": "+reason}`,missingOutput,response.status);
 };
 const content:RemoteExecutionContentV1={
  async has(instance,hash,signal,intent){
   if(hash===EMPTY)return true;
   const response=await send("HEAD",url(instance,"cas",hash),signal,intent===undefined?{}:{headers:{[CAS_LEASE_HEADER_V1]:intent}});await response.body?.cancel();
   if(response.status===200)return true;
   if(response.status===404)return false;
   return refuse(response,`cas/${hash}`);
  },
  async missing(instance,digests,signal,intent){
   const out:RemoteExecutionDigestV1[]=new Array(digests.length),found:boolean[]=new Array(digests.length);let next=0;
   await Promise.all(Array.from({length:Math.min(limit,digests.length)},async()=>{for(;;){const index=next++;if(index>=digests.length)return;found[index]=await content.has(instance,digests[index]!.hash,signal,intent);}}));
   let count=0;
   digests.forEach((digest,index)=>{if(!found[index])out[count++]=digest;});
   return out.slice(0,count);
  },
  async read(instance,digest,signal){
   if(digest.size===0&&digest.hash===EMPTY)return new Uint8Array(0);
   const response=await send("GET",url(instance,"cas",digest.hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`cas/${digest.hash}`);
   return new Uint8Array(await response.arrayBuffer());
  },
  async stream(instance,digest,signal){
   if(digest.size===0&&digest.hash===EMPTY)return (async function*(){})();
   const response=await send("GET",url(instance,"cas",digest.hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`cas/${digest.hash}`);
   return response.body?chunksOf(response.body):(async function*(){})();
  },
  async write(instance,digest,source,signal,intent){
   if(digest.size===0&&digest.hash===EMPTY)return;
   const target=url(instance,"cas",digest.hash),request={"content-length":String(digest.size),"content-type":"application/octet-stream",...intent===undefined?{}:{[CAS_LEASE_HEADER_V1]:intent}};
   if(source instanceof Uint8Array){const response=await send("PUT",target,signal,{body:source as unknown as BodyInit,headers:request});await settle(response,`cas/${digest.hash}`);return;}
   const iterator=source[Symbol.asyncIterator](),body=new ReadableStream<Uint8Array>({async pull(controller){const next=await iterator.next();if(next.done)controller.close();else controller.enqueue(next.value);},cancel(){void iterator.return?.();}});
   const response=await send("PUT",target,signal,{body,headers:request,duplex:"half"} as RequestInit,false);await settle(response,`cas/${digest.hash}`);
  },
  async readActionResult(instance,hash,signal){
   const response=await send("GET",url(instance,"ac",hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`ac/${hash}`);
   return new Uint8Array(await response.arrayBuffer());
  },
  async release(instance,hashes,intent,signal){
   let next=0;
   await Promise.all(Array.from({length:Math.min(limit,hashes.length)},async()=>{for(;;){const index=next++;if(index>=hashes.length)return;const hash=hashes[index]!;if(hash===EMPTY)continue;const response=await send("DELETE",url(instance,"cas",hash),signal,{headers:{[CAS_LEASE_HEADER_V1]:intent}});await response.body?.cancel();}}));
  },
  async writeActionResult(instance,hash,bytes,signal){
   const response=await send("PUT",url(instance,"ac",hash),signal,{body:bytes as unknown as BodyInit,headers:{"content-length":String(bytes.length),"content-type":"application/octet-stream"}});await settle(response,`ac/${hash}`);
  },
 };
 return Object.freeze(content);
}

/** ⏳️ How long and how patiently a transfer waits for an unreachable store (the policy's `worker.outage`). */
export type RemoteExecutionContentOutageV1=Readonly<{seconds:number;initialMilliseconds:number;maximumMilliseconds:number}>;

const retryableStatusV1=(error:RemoteExecutionContentErrorV1):boolean=>error.status===507||(error.code===14&&(error.status===undefined||error.status>=500||error.status===408||error.status===429));

/** 🔁️ Runs `operation` again while the store is unreachable (gRPC code 14 from {@link RemoteExecutionContentErrorV1}) or answers 507 Insufficient storage (the store collects room meanwhile) with a pause that doubles from the outage's initial to its maximum, for at most its seconds from the first failure, so the work already done on a finished action survives a restart of the store; any other failure, an abort and an expired outage end it at once with the last error. The operation builds its source anew per attempt.
 * @see ../🧪️tests/🔁️content-retry/🟦️.ts */
export async function retryRemoteExecutionContentOutageV1<T>(operation:()=>Promise<T>,outage:RemoteExecutionContentOutageV1,signal?:AbortSignal,sleep:(milliseconds:number)=>Promise<void>=runtimeSleepV1,now:()=>number=Date.now):Promise<T>{
 let began:number|undefined,pause=outage.initialMilliseconds;
 for(;;){
  try{return await operation();}
  catch(error){
   if(!(error instanceof RemoteExecutionContentErrorV1)||!retryableStatusV1(error)||signal?.aborted===true)throw error;
   began??=now();
   const left=outage.seconds*1000-(now()-began);
   if(left<=0)throw error;
   await sleep(Math.min(pause,left));pause=Math.min(pause*2,outage.maximumMilliseconds);
  }
 }
}

/** 🛟️ The store of `content` that waits out an outage on every transfer whose input can be replayed: existence checks, reads and byte writes; a stream is handed through (its owner retries by building a new source with {@link retryRemoteExecutionContentOutageV1}). */
export function outageTolerantRemoteExecutionContentV1(content:RemoteExecutionContentV1,outage:RemoteExecutionContentOutageV1,sleep?:(milliseconds:number)=>Promise<void>):RemoteExecutionContentV1{
 const again=<T>(operation:()=>Promise<T>,signal?:AbortSignal):Promise<T>=>retryRemoteExecutionContentOutageV1(operation,outage,signal,sleep);
 return Object.freeze<RemoteExecutionContentV1>({
  has:(instance,hash,signal,intent)=>again(()=>content.has(instance,hash,signal,intent),signal),
  missing:(instance,digests,signal,intent)=>again(()=>content.missing(instance,digests,signal,intent),signal),
  read:(instance,digest,signal)=>again(()=>content.read(instance,digest,signal),signal),
  stream:(instance,digest,signal)=>content.stream(instance,digest,signal),
  write:(instance,digest,source,signal,intent)=>source instanceof Uint8Array?again(()=>content.write(instance,digest,source,signal,intent),signal):content.write(instance,digest,source,signal,intent),
  readActionResult:(instance,hash,signal)=>again(()=>content.readActionResult(instance,hash,signal),signal),
  writeActionResult:(instance,hash,bytes,signal)=>again(()=>content.writeActionResult(instance,hash,bytes,signal),signal),
  ...content.release===undefined?{}:{release:(instance:string,hashes:readonly string[],intent:RemoteExecutionContentIntentV1,signal?:AbortSignal)=>content.release!(instance,hashes,intent,signal)},
 });
}
