import {createHash,timingSafeEqual} from "node:crypto";
import {createFleetGateV1,type FleetAccessV1,type FleetGateV1} from "../🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";
import {readFleetOriginPrivateV1,createFleetRegistryV1,createFleetVerifierV1,fleetContentHashV1,FLEET_CONTENT_HEADER_V1} from "../🛰️fleet/🪪️identity/🟦️.ts";
import {openProvenanceV1,provenanceFingerprintV1,type DivergenceV1,type ProvenanceV1} from "../🧾️provenance/🟦️.ts";
import {createMirrorHandlerV1} from "../🪞️mirror/🟦️.ts";
import {createSealedHandlerV1,exchangePrivateKeyV1,isSealedRequestV1} from "../🔐️sealed/🟦️.ts";
import {FLEET_HOP_HEADER_V1,type FleetPeerCountersV1,type FleetPeerReadThroughV1} from "../🧭️peers/🟦️.ts";
import {runtimeFileV1,runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";
import {mkdir,stat} from "node:fs/promises";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1,validateRemoteCacheActionResultV1,type RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";
import {acquireRemoteCacheLockV1,REMOTE_CACHE_COLLECTOR_LOCK_V1,REMOTE_CACHE_SERVER_LOCK_V1} from "../🧹️collector/🟦️.ts";
import {RemoteCacheStoreV1,type RemoteCacheKindV1,type RemoteCachePutModeV1,type RemoteCacheStoreEventV1} from "../💾️store/🟦️.ts";
import {isSlotIdV1,requireSlotShapeV1,SLOT_HEADER_V1} from "../../🪢️slots/🟦️.ts";
import {readRemoteCacheEntryV1} from "../📂️entry/🟦️.ts";
import {answerClaimRequestV1,CLAIM_POLICY_V1,createClaimTableV1,openClaimJournalV1,parseClaimPathV1,type ClaimSummaryV1,type ClaimTableV1} from "../🎟️claims/🟦️.ts";

/** 🔐️ Operator tokens: honoured from the machine itself only once the server runs with fleet identities, so they never cross the LAN. With neither token and no fleet the server is open (loopback only); a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined}>;

/** 🛰️ What the fleet identities of a server need: the group store whose `origins/` it trusts, the group, where it remembers pinned origin keys and which origin may sign as admin. The fleet may be several groups: `groups` and `peerStores` admit the origins of peer groups (they may read, never write; see the gate), `pinned` fixes origin keys from the topology. `networks` is the optional additional CIDR policy of the topology; without it the verified identity alone decides. */
export type RemoteCacheFleetOptionsV1=Readonly<{storeRoot:string;group:string;networks?:readonly string[]|undefined;groups?:readonly string[];peerStores?:Readonly<Record<string,string>>;pinned?:Readonly<Record<string,string>>;pinsPath?:string;adminOrigin?:string;registryTtlMilliseconds?:number;root?:string;inGroup?:"sealed"|"trusted-lan"}>;

/** 🧾️ The `fleet` part of `GET /status`: how many origins are usable, retired or in conflict and why requests were refused (counts by reason, never the identity of an attacker's key). */
export type RemoteCacheFleetStatusV1=Readonly<{origins:Readonly<Record<string,number>>;denied:Readonly<Record<string,number>>;transport?:Readonly<{crossGroup:"sealed";inGroup:"sealed"|"trusted-lan";sealedRoute:boolean}>}>;

/** 🧮️ Counts what one server did per keyspace since it started; `stale` counts action results that were dropped on a read because their blobs had left the store. */
export type RemoteCacheCountersV1=Record<RemoteCacheKindV1,{hits:number;misses:number;puts:number;conflicts:number;rejected:number;stale:number}>;

/** 📊️ The JSON document `GET /status` answers. */
export type RemoteCacheServerStatusV1=Readonly<{version:1;protocols:readonly ["bazel-http","nx-self-hosted"];entries:number;bytes:number;evicted?:Readonly<{entries:number;bytes:number}>;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;readOnly:boolean;validation:RemoteCacheValidationV1;counters:RemoteCacheCountersV1;startedAt:string;fleet?:RemoteCacheFleetStatusV1|null;divergences?:Readonly<{count:number;recent:readonly DivergenceV1[]}>;claims?:ClaimSummaryV1;peers?:FleetPeerCountersV1}>;

/** 📜️ One answered request, published after the response head is decided. */
export type RemoteCacheRequestEventV1=Readonly<{stage:"request";method:string;path:string;status:number;milliseconds:number}>;

/** 🔌️ Everything the web-standard handler needs. */
export type RemoteCacheHandlerOptionsV1=Readonly<{store:RemoteCacheStoreV1;tokens:RemoteCacheTokensV1;defaultInstance:string;readOnly?:boolean;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;startedAt?:Date;claims?:ClaimTableV1;mirror?:(request:Request)=>Promise<Response|undefined>;peers?:FleetPeerReadThroughV1;gate?:FleetGateV1;provenance?:ProvenanceV1;fleetStatus?:()=>RemoteCacheFleetStatusV1;onRequest?:(event:RemoteCacheRequestEventV1)=>void}>;

const RESERVED=new Set(["ac","cas","v1","healthz","status","claim","mirror"]),INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,DIGEST=/^[0-9a-f]{64}$/u,KEY=/^[a-z0-9_-]{1,128}$/u,TREE_BYTES=16*1048576,
 text=(status:number,message:string,headers:Record<string,string>={})=>new Response(message+"\n",{status,headers:{"content-type":"text/plain",...headers}}),
 digest=(value:string)=>createHash("sha256").update(value).digest(),
 row=()=>({hits:0,misses:0,puts:0,conflicts:0,rejected:0,stale:0});

/** 🚰️ A request body read through its reader, because the runtime's stream iterator is not available for request bodies: `chunks` feeds the store and `drain` discards what is left, up to a bound, so a reply sent before the body was read is not lost to a connection reset. */
export type RemoteCacheBodyV1=Readonly<{chunks:AsyncGenerator<Uint8Array>;drain:(limit:number)=>Promise<void>}>;

/** 📡️ Wraps a request body stream. */
export function remoteCacheBodyV1(stream:ReadableStream<Uint8Array>|null):RemoteCacheBodyV1{
 if(!stream)return Object.freeze({chunks:(async function*(){})(),drain:async()=>{}});
 const reader=stream.getReader();let finished=false;
 async function* chunks():AsyncGenerator<Uint8Array>{for(;;){const {done,value}=await reader.read();if(done){finished=true;return;}yield value;}}
 const drain=async(limit:number):Promise<void>=>{
  let seen=0;
  try{while(!finished){const {done,value}=await reader.read();if(done){finished=true;return;}seen+=value.byteLength;if(seen>limit){await reader.cancel();return;}}}catch{}
 };
 return Object.freeze({chunks:chunks(),drain});
}

/** 🪪️ Compares two secrets in constant time. */
export function remoteCacheTokenMatchesV1(expected:string,presented:string):boolean{return timingSafeEqual(digest(expected),digest(presented));}

/** 🧭️ Resolves a request path to its namespace, keyspace and key, or `undefined` for every path outside the two protocols. */
export function parseRemoteCachePathV1(pathname:string,defaultInstance:string):Readonly<{namespace:string;kind:RemoteCacheKindV1;key:string}>|undefined{
 const segments=pathname.split("/").slice(1);let namespace=defaultInstance;
 if(segments.length>0&&!RESERVED.has(segments[0]!)){namespace=segments.shift()!;if(!INSTANCE.test(namespace))return undefined;}
 if((segments[0]==="ac"||segments[0]==="cas")&&segments.length===2)return DIGEST.test(segments[1]!)?{namespace,kind:segments[0],key:segments[1]!}:undefined;
 if(segments[0]==="v1"&&segments[1]==="cache"&&segments.length===3)return KEY.test(segments[2]!)?{namespace,kind:"nx",key:segments[2]!}:undefined;
 return undefined;
}

/** 🪢️ Resolves `/<instance>/v1/slots` (or `/v1/slots`) to its namespace, or `undefined` for every other path. */
export function parseRemoteCacheSlotsPathV1(pathname:string,defaultInstance:string):string|undefined{
 const segments=pathname.split("/").slice(1);let namespace=defaultInstance;
 if(segments.length===3&&!RESERVED.has(segments[0]!)){namespace=segments.shift()!;if(!INSTANCE.test(namespace))return undefined;}
 return segments.length===2&&segments[0]==="v1"&&segments[1]==="slots"?namespace:undefined;
}

/** 🏗️ Builds the protocol handler as a pure web-standard function, so any HTTP host can serve it: Bazel's HTTP cache (`/ac/<sha256>`, `/cas/<sha256>`) and the Nx self-hosted cache API (`/v1/cache/<hash>`), each optionally under a `/<instance>` prefix. Blobs are verified against their digest; action results are validated (and, when complete, answered only while every blob they name is still stored); a read-only handler refuses every write. */
export function createRemoteCacheHandlerV1(options:RemoteCacheHandlerOptionsV1):(request:Request,peer?:string)=>Promise<Response>{
 const claims=options.claims??createClaimTableV1(),counters:RemoteCacheCountersV1={ac:row(),cas:row(),nx:row()},validation=options.validation??"complete",readOnly=options.readOnly===true,limit=options.maximumActionResultBytes??4194304,
  startedAt=(options.startedAt??new Date()).toISOString(),gate=options.gate??createFleetGateV1({tokens:{read:options.tokens.read,write:options.tokens.write}});
 const bound=(access:FleetAccessV1|undefined,bytes:Uint8Array):boolean=>access===undefined||access.role==="static"||(access.content!=="-"&&fleetContentHashV1(bytes)===access.content);
 const authenticate=async(request:Request,url:URL,peer:string|undefined,write:boolean):Promise<Readonly<{denied:Response}>|Readonly<{access:FleetAccessV1|undefined}>>=>{
  const verdict=await gate.check({method:request.method,target:url.pathname+url.search,authorization:request.headers.get("authorization"),content:request.headers.get(FLEET_CONTENT_HEADER_V1),peer,sealed:isSealedRequestV1(request),roles:write?["cache-write"]:["cache-read","cache-write","admin"],tokens:write?["write"]:["read","write"],...(write?{refused:["read"]}:{})});
  return verdict.ok?{access:verdict.access}:{denied:gate.respond(verdict)};
 };
 const mode=(kind:RemoteCacheKindV1):RemoteCachePutModeV1=>kind==="ac"?"replace":kind==="cas"?"content":"keep";
 const blobs=(namespace:string)=>({has:(hash:string)=>options.store.has(namespace,"cas",hash),read:(hash:string)=>options.store.readBytes(namespace,"cas",hash,TREE_BYTES)});
 const throughPeers=async(request:Request,address:Readonly<{namespace:string;kind:RemoteCacheKindV1;key:string}>):Promise<Response|undefined>=>{
  if(options.peers===undefined||request.method!=="GET"||request.headers.get(FLEET_HOP_HEADER_V1)!==null)return undefined;
  const found=await options.peers.fetch(new URL(request.url).pathname).catch(()=>undefined);
  if(found===undefined)return undefined;
  const bytes=found.bytes,once=():AsyncIterable<Uint8Array>=>(async function*(){yield bytes;})();
  if(address.kind==="cas"){
   if(createHash("sha256").update(bytes).digest("hex")!==address.key)return undefined;
   await options.store.put(address.namespace,"cas",address.key,once(),{mode:"content",verify:true,declaredLength:bytes.byteLength}).catch(()=>undefined);
  }else if(address.kind==="nx")await options.store.put(address.namespace,"nx",address.key,once(),{mode:"keep",declaredLength:bytes.byteLength}).catch(()=>undefined);
  else if(validation!=="off"){try{parseRemoteCacheActionResultV1(bytes);}catch{return undefined;}}
  counters[address.kind].hits++;
  return new Response(bytes as BodyInit,{headers:{"content-type":"application/octet-stream","content-length":String(bytes.byteLength)}});
 };
 const readResult=async(namespace:string,key:string,method:string,request:Request):Promise<Response>=>{
  const counter=counters.ac;
  if(validation!=="complete"){
   const entry=await options.store.read(namespace,"ac",key);
   if(!entry){const near=await throughPeers(request,{namespace,kind:"ac",key});if(near)return near;counter.misses++;return text(404,"Not found");}
   const whole=method==="GET"?await readRemoteCacheEntryV1(entry.path,entry.size):null;
   if(whole===undefined){counter.misses++;return text(404,"Not found");}
   counter.hits++;return new Response(whole==="lazy"?runtimeFileV1(entry.path).body():whole,{headers:{"content-type":"application/octet-stream","content-length":String(entry.size)}});
  }
  const bytes=await options.store.readBytes(namespace,"ac",key,limit);
  if(bytes===undefined){const near=await throughPeers(request,{namespace,kind:"ac",key});if(near)return near;counter.misses++;return text(404,"Not found");}
  try{
   const parsed=parseRemoteCacheActionResultV1(bytes);
   for(const value of[...parsed.references,...parsed.trees])if(!await options.store.has(namespace,"cas",value.hash))throw Error("blob left the store");
  }catch{await options.store.remove(namespace,"ac",key);counter.stale++;counter.misses++;return text(404,"Not found");}
  counter.hits++;return new Response((method==="GET"?bytes:null) as BodyInit|null,{headers:{"content-type":"application/octet-stream","content-length":String(bytes.byteLength)}});
 };
 const answer=async(request:Request,body:RemoteCacheBodyV1,peer:string|undefined):Promise<Response>=>{
  const url=new URL(request.url),method=request.method;
  if(options.mirror!==undefined&&url.pathname.startsWith("/mirror/")){const mirrored=await options.mirror(request);if(mirrored)return mirrored;}
  if(url.pathname==="/healthz")return method==="GET"||method==="HEAD"?new Response(method==="GET"?'{"status":"ok"}\n':null,{headers:{"content-type":"application/json"}}):text(405,"Method not allowed",{allow:"GET, HEAD"});
  if(url.pathname==="/status"){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   const usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,evicted:{entries:usage.evictedEntries,bytes:usage.evictedBytes},reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters,startedAt,fleet:options.fleetStatus?.()??null,divergences:options.provenance?.summary()??{count:0,recent:[]},claims:claims.summary(),...(options.peers===undefined?{}:{peers:options.peers.counters()})};
   return Response.json(status);
  }
  const claimAddress=parseClaimPathV1(url.pathname,options.defaultInstance);
  let claimAccess:FleetAccessV1|undefined;
  if(claimAddress!==undefined){const guard=await authenticate(request,url,peer,claimAddress.action!=="await");if("denied" in guard)return guard.denied;claimAccess=guard.access;}
  const claimed=await answerClaimRequestV1({request,url,table:claims,defaultInstance:options.defaultInstance,authorize:()=>undefined,...(claimAccess===undefined||claimAccess.role==="static"?{}:{scope:claimAccess.origin}),readOnly,has:(namespace,kind,key)=>options.store.has(namespace,kind,key),readBody:async limit=>{const parts:Uint8Array[]=[];let size=0;try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit)return undefined;parts.push(chunk);}}catch{return undefined;}const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}return bound(claimAccess,bytes)?bytes:undefined;}});
  if(claimed)return claimed;
  const slots=parseRemoteCacheSlotsPathV1(url.pathname,options.defaultInstance);
  if(slots!==undefined){
   if(method!=="POST")return text(405,"Method not allowed",{allow:"POST"});
   const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
   if(readOnly)return text(403,"Forbidden: this cache is read-only");
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>1048576)return text(413,"Claim exceeds the maximum size");parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(!bound(guard.access,Buffer.concat(parts)))return text(400,"Content hash required and must match");
   let claim:{entries:{kind:"ac"|"nx";key:string;slot:string}[]};
   try{claim=JSON.parse(new TextDecoder().decode(Buffer.concat(parts)));requireSlotShapeV1("Claim",claim,"claim");}catch(error){return text(400,`Invalid claim: ${(error as Error).message}`);}
   let claimed=0,missing=0;
   for(const entry of claim.entries){
    let outcome;try{outcome=await options.store.claim(slots,entry.kind,entry.key,entry.slot);}catch{outcome="missing" as const;}
    if(outcome==="claimed")claimed++;else missing++;
   }
   return Response.json({claimed,missing});
  }
  const address=parseRemoteCachePathV1(url.pathname,options.defaultInstance);
  if(!address)return text(404,"Not found");
  if(method==="GET"||method==="HEAD"){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   if(address.kind==="ac")return readResult(address.namespace,address.key,method,request);
   const entry=await options.store.read(address.namespace,address.kind,address.key),counter=counters[address.kind];
   if(!entry){const near=await throughPeers(request,address);if(near)return near;counter.misses++;return text(404,"Not found");}
   const whole=method==="GET"?await readRemoteCacheEntryV1(entry.path,entry.size):null;
   if(whole===undefined){counter.misses++;return text(404,"Not found");}
   counter.hits++;const headers={"content-type":"application/octet-stream","content-length":String(entry.size)};
   return new Response(whole==="lazy"?runtimeFileV1(entry.path).body():whole,{headers});
  }
  if(method!=="PUT")return text(405,"Method not allowed",{allow:"GET, HEAD, PUT"});
  const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
  const access=guard.access,signed=access!==undefined&&access.role!=="static";
  if(signed&&access!.content==="-")return text(400,"A signed write must carry the hash of its body");
  if(signed&&address.kind==="cas"&&access!.content!==address.key)return text(400,"The signed content hash of a blob is its digest");
  if(readOnly)return text(403,"Forbidden: this cache is read-only");
  const counter=counters[address.kind],length=request.headers.get("content-length"),slotHeader=request.headers.get(SLOT_HEADER_V1);
  if(slotHeader!==null&&!isSlotIdV1(slotHeader))return text(400,"Invalid slot id");
  if(address.kind==="nx"&&length===null)return text(411,"Content-Length required");
  const declared=length===null?undefined:Number(length);
  if(declared!==undefined&&!Number.isSafeInteger(declared))return text(400,"Content-Length invalid");
  let source:AsyncIterable<Uint8Array>=body.chunks,known=declared,fingerprint:string|undefined,prior:ReturnType<ProvenanceV1["of"]>;
  if(address.kind==="ac"&&validation!=="off"){
   if(declared!==undefined&&declared>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(declared!==undefined&&size!==declared){counter.rejected++;return text(400,"Content-Length mismatch");}
   const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}
   if(!bound(access,bytes)){counter.rejected++;return text(400,"Content hash required and must match");}
   const verdict=await validateRemoteCacheActionResultV1(bytes,validation,blobs(address.namespace));
   if(!verdict.valid){counter.rejected++;return text(400,`Invalid action result: ${verdict.reason}`);}
   if(signed&&options.provenance!==undefined){
    const parsed=parseRemoteCacheActionResultV1(bytes);fingerprint=provenanceFingerprintV1([...parsed.references,...parsed.trees].map(row=>row.hash));
    const held=await options.store.readBytes(address.namespace,"ac",address.key,limit),before=options.provenance.of(address.namespace,"ac",address.key);
    if(held!==undefined&&before?.origin!==access!.origin){
     let kept=before?.fingerprint;
     if(kept===undefined)try{const old=parseRemoteCacheActionResultV1(held);kept=provenanceFingerprintV1([...old.references,...old.trees].map(row=>row.hash));}catch{kept=undefined;}
     if(kept!==undefined&&kept!==fingerprint){
      options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint});
      counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");
     }
    }
    prior=before;
   }
   source=(async function*(){yield bytes;})();known=size;
  }
  let nxHash:ReturnType<typeof createHash>|undefined;
  if(address.kind==="nx"&&signed&&access!.content!=="-"){const hash=nxHash=createHash("sha256"),chunks=body.chunks;source=(async function*(){for await(const chunk of chunks){hash.update(chunk);yield chunk;}})();}
  const outcome=await options.store.put(address.namespace,address.kind,address.key,source,{mode:mode(address.kind),verify:address.kind==="cas",declaredLength:known,...(slotHeader!==null&&address.kind!=="cas"?{slot:{slot:slotHeader}}:{})});
  if(nxHash!==undefined&&(outcome==="created"||outcome==="replaced")&&nxHash.digest("hex")!==access!.content){await options.store.remove(address.namespace,"nx",address.key);counter.rejected++;return text(400,"Content hash mismatch");}
  if(signed&&options.provenance!==undefined&&(outcome==="created"||outcome==="replaced")&&(prior===undefined||prior.origin===access!.origin||outcome==="created"))options.provenance.record(address.namespace,address.kind,address.key,access!.origin,fingerprint);
  switch(outcome){
   case "created":case "replaced":counter.puts++;if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");
   case "exists":if(address.kind==="nx"){counter.conflicts++;return text(409,"Conflict: cannot overwrite an existing cache entry");}if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");
   case "digest-mismatch":counter.rejected++;return text(400,"Digest mismatch");
   case "length-mismatch":counter.rejected++;return text(400,"Content-Length mismatch");
   case "too-large":counter.rejected++;return text(413,"Entry exceeds the maximum entry size");
   case "capacity":counter.rejected++;return text(507,"Insufficient storage",{"retry-after":"5"});
   case "aborted":return text(499,"Client closed request");
  }
 };
 return async(request,peer)=>{
  if(!gate.network(peer))return text(403,"Forbidden");
  const started=performance.now(),body=remoteCacheBodyV1(request.body),response=await answer(request,body,peer);
  await body.drain(options.store.usage().maximumEntryBytes);
  options.onRequest?.({stage:"request",method:request.method,path:new URL(request.url).pathname,status:response.status,milliseconds:Math.round(performance.now()-started)});
  return response;
 };
}

