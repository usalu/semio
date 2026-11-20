import {createHash} from "node:crypto";
import {mkdirSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {GRAPH_STATE_POLICY_V1 as policy} from "../../🟦️.ts";
import {graphRefusalOfV1,type GraphRefusalV1} from "../🧯️refusal/🟦️.ts";

const EMPTY=createHash("sha256").digest("hex");
/** ⚙️ Where the fleet cache is and what this machine may do there; `stateDirectory` holds the markers of the unreachable-or-denied breaker shared by the processes of one machine. */
export type GraphFleetSettingsV1=Readonly<{endpoint:string;instance:string;token:string|undefined;upload:boolean;stateDirectory:string}>;
/** 🚦️ Why a request gave no answer. */
export type GraphFleetFaultV1="down"|"denied"|"unsupported";
export type GraphLookupFaultV1="unavailable"|"transport"|"denied"|"record"|"blob-missing"|"blob-integrity"|"payload"|"unknown";
type GraphBlobFaultV1=Extract<GraphLookupFaultV1,"transport"|"denied"|"blob-missing"|"blob-integrity">;
type GraphResponseV1={status:number;response:Response;signal:AbortSignal};
/** 🎟️ The answer to a claim. */
export type GraphFleetClaimV1={state:"granted";untilMilliseconds:number}|{state:"held";by:string;untilMilliseconds:number;expectedMilliseconds?:number}|{state:"stored"}|{state:"full"}|{state:"unavailable";fault:GraphFleetFaultV1};

/** 📥️ What a write ended with: stored, or the refusal with its true reason (the status and first words of the answer of the cache, no answer in time, no connection). */
export type GraphPutV1=Readonly<{stored:true}>|Readonly<{stored:false;refusal:GraphRefusalV1}>;

/** 📡️ The fleet cache as the graph exchange uses it. Every call is bounded by a timeout and never throws; a fault is an answer and is remembered beside the cache so the next processes skip the tier for a while. */
export interface GraphFleetClientV1{
 readonly settings:GraphFleetSettingsV1;
 usable():boolean;
 breakerRemainingMilliseconds?():number;
 claimsUsable():boolean;
 getResult(key:string,signal?:AbortSignal):Promise<Uint8Array|"miss"|GraphFleetFaultV1>;
 getBlob(hash:string,size:number,signal?:AbortSignal,onFault?:(reason:GraphBlobFaultV1)=>void):Promise<Uint8Array|undefined>;
 hasBlob(hash:string):Promise<boolean|undefined>;
 putBlob(hash:string,bytes:Uint8Array):Promise<GraphPutV1>;
 putResult(key:string,bytes:Uint8Array,slot:string,expect?:string):Promise<GraphPutV1>;
 claim(key:string,owner:string,ttlMilliseconds:number,expectedMilliseconds?:number):Promise<GraphFleetClaimV1>;
 renew(key:string,owner:string,ttlMilliseconds:number):Promise<"granted"|"lost"|"fault">;
 release(key:string,owner:string):Promise<void>;
 await(key:string,timeoutMilliseconds:number,signal?:AbortSignal):Promise<"stored"|"lapsed"|"timeout"|"fault">;
}

const body=(bytes:Uint8Array):BodyInit=>isOwned(bytes)?bytes:new Uint8Array(bytes),isOwned=(bytes:Uint8Array):bytes is Uint8Array<ArrayBuffer>=>bytes.buffer instanceof ArrayBuffer;
const fresh=(path:string,window:number):boolean=>{try{return Date.now()-statSync(path).mtimeMs<window;}catch{return false;}};
const remaining=(path:string,window:number):number=>{try{return Math.max(0,Math.ceil(window-(Date.now()-statSync(path).mtimeMs)));}catch{return 0;}};
const stamp=(directory:string,name:string):void=>{try{mkdirSync(directory,{recursive:true});const path=join(directory,name);writeFileSync(path,String(Date.now()));const now=new Date();utimesSync(path,now,now);}catch{}};

async function readBlobBytesV1(response:Response,size:number,signal:AbortSignal):Promise<Uint8Array|undefined>{
 const reader=response.body?.getReader();
 if(reader===undefined){signal.throwIfAborted();return undefined;}
 const cancel=()=>{void reader.cancel().catch(()=>undefined);};
 signal.addEventListener("abort",cancel,{once:true});
 try{
  signal.throwIfAborted();
  let bytes:Uint8Array|undefined,written=0;
  for(;;){
   const {done,value}=await reader.read();
   signal.throwIfAborted();
   if(done)return written===size?bytes:undefined;
   if(value.byteLength>size-written)return undefined;
   if(value.byteLength===0)continue;
   (bytes??=new Uint8Array(size)).set(value,written);
   written+=value.byteLength;
  }
 }finally{
  signal.removeEventListener("abort",cancel);
  cancel();
  reader.releaseLock();
 }
}

/** 📡️ Creates the client of one process. */
export function createGraphFleetClientV1(settings:GraphFleetSettingsV1,world:Readonly<{fetch?:typeof fetch}>={}):GraphFleetClientV1{
 const request=world.fetch??fetch,base=`${settings.endpoint}/${settings.instance}`,t=policy.timeouts,b=policy.breaker,headers=(extra:Record<string,string>={}):Record<string,string>=>({...(settings.token?{authorization:"Bearer "+settings.token}:{}),...extra});
 const faultOf=(status:number):GraphFleetFaultV1=>status===401||status===403?"denied":status===404||status===405?"unsupported":"down";
 const mark=(fault:GraphFleetFaultV1)=>stamp(settings.stateDirectory,fault==="denied"?"denied":fault==="unsupported"?"noclaims":"down");
 async function call(method:string,url:string,timeout:number,init:{body?:BodyInit;headers?:Record<string,string>}={},signal?:AbortSignal):Promise<GraphResponseV1|GraphFleetFaultV1>{
  const answer=await attempt(method,url,timeout,init,signal);
  if("error" in answer){mark("down");return "down";}
  return answer;
 }
 async function attempt(method:string,url:string,timeout:number,init:{body?:BodyInit;headers?:Record<string,string>},signal?:AbortSignal):Promise<GraphResponseV1|{error:"timeout"|"network"}>{
  signal?.throwIfAborted();
  const deadline=AbortSignal.timeout(timeout),requestSignal=signal===undefined?deadline:AbortSignal.any([signal,deadline]);
  try{const response=await request(url,{method,headers:headers(init.headers),signal:requestSignal,...(init.body===undefined?{}:{body:init.body})});return{status:response.status,response,signal:requestSignal};}
  catch(error){signal?.throwIfAborted();return{error:(error as {name?:string})?.name==="TimeoutError"||(error as {name?:string})?.name==="AbortError"?"timeout":"network"};}
 }
 async function put(url:string,timeout:number,init:{body:BodyInit;headers:Record<string,string>}):Promise<GraphPutV1>{
  const answer=await attempt("PUT",url,timeout,init);
  if("error" in answer){mark("down");return{stored:false,refusal:graphRefusalOfV1(answer.error)};}
  if(answer.status===200)return{stored:true};
  let body="";try{body=await answer.response.text();}catch{}
  return{stored:false,refusal:graphRefusalOfV1(answer.status,body)};
 }
 const json=async<T>(response:Response):Promise<T|undefined>=>{try{return await response.json() as T;}catch{return undefined;}};
 const blobTimeout=(size:number)=>t.blobMilliseconds+Math.ceil(size/t.blobBytesPerSecond*1000);
 return Object.freeze<GraphFleetClientV1>({
  settings,
  usable(){return!fresh(join(settings.stateDirectory,"down"),b.downMilliseconds)&&!fresh(join(settings.stateDirectory,"denied"),b.deniedMilliseconds);},
  breakerRemainingMilliseconds(){return Math.max(remaining(join(settings.stateDirectory,"down"),b.downMilliseconds),remaining(join(settings.stateDirectory,"denied"),b.deniedMilliseconds));},
  claimsUsable(){return settings.upload&&!fresh(join(settings.stateDirectory,"noclaims"),b.noClaimsMilliseconds);},
  async getResult(key,signal){
   const answer=await call("GET",`${base}/ac/${key}`,t.lookupMilliseconds,{},signal);
   if(typeof answer==="string")return answer;
   if(answer.status!==200||signal?.aborted){await answer.response.body?.cancel().catch(()=>undefined);signal?.throwIfAborted();}
   if(answer.status===404)return "miss";
   if(answer.status!==200){const fault=faultOf(answer.status);mark(fault==="unsupported"?"down":fault);return fault==="unsupported"?"down":fault;}
   try{const bytes=new Uint8Array(await answer.response.arrayBuffer());signal?.throwIfAborted();return bytes;}catch{signal?.throwIfAborted();mark("down");return "down";}
  },
  async getBlob(hash,size,signal,onFault){
   signal?.throwIfAborted();
   if(!Number.isSafeInteger(size)||size<0||size>policy.blobMaximumBytes||(hash===EMPTY)!==(size===0)){onFault?.("blob-integrity");return undefined;}
   if(size===0)return new Uint8Array(0);
   const answer=await call("GET",`${base}/cas/${hash}`,blobTimeout(size),{},signal);
   if(typeof answer==="string"){onFault?.("transport");return undefined;}
   if(answer.status!==200||signal?.aborted){await answer.response.body?.cancel().catch(()=>undefined);signal?.throwIfAborted();onFault?.(answer.status===404?"blob-missing":faultOf(answer.status)==="denied"?"denied":"transport");return undefined;}
   let bytes:Uint8Array|undefined;
   try{bytes=await readBlobBytesV1(answer.response,size,answer.signal);}catch{signal?.throwIfAborted();onFault?.("transport");return undefined;}
   signal?.throwIfAborted();
   if(bytes!==undefined&&createHash("sha256").update(bytes).digest("hex")===hash)return bytes;
   onFault?.("blob-integrity");return undefined;
  },
  async hasBlob(hash){
   if(hash===EMPTY)return true;
   const answer=await call("HEAD",`${base}/cas/${hash}`,t.headMilliseconds);
   if(typeof answer==="string")return undefined;
   return answer.status===200?true:answer.status===404?false:undefined;
  },
  async putBlob(hash,bytes){
   if(hash===EMPTY)return{stored:true};
   return put(`${base}/cas/${hash}`,blobTimeout(bytes.length),{body:body(bytes),headers:{"content-type":"application/octet-stream"}});
  },
  async putResult(key,bytes,slot,expect){
   return put(`${base}/ac/${key}`,t.claimMilliseconds*4,{body:body(bytes),headers:{"content-type":"application/octet-stream",[policy.slotHeader]:slot,...(expect===undefined?{}:{"x-semio-expect":expect})}});
  },
  async claim(key,owner,ttlMilliseconds,expectedMilliseconds){
   const answer=await call("POST",`${base}/claim/ac/${key}`,t.claimMilliseconds,{body:JSON.stringify({owner,ttlMilliseconds,...(expectedMilliseconds===undefined?{}:{expectedMilliseconds})})});
   if(typeof answer==="string")return{state:"unavailable",fault:answer};
   const body=await json<Record<string,unknown>>(answer.response);
   if(answer.status===200&&body?.state==="granted")return{state:"granted",untilMilliseconds:Number(body.untilMilliseconds)};
   if(answer.status===200&&body?.state==="stored")return{state:"stored"};
   if(answer.status===409&&body?.state==="held")return{state:"held",by:String(body.by),untilMilliseconds:Number(body.untilMilliseconds),...(body.expectedMilliseconds===undefined?{}:{expectedMilliseconds:Number(body.expectedMilliseconds)})};
   if(answer.status===503&&body?.state==="full")return{state:"full"};
   const fault=faultOf(answer.status);mark(fault);
   return{state:"unavailable",fault};
  },
  async renew(key,owner,ttlMilliseconds){
   const answer=await call("POST",`${base}/claim/ac/${key}/renew`,t.claimMilliseconds,{body:JSON.stringify({owner,ttlMilliseconds})});
   if(typeof answer==="string")return "fault";
   return answer.status===200?"granted":answer.status===409?"lost":"fault";
  },
  async release(key,owner){await call("POST",`${base}/claim/ac/${key}/release`,t.claimMilliseconds,{body:JSON.stringify({owner})});},
  async await(key,timeoutMilliseconds,signal){
   const slice=Math.min(t.awaitSliceMilliseconds,Math.max(0,Math.floor(timeoutMilliseconds)));
   const answer=await call("GET",`${base}/claim/ac/${key}/await?timeoutMilliseconds=${slice}`,slice+t.claimMilliseconds*2,{},signal);
   if(typeof answer!=="string"&&(signal?.aborted||answer.status!==200))await answer.response.body?.cancel().catch(()=>undefined);
   signal?.throwIfAborted();
   if(typeof answer==="string"||answer.status!==200)return "fault";
   const body=await json<{state?:string}>(answer.response);
   signal?.throwIfAborted();
   return body?.state==="stored"||body?.state==="lapsed"||body?.state==="timeout"?body.state:"fault";
  },
 });
}
