import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash,randomBytes} from "node:crypto";
import {createReadStream,createWriteStream,constants} from "node:fs";
import {appendFile,copyFile,mkdir,open,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {Readable} from "node:stream";
import {remoteCacheFetchV1,resolveRemoteCacheV1,fleetStoreRootV1} from "../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {claimFleetWorkV1,WORK_CLAIM_POLICY_V1,type WorkClaimEventV1} from "./🎟️work-claim/🟦️.ts";

/** 📐️ The authored fetch policy: store layout, timeouts, size limits and environment names.
 * @see ./🧬️schema/🔣️.json */
export const FLEET_FETCH_POLICY_V1=schema.definitions.Policy.const;
const policy=FLEET_FETCH_POLICY_V1,DIGEST=/^(?:[a-f0-9]{64}|[a-f0-9]{128})$/u,algorithmOf=(hex:string):"sha256"|"sha512"=>hex.length===128?"sha512":"sha256";

/** 🌍️ Everything the fetch layer needs: the environment, the worktree root, the group store, the cache endpoint with its credential and the clock. */
export type FleetFetchContextV1=Readonly<{
 environment:Readonly<Record<string,string|undefined>>;
 root:string|undefined;
 storeRoot:string|undefined;
 remote:Readonly<{endpoint:string;instance:string;token:string|undefined;authenticated:boolean}>|undefined;
 fetch:typeof fetch;
 now:()=>number;
 say?:(line:string)=>void;
}>;

/** 🧾️ One answered or uploaded blob, as the ledger records it. */
export type FleetFetchLedgerEntryV1=Readonly<{version:1;atMs:number;sha256:string;source:"local"|"store"|"http"|"upstream";bytes:number;milliseconds:number;direction?:"get"|"put";url?:string}>;

/** 🏗️ Builds the context of a process: `SEMIO_FLEET_FETCH=off` disables the fleet tiers, `SEMIO_FLEET_STORE_ROOT` overrides the store of `.cache/f/machine.json`, the cache endpoint and credential come from the one resolver every client of the cache uses. */
export function fleetFetchContextV1(environment:Readonly<Record<string,string|undefined>>=process.env,root:string|undefined=environment.SEMIO_WORKSPACE_ROOT,overrides:Partial<Pick<FleetFetchContextV1,"fetch"|"now"|"storeRoot"|"remote">>={}):FleetFetchContextV1{
 const off=environment[policy.offEnvironment]==="off";
 let remote=overrides.remote;
 if(!off&&!("remote" in overrides)){try{const resolved=resolveRemoteCacheV1(environment,root===undefined?{}:{root});if(resolved)remote={endpoint:resolved.contract.endpoint.replace(/\/+$/u,""),instance:resolved.contract.instance,token:resolved.token,authenticated:resolved.tokenFrom!=="none"};}catch{}}
 const named=environment[policy.storeEnvironment],storeRoot=off?undefined:"storeRoot" in overrides?overrides.storeRoot:named!==undefined&&named!==""?named:root===undefined?undefined:fleetStoreRootV1(root);
 return Object.freeze({environment,root,storeRoot,remote:off?undefined:remote,fetch:overrides.fetch??(remote?.authenticated===true&&!off?remoteCacheFetchV1(environment,root===undefined?{}:{root}):fetch),now:overrides.now??Date.now});
}

const shared=new Map<string,Readonly<{at:number;context:FleetFetchContextV1}>>();
/** ♻️ The context of a process, built once per worktree and renewed every thirty seconds, for callers that fetch thousands of small blobs. */
export function sharedFleetFetchContextV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):FleetFetchContextV1{
 const known=shared.get(root),at=Date.now();
 if(known&&at-known.at<30000)return known.context;
 const context=fleetFetchContextV1(environment,root);shared.set(root,{at,context});return context;
}

