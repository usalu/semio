import {acquireAloneV1,acquireSlotV1,SlotWaitTimeoutV1,tryAcquireSlotV1,type ClaimV1} from "../🔒️lock/🟦️.ts";
import {awaitBrokerHeadroomV1} from "../🛰️broker/🟦️.ts";
import {laneMessageV1} from "../🚪️launch/🟦️.ts";
import {UNIT_CACHE_POLICY_V1,type AdmitFactsV1} from "../🔏️key/🟦️.ts";
import type {FleetHoldV1} from "../🛰️fleet/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
type Kind="slotTimeout"|"aloneTimeout"|"headroomTimeout"|"claimParked"|"claimLostAfterPark";

/** ⌚️ The bounds of the local admission wait, from the unit cache policy (`slot`, `broker`); every field can be narrowed for a test. */
export type CompileWaitBoundsV1=Readonly<{slotMaximumWaitMilliseconds:number;aloneMaximumWaitMilliseconds:number;parkAfterMilliseconds:number;headroomMaximumWaitMilliseconds:number;headroomPauseMilliseconds:number}>;

/** 🕰️ The authored bounds. */
export const COMPILE_WAIT_BOUNDS_V1:CompileWaitBoundsV1=Object.freeze({slotMaximumWaitMilliseconds:UNIT_CACHE_POLICY_V1.slot.maximumWaitMilliseconds,aloneMaximumWaitMilliseconds:UNIT_CACHE_POLICY_V1.slot.aloneMaximumWaitMilliseconds,parkAfterMilliseconds:UNIT_CACHE_POLICY_V1.slot.parkAfterMilliseconds,headroomMaximumWaitMilliseconds:UNIT_CACHE_POLICY_V1.broker.headroomMaximumWaitMilliseconds,headroomPauseMilliseconds:UNIT_CACHE_POLICY_V1.broker.headroomPauseMilliseconds});

/** 🎟️ How the wait ended: the compile slot (`undefined` when its bound passed, the unit then compiles without it and the memory broker still admits it), the time spent waiting, and what became of the fleet claim the unit held (`kept`, `lost` to another machine while it waited, `none` when it held none). */
export type CompileAdmissionV1=Readonly<{slot:ClaimV1|undefined;waitedMs:number;timedOut:"slot"|"alone"|"headroom"|undefined;claim:"kept"|"lost"|"none"}>;

/** 🪂️ Runs `work` (a wait for local admission) and, when it lasts longer than `afterMilliseconds`, gives the fleet claim of the unit back for the rest of the wait (`hold.park`) and takes it again when the wait is over (`hold.resume`), whatever way the wait ended: a unit that waits does not compile, so it must not keep other machines waiting for it. `claim` says what became of the claim; `onParked` and `onLost` report the two moments.
 * @see ../🛰️fleet/🟦️.ts (`FleetHoldV1`) */
export async function whileClaimParkedV1<T>(hold:Pick<FleetHoldV1,"park"|"resume"> | undefined,afterMilliseconds:number,work:()=>Promise<T>,notes:Readonly<{onParked?:()=>void;onLost?:()=>void}>={}):Promise<Readonly<{value:T;claim:"kept"|"lost"|"none"}>>{
 let parked:Promise<void>|undefined;
 const timer=setTimeout(()=>{if(hold===undefined)return;try{notes.onParked?.();}catch{void 0;}parked=hold.park().catch(()=>undefined);},afterMilliseconds);
 timer.unref?.();
 let claim:"kept"|"lost"|"none"=hold===undefined?"none":"kept";
 const settle=async():Promise<void>=>{
  clearTimeout(timer);
  if(parked===undefined||hold===undefined)return;
  await parked;
  claim=await hold.resume().catch(()=>"lost" as const)==="granted"?"kept":"lost";
  if(claim==="lost"){try{notes.onLost?.();}catch{void 0;}}
 };
 let value:T;
 try{value=await work();}catch(error){await settle();throw error;}
 await settle();
 return Object.freeze({value,claim});
}