/** 🔒️ The certificate chain and private key of an optional TLS listener, as file paths. */
export type RemoteCacheTlsV1=Readonly<{certificate:string;key:string}>;

/** ⚙️ Authors one running server: where to listen, what to store, who may use it and what it checks. */
export type RemoteCacheServerOptionsV1=Readonly<{fleet?:RemoteCacheFleetOptionsV1;peers?:FleetPeerReadThroughV1;allowWildcard?:boolean;mirror?:Readonly<{directory:string;maximumBytes?:number}>;hostStore?:import("../../🗄️cas/🗃️store/🟦️.ts").CasStoreV1;directory:string;host:string;port:number;maximumBytes:number;maximumEntryBytes:number;maximumAgeDays:number;defaultInstance:string;tokens:RemoteCacheTokensV1;readOnly?:boolean;tls?:RemoteCacheTlsV1|undefined;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;maintenanceSeconds?:number;graceMilliseconds?:number;claimed?:(namespace:string,kind:RemoteCacheKindV1,key:string)=>boolean;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1|RemoteCacheRequestEventV1|Readonly<{stage:"listening";url:string}>|Readonly<{stage:"stopped"}>)=>void}>;

/** 🛰️ A started server: its address, a live status and an orderly stop. */
export interface RemoteCacheServerV1{readonly url:string;readonly port:number;readonly store:RemoteCacheStoreV1;readonly claims:ClaimTableV1;readonly stopped:Promise<void>;stop():Promise<void>}

