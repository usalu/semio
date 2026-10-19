import {chmod,lstat,mkdir,readdir,readFile,rename,rm,rmdir,stat,unlink,writeFile} from "node:fs/promises";
import type {Dirent} from "node:fs";
import {dirname,join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkResourceLeaseControl,withResourceLeases,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {withRepositoryCacheStoreV1,withRepositoryCacheStoreQueuedMaintenanceV1,withRepositoryCacheStoreMaintenanceTaskV1,requireRepositoryCacheStoreMaintenanceV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {repositoryCacheOrdinaryMaintenanceV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import type {BuildCacheActivityV1,BuildCacheContextV1,BuildCacheStatusV1,BuildCacheToolV1,BuildDryRunPlanV1,BuildTaskGraphV1} from "../../../⚙️operations/🟦️.ts";
import {bazelCacheLayoutV1,bazelServerBaseV1} from "../💾️storage/🟦️.ts";
import type {BazelTaskPlanV1} from "../📋️task-plan/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const;
type Layout=ReturnType<typeof bazelCacheLayoutV1>;
type Walk=Readonly<{phaseControl:ResourceLeaseControl;progress:(stage:string)=>Promise<void>;check?:()=>void}>;

/** 📏️ The files of one disk cache: only `ac` and `cas` hold entries, `tmp` and `gc` are Bazel's bookkeeping. */
export type BazelDiskCacheCensusV1=Readonly<{available:boolean;entries:number;bytes:number}>;

/** 🧮️ What one Bazel build did, counted from the runner summary of its build event stream. */
export type BazelBuildMetricsV1=Readonly<{actions:number;executed:number;cacheHits:number}>;

const missing=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??"");
const blocked=(error:unknown)=>["EACCES","EPERM"].includes((error as NodeJS.ErrnoException).code??"");
const guard=(walk:Walk)=>{checkResourceLeaseControl(walk.phaseControl);walk.check?.();};

async function rowsOf(path:string):Promise<Dirent[]|null>{
 try{return await readdir(path,{withFileTypes:true});}catch(error){if(missing(error))return null;throw error;}
}

/** 🔎️ Counts disk cache entries and bytes without following links, publishing progress and stopping when cancelled. */
export async function measureBazelDiskCacheV1(directory:string,walk:Walk):Promise<BazelDiskCacheCensusV1>{
 guard(walk);
 if(await rowsOf(directory)===null)return Object.freeze({available:false,entries:0,bytes:0});
 let entries=0,bytes=0,visited=0;
 for(const name of policy.diskCache.directories){
  const pending=[{path:join(directory,name),depth:0}];
  while(pending.length){
   guard(walk);
   const {path,depth}=pending.pop()!;
   if(depth>policy.diskCache.maximumDepth||path.length>policy.maximumPath)throw Error("Bounded Bazel disk cache layout required");
   const rows=await rowsOf(path);
   if(rows===null)continue;
   for(const row of rows){
    if(row.isDirectory()){pending.push({path:join(path,row.name),depth:depth+1});continue;}
    entries++;
    if(!row.isFile())continue;
    try{bytes+=(await lstat(join(path,row.name))).size;}catch(error){if(!missing(error))throw error;entries--;}
   }
   if(++visited%policy.progressEntries===0)await walk.progress("measuring");
  }
 }
 guard(walk);
 return Object.freeze({available:true,entries,bytes});
}

async function removeEntry(path:string,remove:(path:string)=>Promise<void>):Promise<boolean>{
 for(let attempt=0;;attempt++){
  try{await remove(path);return true;}
  catch(error){
   if(missing(error))return false;
   if(attempt===0&&blocked(error)){await chmod(dirname(path),0o700).catch(()=>{});await chmod(path,0o700).catch(()=>{});continue;}
   throw error;
  }
 }
}

/** 🧹️ Removes every entry beneath `directory` (links are unlinked, never followed), keeping the directory itself; returns how many entries went. */
async function emptyDirectory(directory:string,walk:Walk,keepRoot:boolean):Promise<number>{
 const directories=[directory],pending=[directory];let removed=0;
 while(pending.length){
  guard(walk);
  const current=pending.pop()!,rows=await rowsOf(current);
  if(rows===null)continue;
  for(const row of rows){
   const path=join(current,row.name);
   if(row.isDirectory()){directories.push(path);pending.push(path);continue;}
   if(await removeEntry(path,unlink))removed++;
   if(removed%policy.progressEntries===0&&removed>0)await walk.progress("clearing");
  }
 }
 for(const path of directories.reverse()){
  guard(walk);
  if(keepRoot&&path===directory)continue;
  await removeEntry(path,rmdir).catch(error=>{if((error as NodeJS.ErrnoException).code!=="ENOTEMPTY")throw error;});
  if(++removed%policy.progressEntries===0)await walk.progress("clearing");
 }
 return removed;
}

/** 🧽️ Removes every file of the disk cache's entry folders and the folders below them, so the next build is a miss; the folders themselves and Bazel's bookkeeping stay. */
export async function clearBazelDiskCacheV1(directory:string,walk:Walk):Promise<number>{
 let removed=0;
 for(const name of policy.diskCache.directories)removed+=await emptyDirectory(join(directory,name),walk,true);
 guard(walk);
 return removed;
}

const running=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🛑️ Stops the persistent servers of the given output bases the way Bazel itself notices a shutdown request: by removing their pid file. */
async function stopBazelServers(bases:readonly string[],walk:Walk):Promise<void>{
 const pids:number[]=[];
 for(const base of bases){
  guard(walk);
  const file=join(base,policy.pidFile);
  try{const pid=Number((await readFile(file,"utf8")).trim());if(Number.isSafeInteger(pid)&&pid>0)pids.push(pid);await removeEntry(file,unlink);}catch(error){if(!missing(error))throw error;}
 }
 const deadline=Date.now()+policy.serverStopMilliseconds;
 for(let alive=pids.filter(running);alive.length;alive=alive.filter(running)){
  guard(walk);
  if(Date.now()>=deadline)throw Error("A Bazel server did not stop after its pid file was removed");
  await sleep(policy.serverPollMilliseconds);
 }
}

/** 🗑️ Stops the persistent servers and removes their per-workspace output bases, which hold Bazel's local action cache and execution roots, plus the activity ledger and pending event streams. */
export async function clearBazelServerBasesV1(directory:string,walk:Walk):Promise<number>{
 const rows=await rowsOf(directory);
 if(rows===null)return 0;
 const pattern=new RegExp(policy.serverPattern),bases=rows.filter(row=>row.isDirectory()&&pattern.test(row.name)).map(row=>join(directory,row.name));
 await stopBazelServers(bases,walk);
 let removed=0;
 for(const base of bases){removed+=await emptyDirectory(base,walk,false);}
 for(const row of rows)if(row.name===policy.ledger||row.name.endsWith(policy.eventsSuffix))if(await removeEntry(join(directory,row.name),unlink))removed++;
 guard(walk);
 return removed;
}

/** 🧾️ Names the build event stream of one workspace's server beside its output base, so concurrent selections never share a file. */
export function bazelBuildEventsFileV1(layout:Layout,workspace:string):string{
 const file=bazelServerBaseV1(layout,workspace)+policy.eventsSuffix;
 if(file.length>policy.maximumPath)throw Error("Bazel storage path exceeds 256 characters");
 return file;
}

/** 📊️ Counts a build from the runner summary of its Bazel build event stream (JSON lines): `total` actions, runners ending in ` cache hit` are restores, `internal` needs no process, every other runner executed. */
export function bazelBuildMetricsV1(events:string):BazelBuildMetricsV1|undefined{
 for(const line of events.split("\n")){
  if(!line.includes('"buildMetrics"'))continue;
  let event:{buildMetrics?:{actionSummary?:{runnerCount?:readonly {name?:unknown;count?:unknown}[]}}};
  try{event=JSON.parse(line);}catch{continue;}
  if(event.buildMetrics===undefined)continue;
  let actions=0,executed=0,cacheHits=0;
  for(const runner of event.buildMetrics.actionSummary?.runnerCount??[]){
   const name=String(runner.name),count=Number(runner.count??0);
   if(name===policy.totalRunner)actions+=count;
   else if(name.endsWith(policy.cacheHitSuffix))cacheHits+=count;
   else if(name!==policy.internalRunner)executed+=count;
  }
  const metrics={actions,executed,cacheHits};
  if(validateJsonSchemaSubset(schema.definitions.BuildMetrics,metrics,schema).length)throw Error("Closed Bazel build metrics required");
  return Object.freeze(metrics);
 }
 return undefined;
}

const empty:BuildCacheActivityV1=Object.freeze({builds:0,actions:0,executed:0,cacheHits:0});

/** 📈️ Reads the activity ledger of a layout; a missing or malformed ledger counts as no activity. */
export async function readBazelLedgerV1(layout:Layout):Promise<BuildCacheActivityV1>{
 try{
  const value=JSON.parse(await readFile(join(layout.server,policy.ledger),"utf8"));
  if(validateJsonSchemaSubset(schema.definitions.Ledger,value,schema).length)return empty;
  const {builds,actions,executed,cacheHits}=value;return Object.freeze({builds,actions,executed,cacheHits});
 }catch(error){if(missing(error)||error instanceof SyntaxError)return empty;throw error;}
}

/** ✍️ Adds the finished build's metrics to the ledger and drops its event stream; never throws, because bookkeeping must not fail a build. */
export async function recordBazelBuildMetricsV1(root:string,layout:Layout,workspace:string,phaseControl:ResourceLeaseControl):Promise<void>{
 try{
  const events=bazelBuildEventsFileV1(layout,workspace);
  let text:string;
  try{
   if((await stat(events)).size>policy.maximumEventBytes)return void await rm(events,{force:true});
   text=await readFile(events,"utf8");
  }catch(error){if(missing(error))return;throw error;}
  await rm(events,{force:true});
  const metrics=bazelBuildMetricsV1(text);
  if(metrics===undefined)return;
  await withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal:phaseControl.signal,control:phaseControl,resources:[{resource:"bazel-metrics",mode:"exclusive"}]},async()=>{
   const before=await readBazelLedgerV1(layout),ledger={version:1,builds:before.builds+1,actions:before.actions+metrics.actions,executed:before.executed+metrics.executed,cacheHits:before.cacheHits+metrics.cacheHits};
   if(validateJsonSchemaSubset(schema.definitions.Ledger,ledger,schema).length)throw Error("Closed Bazel activity ledger required");
   const file=join(layout.server,policy.ledger),temporary=file+"."+process.pid+".tmp";
   await mkdir(layout.server,{recursive:true});
   await writeFile(temporary,JSON.stringify(ledger)+"\n");
   await rename(temporary,file);
  });
 }catch(error){if(!phaseControl.signal.aborted)console.warn(`[bazel] cache activity not recorded: ${(error as Error).message}`);}
}

