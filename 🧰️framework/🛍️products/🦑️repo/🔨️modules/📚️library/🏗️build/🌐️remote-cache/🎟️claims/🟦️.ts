import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {ClaimJournalV1,ClaimSnapshotV1} from "./💽️journal/🟦️.ts";

export {openClaimJournalV1} from "./💽️journal/🟦️.ts";
export type {ClaimJournalV1,ClaimSnapshotV1} from "./💽️journal/🟦️.ts";

/** 📐️ The authored claim policy: TTL bounds, the longest long-poll, and table limits.
 * @see ./🧬️schema/🔣️.json */
export const CLAIM_POLICY_V1=schema.definitions.Policy.const;
const policy=CLAIM_POLICY_V1,OWNER=new RegExp(policy.ownerPattern,"u"),KEY=/^[a-z0-9_-]{1,128}$/u,DIGEST=/^[0-9a-f]{64}$/u,INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,RESERVED=new Set(["ac","cas","v1","healthz","status","claim"]);

/** 🏷️ The keyspaces whose missing entries can be claimed: Bazel action results and Nx task hashes. */
export type ClaimKindV1="ac"|"nx";
/** 🎟️ The answer to a claim: this owner computes it, someone else does, it is already stored, or the table is full (the caller computes locally). */
export type ClaimOutcomeV1=Readonly<{state:"granted";owner:string;untilMilliseconds:number;ttlMilliseconds:number}>|Readonly<{state:"held";by:string;untilMilliseconds:number;expectedMilliseconds?:number}>|Readonly<{state:"stored"}>|Readonly<{state:"full"}>;
/** 🔁️ The answer to a heartbeat. */
export type RenewOutcomeV1=Readonly<{state:"granted";owner:string;untilMilliseconds:number;ttlMilliseconds:number}>|Readonly<{state:"lost"}>;
/** ⏳️ Why a long poll returned. */
export type AwaitOutcomeV1=Readonly<{state:"stored"|"lapsed"|"timeout"}>;
/** 📊️ The claim table summary `GET /status` carries. */
export type ClaimSummaryV1=Readonly<{active:number;waiting:number;oldestSeconds:number}>;

type Claim={owner:string;grantedMs:number;untilMs:number;ttlMs:number;expectedMs:number|undefined};
const lifetimeOf=(claim:Claim):number=>Math.min(policy.maximumLifetimeMilliseconds,Math.max(policy.minimumLifetimeMilliseconds,claim.ttlMs,(claim.expectedMs??0)*policy.lifetimeMultiple));
type Waiter={resolve:(outcome:AwaitOutcomeV1)=>void;timer:ReturnType<typeof setTimeout>|undefined};

/** 🧰️ What the table needs from the outside: a clock and a presence test of the stored entry. */
export type ClaimTableOptionsV1=Readonly<{now?:()=>number;sweepMilliseconds?:number;maximumClaims?:number;maximumWaiters?:number;journal?:ClaimJournalV1}>;

/** 🎟️ The claim table of one cache server. */
export interface ClaimTableV1{
 claim(namespace:string,kind:ClaimKindV1,key:string,request:Readonly<{owner:string;ttlMilliseconds:number;expectedMilliseconds?:number}>,stored?:boolean):ClaimOutcomeV1;
 renew(namespace:string,kind:ClaimKindV1,key:string,request:Readonly<{owner:string;ttlMilliseconds:number}>):RenewOutcomeV1;
 release(namespace:string,kind:ClaimKindV1,key:string,owner:string):"released"|"absent";
 stored(namespace:string,kind:ClaimKindV1,key:string):void;
 await(namespace:string,kind:ClaimKindV1,key:string,timeoutMilliseconds:number,isStored:()=>Promise<boolean>):Promise<AwaitOutcomeV1>;
 claimed(namespace:string,kind:ClaimKindV1,key:string):boolean;
 sweep():void;
 summary():ClaimSummaryV1;
 snapshot():ClaimSnapshotV1;
 flush():Promise<void>;
 close():void;
}

