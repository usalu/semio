import {remoteCacheFileBudgetOfMachineV1} from "../📏️fd-budget/🟦️.ts";
import {createHash,timingSafeEqual} from "node:crypto";
import {casEnforceTickV1,type CasGuardPortsV1} from "../../🗄️cas/🚦️guard/🟦️.ts";
import {readCasCensusV1,readCasUsageHintV1} from "../../🗄️cas/🧹️collector/🟦️.ts";
import {casTextV1} from "../../🗄️cas/🧬️schema/🟦️.ts";
import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {CAS_LEASE_HEADER_V1,parseCasLeaseHeaderV1} from "../../🗄️cas/📌️pins/🟦️.ts";
import {casEffectiveCapBytesV1,casReserveBytesV1,declareCasRoleV1,measureCasPartitionV1} from "../../🗄️cas/🌡️partition/🟦️.ts";
import {createFleetGateV1,type FleetAccessV1,type FleetGateV1} from "../🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";
import {readFleetOriginPrivateV1,createFleetRegistryV1,createFleetVerifierV1,fleetContentHashV1,FLEET_CONTENT_HEADER_V1} from "../🛰️fleet/🪪️identity/🟦️.ts";
import {openProvenanceV1,PROVENANCE_SUPERSEDING_CLASSES_V1,PROVENANCE_VOLATILE_OUTPUTS_V1,provenanceClassV1,provenanceDifferingV1,provenanceFingerprintV1,type ConsumptionV1,type DivergenceV1,type DivergenceWindowV1,type ProvenanceV1} from "../🧾️provenance/🟦️.ts";
import {createMirrorHandlerV1} from "../🪞️mirror/🟦️.ts";
import {openCounterLedgerV1,type CounterLedgerV1} from "../../📈️counter-ledger/🟦️.ts";
import {createSealedHandlerV1,exchangePrivateKeyV1,isSealedRequestV1} from "../🔐️sealed/🟦️.ts";
import {FLEET_HOP_HEADER_V1,type FleetPeerCountersV1,type FleetPeerReadThroughV1} from "../🧭️peers/🟦️.ts";
import {runtimeFileV1,runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";
import {mkdir,stat} from "node:fs/promises";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1,parseRemoteCacheStableResultV1,validateRemoteCacheActionResultV1,type RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";
import {acquireRemoteCacheLockV1,renewRemoteCacheLockV1,REMOTE_CACHE_COLLECTOR_LOCK_V1,REMOTE_CACHE_SERVER_LOCK_V1} from "../🧹️collector/🟦️.ts";
import {openTreeDigestsV1,type TreeDigestsV1} from "../🏷️retention/🍂️staleness/🟦️.ts";
import {RemoteCacheStoreV1,REMOTE_CACHE_KINDS_V1,type RemoteCacheKindV1,type RemoteCachePutModeV1,type RemoteCacheStoreEventV1} from "../💾️store/🟦️.ts";
import {isSlotIdV1,requireSlotShapeV1,SLOT_HEADER_V1,type DeclarationV1,type StalenessModeV1} from "../../🪢️slots/🟦️.ts";
import {readRemoteCacheEntryV1} from "../📂️entry/🟦️.ts";
import {answerClaimRequestV1,CLAIM_POLICY_V1,createClaimTableV1,openClaimJournalV1,parseClaimPathV1,type ClaimSummaryV1,type ClaimTableV1} from "../🎟️claims/🟦️.ts";

const NX_KIND=REMOTE_CACHE_KINDS_V1.nx;

/** 🔐️ Operator tokens: honoured from the machine itself only once the server runs with fleet identities, so they never cross the LAN. With neither token and no fleet the server is open (loopback only); a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined}>;

/** 🛰️ What the fleet identities of a server need: the group store whose `origins/` it trusts, the group, where it remembers pinned origin keys and which origin may sign as admin. The fleet may be several groups: `groups` and `peerStores` admit the origins of peer groups (they may read, never write; see the gate), `pinned` fixes origin keys from the topology. `networks` is the optional additional CIDR policy of the topology; without it the verified identity alone decides. */
export type RemoteCacheFleetOptionsV1=Readonly<{storeRoot:string;group:string;networks?:readonly string[]|undefined;groups?:readonly string[];peerStores?:Readonly<Record<string,string>>;pinned?:Readonly<Record<string,string>>;pinsPath?:string;adminOrigin?:string;registryTtlMilliseconds?:number;root?:string;inGroup?:"sealed"|"trusted-lan"}>;

/** 🧾️ The `fleet` part of `GET /status`: how many origins are usable, retired or in conflict and why requests were refused (counts by reason, never the identity of an attacker's key). */
export type RemoteCacheFleetStatusV1=Readonly<{origins:Readonly<Record<string,number>>;denied:Readonly<Record<string,number>>;transport?:Readonly<{crossGroup:"sealed";inGroup:"sealed"|"trusted-lan";sealedRoute:boolean}>}>;

/** 🧮️ Counts what one server did per keyspace; `stale` counts action results that were dropped on a read because their blobs had left the store. In `GET /status` the `counters` are cumulative over every run of the server (the persisted ledger plus this run, see `📈️counter-ledger`), `session` is this run alone. */
export type RemoteCacheCountersV1=Record<RemoteCacheKindV1,{hits:number;misses:number;puts:number;conflicts:number;rejected:number;stale:number}>;

/** 📊️ The JSON document `GET /status` answers. */
export type RemoteCacheServerStatusV1=Readonly<{version:1;protocols:readonly ["bazel-http","nx-self-hosted"];entries:number;bytes:number;evicted?:Readonly<{entries:number;bytes:number}>;blobs?:Readonly<{referencedBytes:number|null;boundBytes:number|null;evictedBytes:number}>;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;readOnly:boolean;validation:RemoteCacheValidationV1;counters:RemoteCacheCountersV1;session?:RemoteCacheCountersV1;ledger?:Readonly<{sinceMs:number;restarts:number}>;startedAt:string;fleet?:RemoteCacheFleetStatusV1|null;divergences?:Readonly<{count:number;recent:readonly DivergenceV1[];since?:number;byClass?:Readonly<Record<string,number>>;current?:DivergenceWindowV1;lastHour?:DivergenceWindowV1;advisory?:Readonly<Record<string,number>>;superseded?:Readonly<Record<string,number>>;evidence?:Readonly<{count:number;bytes:number;byClass:Readonly<Record<string,number>>}>}>;consumption?:ConsumptionV1;claims?:ClaimSummaryV1;peers?:FleetPeerCountersV1;retention?:ReturnType<RemoteCacheStoreV1["stalenessStatus"]>}>;

/** 📜️ One answered request, published after the response head is decided. */
export type RemoteCacheRequestEventV1=Readonly<{stage:"request";method:string;path:string;status:number;milliseconds:number}>;

/** 🔌️ Everything the web-standard handler needs. */
export type RemoteCacheHandlerOptionsV1=Readonly<{capacityDetail?:()=>string;store:RemoteCacheStoreV1;tokens:RemoteCacheTokensV1;defaultInstance:string;instances?:readonly string[];readOnly?:boolean;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;startedAt?:Date;claims?:ClaimTableV1;mirror?:(request:Request)=>Promise<Response|undefined>;peers?:FleetPeerReadThroughV1;gate?:FleetGateV1;provenance?:ProvenanceV1;fleetStatus?:()=>RemoteCacheFleetStatusV1;hostStore?:Pick<import("../../🗄️cas/🗃️store/🟦️.ts").CasStoreV1,"lease"|"releaseLease">;onRequest?:(event:RemoteCacheRequestEventV1)=>void;ledger?:CounterLedgerV1}>;

const RESERVED=new Set(["ac","cas","v1","healthz","status","evidence","claim","mirror"]),INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,DIGEST=/^[0-9a-f]{64}$/u,KEY=/^[a-z0-9_-]{1,128}$/u,TREE_BYTES=16*1048576,
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
 if(segments[0]==="v1"&&segments[1]==="cache"&&segments.length===3)return KEY.test(segments[2]!)?{namespace,kind:NX_KIND,key:segments[2]!}:undefined;
 return undefined;
}

