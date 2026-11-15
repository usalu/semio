import {randomBytes} from "node:crypto";
import {hostname} from "node:os";
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {type RustcUnitV1,type unitKeyV1} from "../🔏️key/🟦️.ts";
import {type UnitEventV1} from "../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "./📡️client/🟦️.ts";
import {fleetCacheRootV1,remoteCacheFetchV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {compilerReleaseV1,type CompilerReleaseV1,FLEET_POLICY_V1,hostIndependentV1,neutralReleaseV1,platformOfV1,portableRootsV1,remoteKeyV1,slotIdV1,slotPartsV1,type PlatformV1,type PortableRootV1} from "./🪪️identity/🟦️.ts";
import {enqueueUploadV1,markOfferedV1,offeredV1,startUploaderV1} from "./📤️uploader/🟦️.ts";
import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";

const fleet=FLEET_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;

/** 🛰️ One unit as the fleet tier sees it: its key and slot, the platform it needs and the roots that make its paths portable. */
export type FleetUnitV1=Readonly<{client:FleetClientV1;root:string;environment:Environment;localKey:string;key:string;slot:string;slotParts:readonly string[];crate:string;platform:PlatformV1;roots:readonly PortableRootV1[];cwd:string;workspace:string|undefined;laneRoot:string|undefined;owner:string;expected:readonly string[]}>;

/** 🏷️ The owner name a process claims under: machine, process and a random suffix, so a restarted process never inherits a dead one's claim. */
export const fleetOwnerV1=():string=>`${hostname().replace(/[^A-Za-z0-9._-]/gu,"-").slice(0,60)}:${process.pid}:${randomBytes(3).toString("hex")}`;

/** 🪢️ The identity of a unit that does not depend on the fleet being on: the compiler build and the slot (id and parts). `undefined` when the compiler build cannot be determined. */
export type UnitBasicsV1=Readonly<{release:CompilerReleaseV1;slot:string;slotParts:readonly string[]}>;

/** 🪢️ Determines the compiler build and the slot of one unit. Never throws. */
export function unitBasicsV1(input:Readonly<{root:string;environment:Environment;inner:readonly string[];unit:RustcUnitV1;args:readonly string[];cwd:string}>):UnitBasicsV1|undefined{
 try{
  const real=compilerReleaseV1(input.root,input.inner,input.environment),release=hostIndependentV1({release:real,unit:input.unit,args:input.args})?neutralReleaseV1(real):real,parts=slotPartsV1({release,unit:input.unit,args:input.args,environment:input.environment,cwd:input.cwd});
  return Object.freeze({release,slot:slotIdV1(parts),slotParts:parts});
 }catch{return undefined;}
}

/** 🧬️ The compiler identity that goes into the key of a unit: the compiler's own text, or for a unit whose output does not depend on the machine (`hostIndependentV1`) the release without its host line, so Linux, macOS and Windows machines key a WebAssembly library alike. Never throws. */
export function unitToolchainV1(input:Readonly<{root:string;environment:Environment;inner:readonly string[];unit:RustcUnitV1;args:readonly string[]}>,own:()=>string):string{
 try{return unitToolchainOfV1(compilerReleaseV1(input.root,input.inner,input.environment),input.unit,input.args,own);}catch{return own();}
}

/** 🧬️ `unitToolchainV1` for a compiler release already read (pure). */
export function unitToolchainOfV1(release:CompilerReleaseV1,unit:RustcUnitV1,args:readonly string[],own:()=>string):string{
 return hostIndependentV1({release,unit,args})?"host-independent:"+(release.neutral??release.digest):own();
}

/** 🚦️ Why the fleet tier is not used for a unit although the machine has a cache configured: the breaker state (`down`, `denied`) or `error`; `undefined` when the tier is off or usable. */
export function fleetSkipReasonV1(root:string,environment:Environment):string|undefined{
 try{const settings=readFleetSettingsV1(root,environment);if(!settings)return undefined;const state=createFleetClientV1(settings).state();return state==="ok"?"error":state;}catch{return "error";}
}

/** 🛰️ Prepares the fleet tier for one cache-eligible unit; `undefined` when the tier is off, the server is known to be down for now, or the unit cannot be shared. Never throws. */
export function prepareFleetV1(input:Readonly<{root:string;environment:Environment;basics:UnitBasicsV1;unit:RustcUnitV1;args:readonly string[];cwd:string;laneRoot:string|undefined;localKey:string;keyInput:Parameters<typeof unitKeyV1>[0];toolchain:string;crate:string;sensitive?:readonly string[];expected?:readonly string[]}>,world:Readonly<{client?:FleetClientV1}>={}):FleetUnitV1|undefined{
 try{
  const settings=readFleetSettingsV1(input.root,input.environment);if(!settings)return undefined;
  const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined,identityRoot=workspace??fleetCacheRootV1(input.environment);
  const client=world.client??createFleetClientV1(settings,identityRoot===undefined?{}:{fetch:remoteCacheFetchV1(input.environment,{root:identityRoot})});if(!client.usable())return undefined;
  const platform=platformOfV1({release:input.basics.release,unit:input.unit,args:input.args,environment:input.environment,toolchain:input.toolchain}),roots=portableRootsV1({cwd:input.cwd,workspace,hints:[input.environment.CARGO_MANIFEST_DIR,input.environment.PWD]});
  return Object.freeze({client,root:input.root,environment:input.environment,localKey:input.localKey,key:remoteKeyV1(platform,input.keyInput,roots,input.sensitive),slot:input.basics.slot,slotParts:input.basics.slotParts,crate:input.crate,platform,roots,cwd:input.cwd,workspace,laneRoot:input.laneRoot,owner:fleetOwnerV1(),expected:input.expected??[]});
 }catch{return undefined;}
}

const costFile=(unit:FleetUnitV1)=>join(unit.root,fleet.layout.state,fleet.layout.costs,unit.slot.slice(0,2),unit.slot);

/** ⏱️ How long this unit's slot took to compile the last time on this machine, `undefined` when never. */
export function knownCostV1(unit:FleetUnitV1):number|undefined{try{const value=Number(readFileSync(costFile(unit),"utf8"));return Number.isFinite(value)&&value>=0?value:undefined;}catch{return undefined;}}
/** ⏱️ Remembers what a compile of the unit's slot cost. */
export function rememberCostV1(unit:FleetUnitV1,milliseconds:number):void{try{const path=costFile(unit);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,String(Math.round(milliseconds)));}catch{}}

