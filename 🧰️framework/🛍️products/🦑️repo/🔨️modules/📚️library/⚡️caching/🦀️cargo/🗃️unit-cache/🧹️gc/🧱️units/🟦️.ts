import {existsSync,lstatSync,readdirSync,rmdirSync} from "node:fs";
import {basename,dirname,join,relative} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {tryLockSessionV1,type IncrementalLockPortV1} from "../🌱️incremental/🟦️.ts";
import {buildHolderLiveV1} from "../🫀️holder/🟦️.ts";
import {laneLiveV1,lanePathsV1} from "../../🧵️lane/🟦️.ts";
import {allocatedCargoBytesV1,removeCargoGcTreeV1,type CargoGcControlV1} from "../📏️tree/🟦️.ts";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 📐️ The authored policy of the stale build unit collector.
 * @see ./🧬️schema/🔣️.json */
export const UNIT_POLICY_V1=policy;
/** 🧱️ One Cargo build unit (`build/<crate>/<hash>`): its allocated bytes, last use and whether it has a live holder (`buildHolderLiveV1`: a running compiler holds its lock, its profile's build lease or its lane). */
export type BuildUnitV1=Readonly<{path:string;crate:string;profile:string;bytes:number;lastUsedMs:number;held:boolean}>;
/** 🧭️ Why a unit is removed. */
export type UnitReasonV1="superseded"|"pressure";
/** 🗑️ One planned removal. */
export type UnitRemovalV1=Readonly<{path:string;bytes:number;reason:UnitReasonV1}>;
/** 📋️ The measured state the pure planner decides on: `needBytes` is what the image must free to reach its free-space target, `budgetBytes` the most the units may hold together (what they hold beyond it is needed too). */
export type UnitInputV1=Readonly<{needBytes:number;budgetBytes?:number;nowMs:number;units:readonly BuildUnitV1[]}>;
/** 🎫️ A profile of one build directory as the build lease names it. */
export type UnitProfileV1=Readonly<{buildDirectory:string;profileDirectory:string;profile:string}>;
/** 🔌️ Probes the exclusive build lease of a profile without waiting: `busy` while compilers hold it, else a lease this process holds until released. */
export type ProfileLeasePortV1=(profile:UnitProfileV1)=>Promise<"busy"|Readonly<{release():void}>>;

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,byAge=(left:BuildUnitV1,right:BuildUnitV1)=>left.lastUsedMs-right.lastUsedMs||order(left.path,right.path);

/** 🎯️ The units of a profile that compilers hold which belong to an active build and so stay: the newest `keepNewestPerCrate` variants of every crate and every unit used within `activeBuildMilliseconds` (a build touches the fingerprint of every unit it plans, fresh ones included, see `usedAtMs`); any other unit of the profile is not planned by a build that is running and goes like in a profile nobody holds. Pure and language-agnostic.
 * @see ./🧬️schema/🔣️.json */
export function unitsCoveredV1(units:readonly Pick<BuildUnitV1,"path"|"lastUsedMs">[],nowMs:number,rules:Pick<typeof policy,"keepNewestPerCrate"|"activeBuildMilliseconds">=policy):ReadonlySet<string>{
 const newest=newestUnitsV1(units,rules);
 return new Set(units.filter(unit=>newest.has(unit.path)||nowMs-unit.lastUsedMs<rules.activeBuildMilliseconds).map(unit=>unit.path));
}

/** 🥇️ The newest `keepNewestPerCrate` units of every crate in every profile: the variants a build uses now. */
export function newestUnitsV1<T extends Pick<BuildUnitV1,"path"|"lastUsedMs">>(units:readonly T[],rules:Pick<typeof policy,"keepNewestPerCrate">=policy):ReadonlySet<string>{
 const groups=new Map<string,T[]>();for(const unit of units){const key=dirname(unit.path);groups.set(key,[...(groups.get(key)??[]),unit]);}
 return new Set([...groups.values()].flatMap(group=>group.sort((left,right)=>right.lastUsedMs-left.lastUsedMs||order(left.path,right.path)).slice(0,rules.keepNewestPerCrate).map(unit=>unit.path)));
}

