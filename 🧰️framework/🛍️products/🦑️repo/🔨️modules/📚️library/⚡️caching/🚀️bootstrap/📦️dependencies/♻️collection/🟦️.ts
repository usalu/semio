import {chmodSync,existsSync,lstatSync,mkdirSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmdirSync,statSync,unlinkSync,writeFileSync} from "node:fs";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {readFreeSpaceV1} from "../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";
import {processTableSnapshot} from "../../../../🏃️process/📋️process-table/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ♻️ The closed rules of the collection: where holds and generations live, how long a superseded, retained or unfinished generation rests before it may go, the free fraction of the image below which the rest shrinks to nothing, and how many entries one removal batch deletes. */
export const dependencyGenerationCollectionPolicyV1=schema.$defs.Policy.const;
const policy=dependencyGenerationCollectionPolicyV1,KEY=new RegExp(schema.$defs.Key.pattern,"u"),TOMBSTONE=new RegExp(schema.$defs.Tombstone.pattern,"u");
export type DependencyGenerationFactV1=Readonly<{key:string;ready:boolean;ageSeconds:number}>;
export type DependencyGenerationRetainedV1=Readonly<{key:string;retiredSeconds:number;newest:boolean}>;
export type DependencyGenerationPlanV1=Readonly<{generations:readonly DependencyGenerationFactV1[];current:readonly string[];retained:readonly DependencyGenerationRetainedV1[];held:readonly string[];bound:readonly string[];pressure:boolean}>;
export type DependencyGenerationRetirementV1=Readonly<{key:string;reason:"superseded"|"unfinished"}>;

/** 🧭️ Decides which generations may be retired: never one a pointer names as current, a live process holds, a live process runs from (its command line names the directory) or a checkout binds; a generation a pointer retains rests its retention (the retained grace, the rollback time for the newest of its scope), an unreferenced finished one the superseded grace, an unfinished one of a dead build the unfinished grace; under pressure (the image is nearly full of bytes or of inodes) every finished one is due at once and every unfinished one after the short pressure grace except the newest unfinished one, which the next attempt resumes. Pure; the Python oracle of the tests agrees. */
export function planDependencyGenerationRetirementV1(input:DependencyGenerationPlanV1):readonly DependencyGenerationRetirementV1[]{
 const current=new Set(input.current),held=new Set(input.held),bound=new Set(input.bound),retained=new Map(input.retained.map(row=>[row.key,row]));
 const resumed=input.generations.filter(row=>!row.ready).sort((left,right)=>left.ageSeconds-right.ageSeconds||(left.key<right.key?-1:1))[0]?.key;
 const due=(row:DependencyGenerationFactV1):boolean=>{
  const kept=retained.get(row.key);
  if(kept!==undefined)return kept.retiredSeconds>=(input.pressure?0:kept.newest?policy.rollbackSeconds:policy.retainedGraceSeconds);
  if(!row.ready&&input.pressure&&row.key!==resumed)return row.ageSeconds>=policy.unfinishedPressureGraceSeconds;
  return row.ageSeconds>=(row.ready?input.pressure?0:policy.supersededGraceSeconds:policy.unfinishedGraceSeconds);
 };
 return input.generations.filter(row=>!current.has(row.key)&&!held.has(row.key)&&!bound.has(row.key)&&due(row)).map(row=>({key:row.key,reason:row.ready?"superseded" as const:"unfinished" as const})).sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
}

const processAlive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const announced=new Set<string>();

/** 🤝️ Records that this process uses the given generations: one small file per generation and process in the hold directory of the store, removed when the process ends and ignored once its process is gone. Best effort, never throws: a hold that cannot be written only means the generation may be retired earlier than ideal, and the grace period still protects fresh ones. */
export function holdDependencyGenerationsV1(storeRoot:string,keys:readonly string[],pid:number=process.pid,now:()=>number=Date.now):void{
 try{
  const directory=join(storeRoot,policy.holdDirectory);
  for(const key of keys){
   if(!KEY.test(key)||announced.has(storeRoot+"\0"+key))continue;
   mkdirSync(directory,{recursive:true});
   const file=join(directory,key+"."+pid+".json");
   writeFileSync(file+".tmp",JSON.stringify({version:1,purpose:"dependency-generation-hold",generation:key,pid,atMs:now()}));renameSync(file+".tmp",file);
   announced.add(storeRoot+"\0"+key);
   process.once("exit",()=>{try{unlinkSync(file);}catch{}});
  }
 }catch{}
}

