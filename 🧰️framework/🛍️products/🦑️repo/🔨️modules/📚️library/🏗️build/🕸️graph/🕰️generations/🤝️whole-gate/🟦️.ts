import {createHash} from "node:crypto";
import {hostname,totalmem} from "node:os";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import exchange from "../🌐️remote-exchange/🧬️schema/🔣️.json" with {type:"json"};
import {graphFleetClientV1,type GraphFleetClientV1} from "../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../🪪️graph-state/🧶️nx/🟦️.ts";
import {remoteCacheFetchV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {holdV1} from "../../📥️load/🤝️selected-exchange/🟦️.ts";
import type {GenerationLookV1} from "../🌐️remote-exchange/🟦️.ts";

/** 🏛️ The declared policy of the whole read of the fleet: the claim lease and its heartbeat, the slice and the bound of the wait, and from how many stale owners on a machine gates its read.
 * @see ./🧬️schema/🔣️.json */
export const WHOLE_GATE_POLICY_V1=schema.definitions.Policy.const;
/** 🗣️ The English and German texts of the gate, keyed by message.
 * @see ./🧬️schema/🔣️.json */
export const WHOLE_GATE_TEXTS_V1=schema.definitions.TextCatalogue.const;
const policy=WHOLE_GATE_POLICY_V1,sha=(value:string):string=>createHash("sha256").update(value).digest("hex");

/** 🔑️ The claim key of the whole read: the generation registry identity and the platform, nothing of this machine.
 * @see ./🧬️schema/🔣️.json */
export const wholeGateKeyV1=(input:Readonly<{identity:string;platform:string}>):string=>sha(JSON.stringify([policy.domain,input.identity,input.platform]));

/** 🚦️ What the gate decided: `fetch` a complete head is there, `waited` it appeared while this machine waited, `taken-over` the claim holder vanished and this machine reads, `read` this machine holds the claim and reads, `unreachable` the fleet cannot be used or the bound ended, `skipped` the machine holds enough of a generation to read the difference alone. */
export type WholeGateOutcomeV1=Readonly<{outcome:typeof schema.definitions.Outcome.enum[number];waitedMilliseconds:number;candidate?:GenerationLookV1["candidate"];funded?:true;release():Promise<void>}>;
type WholeGateFundingV1=Readonly<{release():void}>;

/** 🧱️ Everything the gate needs: the client of the fleet, the look at the heads of the group, and clock, sleep and rendering. */
export type WholeGateInputV1=Readonly<{client:GraphFleetClientV1;identity:string;platform:string;owner:string;language:"en"|"de";look():Promise<GenerationLookV1>;fund?():Promise<WholeGateFundingV1|undefined>;say?:(line:string)=>void;now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;signal?:AbortSignal;alive?:()=>boolean;capable?:boolean;smallMachineDelayMilliseconds?:number;claimTtlMilliseconds?:number;claimHeartbeatMilliseconds?:number;waitMaximumMilliseconds?:number;waitSliceMilliseconds?:number}>;

const render=(language:"en"|"de",key:keyof typeof WHOLE_GATE_TEXTS_V1,fields:Readonly<Record<string,string|number>>={}):string=>WHOLE_GATE_TEXTS_V1[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));
/** 📝️ One text of the gate in one language. */
export const wholeGateTextV1=render;

const fetchable=(look:GenerationLookV1):boolean=>look.state==="complete"&&look.ahead;

/** 🚪️ Lets one machine of the fleet read the whole workspace and every other fetch what it sealed: a complete head that a restore would take ends the gate at once (`fetch`); else the whole-read claim is taken (the holder reads and calls `release` after it published) or waited for with progress and a bound; the wait ends when a complete head appears (`waited`), when the holder's lease lapses (`taken-over`, this machine reads) or at the bound (`unreachable`). Any fleet fault, a client without claims or upload right ends in a local read, never in a failure. Cancelling throws.
 * @see ./🧪️tests/🟦️.ts */
