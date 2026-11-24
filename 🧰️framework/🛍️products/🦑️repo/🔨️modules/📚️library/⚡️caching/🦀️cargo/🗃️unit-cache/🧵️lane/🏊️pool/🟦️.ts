import {randomBytes} from "node:crypto";
import {mkdirSync} from "node:fs";
import {join} from "node:path";
import {claimHeldV1,claimHolderV1,sleepV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";
import {crateKeyV1,enterLaneV1,laneIdsV1,laneLiveV1,lanePathsV1,laneRecentV1,laneRetainedV1,laneUsedMsV1,readLaneSizeV1,readLaneV1,removeLaneV1,type LaneOwnerV1} from "../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.pool,layout=UNIT_CACHE_POLICY_V1.layout;
/** 🏊️ One lane as the pool sees it: whether a live process holds it, when it was last used, the crate set it was last built for (`""` when unknown) and its recorded size without its Cargo incremental sessions (`null` until the collector measured it): the sessions are bounded by the collector's own incremental share, so a lane that keeps a crate's incremental state warm is not evicted for holding it. */
export type PoolLaneV1=Readonly<{id:string;claimed:boolean;lastUsedMs:number;crateKey:string;bytes:number|null;product?:boolean;retained?:boolean;held?:boolean;recent?:boolean;flavour?:string}>;
/** 📐️ The bounds of the pool: the idle lanes are trimmed to `maximumLanes` lanes and `quotaBytes` recorded bytes, live lanes may exceed them up to `maximumLiveLanes`. */
export type PoolBoundsV1=Readonly<{maximumLanes:number;maximumLiveLanes:number;quotaBytes:number}>;
/** 🎯️ What an acquisition does: take the newest free lane of the same crate set (`warm`), start a lane (`new`), take the least recently used free lane (`lru`) or wait (`busy`). */
export type PoolChoiceV1=Readonly<{kind:"warm"|"lru"|"new"|"busy";id?:string}>;

const newest=(left:PoolLaneV1,right:PoolLaneV1):number=>right.lastUsedMs-left.lastUsedMs||(left.id<right.id?-1:left.id>right.id?1:0);
const oldest=(left:PoolLaneV1,right:PoolLaneV1):number=>left.lastUsedMs-right.lastUsedMs||(left.id<right.id?-1:left.id>right.id?1:0);
const recorded=(lanes:readonly PoolLaneV1[]):number=>lanes.reduce((sum,lane)=>sum+(lane.bytes??0),0);

/** 💰️ The recorded bytes the idle lanes of the pool may hold: the lane share of the smaller of the image capacity and the machine budget's quota of the compiler store (`null` when no budget is known). */
export function poolQuotaBytesV1(capacityBytes:number,machineStoreQuotaBytes:number|null):number{return Math.floor(Math.min(capacityBytes,machineStoreQuotaBytes??capacityBytes)*UNIT_CACHE_POLICY_V1.gc.laneShare);}

/** 🌡️ What a trim knows about the build that is starting: the lanes it requests (never victims) and the image it runs on (`volume`, whose free space decides whether recent lanes may be shed). */
export type PoolTrimContextV1=Readonly<{requested?:readonly string[];flavour?:string;volume?:()=>Readonly<{volumeBytes:number;freeBytes:number}>}>;

/** 📦️ The bytes the lanes of the largest declared build flavour need: the recorded bytes of the lanes of each flavour summed, the greatest sum. The pool's byte bound is never below it. Pure. */
export function poolNeedBytesV1(lanes:readonly PoolLaneV1[]):number{
 const sums=new Map<string,number>();for(const lane of lanes)if(lane.flavour!==undefined)sums.set(lane.flavour,(sums.get(lane.flavour)??0)+(lane.bytes??0));
 return Math.max(0,...sums.values());
}

/** 🪣️ The bounds of the pool for a byte quota. */
export function poolBoundsV1(quotaBytes:number):PoolBoundsV1{return Object.freeze({maximumLanes:rules.maximumLanes,maximumLiveLanes:rules.maximumLiveLanes,quotaBytes});}

/** 🛡️ The lanes the pool bounds apply to: a retained product lane (see `laneRetainedV1`) is neither counted against the lane and byte bounds nor ever a victim of a trim. Pure. */
export function boundedPoolV1(lanes:readonly PoolLaneV1[]):PoolLaneV1[]{return lanes.filter(lane=>!lane.retained);}

/** 🧭️ Chooses the lane of one acquisition. Pure and deterministic. The newest free lane built for the same crate set keeps its incremental state warm, a lane held for a restart (see `holdLaneV1`) included; a held lane is otherwise neither counted against the bounds nor taken as the least recently used lane; otherwise the pool grows while it is inside its bounds, else the least recently used free lane is reused, else (every lane live) the pool grows beyond its bounds up to the live maximum and only then waits. Waiting holds nothing, so no wait chain exists. */
export function planPoolAcquireV1(lanes:readonly PoolLaneV1[],request:Readonly<{crateKey:string}>,bounds:PoolBoundsV1):PoolChoiceV1{
 const free=lanes.filter(lane=>!lane.claimed&&!lane.product),warm=free.filter(lane=>lane.crateKey===request.crateKey).sort(newest)[0];
 if(warm)return{kind:"warm",id:warm.id};
 const counted=lanes.filter(lane=>lane.held!==true);
 if(counted.length<bounds.maximumLanes&&recorded(counted)<=bounds.quotaBytes)return{kind:"new"};
 const cold=free.filter(lane=>lane.held!==true).sort(oldest)[0];
 if(cold)return{kind:"lru",id:cold.id};
 return counted.length<bounds.maximumLiveLanes?{kind:"new"}:{kind:"busy"};
}

/** ✂️ Chooses the idle lanes to remove so the pool is back inside its bounds: least recently used first, never a claimed lane, never a requested lane, and a recent lane of a declared flavour only under free-space `pressure`. Pure; applying the result and planning again removes nothing. */
export function planPoolTrimV1(lanes:readonly PoolLaneV1[],bounds:PoolBoundsV1,context:Readonly<{requested?:readonly string[];pressure?:boolean}>={}):string[]{
 const victims:string[]=[],requested=new Set(context.requested??[]);let count=lanes.length,bytes=recorded(lanes);
 for(const lane of lanes.filter(row=>!row.claimed&&!requested.has(row.id)&&(context.pressure===true||row.recent!==true)).sort(oldest)){
  if(count<=bounds.maximumLanes&&bytes<=bounds.quotaBytes)break;
  victims.push(lane.id);count--;bytes-=lane.bytes??0;
 }
 return victims;
}

/** 🔭️ Reads the pool of a cache root (the reserved lanes, the static default lane of `.cargo/config.toml` that unattributed commands use, are not part of it): claim and live markers say whether a lane is held, the metadata says what it was last built for, the size file what the collector measured. */
export function readPoolV1(root:string,nowMs:number):PoolLaneV1[]{
 const locks=join(root,layout.locks);
 return laneIdsV1(root).filter(id=>!rules.reserved.includes(id)).map(id=>{
  const paths=lanePathsV1(root,id),lastUsedMs=laneUsedMsV1(root,id),flavour=readLaneV1(root,id)?.flavour;
  const holdUntilMs=readLaneV1(root,id)?.holdUntilMs,held=holdUntilMs!==undefined&&nowMs<holdUntilMs&&!id.startsWith(rules.product.prefix);
  return Object.freeze({id,product:id.startsWith(rules.product.prefix),retained:laneRetainedV1(id,lastUsedMs,nowMs,holdUntilMs),held,recent:laneRecentV1(flavour,lastUsedMs,nowMs),...flavour===undefined?{}:{flavour},claimed:claimHeldV1(locks,"lane-"+id)||laneLiveV1(paths,nowMs),lastUsedMs,crateKey:readLaneV1(root,id)?.crateKey??"",bytes:(size=>size?size.bytes-size.incrementalBytes:null)(readLaneSizeV1(root,id))});
 });
}

/** 🪪️ Names who holds the given lanes for a wait message: process id, agent and session of each claimed lane (at most `listedHolders`, the rest counted), `nobody readable` when no claim names a holder. */
export function laneHoldersV1(root:string,ids:readonly string[],nowMs:number):string{
 const locks=join(root,layout.locks),named=ids.flatMap(id=>{const holder=claimHolderV1(locks,"lane-"+id);if(!holder)return[];const lane=readLaneV1(root,id);return[`${id}: pid ${holder.pid}${lane?.agent?`, agent ${lane.agent}`:""}${lane?.session?`, session ${lane.session}`:""}, ${Math.max(0,Math.round((nowMs-holder.atMs)/60000))} min`];});
 return named.length===0?"nobody readable":named.slice(0,rules.listedHolders).join("; ")+(named.length>rules.listedHolders?`; and ${named.length-rules.listedHolders} more`:"");
}

/** 🧹️ Removes the idle lanes the pool must shed, a lane the starting build requests never among them. The byte bound is raised to what the largest declared build flavour needs; when the image cannot hold that need below its high-water mark the pool says so and removes nothing; recent lanes of a declared flavour go only while the image is under free-space pressure. Every removal takes the lane's own claim first, so a lane that was taken since the plan survives, and publishes authenticated binding evidence and every owned receipt before the lane goes; unresolved custody keeps it; returns what was removed. */
export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1,context:PoolTrimContextV1={}):Readonly<{removed:number;freedBytes:number}>{
 const lanes=boundedPoolV1(readPoolV1(root,nowMs)),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0])),need=poolNeedBytesV1(lanes),volume=context.volume?.(),high=volume===undefined?Infinity:Math.floor(volume.volumeBytes*UNIT_CACHE_POLICY_V1.gc.highWaterShare);let removed=0,freedBytes=0;
 if(need>high){report?.("poolCannotHold",{flavour:context.flavour??"",needMib:Math.round(need/1048576),imageMib:Math.round(high/1048576)});return Object.freeze({removed,freedBytes});}
 const pressure=volume!==undefined&&volume.volumeBytes-volume.freeBytes>high,requested=[...context.requested??[],...lanes.filter(lane=>context.flavour!==undefined&&lane.flavour===context.flavour&&lane.recent===true).map(lane=>lane.id)];
 const retain=(paths:ReturnType<typeof lanePathsV1>):boolean=>{const conflicts=retainRemovedLaneV1(paths);if(conflicts.length)report?.("retentionConflict",{lane:paths.id,count:conflicts.length,names:conflicts.slice(0,4).join(", ")});return conflicts.length===0;};
 for(const id of planPoolTrimV1(lanes,{...bounds,quotaBytes:Math.max(bounds.quotaBytes,need)},{requested,pressure}))if(removeLaneV1(root,id,"lane",nowMs,retain)){removed++;freedBytes+=size.get(id)!;}
 return Object.freeze({removed,freedBytes});
}