/** 👁️ The generations that a live process holds; holds of dead processes are removed on the way. */
export function readHeldDependencyGenerationsV1(storeRoot:string,alive:(pid:number)=>boolean=processAlive):readonly string[]{
 const directory=join(storeRoot,policy.holdDirectory),held=new Set<string>();
 let names:string[]=[];try{names=readdirSync(directory);}catch{return[];}
 for(const name of names){
  const match=/^([a-f0-9]{20})\.(\d+)\.json$/u.exec(name);if(!match)continue;
  if(alive(Number(match[2])))held.add(match[1]!);else try{unlinkSync(join(directory,name));}catch{}
 }
 return[...held].sort();
}

/** 📌️ What the pointers say: the generations that a scope names as current and, per retired generation, the seconds since it was retired and whether it is the newest retired one of its scope (a row without a time counts as retired now); `undefined` when no pointer is readable, because then nothing is known to be unused. */
export function readDependencyPointersV1(storeRoot:string,nowMs:number=Date.now()):Readonly<{current:readonly string[];retained:readonly DependencyGenerationRetainedV1[]}>|undefined{
 const directory=join(storeRoot,policy.pointerDirectory),current=new Set<string>(),retiredAt=new Map<string,number>(),newest=new Set<string>();
 let names:string[]=[];try{names=readdirSync(directory).filter(name=>name.endsWith(".pointer.json"));}catch{return undefined;}
 if(names.length===0)return undefined;
 for(const name of names){
  let pointer:{generation?:unknown;retired?:unknown};try{pointer=JSON.parse(readFileSync(join(directory,name),"utf8"));}catch{return undefined;}
  if(typeof pointer.generation!=="string"||pointer.generation.length<20)return undefined;
  current.add(pointer.generation.slice(0,20));
  let latest:{key:string;atMs:number}|undefined;
  for(const row of Array.isArray(pointer.retired)?pointer.retired:[]){
   const id=typeof row==="string"?row:(row as {generation?:unknown})?.generation,at=(row as {atMs?:unknown})?.atMs;
   if(typeof id!=="string"||id.length<20)continue;
   const key=id.slice(0,20),atMs=typeof at==="number"&&Number.isFinite(at)?at:nowMs;
   retiredAt.set(key,Math.max(retiredAt.get(key)??0,atMs));
   if(latest===undefined||atMs>=latest.atMs)latest={key,atMs};
  }
  if(latest!==undefined)newest.add(latest.key);
 }
 const retained=[...retiredAt].filter(([key])=>!current.has(key)).map(([key,atMs])=>({key,retiredSeconds:Math.max(0,Math.floor((nowMs-atMs)/1000)),newest:newest.has(key)})).sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
 return{current:[...current].sort(),retained};
}

/** 🔗️ The generations that a checkout still binds: every `node_modules` link named by the bindings of any ready generation that resolves into the store. Liveness by use, independent of pointers and holds, so a stage of an older binary that took no hold keeps what it runs on. `undefined` when a binding record exists but cannot be read, because then nothing is known to be unbound. */
export function readBoundDependencyGenerationsV1(storeRoot:string):readonly string[]|undefined{
 const generations=join(storeRoot,policy.generationDirectory),bound=new Set<string>(),roots=[resolve(generations)];
 try{roots.push(realpathSync(generations));}catch{}
 let names:string[]=[];try{names=readdirSync(generations,{withFileTypes:true}).filter(entry=>entry.isDirectory()&&KEY.test(entry.name)).map(entry=>entry.name);}catch{return[];}
 const mark=(target:string)=>{for(const root of roots){const child=relative(root,target);if(child&&!child.startsWith("..")&&!isAbsolute(child)){const name=child.split(sep)[0]!;if(KEY.test(name))bound.add(name);}}};
 for(const name of names){
  let rows:unknown;
  try{rows=(JSON.parse(readFileSync(join(generations,name,policy.readyMarker),"utf8")) as {bindings?:unknown}).bindings;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;return undefined;}
  if(rows===undefined)continue;
  if(!Array.isArray(rows))return undefined;
  for(const row of rows){
   const source=(row as {sourceDirectory?:unknown})?.sourceDirectory;
   if(typeof source!=="string")return undefined;
   try{if(!lstatSync(source).isSymbolicLink())continue;}catch{continue;}
   try{mark(resolve(dirname(source),readlinkSync(source)));}catch{}
   try{mark(realpathSync(source));}catch{}
  }
 }
 return[...bound].sort();
}