/** 🛡️ Whether a unit may go: never one with a live holder (a unit of a profile that compilers hold is held, however idle), any other unit once it is older than the minimum age. */
export function unitEligibleV1(unit:BuildUnitV1,nowMs:number,rules:Pick<typeof policy,"minimumAgeMilliseconds">=policy):boolean{
 return !unit.held&&nowMs-unit.lastUsedMs>=rules.minimumAgeMilliseconds;
}

/** 🧮️ Plans which units to remove, oldest first: units that newer variants of their crate supersede and that were idle for a day, then (while `needBytes` are not freed) the oldest eligible units that are not among the newest variants, then the oldest newest-variants of idle profiles. Pure and deterministic. */
export function planUnitsV1(input:UnitInputV1,override:Partial<typeof policy>={}):UnitRemovalV1[]{
 const rules={...policy,...override},ordered=[...input.units].sort(byAge),newest=newestUnitsV1(input.units,rules),removed=new Map<string,UnitRemovalV1>();
 const eligible=(unit:BuildUnitV1)=>unitEligibleV1(unit,input.nowMs,rules),take=(unit:BuildUnitV1,reason:UnitReasonV1)=>{removed.set(unit.path,{path:unit.path,bytes:unit.bytes,reason});};
 for(const unit of ordered)if(eligible(unit)&&!newest.has(unit.path)&&input.nowMs-unit.lastUsedMs>rules.idleMilliseconds)take(unit,"superseded");
 let freed=[...removed.values()].reduce((sum,row)=>sum+row.bytes,0);
 const needBytes=Math.max(input.needBytes,input.budgetBytes===undefined?0:input.units.reduce((sum,unit)=>sum+unit.bytes,0)-input.budgetBytes);
 for(const wanted of[false,true])for(const unit of ordered){if(freed>=needBytes)break;if(newest.has(unit.path)===wanted&&eligible(unit)&&!removed.has(unit.path)){take(unit,"pressure");freed+=unit.bytes;}}
 return ordered.filter(unit=>removed.has(unit.path)).map(unit=>removed.get(unit.path)!);
}

/** 🧩️ What a collection knows of the images the units live in when the partition is split (see `🧩️shard`): which image holds a unit path, the bytes each image must free to reach its free-space target (`0` below its high-water mark), the most its units may hold and whether it reached its target. Every image is kept below its own water marks, not only the sum of them. */
export type UnitImagesV1=Readonly<{of:(path:string)=>string;needBytes:(id:string)=>number;budgetBytes?:(id:string)=>number|undefined;satisfied:(id:string)=>boolean}>;

/** 🧮️ Plans the removals of a split partition: the units of every image are planned on the need and the budget of that image alone (so a full image sheds its own least recently used units and an image with room loses nothing for it), the removals of all images oldest first. With one image it is `planUnitsV1`. Pure and deterministic. */
export function planUnitsByImageV1(input:Readonly<{nowMs:number;units:readonly BuildUnitV1[];images:UnitImagesV1}>,override:Partial<typeof policy>={}):UnitRemovalV1[]{
 const groups=new Map<string,BuildUnitV1[]>();for(const unit of input.units){const id=input.images.of(unit.path);groups.set(id,[...(groups.get(id)??[]),unit]);}
 const rows=[...groups].sort((left,right)=>order(left[0],right[0])).flatMap(([id,units])=>planUnitsV1({needBytes:input.images.needBytes(id),budgetBytes:input.images.budgetBytes?.(id),nowMs:input.nowMs,units},override)),age=new Map(input.units.map(unit=>[unit.path,unit]));
 return rows.sort((left,right)=>age.get(left.path)!.lastUsedMs-age.get(right.path)!.lastUsedMs||order(left.path,right.path));
}

const directoriesOf=(path:string):string[]=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>join(path,entry.name));}catch{return [];}};
const isDirectory=(path:string):boolean=>{try{return lstatSync(path).isDirectory();}catch{return false;}};
const modified=(path:string):number=>{try{return lstatSync(path).mtimeMs;}catch{return 0;}};
const usedAtMs=(directory:string):number=>{let latest=modified(directory);try{for(const name of readdirSync(directory))latest=Math.max(latest,modified(join(directory,name)));}catch{}return latest;};

