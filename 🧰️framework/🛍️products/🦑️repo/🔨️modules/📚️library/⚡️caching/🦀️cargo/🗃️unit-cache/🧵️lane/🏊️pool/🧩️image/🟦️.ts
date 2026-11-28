import{execFile}from"node:child_process";
import{existsSync,readdirSync,statfsSync}from"node:fs";
import{dirname,join,sep}from"node:path";
import{NATIVE_IMAGE_SHARD_V1,createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,measureNativeImageShardFreeV1,measureNativeImageShardV1,nativeImageShardFoldV1,nativeImageShardSizedBytesV1,nativeImageShardHomeLayoutV1,nativeImageShardLayoutV1,nativeImageShardMountedV1,readNativeImageShardLedgerV1,repairNativeImageShardsV1,retireNativeImageShardV1,type NativeImageShardLayoutV1,type NativeImageShardLifecycleV1,type NativeImageShardRuleV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import{createNativeImageDetachPortV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🔁️detach/🟦️.ts";
import{growNativeImageShardPlannedV1,nativeImageGrowthPlanV1,nativeImageLaneSizingV1,shrinkNativeImageShardPlannedV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪜️growth/🟦️.ts";
import{measureNativeImageCommitmentV1,nativeImageCommitmentDeclarableBytesV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧮️commitment/🟦️.ts";
import{NATIVE_IMAGE_RELIEF_V1,NativeImageWritesRefusedErrorV1,nativeImageClampV1,nativeImageReliefTextV1,nativeImageRightSizeV1,nativeImageShrinkV1,probeNativeImageWritesV1,type NativeImageWritesIoV1,type NativeImageWritesRefusedFactsV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪫️relief/🟦️.ts";
import{claimHeldV1,sleepV1,tryClaimV1}from"../../../🔒️lock/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../🔏️key/🟦️.ts";
import{laneIdsV1,laneLiveV1,lanePathsV1,readLaneSizeV1,readLaneV1,readLanePeaksV1}from"../../🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout;
/** 🧩️ What a lane image needs from its world: the partition directory of the image, the shard rule, the platform, the free bytes of the host volume, the free bytes it must keep (`hostFreeFloorBytes`, the machine budget's floor), the largest lane measured so far (`measuredNeedBytes`, what an existing image is stretched to), the sizes of the lanes measured so far (`laneSizes`: a new image is sized from their high percentile instead of the declared need, so a host with less room than the declared need still births one), what an image holds and its host has free (`measure`), the most the declared capacities of the lane images may add up to (`budgetBytes`, the compiler store's share of the machine budget under its host share), what all images of all roots declare in all against the host-wide ceiling (`hostCommitment`, the commitment budget every birth and growth is admitted against), the lifecycle of one shard and the lock that serialises births. */
export type LaneImagePortsV1=Readonly<{directory:string;rule:NativeImageShardRuleV1;platform:string;hostFreeBytes():number;hostFreeFloorBytes?():number;budgetBytes?():number|undefined;hostCommitment?():Readonly<{committedBytes:number;ceilingBytes:number}>|undefined;measuredNeedBytes?():number|undefined;laneSizes?():readonly number[];holders?(layout:NativeImageShardLayoutV1):Promise<number>;measure?(layout:NativeImageShardLayoutV1):Readonly<{usedBytes:number;hostFreeBytes:number}>|undefined;now():number;lifecycle:NativeImageShardLifecycleV1;lock:<T>(body:()=>Promise<T>)=>Promise<T>}>;
/** 🧭️ What became of the image of one lane: it stays in the home image (`home`, `off`), its image existed (`existing`), was stretched because the need outgrew it (`grown`), was born (`birth`), could not be born (`refused`) or the lifecycle failed (`failed`, the lane then stays home). */
export type LaneImageOutcomeV1=Readonly<{kind:"off"|"home"|"existing"|"grown"|"birth"|"refused"|"failed";id?:string;detail?:string}>;

/** 📍️ The partition directory whose home image holds a cache root (`<partition>/m/u`), `undefined` for a root that is no `m/u` of a partition with a home image. Pure apart from the existence check. */
export function laneImageDirectoryV1(root:string,exists:(path:string)=>boolean=existsSync):string|undefined{
 const mount=dirname(root);if(root.slice(mount.length+1)!=="u"||mount.slice(dirname(mount).length+1)!==NATIVE_IMAGE_SHARD_V1.mountDirectory)return undefined;
 const directory=dirname(mount);return exists(nativeImageShardHomeLayoutV1(directory).image)?directory:undefined;
}
/** 🔑️ The logical path of a lane below the home mount, the key the shard rule claims (`u/l/<id>`). */
export const laneImagePathV1=(id:string):string=>["u",layout.lanes,id].join("/");

/** 🏔️ The most one birth may declare under the host-wide commitment budget: a birth promises only its growth reserve (the unit standing images are counted in), so `undefined` (no bound) while the room holds that reserve, the room itself below it. */
export function laneImageHostRoomBytesV1(ports:Pick<LaneImagePortsV1,"hostCommitment">):number|undefined{const row=ports.hostCommitment?.();return row===undefined?undefined:nativeImageCommitmentDeclarableBytesV1(Math.max(0,row.ceilingBytes-row.committedBytes));}

/** 🌍️ What the lane images read of the file system, replaceable in a test: whether a path exists, what a directory holds and whether a path is a mount point. */
export type LaneImageWorldV1=Readonly<{exists(path:string):boolean;entries(path:string):readonly string[];mounted(path:string):boolean;available?(path:string):number|undefined}>;
const REAL_WORLD:LaneImageWorldV1=Object.freeze({exists:existsSync,entries:path=>{try{return readdirSync(path);}catch{return[];}},mounted:nativeImageShardMountedV1,available:path=>{try{const row=statfsSync(path);return Number(row.bavail)*row.bsize;}catch{return undefined;}}});

/** 🌱️ Makes the image of one lane exist before its first byte is written: an existing shard is mounted again (after a reboot), a lane that has no files yet gets a shard of its own when the rule, the platform and the host allow it, a lane whose files already live in the home image stays there (mounting over them would hide them; a re-layout needs the quiet image). The lane never fails because of its image: every refusal and every error leaves it in the home image. */
export async function ensureLaneImageV1(root:string,id:string,ports:LaneImagePortsV1|undefined,world:LaneImageWorldV1=REAL_WORLD):Promise<LaneImageOutcomeV1>{
 const{exists,entries,mounted}=world;
 if(ports===undefined||!ports.rule.enabled)return{kind:"off"};
 const path=laneImagePathV1(id),home=nativeImageShardHomeLayoutV1(ports.directory),base=lanePathsV1(root,id).base;
 if(!mounted(home.mountPoint))return{kind:"home",id,detail:"home image not mounted"};
 const sizing=ports.laneSizes===undefined?undefined:nativeImageLaneSizingV1(ports.rule,0,ports.laneSizes(),ports.platform),live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)),shardId=[...live.values()].find(record=>record.path===path)?.id;
 if(shardId===undefined&&exists(base)&&entries(base).length>0)return{kind:"home",id};
 try{
  const born=sizing?.measuredNeedBytes===undefined?ports.measuredNeedBytes:()=>sizing.measuredNeedBytes,made=await ensureNativeImageShardV1({directory:ports.directory,rule:sizing?.rule??ports.rule,platform:ports.platform,path,hostFreeBytes:ports.hostFreeBytes,...ports.budgetBytes===undefined?{}:{budgetBytes:ports.budgetBytes},...ports.hostCommitment===undefined?{}:{hostRoomBytes:()=>laneImageHostRoomBytesV1(ports)},...born===undefined?{}:{measuredNeedBytes:born},now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock});
  if(made.kind==="refused")return{kind:"refused",id,detail:made.reason};
  if(made.kind==="home")return{kind:"home",id};
  if(made.kind==="existing"&&ports.measure!==undefined){
   try{
    const grown=await growNativeImageShardPlannedV1({directory:ports.directory,rule:ports.rule,platform:ports.platform,id:made.layout.id,measure:ports.measure,...ports.budgetBytes===undefined?{}:{budgetBytes:ports.budgetBytes},...ports.hostCommitment===undefined?{}:{hostRoomBytes:()=>laneImageHostRoomBytesV1(ports)},...ports.measuredNeedBytes===undefined?{}:{measuredNeedBytes:ports.measuredNeedBytes},...ports.hostFreeFloorBytes===undefined?{}:{hostKeepBytes:ports.hostFreeFloorBytes},...ports.holders===undefined?{}:{holders:ports.holders},now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock});
    if(grown.kind==="grown")return{kind:"grown",id:made.layout.id,detail:String(grown.capacityBytes)};
    if(grown.kind==="wait")return{kind:"existing",id:made.layout.id,detail:"grow: "+grown.reason};
   }catch(error){return{kind:"existing",id:made.layout.id,detail:"grow: "+String((error as Error)?.message??error).slice(0,300)};}
  }
  return{kind:made.kind,id:made.layout.id};
 }catch(error){return{kind:"failed",id,detail:String((error as Error)?.message??error).slice(0,300)};}
}

/** 🗑️ Retires the images of lanes that are gone: a live lane shard whose lane directory is no longer there, or is mounted with no lane record left, and whose lane is neither claimed nor in use, is detached and its files removed, so every block of the lane returns to the host at once (a mounted image of at most 2^35 bytes gives freed blocks back only by trim). A busy image stays; returns the ids retired. */
export async function retireIdleLaneImagesV1(root:string,ports:LaneImagePortsV1|undefined,nowMs:number,world:LaneImageWorldV1=REAL_WORLD):Promise<readonly string[]>{
 if(ports===undefined||!ports.rule.enabled)return[];
 const{exists,mounted}=world;
 const retired:string[]=[],locks=join(root,layout.locks);
 for(const record of nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)).values()){
  if(record.root!==0||UNIT_CACHE_POLICY_V1.pool.reserved.includes(record.key))continue;
  const paths=lanePathsV1(root,record.key),attached=mounted(nativeImageShardLayoutV1(ports.directory,record).mountPoint);
  if(exists(paths.base)&&(!attached||exists(paths.metadata)))continue;
  const claim=tryClaimV1(locks,"lane-"+record.key);if(!claim)continue;
  try{if(laneLiveV1(paths,nowMs))continue;if(await retireNativeImageShardV1({directory:ports.directory,id:record.id,now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock}))retired.push(record.id);}
  catch{}
  finally{claim.release();}
 }
 return Object.freeze(retired);
}

/** 📐️ Re-creates a lane image the host clamps: a mounted image whose free bytes `statfs` reports (host free less the part of its declaration the host does not back) fall far below what it holds free inside, and whose declared capacity exceeds the capacity it is sized for from its need and what it holds, is retired whole when no build holds its lane (never a claimed or live lane, a busy mount stays), so the next acquisition births it at the right size and it is usable again whatever the host's free bytes. Returns the ids retired.
 * @see ../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪫️relief/🟦️.ts */
export async function retireClampedLaneImagesV1(root:string,ports:LaneImagePortsV1|undefined,nowMs:number,world:LaneImageWorldV1=REAL_WORLD):Promise<readonly string[]>{
 if(ports===undefined||!ports.rule.enabled||ports.measure===undefined||world.available===undefined)return[];
 const retired:string[]=[],locks=join(root,layout.locks);
 for(const record of nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)).values()){
  if(record.root!==0||UNIT_CACHE_POLICY_V1.pool.reserved.includes(record.key))continue;
  const shard=nativeImageShardLayoutV1(ports.directory,record);if(!world.mounted(shard.mountPoint))continue;
  const used=ports.measure(shard)?.usedBytes,available=world.available(shard.mountPoint);if(used===undefined||available===undefined)continue;
  const sized=nativeImageShardSizedBytesV1(ports.platform,ports.rule,{needBytes:ports.rule.roots[record.root]!.needBytes,measuredBytes:Math.max(used,readLaneSizeV1(root,record.key)?.bytes??0)});
  if(!nativeImageRightSizeV1({capacityBytes:record.capacityBytes,sizedBytes:sized,clamped:nativeImageClampV1({capacityBytes:record.capacityBytes,usedBytes:used,availableBytes:available}).clamped}).retire)continue;
  const claim=tryClaimV1(locks,"lane-"+record.key);if(!claim)continue;
  try{if(laneLiveV1(lanePathsV1(root,record.key),nowMs))continue;if(await retireNativeImageShardV1({directory:ports.directory,id:record.id,now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock}))retired.push(record.id);}
  catch{}
  finally{claim.release();}
 }
 return Object.freeze(retired);
}