/** 🔭️ Resolves `/<instance>/v1/slots/<slot>` (or `/v1/slots/<slot>`) to its namespace and slot id, or `undefined` for every other path. */
export function parseRemoteCacheSlotKeysPathV1(pathname:string,defaultInstance:string):Readonly<{namespace:string;slot:string}>|undefined{
 const segments=pathname.split("/").slice(1);let namespace=defaultInstance;
 if(segments.length===4&&!RESERVED.has(segments[0]!)){namespace=segments.shift()!;if(!INSTANCE.test(namespace))return undefined;}
 return segments.length===3&&segments[0]==="v1"&&segments[1]==="slots"&&/^[0-9a-f]{64}$/u.test(segments[2]!)?{namespace,slot:segments[2]!}:undefined;
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
 const publications=new Map<string,Promise<void>>();
 options.ledger?.bind(()=>({ac:counters.ac,cas:counters.cas,nx:counters.nx,...(options.peers===undefined?{}:{peers:options.peers.counters()})}));
 const cumulative=():Readonly<{counters:RemoteCacheCountersV1;peers?:FleetPeerCountersV1}>=>{
  if(options.ledger===undefined)return {counters,...(options.peers===undefined?{}:{peers:options.peers.counters()})};
  const total=options.ledger.total(),kind=(name:RemoteCacheKindV1)=>({...row(),...total[name]});
  return {counters:{ac:kind("ac"),cas:kind("cas"),nx:kind("nx")},...(options.peers===undefined?{}:{peers:{...options.peers.counters(),...total.peers}})};
 };
 const serializeResult=async<T>(namespace:string,key:string,operation:()=>Promise<T>):Promise<T>=>{
  const address=namespace+"/"+key,previous=publications.get(address);
  let release!:()=>void;
  const complete=new Promise<void>(done=>{release=done;});
  publications.set(address,complete);
  await previous;
  try{return await operation();}
  finally{release();if(publications.get(address)===complete)publications.delete(address);}
 };
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
  }else if(address.kind===NX_KIND)await options.store.put(address.namespace,NX_KIND,address.key,once(),{mode:"keep",declaredLength:bytes.byteLength}).catch(()=>undefined);
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
  }catch{
   await serializeResult(namespace,key,async()=>{
    const current=await options.store.readBytes(namespace,"ac",key,limit);
    if(current!==undefined&&Buffer.compare(current,bytes)===0)await options.store.remove(namespace,"ac",key);
   });
   counter.stale++;counter.misses++;return text(404,"Not found");
  }
  counter.hits++;return new Response((method==="GET"?bytes:null) as BodyInit|null,{headers:{"content-type":"application/octet-stream","content-length":String(bytes.byteLength)}});
 };
 const declarationOf=async(namespace:string,action:Uint8Array,names:readonly string[]):Promise<DeclarationV1|undefined>=>{
  try{
   const named=parseRemoteCacheStableResultV1(action,[]).named.find(row=>names.includes(row.path));if(named===undefined)return undefined;
   const bytes=await blobs(namespace).read(named.hash);if(bytes===undefined)return undefined;
   const value=(JSON.parse(new TextDecoder().decode(bytes)) as {retention?:unknown}).retention;if(value===undefined)return undefined;
   requireSlotShapeV1("Declaration",value,"declaration");return value as DeclarationV1;
  }catch{return undefined;}
 };
 const served=(namespace:string):boolean=>options.instances===undefined||options.instances.includes(namespace);
 const foreign=(namespace:string):Response=>text(421,`This cache stores only the instance ${options.instances!.join(", ")}, not ${namespace}: remove SEMIO_REMOTE_CACHE_INSTANCE from the profile of the machine. / Dieser Cache speichert nur die Instanz ${options.instances!.join(", ")}, nicht ${namespace}: SEMIO_REMOTE_CACHE_INSTANCE aus dem Profil der Maschine entfernen.`);
 const answer=async(request:Request,body:RemoteCacheBodyV1,peer:string|undefined):Promise<Response>=>{
  const url=new URL(request.url),method=request.method;
  if(options.mirror!==undefined&&url.pathname.startsWith("/mirror/")){const mirrored=await options.mirror(request);if(mirrored)return mirrored;}
  if(url.pathname==="/healthz")return method==="GET"||method==="HEAD"?new Response(method==="GET"?'{"status":"ok"}\n':null,{headers:{"content-type":"application/json"}}):text(405,"Method not allowed",{allow:"GET, HEAD"});
  if(url.pathname==="/status"){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   const total=cumulative(),usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,evicted:{entries:usage.evictedEntries,bytes:usage.evictedBytes},blobs:usage.blobs,reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters:total.counters,...(options.ledger===undefined?{}:{session:counters,ledger:{sinceMs:options.ledger.sinceMs,restarts:options.ledger.restarts}}),startedAt,fleet:options.fleetStatus?.()??null,divergences:options.provenance?.summary()??{count:0,recent:[],since:0,byClass:{}},consumption:options.provenance?.consumption()??{since:0,total:0,pairs:{}},claims:claims.summary(),...(total.peers===undefined?{}:{peers:total.peers}),...(options.store.stalenessStatus()===undefined?{}:{retention:options.store.stalenessStatus()})};
   return Response.json(status);
  }
  if(url.pathname==="/evidence"||url.pathname.startsWith("/evidence/")){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   if(method!=="GET")return text(405,"Method not allowed",{allow:"GET"});
   if(options.provenance===undefined)return text(404,"No evidence kept");
   if(url.pathname==="/evidence")return Response.json({version:1,evidence:options.provenance.evidence.list()});
   const [,,id,name]=url.pathname.split("/"),bytes=id===undefined||name===undefined?undefined:await options.provenance.evidence.read(id,name);
   return bytes===undefined?text(404,"No such evidence"):new Response(bytes,{headers:{"content-type":"application/octet-stream"}});
  }
  const claimAddress=parseClaimPathV1(url.pathname,options.defaultInstance);
  let claimAccess:FleetAccessV1|undefined;
  if(claimAddress!==undefined){const guard=await authenticate(request,url,peer,claimAddress.action!=="await");if("denied" in guard)return guard.denied;claimAccess=guard.access;}
  const claimed=await answerClaimRequestV1({request,url,table:claims,defaultInstance:options.defaultInstance,authorize:()=>undefined,...(claimAccess===undefined||claimAccess.role==="static"?{}:{scope:claimAccess.origin}),readOnly,has:(namespace,kind,key)=>options.store.has(namespace,kind,key),readBody:async limit=>{const parts:Uint8Array[]=[];let size=0;try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit)return undefined;parts.push(chunk);}}catch{return undefined;}const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}return bound(claimAccess,bytes)?bytes:undefined;}});
  if(claimed)return claimed;
  const slotKeys=method==="GET"?parseRemoteCacheSlotKeysPathV1(url.pathname,options.defaultInstance):undefined;
  if(slotKeys!==undefined){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   if(!served(slotKeys.namespace))return foreign(slotKeys.namespace);
   return Response.json({slot:slotKeys.slot,keys:options.store.keysOfSlot(slotKeys.namespace,slotKeys.slot)});
  }
  const slots=parseRemoteCacheSlotsPathV1(url.pathname,options.defaultInstance);
  if(slots!==undefined){
   if(method!=="POST")return text(405,"Method not allowed",{allow:"POST"});
   const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
   if(readOnly)return text(403,"Forbidden: this cache is read-only");
   if(!served(slots))return foreign(slots);
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>1048576)return text(413,"Claim exceeds the maximum size");parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(!bound(guard.access,Buffer.concat(parts)))return text(400,"Content hash required and must match");
   let claim:{entries:{kind:Exclude<RemoteCacheKindV1,"cas">;key:string;slot:string}[]};
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
   if(address.kind==="ac"){const answered=await readResult(address.namespace,address.key,method,request);if(method==="GET"&&answered.status===200)options.provenance?.consume(address.namespace,"ac",address.key,guard.access===undefined?"anonymous":guard.access.role==="static"?"static":guard.access.origin);return answered;}
   const lease=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(lease!==undefined&&method==="HEAD"){try{await options.hostStore?.lease([address.key],undefined,lease.action);}catch{return text(503,"The lease could not be taken",{"retry-after":"1"});}}
   const entry=await options.store.read(address.namespace,address.kind,address.key),counter=counters[address.kind];
   if(!entry){const near=await throughPeers(request,address);if(near)return near;counter.misses++;return text(404,"Not found");}
   const whole=method==="GET"?await readRemoteCacheEntryV1(entry.path,entry.size):null;
   if(whole===undefined){counter.misses++;return text(404,"Not found");}
   if(method==="GET"&&address.kind===NX_KIND)options.provenance?.consume(address.namespace,NX_KIND,address.key,guard.access===undefined?"anonymous":guard.access.role==="static"?"static":guard.access.origin);
   counter.hits++;const headers={"content-type":"application/octet-stream","content-length":String(entry.size)};
   return new Response(whole==="lazy"?runtimeFileV1(entry.path).body():whole,{headers});
  }
  if(method==="DELETE"&&address.kind==="cas"&&parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1))!==undefined){
   const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
   if(readOnly)return text(403,"Forbidden: this cache is read-only");
   await options.hostStore?.releaseLease([address.key],parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1))!.action).catch(()=>undefined);return text(200,"Released");
  }
  if(method!=="PUT")return text(405,"Method not allowed",{allow:address.kind==="cas"?"GET, HEAD, PUT, DELETE":"GET, HEAD, PUT"});
  const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
  if(!served(address.namespace))return foreign(address.namespace);
  const access=guard.access,signed=access!==undefined&&access.role!=="static";
  if(signed&&access!.content==="-")return text(400,"A signed write must carry the hash of its body");
  if(signed&&address.kind==="cas"&&access!.content!==address.key)return text(400,"The signed content hash of a blob is its digest");
  if(readOnly)return text(403,"Forbidden: this cache is read-only");
  const counter=counters[address.kind],length=request.headers.get("content-length"),slotHeader=request.headers.get(SLOT_HEADER_V1);
  if(slotHeader!==null&&!isSlotIdV1(slotHeader))return text(400,"Invalid slot id");
  if(address.kind===NX_KIND&&length===null)return text(411,"Content-Length required");
  const declared=length===null?undefined:Number(length);
  if(declared!==undefined&&!Number.isSafeInteger(declared))return text(400,"Content-Length invalid");
  let source:AsyncIterable<Uint8Array>=body.chunks,known=declared,action:Uint8Array|undefined;
  if(address.kind==="ac"&&validation!=="off"){
   if(declared!==undefined&&declared>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(declared!==undefined&&size!==declared){counter.rejected++;return text(400,"Content-Length mismatch");}
   const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}
   if(!bound(access,bytes)){counter.rejected++;return text(400,"Content hash required and must match");}
   const verdict=await validateRemoteCacheActionResultV1(bytes,validation,blobs(address.namespace));
   if(!verdict.valid){counter.rejected++;return text(400,`Invalid action result: ${verdict.reason}`);}
   action=bytes;source=(async function*(){yield bytes;})();known=size;
  }
  let nxHash:ReturnType<typeof createHash>|undefined;
  if(address.kind===NX_KIND&&signed&&access!.content!=="-"){const hash=nxHash=createHash("sha256"),chunks=body.chunks;source=(async function*(){for await(const chunk of chunks){hash.update(chunk);yield chunk;}})();}
  const commit=async()=>{
   let fingerprint:string|undefined,unitShaped=false,prior:ReturnType<ProvenanceV1["of"]>,declaration:DeclarationV1|undefined;
   const staleness=options.store.stalenessPolicy;
   if(action!==undefined&&staleness!==undefined){
    declaration=await declarationOf(address.namespace,action,staleness.manifestNames);
    const stale=declaration===undefined?undefined:await options.store.staleOnArrival(declaration);
    if(stale!==undefined){counter.rejected++;return text(422,`stale-on-arrival: ${stale.reason} ${stale.detail}`);}
   }
   if(action!==undefined&&signed&&options.provenance!==undefined){
    const parsed=parseRemoteCacheStableResultV1(action,PROVENANCE_VOLATILE_OUTPUTS_V1);fingerprint=provenanceFingerprintV1([...parsed.stable,...parsed.trees].map(row=>row.hash));unitShaped=parsed.manifest;
    const held=await options.store.readBytes(address.namespace,"ac",address.key,limit),before=options.provenance.of(address.namespace,"ac",address.key),expect=request.headers.get("x-semio-expect");
    if(expect!==null&&held!==undefined&&expect!==createHash("sha256").update(held).digest("hex")){counter.conflicts++;return text(409,"Conflict: the head moved since it was read");}
    let old:ReturnType<typeof parseRemoteCacheStableResultV1>|undefined,superseding=false;
    if(held!==undefined)try{old=parseRemoteCacheStableResultV1(held,PROVENANCE_VOLATILE_OUTPUTS_V1);}catch{}
    const sort=provenanceClassV1([...parsed.paths,...(old?.paths??[])],parsed.manifest||old?.manifest===true);
    if(held!==undefined&&expect===null&&before?.origin!==access!.origin){
     if(sort.advisory)options.provenance.advised(sort.sort);
     else{
      const kept=old===undefined?before?.fingerprint:provenanceFingerprintV1([...old.stable,...old.trees].map(row=>row.hash));
      if(kept!==undefined&&kept!==fingerprint){
       const refusedRows=parsed.named,keptRows=old?.named??[],differing=provenanceDifferingV1(keptRows,refusedRows),reader=blobs(address.namespace);
       superseding=PROVENANCE_SUPERSEDING_CLASSES_V1.includes(sort.sort);
       if(!superseding)options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint,sort:sort.sort,differing});
       else options.provenance.superseded(sort.sort);
       void options.provenance.evidence.keep({outcome:superseding?"superseded":"refused",sort:sort.sort,namespace:address.namespace,key:address.key,keptOrigin:before?.origin??"unknown",refusedOrigin:access!.origin,differing,kept:held,refused:action,keptRows,refusedRows,blob:hash=>reader.read(hash)});
       if(!superseding){counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");}
      }
     }
    }
    prior=sort.advisory||superseding?undefined:before;
   }
   const leased=address.kind==="cas"?parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1)):undefined;
   if(leased!==undefined){try{await options.hostStore?.lease([address.key],undefined,leased.action);}catch{return text(503,"The lease could not be taken",{"retry-after":"1"});}}
   const outcome=await options.store.put(address.namespace,address.kind,address.key,source,{mode:mode(address.kind),verify:address.kind==="cas",declaredLength:known,...(address.kind==="cas"&&parseCasLeaseHeaderV1(request.headers.get(CAS_LEASE_HEADER_V1))!==undefined?{transient:true}:{}),...(slotHeader!==null&&address.kind!=="cas"?{slot:{slot:slotHeader,...(declaration===undefined?{}:{declaration})}}:{})});
   if(nxHash!==undefined&&(outcome==="created"||outcome==="replaced")&&nxHash.digest("hex")!==access!.content){await options.store.remove(address.namespace,NX_KIND,address.key);counter.rejected++;return text(400,"Content hash mismatch");}
   if(signed&&options.provenance!==undefined&&(outcome==="created"||outcome==="replaced")&&(prior===undefined||prior.origin===access!.origin||outcome==="created"))options.provenance.record(address.namespace,address.kind,address.key,access!.origin,fingerprint,unitShaped);
   switch(outcome){
    case "created":case "replaced":counter.puts++;if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");
    case "exists":if(address.kind===NX_KIND){counter.conflicts++;return text(409,"Conflict: cannot overwrite an existing cache entry");}if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");
    case "digest-mismatch":counter.rejected++;return text(400,"Digest mismatch");
    case "length-mismatch":counter.rejected++;return text(400,"Content-Length mismatch");
    case "too-large":counter.rejected++;return text(413,"Entry exceeds the maximum entry size");
    case "capacity":counter.rejected++;return text(507,"Insufficient storage"+(options.capacityDetail===undefined?"":": "+options.capacityDetail()),{"retry-after":"5"});
    case "unavailable":counter.rejected++;return text(503,"Cache write temporarily unavailable",{"retry-after":"5"});
    case "aborted":return text(499,"Client closed request");
   }
  };
  return address.kind==="ac"?serializeResult(address.namespace,address.key,commit):commit();
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
export type RemoteCacheServerOptionsV1=Readonly<{blobSharePercent?:number;graph?:Readonly<{sharePercent:number;supersedeGraceSeconds:number}>;fileBudget?:number;maximumConcurrentRequests?:number;reusePort?:boolean;successor?:boolean;startupScan?:"await"|"background";stopGraceMilliseconds?:number;onActivity?:(active:number)=>void;fleet?:RemoteCacheFleetOptionsV1;peers?:FleetPeerReadThroughV1;allowWildcard?:boolean;mirror?:Readonly<{directory:string;maximumBytes?:number}>;hostStore?:import("../../🗄️cas/🗃️store/🟦️.ts").CasStoreV1;relieveVolume?:CasGuardPortsV1["relieve"];directory:string;host:string;additionalHosts?:readonly string[];port:number;maximumBytes:number;maximumEntryBytes:number;maximumAgeDays:number;defaultInstance:string;instances?:readonly string[];tokens:RemoteCacheTokensV1;readOnly?:boolean;tls?:RemoteCacheTlsV1|undefined;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;maintenanceSeconds?:number;graceMilliseconds?:number;staleness?:Readonly<{mode:StalenessModeV1;workspace:string;recheckSeconds:number;manifestNames:readonly string[];examples:number;toolchains?:()=>ReadonlySet<string>|undefined}>;claimed?:(namespace:string,kind:RemoteCacheKindV1,key:string)=>boolean;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1|RemoteCacheRequestEventV1|Readonly<{stage:"listening";url:string}>|Readonly<{stage:"stopped"}>)=>void}>;