/** 🗺️ Every profile directory of the compiler image that holds a `build` directory, with the build directory its lease is named by: the declared bases, at most `maximumProfileDepth` levels of profile and target-triple directories below each. Bounded directory reads only. */
export function unitProfilesV1(store:string):UnitProfileV1[]{
 const found=new Map<string,UnitProfileV1>();
 for(const pattern of policy.bases){
  let current=[store];
  for(const segment of pattern.split("/"))current=current.flatMap(dir=>segment==="*"?directoriesOf(dir):isDirectory(join(dir,segment))?[join(dir,segment)]:[]);
  for(const base of current){
   let level=[base];
   for(let depth=0;depth<policy.maximumProfileDepth;depth++){level=level.flatMap(directoriesOf).filter(dir=>basename(dir)!==policy.directory&&!(pattern===policy.laneBase&&depth===0&&policy.excludedLaneChildren.includes(basename(dir))));for(const dir of level)if(isDirectory(join(dir,policy.directory)))found.set(dir,{buildDirectory:base,profileDirectory:dir,profile:basename(dir)});}
  }
 }
 return [...found.values()].sort((left,right)=>order(left.profileDirectory,right.profileDirectory));
}

/** 🫀️ Whether a profile's lane has a live holder: a build that entered the lane and may still read or report any unit below it, however long a unit has been idle. Probed anew at every use, never cached. */
export type ProfileLivePortV1=(profile:UnitProfileV1)=>boolean;

/** 🫀️ The real port: the lane of a profile is the `u/l/<id>` directory above it; a profile outside a lane is never live. */
export function profileLivePortV1(store:string,nowMs:number=Date.now()):ProfileLivePortV1{
 const root=join(store,"u");
 return profile=>{
  const parts=relative(root,profile.profileDirectory).replaceAll("\\","/").split("/");
  if(parts[0]!==policy.laneBase.split("/")[1]||parts.length<2||parts[1]===".."||!/^[a-z0-9]{1,12}$/u.test(parts[1]!))return false;
  return laneLiveV1(lanePathsV1(root,parts[1]!),nowMs);
 };
}

const turn=()=>new Promise<void>(done=>setImmediate(done));
/** 🔭️ Measures every unit below the profiles: allocated bytes, the last time anything in it changed (never an access time, which measuring itself would renew) and whether it has a live holder (`buildHolderLiveV1`): a running compiler holds its lock, compilers hold the build lease of its profile or a live build holds its lane. Without a lock backend a unit counts as held until the held grace has passed. */
export async function measureUnitsV1(profiles:readonly UnitProfileV1[],options:CargoGcControlV1&Readonly<{nowMs:number;busy:(profile:UnitProfileV1)=>boolean;live?:ProfileLivePortV1;lock?:IncrementalLockPortV1}>):Promise<BuildUnitV1[]>{
 const lock=options.lock??tryLockSessionV1,rows:BuildUnitV1[]=[];let done=0;
 for(const [index,profile] of profiles.entries()){
  const busy=options.busy(profile),live=options.live?.(profile)??false;
  for(const crate of directoriesOf(join(profile.profileDirectory,policy.directory))){
   options.signal?.throwIfAborted();
   const pending:Array<Omit<BuildUnitV1,"held">&Readonly<{lock:"held"|"free"|"missing"|"unsupported"}>>=[];
   for(const path of directoriesOf(crate)){
    if(!isDirectory(join(path,policy.marker)))continue;
    const used=Math.floor(Math.max(modified(path),usedAtMs(join(path,policy.marker)),modified(join(path,policy.outputDirectory)),modified(join(path,policy.lockName)))),probe=lock(join(path,policy.lockName));
    if(typeof probe==="object")probe.release();
    const bytes=await allocatedCargoBytesV1(path,policy.maximumEntriesPerUnit,options);
    pending.push({path,crate:basename(crate),profile:profile.profile,bytes,lastUsedMs:used,lock:typeof probe==="object"?"free":probe});
    if(++done%policy.yieldEvery===0)await turn();
   }
   const covered=busy?unitsCoveredV1(pending,options.nowMs):new Set<string>();
   for(const{lock:state,...unit}of pending)rows.push({...unit,held:buildHolderLiveV1({lane:live,lease:busy&&covered.has(unit.path),lock:state,idleMilliseconds:options.nowMs-unit.lastUsedMs},policy.heldGraceMilliseconds)});
  }
  await options.progress?.(index+1,profiles.length,"scan",profile.profileDirectory);
 }
 return rows;
}

