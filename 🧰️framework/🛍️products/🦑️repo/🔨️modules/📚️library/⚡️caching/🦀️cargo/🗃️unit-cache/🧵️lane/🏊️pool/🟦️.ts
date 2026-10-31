import {randomBytes} from "node:crypto";
import {mkdirSync,statSync} from "node:fs";
import {join} from "node:path";
import {claimHeldV1,sleepV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";
import {crateKeyV1,enterLaneV1,laneIdsV1,laneLiveV1,lanePathsV1,readLaneSizeV1,readLaneV1,removeLaneV1,type LaneOwnerV1} from "../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.pool,layout=UNIT_CACHE_POLICY_V1.layout;
/** 🏊️ One lane as the pool sees it: whether a live process holds it, when it was last used, the crate set it was last built for (`""` when unknown) and its recorded size without its Cargo incremental sessions (`null` until the collector measured it): the sessions are bounded by the collector's own incremental share, so a lane that keeps a crate's incremental state warm is not evicted for holding it. */
export type PoolLaneV1=Readonly<{id:string;claimed:boolean;lastUsedMs:number;crateKey:string;bytes:number|null;product?:boolean}>;
/** 📐️ The bounds of the pool: the idle lanes are trimmed to `maximumLanes` lanes and `quotaBytes` recorded bytes, live lanes may exceed them up to `maximumLiveLanes`. */
export type PoolBoundsV1=Readonly<{maximumLanes:number;maximumLiveLanes:number;quotaBytes:number}>;
/** 🎯️ What an acquisition does: take the newest free lane of the same crate set (`warm`), start a lane (`new`), take the least recently used free lane (`lru`) or wait (`busy`). */
export type PoolChoiceV1=Readonly<{kind:"warm"|"lru"|"new"|"busy";id?:string}>;

const newest=(left:PoolLaneV1,right:PoolLaneV1):number=>right.lastUsedMs-left.lastUsedMs||(left.id<right.id?-1:left.id>right.id?1:0);
const oldest=(left:PoolLaneV1,right:PoolLaneV1):number=>left.lastUsedMs-right.lastUsedMs||(left.id<right.id?-1:left.id>right.id?1:0);
const recorded=(lanes:readonly PoolLaneV1[]):number=>lanes.reduce((sum,lane)=>sum+(lane.bytes??0),0);

/** 💰️ The recorded bytes the idle lanes of the pool may hold: the lane share of the smaller of the image capacity and the machine budget's quota of the compiler store (`null` when no budget is known). */
export function poolQuotaBytesV1(capacityBytes:number,machineStoreQuotaBytes:number|null):number{return Math.floor(Math.min(capacityBytes,machineStoreQuotaBytes??capacityBytes)*UNIT_CACHE_POLICY_V1.gc.laneShare);}

/** 🪣️ The bounds of the pool for a byte quota. */
export function poolBoundsV1(quotaBytes:number):PoolBoundsV1{return Object.freeze({maximumLanes:rules.maximumLanes,maximumLiveLanes:rules.maximumLiveLanes,quotaBytes});}

/** 🧭️ Chooses the lane of one acquisition. Pure and deterministic. The newest free lane built for the same crate set keeps its incremental state warm; otherwise the pool grows while it is inside its bounds, else the least recently used free lane is reused, else (every lane live) the pool grows beyond its bounds up to the live maximum and only then waits. Waiting holds nothing, so no wait chain exists. */
export function planPoolAcquireV1(lanes:readonly PoolLaneV1[],request:Readonly<{crateKey:string}>,bounds:PoolBoundsV1):PoolChoiceV1{
 const free=lanes.filter(lane=>!lane.claimed&&!lane.product),warm=free.filter(lane=>lane.crateKey===request.crateKey).sort(newest)[0];
 if(warm)return{kind:"warm",id:warm.id};
 if(lanes.length<bounds.maximumLanes&&recorded(lanes)<=bounds.quotaBytes)return{kind:"new"};
 const cold=[...free].sort(oldest)[0];
 if(cold)return{kind:"lru",id:cold.id};
 return lanes.length<bounds.maximumLiveLanes?{kind:"new"}:{kind:"busy"};
}

/** ✂️ Chooses the idle lanes to remove so the pool is back inside its bounds: least recently used first, never a claimed lane. Pure; applying the result and planning again removes nothing. */
export function planPoolTrimV1(lanes:readonly PoolLaneV1[],bounds:PoolBoundsV1):string[]{
 const victims:string[]=[];let count=lanes.length,bytes=recorded(lanes);
 for(const lane of lanes.filter(row=>!row.claimed).sort(oldest)){
  if(count<=bounds.maximumLanes&&bytes<=bounds.quotaBytes)break;
  victims.push(lane.id);count--;bytes-=lane.bytes??0;
 }
 return victims;
}

/** 🔭️ Reads the pool of a cache root (the reserved lanes, the static default lane of `.cargo/config.toml` that unattributed commands use, are not part of it): claim and live markers say whether a lane is held, the metadata says what it was last built for, the size file what the collector measured. */
export function readPoolV1(root:string,nowMs:number):PoolLaneV1[]{
 const locks=join(root,layout.locks);
 return laneIdsV1(root).filter(id=>!rules.reserved.includes(id)).map(id=>{
  const paths=lanePathsV1(root,id);let lastUsedMs=0;try{lastUsedMs=Math.floor(statSync(paths.metadata).mtimeMs);}catch{}
  return Object.freeze({id,product:id.startsWith(rules.product.prefix),claimed:claimHeldV1(locks,"lane-"+id)||laneLiveV1(paths,nowMs),lastUsedMs,crateKey:readLaneV1(root,id)?.crateKey??"",bytes:(size=>size?size.bytes-size.incrementalBytes:null)(readLaneSizeV1(root,id))});
 });
}

/** 🧹️ Removes the idle lanes the pool must shed. Every removal takes the lane's own claim first, so a lane that was taken since the plan survives, and publishes the receipts a holder that was killed before its own retention left in the lane (see `retainRemovedLaneV1`) before the lane goes; returns what was removed. */
export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1):Readonly<{removed:number;freedBytes:number}>{
 const lanes=readPoolV1(root,nowMs),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0]));let removed=0,freedBytes=0;
 const retain=(paths:ReturnType<typeof lanePathsV1>):void=>{const conflicts=retainRemovedLaneV1(paths);if(conflicts.length)report?.("retentionConflict",{lane:paths.id,count:conflicts.length,names:conflicts.slice(0,4).join(", ")});};
 for(const id of planPoolTrimV1(lanes,bounds))if(removeLaneV1(root,id,"lane",nowMs,retain)){removed++;freedBytes+=size.get(id)!;}
 return Object.freeze({removed,freedBytes});
}