const sha256Hex=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex");
const guarded=async<T>(milliseconds:number,work:(signal:AbortSignal)=>Promise<T>):Promise<T|undefined>=>{
 const controller=new AbortController();let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([work(controller.signal),new Promise<undefined>(resolve=>{timer=setTimeout(()=>{controller.abort();resolve(undefined);},milliseconds);})]);}catch{return undefined;}finally{if(timer)clearTimeout(timer);}
};

/** 📒️ Appends one ledger line to the file `SEMIO_FLEET_FETCH_LEDGER` names; a ledger that cannot be written never fails a download. */
export async function recordFleetFetchV1(context:FleetFetchContextV1,entry:Omit<FleetFetchLedgerEntryV1,"version"|"atMs">):Promise<void>{
 const path=context.environment[policy.ledgerEnvironment];if(!path)return;
 try{await appendFile(path,JSON.stringify({version:1,atMs:context.now(),...entry})+"\n");}catch{}
}

/** 🗂️ The store path of a blob: `<store>/v1/fleet/fetch/<first two hex>/<sha256>`. */
export function fleetBlobStorePathV1(storeRoot:string,sha256:string):string{
 if(!DIGEST.test(sha256))throw Error("Lowercase sha256 required");
 return join(storeRoot,...policy.storeDirectory,sha256.slice(0,2),sha256);
}

/** 🔏️ Hashes a regular file as a stream; `undefined` when the file is unreadable. */
export async function hashFleetFileV1(path:string,signal?:AbortSignal,algorithm:"sha256"|"sha512"="sha256"):Promise<Readonly<{sha256:string;bytes:number}>|undefined>{
 try{
  const hash=createHash(algorithm);let bytes=0;
  for await(const chunk of createReadStream(path,{highWaterMark:1<<20,...(signal?{signal}:{})})){hash.update(chunk as Buffer);bytes+=(chunk as Buffer).byteLength;}
  return {sha256:hash.digest("hex"),bytes};
 }catch{return undefined;}
}

const markerPath=(context:FleetFetchContextV1,sha256:string):string|undefined=>context.root===undefined?undefined:machinePathV1(context.root,".cache","f","fetch-verified",sha256);
const stamp=(facts:{size:number;mtimeMs:number}):string=>`${facts.size}:${Math.floor(facts.mtimeMs)}`;

/** ✅️ Proves a store file holds its sha256, remembering the proof by size and modification time so a large blob is hashed once per machine. */
async function verifiedStoreFileV1(context:FleetFetchContextV1,path:string,sha256:string,bytes:number|undefined):Promise<number|undefined>{
 const facts=await guarded(policy.storeTimeoutMilliseconds,()=>stat(path));
 if(!facts||!facts.isFile()||facts.size<1||(bytes!==undefined&&facts.size!==bytes))return undefined;
 const marker=markerPath(context,sha256);
 if(marker){const known=await guarded(policy.storeTimeoutMilliseconds,()=>readFile(marker,"utf8"));if(known===stamp(facts))return facts.size;}
 const actual=await guarded(policy.storeTimeoutMilliseconds+Math.ceil(facts.size/(20*1048576))*1000,signal=>hashFleetFileV1(path,signal,algorithmOf(sha256)));
 if(!actual||actual.sha256!==sha256){await rm(path,{force:true}).catch(()=>{});return undefined;}
 if(marker)await mkdir(dirname(marker),{recursive:true}).then(()=>writeFile(marker,stamp(facts))).catch(()=>{});
 return facts.size;
}

/** 📖️ A verified blob ready to be read: where it came from, its size and a one-shot stream. */
export type FleetBlobV1=Readonly<{source:"store"|"http";bytes:number;path:string;dispose():Promise<void>}>;