/** 🚧️ Raised when another living server already owns the store directory. */
export class RemoteCacheDirectoryBusyV1 extends Error{constructor(readonly directory:string){super(`Another remote cache server uses ${directory}`);}}

const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]),WILDCARD=new Set(["0.0.0.0","::","[::]"]);

/** 🚀️ Opens the store and serves the handler through the runtime interface, over TLS when a certificate is given; refuses to listen beyond loopback without a token, refuses a directory another server owns, keeps the store in step with outside collectors, and stops when `signal` aborts. */
export async function startRemoteCacheServerV1(options:RemoteCacheServerOptionsV1):Promise<RemoteCacheServerV1>{
 const registry=options.fleet===undefined?undefined:createFleetRegistryV1({storeRoot:options.fleet.storeRoot,group:options.fleet.group,...(options.fleet.groups===undefined?{}:{groups:options.fleet.groups}),...(options.fleet.peerStores===undefined?{}:{peerStores:options.fleet.peerStores}),...(options.fleet.pinned===undefined?{}:{pinned:options.fleet.pinned}),...(options.fleet.registryTtlMilliseconds===undefined?{}:{ttlMilliseconds:options.fleet.registryTtlMilliseconds}),pinsPath:options.fleet.pinsPath??join(options.directory,"fleet","pins.json")});
 const verifier=registry===undefined||options.fleet===undefined?undefined:createFleetVerifierV1({registry,audience:"cache",adminOrigin:options.fleet.adminOrigin??`${options.fleet.group}-server`});
 const gate=createFleetGateV1({verifier,networks:options.fleet?.networks,home:options.fleet?.group,...(options.fleet?.inGroup===undefined?{}:{inGroup:options.fleet.inGroup}),tokens:{read:options.tokens.read,write:options.tokens.write}});
 if(!LOOPBACK.has(options.host)&&gate.open)throw Error("A token is required to listen beyond loopback (or fleet identities)");
 if(WILDCARD.has(options.host)&&options.fleet!==undefined&&options.allowWildcard!==true)throw Error("Listen on the fleet interface, not on a wildcard address");
 if(!INSTANCE.test(options.defaultInstance)||RESERVED.has(options.defaultInstance))throw Error("Valid default instance required");
 if(options.tls){for(const path of[options.tls.certificate,options.tls.key])if(!(await stat(path).catch(()=>undefined))?.isFile())throw Error(`TLS file required: ${path}`);}
 const root=join(options.directory,"v1");await mkdir(root,{recursive:true});
 const owner=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1);
 if(owner.held)throw new RemoteCacheDirectoryBusyV1(options.directory);
 let store:RemoteCacheStoreV1|undefined,timer:ReturnType<typeof setInterval>|undefined;
 const table=createClaimTableV1({journal:openClaimJournalV1(join(root,CLAIM_POLICY_V1.journalFile))});
 const provenance=await openProvenanceV1(options.directory);
 await registry?.refresh(true);
 const originPrivate=options.fleet?.root===undefined?undefined:readFleetOriginPrivateV1(options.fleet.root),exchangeKey=originPrivate===undefined?undefined:exchangePrivateKeyV1(originPrivate);
 const fleetStatus=():RemoteCacheFleetStatusV1=>{const origins:Record<string,number>={};for(const row of Object.values(registry?.origins()??{}))origins[row.status]=(origins[row.status]??0)+1;return {origins,denied:verifier?.denied()??{},transport:{crossGroup:"sealed",inGroup:options.fleet?.inGroup??"trusted-lan",sealedRoute:exchangeKey!==undefined}};};
 try{
  store=await RemoteCacheStoreV1.open({directory:options.directory,...(options.hostStore?{host:options.hostStore}:{}),maximumBytes:options.maximumBytes,maximumEntryBytes:options.maximumEntryBytes,maximumAgeMilliseconds:options.maximumAgeDays*86400000,...(options.graceMilliseconds===undefined?{}:{graceMilliseconds:options.graceMilliseconds}),claimed:(namespace,kind,key)=>kind!=="cas"&&(table.claimed(namespace,kind,key)||options.claimed?.(namespace,kind,key)===true),...(options.signal?{signal:options.signal}:{}),...(options.onEvent?{onEvent:options.onEvent}:{})});
  const opened=store,handler=createRemoteCacheHandlerV1({store:opened,claims:table,gate,provenance,...(options.peers===undefined?{}:{peers:options.peers}),...(registry===undefined?{}:{fleetStatus}),...(options.mirror?{mirror:createMirrorHandlerV1(options.mirror)}:{}),tokens:options.tokens,defaultInstance:options.defaultInstance,...(options.readOnly===undefined?{}:{readOnly:options.readOnly}),...(options.validation===undefined?{}:{validation:options.validation}),...(options.maximumActionResultBytes===undefined?{}:{maximumActionResultBytes:options.maximumActionResultBytes}),...(options.onEvent?{onRequest:options.onEvent}:{})});
  const sealed=exchangeKey===undefined||options.fleet===undefined?undefined:createSealedHandlerV1({privateKey:exchangeKey,group:options.fleet.group,maximumBytes:options.maximumEntryBytes,handle:handler}),entry=async(request:Request,peer?:string):Promise<Response>=>{if(sealed!==undefined&&!gate.network(peer))return new Response("Forbidden\n",{status:403});return (await sealed?.(request,peer))??handler(request,peer);};
  const server=await runtimeServeV1({hostname:options.host,port:options.port,idleTimeoutSeconds:60,maximumRequestBodyBytes:options.maximumEntryBytes*2,...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),fetch:(request,peer)=>entry(request,peer)});
  const host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host,port=server.port,url=`${options.tls?"https":"http"}://${host}:${port}`;
  let finish!:()=>void;const stopped=new Promise<void>(resolve=>{finish=resolve;});let closing:Promise<void>|undefined;
  const stop=():Promise<void>=>closing??=(async()=>{if(timer)clearInterval(timer);table.close();await server.stop(true);await opened.close();await provenance.flush();await owner.release();options.onEvent?.({stage:"stopped"});finish();})();
  const maintenance=(options.maintenanceSeconds??0)*1000;
  if(maintenance>0){
   let running=false;
   timer=setInterval(()=>{
    if(running)return;running=true;
    void(async()=>{const lock=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_COLLECTOR_LOCK_V1);if(lock.held)return;try{await opened.refresh();}finally{await lock.release();}})().catch(()=>{}).finally(()=>{running=false;});
   },maintenance);
   timer.unref();
  }
  if(options.signal){if(options.signal.aborted)await stop();else options.signal.addEventListener("abort",()=>{void stop();},{once:true});}
  options.onEvent?.({stage:"listening",url});
  return Object.freeze({url,port,store:opened,claims:table,stopped,stop});
 }catch(error){
  await store?.close().catch(()=>{});await owner.release();throw error;
 }
}