/** 🕰️ When anything directly inside a generation changed last: an install writes into its top-level entries long before the directory itself changes. */
const newestChangeMs=(path:string):number=>{
 let newest=statSync(path).mtimeMs;
 try{for(const entry of readdirSync(path)){try{newest=Math.max(newest,lstatSync(join(path,entry)).mtimeMs);}catch{}}}catch{}
 return newest;
};

/** 🌡️ The free fraction of the volume that holds the store, 1 when it cannot be measured (unknown is never pressure). */
const volumeFreeFraction=(root:string):number=>{
 const facts=readFreeSpaceV1(root,{nearest:true});
 if(!facts.known)return 1;
 const bytes=facts.totalBytes!==undefined&&facts.totalBytes>0?facts.availableBytes/facts.totalBytes:1,inodes=facts.freeInodes!==null&&facts.totalInodes!==undefined&&facts.totalInodes>0?facts.freeInodes/facts.totalInodes:1;
 return Math.min(bytes,inodes);
};

export type DependencyGenerationCollectionPlanV1=Readonly<{retire:readonly DependencyGenerationRetirementV1[];tombstones:readonly string[];kept:number;pressure:boolean;refused?:string}>;
export type DependencyGenerationPlanOptionsV1=Readonly<{now?:()=>number;alive?:(pid:number)=>boolean;commands?:()=>readonly string[]|undefined;only?:ReadonlySet<string>;freeFraction?:()=>number}>;

/** 🔭️ The generations that a live process runs from: the keys whose directory a command line of the process table names (any separator, any case), so a server started from an installed `node_modules` by any tool keeps it without having announced a hold; `undefined` when the table cannot be read, because then nothing is known to be unused. */
export function dependencyGenerationsInUseV1(keys:readonly string[],commands:()=>readonly string[]|undefined=()=>processTableSnapshot()?.map(row=>row.command)):ReadonlySet<string>|undefined{
 if(keys.length===0)return new Set();
 const rows=commands();if(rows===undefined)return undefined;
 const text=rows.join(" ").split("\\").join("/").toLowerCase(),marker="/"+policy.generationDirectory+"/";
 return new Set(keys.filter(key=>new RegExp(marker+key+"(?![a-f0-9])","u").test(text)));
}

/** 🗺️ Reads the store and plans one collection without touching it: the generations to retire, the tombstones of earlier interrupted removals to finish, and whether the image is under pressure. Refuses (retires nothing, still lists the tombstones, which are garbage already) when no pointer or no binding record can be read. */
export function planDependencyGenerationCollectionV1(storeRoot:string,options:DependencyGenerationPlanOptionsV1={}):DependencyGenerationCollectionPlanV1{
 const now=(options.now??Date.now)(),generations=join(storeRoot,policy.generationDirectory),pressure=(options.freeFraction??(()=>volumeFreeFraction(storeRoot)))()<policy.pressureFreeFraction;
 let entries:string[]=[],tombstones:string[]=[];
 try{const rows=readdirSync(generations,{withFileTypes:true}).filter(entry=>entry.isDirectory());entries=rows.map(entry=>entry.name).filter(name=>KEY.test(name));tombstones=rows.map(entry=>entry.name).filter(name=>TOMBSTONE.test(name)).map(name=>name.slice(policy.tombstonePrefix.length)).sort();}catch{return{retire:[],tombstones:[],kept:0,pressure};}
 const pointers=readDependencyPointersV1(storeRoot,now);
 if(pointers===undefined)return{retire:[],tombstones,kept:0,pressure,refused:"no readable dependency pointer"};
 const bound=readBoundDependencyGenerationsV1(storeRoot);
 if(bound===undefined)return{retire:[],tombstones,kept:0,pressure,refused:"no readable dependency binding record"};
 const facts:DependencyGenerationFactV1[]=[];
 for(const key of entries){
  let mtime:number;try{mtime=newestChangeMs(join(generations,key));}catch{continue;}
  facts.push({key,ready:existsSync(join(generations,key,policy.readyMarker)),ageSeconds:Math.max(0,Math.floor((now-mtime)/1000))});
 }
 const held=readHeldDependencyGenerationsV1(storeRoot,options.alive),due=planDependencyGenerationRetirementV1({generations:facts,current:pointers.current,retained:pointers.retained,held,bound,pressure}),used=dependencyGenerationsInUseV1(due.map(row=>row.key),options.commands);
 if(used===undefined)return{retire:[],tombstones,kept:facts.length,pressure,refused:"no readable process table"};
 const all=due.filter(row=>!used.has(row.key)),retire=all.filter(row=>options.only===undefined||options.only.has(row.key));
 return{retire,tombstones,kept:facts.length-retire.length,pressure};
}