/** 🛰️ A started server: its address, a live status and an orderly stop. */
export interface RemoteCacheServerV1{readonly url:string;readonly port:number;readonly store:RemoteCacheStoreV1;readonly claims:ClaimTableV1;readonly stopped:Promise<void>;stop():Promise<void>}

/** 🚧️ Raised when another living server already owns the store directory. */
export class RemoteCacheDirectoryBusyV1 extends Error{readonly directory:string; constructor(directory:string){super(`Another remote cache server uses ${directory}`);this.directory=directory;}}

const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]),WILDCARD=new Set(["0.0.0.0","::","[::]"]);


/** 📐️ The bytes of blobs the entries of the store may reference: `sharePercent` of the host store's cap in force (the declared cap, bounded by the partition and by the room the host can still give, re-read from the host on every call, so the bound moves with the cap), which leaves the rest to transient offload inputs and local unit outputs. `undefined` when no share is declared. */
export async function remoteCacheBlobBoundV1(host:Pick<CasStoreV1,"root"|"policy">,sharePercent:number|undefined):Promise<number|undefined>{
 if(sharePercent===undefined)return undefined;
 const partition=measureCasPartitionV1(host.root),hint=await readCasUsageHintV1(host.root).catch(()=>undefined),cap=partition===undefined?host.policy.capBytes:casEffectiveCapBytesV1(host.policy,partition,hint?.bytes);
 return Math.floor(cap*sharePercent/100);
}