/** 🔎️ Finds a blob by its sha256 in the group store, then in the cache's content-addressed store; the result is verified. The store file is read in place (no second copy); an HTTP hit is staged in a machine-local temporary file that `dispose` removes. `undefined` on every miss, timeout or failure. */
export async function fleetBlobOpenV1(context:FleetFetchContextV1,sha256:string,expectedBytes?:number):Promise<FleetBlobV1|undefined>{
 if(!DIGEST.test(sha256))return undefined;
 const started=context.now();
 if(context.storeRoot!==undefined){
  const path=fleetBlobStorePathV1(context.storeRoot,sha256),bytes=await verifiedStoreFileV1(context,path,sha256,expectedBytes);
  if(bytes!==undefined){await recordFleetFetchV1(context,{sha256,source:"store",bytes,milliseconds:context.now()-started,direction:"get"});return Object.freeze({source:"store",bytes,path,dispose:async()=>{}});}
 }
 if(context.remote!==undefined&&sha256.length===64&&(expectedBytes===undefined||expectedBytes<=policy.httpMaximumBytes)){
  const remote=context.remote,staging=context.root===undefined?undefined:machinePathV1(context.root,".cache","f","fetch-tmp"),temporary=staging===undefined?undefined:join(staging,randomBytes(8).toString("hex"));
  if(temporary!==undefined&&staging!==undefined){
   const bytes=await guarded(policy.httpTimeoutMilliseconds*4,async signal=>{
    await mkdir(staging,{recursive:true});
    const response=await context.fetch(`${remote.endpoint}/${remote.instance}/cas/${sha256}`,{signal,...(remote.token?{headers:{authorization:`Bearer ${remote.token}`}}:{})});
    if(response.status!==200||!response.body)return undefined;
    const hash=createHash("sha256"),file=createWriteStream(temporary,{flags:"wx",mode:0o600});let size=0;
    for await(const chunk of response.body as unknown as AsyncIterable<Uint8Array>){hash.update(chunk);size+=chunk.byteLength;if(!file.write(chunk))await new Promise<void>(resolve=>file.once("drain",()=>resolve()));}
    await new Promise<void>((resolve,reject)=>file.end((error?:Error|null)=>error?reject(error):resolve()));
    return hash.digest("hex")===sha256&&size>0&&(expectedBytes===undefined||size===expectedBytes)?size:undefined;
   });
   if(bytes!==undefined){await recordFleetFetchV1(context,{sha256,source:"http",bytes,milliseconds:context.now()-started,direction:"get"});return Object.freeze({source:"http",bytes,path:temporary,dispose:()=>rm(temporary,{force:true})});}
   await rm(temporary,{force:true}).catch(()=>{});
  }
 }
 return undefined;
}

/** 📥️ Reads a small blob by its sha256 into memory, or `undefined`. */
export async function fleetBlobBytesV1(context:FleetFetchContextV1,sha256:string,maximum:number=policy.httpMaximumBytes):Promise<Uint8Array|undefined>{
 const blob=await fleetBlobOpenV1(context,sha256);if(!blob)return undefined;
 try{if(blob.bytes>maximum)return undefined;return new Uint8Array(await readFile(blob.path));}catch{return undefined;}finally{await blob.dispose();}
}

