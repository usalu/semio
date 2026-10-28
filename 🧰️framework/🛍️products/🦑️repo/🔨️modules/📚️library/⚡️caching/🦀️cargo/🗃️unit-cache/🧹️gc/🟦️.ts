import {readdirSync,rmSync,statSync,truncateSync,writeFileSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {listObjectsV1,pruneEntriesV1,removeObjectsV1} from "../💾️store/🟦️.ts";
import {listLanesV1,removeLaneV1} from "../🧵️lane/🟦️.ts";
import {tryClaimV1} from "../🔒️lock/🟦️.ts";
import {collectIncrementalV1,type IncrementalReportV1} from "./🌱️incremental/🟦️.ts";
import {collectUnitsV1,measureUnitsV1,unitProfilesV1} from "./🧱️units/🟦️.ts";
import {recordFillSampleV1} from "./📈️fill/🟦️.ts";
import {pruneAdmissionsV1} from "./📖️admissions/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.gc,layout=UNIT_CACHE_POLICY_V1.layout,objectBatch=64;
/** 🧮️ One measured lane. */
export type GcLaneV1=Readonly<{id:string;bytes:number;incrementalBytes:number;lastUsedMs:number;live:boolean}>;
/** 📦️ One cached object. */
export type GcObjectV1=Readonly<{digest:string;bytes:number;lastUsedMs:number}>;
/** 📥️ The measured state of one cache root: `capacityBytes` is the share basis of its parts, `volumeBytes` and `freeBytes` are the image the root lives in. */
export type GcInputV1=Readonly<{capacityBytes:number;volumeBytes:number;freeBytes:number;nowMs:number;lanes:readonly GcLaneV1[];objects:readonly GcObjectV1[]}>;
/** 🚦️ What a command that just looked at the image's free space does about the unit cache. */
export type GcPressureV1="enough"|"maintain"|"watch"|"launch"|"throttled"|"collect-now";
/** 📋️ What to remove, nothing else. */
export type GcPlanV1=Readonly<{removeLanes:string[];removeIncremental:string[];removeObjects:string[]}>;

const newestFirst=<T extends {lastUsedMs:number}>(key:(row:T)=>string)=>(left:T,right:T)=>right.lastUsedMs-left.lastUsedMs||(key(left)<key(right)?-1:key(left)>key(right)?1:0);
const oldestFirst=<T extends {lastUsedMs:number}>(key:(row:T)=>string)=>(left:T,right:T)=>left.lastUsedMs-right.lastUsedMs||(key(left)<key(right)?-1:key(left)>key(right)?1:0);

/** 🌊️ The bytes a collection must free: nothing at or below the high-water mark of the image, everything above the low-water mark once it is crossed. */
export function gcNeedBytesV1(volumeBytes:number,freeBytes:number,rules:Pick<typeof policy,"highWaterShare"|"lowWaterShare">=policy):number{
 const used=Math.max(0,volumeBytes-freeBytes);
 return used>Math.floor(volumeBytes*rules.highWaterShare)?used-Math.floor(volumeBytes*rules.lowWaterShare):0;
}

/** ⚖️ Pure rule: free space at or above the target (the low-water mark) needs only the periodic maintenance (at most once per maintenance interval); between the target and the floor (the high-water mark) the watching background collector is launched at most once per collector interval, so it is already alive when a running compile crosses the floor; below the floor the same collector is launched, and below what one compile must find free the command collects itself before it starts. */
export function gcPressureV1(input:Readonly<{volumeBytes:number;freeBytes:number;emergencyBytes:number;nowMs:number;lastLaunchMs:number|null}>,rules:Pick<typeof policy,"highWaterShare"|"lowWaterShare"|"collectorIntervalMilliseconds"|"maintenanceIntervalMilliseconds">=policy):GcPressureV1{
 const age=input.lastLaunchMs===null?Infinity:input.nowMs-input.lastLaunchMs;
 const used=input.volumeBytes-input.freeBytes;
 if(used<=Math.floor(input.volumeBytes*rules.lowWaterShare))return age>=rules.maintenanceIntervalMilliseconds?"maintain":"enough";
 if(used<=Math.floor(input.volumeBytes*rules.highWaterShare))return age<rules.collectorIntervalMilliseconds?"throttled":"watch";
 if(input.freeBytes<input.emergencyBytes)return "collect-now";
 return age<rules.collectorIntervalMilliseconds?"throttled":"launch";
}

/** 🛑️ Whether compiler output says the image ran out of space: the operating system's own error number, never a translated sentence alone. */
export function gcOutOfSpaceV1(text:string,patterns:readonly string[]=policy.outOfSpacePatterns):boolean{return patterns.some(pattern=>new RegExp(pattern,"u").test(text));}

/** 🧹️ Plans the bounded cleanup of one cache root. Pure and deterministic.
 * Order: idle lanes; free-space pressure above the high-water mark down to the low-water mark of the image (incremental directories, then lanes, then objects, oldest first); the lane, incremental and object shares of the capacity. Live lanes are never touched and count against their share first. */
export function planGcV1(input:GcInputV1,override:Partial<typeof policy>={}):GcPlanV1{
 const rules={...policy,...override},cap=(share:number)=>Math.floor(input.capacityBytes*share);
 const removeLanes=new Set<string>(),removeIncremental=new Set<string>(),removeObjects=new Set<string>();
 const lanes=[...input.lanes].sort(oldestFirst(row=>row.id)),idle=lanes.filter(row=>!row.live);
 for(const lane of idle)if(input.nowMs-lane.lastUsedMs>rules.laneIdleMilliseconds)removeLanes.add(lane.id);
 const laneBytes=(lane:GcLaneV1)=>removeLanes.has(lane.id)?0:lane.bytes-(removeIncremental.has(lane.id)?lane.incrementalBytes:0);
 const objects=[...input.objects].sort(oldestFirst(row=>row.digest));
 let need=gcNeedBytesV1(input.volumeBytes,input.freeBytes,rules);
 const freed=()=>input.lanes.reduce((sum,lane)=>sum+lane.bytes-laneBytes(lane),0)+objects.filter(row=>removeObjects.has(row.digest)).reduce((sum,row)=>sum+row.bytes,0);
 if(need>0){
  for(const lane of idle){if(freed()>=need)break;if(!removeLanes.has(lane.id)&&!removeIncremental.has(lane.id)&&lane.incrementalBytes>0)removeIncremental.add(lane.id);}
  for(const lane of idle){if(freed()>=need)break;removeLanes.add(lane.id);}
  const protectedDigests=new Set<string>();let protectedBytes=0;
  for(const object of [...objects].reverse()){protectedBytes+=object.bytes;if(protectedBytes>cap(rules.casProtectedShare))break;protectedDigests.add(object.digest);}
  for(const object of objects){if(freed()>=need)break;if(!protectedDigests.has(object.digest))removeObjects.add(object.digest);}
 }
 const keepPrefix=<T extends {lastUsedMs:number}>(rows:readonly T[],bytes:(row:T)=>number,budget:number,key:(row:T)=>string):T[]=>{
  const removed:T[]=[];let sum=0,overflow=false;
  for(const row of [...rows].sort(newestFirst(key))){sum+=bytes(row);if(overflow||sum>budget){overflow=true;removed.push(row);}}
  return removed;
 };
 const liveBytes=input.lanes.filter(row=>row.live).reduce((sum,row)=>sum+row.bytes,0);
 for(const lane of keepPrefix(idle.filter(row=>!removeLanes.has(row.id)),laneBytes,Math.max(0,cap(rules.laneShare)-liveBytes),row=>row.id))removeLanes.add(lane.id);
 const liveIncremental=input.lanes.filter(row=>row.live).reduce((sum,row)=>sum+row.incrementalBytes,0);
 const candidates=idle.filter(row=>!removeLanes.has(row.id));
 for(const lane of keepPrefix(candidates,row=>removeIncremental.has(row.id)?0:row.incrementalBytes,Math.max(0,cap(rules.incrementalShare)-liveIncremental),row=>row.id))if(lane.incrementalBytes>0)removeIncremental.add(lane.id);
 for(const object of keepPrefix(objects.filter(row=>!removeObjects.has(row.digest)),row=>row.bytes,cap(rules.casShare),row=>row.digest))removeObjects.add(object.digest);
 return Object.freeze({removeLanes:[...removeLanes].sort(),removeIncremental:[...removeIncremental].filter(id=>!removeLanes.has(id)).sort(),removeObjects:[...removeObjects].sort()});
}

/** 🔬️ The size and free bytes of the image a cache root lives in. */
export type VolumeFactsV1=Readonly<{volumeBytes:number;freeBytes:number;unmeasured?:true}>;
/** 🔭️ The size of the image a cache root lives in and its free bytes. A root whose volume cannot be read is unknown: it is reported as `unmeasured:true` and, so that no collection starts on a guess, counts as empty and as large as its capacity. */
export function measureVolumeV1(root:string,capacityBytes:number):VolumeFactsV1{
 const space=nativeFreeSpaceSyncV1(root);
 if(space?.totalBytes===undefined)return{volumeBytes:capacityBytes,freeBytes:capacityBytes,unmeasured:true};
 return{volumeBytes:Math.max(1,space.totalBytes),freeBytes:Math.min(space.totalBytes,space.availableBytes)};
}

/** 📏️ Measures a cache root on disk. `capacityBytes` is the share basis of its parts (the image minus one compile workspace), the pressure comes from the image itself. */
export function measureGcV1(root:string,capacityBytes:number,nowMs:number,measure:(root:string,capacityBytes:number)=>VolumeFactsV1=measureVolumeV1):GcInputV1{
 return{capacityBytes,...measure(root,capacityBytes),nowMs,lanes:listLanesV1(root,nowMs),objects:listObjectsV1(root)};
}

/** 🗓️ Applies a plan. Live lanes are re-checked at removal time, so a lane that became live since measuring survives. Returns what was removed. */
export function applyGcV1(root:string,plan:GcPlanV1,nowMs:number):{lanes:number;incremental:number;objects:number}{
 let lanes=0,incremental=0;
 for(const id of plan.removeLanes)if(removeLaneV1(root,id,"lane",nowMs))lanes++;
 for(const id of plan.removeIncremental)if(removeLaneV1(root,id,"incremental",nowMs))incremental++;
 const objects=removeObjectsV1(root,plan.removeObjects);
 pruneEntriesV1(root,nowMs);pruneEvents(root);
 return{lanes,incremental,objects};
}

/** 🔁️ Applies a plan item by item and lets the event loop run between items, so the collector's claim keeps beating and a cancellation or progress report is served during a long collection. */
export async function applyGcYieldingV1(root:string,plan:GcPlanV1,nowMs:number,options:Readonly<{signal?:AbortSignal;progress?:(done:number,total:number)=>void}>={}):Promise<{lanes:number;incremental:number;objects:number}>{
 const turn=()=>new Promise<void>(done=>setImmediate(done)),chunks=Math.ceil(plan.removeObjects.length/objectBatch),total=plan.removeLanes.length+plan.removeIncremental.length+chunks;let lanes=0,incremental=0,objects=0,done=0;
 const guard=():void=>options.signal?.throwIfAborted(),step=async()=>{options.progress?.(++done,total);await turn();};
 for(const id of plan.removeLanes){guard();if(removeLaneV1(root,id,"lane",nowMs))lanes++;await step();}
 for(const id of plan.removeIncremental){guard();if(removeLaneV1(root,id,"incremental",nowMs))incremental++;await step();}
 for(let index=0;index<plan.removeObjects.length;index+=objectBatch){guard();objects+=removeObjectsV1(root,plan.removeObjects.slice(index,index+objectBatch));await step();}
 pruneEntriesV1(root,nowMs);pruneEvents(root);
 return{lanes,incremental,objects};
}

/** 🌱️ What a collection removed: lanes, lane incremental sets, objects, the incremental sessions of the build and target directories and the stale build units. */
export type GcAppliedV1={lanes:number;incremental:number;objects:number;sessions:number;units:number};

/** 🚰️ Collects once at a time: a second collector finds the claim taken and reports `running` instead of racing the first. The incremental sessions of every build, target and lane directory of the image go first (lock-aware, least recently used first, they regrow within hours), then lanes and objects are planned on what the image looks like afterwards. */
export async function collectGarbageSingleFlightV1(root:string,capacityBytes:number,options:Readonly<{signal?:AbortSignal;progress?:(done:number,total:number)=>void;nowMs?:number;report?:IncrementalReportV1;volume?:(root:string,capacityBytes:number)=>VolumeFactsV1}>={}):Promise<{running:true}|{running:false;plan:GcPlanV1;applied:GcAppliedV1}>{
 const measure=options.volume??measureVolumeV1,claim=tryClaimV1(join(root,layout.locks),policy.collectorClaim);if(!claim)return{running:true};
 try{
  const nowMs=options.nowMs??Date.now();let sessions=0,units=0;
  if(basename(root)==="u"){
   const before=measure(root,capacityBytes),need=gcNeedBytesV1(before.volumeBytes,before.freeBytes);
   const result=await collectIncrementalV1(dirname(root),{dryRun:false,volumeBytes:before.volumeBytes,needBytes:()=>need,satisfied:()=>need>0&&measure(root,capacityBytes).freeBytes-before.freeBytes>=need,nowMs,signal:options.signal,report:options.report});
   sessions=result.removed.length;
   const middle=measure(root,capacityBytes),remaining=gcNeedBytesV1(middle.volumeBytes,middle.freeBytes),stamp=join(root,policy.unitStamp);let last=0;try{last=statSync(stamp).mtimeMs;}catch{}
   if(remaining>0||nowMs-last>=policy.unitIntervalMilliseconds){
    writeFileSync(stamp,String(nowMs));
    const result=await collectUnitsV1(dirname(root),{dryRun:false,needBytes:()=>remaining,satisfied:()=>remaining>0&&measure(root,capacityBytes).freeBytes-middle.freeBytes>=remaining,nowMs,signal:options.signal,report:options.report}),after=measure(root,capacityBytes);
    units=result.removed.length;recordFillSampleV1(root,{atMs:nowMs,freeBytes:after.freeBytes,volumeBytes:after.volumeBytes,profiles:result.profiles});
   }
  }
  const plan=planGcV1(measureGcV1(root,capacityBytes,nowMs,measure)),applied={...await applyGcYieldingV1(root,plan,nowMs,options),sessions,units};
  await pruneAdmissionsV1(root,nowMs,{retentionMs:policy.admissionRetentionMilliseconds,temporaryMs:policy.admissionTemporaryMilliseconds,maximumBytes:Math.floor(capacityBytes*policy.admissionShare)},options.signal);
  return{running:false,plan,applied};
 }finally{claim.release();}
}

/** 👁️ How a watch ended: another watcher owns the cache root, or this one saw the image quiet for the quiet period, ran out of its lifetime, or kept collecting without freeing anything. */
export type GcWatchV1=Readonly<{state:"running"}|{state:"done";reason:"quiet"|"expired"|"stalled"|"cancelled";rounds:number;collections:number;freedBytes:number}>;

/** ⌚️ Keeps the unit cache relieved while compiles run: one watcher per cache root (its claim), polling the image every `watchPollMilliseconds`; below the floor it collects (the same single-flight collection as the one-shot collector), between floor and target it maintains (the share-bounded collection) once at its start and then every maintenance interval, at or above the target for the quiet period it ends; it also ends after its maximum lifetime and after `watchStalledRounds` collections in a row that freed nothing (running compilers hold the space), which the next compile launch restarts. Every poll appends a sample to the fill telemetry. Never throws except for cancellation. */
export async function watchUnitCachePressureV1(root:string,capacityBytes:number,options:Readonly<{signal?:AbortSignal;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;measure?:typeof measureVolumeV1;collect?:typeof collectGarbageSingleFlightV1;profiles?:()=>Promise<Readonly<Record<string,number>>>;report?:IncrementalReportV1}>={}):Promise<GcWatchV1>{
 const claim=tryClaimV1(join(root,layout.locks),policy.watchClaim);if(!claim)return{state:"running"};
 const now=options.now??Date.now,sleep=options.sleep??((milliseconds:number,signal?:AbortSignal)=>new Promise<void>((done,fail)=>{const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);fail(signal.reason);},{once:true});})),measure=options.measure??measureVolumeV1,collect=options.collect??collectGarbageSingleFlightV1,started=now();
 const scan=options.profiles??(async()=>{const store=dirname(root),totals:Record<string,number>={};for(const unit of await measureUnitsV1(unitProfilesV1(store),{nowMs:now(),busy:()=>true,signal:options.signal}))totals[dirname(dirname(dirname(unit.path))).slice(store.length+1)]=(totals[dirname(dirname(dirname(unit.path))).slice(store.length+1)]??0)+unit.bytes;return totals;});
 let rounds=0,collections=0,freedBytes=0,idle=0,quietSince:number|null=null,scanned=-Infinity,maintained=-Infinity;
 try{
  for(;;){
   options.signal?.throwIfAborted();rounds++;
   const before=measure(root,capacityBytes),at=now(),due=at-scanned>=policy.watchScanMilliseconds,profiles=due?await scan().catch(()=>undefined):undefined;if(due)scanned=at;recordFillSampleV1(root,{atMs:at,freeBytes:before.freeBytes,volumeBytes:before.volumeBytes,...profiles?{profiles}:{}});
   const need=gcNeedBytesV1(before.volumeBytes,before.freeBytes),target=before.volumeBytes-Math.floor(before.volumeBytes*policy.lowWaterShare);
   if(need>0){
    quietSince=null;const result=await collect(root,capacityBytes,{nowMs:at,signal:options.signal,report:options.report});collections++;
    const after=measure(root,capacityBytes),gained=Math.max(0,after.freeBytes-before.freeBytes);freedBytes+=gained;idle=!result.running&&result.applied.lanes+result.applied.incremental+result.applied.sessions+result.applied.units+result.applied.objects>0?0:idle+1;
    if(idle>=policy.watchStalledRounds)return{state:"done",reason:"stalled",rounds,collections,freedBytes};
   }else if(before.freeBytes>=target){
    idle=0;quietSince??=at;if(at-quietSince>=policy.watchQuietMilliseconds)return{state:"done",reason:"quiet",rounds,collections,freedBytes};
   }else{
    idle=0;quietSince=null;
    if(at-maintained>=policy.maintenanceIntervalMilliseconds){maintained=at;await collect(root,capacityBytes,{nowMs:at,signal:options.signal,report:options.report});collections++;}
   }
   if(now()-started>=policy.watchMaximumMilliseconds)return{state:"done",reason:"expired",rounds,collections,freedBytes};
   await sleep(policy.watchPollMilliseconds,options.signal);
  }
 }catch(error){if(options.signal?.aborted)return{state:"done",reason:"cancelled",rounds,collections,freedBytes};throw error;}
 finally{claim.release();}
}