/** 🧾️ Names the bound behind a 507 in one line, English and German: the bytes the host store holds, the cap in force for its role and the free room of its partition, and the three largest holders of the census the last complete scan of the store left (read at most once a minute, so a refusal never waits for a scan). */
export function remoteCacheCapacityDetailV1(host:Pick<CasStoreV1,"root"|"policy">):()=>string{
 let holders="",used:number|undefined,measuredAt=0,running=false;
 const measure=():void=>{
  const at=Date.now();if(running||at-measuredAt<60000)return;running=true;
  void Promise.all([readCasCensusV1(host.root),readCasUsageHintV1(host.root)]).then(([census,hint])=>{
   used=census?.bytes??hint?.bytes;holders=[...census?.holders??[],...census?[{kind:"inputs",label:"transient-inputs",bytes:census.classes.leased.bytes}]:[]].sort((left,right)=>right.bytes-left.bytes).slice(0,3).map(row=>`${row.kind} ${row.label} ${Math.round(row.bytes/1073741824*100)/100} GiB`).join("; ");measuredAt=Date.now();
  }).catch(()=>undefined).finally(()=>{running=false;});
 };
 return ()=>{
  measure();
  const partition=measureCasPartitionV1(host.root),hint=used,gib=(bytes:number|undefined):number=>bytes===undefined?0:Math.round(bytes/1073741824*100)/100;
  const values={usedGib:gib(hint),capGib:gib(partition===undefined?host.policy.capBytes:casEffectiveCapBytesV1(host.policy,partition,hint)),role:partition?.role??host.policy.sizing.defaultRole,freeGib:gib(partition?.freeBytes),reserveGib:gib(partition===undefined?undefined:casReserveBytesV1(host.policy,partition)),holders:holders===""?"being measured":holders};
  return casTextV1("capRefusedDetail","en",values)+" / "+casTextV1("capRefusedDetail","de",values);
 };
}