/** 🛎️ Waits until a unit may compile on this machine: the broker's headroom first (the slot is only taken once the broker would admit the unit), then the compile slot, or the whole host for a unit that needs it. EVERY wait is bounded and ends in a bilingual line (`report`); a unit that finds the broker admitting and the slot free passes at once and never touches the fleet. A unit that has to wait longer than `parkAfterMilliseconds` gives its fleet claim back (`whileClaimParkedV1`), so no claim is held while the unit waits for admission; it takes the claim again before it runs.
 * @see ../🧪️tests/🪢️wedge/🟦️.ts
 * @see ../🛰️fleet/🟦️.ts (`FleetHoldV1`) */
export async function awaitCompileAdmissionV1(world:Readonly<{root:string;slots:number;crate:string;since:number;environment:Environment;facts:AdmitFactsV1|undefined;alone:boolean;hold?:Pick<FleetHoldV1,"park"|"resume">|undefined;report:(text:string)=>void;event:(waitedMs:number,reason?:string)=>void;count?:(reason:string)=>void;signal?:AbortSignal;bounds?:Partial<CompileWaitBoundsV1>}>):Promise<CompileAdmissionV1>{
 const bounds={...COMPILE_WAIT_BOUNDS_V1,...world.bounds},started=Date.now();
 const text=(kind:Kind,parameters:Readonly<Record<string,string|number>>={}):string=>(["en","de"] as const).map(language=>laneMessageV1(kind,{crate:world.crate,...parameters},language)).join(" / ");
 const say=(kind:Kind,parameters:Readonly<Record<string,string|number>>={}):void=>{try{world.report(text(kind,parameters));}catch{void 0;}};
 const seconds=():number=>Math.round((Date.now()-started)/1000);
 let slot:ClaimV1|undefined,timedOut:CompileAdmissionV1["timedOut"];
 const wait=async():Promise<void>=>{
  if(world.alone){
   try{slot=await acquireAloneV1(world.root,world.slots,{since:world.since,...world.signal?{signal:world.signal}:{},maximumWaitMilliseconds:bounds.aloneMaximumWaitMilliseconds,onWait:waited=>world.event(waited,"alone")});}
   catch(error){if(!(error instanceof SlotWaitTimeoutV1))throw error;timedOut="alone";say("aloneTimeout",{seconds:seconds(),bound:Math.round(bounds.aloneMaximumWaitMilliseconds/1000)});world.event(error.waitedMs,"timeout: compiled without being alone");world.count?.("alone-timeout");}
   return;
  }
  if(world.facts!==undefined&&await awaitBrokerHeadroomV1(world.environment,world.facts,{maximumWaitMilliseconds:bounds.headroomMaximumWaitMilliseconds,pauseMilliseconds:bounds.headroomPauseMilliseconds})==="timeout"){
   timedOut="headroom";say("headroomTimeout",{bound:Math.round(bounds.headroomMaximumWaitMilliseconds/1000)});world.event(0,"headroom: the broker did not admit the unit in time, taking the slot");
  }
  slot=tryAcquireSlotV1(world.root,world.slots);
  if(slot===undefined)try{slot=await acquireSlotV1(world.root,world.slots,{since:world.since,maximumWaitMilliseconds:bounds.slotMaximumWaitMilliseconds,...world.signal?{signal:world.signal}:{},onWait:waited=>world.event(waited)});}
  catch(error){if(!(error instanceof SlotWaitTimeoutV1))throw error;timedOut="slot";say("slotTimeout",{seconds:seconds(),bound:Math.round(bounds.slotMaximumWaitMilliseconds/1000)});world.event(error.waitedMs,"timeout: compiled without the slot");world.count?.("slot-timeout");}
 };
 const outcome=await whileClaimParkedV1(world.hold,bounds.parkAfterMilliseconds,wait,{onParked:()=>world.event(Date.now()-started,"claim-parked: "+text("claimParked")),onLost:()=>say("claimLostAfterPark")});
 return Object.freeze({slot,waitedMs:Date.now()-started,timedOut,claim:outcome.claim});
}