/** 📣️ What the pool tells its caller, to be rendered in the caller's language. */
export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|"poolCannotHold"|"productLaneHeld"|"retentionConflict",parameters:Readonly<Record<string,string|number>>)=>void;
/** 🎒️ A lane taken from the pool: its directories, how it was chosen and the release that frees it. */
export type PooledLaneV1=Readonly<{paths:ReturnType<typeof lanePathsV1>;choice:"warm"|"lru"|"new";release():void}>;

/** 🔐️ Takes a lane for one Cargo process: exactly one process holds a lane (its claim, a file created exclusively), so no two Cargo processes ever share a build directory and Cargo's own unit locks can never form a cycle between processes. The lane of the same crate set is preferred, so its incremental state stays warm; afterwards the idle lanes are trimmed to the pool bounds. Without a free lane and below the live maximum the pool grows, at the live maximum it waits by polling, holding nothing.
 * @see ../../🧹️gc/🟦️.ts */
export async function acquirePooledLaneV1(root:string,request:Readonly<{crates:readonly string[];quotaBytes:number;owner?:LaneOwnerV1;flavour?:string}>,options:Readonly<{signal?:AbortSignal;now?:()=>number;report?:PoolReportV1;volume?:PoolTrimContextV1["volume"]}>={}):Promise<PooledLaneV1>{
 const locks=join(root,layout.locks),bounds=poolBoundsV1(request.quotaBytes),key=crateKeyV1(request.crates),now=options.now??Date.now;
 mkdirSync(join(root,layout.lanes),{recursive:true});
 let poll=rules.pollMilliseconds,reported=false;
 for(;;){
  options.signal?.throwIfAborted();
  const catalog=tryClaimV1(locks,rules.catalogClaim);
  if(!catalog){await sleepV1(poll,options.signal);continue;}
  let acquired:PooledLaneV1|undefined;
  try{acquired=(()=>{
  const pool=readPoolV1(root,now()),lanes=[...boundedPoolV1(pool),...pool.filter(lane=>lane.held===true)],choice=planPoolAcquireV1(lanes,{crateKey:key},bounds);
  if(choice.kind==="busy"){
   if(!reported){reported=true;options.report?.("poolBusy",{live:lanes.length,maximum:bounds.maximumLiveLanes,holders:laneHoldersV1(root,lanes.map(lane=>lane.id),now())});}
   return undefined;
  }
  const id=choice.id??"p"+randomBytes(3).toString("hex"),claim=tryClaimV1(locks,"lane-"+id);
  if(!claim)return undefined;
  let entered:ReturnType<typeof enterLaneV1>;try{entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);}catch(error){claim.release();throw error;}
  if(choice.kind==="new"&&lanes.filter(lane=>lane.held!==true).length>=bounds.maximumLanes)options.report?.("overflow",{busy:lanes.filter(lane=>lane.claimed).length,total:lanes.filter(lane=>lane.held!==true).length,lane:id});
  try{const trimmed=trimPoolV1(root,bounds,now(),options.report,{requested:[id],...request.flavour===undefined?{}:{flavour:request.flavour},...options.volume===undefined?{}:{volume:options.volume}});if(trimmed.removed>0)options.report?.("trimmed",{count:readPoolV1(root,now()).length,quotaMib:Math.round(bounds.quotaBytes/1048576),removed:trimmed.removed});}catch{}
  return Object.freeze({paths:entered.paths,choice:choice.kind,release(){try{entered.release();}finally{claim.release();}}});
  })();}finally{catalog.release();}
  if(acquired)return acquired;
  await sleepV1(poll,options.signal);poll=Math.min(rules.pollMaximumMilliseconds,Math.ceil(poll*rules.pollBackoffBasisPoints/10000));
 }
}
