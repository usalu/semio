import {type FleetClientV1} from "../../🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🎟️ The authored claim and wait limits of the fleet flight.
 * @see ../🧬️schema/🔣️.json */
export const FLIGHT_POLICY_V1=schema.definitions.Policy.const.claim;
const timeouts=schema.definitions.Policy.const.timeouts;
type Policy=typeof FLIGHT_POLICY_V1;

/** 🫀️ A claim this process holds at the fleet cache and keeps alive while it computes. */
export type FlightHoldV1=Readonly<{release():Promise<void>;stop():void;lost():boolean}>;

/** 🧭️ What the flight decided: the entry was fetched (`hit`, `via` says whether after waiting for another machine), or the caller computes, holding the claim when it was granted. */
export type FlightDecisionV1=Readonly<{outcome:"hit";waitedMs:number;via:"cache"|"wait"}>|Readonly<{outcome:"compute";hold:FlightHoldV1|undefined;waitedMs:number;reason:string}>;

/** 🧾️ What a flight needs: the client, the entry's key, the claim owner, how to fetch the entry and how long computing it usually takes. */
export type FlightInputV1=Readonly<{client:FleetClientV1;key:string;owner:string;fetch():Promise<"hit"|"miss"|"fault">;expectedMilliseconds?:number;signal?:AbortSignal;policy?:Partial<Policy>}>;

function hold(client:FleetClientV1,key:string,owner:string,policy:Policy):FlightHoldV1{
 let stopped=false,gone=false;
 const timer=setInterval(()=>{void client.renew(key,owner,policy.ttlMilliseconds).then(async state=>{if(state==="lost"&&!stopped){const again=await client.claim(key,owner,policy.ttlMilliseconds);if(again.state==="granted")return;gone=true;stopped=true;clearInterval(timer);}});},policy.heartbeatMilliseconds);
 timer.unref?.();
 const stop=()=>{if(!stopped){stopped=true;clearInterval(timer);}};
 return Object.freeze({stop,lost:()=>gone,async release(){stop();if(!gone)await Promise.race([client.release(key,owner),new Promise<void>(done=>{const wait=setTimeout(done,timeouts.releaseMilliseconds);wait.unref?.();})]);}});
}

/** 🧭️ The protocol for one missing entry. Fetch it; on a miss claim it; if another machine holds the claim wait (long-polling, bounded by twice the holder's expected cost, at most the policy maximum) for the upload and fetch it; when the holder lapses claim it ourselves. Anything unreachable, denied, refused or too slow ends in "compute here": correctness never depends on the fleet, and no step waits without a bound.
 * @see ../../🦀️cargo/🗃️unit-cache/🛰️fleet/🟦️.ts */
export async function obtainFleetEntryV1(input:FlightInputV1):Promise<FlightDecisionV1>{
 const policy={...FLIGHT_POLICY_V1,...input.policy},started=Date.now(),compute=(reason:string,held?:FlightHoldV1):FlightDecisionV1=>({outcome:"compute",hold:held,waitedMs:Date.now()-started,reason});
 try{
  const first=await input.fetch();
  if(first==="hit")return {outcome:"hit",waitedMs:0,via:"cache"};
  if(first==="fault")return compute("fault");
  if(!input.client.claimsUsable())return compute("no-claims");
  const expected=input.expectedMilliseconds??policy.defaultExpectedMilliseconds;let deadline=Infinity;
  for(let round=0;round<policy.maximumRounds;round++){
   if(input.signal?.aborted)return compute("aborted");
   const claim=await input.client.claim(input.key,input.owner,policy.ttlMilliseconds,expected);
   if(claim.state==="granted")return compute("granted",hold(input.client,input.key,input.owner,policy));
   if(claim.state==="stored"){const again=await input.fetch();return again==="hit"?{outcome:"hit",waitedMs:Date.now()-started,via:"wait"}:compute("stored-unreadable");}
   if(claim.state!=="held")return compute(claim.state==="full"?"claims-full":"claims-"+claim.fault);
   if(deadline===Infinity)deadline=Date.now()+Math.min(policy.maximumWaitMilliseconds,Math.max(policy.minimumWaitMilliseconds,policy.waitFactor*(claim.expectedMilliseconds??policy.defaultExpectedMilliseconds)));
   const remaining=deadline-Date.now();
   if(remaining<=0)return compute("holder-slow");
   const waited=await input.client.await(input.key,remaining);
   if(waited==="fault")return compute("await-fault");
   if(waited==="stored"){const again=await input.fetch();if(again==="hit")return {outcome:"hit",waitedMs:Date.now()-started,via:"wait"};if(again==="fault")return compute("fault");}
  }
  return compute("rounds");
 }catch{return compute("error");}
}