/** 🧾️ What the write check found of one mounted shard: it takes writes (`writable`), was `shrunk` in place and takes them again, or `refuses` them although it holds room inside (the host clamp) and could not be repaired because a build holds it. */
export type ShardWritesRowV1=Readonly<{id:string;key:string;state:"writable"|"shrunk"|"refuses";shrunk?:Readonly<{fromBytes:number;toBytes:number;usedBytes:number}>;facts?:NativeImageWritesRefusedFactsV1}>;
/** 🛡️ Finds the mounted shards of every root (lanes, `w`, `sealed`, `cargo`) that refuse writes before a build starts: each one takes one small synced write, and only a shard that refuses it or reports less than the usable floor is measured. A shard that holds room inside but reports (almost) none is clamped by the host (it declares more than the host backs): it is shrunk in place with its content when no build holds it, then written again. A really full shard is left to the full-image relief; a clamped shard that stays refused answers `refuses` with the facts. Only touches idle images and never raises.
 * @see ../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪫️relief/🟦️.ts */
export async function repairRefusingShardsV1(root:string,ports:LaneImagePortsV1|undefined,world:LaneImageWorldV1&Readonly<{io?:NativeImageWritesIoV1}>=REAL_WORLD):Promise<readonly ShardWritesRowV1[]>{
 if(ports===undefined||!ports.rule.enabled||ports.measure===undefined)return[];
 const rows:ShardWritesRowV1[]=[],relief=NATIVE_IMAGE_RELIEF_V1;
 for(const record of nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)).values()){
  const shard=nativeImageShardLayoutV1(ports.directory,record);if(!world.mounted(shard.mountPoint))continue;
  let probe=probeNativeImageWritesV1(shard.mountPoint,world.io);
  if(probe.writable&&probe.availableBytes>=relief.floorFreeBytes){rows.push({id:record.id,key:record.key,state:"writable"});continue;}
  const used=ports.measure(shard)?.usedBytes;if(used===undefined){rows.push({id:record.id,key:record.key,state:"writable"});continue;}
  const clamp=nativeImageClampV1({capacityBytes:record.capacityBytes,usedBytes:used,availableBytes:probe.availableBytes});
  if(!clamp.clamped){rows.push({id:record.id,key:record.key,state:"writable"});continue;}
  let shrunk:ShardWritesRowV1["shrunk"];
  try{
   const out=await shrinkNativeImageShardPlannedV1({directory:ports.directory,rule:ports.rule,platform:ports.platform,id:record.id,measure:ports.measure,available:()=>probe.availableBytes,...record.root!==0||ports.measuredNeedBytes===undefined?{}:{measuredNeedBytes:ports.measuredNeedBytes},...ports.holders===undefined?{}:{holders:ports.holders},now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock});
   if(out.kind==="shrunk"){shrunk={fromBytes:out.fromBytes,toBytes:out.capacityBytes,usedBytes:out.usedBytes};probe=probeNativeImageWritesV1(shard.mountPoint,world.io);}
  }catch{}
  if(probe.writable){rows.push({id:record.id,key:record.key,state:shrunk===undefined?"writable":"shrunk",...shrunk===undefined?{}:{shrunk}});continue;}
  const sized=nativeImageShardSizedBytesV1(ports.platform,ports.rule,{needBytes:ports.rule.roots[record.root]!.needBytes,measuredBytes:used});
  rows.push({id:record.id,key:record.key,state:"refuses",facts:{image:record.path,reason:probe.reason??"ENOSPC",capacityBytes:record.capacityBytes,availableBytes:probe.availableBytes,innerFreeBytes:clamp.innerFreeBytes,hostFreeBytes:ports.hostFreeBytes(),targetBytes:nativeImageShrinkV1({capacityBytes:record.capacityBytes,usedBytes:used,sizedBytes:sized,clamped:true}).targetBytes}});
 }
 return Object.freeze(rows);
}
/** 🚧️ Lets a build start only while every mounted shard of the cache root takes writes: repairs the clamped ones (`repairRefusingShardsV1`), reports each shrink in place (`report`) and refuses with the typed bilingual `NativeImageWritesRefusedErrorV1` for the first shard that still refuses, before the first compiler starts instead of half an hour into it. A cache root without images passes. */
export async function requireShardsWritableV1(root:string,options:Readonly<{signal?:AbortSignal;language:"en"|"de";report?:(line:string)=>void;ports?:LaneImagePortsV1;world?:LaneImageWorldV1&Readonly<{io?:NativeImageWritesIoV1}>}>):Promise<readonly ShardWritesRowV1[]>{
 const ports=options.ports??await laneImagePortsV1(root,options.signal),rows=await repairRefusingShardsV1(root,ports,options.world);
 for(const row of rows)if(row.shrunk!==undefined)options.report?.(nativeImageReliefTextV1("shrunk",options.language,{image:row.key,capacity:gib(row.shrunk.fromBytes),target:gib(row.shrunk.toBytes),used:gib(row.shrunk.usedBytes)}));
 const refused=rows.find(row=>row.state==="refuses");if(refused?.facts!==undefined)throw new NativeImageWritesRefusedErrorV1(refused.facts);
 return rows;
}
const gib=(bytes:number):string=>(bytes/1073741824).toFixed(1)+" GiB";