export async function wholeReadGateV1(input:WholeGateInputV1):Promise<WholeGateOutcomeV1>{
 const{client,language}=input,now=input.now??Date.now,started=now(),say=input.say??(()=>undefined),key=wholeGateKeyV1(input),ttl=input.claimTtlMilliseconds??policy.claimTtlMilliseconds,beat=input.claimHeartbeatMilliseconds??policy.claimHeartbeatMilliseconds,limit=input.waitMaximumMilliseconds??policy.waitMaximumMilliseconds,slice=input.waitSliceMilliseconds??policy.waitSliceMilliseconds;
 let booking:WholeGateFundingV1|undefined,transferred=false;
 const releaseFunding=()=>{const current=booking;booking=undefined;current?.release();};
 const done=(outcome:WholeGateOutcomeV1["outcome"],release:()=>Promise<void>=async()=>undefined,keep=false):WholeGateOutcomeV1=>{
  transferred=keep&&booking!==undefined;
  return {outcome,waitedMilliseconds:now()-started,...transferred?{funded:true as const,release:async()=>{try{releaseFunding();}finally{await release();}}}:{release}};
 };
 const fetching=(look:GenerationLookV1,outcome:"fetch"|"waited")=>{say(render(language,"fetching",{owners:look.owners??0,age:Math.round((look.ageSeconds??0)/60)}));return {...done(outcome),candidate:look.candidate};};
 const unusable=(reason:string)=>{say(render(language,"unreachable",{reason}));return done("unreachable");};
 const first=await input.look().catch(()=>undefined);
 if(first!==undefined&&fetchable(first))return fetching(first,"fetch");
 if(first!==undefined&&first.held>0)return done("skipped");
 if(!client.usable()||!client.claimsUsable())return unusable("claims");
 if(!client.settings.upload)return unusable("read-only");
 let since:number|undefined,lapsed=false,last=first;
 if(input.capable===false){
  const delay=input.smallMachineDelayMilliseconds??policy.smallMachineDelayMilliseconds,until=now()+delay;
  say(render(language,"yielding",{minutes:Math.max(1,Math.round(delay/60000))}));
  while(now()<until){
   input.signal?.throwIfAborted();
   await(input.sleep??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))))(Math.min(slice,Math.max(1,until-now())));
   last=await input.look().catch(()=>last);
   if(last!==undefined&&fetchable(last))return fetching(last,"waited");
   if(last!==undefined&&last.held>0)return done("skipped");
  }
 }
 let peer:{state:string;by?:string}|undefined;
 try{
  for(let round=0;round<policy.maximumRounds;round++){
   input.signal?.throwIfAborted();
   if(peer===undefined&&input.fund!==undefined){
    booking=await input.fund();
    input.signal?.throwIfAborted();
    if(booking===undefined)return done("fetch");
   }
   const claim=peer??await client.claim(key,input.owner,ttl).catch(()=>({state:"fault"}) as never) as {state:string;by?:string};
   if(claim.state==="granted"){
    const hold=holdV1(client,key,input.owner,ttl,beat,input.alive);
    try{
     input.signal?.throwIfAborted();
     const again=await input.look().catch(()=>undefined);
     input.signal?.throwIfAborted();
     if(again!==undefined&&fetchable(again)){releaseFunding();await hold.release();return fetching(again,"waited");}
     say(render(language,lapsed?"takenOver":"claimed"));
     return done(lapsed?"taken-over":"read",()=>hold.release(),true);
    }catch(error){releaseFunding();await hold.release();throw error;}
   }
   input.signal?.throwIfAborted();
   if(claim.state!=="held"&&claim.state!=="stored"){
    say(render(language,"unreachable",{reason:claim.state}));
    return done("unreachable",undefined,true);
   }
   releaseFunding();
   since??=now();
   const remaining=since+limit-now();
   if(remaining<=0){say(render(language,"expired",{minutes:Math.round(limit/60000),by:String(claim.by??"?")}));return done("unreachable");}
   const progress=last!==undefined&&last.state==="partial"?render(language,"progress",{owners:last.owners??0,stale:last.stale??0}):"";
   say(render(language,"waiting",{by:String(claim.by??"?"),seconds:Math.round((now()-since)/1000),limit:Math.round(limit/60000),progress}));
   const waited=await client.await(key,Math.min(slice,remaining),input.signal).catch(()=>"fault" as const);
   input.signal?.throwIfAborted();
   if(waited==="fault")return unusable("await");
   last=await input.look().catch(()=>last);
   if(last!==undefined&&fetchable(last))return fetching(last,"waited");
   peer=input.fund!==undefined&&waited!=="lapsed"?claim:undefined;
   if(waited==="lapsed"){lapsed=true;if(input.sleep)await input.sleep(50);}
   if(peer!==undefined&&waited==="stored")await (input.sleep??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds))))(Math.min(slice,remaining));
  }
  return unusable("rounds");
 }finally{if(!transferred)releaseFunding();}
}

/** 🔌️ The gate of this machine for the loader: `undefined` while no fleet cache is configured or the fleet graph exchange is off; the client is the one of the selected exchange (same fleet settings, same signed fetch).
 * @see ../🔌️load/🟨️.mjs */
export function wholeReadGateOfHostV1(input:Readonly<{root:string;environment?:Readonly<Record<string,string|undefined>>;platform?:string;language:"en"|"de";say?:(line:string)=>void;origin?:string;memoryMegabytes?:number}>):((request:Readonly<{identity:string;look():Promise<GenerationLookV1>;signal?:AbortSignal;alive?:()=>boolean;fund?:WholeGateInputV1["fund"]}>)=>Promise<WholeGateOutcomeV1>)|undefined{
 const environment=input.environment??process.env;let client:GraphFleetClientV1|undefined;
 try{const settings=graphFleetSettingsV1(input.root,environment);client=settings===undefined?undefined:graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:input.root})});}catch{client=undefined;}
 if(client===undefined)return undefined;
 const platform=input.platform??`${process.platform}-${process.arch}`.toLowerCase(),owner=`${input.origin??hostname()}:${process.pid}`;
 const capable=(input.memoryMegabytes??totalmem()/1048576)>=exchange.definitions.Policy.const.servingTotalMegabytes;
 return request=>wholeReadGateV1({client:client!,platform,owner,language:input.language,capable,...input.say?{say:input.say}:{},...request});
}
