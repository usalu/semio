import {createHash,randomBytes} from "node:crypto";
import {closeSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";
import {basename,join,relative,resolve,sep} from "node:path";
import {processAliveV1,tryClaimV1} from "../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {ownProcessIdentityV1,parseProcessIdentityV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout,idPattern=/^[a-z0-9]{1,12}$/u;
/** 🪪️ Who a lane belongs to: the agent session address and the agent, recorded in the lane's metadata so caches and cleanup can be attributed. */
export type LaneOwnerV1=Readonly<{session:string;agent:string}>;
/** 🪪️ Names a target namespace by its complete authored physical layout. */
export function laneTargetNamespaceV1(input:Readonly<{version:number;build:string;target:string}>):string{return input.target+"-"+createHash("sha256").update(JSON.stringify({version:input.version,build:input.build,target:input.target})).digest("hex").slice(0,8)}
/** 🗺️ The directories of one lane inside a cache root; compact on purpose so Windows paths stay short. */
export function lanePathsV1(root:string,id:string){
 if(!idPattern.test(id))throw Error("Invalid Cargo lane identifier");
 const base=join(root,layout.lanes,id);
 return Object.freeze({id,base,target:join(base,laneTargetNamespaceV1({version:layout.targetIdentityVersion,build:".",target:layout.target})),build:base,temporaries:join(base,layout.compilerTemporaries),live:join(base,layout.live),stamps:join(base,layout.stamps),metadata:join(base,layout.metadata),size:join(base,layout.size)});
}

/** 🧹️ Hands rustc the short temporary directory of a lane for its codegen files (`-Z temps-dir`): their names carry the crate name up to three times, so below the unit directory they overran the 256 character bound that the path envelope (`📏️path-envelope`) now models at this place. Only compilations whose arguments already carry an unstable flag (the repository's rustflags carry `-Z threads=0`, so its nightly toolchain takes them) get it; probes and a stable toolchain get nothing. */
export function compilerTemporaryArgumentsV1(directory:string,args:readonly string[]):readonly string[]{
 if(!args.includes("--crate-name")||!args.includes("--out-dir")||!args.some(argument=>argument==="-Z"||argument.startsWith("-Z")))return[];
 mkdirSync(directory,{recursive:true});return["-Z","temps-dir="+directory];
}

/** 🔑️ The affinity key of a crate set: the same sorted, de-duplicated names always give the same key, an empty set (a whole-workspace command) gives `*`. */
export function crateKeyV1(crates:readonly string[]):string{
 const names=[...new Set(crates)].sort();return names.length===0?"*":createHash("sha256").update(JSON.stringify(names)).digest("hex").slice(0,12);
}

/** 📖️ A lane's metadata file; `crateKey` is the affinity key of the crate set the lane's incremental state was last built for. */
export type LaneRecordV1=Readonly<{version:1;id:string;createdMs:number;session?:string;agent?:string;crateKey?:string;crates?:readonly string[];holdUntilMs?:number;usedMs?:number;flavour?:string}>;

/** 📚️ Reads a lane's metadata file. */
export function readLaneV1(root:string,id:string):LaneRecordV1|undefined{try{return JSON.parse(readFileSync(lanePathsV1(root,id).metadata,"utf8"));}catch{return undefined;}}

/** 📏️ The size last measured for a lane by the collector, with the part its Cargo incremental sessions hold; `undefined` until it was measured. */
export function readLaneSizeV1(root:string,id:string):Readonly<{bytes:number;incrementalBytes:number;measuredMs:number;temporariesBytes?:number}>|undefined{
 try{const row=JSON.parse(readFileSync(lanePathsV1(root,id).size,"utf8"));return Number.isSafeInteger(row?.bytes)&&Number.isSafeInteger(row?.incrementalBytes)&&Number.isSafeInteger(row?.measuredMs)?{bytes:row.bytes,incrementalBytes:row.incrementalBytes,measuredMs:row.measuredMs,...Number.isSafeInteger(row.temporariesBytes)?{temporariesBytes:row.temporariesBytes}:{}}:undefined;}catch{return undefined;}
}

const writeAtomicV1=(path:string,text:string):void=>{const temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,text);renameSync(temporary,path);};

/** 🧮️ Records the measured size of a lane and the part its incremental sessions hold next to its metadata (a file of its own, so a measurement never overwrites a concurrent acquisition's record). */
export function recordLaneSizeV1(root:string,id:string,bytes:number,incrementalBytes:number,nowMs:number,temporariesBytes?:number):void{try{writeAtomicV1(lanePathsV1(root,id).size,JSON.stringify({bytes,incrementalBytes,measuredMs:nowMs,...temporariesBytes===undefined?{}:{temporariesBytes}}));}catch{}}