/** 📤️ Publishes a verified file under its sha256: to the group store by temporary file and rename (a blob is never visible half written) and, when small enough, to the cache's content-addressed store. A failing tier is skipped; the return says which tiers took the blob. */
export async function fleetBlobPublishFileV1(context:FleetFetchContextV1,sha256:string,path:string,bytes:number,adopt:boolean=false):Promise<readonly ("store"|"http")[]>{
 const placed:("store"|"http")[]=[];if(!DIGEST.test(sha256))return placed;
 const started=context.now();
 if(context.storeRoot!==undefined){
  const target=fleetBlobStorePathV1(context.storeRoot,sha256),done=await guarded(policy.storeTimeoutMilliseconds+Math.ceil(bytes/(20*1048576))*1000,async()=>{
   if((await stat(target).catch(()=>undefined))?.isFile()){if(adopt)await rm(path,{force:true});return true;}
   await mkdir(dirname(target),{recursive:true});
   if(adopt){await rename(path,target);return true;}
   const temporary=`${target}.${randomBytes(6).toString("hex")}.tmp`;
   try{await copyFile(path,temporary,constants.COPYFILE_FICLONE);await rename(temporary,target);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
   return true;
  });
  if(done){placed.push("store");await recordFleetFetchV1(context,{sha256,source:"store",bytes,milliseconds:context.now()-started,direction:"put"});}
 }
 if(context.remote!==undefined&&context.remote.authenticated&&sha256.length===64&&bytes<=policy.httpMaximumBytes){
  const remote=context.remote,token=context.remote.token,done=await guarded(policy.httpTimeoutMilliseconds*4,async signal=>{
   const source=adopt&&context.storeRoot!==undefined?fleetBlobStorePathV1(context.storeRoot,sha256):path,body=Readable.toWeb(createReadStream(source)) as unknown as ReadableStream<Uint8Array>;
   const response=await context.fetch(`${remote.endpoint}/${remote.instance}/cas/${sha256}`,{method:"PUT",signal,headers:{...(token===undefined?{}:{authorization:`Bearer ${token}`}),"content-length":String(bytes)},body,duplex:"half"} as RequestInit);
   await response.body?.cancel().catch(()=>{});return response.status===200;
  });
  if(done){placed.push("http");await recordFleetFetchV1(context,{sha256,source:"http",bytes,milliseconds:context.now()-started,direction:"put"});}
 }
 return placed;
}

/** 📮️ Publishes bytes held in memory under their sha256. */
export async function fleetBlobPublishBytesV1(context:FleetFetchContextV1,bytes:Uint8Array):Promise<string>{
 const sha256=sha256Hex(bytes);if(context.storeRoot===undefined&&context.remote===undefined)return sha256;
 const staging=join(context.storeRoot??context.root??".",`.fleet-fetch-${randomBytes(6).toString("hex")}.tmp`);
 try{await writeFile(staging,bytes,{flag:"wx",mode:0o600});await fleetBlobPublishFileV1(context,sha256,staging,bytes.byteLength);}catch{}finally{await rm(staging,{force:true}).catch(()=>{});}
 return sha256;
}

/** 🔖️ Turns an npm integrity string (`sha512-<base64>`, also `sha256-`) into the pin of a download; `undefined` for any other algorithm. */
export function pinFromIntegrityV1(url:string,integrity:string,bytes?:number):PinnedDownloadV1|undefined{
 const match=/^(sha256|sha512)-([A-Za-z0-9+/]+={0,2})$/u.exec(integrity);if(!match)return undefined;
 const hex=Buffer.from(match[2]!,"base64").toString("hex");
 return hex.length===(match[1]==="sha256"?64:128)?{url,sha256:hex,...(bytes===undefined?{}:{bytes})}:undefined;
}

/** 🌐️ What a pinned download is: the upstream URL, the sha256 that is its whole trust and its size when pinned. */
export type PinnedDownloadV1=Readonly<{url:string;sha256:string;bytes?:number}>;

/** 📣️ Makes the progress sink of one blob claim: the first wait and then every `everyMilliseconds` it says in both languages who is fetching which blob and how long this machine has waited of the bound, so a download that waits for the fleet is never silent. @see ./🧬️schema/🔣️.json */
export function claimWaitNoticeV1(context:FleetFetchContextV1,pin:PinnedDownloadV1):(event:WorkClaimEventV1)=>void{
 const texts=schema.definitions.ClaimTexts.const,started=context.now(),say=context.say??((line:string)=>{process.stderr.write(line+"\n");});let last=-Infinity;
 return event=>{
  if(event.stage!=="wait"||context.now()-last<texts.everyMilliseconds)return;
  last=context.now();
  const values={holder:event.holder??"?",blob:`${pin.sha256.slice(0,12)} (${(()=>{try{return new URL(pin.url).host;}catch{return "?";}})()})`,seconds:Math.round((last-started)/1000),bound:Math.round(WORK_CLAIM_POLICY_V1.waitMaximumMilliseconds/1000)};
  for(const language of ["en","de"] as const)try{say(texts.wait[language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name as keyof typeof values]??"")));}catch{}
 };
}