/** 📣️ What the pool tells its caller, to be rendered in the caller's language. */
export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|"retentionConflict",parameters:Readonly<Record<string,string|number>>)=>void;
/** 🎒️ A lane taken from the pool: its directories, how it was chosen and the release that frees it. */
export type PooledLaneV1=Readonly<{paths:ReturnType<typeof lanePathsV1>;choice:"warm"|"lru"|"new";release():void}>;

/** 🔐️ Takes a lane for one Cargo process: exactly one process holds a lane (its claim, a file created exclusively), so no two Cargo processes ever share a build directory and Cargo's own unit locks can never form a cycle between processes. The lane of the same crate set is preferred, so its incremental state stays warm; afterwards the idle lanes are trimmed to the pool bounds. Without a free lane and below the live maximum the pool grows, at the live maximum it waits by polling, holding nothing.
 * @see ../../🧹️gc/🟦️.ts */
export async function acquirePooledLaneV1(root:string,request:Readonly<{crates:readonly string[];quotaBytes:number;owner?:LaneOwnerV1}>,options:Readonly<{signal?:AbortSignal;now?:()=>number;report?:PoolReportV1}>={}):Promise<PooledLaneV1>{
 const locks=join(root,layout.locks),bounds=poolBoundsV1(request.quotaBytes),key=crateKeyV1(request.crates),now=options.now??Date.now;
 mkdirSync(join(root,layout.lanes),{recursive:true});
 let poll=rules.pollMilliseconds,reported=false;
 for(;;){
  options.signal?.throwIfAborted();
  const catalog=tryClaimV1(locks,rules.catalogClaim);
  if(!catalog){await sleepV1(poll,options.signal);continue;}
  let acquired:PooledLaneV1|undefined;
  try{acquired=(()=>{
  const lanes=readPoolV1(root,now()),choice=planPoolAcquireV1(lanes,{crateKey:key},bounds);
  if(choice.kind==="busy"){
   if(!reported){reported=true;options.report?.("poolBusy",{live:lanes.length,maximum:bounds.maximumLiveLanes});}
   return undefined;
  }
  const id=choice.id??"p"+randomBytes(3).toString("hex"),claim=tryClaimV1(locks,"lane-"+id);
  if(!claim)return undefined;
  let entered:ReturnType<typeof enterLaneV1>;try{entered=enterLaneV1(root,id,request.owner,request.crates);}catch(error){claim.release();throw error;}
  if(choice.kind==="new"&&lanes.length>=bounds.maximumLanes)options.report?.("overflow",{busy:lanes.filter(lane=>lane.claimed).length,total:lanes.length,lane:id});
  try{const trimmed=trimPoolV1(root,bounds,now(),options.report);if(trimmed.removed>0)options.report?.("trimmed",{count:laneIdsV1(root).length,quotaMib:Math.round(bounds.quotaBytes/1048576),removed:trimmed.removed});}catch{}
  return Object.freeze({paths:entered.paths,choice:choice.kind,release(){try{entered.release();}finally{claim.release();}}});
  })();}finally{catalog.release();}
  if(acquired)return acquired;
  await sleepV1(poll,options.signal);poll=Math.min(rules.pollMaximumMilliseconds,Math.ceil(poll*rules.pollBackoffBasisPoints/10000));
 }
}