/** 🔎️ The lease directory of a store: the first ancestor that carries the repository's lease data, undefined when none does (every profile then counts as held). */
export function leaseDirectoryOfStoreV1(store:string):string|undefined{
 let current=store;for(let step=0;step<policy.lease.maximumAncestors;step++){const candidate=join(current,policy.lease.directoryFromStore);if(existsSync(candidate))return candidate;const parent=dirname(current);if(parent===current)return undefined;current=parent;}
 return undefined;
}

/** 🔒️ The build lease resource of a profile, exactly as Cargo builds name it. */
export function profileLeaseResourceV1(profile:UnitProfileV1):string{return JSON.stringify([policy.lease.namespace,profile.buildDirectory.replaceAll("\\","/").normalize("NFC"),profile.profile]);}

/** 👤️ The real port: a non-waiting exclusive try on the profile's build lease, so a compiler that starts meanwhile queues behind the collection; without a lease directory every profile counts as held. */
export function profileLeasePortV1(store:string,signal?:AbortSignal):ProfileLeasePortV1{
 const directory=leaseDirectoryOfStoreV1(store);
 return async profile=>{
  if(directory===undefined)return "busy";
  try{const lease=await acquireResourceLease({directory,resource:profileLeaseResourceV1(profile),mode:"exclusive",signal:signal??new AbortController().signal,timeoutMs:0});return {release:()=>lease.release()};}
  catch(error){if(/timed out/iu.test(String((error as Error).message)))return "busy";throw error;}
 };
}

