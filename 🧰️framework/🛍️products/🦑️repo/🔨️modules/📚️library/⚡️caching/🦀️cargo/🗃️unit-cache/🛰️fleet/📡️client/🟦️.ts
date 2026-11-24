import {createHash} from "node:crypto";
import {appendFileSync,mkdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";
import {fleetCacheRootV1,readLocalFleetCacheFactsV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";

const fleet=FLEET_POLICY_V1,EMPTY=createHash("sha256").digest("hex");
type Environment=Readonly<Record<string,string|undefined>>;

/** ⚙️ Where the fleet cache is and what this machine may do there; `timeoutMilliseconds` (`SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS`) caps the time of every call and `retries` (`SEMIO_REMOTE_CACHE_RETRIES`) the ride-through attempts after a failed one, both from the remote cache contract (absent: the fleet policy alone decides). */
export type FleetSettingsV1=Readonly<{endpoint:string;instance:string;token:string|undefined;upload:boolean;timeoutMilliseconds:number|undefined;retries:number|undefined;stateDirectory:string;signedRoot:string|undefined;identity:string}>;

const whole=(value:string|undefined,minimum:number):number|undefined=>{const number=value===undefined||!/^\d{1,6}$/u.test(value)?NaN:Number(value);return Number.isInteger(number)&&number>=minimum?number:undefined;};

/** 🔑️ The access the fleet facts of this machine grant, for a process whose environment names the endpoint but not the access (an empty value counts as not named): the same answer the one resolver of the remote cache gives, so such a process is no longer read-only by accident. */
function accessOfFactsV1(environment:Environment):string|undefined{
 try{const root=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);return root===undefined?undefined:readLocalFleetCacheFactsV1(root)?.facts.cache.access;}catch{return undefined;}
}

/** ⚙️ Reads the fleet tier's settings from the environment the launcher exports (`SEMIO_REMOTE_CACHE_*`, the variables of the remote cache contract); `undefined` when the tier is off: no endpoint, `SEMIO_UNIT_CACHE_REMOTE=off`, or a read-only access that never claims or uploads still downloads, so only the endpoint decides. */
export function readFleetSettingsV1(root:string,environment:Environment):FleetSettingsV1|undefined{
 const endpoint=environment.SEMIO_REMOTE_CACHE_ENDPOINT;
 if(!endpoint||environment.SEMIO_UNIT_CACHE_REMOTE==="off"||environment.SEMIO_UNIT_CACHE==="off")return undefined;
 const access=environment.SEMIO_REMOTE_CACHE_ACCESS||accessOfFactsV1(environment)||"read-only";
 return Object.freeze({endpoint:endpoint.replace(/\/+$/u,""),instance:environment.SEMIO_REMOTE_CACHE_INSTANCE||"main",token:environment.SEMIO_REMOTE_CACHE_TOKEN||undefined,upload:access==="read-write"&&environment.SEMIO_REMOTE_CACHE_UPLOAD!=="false",timeoutMilliseconds:whole(environment.SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS,1)===undefined?undefined:whole(environment.SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS,1)!*1000,retries:whole(environment.SEMIO_REMOTE_CACHE_RETRIES,0),stateDirectory:join(root,fleet.layout.state),signedRoot:environment.SEMIO_REMOTE_CACHE_SIGNED==="1"?environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined:undefined,identity:endpoint+"|"+(environment.SEMIO_REMOTE_CACHE_GENERATION??"")+"|"+(environment.SEMIO_REMOTE_CACHE_TOKEN?createHash("sha256").update(environment.SEMIO_REMOTE_CACHE_TOKEN).digest("hex").slice(0,8):"")});
}

/** 🚦️ Why a request did not give an answer: the server is unreachable or failing, refuses the credential, or does not know the route. */
export type FleetFaultV1="down"|"denied"|"unsupported";
/** 📮️ What the server answered to a write: `stored`; `capacity` (507, its store is full: retry after it evicted); `busy` (any other refusal that can pass: 5xx, 429, 401, 403); `refused` (400 or 413: the bytes will never be taken); `down` (no answer); `diverged` and `stale` for an action result only. */
export type FleetVerdictV1="stored"|"capacity"|"busy"|"refused"|"down"|"diverged"|"stale";
/** 🎟️ The server's answer to a claim, or the fault that makes the caller compute locally. */
export type FleetClaimV1={state:"granted";untilMilliseconds:number}|{state:"held";by:string;untilMilliseconds:number;expectedMilliseconds?:number}|{state:"stored"}|{state:"full"}|{state:"unavailable";fault:FleetFaultV1};

/** 📡️ The fleet cache as the unit cache uses it. Every call is bounded by a timeout and never throws, and a server that was reachable a moment ago and is not now (a restart takes a few seconds) is retried with jittered back-off for `rideThroughMilliseconds` before the call counts as a fault: a fault is an answer, and the client remembers it beside the cache so the next processes skip the tier for a while instead of each paying the timeout. */
export interface FleetClientV1{
 readonly settings:FleetSettingsV1;
 usable():boolean;
 state():"ok"|"down"|"denied";
 claimsUsable():boolean;
 getResult(key:string):Promise<Uint8Array|"miss"|FleetFaultV1>;
 hasResult(key:string):Promise<boolean|undefined>;
 getBlob(hash:string,size:number):Promise<Uint8Array|undefined>;
 hasBlob(hash:string):Promise<boolean|undefined>;
 putBlob(hash:string,bytes:Uint8Array):Promise<boolean>;
 storeBlob(hash:string,bytes:Uint8Array):Promise<FleetVerdictV1>;
 putResult(key:string,bytes:Uint8Array,slot:string):Promise<boolean|"diverged"|"stale">;
 storeResult(key:string,bytes:Uint8Array,slot:string):Promise<FleetVerdictV1>;
 claim(key:string,owner:string,ttlMilliseconds:number,expectedMilliseconds?:number):Promise<FleetClaimV1>;
 renew(key:string,owner:string,ttlMilliseconds:number):Promise<"granted"|"lost"|"fault">;
 release(key:string,owner:string):Promise<void>;
 claimSlot(key:string,slot:string):Promise<boolean>;
 await(key:string,timeoutMilliseconds:number):Promise<"stored"|"lapsed"|"timeout"|"fault">;
}

const sendable=(bytes:Uint8Array):Uint8Array<ArrayBuffer>=>bytes.buffer instanceof ArrayBuffer?new Uint8Array(bytes.buffer,bytes.byteOffset,bytes.byteLength):new Uint8Array(bytes);
function fresh(path:string,windowMs:number):boolean{try{return Date.now()-statSync(path).mtimeMs<windowMs;}catch{return false;}}
function stamp(directory:string,name:string,identity=""):void{try{mkdirSync(directory,{recursive:true});const path=join(directory,name);writeFileSync(path,identity);const now=new Date();utimesSync(path,now,now);}catch{}}

/** 📡️ Creates the client of one process. */
export function createFleetClientV1(settings:FleetSettingsV1,world:Readonly<{fetch?:typeof fetch}>={}):FleetClientV1{
 let signing:Promise<typeof fetch>|undefined;
 const signedFetch=(): Promise<typeof fetch>=>signing??=import("../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts").then(module=>module.fleetSignedFetchV1({root:settings.signedRoot!,audience:"cache"}));
 const request:typeof fetch=world.fetch??(settings.signedRoot===undefined?fetch:(((input:RequestInfo|URL,init?:RequestInit)=>signedFetch().then(send=>send(input,init))) as typeof fetch)),base=`${settings.endpoint}/${settings.instance}`,headers=(extra:Record<string,string>={}):Record<string,string>=>({...(settings.token?{authorization:"Bearer "+settings.token}:{}),...extra});
 const bounded=(milliseconds:number):number=>settings.timeoutMilliseconds===undefined?milliseconds:Math.min(milliseconds,settings.timeoutMilliseconds);
 const downNow=():boolean=>{try{const now=Date.now();return readFileSync(join(settings.stateDirectory,fleet.layout.down),"utf8").split("\n").filter(Boolean).filter(at=>now-Number(at)<fleet.breaker.downMilliseconds).length>=fleet.breaker.downFaults;}catch{return false;}};
 const deniedNow=():boolean=>{const path=join(settings.stateDirectory,fleet.layout.denied);if(!fresh(path,fleet.breaker.deniedMilliseconds))return false;try{return readFileSync(path,"utf8")===settings.identity;}catch{return false;}};
 const mark=(fault:FleetFaultV1)=>{
  if(fault==="down"){try{mkdirSync(settings.stateDirectory,{recursive:true});appendFileSync(join(settings.stateDirectory,fleet.layout.down),Date.now()+"\n");}catch{}}
  else if(fault==="denied")stamp(settings.stateDirectory,fleet.layout.denied,settings.identity);
  else stamp(settings.stateDirectory,fleet.layout.noClaims);
 };
 const faultOf=(status:number):FleetFaultV1=>status===401||status===403?"denied":status===404||status===405?"unsupported":"down";
 const reachablePath=join(settings.stateDirectory,fleet.layout.reachable);
 const reachableNow=():boolean=>fresh(reachablePath,fleet.breaker.reachableMilliseconds);
 let stampedAt=0;
 const reachable=():void=>{const at=Date.now();if(at-stampedAt<5000)return;stampedAt=at;stamp(settings.stateDirectory,fleet.layout.reachable);};
 const pause=(milliseconds:number)=>new Promise<void>(done=>{setTimeout(done,milliseconds);});
 async function call(method:string,url:string,timeout:number,init:{body?:BodyInit;headers?:Record<string,string>;transfer?:boolean}={}):Promise<{status:number;response:Response}|FleetFaultV1>{
  const started=Date.now(),budget=reachableNow()?fleet.timeouts.rideThroughMilliseconds:0;
  for(let attempt=0;;attempt++){
   try{
    const response=await request(url,{method,headers:headers(init.headers),signal:AbortSignal.timeout(init.transfer?timeout:bounded(timeout)),...(init.body===undefined?{}:{body:init.body})});
    reachable();
    if(response.status===401)mark("denied");
    return {status:response.status,response};
   }catch{
    const wait=Math.min(fleet.timeouts.rideThroughMaximumMilliseconds,fleet.timeouts.rideThroughInitialMilliseconds*2**attempt)*(0.5+Math.random());
    if(attempt>=(settings.retries??Infinity)||Date.now()-started+wait>budget){try{rmSync(reachablePath,{force:true});}catch{}mark("down");return "down";}
    await pause(wait);
   }
  }
 }
 const verdictOf=(answer:{status:number}|FleetFaultV1):FleetVerdictV1=>{
  if(typeof answer==="string")return "down";
  const status=answer.status;
  return status===200?"stored":status===507?"capacity":status===409?"diverged":status===422?"stale":status===400||status===413?"refused":"busy";
 };
 const storeBlob=async(hash:string,bytes:Uint8Array):Promise<FleetVerdictV1>=>{
  if(hash===EMPTY)return "stored";
  return verdictOf(await call("PUT",`${base}/cas/${hash}`,blobTimeout(bytes.length),{body:sendable(bytes),headers:{"content-type":"application/octet-stream"},transfer:true}));
 };
 const storeResult=async(key:string,bytes:Uint8Array,slot:string):Promise<FleetVerdictV1>=>verdictOf(await call("PUT",`${base}/ac/${key}`,fleet.timeouts.claimMilliseconds*4,{body:sendable(bytes),headers:{"content-type":"application/octet-stream",[fleet.slotHeader]:slot}}));
 const json=async<T>(response:Response):Promise<T|undefined>=>{try{return await response.json() as T;}catch{return undefined;}};
 const blobTimeout=(size:number)=>bounded(fleet.timeouts.blobMilliseconds)+Math.ceil(size/fleet.timeouts.blobBytesPerSecond*1000);
 return Object.freeze<FleetClientV1>({
  settings,
  state(){return downNow()?"down":deniedNow()?"denied":"ok";},
  usable(){return!downNow()&&!deniedNow();},
  claimsUsable(){return settings.upload&&!fresh(join(settings.stateDirectory,fleet.layout.noClaims),fleet.breaker.noClaimsMilliseconds);},
  async getResult(key){
   const answer=await call("GET",`${base}/ac/${key}`,fleet.timeouts.lookupMilliseconds);
   if(typeof answer==="string")return answer;
   if(answer.status===404)return "miss";
   if(answer.status!==200){const fault=faultOf(answer.status);mark(fault==="unsupported"?"down":fault);return fault==="unsupported"?"down":fault;}
   try{return new Uint8Array(await answer.response.arrayBuffer());}catch{mark("down");return "down";}
  },
  async hasResult(key){
   const answer=await call("HEAD",`${base}/ac/${key}`,fleet.timeouts.lookupMilliseconds);
   if(typeof answer==="string")return undefined;
   return answer.status===200?true:answer.status===404?false:undefined;
  },
  async getBlob(hash,size){
   if(hash===EMPTY)return new Uint8Array(0);
   const answer=await call("GET",`${base}/cas/${hash}`,blobTimeout(size),{transfer:true});
   if(typeof answer==="string"||answer.status!==200)return undefined;
   try{
    const bytes=new Uint8Array(await answer.response.arrayBuffer());
    return bytes.length===size&&createHash("sha256").update(bytes).digest("hex")===hash?bytes:undefined;
   }catch{return undefined;}
  },
  async hasBlob(hash){
   if(hash===EMPTY)return true;
   const answer=await call("HEAD",`${base}/cas/${hash}`,fleet.timeouts.headMilliseconds);
   if(typeof answer==="string")return undefined;
   return answer.status===200?true:answer.status===404?false:undefined;
  },
  storeBlob,
  async putBlob(hash,bytes){return await storeBlob(hash,bytes)==="stored";},
  storeResult,
  async putResult(key,bytes,slot){
   const verdict=await storeResult(key,bytes,slot);
   return verdict==="diverged"||verdict==="stale"?verdict:verdict==="stored";
  },
  async claim(key,owner,ttlMilliseconds,expectedMilliseconds){
   const answer=await call("POST",`${base}/claim/ac/${key}`,fleet.timeouts.claimMilliseconds,{body:JSON.stringify({owner,ttlMilliseconds,...(expectedMilliseconds===undefined?{}:{expectedMilliseconds})})});
   if(typeof answer==="string")return {state:"unavailable",fault:answer};
   const body=await json<FleetClaimV1 & {untilMilliseconds?:number}>(answer.response);
   if(answer.status===200&&body?.state==="granted")return {state:"granted",untilMilliseconds:Number(body.untilMilliseconds)};
   if(answer.status===200&&body?.state==="stored")return {state:"stored"};
   if(answer.status===409&&body?.state==="held")return {state:"held",by:String((body as {by?:string}).by),untilMilliseconds:Number(body.untilMilliseconds),...((body as {expectedMilliseconds?:number}).expectedMilliseconds===undefined?{}:{expectedMilliseconds:Number((body as {expectedMilliseconds?:number}).expectedMilliseconds)})};
   if(answer.status===503&&body?.state==="full")return {state:"full"};
   const fault=faultOf(answer.status);mark(fault);
   return {state:"unavailable",fault};
  },
  async renew(key,owner,ttlMilliseconds){
   const answer=await call("POST",`${base}/claim/ac/${key}/renew`,fleet.timeouts.claimMilliseconds,{body:JSON.stringify({owner,ttlMilliseconds})});
   if(typeof answer==="string")return "fault";
   return answer.status===200?"granted":answer.status===409?"lost":"fault";
  },
  async claimSlot(key,slot){
   if(!settings.upload)return false;
   const answer=await call("POST",`${base}/v1/slots`,fleet.timeouts.claimMilliseconds,{body:JSON.stringify({entries:[{kind:"ac",key,slot}]}),headers:{"content-type":"application/json"}});
   return typeof answer!=="string"&&answer.status===200;
  },
  async release(key,owner){await call("POST",`${base}/claim/ac/${key}/release`,fleet.timeouts.claimMilliseconds,{body:JSON.stringify({owner})});},
  async await(key,timeoutMilliseconds){
   const slice=Math.min(fleet.timeouts.awaitSliceMilliseconds,Math.max(0,Math.floor(timeoutMilliseconds)));
   const answer=await call("GET",`${base}/claim/ac/${key}/await?timeoutMilliseconds=${slice}`,slice+fleet.timeouts.claimMilliseconds*2);
   if(typeof answer==="string"||answer.status!==200)return "fault";
   const body=await json<{state?:string}>(answer.response);
   return body?.state==="stored"||body?.state==="lapsed"||body?.state==="timeout"?body.state:"fault";
  },
 });
}