/** 🛟️ Frees at least `needBytes` of the image before a build's storage reservation would refuse: incremental sessions first, then stale build units, both lock-aware and least recently used first, one collector at a time (a second caller returns 0 and measures again). Returns the bytes the image gained. */
export async function reclaimUnitStorageV1(store:string,needBytes:number,options:Readonly<{signal?:AbortSignal;report?:IncrementalReportV1}>={}):Promise<number>{
 const root=join(store,"u"),claim=tryClaimV1(join(root,layout.locks),policy.collectorClaim);if(!claim)return 0;
 try{
  const before=measureVolumeV1(store,1);
  await collectIncrementalV1(store,{dryRun:false,volumeBytes:before.volumeBytes,needBytes:()=>needBytes,satisfied:()=>measureVolumeV1(store,1).freeBytes-before.freeBytes>=needBytes,signal:options.signal,report:options.report});
  const middle=measureVolumeV1(store,1).freeBytes-before.freeBytes;
  if(middle<needBytes)await collectUnitsV1(store,{dryRun:false,needBytes:()=>needBytes-middle,satisfied:()=>measureVolumeV1(store,1).freeBytes-before.freeBytes>=needBytes,signal:options.signal,report:options.report});
  return Math.max(0,measureVolumeV1(store,1).freeBytes-before.freeBytes);
 }finally{claim.release();}
}

function pruneEvents(root:string):void{
 const directory=join(root,UNIT_CACHE_POLICY_V1.layout.events);let names:string[];try{names=readdirSync(directory);}catch{return;}
 for(const name of names){try{const path=join(directory,name);if(statSync(path).size>policy.eventLogMaximumBytes)truncateSync(path,0);}catch{}}
}

/** 🪜️ Measures, plans and applies in one step. */
export function collectGarbageV1(root:string,capacityBytes:number,nowMs=Date.now()):{plan:GcPlanV1;applied:{lanes:number;incremental:number;objects:number}}{
 const plan=planGcV1(measureGcV1(root,capacityBytes,nowMs));
 return{plan,applied:applyGcV1(root,plan,nowMs)};
}

/** 🗑️ Removes the whole cache root. */
export function removeCacheRootV1(root:string):void{rmSync(root,{recursive:true,force:true});}