/** 🚪️ Registers one running command in its lane and records who uses it for which crate set. The marker is kept fresh by a heartbeat and removed on release; garbage collection never touches a lane with a fresh marker. */
export function enterLaneV1(root:string,id:string,owner?:LaneOwnerV1,crates?:readonly string[],flavour?:string):Readonly<{paths:ReturnType<typeof lanePathsV1>;release():void}>{
 const paths=lanePathsV1(root,id);
 for(const directory of[paths.target,paths.build,paths.live,paths.stamps])mkdirSync(directory,{recursive:true});
 const before=readLaneV1(root,id),names=crates===undefined?before?.crates:[...new Set(crates)].sort(),who=owner??(before?.session===undefined?undefined:{session:before.session,agent:before.agent!});
 const kept=flavour??before?.flavour;
 writeAtomicV1(paths.metadata,JSON.stringify({version:1,id,createdMs:before?.createdMs??Date.now(),usedMs:Date.now(),...kept===undefined?{}:{flavour:kept},...who===undefined?{}:{session:who.session,agent:who.agent},...names===undefined?{}:{crateKey:crateKeyV1(names),crates:names.slice(0,policy.pool.listedCrates)}}));
 const touch=()=>{try{const now=new Date(),record=readLaneV1(root,id);if(record!==undefined)writeAtomicV1(paths.metadata,JSON.stringify({...record,usedMs:now.getTime()}));utimesSync(paths.metadata,now,now);}catch{}};
 const marker=join(paths.live,process.pid+"-"+randomBytes(3).toString("hex")),write=()=>writeAtomicV1(marker,JSON.stringify({...ownProcessIdentityV1(),startedMs:Date.now()}));write();
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(marker,now,now);}catch{}},policy.slot.heartbeatMilliseconds);beat.unref?.();
 let released=false;
 return Object.freeze({paths,release(){if(released)return;released=true;clearInterval(beat);rmSync(marker,{force:true});touch();}});
}

/** 🫀️ Whether any marker of the lane belongs to a live process, however old its heartbeat: a holder whose event loop is busy for long (a provenance snapshot) is still a holder; only a dead one frees the lane. */
export function laneLiveV1(paths:ReturnType<typeof lanePathsV1>,nowMs:number):boolean{
 let names:string[];try{names=readdirSync(paths.live);}catch{return false;}
 for(const name of names){
  try{const holder=parseProcessIdentityV1(JSON.parse(readFileSync(join(paths.live,name),"utf8")));if(holder!==undefined&&processAliveV1(holder))return true;rmSync(join(paths.live,name),{force:true});}catch{}
 }
 return false;
}

function treeBytes(path:string,limit:{files:number},skip?:string):number{
 let total=0,entries;try{entries=readdirSync(path,{withFileTypes:true});}catch{return 0;}
 for(const entry of entries){
  if(limit.files--<=0)return total;
  const child=join(path,entry.name);
  if(entry.isDirectory()){if(child!==skip)total+=treeBytes(child,limit);}else if(entry.isFile()){try{total+=statSync(child).size;}catch{}}
 }
 return total;
}

/** 📁️ The incremental-compilation directories of a lane's build directory (`<profile>/incremental` and `<triple>/<profile>/incremental`). */
export function incrementalDirectoriesV1(build:string):string[]{
 const found:string[]=[];
 const walk=(directory:string,depth:number)=>{let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}for(const entry of entries){if(!entry.isDirectory()||depth===0&&[layout.target,laneTargetNamespaceV1({version:layout.targetIdentityVersion,build:".",target:layout.target}),layout.live,layout.stamps].includes(entry.name))continue;const path=join(directory,entry.name);if(entry.name==="incremental")found.push(path);else if(depth<2)walk(path,depth+1);}};
 walk(build,0);return found;
}

/** 🛡️ Whether a lane is a retained product lane: a stable product workspace used within `retainMilliseconds` belongs to a release generation that is still open, so neither the pool nor the collector evicts it for its count or byte share (only free-space pressure may, after every other lane). */
export function laneRetainedV1(id:string,lastUsedMs:number,nowMs:number,holdUntilMs?:number):boolean{return id.startsWith(policy.pool.product.prefix)&&nowMs-lastUsedMs<policy.pool.product.retainMilliseconds||holdUntilMs!==undefined&&nowMs<holdUntilMs;}

/** 🕰️ When a lane was last used: the later of its recorded use stamp (`usedMs`, content that a restore or copy keeps) and the modification time of its metadata (heartbeat of older records), else the modification time of the lane directory, else `0`. */
export function laneUsedMsV1(root:string,id:string):number{
 const paths=lanePathsV1(root,id),stamp=(path:string):number=>{try{return Math.floor(statSync(path).mtimeMs);}catch{return 0;}};
 const record=readLaneV1(root,id);return Math.max(Number.isSafeInteger(record?.usedMs)?record!.usedMs!:0,stamp(paths.metadata),record===undefined?stamp(paths.base):0);
}

/** 🌡️ Whether a lane of a declared build flavour was used within `recentMilliseconds`: the lanes a build of that flavour needs are not shed for the pool's soft bounds, only by free-space pressure and never when the starting build requests them. */
export function laneRecentV1(flavour:string|undefined,lastUsedMs:number,nowMs:number):boolean{return flavour!==undefined&&nowMs-lastUsedMs<policy.pool.recentMilliseconds;}