/** 🚀️ Opens the store and serves the handler through the runtime interface, over TLS when a certificate is given; refuses to listen beyond loopback without a token, refuses a directory another server owns, keeps the store in step with outside collectors, and stops when `signal` aborts. */
export async function startRemoteCacheServerV1(options:RemoteCacheServerOptionsV1):Promise<RemoteCacheServerV1>{
 const registry=options.fleet===undefined?undefined:createFleetRegistryV1({storeRoot:options.fleet.storeRoot,group:options.fleet.group,...(options.fleet.groups===undefined?{}:{groups:options.fleet.groups}),...(options.fleet.peerStores===undefined?{}:{peerStores:options.fleet.peerStores}),...(options.fleet.pinned===undefined?{}:{pinned:options.fleet.pinned}),...(options.fleet.registryTtlMilliseconds===undefined?{}:{ttlMilliseconds:options.fleet.registryTtlMilliseconds}),pinsPath:options.fleet.pinsPath??join(options.directory,"fleet","pins.json")});
 const verifier=registry===undefined||options.fleet===undefined?undefined:createFleetVerifierV1({registry,audience:"cache",adminOrigin:options.fleet.adminOrigin??`${options.fleet.group}-server`});
 const gate=createFleetGateV1({verifier,networks:options.fleet?.networks,home:options.fleet?.group,...(options.fleet?.inGroup===undefined?{}:{inGroup:options.fleet.inGroup}),tokens:{read:options.tokens.read,write:options.tokens.write}});
 const hosts=[...new Set([options.host,...(options.additionalHosts??[])])];
 if(hosts.some(host=>!LOOPBACK.has(host))&&gate.open)throw Error("A token is required to listen beyond loopback (or fleet identities)");
 if(hosts.some(host=>WILDCARD.has(host))&&options.fleet!==undefined&&options.allowWildcard!==true)throw Error("Listen on the fleet interface, not on a wildcard address");
 if(!INSTANCE.test(options.defaultInstance)||RESERVED.has(options.defaultInstance))throw Error("Valid default instance required");
 if(options.tls){for(const path of[options.tls.certificate,options.tls.key])if(!(await stat(path).catch(()=>undefined))?.isFile())throw Error(`TLS file required: ${path}`);}
 const root=join(options.directory,"v1");await mkdir(root,{recursive:true});
 let owner=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1),closed=false;
 if(owner.held&&options.successor!==true)throw new RemoteCacheDirectoryBusyV1(options.directory);
 const release=async():Promise<void>=>{if(!owner.held)await owner.release();};
 const listeners:Awaited<ReturnType<typeof runtimeServeV1>>[]=[];
 let store:RemoteCacheStoreV1|undefined,timer:ReturnType<typeof setInterval>|undefined,enforceTimer:ReturnType<typeof setInterval>|undefined,beatTimer:ReturnType<typeof setInterval>|undefined;
 const tree:TreeDigestsV1|undefined=options.staleness===undefined||options.staleness.mode==="off"?undefined:await openTreeDigestsV1(options.staleness.workspace,join(root,".retention","digests.jsonl"));
 const table=createClaimTableV1({journal:openClaimJournalV1(join(root,CLAIM_POLICY_V1.journalFile))});
 const provenance=await openProvenanceV1(options.directory);
 const ledger=openCounterLedgerV1({path:join(root,"counters.json"),name:"remote-cache"});
 await registry?.refresh(true);
 const originPrivate=options.fleet?.root===undefined?undefined:readFleetOriginPrivateV1(options.fleet.root),exchangeKey=originPrivate===undefined?undefined:exchangePrivateKeyV1(originPrivate);
 const fleetStatus=():RemoteCacheFleetStatusV1=>{const origins:Record<string,number>={};for(const row of Object.values(registry?.origins()??{}))origins[row.status]=(origins[row.status]??0)+1;return {origins,denied:verifier?.denied()??{},transport:{crossGroup:"sealed",inGroup:options.fleet?.inGroup??"trusted-lan",sealedRoute:exchangeKey!==undefined}};};
 try{
  store=await RemoteCacheStoreV1.open({directory:options.directory,...options.fileBudget===undefined?{}:{fileBudget:options.fileBudget},...options.startupScan===undefined?{}:{startupScan:options.startupScan},...(options.hostStore?{host:options.hostStore,pinBudgetBytes:()=>{const host=options.hostStore!,partition=measureCasPartitionV1(host.root);return partition===undefined?undefined:Math.floor(casEffectiveCapBytesV1(host.policy,partition)*0.8);},blobBoundBytes:()=>remoteCacheBlobBoundV1(options.hostStore!,options.blobSharePercent),...options.graph===undefined?{}:{graph:{sharePercent:options.graph.sharePercent,supersedeGraceMilliseconds:options.graph.supersedeGraceSeconds*1000}}}:{}),maximumBytes:options.maximumBytes,maximumEntryBytes:options.maximumEntryBytes,maximumAgeMilliseconds:options.maximumAgeDays*86400000,...(tree===undefined||options.staleness===undefined?{}:{staleness:{mode:options.staleness.mode,tree,examples:options.staleness.examples,recheckMilliseconds:options.staleness.recheckSeconds*1000,manifestNames:options.staleness.manifestNames,...options.staleness.toolchains===undefined?{}:{toolchains:options.staleness.toolchains}}}),...(options.graceMilliseconds===undefined?{}:{graceMilliseconds:options.graceMilliseconds}),claimed:(namespace,kind,key)=>kind!=="cas"&&(table.claimed(namespace,kind,key)||options.claimed?.(namespace,kind,key)===true),...(options.signal?{signal:options.signal}:{}),...(options.onEvent?{onEvent:options.onEvent}:{})});
  const opened=store,handler=createRemoteCacheHandlerV1({store:opened,ledger,...(options.hostStore===undefined?{}:{capacityDetail:remoteCacheCapacityDetailV1(options.hostStore)}),claims:table,gate,provenance,...(options.hostStore===undefined?{}:{hostStore:options.hostStore}),...(options.peers===undefined?{}:{peers:options.peers}),...(registry===undefined?{}:{fleetStatus}),...(options.mirror?{mirror:createMirrorHandlerV1(options.mirror)}:{}),tokens:options.tokens,...(options.instances===undefined?{}:{instances:options.instances}),defaultInstance:options.defaultInstance,...(options.readOnly===undefined?{}:{readOnly:options.readOnly}),...(options.validation===undefined?{}:{validation:options.validation}),...(options.maximumActionResultBytes===undefined?{}:{maximumActionResultBytes:options.maximumActionResultBytes}),...(options.onEvent?{onRequest:options.onEvent}:{})});
  const sealed=exchangeKey===undefined||options.fleet===undefined?undefined:createSealedHandlerV1({privateKey:exchangeKey,group:options.fleet.group,maximumBytes:options.maximumEntryBytes,handle:handler}),answer=async(request:Request,peer?:string):Promise<Response>=>{if(sealed!==undefined&&!gate.network(peer))return new Response("Forbidden\n",{status:403});return (await sealed?.(request,peer))??handler(request,peer);};
  let active=0;const ceiling=options.maximumConcurrentRequests??Math.max(16,Math.floor((options.fileBudget??remoteCacheFileBudgetOfMachineV1())/4));const entry=async(request:Request,peer?:string):Promise<Response>=>{if(active>=ceiling)return new Response("Busy\n",{status:503,headers:{"retry-after":"1"}});active++;options.onActivity?.(active);try{return await answer(request,peer);}finally{active--;options.onActivity?.(active);}};
  for(const hostname of hosts)listeners.push(await runtimeServeV1({hostname,port:listeners[0]?.port??options.port,idleTimeoutSeconds:60,maximumRequestBodyBytes:options.maximumEntryBytes*2,...options.reusePort===true||options.successor===true?{reusePort:true}:{},...(options.tls?{tls:{certificate:options.tls.certificate,key:options.tls.key}}:{}),fetch:(request,peer)=>entry(request,peer)}));
  const host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host,port=listeners[0]!.port,url=`${options.tls?"https":"http"}://${host}:${port}`;
  let finish!:()=>void;const stopped=new Promise<void>(resolve=>{finish=resolve;});let closing:Promise<void>|undefined;
  const stop=():Promise<void>=>closing??=(async()=>{if(timer)clearInterval(timer);if(enforceTimer)clearInterval(enforceTimer);if(beatTimer)clearInterval(beatTimer);table.close();await table.flush();const graceful=Promise.all(listeners.map(listener=>listener.stop(false))),limit=options.stopGraceMilliseconds??20000;await Promise.race([graceful,new Promise<void>(done=>{const wait=setTimeout(done,limit);wait.unref();})]);await Promise.all(listeners.map(listener=>listener.stop(true)));closed=true;await opened.close();ledger.close();await provenance.flush();await release();options.onEvent?.({stage:"stopped"});finish();})();
  if(owner.held)void(async()=>{while(!closed){await new Promise<void>(done=>{const wait=setTimeout(done,500);wait.unref();});if(closed)return;const next=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1).catch(()=>undefined);if(next!==undefined&&!next.held){if(closed)await next.release();else owner=next;return;}}})();
  beatTimer=setInterval(()=>{if(!owner.held)void renewRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1).catch(()=>{});},60000);beatTimer.unref();
  const maintenance=(options.maintenanceSeconds??0)*1000;
  if(maintenance>0){
   let running=false;
   timer=setInterval(()=>{
    if(running)return;running=true;
    void(async()=>{const lock=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_COLLECTOR_LOCK_V1);if(lock.held)return;try{await opened.refresh();}finally{await lock.release();}})().catch(()=>{}).finally(()=>{running=false;});
   },maintenance);
   timer.unref();
  }
  if(options.hostStore!==undefined){
   const host=options.hostStore;let enforcing=false,declaredAt=0;
   const declare=():void=>{const at=Date.now();if(at-declaredAt<host.policy.holderRenewMs)return;declaredAt=at;try{declareCasRoleV1(host.root,"cache-host",host.policy,at);}catch{}};
   declare();
   enforceTimer=setInterval(()=>{
    if(enforcing)return;enforcing=true;declare();
    void casEnforceTickV1({root:host.root,policy:host.policy,...options.relieveVolume===undefined?{}:{relieve:options.relieveVolume}}).catch(()=>{}).finally(()=>{enforcing=false;});
   },host.policy.enforceTickMilliseconds);
   enforceTimer.unref();
  }
  if(options.signal){if(options.signal.aborted)await stop();else options.signal.addEventListener("abort",()=>{void stop();},{once:true});}
  options.onEvent?.({stage:"listening",url});
  return Object.freeze({url,port,store:opened,claims:table,stopped,stop});
 }catch(error){
  await Promise.allSettled(listeners.map(listener=>listener.stop(true)));
  closed=true;ledger.close();await store?.close().catch(()=>{});await release();throw error;
 }
}