/** 🌫️ The plan of an `affected` selection that touches nothing: a successful run without tasks, like every other tool. */
export const bazelEmptyPlanV1:BazelTaskPlanV1=Object.freeze({version:1,tasks:Object.freeze({}),selected:Object.freeze([]) as unknown as string[]});

/** 🗺️ Projects the plan Bazel would execute into the neutral dry-run graph: roots have no dependencies, and dependencies on continuous tasks are kept apart because they only need to be running. */
export function bazelDryRunPlanV1(plan:BazelTaskPlanV1):BuildDryRunPlanV1&Readonly<{selected:readonly string[]}>{
 const ids=Object.keys(plan.tasks),continuous=(id:string)=>plan.tasks[id]!.definition.continuous===true,dependencies:Record<string,string[]>={},waiting:Record<string,string[]>={};
 for(const id of ids){
  const owned=plan.tasks[id]!.dependencies.filter(dependency=>dependency!==id);
  dependencies[id]=owned.filter(dependency=>!continuous(dependency));waiting[id]=owned.filter(continuous);
 }
 const graph:BuildTaskGraphV1={
  roots:ids.filter(id=>!dependencies[id]!.length&&!waiting[id]!.length),
  tasks:Object.fromEntries(ids.map(id=>{const task=plan.tasks[id]!;return [id,{id,target:{project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{})}}];})),
  dependencies,
  continuousDependencies:waiting
 };
 return Object.freeze({tasks:graph,selected:plan.selected});
}

