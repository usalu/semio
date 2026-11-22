import {createHash,randomBytes} from "node:crypto";
import {hostname} from "node:os";
import {mkdirSync,readFileSync,realpathSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {type RustcUnitV1,type unitKeyV1} from "../🔏️key/🟦️.ts";
import {type UnitEventV1} from "../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "./📡️client/🟦️.ts";
import {fleetCacheRootV1,remoteCacheFetchV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {compilerReleaseV1,type CompilerReleaseV1,FLEET_POLICY_V1,hostIndependentV1,neutralReleaseV1,platformOfV1,portableRootsV1,remoteKeyV1,slotIdV1,slotPartsV1,type PlatformV1,type PortableRootV1,type ToolRootsV1} from "./🪪️identity/🟦️.ts";
import {enqueueUploadV1,markOfferedV1,offeredV1,startUploaderV1} from "./📤️uploader/🟦️.ts";
import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../../../🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";

const fleet=FLEET_POLICY_V1;
const pause=(milliseconds:number):Promise<void>=>new Promise(done=>{setTimeout(done,milliseconds);});
type Environment=Readonly<Record<string,string|undefined>>;

/** 🛰️ One unit as the fleet tier sees it: its key and slot, the platform it needs and the roots that make its paths portable. */
export type FleetUnitV1=Readonly<{client:FleetClientV1;root:string;environment:Environment;localKey:string;key:string;slot:string;slotParts:readonly string[];crate:string;platform:PlatformV1;roots:readonly PortableRootV1[];cwd:string;workspace:string|undefined;laneRoot:string|undefined;owner:string;expected:readonly string[];tools?:ToolRootsV1;run?:true}>;

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

/** 🚦️ Why a unit that this machine could share has no fleet identity (`prepareFleetV1` answered nothing) although a cache is configured: `no-compiler-identity` (the compiler build or the slot of the unit cannot be determined); `undefined` when no cache is configured. A breaker that is open is no reason: the unit is prepared anyway and its upload waits in the spool. */
export function fleetUnidentifiedReasonV1(root:string,environment:Environment):string|undefined{
 try{return readFleetSettingsV1(root,environment)?"no-compiler-identity":undefined;}catch{return "no-compiler-identity";}
}

/** 🛰️ Prepares the fleet tier for one cache-eligible unit, also while the breaker is open (lookup and claims then wait for the breaker, the upload of the compiled unit waits in the spool); `undefined` when the tier is off or the unit has no identity. Never throws. */
export function prepareFleetV1(input:Readonly<{root:string;environment:Environment;basics:UnitBasicsV1;unit:RustcUnitV1;args:readonly string[];cwd:string;laneRoot:string|undefined;localKey:string;keyInput:Parameters<typeof unitKeyV1>[0];toolchain:string;crate:string;sensitive?:readonly string[];expected?:readonly string[];tools?:ToolRootsV1}>,world:Readonly<{client?:FleetClientV1}>={}):FleetUnitV1|undefined{
 try{
  const settings=readFleetSettingsV1(input.root,input.environment);if(!settings)return undefined;
  const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined,identityRoot=workspace??fleetCacheRootV1(input.environment);
  const client=world.client??createFleetClientV1(settings,identityRoot===undefined?{}:{fetch:remoteCacheFetchV1(input.environment,{root:identityRoot})});
  const platform=platformOfV1({release:input.basics.release,unit:input.unit,args:input.args,environment:input.environment,toolchain:input.toolchain}),roots=portableRootsV1({cwd:input.cwd,workspace,hints:[input.environment.CARGO_MANIFEST_DIR,input.environment.PWD],...input.tools});
  return Object.freeze({client,root:input.root,environment:input.environment,localKey:input.localKey,key:remoteKeyV1(platform,input.keyInput,roots,input.sensitive),slot:input.basics.slot,slotParts:input.basics.slotParts,crate:input.crate,platform,roots,cwd:input.cwd,workspace,laneRoot:input.laneRoot,owner:fleetOwnerV1(),expected:input.expected??[],...input.tools===undefined?{}:{tools:input.tools}});
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
  const pending=Promise.race([unit.client.claimSlot(unit.key,unit.slot).then(()=>undefined,()=>undefined),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);
  settling.add(pending);void pending.finally(()=>settling.delete(pending));
 }catch{}
 return "hit";
}

const settling=new Set<Promise<void>>();
/** ⏳️ Waits (at most 600 ms, the bound of each claim) for the slot claims that fleet hits started and did not wait for: a hit serves its unit while the fleet is told, and the caller awaits this once before the process ends. */
export async function awaitFleetSettlesV1():Promise<void>{
 if(settling.size>0)await Promise.allSettled([...settling]);
}

/** 🫀️ A claim this process holds at the fleet cache and keeps alive while it compiles. */
export type FleetHoldV1=Readonly<{release():Promise<void>;stop():void;lost():boolean}>;

function hold(unit:FleetUnitV1):FleetHoldV1{
 let stopped=false,gone=false;
 const timer=setInterval(()=>{void unit.client.renew(unit.key,unit.owner,fleet.claim.ttlMilliseconds).then(async state=>{if(state==="lost"&&!stopped){const reasserted=await unit.client.claim(unit.key,unit.owner,fleet.claim.ttlMilliseconds);if(reasserted.state==="granted")return;gone=true;stopped=true;clearInterval(timer);}});},fleet.claim.heartbeatMilliseconds);timer.unref?.();
 const stop=()=>{if(!stopped){stopped=true;clearInterval(timer);}};
 return Object.freeze({stop,lost:()=>gone,async release(){stop();if(!gone)await Promise.race([unit.client.release(unit.key,unit.owner),new Promise<void>(done=>{const wait=setTimeout(done,600);wait.unref?.();})]);}});
}

/** 🌳️ The directories of a machine a compiled unit can hold although its source never names them: the kinds a unit is keyed on when it does (`lane` is no kind: a lane directory belongs to one lane of one machine, a unit holding it is not shared). */
export const ROOT_KINDS_V1=Object.freeze(["cargo-home","cwd","manifest","rustup-home","workspace"] as const);
export type RootKindV1=typeof ROOT_KINDS_V1[number];

/** 🌳️ Where the root hint of a unit lives: an action result beside the neutral key of the unit, so a machine that finds it knows the unit is keyed on its directories. */
export const rootHintKeyV1=(neutralKey:string):string=>createHash("sha256").update(JSON.stringify(["semio.cargo.unit-cache.root-hint/v1",neutralKey])).digest("hex");

/** 🌳️ The root hint: an action result whose inline standard output names the kinds of directory the unit holds. */
export const rootHintBytesV1=(kinds:readonly string[]):Uint8Array=>encodeProtobufV1(REAPI_V1.actionResult,{stdout_raw:Buffer.from(JSON.stringify({version:1,roots:[...new Set(kinds)].sort()})),exit_code:0});

/** 🌳️ The kinds a root hint names (sorted, without duplicates); `undefined` for anything that is not a hint of the closed shape. */
export function parseRootHintV1(bytes:Uint8Array):readonly RootKindV1[]|undefined{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),value=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8")) as {version:unknown;roots:unknown};
  if(value.version!==1||!Array.isArray(value.roots)||value.roots.length===0||!value.roots.every(kind=>(ROOT_KINDS_V1 as readonly unknown[]).includes(kind)))return undefined;
  return [...new Set(value.roots as RootKindV1[])].sort();
 }catch{return undefined;}
}

/** 🌳️ The directories a unit is keyed on for the given kinds, as sorted `kind=path` texts; `undefined` when this machine lacks one of them. */
export function rootPathsV1(kinds:readonly string[],named:Readonly<Record<string,string|undefined>>):readonly string[]|undefined{
 const rows=[...new Set(kinds)].sort().map(kind=>{const value=named[kind];return value===undefined||value===""?undefined:kind+"="+value;});
 return rows.every(row=>row!==undefined)?rows as string[]:undefined;
}

/** 🌳️ Which of the named directories (and their resolved spellings) the compiled outputs hold, with the first output that does (dependency files are not scanned); `undefined` when none. */
export function leakedRootKindsV1(outputs:readonly string[],named:Readonly<Record<string,string|undefined>>):Readonly<{file:string;kinds:readonly string[]}>|undefined{
 const resolved=(value:string)=>{try{return realpathSync(value);}catch{return value;}};
 const roots=Object.entries(named).filter((row):row is [string,string]=>typeof row[1]==="string"&&row[1].length>=4).map(([kind,value])=>[kind,[...new Set([value,resolved(value)])]] as const),kinds=new Set<string>();
 let file:string|undefined;
 for(const path of outputs){
  if(path.endsWith(".d"))continue;
  let bytes:Buffer;try{bytes=readFileSync(path);}catch{continue;}
  for(const [kind,spellings] of roots)if(spellings.some(spelling=>bytes.includes(spelling))){kinds.add(kind);file??=path.split("/").at(-1)+": "+spellings[0];}
 }
 return kinds.size===0||file===undefined?undefined:Object.freeze({file,kinds:[...kinds].sort()});
}

/** 🌳️ The kinds the root hint of a unit names, `undefined` when there is none or the fleet cannot say. Takes the unit under its NEUTRAL key. */
export async function rootHintV1(neutral:FleetUnitV1):Promise<readonly RootKindV1[]|undefined>{
 try{if(!neutral.client.usable())return undefined;const answer=await neutral.client.getResult(rootHintKeyV1(neutral.key));return typeof answer==="string"?undefined:parseRootHintV1(answer);}catch{return undefined;}
}

/** 🌳️ Publishes the root hint of a unit (taking the unit under its neutral key) and answers the kinds in force: the ones given, or those of a hint that was there first. `undefined` when the fleet refused or cannot say. */
export async function publishRootHintV1(neutral:FleetUnitV1,kinds:readonly string[]):Promise<readonly RootKindV1[]|undefined>{
 return publishRootHintByKeyV1(neutral.client,neutral.key,kinds);
}

/** 🌳️ `publishRootHintV1` for a client and the neutral key of the unit (the upload worker publishes the hint of a unit that was compiled while the cache was unreachable). */
export async function publishRootHintByKeyV1(client:FleetClientV1,neutralKey:string,kinds:readonly string[]):Promise<readonly RootKindV1[]|undefined>{
 try{
  if(!client.settings.upload||!client.usable())return undefined;
  const key=rootHintKeyV1(neutralKey),stored=await client.putResult(key,rootHintBytesV1(kinds),key);
  if(stored===true)return parseRootHintV1(rootHintBytesV1(kinds));
  if(stored!=="diverged")return undefined;
  const answer=await client.getResult(key);return typeof answer==="string"?undefined:parseRootHintV1(answer);
 }catch{return undefined;}
}

/** 🫀️ Claims the key of a unit for this process and keeps the claim alive; `undefined` when another machine holds it or the fleet cannot say. */
export async function holdFleetKeyV1(unit:FleetUnitV1):Promise<FleetHoldV1|undefined>{
 try{const claim=await unit.client.claim(unit.key,unit.owner,fleet.claim.ttlMilliseconds);return claim.state==="granted"?hold(unit):undefined;}catch{return undefined;}
}

/** 🧭️ What the fleet tier decided for one unit: it was installed (`hit`), or the caller compiles — holding the fleet claim when it was granted. `waitedMs` is the time spent waiting for another machine. */
export type FleetDecisionV1=Readonly<{outcome:"hit";waitedMs:number;via:"cache"|"wait"}>|Readonly<{outcome:"compile";hold:FleetHoldV1|undefined;waitedMs:number;reason:string}>;

/** 🧭️ The protocol for one missing unit. Look it up; on a miss claim it; if another machine holds the claim wait (long-polling, bounded by twice the holder's expected cost) for the upload and fetch it; when the holder lapses claim it ourselves (`redirect`, asked before every further round, may answer the same unit under another key, which the protocol then follows: the root-keyed key of a unit that turned out to hold a directory). A claim, wait or fetch that fails because the server is momentarily unreachable (`down`) is tried again up to `retries` times with a growing pause before the unit compiles locally; a refused credential or an unsupported server is not retried. Anything unreachable, denied or too slow ends in "compile locally" with its reason (the caller counts it): correctness never depends on the fleet. */
export async function obtainFleetUnitV1(unit:FleetUnitV1,event:(kind:UnitEventV1["kind"],milliseconds:number,waitedMs:number,reason?:string)=>void,overrides:Partial<typeof fleet.claim>={},redirect?:()=>Promise<FleetUnitV1|undefined>):Promise<FleetDecisionV1>{
 const policy={...fleet.claim,...overrides},started=Date.now(),compile=(reason:string,held?:FleetHoldV1):FleetDecisionV1=>({outcome:"compile",hold:held,waitedMs:Date.now()-started,reason});
 try{
  const first=await fetchFleetUnitV1(unit);
  if(first==="hit"){event("remote-hit",Date.now()-started,0,"cache");return {outcome:"hit",waitedMs:0,via:"cache"};}
  if(first==="fault"){event("remote-skip",Date.now()-started,0,"fault");return compile("fault");}
  event("remote-miss",Date.now()-started,0);
  if(!unit.client.claimsUsable())return compile("no-claims");
  const expected=knownCostV1(unit);let limit:number|undefined,deadline=Infinity,transient=0;
  for(let round=0;round<policy.maximumRounds;round++){
   if(round>0&&redirect!==undefined){const moved=await redirect();if(moved!==undefined)return obtainFleetUnitV1(moved,event,overrides);}
   const claim=await unit.client.claim(unit.key,unit.owner,policy.ttlMilliseconds,expected);
   if(claim.state==="granted"){event("remote-claim",Date.now()-started,Date.now()-started,"granted");return compile("granted",hold(unit));}
   if(claim.state==="stored"){const again=await fetchFleetUnitV1(unit);if(again==="hit"){event("remote-hit",Date.now()-started,Date.now()-started,"stored");return {outcome:"hit",waitedMs:Date.now()-started,via:"wait"};}return compile("stored-unreadable");}
   if(claim.state==="unavailable"&&claim.fault==="down"&&transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}
   if(claim.state!=="held")return compile(claim.state==="full"?"claims-full":"claims-"+claim.fault);
   if(limit===undefined){limit=Math.min(policy.maximumWaitMilliseconds,Math.max(policy.minimumWaitMilliseconds,policy.waitFactor*(claim.expectedMilliseconds??policy.defaultExpectedMilliseconds)));deadline=Date.now()+limit;}
   const remaining=deadline-Date.now();
   if(remaining<=0){event("remote-wait",0,Date.now()-started,"timeout");return compile("holder-slow");}
   const waited=await unit.client.await(unit.key,remaining);
   if(waited==="fault"){if(transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}return compile("await-fault");}
   if(waited==="stored"){
    const again=await fetchFleetUnitV1(unit);
    if(again==="hit"){event("remote-hit",Date.now()-started,Date.now()-started,"awaited");return {outcome:"hit",waitedMs:Date.now()-started,via:"wait"};}
    if(again==="fault"){if(transient<policy.retries){transient++;await pause(policy.retryBackoffMilliseconds*transient);continue;}return compile("fault");}
   }
  }
  return compile("rounds");
 }catch{return compile("error");}
}

const jobExtras=(unit:FleetUnitV1)=>({...unit.run?{run:true as const}:{},...unit.tools===undefined?{}:{tools:{...unit.tools}}});

/** 🎁️ A local hit of a unit this machine never shared: queues it once so a machine that already built something fills the fleet cache too (the worker first asks the server whether it has the unit). Cheap when the unit is settled: one file test. */
export function offerFleetUnitV1(unit:FleetUnitV1):void{
 try{
  if(!unit.client.settings.upload||offeredV1(unit.root,unit.key))return;
  enqueueUploadV1(unit.root,{version:1,key:unit.key,localKey:unit.localKey,slot:unit.slot,crate:unit.crate,cwd:unit.cwd,workspace:unit.workspace??null,laneRoot:unit.laneRoot??null,owner:unit.owner,claimed:false,offer:true,costMs:0,platform:unit.platform,queuedMs:Date.now(),...jobExtras(unit)})&&startUploaderV1(unit.root,unit.environment);
 }catch{}
}

/** 🚫️ Why a unit that was compiled and published locally did not enter the upload spool: `read-only` (this machine may not write to the fleet cache: nothing it compiles ever seeds the fleet), else `spool-full`. */
export function unsharedReasonV1(unit:FleetUnitV1):string{
 return unit.client.settings.upload?"spool-full":"read-only";
}

/** 📮️ After a successful compile that was published locally: queues the unit for upload (also while the breaker is open: the worker holds the job until the fleet cache answers or the job is too old) and starts the worker; the claim, if held, is handed over (the worker keeps it alive and the server drops it when the result arrives). Never waits for the network. */
export function shareFleetUnitV1(unit:FleetUnitV1,input:Readonly<{claimed:FleetHoldV1|undefined;costMs:number;hint?:Readonly<{neutralKey:string;kinds:readonly string[]}>}>):boolean{
 try{
  rememberCostV1(unit,input.costMs);
  if(!unit.client.settings.upload)return false;
  const queued=enqueueUploadV1(unit.root,{version:1,key:unit.key,localKey:unit.localKey,slot:unit.slot,crate:unit.crate,cwd:unit.cwd,workspace:unit.workspace??null,laneRoot:unit.laneRoot??null,owner:unit.owner,claimed:input.claimed!==undefined&&!input.claimed.lost(),costMs:Math.round(input.costMs),platform:unit.platform,queuedMs:Date.now(),...jobExtras(unit),...input.hint===undefined?{}:{hint:{neutralKey:input.hint.neutralKey,kinds:[...input.hint.kinds]}}});
  if(queued){input.claimed?.stop();startUploaderV1(unit.root,unit.environment);}
  return queued;
 }catch{return false;}
}