export type DependencyGenerationProgressV1=Readonly<{key:string;phase:"finishing"|"retiring";entries:number;done:number;total:number}>;
export type DependencyGenerationCollectionOptionsV1=DependencyGenerationPlanOptionsV1&Readonly<{dryRun:boolean;signal?:AbortSignal;batchEntries?:number;maximumMilliseconds?:number;clock?:()=>number;progress?:(row:DependencyGenerationProgressV1)=>void|Promise<void>}>;
export type DependencyGenerationCollectionResultV1=Readonly<{retired:readonly DependencyGenerationRetirementV1[];finished:readonly string[];kept:number;entries:number;pressure:boolean;more:boolean;refused?:string}>;

const budgetSpent=Symbol("dependency generation collection budget");

/** 🧺️ Removes one directory tree in batches of `batch` entries: after every batch it lets the process breathe, reports the entries removed so far and checks cancellation and the time budget, so a tree of any size is removed without one long block. What is left after an interruption is simply the rest of the tree, the cursor of the next run. A directory that is not writable is opened first; an entry that is gone is fine. Returns the entries removed by this call. */
async function removeTreePagedV1(path:string,batch:number,tick:(removed:number)=>Promise<void>):Promise<number>{
 let removed=0;
 const gone=(error:unknown)=>(error as NodeJS.ErrnoException)?.code==="ENOENT",count=async():Promise<void>=>{if(++removed%batch===0)await tick(removed);};
 const walk=async(directory:string):Promise<void>=>{
  let rows:import("node:fs").Dirent[];
  try{if((lstatSync(directory).mode&0o700)!==0o700)chmodSync(directory,0o700);rows=readdirSync(directory,{withFileTypes:true});}catch(error){if(gone(error))return;throw error;}
  for(const row of rows){
   const child=join(directory,row.name);
   if(row.isDirectory())await walk(child);else try{unlinkSync(child);await count();}catch(error){if(!gone(error))throw error;}
  }
  try{rmdirSync(directory);await count();}catch(error){if(!gone(error))throw error;}
 };
 await walk(path);
 return removed;
}

/** 🧹️ Retires what `planDependencyGenerationCollectionV1` names, even while other processes use the store, in bounded batches: first the tombstones of earlier interrupted removals, then every planned generation, which is renamed to its tombstone after its ready marker went (so a crash leaves a tombstone that the next run finishes, never a half-removed generation under its name) and removed batch by batch. Reports progress per batch, stops at the signal (throws) or at the time budget (`more` is true, the tombstone stays), and never gives up on a store because it is large: any number of generations and entries is paged through. A failing removal leaves its tombstone and the next generation goes on. */
export async function collectDependencyGenerationsV1(storeRoot:string,options:DependencyGenerationCollectionOptionsV1):Promise<DependencyGenerationCollectionResultV1>{
 const generations=join(storeRoot,policy.generationDirectory),plan=planDependencyGenerationCollectionV1(storeRoot,options),batch=Math.max(1,options.batchEntries??policy.batchEntries),clock=options.clock??(()=>performance.now()),started=clock(),total=plan.tombstones.length+plan.retire.length,retired:DependencyGenerationRetirementV1[]=[],finished:string[]=[];
 let entries=0,more=false,done=0;
 const base={kept:plan.kept,pressure:plan.pressure,...plan.refused===undefined?{}:{refused:plan.refused}};
 if(options.dryRun)return{retired:plan.retire,finished:plan.tombstones,entries:0,more:false,...base};
 const remove=async(key:string,phase:"finishing"|"retiring"):Promise<boolean>=>{
  const tombstone=join(generations,policy.tombstonePrefix+key);
  try{
   const removed=await removeTreePagedV1(tombstone,batch,async count=>{
    options.signal?.throwIfAborted();
    await options.progress?.({key,phase,entries:entries+count,done,total});
    await new Promise<void>(accept=>setImmediate(accept));
    options.signal?.throwIfAborted();
    if(options.maximumMilliseconds!==undefined&&clock()-started>=options.maximumMilliseconds)throw budgetSpent;
   });
   entries+=removed;await options.progress?.({key,phase,entries,done:done+1,total});return true;
  }catch(error){
   if(error===budgetSpent){more=true;return false;}
   if(options.signal?.aborted)throw error;
   return false;
  }
 };
 for(const key of plan.tombstones){
  if(more)break;
  if(await remove(key,"finishing"))finished.push(key);
  done++;
 }
 for(const row of plan.retire){
  if(more)break;
  options.signal?.throwIfAborted();
  const path=join(generations,row.key),tombstone=join(generations,policy.tombstonePrefix+row.key);
  try{
   if(!lstatSync(path).isDirectory())continue;
   try{unlinkSync(join(path,policy.readyMarker));}catch{}
   renameSync(path,tombstone);
  }catch{done++;continue;}
  if(await remove(row.key,"retiring"))retired.push(row);
  done++;
 }
 return{retired,finished,entries,more,...base,kept:plan.kept+plan.retire.length-retired.length};
}