function productsLayout(root:string,products:string):Layout{
 return bazelCacheLayoutV1({tools:repositoryCachePartitionRootV1(root,"tools"),dependencies:repositoryCachePartitionRootV1(root,"dependencies"),products});
}

async function bazelCacheStatusV1(context:BuildCacheContextV1):Promise<BuildCacheStatusV1>{
 return withRepositoryCacheStoreV1(context.root,"products",context.invocation,context.phaseControl,async(store,phase)=>{
  const layout=productsLayout(context.root,store.root),census=await measureBazelDiskCacheV1(layout.actions,{phaseControl:phase,progress:context.progress});
  return Object.freeze({version:1,tool:"bazel",available:census.available,directory:layout.actions,entries:census.entries,bytes:census.bytes,activity:await readBazelLedgerV1(layout)});
 });
}

async function bazelCacheClearV1(context:BuildCacheContextV1):Promise<void>{
 await withRepositoryCacheStoreQueuedMaintenanceV1(context.root,"products",context.invocation,context.phaseControl,repositoryCacheOrdinaryMaintenanceV1(),(store,phase)=>withRepositoryCacheStoreMaintenanceTaskV1(store,context.root,"products",async()=>{
  const layout=productsLayout(context.root,store.root),walk:Walk={phaseControl:phase,progress:context.progress,check:()=>requireRepositoryCacheStoreMaintenanceV1(store,context.root,"products")};
  for(const path of [layout.server,layout.actions])requireNativeStoragePathV1(store.root,path);
  await clearBazelServerBasesV1(layout.server,walk);
  await clearBazelDiskCacheV1(layout.actions,walk);
  await store.recheck();guard(walk);
 }));
}

/** 🔌️ Bazel's side of `cache status|clear`: status measures the disk cache of the products partition and reports the recorded build activity; clear stops the persistent servers, removes their output bases (Bazel's local action cache) and empties the disk cache, leaving toolchains and modules in place. */
export function bazelBuildCacheToolV1():BuildCacheToolV1{
 return Object.freeze({tool:"bazel",status:bazelCacheStatusV1,clear:bazelCacheClearV1});
}