/** 🏗️ Creates the table. A claim lives `ttlMilliseconds` (clamped) and dies with its owner unless the owner renews it; a stored entry or a release wakes every waiter, a lapse wakes them with `lapsed` so one of them can claim and compute. The table lives in memory and, when a journal is given, in an append-only file as well: a restarted server restores the claims that were alive (their holders keep renewing and the fleet never computes a unit twice because the server was replaced); without a journal a restart forgets claims, which costs duplicate work, never a wrong result. */
export function createClaimTableV1(options:ClaimTableOptionsV1={}):ClaimTableV1{
 const now=options.now??Date.now,maximumClaims=options.maximumClaims??policy.maximumClaims,maximumWaiters=options.maximumWaiters??policy.maximumWaiters,claims=new Map<string,Claim>(),waiters=new Map<string,Set<Waiter>>();
 const id=(namespace:string,kind:string,key:string)=>`${namespace}\u0000${kind}\u0000${key}`,capped=new Map<string,{owner:string;untilMs:number}>();
 const clamp=(ttl:number)=>Math.min(policy.maximumTtlMilliseconds,Math.max(policy.minimumTtlMilliseconds,Math.floor(ttl)));
 const journal=options.journal;
 let waiting=0;
 const put=(address:string,claim:Claim):void=>{claims.set(address,claim);journal?.record({t:"set",address,owner:claim.owner,grantedMs:claim.grantedMs,untilMs:claim.untilMs,ttlMs:claim.ttlMs,...(claim.expectedMs===undefined?{}:{expectedMs:claim.expectedMs})});};
 const drop=(address:string):void=>{if(claims.delete(address))journal?.record({t:"drop",address});};
 const cool=(address:string,owner:string,untilMs:number):void=>{capped.set(address,{owner,untilMs});journal?.record({t:"cool",address,owner,untilMs});};
 if(journal){
  for(const row of journal.restored.claims)claims.set(row.address,{owner:row.owner,grantedMs:row.grantedMs,untilMs:row.untilMs,ttlMs:row.ttlMs,expectedMs:row.expectedMs});
  for(const row of journal.restored.cooling)capped.set(row.address,{owner:row.owner,untilMs:row.untilMs});
 }
 const wake=(address:string,state:AwaitOutcomeV1["state"])=>{
  const set=waiters.get(address);if(!set)return;waiters.delete(address);
  for(const waiter of set){if(waiter.timer)clearTimeout(waiter.timer);waiting--;waiter.resolve({state});}
 };
 const live=(address:string,at:number):Claim|undefined=>{
  const claim=claims.get(address);if(!claim)return undefined;
  if(claim.untilMs>at)return claim;
  drop(address);wake(address,"lapsed");return undefined;
 };
 const sweep=()=>{const at=now();for(const address of[...claims.keys()])live(address,at);};
 const sweepMilliseconds=options.sweepMilliseconds??policy.sweepMilliseconds,timer=sweepMilliseconds>0?setInterval(sweep,sweepMilliseconds):undefined;
 timer?.unref?.();
 const table:ClaimTableV1=Object.freeze<ClaimTableV1>({
  claim(namespace,kind,key,request,stored=false){
   if(stored)return {state:"stored"};
   const address=id(namespace,kind,key),at=now(),current=live(address,at),ttl=clamp(request.ttlMilliseconds),cooling=capped.get(address);
   if(cooling&&cooling.untilMs>at&&cooling.owner===request.owner)return {state:"held",by:"lifetime-cap",untilMilliseconds:cooling.untilMs};
   if(current&&current.owner===request.owner&&at-current.grantedMs>=lifetimeOf(current)){drop(address);cool(address,current.owner,at+policy.cooloffMilliseconds);wake(address,"lapsed");return {state:"held",by:"lifetime-cap",untilMilliseconds:at+policy.cooloffMilliseconds};}
   if(current&&current.owner!==request.owner)return {state:"held",by:current.owner,untilMilliseconds:current.untilMs,...(current.expectedMs===undefined?{}:{expectedMilliseconds:current.expectedMs})};
   if(!current&&claims.size>=maximumClaims){sweep();if(claims.size>=maximumClaims)return {state:"full"};}
   const next:Claim={owner:request.owner,grantedMs:current?.grantedMs??at,untilMs:at+ttl,ttlMs:ttl,expectedMs:request.expectedMilliseconds??current?.expectedMs};
   put(address,next);return {state:"granted",owner:next.owner,untilMilliseconds:next.untilMs,ttlMilliseconds:ttl};
  },
  renew(namespace,kind,key,request){
   const address=id(namespace,kind,key),at=now(),current=live(address,at);
   if(!current||current.owner!==request.owner)return {state:"lost"};
   if(at-current.grantedMs>=lifetimeOf(current)){drop(address);cool(address,current.owner,at+policy.cooloffMilliseconds);wake(address,"lapsed");return {state:"lost"};}
   const ttl=clamp(request.ttlMilliseconds);current.untilMs=Math.min(at+ttl,current.grantedMs+lifetimeOf(current));current.ttlMs=ttl;put(address,current);
   return {state:"granted",owner:current.owner,untilMilliseconds:current.untilMs,ttlMilliseconds:ttl};
  },
  release(namespace,kind,key,owner){
   const address=id(namespace,kind,key),current=live(address,now());
   if(!current||current.owner!==owner)return "absent";
   drop(address);wake(address,"lapsed");return "released";
  },
  stored(namespace,kind,key){const address=id(namespace,kind,key);drop(address);wake(address,"stored");},
  async await(namespace,kind,key,timeoutMilliseconds,isStored){
   const address=id(namespace,kind,key),limit=Math.min(policy.maximumAwaitMilliseconds,Math.max(0,Math.floor(timeoutMilliseconds)));
   if(waiting>=maximumWaiters)return {state:"timeout"};
   if(await isStored())return {state:"stored"};
   if(!live(address,now()))return {state:"lapsed"};
   return new Promise<AwaitOutcomeV1>(resolve=>{
    const waiter:Waiter={resolve,timer:undefined},set=waiters.get(address)??new Set<Waiter>();
    waiter.timer=setTimeout(()=>{set.delete(waiter);if(set.size===0&&waiters.get(address)===set)waiters.delete(address);waiting--;resolve({state:"timeout"});},limit);
    set.add(waiter);waiters.set(address,set);waiting++;
    void isStored().then(done=>{if(done&&set.has(waiter)){set.delete(waiter);if(waiter.timer)clearTimeout(waiter.timer);waiting--;resolve({state:"stored"});}},()=>{});
   });
  },
  claimed(namespace,kind,key){return live(id(namespace,kind,key),now())!==undefined;},
  sweep,
  snapshot(){return {claims:[...claims].map(([address,claim])=>({address,owner:claim.owner,grantedMs:claim.grantedMs,untilMs:claim.untilMs,ttlMs:claim.ttlMs,...(claim.expectedMs===undefined?{}:{expectedMs:claim.expectedMs})})),cooling:[...capped].map(([address,row])=>({address,owner:row.owner,untilMs:row.untilMs}))};},
  summary(){
   const at=now();let oldest=0,active=0;
   for(const claim of claims.values()){if(claim.untilMs<=at)continue;active++;oldest=Math.max(oldest,at-claim.grantedMs);}
   return {active,waiting,oldestSeconds:Math.floor(oldest/1000)};
  },
  flush:()=>journal?.flush()??Promise.resolve(),
  close(){if(timer)clearInterval(timer);for(const address of[...waiters.keys()])wake(address,"timeout");claims.clear();},
 });
 journal?.bind(()=>table.snapshot());
 return table;
}