/** 📊️ What a collection did or, in a dry run, would do. */
export type UnitResultV1=Readonly<{dryRun:boolean;total:number;removed:readonly UnitRemovalV1[];freedBytes:number;held:number;kept:number;busyProfiles:number;profiles:Readonly<Record<string,number>>}>;
/** 💬️ One localized line with its placeholders filled. */
export function unitTextV1(key:keyof typeof messages,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
const size=(bytes:number)=>bytes>=1073741824?(bytes/1073741824).toFixed(1)+" GiB":(bytes/1048576).toFixed(0)+" MiB";

/** 🧹️ Measures, plans and (unless `dryRun`) removes stale build units. A unit with a live holder is never touched, whatever its age or the budget: not a unit of a lane with a live holder, not a unit a running build uses of a profile whose build lease compilers hold (Cargo reuses a unit without locking it, so a lock alone does not cover the units a running build has planned; it touches the fingerprint of every unit it plans, so use shows in `lastUsedMs`), not a unit whose compiler lock is held. Profiles are probed first: a profile whose build lease is free is held exclusively while its units go (compilers that start meanwhile queue); a profile compilers hold keeps only the units its builds use (`unitsCoveredV1`: newest variants and units used within the active build window) and loses the others, each under its own lock; every removal re-takes the unit's own lock and holds it while the unit goes, and a unit a build touched since it was measured stays. Pressure removals stop once `satisfied()` says the image reached its free-space target. */
export async function collectUnitsV1(store:string,options:CargoGcControlV1&Readonly<{dryRun:boolean;needBytes:()=>number;budgetBytes?:number;satisfied?:()=>boolean;images?:UnitImagesV1;nowMs?:number;lock?:IncrementalLockPortV1;leases?:ProfileLeasePortV1;live?:ProfileLivePortV1;report?:Readonly<{language:"en"|"de";write:(line:string)=>void}>}>):Promise<UnitResultV1>{
 const nowMs=options.nowMs??Date.now(),lock=options.lock??tryLockSessionV1,leases=options.leases??profileLeasePortV1(store,options.signal),live=options.live??profileLivePortV1(store,nowMs),report=options.report,say=(key:keyof typeof messages,variables:Readonly<Record<string,string|number>>={}):void=>{if(report)report.write(unitTextV1(key,report.language,variables));};
 say("scan",{root:store});
 const profiles=unitProfilesV1(store),busy=new Set<string>();
 for(const profile of profiles){const probe=await leases(profile);if(probe==="busy"){busy.add(profile.profileDirectory);say("busyProfile",{profile:profile.profile,directory:profile.buildDirectory});}else probe.release();}
 const units=await measureUnitsV1(profiles,{nowMs,busy:profile=>busy.has(profile.profileDirectory),lock,live,signal:options.signal,progress:options.progress}),usedBefore=new Map(units.map(unit=>[unit.path,unit.lastUsedMs])),images=options.images,imageOf=(path:string)=>images?.of(path)??"",plan=images?planUnitsByImageV1({nowMs,units,images}):planUnitsV1({needBytes:options.needBytes(),budgetBytes:options.budgetBytes,nowMs,units}),excessOf=(id:string)=>{const budget=images?images.budgetBytes?.(id):options.budgetBytes;return budget===undefined?0:Math.max(0,units.filter(unit=>imageOf(unit.path)===id).reduce((sum,unit)=>sum+unit.bytes,0)-budget);},removed:UnitRemovalV1[]=[],crates=new Set<string>(),profileByDirectory=new Map(profiles.map(profile=>[profile.profileDirectory,profile])),profileOf=(path:string):UnitProfileV1=>profileByDirectory.get(dirname(dirname(dirname(path))))??{buildDirectory:dirname(dirname(dirname(path))),profileDirectory:dirname(dirname(dirname(path))),profile:basename(dirname(dirname(dirname(path))))};
 const reason=(row:UnitRemovalV1)=>report?unitTextV1(`reason.${row.reason}` as keyof typeof messages,report.language,{hours:Math.round(policy.idleMilliseconds/3600000)}):"";
 let held=units.filter(unit=>unit.held).length;
 if(options.dryRun){
  for(const row of plan){removed.push(row);if(removed.length<=policy.reportedRemovals)say("would",{path:row.path,bytes:size(row.bytes),reason:reason(row)});}
 }else{
  const groups=new Map<string,UnitRemovalV1[]>();for(const row of plan){const key=profileOf(row.path).profileDirectory;groups.set(key,[...(groups.get(key)??[]),row]);}
  let done=0;
  for(const[directory,rows]of groups){
   const profile=profileOf(join(directory,policy.directory,"_","_"));if(live(profile))continue;
   const taken=await leases(profile),exclusive=taken==="busy"?undefined:taken;
   try{
    for(const row of rows){
     options.signal?.throwIfAborted();
     const image=imageOf(row.path);
     if(row.reason==="pressure"&&(images?images.satisfied(image):options.satisfied?.())&&removed.filter(done=>imageOf(done.path)===image).reduce((sum,done)=>sum+done.bytes,0)>=excessOf(image))continue;
     const unitLock=lock(join(row.path,policy.lockName));
     if(unitLock==="held"){held++;say("held",{path:row.path});continue;}
     if(Math.floor(usedAtMs(join(row.path,policy.marker)))>(usedBefore.get(row.path)??0)){if(typeof unitLock==="object")unitLock.release();held++;say("held",{path:row.path});continue;}
     try{await removeCargoGcTreeV1(row.path,options);removed.push(row);crates.add(dirname(row.path));if(removed.length<=policy.reportedRemovals)say("removed",{path:row.path,bytes:size(row.bytes),reason:reason(row)});}
     finally{if(typeof unitLock==="object")unitLock.release();}
     await options.progress?.(++done,plan.length,"remove",row.path);if(done%policy.yieldEvery===0)await turn();
    }
   }finally{exclusive?.release();}
  }
  for(const crate of crates)try{if(!readdirSync(crate).length)rmdirSync(crate);}catch{}
 }
 const freedBytes=removed.reduce((sum,row)=>sum+row.bytes,0);
 say(options.dryRun?"summaryPlan":"summaryApply",{count:removed.length,total:units.length,bytes:size(freedBytes),held,kept:units.length-removed.length-held});
 const gone=new Set(removed.map(row=>row.path)),totals:Record<string,number>={};for(const unit of units)if(!gone.has(unit.path)||options.dryRun){const name=relative(store,profileOf(unit.path).profileDirectory).replaceAll("\\","/");totals[name]=(totals[name]??0)+unit.bytes;}
 return Object.freeze({dryRun:options.dryRun,total:units.length,removed:Object.freeze(removed),freedBytes,held,kept:units.length-removed.length-held,busyProfiles:busy.size,profiles:Object.freeze(totals)});
}
