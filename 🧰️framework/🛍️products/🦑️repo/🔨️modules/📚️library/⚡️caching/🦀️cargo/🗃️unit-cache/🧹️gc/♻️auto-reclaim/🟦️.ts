import {execFile} from "node:child_process";
import {existsSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import {createNativeImageKickPortV1,kickNativeImageV1,nativeImageKickIneffectiveTextV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/👟️kick/🟦️.ts";
import {nativeImageBackingFilenameV1,nativeImageRootV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {sleepV1,type ClaimV1} from "../../🔒️lock/🟦️.ts";
import {cacheLedgerV1,cacheReclaimShareBytesV1} from "../../../../💾️storage/🧾️ledger/🟦️.ts";
import {gcNeedBytesV1,measureVolumeV1,unitBudgetBytesV1,type VolumeFactsV1} from "../🟦️.ts";
import {tryClaimWithBallastV1} from "../🛟️ballast/🟦️.ts";
import {collectIncrementalV1,type IncrementalLockPortV1} from "../🌱️incremental/🟦️.ts";
import {collectComponentRunsV1} from "../🧩️runs/🟦️.ts";
import {collectUnitsV1,type ProfileLeasePortV1,type ProfileLivePortV1} from "../🧱️units/🟦️.ts";
import {fillRatesV1,fillReportV1,readFillSamplesV1} from "../📈️fill/🟦️.ts";
import type {CargoGcControlV1} from "../📏️tree/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const,layout=UNIT_CACHE_POLICY_V1.layout,GIB=1024**3;
/** 📐️ The authored policy of the automatic reclaim of the Rust compiler image.
 * @see ./🧬️schema/🔣️.json */
export const RUST_IMAGE_RECLAIM_POLICY_V1=policy;
/** 🚰️ The free space below which the image is reclaimed and the free space the reclaim works up to. */
export type RustImageReclaimRuleV1=Readonly<{softAvailableBytes:number;targetAvailableBytes:number}>;
/** 🌱️ The held-unit ports of the collectors; tests replace them, production uses the compiler locks and build leases. */
export type RustImageHeldPortsV1=Readonly<{lock?:IncrementalLockPortV1;leases?:ProfileLeasePortV1;live?:ProfileLivePortV1}>;

/** 📜️ The soft and target available bytes the ledger declares for the Rust image (`cargo-store`) on an image of this volume: a share of the volume, because the images of the fleet differ in size. */
export function rustImageReclaimRuleV1(volumeBytes:number):RustImageReclaimRuleV1{
 const area=cacheLedgerV1().areas.find(row=>row.id==="cargo-store");
 if(area?.reclaimShare)return cacheReclaimShareBytesV1(area.reclaimShare,volumeBytes);
 if(area?.reclaim)return area.reclaim;
 throw Error("Rust image reclaim rule required in the cache ledger");
}

/** ⚖️ Whether the available space is below the soft threshold. Pure. */
export function autoReclaimDueV1(availableBytes:number,rule:RustImageReclaimRuleV1):boolean{return availableBytes<rule.softAvailableBytes;}

/** 🎯️ The bytes a due reclaim must free to reach the target, none while not due. Pure. */
export function autoReclaimDemandBytesV1(availableBytes:number,rule:RustImageReclaimRuleV1):number{return autoReclaimDueV1(availableBytes,rule)?Math.max(0,rule.targetAvailableBytes-availableBytes):0;}

/** 💬️ One operator message with its placeholders filled. */
export function autoReclaimTextV1(kind:keyof typeof messages,parameters:Readonly<Record<string,string|number>>,language:"en"|"de"):string{return messages[kind][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(parameters[name]??""));}

/** 🌱️ Collects (or, in a dry run, only reports) what the Rust image can give back, least recently used first: the Cargo incremental sessions, then the component runs a newer run supersedes, then the stale build units. Lock-aware: a unit a running build holds is never removed. It frees the most of the pressure rule of the image, the machine quota and the demand, measured by `measure`. Shared by `repo cache gc` and the automatic reclaim. */
export async function collectRustImageV1(store:string,options:Readonly<{dryRun:boolean;language:"en"|"de";demandBytes:number;quotaBytes:number;unitBudgetBytes:number;measure:()=>VolumeFactsV1;control:(collector:string)=>CargoGcControlV1;write:(line:string)=>void}&RustImageHeldPortsV1>):Promise<void>{
 const{dryRun,measure,quotaBytes,demandBytes}=options,volume=measure(),need=Math.max(gcNeedBytesV1(volume.volumeBytes,volume.freeBytes),quotaBytes,demandBytes),report={language:options.language,write:options.write},held={lock:options.lock,leases:options.leases,live:options.live};
 const remaining=(planned:number):number=>{if(dryRun)return Math.max(0,need-planned);const now=measure(),gained=now.freeBytes-volume.freeBytes;return Math.max(gcNeedBytesV1(now.volumeBytes,now.freeBytes),quotaBytes-gained,demandBytes-gained,0);};
 const sessions=await collectIncrementalV1(store,{dryRun,volumeBytes:volume.volumeBytes,needBytes:()=>need,satisfied:()=>need>0&&measure().freeBytes-volume.freeBytes>=need,report,lock:held.lock,...options.control("incremental")});
 const afterSessions=dryRun?volume:measure(),needRuns=remaining(sessions.freedBytes);
 const runs=await collectComponentRunsV1(store,{dryRun,needBytes:()=>needRuns,satisfied:()=>{
  const current=measure();
  return afterSessions.unmeasured||current.unmeasured?undefined:current.freeBytes-afterSessions.freeBytes>=needRuns;
 },report,...options.control("runs")});
 const afterRuns=dryRun?volume:measure(),needUnits=remaining(sessions.freedBytes+runs.freedBytes);
 await collectUnitsV1(store,{dryRun,needBytes:()=>needUnits,budgetBytes:options.unitBudgetBytes,satisfied:()=>needUnits>0&&measure().freeBytes-afterRuns.freeBytes>=needUnits,report,...held,...options.control("units")});
 options.write(fillReportV1(fillRatesV1(readFillSamplesV1(join(store,"u")),Date.now()),measure().freeBytes,options.language));
}

/** 🏁️ How one request ended: nothing to do, space reclaimed, nothing reclaimable (all held), another agent's reclaim finished meanwhile, or a recent empty reclaim still cooling. */
export type RustImageReclaimResultV1=Readonly<{outcome:"not-due"|"reclaimed"|"nothing"|"joined"|"cooling";availableBytes:number;freedBytes:number}>;
/** 🔌️ What a request needs from outside the module. */
export type RustImageReclaimRequestV1=Readonly<{
 root:string;capacityBytes:number;rule:RustImageReclaimRuleV1|((volumeBytes:number)=>RustImageReclaimRuleV1);language:"en"|"de";report:(line:string)=>void;signal?:AbortSignal;
 measure?:(root:string,capacityBytes:number)=>VolumeFactsV1;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;
 claim?:(directory:string,name:string)=>ClaimV1|undefined;
 collect?:(request:Readonly<{store:string;demandBytes:number;signal:AbortSignal;language:"en"|"de";progress:(stage:string,done:number,total:number)=>void;report:(line:string)=>void}>&RustImageHeldPortsV1)=>Promise<void>;
 kick?:(request:Readonly<{signal:AbortSignal}>)=>Promise<void>;
}&RustImageHeldPortsV1>;

type FlightV1={promise:Promise<RustImageReclaimResultV1>;controller:AbortController;refs:number};
const flights=new Map<string,FlightV1>(),cooling=new Map<string,number>(),gib=(bytes:number):string=>(bytes/GIB).toFixed(1);

async function runFlight(options:RustImageReclaimRequestV1,signal:AbortSignal):Promise<RustImageReclaimResultV1>{
 const measure=options.measure??measureVolumeV1,now=options.now??Date.now,sleep=options.sleep??sleepV1,claimOf=options.claim??tryClaimWithBallastV1,{language}=options,store=dirname(options.root);
 const read=():VolumeFactsV1=>measure(options.root,options.capacityBytes),text=(kind:keyof typeof messages,facts:Readonly<Record<string,string|number>>)=>autoReclaimTextV1(kind,{soft:gib(rule.softAvailableBytes),target:gib(rule.targetAvailableBytes),...facts},language);
 const first=read();
 let rule:RustImageReclaimRuleV1=typeof options.rule==="function"?{softAvailableBytes:0,targetAvailableBytes:1}:options.rule;
 if(!first.unmeasured&&typeof options.rule==="function")rule=options.rule(first.volumeBytes);
 if(first.unmeasured||!autoReclaimDueV1(first.freeBytes,rule))return{outcome:"not-due",availableBytes:first.freeBytes,freedBytes:0};
 const key=options.root,cooled=cooling.get(key);
 if(cooled!==undefined&&now()-cooled<policy.cooldownMilliseconds)return{outcome:"cooling",availableBytes:first.freeBytes,freedBytes:0};
 options.report(text("due",{free:gib(first.freeBytes)}));
 let claim:ClaimV1|undefined,announced=false;
 try{
  for(;;){
   signal.throwIfAborted();
   claim=claimOf(join(options.root,layout.locks),policy.claim);
   if(claim)break;
   if(!announced){announced=true;options.report(text("joined",{free:gib(read().freeBytes)}));}
   await sleep(policy.joinPollMilliseconds,signal);
   const meanwhile=read();
   if(!autoReclaimDueV1(meanwhile.freeBytes,rule))return{outcome:"joined",availableBytes:meanwhile.freeBytes,freedBytes:0};
  }
  const before=read();
  if(!autoReclaimDueV1(before.freeBytes,rule))return{outcome:announced?"joined":"not-due",availableBytes:before.freeBytes,freedBytes:0};
  let lastProgress=-Infinity;
  const progress=(stage:string,done:number,total:number)=>{const at=now();if(at-lastProgress<policy.progressMilliseconds)return;lastProgress=at;options.report(text("progress",{stage,done,total}));};
  const demandBytes=autoReclaimDemandBytesV1(before.freeBytes,rule);
  await(options.collect??defaultCollect(options))({store,demandBytes,signal,language,progress,report:options.report,lock:options.lock,leases:options.leases,live:options.live});
  try{await(options.kick??defaultKick(options))({signal});}catch(error){signal.throwIfAborted();void error;}
  const after=read(),freedBytes=Math.max(0,after.freeBytes-before.freeBytes);
  if(freedBytes>0){cooling.delete(key);options.report(text("done",{freed:gib(freedBytes),free:gib(after.freeBytes)}));}
  else{cooling.set(key,now());options.report(text("nothing",{free:gib(after.freeBytes)}));}
  return{outcome:freedBytes>0?"reclaimed":"nothing",availableBytes:after.freeBytes,freedBytes};
 }catch(error){
  if(signal.aborted)options.report(text("cancelled",{}));
  throw error;
 }finally{claim?.release();}
}

/** 👟️ Returns the blocks the collection freed inside the mounted image to the host while builds still run: the host free space, which caps the honest available space, only grows once the image gives its freed blocks back. A root that is not inside a mounted image, a platform without an in-place return (Windows) and any failure leave the collection as it was.
 * @see ../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/👟️kick/🟦️.ts */
const warned=new Set<string>();
function defaultKick(options:RustImageReclaimRequestV1):NonNullable<RustImageReclaimRequestV1["kick"]>{
 return async request=>{
  for(let at=options.root,parent=dirname(at);parent!==at;at=parent,parent=dirname(at)){
   const image=join(parent,nativeImageBackingFilenameV1());
   if(!existsSync(image)||nativeImageRootV1(parent)!==at)continue;
   const run=(program:string,args:readonly string[])=>new Promise<string>((resolve,reject)=>execFile(program,[...args],{env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"},maxBuffer:1048576,signal:request.signal},(error,stdout)=>error?reject(error):resolve(stdout)));
   const port=createNativeImageKickPortV1(process.platform,{image,mountPoint:at,run});
   if(port!==null){
    const result=await kickNativeImageV1(port,{check:()=>request.signal.throwIfAborted()});
    if(!result.effective&&!warned.has(image)){warned.add(image);const values={image,returned:gib(result.canaryReturnedBytes??0)+" GiB",written:gib(result.canaryBytes??0)+" GiB"};options.report(nativeImageKickIneffectiveTextV1(options.language,values));}
   }
   return;
  }
 };
}

function defaultCollect(options:RustImageReclaimRequestV1):NonNullable<RustImageReclaimRequestV1["collect"]>{
 const measure=options.measure??measureVolumeV1;
 return request=>collectRustImageV1(request.store,{dryRun:false,language:request.language,demandBytes:request.demandBytes,quotaBytes:0,unitBudgetBytes:unitBudgetBytesV1(options.capacityBytes),measure:()=>measure(options.root,options.capacityBytes),lock:request.lock,leases:request.leases,live:request.live,write:request.report,control:collector=>({signal:request.signal,progress:(done,total,phase)=>request.progress(collector+":"+phase,done,total)})});
}

/** 🚰️ Reclaims the Rust image once when its available space fell below the soft threshold of the ledger. Single-flight everywhere: concurrent requests in this process share one reclaim, and across agents the cross-process claim `repo cache gc` takes too lets one reclaim run while the others wait for it and stop as soon as the space is back above the threshold. Collection is lock-aware (never a unit a running build holds), reports bilingual progress, and cancels only when every requester has cancelled. A reclaim that freed nothing cools for a few seconds instead of rescanning at every admission. A request whose root is not a unit cache root `u` is not due. */
export function requestRustImageReclaimV1(options:RustImageReclaimRequestV1):Promise<RustImageReclaimResultV1>{
 if(basename(options.root)!=="u")return Promise.resolve({outcome:"not-due",availableBytes:0,freedBytes:0});
 const key=options.root,signal=options.signal;
 let flight=flights.get(key);
 if(flight)options.report(autoReclaimTextV1("joined",{free:gib((options.measure??measureVolumeV1)(options.root,options.capacityBytes).freeBytes)},options.language));
 else{
  const controller=new AbortController(),created:FlightV1={controller,refs:0,promise:undefined as never};
  created.promise=runFlight(options,controller.signal).finally(()=>{if(flights.get(key)===created)flights.delete(key);});
  flights.set(key,created);flight=created;
 }
 const mine=flight;mine.refs++;
 return new Promise((resolve,reject)=>{
  let settled=false;
  const leave=()=>{settled=true;signal?.removeEventListener("abort",abort);if(--mine.refs===0)mine.controller.abort(new DOMException("reclaim cancelled","AbortError"));};
  const abort=()=>{if(!settled){leave();reject(signal!.reason);}};
  if(signal?.aborted){abort();return;}
  signal?.addEventListener("abort",abort,{once:true});
  mine.promise.then(value=>{if(!settled){leave();resolve(value);}},error=>{if(!settled){leave();reject(error);}});
 });
}