/** 🧭️ A parsed claim route. */
export type ClaimAddressV1=Readonly<{namespace:string;kind:ClaimKindV1;key:string;action:"claim"|"renew"|"release"|"await"}>;

/** 🧭️ Resolves `/<instance>/claim/<kind>/<key>[/renew|/release|/await]` to its parts, `undefined` for every other path. */
export function parseClaimPathV1(pathname:string,defaultInstance:string):ClaimAddressV1|undefined{
 const segments=pathname.split("/").slice(1);let namespace=defaultInstance;
 if(segments.length>0&&!RESERVED.has(segments[0]!)){namespace=segments.shift()!;if(!INSTANCE.test(namespace))return undefined;}
 if(segments[0]!=="claim"||segments.length<3||segments.length>4)return undefined;
 const kind=segments[1] as ClaimKindV1,key=segments[2]!,action=segments[3]??"claim";
 if(!(policy.kinds as readonly string[]).includes(kind))return undefined;
 if(!(kind==="ac"?DIGEST:KEY).test(key))return undefined;
 if(action!=="claim"&&action!=="renew"&&action!=="release"&&action!=="await")return undefined;
 if(segments.length===3&&action!=="claim")return undefined;
 if(segments.length===4&&segments[3]==="claim")return undefined;
 return {namespace,kind,key,action};
}