/** 📥️ Fetches the unit from the fleet cache and installs it into the local store, as if it had been compiled here (see `installFleetUnitV1`), then settles it: it is not offered again and the fleet is told it is current for its slot. */
export async function fetchFleetUnitV1(unit:FleetUnitV1):Promise<"hit"|"miss"|"fault">{
 const outcome=await installFleetUnitV1(unit);
 if(outcome!=="hit")return outcome;
 try{
  markOfferedV1(unit.root,unit.key);
  await Promise.race([unit.client.claimSlot(unit.key,unit.slot),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);
 }catch{}
 return "hit";
}

/** 🫀️ A claim this process holds at the fleet cache and keeps alive while it compiles. */
export type FleetHoldV1=Readonly<{release():Promise<void>;stop():void;lost():boolean}>;

function hold(unit:FleetUnitV1):FleetHoldV1{
 let stopped=false,gone=false;
 const timer=setInterval(()=>{void unit.client.renew(unit.key,unit.owner,fleet.claim.ttlMilliseconds).then(async state=>{if(state==="lost"&&!stopped){const reasserted=await unit.client.claim(unit.key,unit.owner,fleet.claim.ttlMilliseconds);if(reasserted.state==="granted")return;gone=true;stopped=true;clearInterval(timer);}});},fleet.claim.heartbeatMilliseconds);timer.unref?.();
 const stop=()=>{if(!stopped){stopped=true;clearInterval(timer);}};
 return Object.freeze({stop,lost:()=>gone,async release(){stop();if(!gone)await Promise.race([unit.client.release(unit.key,unit.owner),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);}});
}

/** 🧭️ What the fleet tier decided for one unit: it was installed (`hit`), or the caller compiles — holding the fleet claim when it was granted. `waitedMs` is the time spent waiting for another machine. */
export type FleetDecisionV1=Readonly<{outcome:"hit";waitedMs:number;via:"cache"|"wait"}>|Readonly<{outcome:"compile";hold:FleetHoldV1|undefined;waitedMs:number;reason:string}>;

/** 🧭️ The protocol for one missing unit. Look it up; on a miss claim it; if another machine holds the claim wait (long-polling, bounded by twice the holder's expected cost) for the upload and fetch it; when the holder lapses claim it ourselves. Anything unreachable, denied or too slow ends in "compile locally": correctness never depends on the fleet. */
export async function obtainFleetUnitV1(unit:FleetUnitV1,event:(kind:UnitEventV1["kind"],milliseconds:number,waitedMs:number,reason?:string)=>void,overrides:Partial<typeof fleet.claim>={}):Promise<FleetDecisionV1>{
 const policy={...fleet.claim,...overrides},started=Date.now(),compile=(reason:string,held?:FleetHoldV1):FleetDecisionV1=>({outcome:"compile",hold:held,waitedMs:Date.now()-started,reason});
 try{
  const first=await fetchFleetUnitV1(unit);
  if(first==="hit"){event("remote-hit",Date.now()-started,0,"cache");return {outcome:"hit",waitedMs:0,via:"cache"};}
  if(first==="fault"){event("remote-skip",Date.now()-started,0,"fault");return compile("fault");}
  event("remote-miss",Date.now()-started,0);
  if(!unit.client.claimsUsable())return compile("no-claims");
  const expected=knownCostV1(unit);let limit:number|undefined,deadline=Infinity;
  for(let round=0;round<policy.maximumRounds;round++){
   const claim=await unit.client.claim(unit.key,unit.owner,policy.ttlMilliseconds,expected);
   if(claim.state==="granted"){event("remote-claim",Date.now()-started,Date.now()-started,"granted");return compile("granted",hold(unit));}
   if(claim.state==="stored"){const again=await fetchFleetUnitV1(unit);if(again==="hit"){event("remote-hit",Date.now()-started,Date.now()-started,"stored");return {outcome:"hit",waitedMs:Date.now()-started,via:"wait"};}return compile("stored-unreadable");}
   if(claim.state!=="held")return compile(claim.state==="full"?"claims-full":"claims-"+claim.fault);
   if(limit===undefined){limit=Math.min(policy.maximumWaitMilliseconds,Math.max(policy.minimumWaitMilliseconds,policy.waitFactor*(claim.expectedMilliseconds??policy.defaultExpectedMilliseconds)));deadline=Date.now()+limit;}
   const remaining=deadline-Date.now();
   if(remaining<=0){event("remote-wait",0,Date.now()-started,"timeout");return compile("holder-slow");}
   const waited=await unit.client.await(unit.key,remaining);
   if(waited==="fault")return compile("await-fault");
   if(waited==="stored"){
    const again=await fetchFleetUnitV1(unit);
    if(again==="hit"){event("remote-hit",Date.now()-started,Date.now()-started,"awaited");return {outcome:"hit",waitedMs:Date.now()-started,via:"wait"};}
    if(again==="fault")return compile("fault");
   }
  }
  return compile("rounds");
 }catch{return compile("error");}
}

/** 🎁️ A local hit of a unit this machine never shared: queues it once so a machine that already built something fills the fleet cache too (the worker first asks the server whether it has the unit). Cheap when the unit is settled: one file test. */
export function offerFleetUnitV1(unit:FleetUnitV1):void{
 try{
  if(!unit.client.settings.upload||offeredV1(unit.root,unit.key))return;
  enqueueUploadV1(unit.root,{version:1,key:unit.key,localKey:unit.localKey,slot:unit.slot,crate:unit.crate,cwd:unit.cwd,workspace:unit.workspace??null,laneRoot:unit.laneRoot??null,owner:unit.owner,claimed:false,offer:true,costMs:0,platform:unit.platform,queuedMs:Date.now()})&&startUploaderV1(unit.root,unit.environment);
 }catch{}
}

/** 📮️ After a successful compile that was published locally: queues the unit for upload and starts the worker; the claim, if held, is handed over (the worker keeps it alive and the server drops it when the result arrives). Never waits for the network. */
export function shareFleetUnitV1(unit:FleetUnitV1,input:Readonly<{claimed:FleetHoldV1|undefined;costMs:number}>):boolean{
 try{
  rememberCostV1(unit,input.costMs);
  if(!unit.client.settings.upload||!unit.client.usable())return false;
  const queued=enqueueUploadV1(unit.root,{version:1,key:unit.key,localKey:unit.localKey,slot:unit.slot,crate:unit.crate,cwd:unit.cwd,workspace:unit.workspace??null,laneRoot:unit.laneRoot??null,owner:unit.owner,claimed:input.claimed!==undefined&&!input.claimed.lost(),costMs:Math.round(input.costMs),platform:unit.platform,queuedMs:Date.now()});
  if(queued){input.claimed?.stop();startUploaderV1(unit.root,unit.environment);}
  return queued;
 }catch{return false;}
}