export type DependencyGenerationPinV1=Readonly<{release:()=>void}>;

/** 📌️ Collects like `collectDependencyGenerationsV1` but only what it can pin exclusively: every candidate is first claimed through the shared pin leases that readers and producers of any binary hold, a candidate that a live pin holds is kept, and an unreadable lease registry (the claim throws) releases everything and retires nothing (the tombstones are still finished: nobody can pin what is already retired). `acquire` returns the exclusive pin, or `null` while a reader holds it. The pins stay held during the removal and are released afterwards. */
export async function collectUnpinnedDependencyGenerationsV1(storeRoot:string,options:DependencyGenerationCollectionOptionsV1&Readonly<{acquire:(key:string)=>Promise<DependencyGenerationPinV1|null>}>):Promise<DependencyGenerationCollectionResultV1>{
 const base=planDependencyGenerationCollectionV1(storeRoot,options);
 if(base.refused!==undefined||options.dryRun)return collectDependencyGenerationsV1(storeRoot,options);
 const pins=new Map<string,DependencyGenerationPinV1>(),release=()=>{for(const pin of pins.values())try{pin.release();}catch{}pins.clear();};
 try{
  for(const row of base.retire){options.signal?.throwIfAborted();let pin:DependencyGenerationPinV1|null;try{pin=await options.acquire(row.key);}catch{release();return{...await collectDependencyGenerationsV1(storeRoot,{...options,only:new Set()}),refused:"no readable dependency pin lease registry"};}if(pin!==null)pins.set(row.key,pin);}
  const result=await collectDependencyGenerationsV1(storeRoot,{...options,only:new Set(pins.keys())});
  return result;
 }finally{release();}
}

/** 🕳️ Names the first visible top-level entry of a bound `node_modules` directory that does not resolve (a link whose target is gone), or `undefined` when every one resolves: a cheap physical walk that no receipt or hash may replace. Hidden entries (`.bin`, `.cache`) are not part of the resolution contract. */
export function danglingDependencyEntryV1(directory:string):string|undefined{
 for(const name of readdirSync(directory).sort()){if(name.startsWith("."))continue;try{statSync(join(directory,name));}catch{return name;}}
 return undefined;
}

export type DependencyGenerationDiscardV1=Readonly<{receipts:number;generations:number}>;

/** 🧯️ Makes the next dependency synchronization rebuild instead of trusting what it cannot use: removes every synchronization receipt and the ready marker of each generation a pointer names as current, so the producer treats them as unfinished, resumes or rebuilds them and republishes. Used when a verified-current store still lacks the tooling compiler. Best effort, never throws. */
export function discardDependencyGenerationsV1(storeRoot:string):DependencyGenerationDiscardV1{
 let receipts=0,generations=0;
 try{const directory=join(storeRoot,"generation");for(const name of readdirSync(directory))if(name.endsWith(".sync.json")){try{unlinkSync(join(directory,name));receipts++;}catch{}}}catch{}
 try{
  const directory=join(storeRoot,policy.pointerDirectory);
  for(const name of readdirSync(directory).filter(entry=>entry.endsWith(".pointer.json"))){
   try{const pointer=JSON.parse(readFileSync(join(directory,name),"utf8")) as {generation?:unknown};if(typeof pointer.generation!=="string"||!KEY.test(pointer.generation.slice(0,20)))continue;unlinkSync(join(storeRoot,policy.generationDirectory,pointer.generation.slice(0,20),policy.readyMarker));generations++;}catch{}
  }
 }catch{}
 return{receipts,generations};
}