/** ⬇️ A drop-in `fetch` for one pinned artifact. The fleet is asked first, by the sha256, and answers from the group store or the cache without touching the internet; on a miss the upstream is fetched, and while the body streams to the caller it is hashed and staged, and a body whose hash and size match the pin is published to the fleet when the stream ends. The caller keeps verifying the pin itself. On a fleet miss the download is claimed first (`claimFleetWorkV1`, kind `blob`, identity the pin), so of any number of machines asking at once exactly one downloads while the others wait and are served the published blob; a claim that cannot be had means the download runs here as before. A request with a method other than GET, a `Range` header or a non-success status passes through unchanged. */
export async function pinnedFetchV1(context:FleetFetchContextV1,pin:PinnedDownloadV1,init:RequestInit={}):Promise<Response>{
 const method=(init.method??"GET").toUpperCase(),headers=new Headers(init.headers);
 if(method!=="GET"||headers.has("range")||!DIGEST.test(pin.sha256))return context.fetch(pin.url,init);
 const found:{blob?:FleetBlobV1}={blob:await fleetBlobOpenV1(context,pin.sha256,pin.bytes)};
 const claim=found.blob?undefined:await claimFleetWorkV1(context,{kind:"blob",identity:pin.sha256,ready:async()=>(found.blob??=await fleetBlobOpenV1(context,pin.sha256,pin.bytes))!==undefined,event:claimWaitNoticeV1(context,pin)});
 const hit=found.blob;
 if(hit){
  const stream=Readable.toWeb(createReadStream(hit.path)) as unknown as ReadableStream<Uint8Array>,closing=new TransformStream<Uint8Array,Uint8Array>({flush:async()=>{await hit.dispose();}});
  return new Response(stream.pipeThrough(closing),{status:200,headers:{"content-length":String(hit.bytes),"content-type":"application/octet-stream","x-semio-fleet-source":hit.source}});
 }
 const started=context.now();
 let response:Response;
 try{response=await context.fetch(pin.url,init);}catch(error){await claim?.release();throw error;}
 if(!response.ok||!response.body||(context.storeRoot===undefined&&context.remote===undefined)){await claim?.release();return response;}
 const staging=context.storeRoot!==undefined?`${fleetBlobStorePathV1(context.storeRoot,pin.sha256)}.${randomBytes(6).toString("hex")}.tmp`:machinePathV1(context.root??".",".cache","f","fetch-tmp",`${randomBytes(6).toString("hex")}.tmp`),hash=createHash(algorithmOf(pin.sha256));
 let file:Awaited<ReturnType<typeof open>>|undefined,bytes=0,broken=false;
 try{await mkdir(dirname(staging),{recursive:true});file=await open(staging,"wx",0o600);}catch{broken=true;}
 const transform=new TransformStream<Uint8Array,Uint8Array>({
  transform:async(chunk,controller)=>{
   hash.update(chunk);bytes+=chunk.byteLength;
   if(file&&!broken){try{await file.write(chunk);}catch{broken=true;}}
   controller.enqueue(chunk);
  },
  flush:async()=>{
   try{await file?.close();}catch{broken=true;}
   try{
    await recordFleetFetchV1(context,{sha256:pin.sha256,source:"upstream",bytes,milliseconds:context.now()-started,direction:"get",url:pin.url.slice(0,512)});
    if(!broken&&hash.digest("hex")===pin.sha256&&(pin.bytes===undefined||bytes===pin.bytes))await fleetBlobPublishFileV1(context,pin.sha256,staging,bytes,context.storeRoot!==undefined);
   }finally{await rm(staging,{force:true}).catch(()=>{});await claim?.release();}
  }
 });
 return new Response(response.body.pipeThrough(transform),{status:response.status,statusText:response.statusText,headers:response.headers});
}
