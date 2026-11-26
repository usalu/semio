import {spawn,spawnSync} from "node:child_process";
import {processPidExistsV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts"
import {statSync} from "node:fs";
import {nativeImageVolumeUsedBytesSyncV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/⚖️availability/🟦️.ts";
import {chmod,lstat,mkdir,readdir,readFile,realpath,rename,rmdir,stat,unlink,utimes,writeFile} from "node:fs/promises";
import {nativeDirectoryProtocolV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {nativePartitionOfV1,nativeStatfsV1,nativeStorageBackendOfDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {isAbsolute,join,relative,resolve,sep} from "node:path";
import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";
import ticket from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧬️schema/📜️ticket/🔣️json/🟦️.ts";
import budgetPolicy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {CACHE_ROOT_IDS_V1,CACHE_RUNTIME_V1,CACHE_SIZE_FILE_V1,cacheAreaBudgetBytesV1,cacheAreaDirectoryV1,cacheRootDirectoryV1,cacheStrayNamesV1,cacheTextV1,classifyCachePathV1,formatCacheBytesV1,cacheRecencyMsV1,formatCacheReasonV1,planCacheCollectionV1,readCacheSizeLedgerV1,type CacheAreaRowV1,type CacheEntryRowV1,type CacheLanguageV1,type CacheLedgerV1,type CachePartitionBudgetsV1,type CachePartitionRowV1,type CacheRemovalV1,type CacheReportV1,type CacheRootIdV1,type CacheSizeLedgerV1,type CacheSizeRecordV1,type CacheSourceV1,type CacheTargetPlanV1,type CacheTargetV1,type CacheUnitV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {formatMachinePlanV1,planMachineBudgetV1,splitMachineBudgetV1,type MachinePlanV1,type MachinePolicyV1,type MachineSourceV1,type MachineUseV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import { SqliteDatabaseV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗃️sqlite/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const runtime=CACHE_RUNTIME_V1,protocol=ticket.definitions.Protocol.const,mount="m",tickets=new RegExp(protocol.ticketPattern,"u");

/** 🔌️ Everything the runner needs from its host: cancellation, a clock, progress, the exclusive non-blocking lease and the liveness probes; tests replace them. */
export type CacheRunnerPortsV1=Readonly<{
 signal:AbortSignal;
 now():number;
 progress(stage:string):Promise<void>;
 exclusive(resource:string):Promise<Readonly<{release():void}>|undefined>;
 alive(pid:number):boolean;
 sqliteFree(file:string):Promise<boolean>;
 atimeReliable?(directory:string):Promise<boolean>;
 strays?(workspace:string,ledger:CacheLedgerV1):Promise<readonly string[]>;
 mountPoints?():Promise<readonly string[]>;
 platform?:NodeJS.Platform;
 delegate?(owner:string,areaId:string,dryRun:boolean,language:CacheLanguageV1,context?:Readonly<{machine:MachinePlanV1|null;selected:boolean}>):Promise<string|undefined>;
}>;

/** 🧟️ Asks the operating system whether a process still exists; a process we may not signal still exists. */
export function cachePidAliveV1(pid:number):boolean{
 return processPidExistsV1(pid);
}

/** 🥀️ Asks git for the ignored directories of the checkout and keeps those that carry a cache root name below a member directory (never the root itself, never inside an ignored segment or an excluded directory, never a link); git absent, too slow or cancelled means none were found. */
export async function cacheStrayRootsV1(workspace:string,ledger:CacheLedgerV1,signal:AbortSignal):Promise<readonly string[]>{
 const rule=ledger.strays;if(rule===undefined||signal.aborted)return[];
 const names=new Set(cacheStrayNamesV1(ledger)),listing=await new Promise<string>(settle=>{
  const chunks:Buffer[]=[];let child:ReturnType<typeof spawn>|undefined;
  const finish=(text:string):void=>{clearTimeout(timer);signal.removeEventListener("abort",stop);settle(text);},stop=():void=>{child?.kill();finish("");},timer=setTimeout(stop,CACHE_GC_LIMITS_V1.strayScanMilliseconds);
  signal.addEventListener("abort",stop,{once:true});
  try{child=spawn("git",["-c","core.quotepath=off","ls-files","-z","--others","--ignored","--exclude-standard","--directory"],{cwd:workspace,stdio:["ignore","pipe","ignore"],windowsHide:true});}catch{finish("");return;}
  child.stdout!.on("data",(chunk:Buffer)=>{chunks.push(chunk);});
  child.on("error",()=>finish(""));child.on("close",code=>finish(code===0?Buffer.concat(chunks).toString("utf8"):""));
 });
 return Object.freeze(listing.split("\0").filter(entry=>entry.endsWith("/")).map(entry=>entry.slice(0,-1)).filter(entry=>{
  const parts=entry.split("/");
  return parts.length>1&&names.has(parts.at(-1)!)&&!parts.some(part=>rule.ignoredSegments.includes(part))&&!rule.excludedDirectories.includes(parts[0]!);
 }).sort());
}

/** ⛰️ Lists every mount point of the machine, so no directory that has a volume mounted below it is ever emptied; platforms without a mount table list none. */
export async function cacheMountPointsV1(platform:NodeJS.Platform=process.platform):Promise<readonly string[]>{
 if(platform==="win32")return[];
 if(platform==="linux")return Object.freeze((await readFile("/proc/self/mounts","utf8").catch(()=>"")).split("\n").map(line=>line.split(" ")[1]?.replace(/\\([0-7]{3})/gu,(_,code:string)=>String.fromCharCode(parseInt(code,8)))).filter((point):point is string=>!!point));
 const table=spawnSync("mount",{encoding:"utf8",timeout:10000}).stdout??"";
 return Object.freeze(table.split("\n").map(line=>/^.+? on (.+) \([^()]*\)$/u.exec(line)?.[1]).filter((point):point is string=>!!point));
}

/** 🔐️ Proves a SQLite lease file is unheld: an exclusive transaction without waiting only succeeds when no process holds a shared or exclusive lock; the probe gives the file its times back, so asking never counts as using it. */
export async function cacheSqliteFreeV1(file:string):Promise<boolean>{
 const {Database}={ Database: SqliteDatabaseV1 };let database:InstanceType<typeof Database>|undefined;
 const before=await lstat(file).catch(()=>undefined);
 try{database=new Database(file,{create:false,readwrite:true});database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");database.exec("ROLLBACK");return true;}
 catch(error){return !/locked|busy/iu.test(String((error as Error).message));}
 finally{try{database?.close();}catch{}if(before)await utimes(file,before.atime,before.mtime).catch(()=>undefined);}
}

/** 🕰️ Proves that reading a file moves its access time on the volume of a directory: a probe file is dated two days back and read once. A volume mounted without access times (`noatime`, the default of Windows) never does, so the recency of a cache that is only read there says nothing about whether somebody uses it. */
export async function cacheAtimeReliableV1(directory:string):Promise<boolean>{
 const path=join(directory,`.atime-probe-${process.pid}-${Date.now()}`),past=new Date(Date.now()-CACHE_GC_LIMITS_V1.atimeProbeAgeMilliseconds);
 try{await writeFile(path,"x");await utimes(path,past,past);await readFile(path);return (await stat(path)).atimeMs>past.getTime()+CACHE_GC_LIMITS_V1.atimeProbeToleranceMilliseconds;}
 catch{return false;}
 finally{await unlink(path).catch(()=>undefined);}
}

/** 🏭️ Completes the host ports with the portable defaults. */
export function cacheRunnerPortsV1(ports:Pick<CacheRunnerPortsV1,"signal"|"progress"|"exclusive">&Partial<CacheRunnerPortsV1>):CacheRunnerPortsV1{
 return Object.freeze({now:()=>Date.now(),alive:cachePidAliveV1,sqliteFree:cacheSqliteFreeV1,atimeReliable:cacheAtimeReliableV1,...ports});
}

type Ticker=()=>Promise<void>;
function ticker(ports:CacheRunnerPortsV1,stage:()=>string):Ticker{
 let last=-Infinity;
 return async()=>{ports.signal.throwIfAborted();const now=ports.now();if(now-last>=runtime.progressIntervalMs){last=now;await ports.progress(stage());ports.signal.throwIfAborted();}};
}

/** 📏️ What one bounded walk found: allocated bytes, newest file time, entries visited, and the lease files and queues inside. */
export type CacheMeasureV1=Readonly<{present:boolean;bytes:number;recencyMs:number;entries:number;partial:boolean;sqlite:readonly string[];queues:readonly string[]}>;

/** 🚶️ Walks one tree without following links, counts hardlinks once, takes recency from file times only (directory times only when no file exists), and stops at the entry limit or on cancellation. */
export async function measureCacheTreeV1(path:string,ports:CacheRunnerPortsV1,limit:number,stage:string=""):Promise<CacheMeasureV1>{
 let first;try{first=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{present:false,bytes:0,recencyMs:0,entries:0,partial:false,sqlite:[],queues:[]};throw error;}
 const allocated=(info:{size:number;blocks?:number}):number=>info.blocks?info.blocks*512:info.size,seen=new Set<string>(),sqlite:string[]=[],queues:string[]=[],tick=ticker(ports,()=>stage);
 if(!first.isDirectory())return{present:true,bytes:first.isSymbolicLink()?0:allocated(first),recencyMs:cacheRecencyMsV1(first,ports.now()),entries:1,partial:false,sqlite:path.endsWith(".sqlite")?[path]:[],queues:[]};
 let bytes=0,files=0,entries=0,partial=false;const stack=[path];let recency=0,directories=first.mtimeMs;
 while(stack.length){
  await tick();const directory=stack.pop()!;let children;
  try{children=await readdir(directory,{withFileTypes:true});}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT"||code==="ENOTDIR")continue;if(code==="EACCES"||code==="EPERM"){partial=true;continue;}throw error;}
  if(directory.endsWith(".queue")&&queues.length<runtime.sqliteProbesPerUnit)queues.push(directory);
  for(const child of children){
   if(++entries>limit){partial=true;stack.length=0;break;}
   if(entries%runtime.checkEvery===0)await tick();
   const full=join(directory,child.name);
   if(child.isDirectory()){stack.push(full);try{directories=Math.max(directories,(await lstat(full)).mtimeMs);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;directories=Math.max(directories,ports.now());recency=Math.max(recency,ports.now());}}continue;}
   let info;try{info=await lstat(full);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){partial=true;recency=Math.max(recency,ports.now());}continue;}
   if(info.isSymbolicLink())continue;
   if(info.nlink>1){const exact=Number.isSafeInteger(info.ino)?info:await lstat(full,{bigint:true}).catch(()=>info),key=exact.dev+":"+exact.ino;if(seen.has(key))continue;seen.add(key);}
   bytes+=allocated(info);files++;recency=Math.max(recency,cacheRecencyMsV1(info,ports.now()));
   if(child.name.endsWith(".sqlite")&&sqlite.length<runtime.sqliteProbesPerUnit)sqlite.push(full);
  }
 }
 return{present:true,bytes,recencyMs:files?recency:directories,entries,partial,sqlite,queues};
}

/** 🔒️ Names what holds a unit: a SQLite lease file with a live lock, or a queue ticket whose process is alive; nothing means nobody. */
export async function probeCacheHeldV1(measure:CacheMeasureV1,ports:CacheRunnerPortsV1):Promise<string|undefined>{
 for(const file of measure.sqlite){ports.signal.throwIfAborted();if(!await ports.sqliteFree(file))return "sqlite:"+file;}
 for(const queue of measure.queues){
  ports.signal.throwIfAborted();let names:string[];try{names=await readdir(queue);}catch{continue;}
  for(const name of names)if(tickets.test(name)){const pid=Number.parseInt(name.split("-")[1]!,protocol.pidBase);if(Number.isSafeInteger(pid)&&pid>0&&ports.alive(pid))return "pid:"+pid;}
 }
 return undefined;
}

const exists=async(path:string):Promise<boolean>=>lstat(path).then(()=>true,()=>false);

/** 🧮️ Reads occupied volume bytes even when the mounted filesystem reports writable headroom limited by the host. */
export function mountedPartitionUsageV1(mountPoint:string,volume:Parameters<typeof nativeImageVolumeUsedBytesSyncV1>[1],platform:NodeJS.Platform,readUsage=nativeImageVolumeUsedBytesSyncV1){
 const usedBytes=readUsage(mountPoint,volume,platform)??null;
 return{usedBytes,capacityBytes:volume.blocks*volume.bsize,partial:usedBytes===null};
}

async function partitionStateV1(directory:string,ports:CacheRunnerPortsV1,deep:boolean):Promise<Readonly<{state:CachePartitionRowV1["state"];usedBytes:number|null;capacityBytes:number|null;partial:boolean;walked:boolean}>>{
 let outer;try{outer=await lstat(directory);}catch{return{state:"absent",usedBytes:null,capacityBytes:null,partial:false,walked:false};}
 const inner=join(directory,mount);let child;try{child=await lstat(inner);}catch{child=undefined;}
 const platform=ports.platform??process.platform,declaration=nativeStorageBackendOfDirectoryV1(directory,{platform}),backing=declaration.backingFilename,mounted=declaration.capabilities.includes("mounted-volume")?child?.isDirectory()===true&&child.dev!==outer.dev:nativePartitionOfV1(inner,platform)?.attached===true;
 if(mounted){const row=await nativeStatfsV1(inner,{platform,...(deep?{}:{maximumAgeMilliseconds:nativeDirectoryProtocolV1().usage.statusMaximumAgeMilliseconds})});return{state:"mounted",...mountedPartitionUsageV1(inner,row,platform),walked:false};}
 if(await exists(join(directory,backing)))return{state:"detached",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 if(!child)return{state:"absent",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 if(!deep)return{state:"directory",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 const measure=await measureCacheTreeV1(inner,ports,runtime.statusEntriesPerTarget,"partition");
 return{state:measure.present?"directory":"absent",usedBytes:measure.present?measure.bytes:null,capacityBytes:null,partial:measure.partial,walked:true};
}

/** 🪪️ Reads the physical size an image was born with from its `image.json`; no such record (directory hosts, absent image) is null. */
export async function readImageEnvelopeV1(directory:string):Promise<number|null>{
 try{const value=JSON.parse(await readFile(join(directory,"image.json"),"utf8")) as {maximumPhysicalBytes?:unknown};return Number.isSafeInteger(value.maximumPhysicalBytes)?value.maximumPhysicalBytes as number:null;}catch{return null;}
}

/** 📒️ Reads the size ledger of the repository cache root; a missing or damaged file is an empty ledger. */
export async function loadCacheSizesV1(workspace:string,ledger:CacheLedgerV1):Promise<CacheSizeLedgerV1>{
 try{return readCacheSizeLedgerV1(JSON.parse(await readFile(join(cacheRootDirectoryV1(workspace,ledger,"repository"),CACHE_SIZE_FILE_V1),"utf8")));}catch{return{version:1,records:{}};}
}

/** ✍️ Merges measured sizes into the size ledger (null forgets a key) and replaces the file atomically; losing a write only costs the next status a deep walk. */
export async function recordCacheSizesV1(workspace:string,ledger:CacheLedgerV1,updates:Readonly<Record<string,CacheSizeRecordV1|null>>):Promise<boolean>{
 if(!Object.keys(updates).length)return true;
 const directory=cacheRootDirectoryV1(workspace,ledger,"repository"),file=join(directory,CACHE_SIZE_FILE_V1),records:Record<string,CacheSizeRecordV1>={...(await loadCacheSizesV1(workspace,ledger)).records};
 for(const[key,value]of Object.entries(updates))if(value===null)delete records[key];else records[key]=value;
 const newest=Object.entries(records).sort((left,right)=>right[1].atMs-left[1].atMs||(left[0]<right[0]?-1:1)).slice(0,runtime.maximumSizeRecords);
 const temporary=file+"."+process.pid+".tmp";
 try{await mkdir(directory,{recursive:true});await writeFile(temporary,JSON.stringify({version:1,records:Object.fromEntries(newest.sort((left,right)=>left[0]<right[0]?-1:1))},null,1)+"\n");await rename(temporary,file);return true;}
 catch{await unlink(temporary).catch(()=>{});return false;}
}

type Sized=Readonly<{present:boolean;usedBytes:number|null;entries:number;partial:boolean;source:CacheSourceV1;recordedAtMs:number|null}>;
const MISSING:Sized={present:false,usedBytes:null,entries:0,partial:false,source:"unknown",recordedAtMs:null};
const UNMEASURED_FLEET:Sized={...MISSING,present:true,partial:true};

async function measureCollectionFleetV1(directory:string,id:string,record:CacheSizeRecordV1|undefined,ports:CacheRunnerPortsV1):Promise<Sized>{
 let original;
 try{original=await lstat(directory,{bigint:true});}catch(error){ports.signal.throwIfAborted();return(error as NodeJS.ErrnoException).code==="ENOENT"?MISSING:UNMEASURED_FLEET;}
 if(!original.isDirectory())return UNMEASURED_FLEET;
 if(record&&!record.partial)return{present:true,usedBytes:record.usedBytes,entries:0,partial:false,source:"recorded",recordedAtMs:record.atMs};
 try{
  const measure=await measureCacheTreeV1(directory,ports,runtime.statusEntriesPerTarget,id),current=await lstat(directory,{bigint:true});
  if(!measure.present||!current.isDirectory()||current.dev!==original.dev||current.ino!==original.ino)return UNMEASURED_FLEET;
  return{present:true,usedBytes:measure.bytes,entries:measure.entries,partial:measure.partial,source:"walked",recordedAtMs:null};
 }catch{ports.signal.throwIfAborted();return UNMEASURED_FLEET;}
}

/** 📈️ Reads the ledger's status from the disk without taking a lease and, by default, without walking any tree: partitions come from native filesystem measurements of their mounted image, every other size from the size ledger that collections and deep status write, existence from one lstat or readdir. `deep` walks everything with progress and cancellation, then records what it measured. */
export async function reportCacheLedgerV1(workspace:string,ledger:CacheLedgerV1,ports:CacheRunnerPortsV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions,only?:ReadonlySet<string>,options:Readonly<{deep:boolean;measureFleetForCollection?:boolean}>={deep:false}):Promise<CacheReportV1>{
 const deep=options.deep,sizes=deep?{version:1 as const,records:{}}:await loadCacheSizesV1(workspace,ledger),fresh:Record<string,CacheSizeRecordV1>={},rows:CachePartitionRowV1[]=[],areaRows:CacheAreaRowV1[]=[],states=new Map<string,Awaited<ReturnType<typeof partitionStateV1>>>(),used=new Map<string,Sized>(),partial=new Map<CacheRootIdV1,boolean>(),selected=(id:string)=>only===undefined||only.has(id),now=ports.now();
 const sized=async(key:string,paths:readonly string[],stage:string):Promise<Sized>=>{
  if(deep){
   let bytes=0,entries=0,isPartial=false,present=false;
   for(const path of paths){const measure=await measureCacheTreeV1(path,ports,runtime.statusEntriesPerTarget,stage);bytes+=measure.bytes;entries+=measure.entries;isPartial=isPartial||measure.partial;present=present||measure.present;}
   if(present)fresh[key]={usedBytes:bytes,partial:isPartial,atMs:now};
   return{present,usedBytes:present?bytes:null,entries,partial:isPartial,source:present?"walked":"unknown",recordedAtMs:null};
  }
  let present=false;for(const path of paths)if(await exists(path)){present=true;break;}
  if(!present)return MISSING;
  const record=sizes.records[key];
  return record?{present:true,usedBytes:record.usedBytes,entries:0,partial:record.partial,source:"recorded",recordedAtMs:record.atMs}:{present:true,usedBytes:null,entries:0,partial:false,source:"unknown",recordedAtMs:null};
 };
 const matching=async(root:string,pattern:string):Promise<string[]>=>{const matcher=new RegExp(pattern,"u");try{return(await readdir(root)).filter(name=>matcher.test(name)).map(name=>join(root,name));}catch{return[];}};
 const totals=new Map<CacheRootIdV1,number>(CACHE_ROOT_IDS_V1.map(root=>[root,0])),count=(root:CacheRootIdV1,size:Sized)=>{totals.set(root,totals.get(root)!+(size.usedBytes??0));partial.set(root,(partial.get(root)??false)||size.partial||size.usedBytes===null);};
 for(const area of ledger.areas.filter(area=>area.kind==="partition"||area.kind==="control"&&area.managed)){
  if(!selected(area.id))continue;
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions)!,budgetBytes=cacheAreaBudgetBytesV1(ledger,area,partitions);
  if(area.kind==="partition"){
   const state=await partitionStateV1(directory,ports,deep);states.set(area.partition!,state);
   rows.push({id:area.id,kind:"partition",directory,state:state.state,usedBytes:state.usedBytes,budgetBytes,capacityBytes:state.capacityBytes,partial:state.partial,owner:area.managed!.owner,source:state.usedBytes===null?"unknown":state.walked?"walked":"statfs",recordedAtMs:null,envelopeBytes:await readImageEnvelopeV1(directory),declaredBytes:partitions[area.partition!]!.capacityBytes,recreate:area.recreate==="derived"});
  }else{
   const size=area.managed!.owner==="fleet"&&options.measureFleetForCollection?await measureCollectionFleetV1(directory,area.id,sizes.records[area.id],ports):await sized(area.id,[directory],area.id);
   if(area.managed!.owner==="fleet"&&size.present)count(area.root,size);
   rows.push({id:area.id,kind:"control",directory,state:size.present?"directory":"absent",usedBytes:size.usedBytes,budgetBytes,capacityBytes:null,partial:size.partial,owner:area.managed!.owner,source:size.source,recordedAtMs:size.recordedAtMs,envelopeBytes:null,declaredBytes:null,recreate:false});
  }
 }
 const nested=ledger.areas.filter(area=>area.kind==="nested"&&selected(area.id)).sort((left,right)=>Number("remainder" in (left.budget??{}))-Number("remainder" in (right.budget??{})));
 for(const area of nested){
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const state=states.get(area.partition!)??await partitionStateV1(join(workspace,partitions[area.partition!]!.directory),ports,deep),directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions)!,budget=cacheAreaBudgetBytesV1(ledger,area,partitions),remainder=area.budget&&"remainder" in area.budget,owner=area.managed?.owner??null;
  let row:CacheAreaRowV1;
  if(state.state==="detached"||state.state==="absent")row={id:area.id,root:area.root,partition:area.partition!,directory,present:false,measurable:false,usedBytes:null,budgetBytes:budget,entries:0,partial:false,over:false,owner,source:"unknown",recordedAtMs:null};
  else if(remainder){
   const siblings=nested.filter(other=>other.partition===area.partition&&other.id!==area.id).map(other=>used.get(other.id)),known=siblings.every(row=>row!==undefined&&row.usedBytes!==null),total=state.usedBytes;
   if(total===null||!known)row={id:area.id,root:area.root,partition:area.partition!,directory,present:true,measurable:total!==null,usedBytes:null,budgetBytes:budget,entries:0,partial:false,over:false,owner,source:"unknown",recordedAtMs:null};
   else{
    const own=Math.max(0,total-siblings.reduce((sum,row)=>sum+row!.usedBytes!,0)),stamps=siblings.map(row=>row!.recordedAtMs).filter((value):value is number=>value!==null),recorded=stamps.length>0;
    row={id:area.id,root:area.root,partition:area.partition!,directory,present:true,measurable:true,usedBytes:own,budgetBytes:budget,entries:0,partial:state.partial||siblings.some(row=>row!.partial),over:budget!==null&&own>budget,owner,source:recorded?"recorded":state.walked?"walked":"statfs",recordedAtMs:recorded?Math.min(...stamps):null};
   }
  }else{
   const size=await sized(area.id,area.collect?.units===undefined?[directory]:await matching(directory,area.collect.units),area.id);used.set(area.id,size);
   row={id:area.id,root:area.root,partition:area.partition!,directory,present:size.present,measurable:true,usedBytes:size.usedBytes,budgetBytes:budget,entries:size.entries,partial:size.partial,over:budget!==null&&size.usedBytes!==null&&size.usedBytes>budget,owner,source:size.source,recordedAtMs:size.recordedAtMs};
  }
  areaRows.push(row);
 }
 const retired:CacheEntryRowV1[]=[],undeclared:CacheEntryRowV1[]=[];
 for(const area of ledger.areas.filter(area=>(area.kind==="tree"||area.kind==="control"&&area.collect)&&selected(area.id))){
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions),budget=cacheAreaBudgetBytesV1(ledger,area,partitions),size=await sized(area.id,directory!==null?[directory]:await matching(cacheRootDirectoryV1(workspace,ledger,area.root),area.pattern!),area.id);
  if(size.present)count(area.root,size);
  areaRows.push({id:area.id,root:area.root,partition:null,directory,present:size.present,measurable:true,usedBytes:size.usedBytes,budgetBytes:budget,entries:size.entries,partial:size.partial,over:budget!==null&&size.usedBytes!==null&&size.usedBytes>budget,owner:area.managed?.owner??null,source:size.source,recordedAtMs:size.recordedAtMs});
 }
 for(const retirement of ledger.retirements){
  if(only&&!only.has("retired"))continue;
  await ports.progress("status:"+retirement.directory);ports.signal.throwIfAborted();
  const size=await sized("retired:"+retirement.root+":"+retirement.directory,[join(cacheRootDirectoryV1(workspace,ledger,retirement.root),...retirement.directory.split("/"))],retirement.directory);
  if(!size.present)continue;
  retired.push({root:retirement.root,path:retirement.root+":"+retirement.directory,kind:"retired",usedBytes:size.usedBytes,entries:size.entries,partial:size.partial,source:size.source,recordedAtMs:size.recordedAtMs});count(retirement.root,size);
 }
 if(!only||only.has("undeclared"))for(const root of CACHE_ROOT_IDS_V1){
  for(const entry of await undeclaredEntriesV1(workspace,ledger,root)){
   await ports.progress("status:"+entry);ports.signal.throwIfAborted();
   const size=await sized("undeclared:"+root+":"+entry,[join(cacheRootDirectoryV1(workspace,ledger,root),...entry.split("/"))],entry);
   undeclared.push({root,path:root+":"+entry,kind:"undeclared",usedBytes:size.usedBytes,entries:size.entries,partial:size.partial,source:size.source,recordedAtMs:size.recordedAtMs});count(root,size);
  }
 }
 if(deep)await recordCacheSizesV1(workspace,ledger,fresh);
 const roots=(CACHE_ROOT_IDS_V1).map(id=>({id,directory:ledger.roots[id].directory,usedBytes:totals.get(id)!,budgetBytes:ledger.roots[id].maximumBytes,partial:partial.get(id)??false}));
 return{version:1,workspace,generatedAtMs:now,partitions:rows,areas:areaRows,roots,retired,undeclared};
}

/** 🕵️ Lists the root-relative entries nobody declared, descending only through containers of declared homes. */
export async function undeclaredEntriesV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1):Promise<readonly string[]>{
 const base=cacheRootDirectoryV1(workspace,ledger,root),found:string[]=[],pending=[""];
 while(pending.length){
  const prefix=pending.shift()!;let names:string[];try{names=await readdir(prefix?join(base,...prefix.split("/")):base);}catch(error){if(prefix&&(error as NodeJS.ErrnoException).code==="ENOTDIR"){const path=root+":"+prefix;throw Error(cacheTextV1("en","error.container",{path})+"\n"+cacheTextV1("de","error.container",{path}));}continue;}
  for(const name of names.sort()){
   const path=prefix?prefix+"/"+name:name,kind=classifyCachePathV1(ledger,root,path).kind;
   if(kind==="undeclared")found.push(path);else if(kind==="container")pending.push(path);
  }
 }
 return found;
}

async function unlinkPath(path:string):Promise<void>{
 try{await unlink(path);}
 catch(error){
  const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")return;
  if(code!=="EPERM"&&code!=="EACCES"&&code!=="EBUSY")throw error;
  await chmod(path,0o666).catch(()=>{});await unlink(path);
 }
}

/** 🗑️ Removes one tree in cancellable batches without ever following a link; a link is unlinked, never entered. */
export async function removeCacheTreeV1(path:string,ports:CacheRunnerPortsV1,stage:string):Promise<void>{
 let info;try{info=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
 if(!info.isDirectory()){await unlinkPath(path);return;}
 const stack:{path:string;visited:boolean}[]=[{path,visited:false}],tick=ticker(ports,()=>stage);
 while(stack.length){
  await tick();const top=stack[stack.length-1]!;
  if(top.visited){stack.pop();await rmdir(top.path).catch(error=>{if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;});continue;}
  top.visited=true;const files:string[]=[];
  for(const child of await readdir(top.path,{withFileTypes:true}).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}))child.isDirectory()?stack.push({path:join(top.path,child.name),visited:false}):files.push(join(top.path,child.name));
  for(let index=0;index<files.length;index+=runtime.removeBatch){await tick();await Promise.all(files.slice(index,index+runtime.removeBatch).map(unlinkPath));}
 }
}

/** 💽️ The bytes the host volume of a workspace still has free; unreadable is null. */
export function machineHostFreeBytesV1(workspace:string):number|null{return nativeFreeSpaceSyncV1(workspace)?.availableBytes??null;}
/** 💽️ The size of the host volume of a workspace; unknown or unreadable is null. */
export function machineHostTotalBytesV1(workspace:string):number|null{return nativeFreeSpaceSyncV1(workspace)?.totalBytes??null;}

/** 💽️ The bytes the host really holds for the fixed image of a partition directory (blocks, not size), null without an image file: raw images keep blocks freed inside them, so this is what the machine budget must count. */
export function hostAllocationV1(directory:string):number|null{try{return statSync(join(directory,"fixed.dmg")).blocks*512;}catch{return null;}}

const fleetControl=(row:CachePartitionRowV1):boolean=>row.kind==="control"&&row.owner==="fleet";
function fleetStateBytesV1(report:CacheReportV1):number{return report.partitions.filter(fleetControl).reduce((bytes,row)=>bytes+(row.usedBytes??0),0);}

/** 🏬️ Counts fixed images and loose usage, protecting recorded fleet state when status or a collection supplies loose bytes. */
export function machineUsesV1(report:CacheReportV1,collection?:Readonly<{usedBytes:number;evictableBytes:number}>):readonly MachineUseV1[]{
 const uses:MachineUseV1[]=report.partitions.filter(row=>row.kind==="partition").map(row=>({id:row.id,usedBytes:row.usedBytes??0,envelopeBytes:row.envelopeBytes??row.capacityBytes,allocatedBytes:hostAllocationV1(row.directory),evictableBytes:null}));
 const protectedBytes=fleetStateBytesV1(report),usedBytes=collection===undefined?report.roots.reduce((sum,root)=>sum+root.usedBytes,0):collection.usedBytes+protectedBytes,ordinaryBytes=Math.max(0,usedBytes-protectedBytes);
 uses.push({id:"loose",usedBytes,envelopeBytes:null,evictableBytes:Math.min(ordinaryBytes,collection?.evictableBytes??ordinaryBytes)});
 return uses;
}

/** 📤️ One line of collector output, already localized. */
export type CacheOutputV1=Readonly<{write(line:string):void;language:CacheLanguageV1}>;
/** 🧮️ What a collection did or would do. */
export type CacheCollectionV1=Readonly<{dryRun:boolean;plans:readonly CacheTargetPlanV1[];removed:readonly CacheRemovalV1[];kept:number;failed:readonly Readonly<{target:string;name:string;message:string}>[];freedBytes:number;delegated:readonly Readonly<{owner:string;text:string}>[];machine:MachinePlanV1|null;fleetStatePartial:boolean;sizeUpdates:Readonly<Record<string,CacheSizeRecordV1|null>>}>;

type Resolved=Readonly<{target:CacheTargetV1;base:string;prefix:string[];unusedAgeMs:number|null;loose?:boolean;custody?:"quiet"|"sqlite"|"lease"|"pid";units?:RegExp;release?:()=>void}>;

function insideBase(base:string,path:string):boolean{const distance=relative(base,path);return distance!==""&&!distance.startsWith(".."+sep)&&distance!==".."&&!isAbsolute(distance);}

async function unitNamesV1(base:string,depth:1|2):Promise<string[]>{
 const children=await readdir(base).catch(()=>[] as string[]);
 if(depth===1)return children;
 const names:string[]=[];
 for(const child of children){const grandchildren=await readdir(join(base,child)).catch(()=>null);if(grandchildren===null)names.push(child);else for(const grandchild of grandchildren)names.push(child+"/"+grandchild);}
 return names;
}

/** 🏷️ Names the live process a unit directory belongs to when its name carries a process id (`(?<pid>…)` in the unit pattern); a process that cannot be signalled still counts as alive. */
export function cachePidHolderV1(name:string,units:RegExp|undefined,ports:Pick<CacheRunnerPortsV1,"alive">):string|undefined{
 const pid=Number(units?.exec(name)?.groups?.pid);
 return Number.isSafeInteger(pid)&&pid>0&&ports.alive(pid)?"pid:"+pid:undefined;
}

async function observe(base:string,names:readonly string[],id:string,custody:"quiet"|"sqlite"|"lease"|"pid",ports:CacheRunnerPortsV1,pattern?:RegExp):Promise<CacheUnitV1[]>{
 const units:CacheUnitV1[]=[];let current="";const tick=ticker(ports,()=>"scan:"+id+"/"+current);
 for(const name of names){
  current=name;await tick();
  const measure=await measureCacheTreeV1(join(base,...name.split("/")),ports,runtime.collectEntriesPerUnit,id);
  if(!measure.present)continue;
  const held=custody==="lease"?undefined:custody==="pid"?cachePidHolderV1(name,pattern,ports)??await probeCacheHeldV1(measure,ports):await probeCacheHeldV1(measure,ports);
  units.push({name,bytes:measure.bytes,recencyMs:measure.recencyMs,...held?{held}:{}});
 }
 return units;
}

/** 🧹️ Plans and, unless asked to only plan, performs the collection: it scans every selected target, takes the area's exclusive lease without waiting, re-checks each unit right before removing it, and never touches a held or recently used one. */
export async function collectCacheV1(workspace:string,ledger:CacheLedgerV1,request:Readonly<{dryRun:boolean;only?:ReadonlySet<string>;reclaimBytes?:number;machine?:Readonly<{totalBytes:number;source:MachineSourceV1;policy?:MachinePolicyV1}>}>,ports:CacheRunnerPortsV1,output:CacheOutputV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):Promise<CacheCollectionV1>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(output.language,key,variables),selected=(id:string)=>request.only===undefined||request.only.has(id),resolved:Resolved[]=[],delegated:{owner:string;text:string}[]=[],deferred:{slot:number;owner:string;id:string}[]=[],notes:string[]=[],now=ports.now();
 try{
  for(const area of ledger.areas){
   if(!selected(area.id)&&!(area.partition!==undefined&&selected(area.partition)))continue;
   if(area.managed&&area.kind!=="partition"&&area.kind!=="control"){
    deferred.push({slot:notes.length,owner:area.managed.owner,id:area.id});notes.push("");
    continue;
   }
   if(!area.collect)continue;
   const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions);let names:string[]=[],base:string;
   const matcher=area.collect.units===undefined?undefined:new RegExp(area.collect.units,"u");
   if(directory!==null){base=directory;names=(await unitNamesV1(directory,area.collect.depth??1)).filter(name=>matcher===undefined||matcher.test(name));}
   else{base=cacheRootDirectoryV1(workspace,ledger,area.root);const matcher=new RegExp(area.pattern!,"u");try{names=(await readdir(base)).filter(name=>matcher.test(name));}catch{names=[];}}
   const lease=area.collect.custody==="lease"?await ports.exclusive(area.collect.lease!):undefined,heldBy=area.collect.custody==="lease"&&!lease?area.collect.lease!:undefined;
   const units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports,matcher);
   resolved.push({target:{id:area.id,kind:"area",budgetBytes:cacheAreaBudgetBytesV1(ledger,area,partitions),unusedAgeMs:area.collect.unusedAgeMs,quietMs:area.collect.quietMs,...heldBy?{heldBy}:{},...area.collect.pressure==="pinned"?{pinned:true as const}:{},units},base,prefix:[],unusedAgeMs:area.collect.unusedAgeMs,loose:area.kind==="tree"||area.kind==="control",custody:area.collect.custody,...matcher?{units:matcher}:{},...lease?{release:()=>lease.release()}:{}});
   if(heldBy)notes.push(text("gc.held",{path:area.id,holder:heldBy}));
  }
  if(request.only===undefined||request.only.has("retired"))for(const retirement of ledger.retirements){
   const base=join(cacheRootDirectoryV1(workspace,ledger,retirement.root),...retirement.directory.split("/"));let names:string[];
   try{names=await readdir(base);}catch{continue;}
   resolved.push({target:{id:retirement.root+":"+retirement.directory,kind:"retired",budgetBytes:null,unusedAgeMs:0,quietMs:retirement.quietMs,units:await observe(base,names,retirement.directory,"quiet",ports)},base,prefix:[],unusedAgeMs:0});
  }
  if(ledger.strays!==undefined&&(request.only===undefined||request.only.has("retired"))){
   const found=await(ports.strays?.(workspace,ledger)??cacheStrayRootsV1(workspace,ledger,ports.signal)),mounts=found.length?await(ports.mountPoints?.()??cacheMountPointsV1(ports.platform)):[];
   if(found.length)notes.push(ledger.strays.reason[output.language]);
   for(const stray of found){
    ports.signal.throwIfAborted();
    const base=join(workspace,...stray.split("/")),real=await realpath(base).catch(()=>base),mounted=mounts.find(point=>[base,real].some(path=>point===path||point.startsWith(path+sep)));
    if(mounted!==undefined){notes.push(text("gc.held",{path:"stray:"+stray,holder:"mount:"+mounted}));continue;}
    if(!(await lstat(base).catch(()=>undefined))?.isDirectory())continue;
    resolved.push({target:{id:"stray:"+stray,kind:"retired",budgetBytes:null,unusedAgeMs:0,quietMs:ledger.strays.quietMs,units:await observe(base,await readdir(base).catch(()=>[] as string[]),"stray:"+stray,"quiet",ports)},base,prefix:[],unusedAgeMs:0});
   }
  }
  if(request.only===undefined||request.only.has("undeclared"))for(const root of CACHE_ROOT_IDS_V1){
   const base=cacheRootDirectoryV1(workspace,ledger,root),names=await undeclaredEntriesV1(workspace,ledger,root);
   if(names.length)resolved.push({target:{id:root+":undeclared",kind:"undeclared",budgetBytes:null,unusedAgeMs:ledger.roots[root].undeclaredUnusedAgeMs,quietMs:ledger.roots[root].undeclaredQuietMs,units:await observe(base,names,root+":undeclared","quiet",ports)},base,prefix:[],unusedAgeMs:ledger.roots[root].undeclaredUnusedAgeMs,loose:true});
  }
  const updates:Record<string,CacheSizeRecordV1|null>={};
  let machine:MachinePlanV1|null=null,fleetStatePartial=false,targets=resolved.map(row=>row.target);
  if(request.machine){
   const report=await reportCacheLedgerV1(workspace,ledger,ports,partitions,undefined,{deep:false,measureFleetForCollection:true}),fleet=report.partitions.filter(fleetControl);
   for(const row of fleet)updates[row.id]=row.usedBytes===null?null:{usedBytes:row.usedBytes,partial:row.partial,atMs:row.recordedAtMs??report.generatedAtMs};
   fleetStatePartial=fleet.some(row=>row.state!=="absent"&&(row.usedBytes===null||row.partial));
   if(fleetStatePartial)notes.push(text("machine.gc.fleetIncomplete"));
   else{
    const first=planCacheCollectionV1(targets,now),retained=new Map(first.map(plan=>[plan.id,plan.retainedBytes])),members=resolved.filter(row=>row.loose&&row.target.kind==="area"),looseBytes=resolved.filter(row=>row.loose).reduce((sum,row)=>sum+(retained.get(row.target.id)??0),0);
    const evictableBytes=resolved.filter(row=>row.loose&&row.target.heldBy===undefined&&!row.target.pinned).reduce((sum,row)=>sum+row.target.units.filter(unit=>unit.held===undefined&&now-unit.recencyMs>=row.target.quietMs).reduce((inner,unit)=>inner+unit.bytes,0),0);
    machine=planMachineBudgetV1({totalBytes:request.machine.totalBytes,hostFreeBytes:machineHostFreeBytesV1(workspace),hostTotalBytes:machineHostTotalBytesV1(workspace),uses:machineUsesV1(report,{usedBytes:looseBytes,evictableBytes})},request.machine.policy);
    const quota=machine.stores.find(store=>store.id==="loose");
    if(quota){
     const split=splitMachineBudgetV1(Math.max(0,quota.contentLimitBytes-fleetStateBytesV1(report)),members.map(row=>({id:row.target.id,weight:Math.max(1,row.target.budgetBytes??retained.get(row.target.id)??1),floor:0,demand:retained.get(row.target.id)??0})),false);
     const tightened:CacheTargetV1[]=[];
     for(const target of targets){
      const share=split.get(target.id),row=resolved.find(candidate=>candidate.target===target);
      if(share===undefined||row===undefined||share>=(target.budgetBytes??Infinity)){tightened.push(share===undefined?target:{...target,budgetBytes:Math.min(target.budgetBytes??Infinity,share)});continue;}
      if(row.custody!=="quiet"){tightened.push({...target,budgetBytes:share});continue;}
      if(!await(ports.atimeReliable??cacheAtimeReliableV1)(row.base)){tightened.push(target);notes.push(text("machine.gc.atime",{id:target.id}));continue;}
      tightened.push({...target,budgetBytes:share,quietMs:Math.max(target.quietMs,runtime.quotaQuietMs)});
     }
     targets=tightened;
     notes.push(text("machine.gc.quota",{quota:formatCacheBytesV1(quota.contentLimitBytes),over:formatCacheBytesV1(quota.excessBytes)}));
    }
    for(const line of formatMachinePlanV1(output.language,machine,request.machine.source))notes.push(line);
   }
  }
  for(const row of deferred){
   const answer=ports.delegate?await ports.delegate(row.owner,row.id,request.dryRun,output.language,{machine,selected:request.only!==undefined}):undefined;
   if(answer!==undefined){delegated.push({owner:row.owner,text:answer});notes[row.slot]=text("gc.owner",{owner:row.owner,text:answer});}else notes[row.slot]=text("gc.managed",{id:row.id,owner:row.owner});
  }
  const plans=planCacheCollectionV1(targets,now,request.reclaimBytes??0),removed:CacheRemovalV1[]=[],failed:{target:string;name:string;message:string}[]=[],gone=new Map<string,number>(),goneNames=new Set<string>();let kept=0,freed=0;
  output.write(text("gc.start",{count:plans.length,mode:text(request.dryRun?"gc.mode.plan":"gc.mode.apply")}));
  for(const note of notes)output.write(note);
  for(const[index,plan]of plans.entries()){
   const row=resolved[index]!,quietMs=targets[index]!.quietMs;ports.signal.throwIfAborted();
   for(const unit of plan.kept){kept++;output.write(unit.reason==="held"?text("gc.held",{path:plan.id+"/"+unit.name,holder:row.target.units.find(candidate=>candidate.name===unit.name)?.held??row.target.heldBy??"?"}):text("gc.quiet",{path:plan.id+"/"+unit.name,minutes:Math.round(quietMs/60000)}));}
   let shown=0,hiddenCount=0,hiddenBytes=0;
   const announce=(line:string,bytes:number):void=>{if(shown<runtime.listedRemovalsPerTarget){shown++;output.write(line);}else{hiddenCount++;hiddenBytes+=bytes;}};
   for(const removal of plan.removals){
    ports.signal.throwIfAborted();
    const path=resolve(row.base,...removal.name.split("/")),reason=formatCacheReasonV1(output.language,removal.reason,row.unusedAgeMs),label=plan.id+"/"+removal.name;
    if(!insideBase(resolve(row.base),path)){failed.push({target:plan.id,name:removal.name,message:text("error.refused",{path:label})});continue;}
    if(request.dryRun){removed.push(removal);freed+=removal.bytes;announce(text("gc.would",{path:label,bytes:formatCacheBytesV1(removal.bytes),reason}),removal.bytes);continue;}
    const again=await measureCacheTreeV1(path,ports,runtime.collectEntriesPerUnit,plan.id),unit=row.target.units.find(candidate=>candidate.name===removal.name)!;
    if(!again.present)continue;
    const held=await probeCacheHeldV1(again,ports)??(row.custody==="pid"?cachePidHolderV1(removal.name,row.units,ports):undefined);
    if(held||ports.now()-again.recencyMs<quietMs||again.recencyMs>unit.recencyMs){kept++;output.write(held?text("gc.held",{path:label,holder:held}):text("gc.changed",{path:label}));continue;}
    announce(text("gc.remove",{path:label,bytes:formatCacheBytesV1(removal.bytes),reason}),removal.bytes);
    try{await removeCacheTreeV1(path,ports,"remove:"+label);removed.push(removal);freed+=removal.bytes;gone.set(plan.id,(gone.get(plan.id)??0)+removal.bytes);goneNames.add(plan.id+"/"+removal.name);}
    catch(error){if(ports.signal.aborted)throw error;failed.push({target:plan.id,name:removal.name,message:String((error as Error).message??error)});}
   }
   if(hiddenCount)output.write(text("gc.more",{id:plan.id,count:hiddenCount,bytes:formatCacheBytesV1(hiddenBytes)}));
   if(!request.dryRun&&row.target.kind==="retired"&&plan.removals.length===row.target.units.length&&!failed.some(item=>item.target===plan.id))await rmdir(row.base).catch(()=>{});
  }
  output.write(text(request.dryRun?"gc.summary.plan":"gc.summary.apply",{count:removed.length,bytes:formatCacheBytesV1(freed),kept}));
  const at=ports.now(),record=(bytes:number)=>({usedBytes:Math.max(0,bytes),partial:false,atMs:at});
  for(const[index,plan]of plans.entries()){
   const row=resolved[index]!;if(row.target.heldBy!==undefined)continue;
   if(row.target.kind==="area")updates[row.target.id]=record(plan.totalBytes-(gone.get(plan.id)??0));
   else if(row.target.kind==="retired"){if(!row.target.id.startsWith("stray:"))updates["retired:"+row.target.id]=row.target.units.every(unit=>goneNames.has(plan.id+"/"+unit.name))?null:record(plan.totalBytes-(gone.get(plan.id)??0));}
   else for(const unit of row.target.units)updates["undeclared:"+row.target.id.slice(0,row.target.id.indexOf(":"))+":"+unit.name]=goneNames.has(plan.id+"/"+unit.name)?null:record(unit.bytes);
  }
  if(!request.dryRun)await recordCacheSizesV1(workspace,ledger,updates);
  return Object.freeze({dryRun:request.dryRun,plans,removed,kept,failed,freedBytes:freed,delegated,machine,fleetStatePartial,sizeUpdates:Object.freeze(updates)});
 }finally{for(const row of resolved)try{row.release?.();}catch{}}
}