/** 🩹️ After a reboot: mounts every live lane shard again and retires those whose files are gone. */
export async function repairLaneImagesV1(ports:LaneImagePortsV1|undefined,world:LaneImageWorldV1=REAL_WORLD):Promise<Readonly<Record<string,"mounted"|"retired">>>{
 if(ports===undefined||!ports.rule.enabled||!world.mounted(nativeImageShardHomeLayoutV1(ports.directory).mountPoint))return{};
 return repairNativeImageShardsV1({directory:ports.directory,now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock});
}

const CLAIM="native-image-shards";
/** 🔒️ The lock that serialises the births and retirements of the shards of one partition, a claim file below the cache root polled until it is free, so two processes never birth the same lane. */
export function laneImageLockV1(root:string,signal?:AbortSignal):LaneImagePortsV1["lock"]{
 return async body=>{
  const locks=join(root,layout.locks);let claim=tryClaimV1(locks,CLAIM);
  while(claim===undefined){await sleepV1(250,signal);claim=tryClaimV1(locks,CLAIM);}
  try{return await body();}finally{claim.release();}
 };
}
/** 📊️ The sizes a new lane image is sized from: the peak recorded for the crate set (it outlives the lanes that reached it) and the sizes the collector recorded for the lanes of that crate set; without any evidence for the crate set (or without a crate set) the sizes of all lanes of the cache root, a lane without a record counts nothing. */
export function laneSizesV1(root:string,crateKey?:string):readonly number[]{
 const peak=crateKey===undefined?undefined:readLanePeaksV1(root)[crateKey],own=laneIdsV1(root).filter(id=>crateKey===undefined||readLaneV1(root,id)?.crateKey===crateKey).map(id=>readLaneSizeV1(root,id)?.bytes??0),evidence=[...peak===undefined?[]:[peak],...own].filter(bytes=>bytes>0);
 return evidence.length>0||crateKey===undefined?evidence:laneIdsV1(root).map(id=>readLaneSizeV1(root,id)?.bytes??0).filter(bytes=>bytes>0);
}
/** 📏️ The most bytes one lane of a crate set has held (all lanes without a crate set or evidence for it), from the recorded peak and the sizes the collector recorded (`undefined` while none was measured): the measured need a lane image is sized for and stretched to. */
export function laneMeasuredNeedBytesV1(root:string,crateKey?:string):number|undefined{
 const most=Math.max(0,...laneSizesV1(root,crateKey));
 return most>0?most:undefined;
}
/** 🧮️ The declared bytes of idle lane images that nothing uses: of a lane shard that no process holds only what it really holds counts against the commitment budget of its root (it can only be written again once a build holds it, and its growth is admitted against the budget then), so a fleet of dead sessions' sparse images does not refuse the lane of a live one. The declared capacity less the larger of the bytes the image holds and the size recorded for its lane; a claimed or live lane counts in full. */
export function laneIdleSlackBytesV1(root:string,directory:string,nowMs:number,measure:(layout:NativeImageShardLayoutV1)=>Readonly<{usedBytes:number}>|undefined=measureNativeImageShardFreeV1):number{
 const locks=join(root,layout.locks);let slack=0;
 for(const record of nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).values()){
  if(record.root!==0||UNIT_CACHE_POLICY_V1.pool.reserved.includes(record.key))continue;
  const paths=lanePathsV1(root,record.key);if(claimHeldV1(locks,"lane-"+record.key)||laneLiveV1(paths,nowMs))continue;
  const used=measure(nativeImageShardLayoutV1(directory,record))?.usedBytes;if(used===undefined)continue;
  slack+=Math.max(0,record.capacityBytes-Math.max(used,readLaneSizeV1(root,record.key)?.bytes??0));
 }
 return slack;
}
/** 🈵️ What a lane whose image is full does next: its image `grows` to `targetBytes` when no build holds it, or stays `capped` for the reason the growth plan names (the cap of the platform, the host keep, the commitment budget). Pure, from the plan of `nativeImageGrowthPlanV1` over the lane's peak. */
export type LaneFullVerdictV1=Readonly<{kind:"grows";targetBytes:number}|{kind:"capped";reason:string}>;
/** 🈵️ Decides the outlook of a full lane image (see `LaneFullVerdictV1`). The peak is the larger of what the image holds and the recorded need, so a lane that reached its capacity always aims above it. */
export function laneFullVerdictV1(platform:string,rule:Parameters<typeof nativeImageGrowthPlanV1>[1],facts:Readonly<{capacityBytes:number;usedBytes:number;peakBytes:number;hostFreeBytes:number;hostKeepBytes?:number;budgetRoomBytes?:number}>):LaneFullVerdictV1{
 const plan=nativeImageGrowthPlanV1(platform,rule,{capacityBytes:facts.capacityBytes,usedBytes:facts.usedBytes,measuredBytes:Math.max(facts.peakBytes,facts.usedBytes),hostFreeBytes:facts.hostFreeBytes,held:false,...facts.hostKeepBytes===undefined?{}:{hostKeepBytes:facts.hostKeepBytes},...facts.budgetRoomBytes===undefined?{}:{budgetRoomBytes:facts.budgetRoomBytes}});
 return plan.action==="grow"||plan.action==="wait"?{kind:"grows",targetBytes:plan.targetBytes}:{kind:"capped",reason:plan.reason};
}
/** 🔌️ The real ports of a cache root, `undefined` when it has no image to shard (another platform, no home image, no budget, a rule that is off). The rule comes from the repository cache budget, read when needed so the lane module never depends on it statically. */
export async function laneImagePortsV1(root:string,signal?:AbortSignal,commitmentBytes?:number,crateKey?:string):Promise<LaneImagePortsV1|undefined>{
 if(process.platform!=="darwin")return undefined;
 const directory=laneImageDirectoryV1(root);if(directory===undefined)return undefined;
 const marker=[".cache","b","r"].join(sep),at=directory.lastIndexOf(sep+marker);if(at<0||directory.slice(at+1)!==marker)return undefined;
 try{
  const{repositoryCacheBudgetV1}=await import("../../../../../💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),{machineHostFloorV1,machinePolicyV1}=await import("../../../../../💾️storage/🧾️ledger/🎚️machine/🟦️.ts"),floor=(total:number)=>machineHostFloorV1(machinePolicyV1(),total),rule=repositoryCacheBudgetV1(directory.slice(0,at)).partitions.rust.imageShards;
  if(rule===undefined||!rule.enabled)return undefined;
  const run=(program:string,args:readonly string[])=>new Promise<string>((done,fail)=>execFile(program,[...args],{encoding:"utf8",maxBuffer:8*1048576,timeout:NATIVE_IMAGE_SHARD_V1.commandMilliseconds,...signal===undefined?{}:{signal}},(error,stdout,stderr)=>error?fail(Error(`${program} ${args.join(" ")}: ${stderr||error.message}`)):done(stdout)));
  return Object.freeze({directory,rule:rule as NativeImageShardRuleV1,platform:process.platform,hostFreeBytes:()=>{const row=statfsSync(dirname(directory));return Number(row.bavail)*row.bsize;},hostFreeFloorBytes:()=>{const row=statfsSync(dirname(directory));return floor(Number(row.blocks)*row.bsize);},...commitmentBytes===undefined?{}:{budgetBytes:()=>commitmentBytes+laneIdleSlackBytesV1(root,directory,Date.now())},hostCommitment:()=>{const row=measureNativeImageCommitmentV1(dirname(directory));return{committedBytes:row.committedBytes,ceilingBytes:row.ceilingBytes};},measuredNeedBytes:()=>laneMeasuredNeedBytesV1(root,crateKey),laneSizes:()=>laneSizesV1(root,crateKey),holders:async layout=>(await createNativeImageDetachPortV1({cwd:directory,run,sleep:async()=>{}}).holders(layout.mountPoint)).length,measure:layout=>{const row=measureNativeImageShardV1(layout);return row&&{usedBytes:row.usedBytes,hostFreeBytes:row.hostFreeBytes};},now:Date.now,lifecycle:createNativeImageShardDarwinLifecycleV1({run}),lock:laneImageLockV1(root,signal)});
 }catch{return undefined;}
}