/** 🛡️ Whether a string is a valid owner name. */
export const claimOwnerValidV1=(owner:unknown):owner is string=>typeof owner==="string"&&OWNER.test(owner);

/** 🛣️ What the server gives the claim routes: authorization, the stored-entry test, a bounded body reader. */
export type ClaimRouteInputV1=Readonly<{scope?:string;request:Request;url:URL;table:ClaimTableV1;defaultInstance:string;authorize:(write:boolean)=>Response|undefined;readOnly:boolean;has:(namespace:string,kind:ClaimKindV1,key:string)=>Promise<boolean>;readBody:(limit:number)=>Promise<Uint8Array|undefined>}>;

const json=(status:number,body:unknown)=>Response.json(body,{status}),bad=(message:string)=>new Response(message+"\n",{status:400,headers:{"content-type":"text/plain"}});

/** 🛣️ Answers one claim route (`POST …/claim/<kind>/<key>`, `…/renew`, `…/release`, `GET …/await`) or returns `undefined` when the path is none. Claiming, renewing and releasing need the write token; awaiting needs the read token. */
export async function answerClaimRequestV1(input:ClaimRouteInputV1):Promise<Response|undefined>{
 const address=parseClaimPathV1(input.url.pathname,input.defaultInstance);if(!address)return undefined;
 const {namespace,kind,key,action}=address,method=input.request.method;
 if(action==="await"){
  if(method!=="GET")return new Response("Method not allowed\n",{status:405,headers:{allow:"GET"}});
  const denied=input.authorize(false);if(denied)return denied;
  const asked=Number(input.url.searchParams.get("timeoutMilliseconds")??policy.maximumAwaitMilliseconds);
  if(!Number.isFinite(asked)||asked<0)return bad("timeoutMilliseconds invalid");
  return json(200,await input.table.await(namespace,kind,key,asked,()=>input.has(namespace,kind,key)));
 }
 if(method!=="POST")return new Response("Method not allowed\n",{status:405,headers:{allow:"POST"}});
 const denied=input.authorize(true);if(denied)return denied;
 if(input.readOnly)return new Response("Forbidden: this cache is read-only\n",{status:403});
 const raw=await input.readBody(4096);if(raw===undefined)return bad("Body too large or content hash missing");
 let body:Record<string,unknown>;try{body=JSON.parse(new TextDecoder().decode(raw)||"{}");}catch{return bad("Body is not JSON");}
 if(typeof body!=="object"||body===null||Array.isArray(body))return bad("Body must be an object");
 if(!claimOwnerValidV1(body.owner))return bad("owner invalid");
 const given=body.owner,owner=input.scope===undefined?given:`${input.scope}|${given}`,view=(row:Record<string,unknown>):Record<string,unknown>=>{const out={...row};for(const field of["owner","by"]){const value=out[field];if(typeof value!=="string"||input.scope===undefined)continue;const bar=value.indexOf("|");if(bar>0)out[field]=value===owner?given:value.slice(0,bar);}return out;};
 if(action==="release")return json(200,{state:input.table.release(namespace,kind,key,owner)});
 const ttl=body.ttlMilliseconds;if(typeof ttl!=="number"||!Number.isFinite(ttl)||ttl<1)return bad("ttlMilliseconds invalid");
 if(action==="renew"){const outcome=input.table.renew(namespace,kind,key,{owner,ttlMilliseconds:ttl});return json(outcome.state==="granted"?200:409,view(outcome));}
 const expected=body.expectedMilliseconds;if(expected!==undefined&&(typeof expected!=="number"||!Number.isFinite(expected)||expected<0))return bad("expectedMilliseconds invalid");
 const outcome=input.table.claim(namespace,kind,key,{owner,ttlMilliseconds:ttl,...(expected===undefined?{}:{expectedMilliseconds:Math.floor(expected as number)})},await input.has(namespace,kind,key));
 if(outcome.state==="granted")await input.table.flush();
 return json(outcome.state==="held"?409:outcome.state==="full"?503:200,view(outcome));
}
