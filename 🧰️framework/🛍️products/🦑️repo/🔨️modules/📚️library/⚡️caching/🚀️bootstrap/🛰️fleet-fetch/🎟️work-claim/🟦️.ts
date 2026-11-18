import {createHash,randomBytes} from "node:crypto";
import {hostname} from "node:os";
import type {FleetFetchContextV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored work-claim policy: key prefix, lease and heartbeat, wait bounds and the kinds of work.
 * @see ./🧬️schema/🔣️.json */
export const WORK_CLAIM_POLICY_V1=schema.definitions.Policy.const;
const policy=WORK_CLAIM_POLICY_V1;

/** 🏷️ What is being fetched or built: a pinned blob, the crate files of a lockfile, the wheels of a uv lock. */
export type WorkKindV1=typeof schema.definitions.Kind.enum[number];
/** 📣️ A step of one acquisition, for the caller's progress line. */
export type WorkClaimEventV1=Readonly<{stage:"claim"|"wait"|"ready"|"alone";key:string;holder?:string}>;
/** 🎟️ How an acquisition ended: the work is `ready` in the fleet (fetch it, compute nothing), this machine `granted` the claim and must do the work, then `release` (a heartbeat keeps the lease alive until then), or it runs `alone` because the fleet could not be asked or the wait ran out (the fail-open path: compute locally, nothing to release). */
export type WorkClaimV1=Readonly<{outcome:"ready"|"granted"|"alone";key:string;waitedMilliseconds:number;release():Promise<void>}>;

/** 🔑️ The claim key of one piece of work: `work-<kind>-<sha256 of the identity>`, the same on every machine and every platform for the same identity, and within the key alphabet of the claim route. */
export function workClaimKeyV1(kind:WorkKindV1,identity:string):string{
 if(!(policy.kinds as readonly string[]).includes(kind))throw Error("Known work kind required / Bekannte Arbeitsart erforderlich");
 return `${policy.keyPrefix}${kind}-${createHash("sha256").update(identity).digest("hex")}`;
}

const headers=(context:FleetFetchContextV1,json:boolean):Record<string,string>=>({...(context.remote?.token?{authorization:`Bearer ${context.remote.token}`}:{}),...(json?{"content-type":"application/json"}:{})});
const route=(context:FleetFetchContextV1,key:string,suffix:string):string=>`${context.remote!.endpoint}/${context.remote!.instance}/claim/nx/${key}${suffix}`;
const none=(key:string,outcome:"alone"|"ready",waited:number):WorkClaimV1=>Object.freeze({outcome,key,waitedMilliseconds:waited,release:async()=>{}});

const post=async(context:FleetFetchContextV1,key:string,suffix:string,body:Record<string,unknown>):Promise<Readonly<{status:number;state:string;by:string}>|undefined>=>{
 try{
  const response=await context.fetch(route(context,key,suffix),{method:"POST",headers:headers(context,true),body:JSON.stringify(body),signal:AbortSignal.timeout(policy.claimTimeoutMilliseconds)});
  const value=await response.json().catch(()=>({})) as {state?:string;by?:string};
  return {status:response.status,state:value.state??"",by:value.by??""};
 }catch{return undefined;}
};

/** 🎟️ Takes the fleet claim of one piece of work so that exactly one machine does it while the others wait for its result. In a loop bounded by `waitMaximumMilliseconds`: when `ready()` says the fleet already holds the result, the answer is `ready`; otherwise the claim route is asked (`POST claim/nx/<key>`): `granted` starts a heartbeat and re-checks `ready()` once (a holder may have finished between the check and the grant) before answering `granted`; `held` long-polls (`GET …/await`) until the holder releases, lapses or the slice ends and asks again, so a dead holder is replaced after its lease and a finished one is detected the moment it releases. A server that cannot be reached, refuses claims or is full, a context without a credential and an exhausted wait all answer `alone`: the caller does the work itself, which is what happened before the claim existed, so a broken claim path costs sharing, never the build. A cancellation (`check` throwing) releases the claim and propagates. A holder that finishes without publishing (or dies) wakes the waiters as `lapsed`; one of them takes the work over, and after `maximumLapses` such wakes without a result the rest stop waiting and work alone in parallel, so a result that cannot be published never serializes the fleet. The same key claimed twice by one owner is the same claim. */
export async function claimFleetWorkV1(context:FleetFetchContextV1,request:Readonly<{kind:WorkKindV1;identity:string;ready:()=>Promise<boolean>;owner?:string;check?:()=>void;event?:(event:WorkClaimEventV1)=>void;waitMaximumMilliseconds?:number;ttlMilliseconds?:number}>):Promise<WorkClaimV1>{
 const key=workClaimKeyV1(request.kind,request.identity),started=context.now(),waited=()=>context.now()-started;
 if(context.remote?.authenticated!==true)return none(key,"alone",0);
 const owner=request.owner??`${hostname().replace(/[^A-Za-z0-9._-]/gu,"-").slice(0,60)}:${process.pid}:${randomBytes(3).toString("hex")}`,ttl=request.ttlMilliseconds??policy.ttlMilliseconds,deadline=started+(request.waitMaximumMilliseconds??policy.waitMaximumMilliseconds),check=request.check??(()=>{});
 let timer:ReturnType<typeof setInterval>|undefined,held=false,lapses=0;
 const release=async():Promise<void>=>{if(timer){clearInterval(timer);timer=undefined;}if(held){held=false;await post(context,key,"/release",{owner});}};
 const finish=(outcome:"ready"|"granted"|"alone"):WorkClaimV1=>{request.event?.({stage:outcome==="granted"?"claim":outcome,key});return outcome==="granted"?Object.freeze({outcome,key,waitedMilliseconds:waited(),release}):Object.freeze({outcome,key,waitedMilliseconds:waited(),release:async()=>{}});};
 try{
  for(;;){
   check();
   if(await request.ready().catch(()=>false))return finish("ready");
   if(context.now()>=deadline)return finish("alone");
   const answer=await post(context,key,"",{owner,ttlMilliseconds:ttl});
   if(answer===undefined||answer.state==="full"||(answer.status!==200&&answer.status!==409))return finish("alone");
   if(answer.state==="granted"){
    held=true;let beats=0;
    timer=setInterval(()=>{if(++beats>policy.maximumBeats){clearInterval(timer);timer=undefined;return;}void post(context,key,"/renew",{owner,ttlMilliseconds:ttl});},policy.renewMilliseconds);
    (timer as {unref?:()=>void}).unref?.();
    if(await request.ready().catch(()=>false)){await release();return finish("ready");}
    return finish("granted");
   }
   request.event?.({stage:"wait",key,holder:answer.by});
   const slice=Math.max(0,Math.min(policy.awaitMilliseconds,deadline-context.now()));
   try{
    const response=await context.fetch(route(context,key,`/await?timeoutMilliseconds=${Math.floor(slice)}`),{headers:headers(context,false),signal:AbortSignal.timeout(slice+policy.claimTimeoutMilliseconds)});
    const waking=await response.json().catch(()=>({})) as {state?:string};
    if(response.status!==200)return finish("alone");
    if(waking.state==="lapsed"&&++lapses>=policy.maximumLapses&&!await request.ready().catch(()=>false))return finish("alone");
   }catch{return finish("alone");}
  }
 }catch(error){await release();throw error;}
}