/** 🏷️ The build flavour a shared compiler build directory stands for: its own directory name (one per build mode, see the compiler layout), `undefined` when the name is no flavour. Pure. */
export function laneFlavourV1(sharedBuild:string):string|undefined{const name=basename(sharedBuild);return new RegExp(policy.pool.flavourPattern,"u").test(name)?name:undefined;}

/** ⏳️ Holds a lane for the command that replaces the one using it (a pass the host killed a unit of restarts in the lane it compiled in, which keeps the Cargo fingerprints, incremental state and identity stamps of every unit the pass finished): until `restartHoldMilliseconds` have passed the lane is retained (see `laneRetainedV1`), and the next entry into the lane clears the hold. Best effort and silent for a lane without metadata. */
export function holdLaneV1(root:string,id:string,nowMs:number):boolean{
 try{const before=readLaneV1(root,id);if(before===undefined)return false;writeAtomicV1(lanePathsV1(root,id).metadata,JSON.stringify({...before,holdUntilMs:nowMs+policy.pool.restartHoldMilliseconds}));return true;}catch{return false;}
}

/** ⏳️ Holds the pooled lane whose build directory is `buildDirectory` under `cacheRoot` (see `holdLaneV1`); `false` for any other directory, such as the default lane's or a fresh generation's. */
export function holdLaneOfBuildDirectoryV1(cacheRoot:string,buildDirectory:string,nowMs:number):boolean{
 const lanes=join(resolve(cacheRoot),layout.lanes),parts=relative(lanes,resolve(buildDirectory)).split(sep);
 return parts.length===1&&parts[0]!==""&&resolve(lanePathsV1(resolve(cacheRoot),parts[0]!).build)===resolve(buildDirectory)&&!policy.pool.reserved.includes(parts[0]!)&&holdLaneV1(resolve(cacheRoot),parts[0]!,nowMs);
}

/** 🧬️ The identifiers of the lanes that exist in a cache root. */
export function laneIdsV1(root:string):string[]{try{return readdirSync(join(root,layout.lanes)).filter(name=>idPattern.test(name)).sort();}catch{return[];}}

/** 🔭️ Measures every lane of a cache root for the garbage collector (logical bytes; reflinked files count in full, which only makes collection earlier) and records each size beside the lane, where the pool reads it. */
export function listLanesV1(root:string,nowMs:number):{id:string;bytes:number;incrementalBytes:number;temporariesBytes:number;lastUsedMs:number;live:boolean;retained:boolean}[]{
 const base=join(root,layout.lanes);let ids:string[];try{ids=readdirSync(base).filter(name=>idPattern.test(name)&&!policy.pool.reserved.includes(name));}catch{return[];}
 return ids.map(id=>{const paths=lanePathsV1(root,id),limit={files:2_000_000},lastUsedMs=laneUsedMsV1(root,id);
  const incrementalBytes=incrementalDirectoriesV1(paths.build).reduce((sum,path)=>sum+treeBytes(path,limit),0),temporariesBytes=treeBytes(paths.temporaries,limit),bytes=temporariesBytes+treeBytes(paths.base,limit,paths.temporaries);recordLaneSizeV1(root,id,bytes,incrementalBytes,nowMs,temporariesBytes);
  return{id,bytes,incrementalBytes,temporariesBytes,lastUsedMs,live:laneLiveV1(paths,nowMs),retained:laneRetainedV1(id,lastUsedMs,nowMs,readLaneV1(root,id)?.holdUntilMs)||laneRecentV1(readLaneV1(root,id)?.flavour,lastUsedMs,nowMs)};});
}

/** 📐️ The logical bytes a lane holds right now, measured with the same bound as the collector's measurement; `0` for a lane that does not exist. */
export function measureLaneBytesV1(root:string,id:string):number{return treeBytes(lanePathsV1(root,id).base,{files:2_000_000});}

/** 🗑️ Removes a whole lane or only its incremental directories. It takes the lane's own claim first, so it can never race an acquirer, and refuses while the claim is held or a live marker exists; `beforeRemoval` runs for a whole lane under that claim, once the lane is known to be idle and before anything is deleted (the retention of what a holder that is gone left behind). */
export function removeLaneV1(root:string,id:string,scope:"lane"|"incremental",nowMs:number,beforeRemoval?:(paths:ReturnType<typeof lanePathsV1>)=>boolean|void):boolean{
 const paths=lanePathsV1(root,id),claim=tryClaimV1(join(root,layout.locks),"lane-"+id);if(!claim)return false;
 try{
  if(laneLiveV1(paths,nowMs))return false;
  if(scope==="lane"){if(beforeRemoval?.(paths)===false)return false;rmSync(paths.base,{recursive:true,force:true});}else for(const directory of incrementalDirectoriesV1(paths.build))rmSync(directory,{recursive:true,force:true});
  return true;
 }finally{claim.release();}
}
