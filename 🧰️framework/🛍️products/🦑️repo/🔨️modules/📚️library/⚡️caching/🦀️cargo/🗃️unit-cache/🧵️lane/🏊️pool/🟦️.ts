import {randomBytes} from "node:crypto";
import {mkdirSync} from "node:fs";
import {join} from "node:path";
import {claimHeldV1,claimHolderV1,sleepV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";
import {nativeImageShardCommittedBytesV1,nativeImageShardFoldV1,nativeImageShardKeyV1,nativeImageShardWantBytesV1,readNativeImageShardLedgerV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import {nativeImageCommitmentRequestCostBytesV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧮️commitment/🟦️.ts";
import {ensureLaneImageV1,laneImagePathV1,laneImagePortsV1,repairLaneImagesV1,retireClampedLaneImagesV1,retireIdleLaneImagesV1,type LaneImagePortsV1,type LaneImageWorldV1} from "./🧩️image/🟦️.ts";
import {crateKeyV1,enterLaneV1,laneIdsV1,laneLiveV1,lanePathsV1,laneRecentV1,laneRetainedV1,laneUsedMsV1,readLaneSizeV1,readLaneV1,removeLaneV1,type LaneOwnerV1} from "../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.pool,layout=UNIT_CACHE_POLICY_V1.layout;
/** 🏊️ One lane as the pool sees it: whether a live process holds it, when it was last used, the crate set it was last built for (`""` when unknown), who it belongs to (`owner`, session and agent, absent when unattributed) and its recorded size without its Cargo incremental sessions (`null` until the collector measured it): the sessions are bounded by the collector's own incremental share, so a lane that keeps a crate's incremental state warm is not evicted for holding it. */
export type PoolLaneV1=Readonly<{id:string;claimed:boolean;lastUsedMs:number;crateKey:string;bytes:number|null;owner?:string;product?:boolean;retained?:boolean;held?:boolean;recent?:boolean;flavour?:string}>;
/** 📐️ The bounds of the pool: the idle lanes are trimmed to `maximumLanes` lanes and `quotaBytes` recorded bytes, live lanes may exceed them up to `maximumLiveLanes`. */
export type PoolBoundsV1=Readonly<{maximumLanes:number;maximumLiveLanes:number;quotaBytes:number}>;
/** 🎯️ What an acquisition does: take the newest free lane of the same crate set (`warm`), start a lane (`new`), take the least recently used free lane (`lru`) or wait (`busy`). */
export type PoolChoiceV1=Readonly<{kind:"warm"|"lru"|"new"|"busy";id?:string}>;
/** 🪪️ The owner key of a lane: its session and agent, so two lanes of the same owner can be told apart from lanes of other sessions. */
export const poolOwnerKeyV1=(owner:LaneOwnerV1):string=>owner.session+"|"+owner.agent;

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

/** 🧭️ Chooses the lane of one acquisition. Pure and deterministic. The newest free lane built for the same crate set keeps its incremental state warm, a lane held for a restart (see `holdLaneV1`) included; a held lane is otherwise neither counted against the bounds nor taken as the least recently used lane; otherwise a lane of the same owner, crate set and build flavour that is held by a live process is never copied (a second lane would rebuild the same dependency outputs on the same disk): the acquisition waits for it (`busy` naming that lane); otherwise the pool grows while it is inside its bounds, else the least recently used free lane is reused, else (every lane live) the pool grows beyond its bounds up to the live maximum and only then waits. Waiting holds nothing, so no wait chain exists. */
export function planPoolAcquireV1(lanes:readonly PoolLaneV1[],request:Readonly<{crateKey:string;owner?:string;flavour?:string}>,bounds:PoolBoundsV1):PoolChoiceV1{
 const free=lanes.filter(lane=>!lane.claimed&&!lane.product),warm=free.filter(lane=>lane.crateKey===request.crateKey).sort(newest)[0];
 if(warm)return{kind:"warm",id:warm.id};
 const twin=request.owner===undefined||request.crateKey===""?undefined:lanes.find(lane=>lane.claimed&&!lane.product&&lane.owner===request.owner&&lane.crateKey===request.crateKey&&lane.flavour===request.flavour);
 if(twin)return{kind:"busy",id:twin.id};
 const counted=lanes.filter(lane=>lane.held!==true);
 if(counted.length<bounds.maximumLanes&&recorded(counted)<=bounds.quotaBytes)return{kind:"new"};
 const cold=free.filter(lane=>lane.held!==true).sort(oldest)[0];
 if(cold)return{kind:"lru",id:cold.id};
 return counted.length<bounds.maximumLiveLanes?{kind:"new"}:{kind:"busy"};
}

/** 👯️ Chooses the idle lanes that copy another lane: of all lanes of one owner, crate set and build flavour only one is kept, the one a live process holds, else a requested one, else the most recently used; the others are idle copies of the same dependency outputs and go whatever the bounds say, never a claimed or requested lane, a held restart lane or a product lane. Lanes without an owner or without a crate set are never copies. Pure; applying the result and planning again removes nothing. */
export function planPoolDuplicatesV1(lanes:readonly PoolLaneV1[],requested:readonly string[]=[]):string[]{
 const groups=new Map<string,PoolLaneV1[]>(),asked=new Set(requested),victims:PoolLaneV1[]=[];
 for(const lane of lanes)if(lane.owner!==undefined&&lane.crateKey!==""&&lane.product!==true&&lane.held!==true){const key=[lane.owner,lane.crateKey,lane.flavour??""].join("\n");groups.set(key,[...groups.get(key)??[],lane]);}
 for(const group of groups.values()){
  if(group.length<2)continue;
  const kept=group.filter(lane=>lane.claimed).sort(newest)[0]??group.filter(lane=>asked.has(lane.id)).sort(newest)[0]??[...group].sort(newest)[0]!;
  victims.push(...group.filter(lane=>lane!==kept&&!lane.claimed&&!asked.has(lane.id)));
 }
 return victims.sort(oldest).map(lane=>lane.id);
}

/** ✂️ Chooses the idle lanes to remove so the pool is back inside its bounds: the idle copies of another lane first (`planPoolDuplicatesV1`), then least recently used first, never a claimed lane, never a requested lane, and a recent lane of a declared flavour only under free-space `pressure`. Pure; applying the result and planning again removes nothing. */
export function planPoolTrimV1(lanes:readonly PoolLaneV1[],bounds:PoolBoundsV1,context:Readonly<{requested?:readonly string[];pressure?:boolean}>={}):string[]{
 const requested=new Set(context.requested??[]),copies=planPoolDuplicatesV1(lanes,context.requested),victims:string[]=[...copies],gone=new Set(copies);let count=lanes.length-copies.length,bytes=recorded(lanes.filter(lane=>!gone.has(lane.id)));
 for(const lane of lanes.filter(row=>!gone.has(row.id)&&!row.claimed&&!requested.has(row.id)&&(context.pressure===true||row.recent!==true)).sort(oldest)){
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
  const record=readLaneV1(root,id),owner=record?.session===undefined||record.agent===undefined?undefined:poolOwnerKeyV1({session:record.session,agent:record.agent});
  const holdUntilMs=readLaneV1(root,id)?.holdUntilMs,held=holdUntilMs!==undefined&&nowMs<holdUntilMs&&!id.startsWith(rules.product.prefix);
  return Object.freeze({id,product:id.startsWith(rules.product.prefix),...owner===undefined?{}:{owner},retained:laneRetainedV1(id,lastUsedMs,nowMs,holdUntilMs),held,recent:laneRecentV1(flavour,lastUsedMs,nowMs),...flavour===undefined?{}:{flavour},claimed:claimHeldV1(locks,"lane-"+id)||laneLiveV1(paths,nowMs),lastUsedMs,crateKey:readLaneV1(root,id)?.crateKey??"",bytes:(size=>size?size.bytes-size.incrementalBytes:null)(readLaneSizeV1(root,id))});
 });
}

/** 🪪️ Names who holds the given lanes for a wait message: process id, agent and session of each claimed lane (at most `listedHolders`, the rest counted), `nobody readable` when no claim names a holder. */
export function laneHoldersV1(root:string,ids:readonly string[],nowMs:number):string{
 const locks=join(root,layout.locks),named=ids.flatMap(id=>{const holder=claimHolderV1(locks,"lane-"+id);if(!holder)return[];const lane=readLaneV1(root,id);return[`${id}: pid ${holder.pid}${lane?.agent?`, agent ${lane.agent}`:""}${lane?.session?`, session ${lane.session}`:""}${holder.command?`, command ${holder.command.slice(0,120)}`:""}, ${Math.max(0,Math.round((nowMs-holder.atMs)/60000))} min`];});
 return named.length===0?"nobody readable":named.slice(0,rules.listedHolders).join("; ")+(named.length>rules.listedHolders?`; and ${named.length-rules.listedHolders} more`:"");
}

/** 🧹️ Removes the idle lanes the pool must shed, a lane the starting build requests never among them. The byte bound is raised to what the largest declared build flavour needs; when the image cannot hold that need below its high-water mark the pool says so and removes nothing; recent lanes of a declared flavour go only while the image is under free-space pressure. Every removal takes the lane's own claim first, so a lane that was taken since the plan survives, and publishes authenticated binding evidence and every owned receipt before the lane goes; unresolved custody keeps it; returns what was removed. */
export function trimPoolV1(root:string,bounds:PoolBoundsV1,nowMs:number,report?:PoolReportV1,context:PoolTrimContextV1={}):Readonly<{removed:number;freedBytes:number}>{
 const lanes=boundedPoolV1(readPoolV1(root,nowMs)),size=new Map(lanes.map(lane=>[lane.id,lane.bytes??0])),need=poolNeedBytesV1(lanes),volume=context.volume?.(),high=volume===undefined?Infinity:Math.floor(volume.volumeBytes*UNIT_CACHE_POLICY_V1.gc.highWaterShare);let removed=0,freedBytes=0;
 if(need>high){report?.("poolCannotHold",{flavour:context.flavour??"",needMib:Math.round(need/1048576),imageMib:Math.round(high/1048576)});return Object.freeze({removed,freedBytes});}
 const pressure=volume!==undefined&&volume.volumeBytes-volume.freeBytes>high,requested=[...context.requested??[],...lanes.filter(lane=>context.flavour!==undefined&&lane.flavour===context.flavour&&lane.recent===true).map(lane=>lane.id)];
 for(const id of planPoolTrimV1(lanes,{...bounds,quotaBytes:Math.max(bounds.quotaBytes,need)},{requested,pressure}))if(removeIdleLaneV1(root,id,nowMs,report)){removed++;freedBytes+=size.get(id)!;}
 return Object.freeze({removed,freedBytes});
}

/** 🗑️ Removes one idle lane the way every shedding does: under its own claim, never while a live marker exists, after its binding evidence and owned receipts are published (`retainRemovedLaneV1`); `false` when the lane is held, live or its custody is unresolved. */
export function removeIdleLaneV1(root:string,id:string,nowMs:number,report?:PoolReportV1):boolean{
 const retain=(paths:ReturnType<typeof lanePathsV1>):boolean=>{const conflicts=retainRemovedLaneV1(paths);if(conflicts.length)report?.("retentionConflict",{lane:paths.id,count:conflicts.length,names:conflicts.slice(0,4).join(", ")});return conflicts.length===0;};
 return removeLaneV1(root,id,"lane",nowMs,retain);
}

/** 🚦️ One idle lane as the admission of a new lane sees it: when it was last used, the bytes of its files on the host (`null` until the collector measured them) and the capacity its image declares (`0` for a lane that lives in the home image). */
export type PoolAdmissionLaneV1=Readonly<{id:string;lastUsedMs:number;bytes:number|null;capacityBytes:number}>;
/** 🚦️ What a new lane is short of: free host bytes below the floor plus the lane's working set, and room in the commitment budget of the lane images below the capacity its image is born with. */
export type PoolShortageV1=Readonly<{hostBytes:number;commitmentBytes:number}>;
/** 🚦️ Chooses the idle lanes to retire before a new lane is born, least recently used first, until their measured bytes cover the host shortage and their image capacities cover the commitment shortage. A shortage that all candidates together cannot cover is not worked on: retiring everything would destroy warm state without admitting the lane (the birth is then refused and the lane stays in the home image). Pure; the result is a prefix of the least-recently-used order. */
export function planLaneAdmissionV1(candidates:readonly PoolAdmissionLaneV1[],shortage:PoolShortageV1):string[]{
 const order=[...candidates].sort((left,right)=>left.lastUsedMs-right.lastUsedMs||(left.id<right.id?-1:left.id>right.id?1:0));
 let host=shortage.hostBytes>order.reduce((sum,lane)=>sum+(lane.bytes??0),0)?0:shortage.hostBytes,commitment=shortage.commitmentBytes>order.reduce((sum,lane)=>sum+lane.capacityBytes,0)?0:shortage.commitmentBytes;
 const victims:string[]=[];
 for(const lane of order){if(host<=0&&commitment<=0)break;victims.push(lane.id);host-=lane.bytes??0;commitment-=lane.capacityBytes;}
 return victims;
}

/** 🚪️ Admits a new lane before its image is born: while the host has less free space than its floor plus the working set of one lane, or the declared capacities of the lane images (or what all images of the host promise, the host-wide commitment ceiling, counting a new image by its growth reserve like every standing one) leave less commitment budget than a new image needs, the least recently used idle lane (never a claimed, live, held or product lane, never the new one) is removed and its image retired whole, so its blocks and its declared capacity return at once; every step measures again. Never the lane of the requester (`requester`: the same owner and crate set), the lane its own next build reuses; returns the lanes it retired; a lane that cannot be admitted stays admitted to the home image by the birth's own refusal. */
export async function admitNewLaneV1(root:string,id:string,ports:LaneImagePortsV1|undefined,nowMs:()=>number,options:Readonly<{report?:PoolReportV1;world?:LaneImageWorldV1;requester?:Readonly<{owner:string;crateKey:string}>}>={}):Promise<readonly string[]>{
 if(ports===undefined||!ports.rule.enabled)return[];
 const found=nativeImageShardKeyV1(ports.rule,laneImagePathV1(id));if(found===null)return[];
 const need=ports.rule.roots[found.root]!.needBytes,want=nativeImageShardWantBytesV1(ports.rule,need),retired:string[]=[],tried=new Set<string>([id]);
 for(;;){
  const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory));if(live.has(id))return Object.freeze(retired);
  const budget=ports.budgetBytes?.(),host=ports.hostCommitment?.(),hostFree=ports.hostFreeBytes(),shortage:PoolShortageV1={hostBytes:Math.max(0,(ports.hostFreeFloorBytes?.()??0)+need-hostFree),commitmentBytes:Math.max(budget===undefined?0:Math.max(0,nativeImageShardCommittedBytesV1(live,found.root)+want-budget),host===undefined?0:Math.max(0,host.committedBytes+nativeImageCommitmentRequestCostBytesV1(want)-host.ceilingBytes))};
  if(shortage.hostBytes<=0&&shortage.commitmentBytes<=0)break;
  const candidates=boundedPoolV1(readPoolV1(root,nowMs())).filter(lane=>!lane.claimed&&lane.product!==true&&lane.held!==true&&!tried.has(lane.id)&&!(options.requester!==undefined&&lane.owner===options.requester.owner&&lane.crateKey===options.requester.crateKey)).map(lane=>({id:lane.id,lastUsedMs:lane.lastUsedMs,bytes:readLaneSizeV1(root,lane.id)?.bytes??null,capacityBytes:live.get(lane.id)?.capacityBytes??0}));
  const victim=planLaneAdmissionV1(candidates,shortage)[0];if(victim===undefined)break;
  tried.add(victim);
  if(!removeIdleLaneV1(root,victim,nowMs(),options.report))continue;
  retired.push(victim);
  try{await retireIdleLaneImagesV1(root,ports,nowMs(),options.world);}catch{}
 }
 if(retired.length>0)options.report?.("poolAdmitted",{count:retired.length,lanes:retired.join(", "),freeGib:Math.round(ports.hostFreeBytes()/1073741824)});
 return Object.freeze(retired);
}

/** 📣️ What the pool tells its caller, to be rendered in the caller's language. */
export type PoolReportV1=(kind:"overflow"|"trimmed"|"poolBusy"|"poolSameCrates"|"poolAdmitted"|"poolCannotHold"|"productLaneHeld"|"productLaneQueued"|"productLaneWaiting"|"retentionConflict",parameters:Readonly<Record<string,string|number>>)=>void;
/** 🎒️ A lane taken from the pool: its directories, how it was chosen and the release that frees it. */
export type PooledLaneV1=Readonly<{paths:ReturnType<typeof lanePathsV1>;choice:"warm"|"lru"|"new";release():void}>;

/** 🔐️ Takes a lane for one Cargo process: exactly one process holds a lane (its claim, a file created exclusively), so no two Cargo processes ever share a build directory and Cargo's own unit locks can never form a cycle between processes. The lane of the same crate set is preferred, so its incremental state stays warm; afterwards the idle lanes are trimmed to the pool bounds. Without a free lane and below the live maximum the pool grows, at the live maximum it waits by polling, holding nothing.
 * @see ../../🧹️gc/🟦️.ts */
export async function acquirePooledLaneV1(root:string,request:Readonly<{crates:readonly string[];quotaBytes:number;commitmentBytes?:number;owner?:LaneOwnerV1;flavour?:string;fresh?:boolean}>,options:Readonly<{signal?:AbortSignal;now?:()=>number;report?:PoolReportV1;volume?:PoolTrimContextV1["volume"]}>={}):Promise<PooledLaneV1>{
 const locks=join(root,layout.locks),bounds=poolBoundsV1(request.quotaBytes),key=crateKeyV1(request.crates),now=options.now??Date.now;
 mkdirSync(join(root,layout.lanes),{recursive:true});
 const lanePorts=await laneImagePortsV1(root,options.signal,request.commitmentBytes,key);
 if(lanePorts!==undefined)try{await repairLaneImagesV1(lanePorts);await retireIdleLaneImagesV1(root,lanePorts,now());await retireClampedLaneImagesV1(root,lanePorts,now());}catch{}
 let poll=rules.pollMilliseconds,reported=false;
 for(;;){
  options.signal?.throwIfAborted();
  const catalog=tryClaimV1(locks,rules.catalogClaim);
  if(!catalog){await sleepV1(poll,options.signal);continue;}
  let acquired:PooledLaneV1|undefined;
  try{acquired=await(async()=>{
  const pool=readPoolV1(root,now()),lanes=[...boundedPoolV1(pool),...pool.filter(lane=>lane.held===true)],choice:PoolChoiceV1=request.fresh===true?{kind:"new"}:planPoolAcquireV1(lanes,{crateKey:key,...request.owner===undefined?{}:{owner:poolOwnerKeyV1(request.owner)},...request.flavour===undefined?{}:{flavour:request.flavour}},bounds);
  if(choice.kind==="busy"){
   if(!reported){reported=true;options.report?.(choice.id===undefined?"poolBusy":"poolSameCrates",choice.id===undefined?{live:lanes.length,maximum:bounds.maximumLiveLanes,holders:laneHoldersV1(root,lanes.map(lane=>lane.id),now())}:{lane:choice.id,holders:laneHoldersV1(root,[choice.id],now())});}
   return undefined;
  }
  const id=choice.id??"p"+randomBytes(3).toString("hex"),claim=tryClaimV1(locks,"lane-"+id);
  if(!claim)return undefined;
  let entered:ReturnType<typeof enterLaneV1>;try{if(choice.kind==="new")await admitNewLaneV1(root,id,lanePorts,now,{...options.report===undefined?{}:{report:options.report},...request.owner===undefined?{}:{requester:{owner:poolOwnerKeyV1(request.owner),crateKey:key}}});const image=await ensureLaneImageV1(root,id,lanePorts);if(image.kind==="birth"||image.kind==="grown"||image.kind==="failed"||image.kind==="refused")process.stderr.write(`[lane] image of lane ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"} / Image der Spur ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"}\n`);entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);}catch(error){claim.release();throw error;}
  if(choice.kind==="new"&&lanes.filter(lane=>lane.held!==true).length>=bounds.maximumLanes)options.report?.("overflow",{busy:lanes.filter(lane=>lane.claimed).length,total:lanes.filter(lane=>lane.held!==true).length,lane:id});
  try{const trimmed=trimPoolV1(root,bounds,now(),options.report,{requested:[id],...request.flavour===undefined?{}:{flavour:request.flavour},...options.volume===undefined?{}:{volume:options.volume}});if(trimmed.removed>0)options.report?.("trimmed",{count:readPoolV1(root,now()).length,quotaMib:Math.round(bounds.quotaBytes/1048576),removed:trimmed.removed});}catch{}
  return Object.freeze({paths:entered.paths,choice:choice.kind,release(){try{entered.release();}finally{claim.release();}}});
  })();}finally{catalog.release();}
  if(acquired)return acquired;
  await sleepV1(poll,options.signal);poll=Math.min(rules.pollMaximumMilliseconds,Math.ceil(poll*rules.pollBackoffBasisPoints/10000));
 }
}
