import {lstat,mkdir,open,readdir,readFile,stat,unlink} from "node:fs/promises";
import {join} from "node:path";
import {compactCasShardV1,foldCasShardV1,openCasShardViewV1,type CasShardViewV1} from "../📒️ledger/🟦️.ts";
import {casPinDirectoryV1,readCasPinsV1} from "../📌️pins/🟦️.ts";
import {CAS_POLICY_V1,casTextV1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";
import {createCasPlannerV1,type CasPlanTotalsV1,type CasReasonV1} from "./🧮️plan.ts";

export {casCutV1,createCasPlannerV1} from "./🧮️plan.ts";
export type {CasCutV1,CasPlanTotalsV1,CasReasonV1} from "./🧮️plan.ts";

/** 📂️ One loose-file area outside the store: independent cache files below `directory` (for example the host's graph memo or a workspace's build-input closures) whose bytes count against the host cap. `id` names the policy bounds it falls under. */
export type CasAreaSourceV1=Readonly<{id:string;directory:string;skip?:RegExp}>;
/** 📣️ Progress of one collection. */
export type CasProgressV1=Readonly<{stage:"scan"|"evict"|"compact";entries:number;usedBytes:number;freedBytes:number}>;
/** 🔌️ Everything a collection needs from its host. */
export type CasCollectOptionsV1=Readonly<{
 root:string;policy?:CasPolicyV1;areas?:readonly CasAreaSourceV1[];externalBytes?:number;signal?:AbortSignal;dryRun?:boolean;
 now?():number;pid?:number;progress?(event:CasProgressV1):void|Promise<void>;
 beforeEvict?(path:string):void|Promise<void>;
}>;
/** 🧾️ What a collection did. */
export type CasCollectionV1=Readonly<{status:"idle"|"collected"|"locked";partial:boolean;dryRun:boolean;scanned:number;evicted:number;freedBytes:number;busy:number;skipped:number;totals:CasPlanTotalsV1;byReason:Readonly<Record<CasReasonV1,number>>}>;

const STORE="store",zero:CasPlanTotalsV1={totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,needBytes:0};
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const sorted=(names:string[]):string[]=>names.sort((left,right)=>left<right?-1:left>right?1:0);
const TEMPORARY=/(^\.)|(\.tmp$)/u;

type Entry={area:string;path:string;size:number;access:number;protectedEntry:boolean;shard?:string;hash?:string};

async function acquire(root:string,policy:CasPolicyV1,nowMs:number,pid:number):Promise<(()=>Promise<void>)|undefined>{
 const lock=join(root,"v1","gc.lock");await mkdir(join(root,"v1"),{recursive:true});
 for(let attempt=0;attempt<2;attempt++){
  try{
   const handle=await open(lock,"wx");await handle.writeFile(JSON.stringify({pid,atMs:nowMs}));await handle.close();
   return async()=>{await unlink(lock).catch(()=>undefined);};
  }catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   try{
    const held=JSON.parse(await readFile(lock,"utf8")) as {pid:number;atMs:number};
    if(alive(held.pid)&&nowMs-held.atMs<policy.lockStaleMs)return undefined;
   }catch{}
   await unlink(lock).catch(()=>undefined);
  }
 }
 return undefined;
}

async function* walk(directory:string,skip:RegExp,signal?:AbortSignal):AsyncGenerator<{path:string;size:number;mtimeMs:number;atimeMs:number;nlink:number}>{
 let names:string[];try{names=sorted(await readdir(directory));}catch{return;}
 for(const name of names){
  signal?.throwIfAborted();const path=join(directory,name);let info;
  try{info=await lstat(path);}catch{continue;}
  if(info.isSymbolicLink())continue;
  if(info.isDirectory()){yield*walk(path,skip,signal);continue;}
  if(!info.isFile()||skip.test(name))continue;
  yield{path,size:info.size,mtimeMs:info.mtimeMs,atimeMs:info.atimeMs,nlink:info.nlink};
 }
}

async function* entries(options:CasCollectOptionsV1,policy:CasPolicyV1,pins:ReadonlyMap<string,string>,nowMs:number):AsyncGenerator<Entry>{
 const objects=join(options.root,"v1","o");let shards:string[];
 try{shards=sorted(await readdir(objects));}catch{shards=[];}
 for(const shard of shards){
  options.signal?.throwIfAborted();const ledger=await foldCasShardV1(options.root,shard);let names:string[];
  try{names=sorted(await readdir(join(objects,shard)));}catch{continue;}
  for(const name of names){
   const path=join(objects,shard,name),hash=shard+name;let info;try{info=await lstat(path);}catch{continue;}
   if(!info.isFile())continue;
   const access=Math.max(info.mtimeMs,ledger.get(hash)??0);
   yield{area:STORE,path,size:info.size,access,protectedEntry:pins.has(hash)||info.nlink>1||nowMs-access<policy.minAgeMs,shard,hash};
  }
 }
 for(const area of options.areas??[]){
  for await(const file of walk(area.directory,area.skip??TEMPORARY,options.signal)){
   const access=Math.max(file.mtimeMs,file.atimeMs);
   yield{area:area.id,path:file.path,size:file.size,access,protectedEntry:nowMs-access<policy.minAgeMs};
  }
 }
}

async function sweepTemporary(root:string,policy:CasPolicyV1,nowMs:number):Promise<void>{
 const directory=join(root,"v1","t");let names:string[];try{names=await readdir(directory);}catch{return;}
 for(const name of names)try{if(nowMs-(await lstat(join(directory,name))).mtimeMs>policy.temporaryMaxAgeMs)await unlink(join(directory,name));}catch{}
}

/** 🧹️ Collects the host store and its declared areas: scans once (bounded memory, shard by shard), decides with the streaming least-recently-accessed planner, then evicts in a second pass that re-checks pins and fresh access right before every removal. Single-flight per root through a lock file with stale-owner takeover. Idempotent and stateless, so a crash or cancellation leaves a consistent store and the next run simply continues; objects are written by atomic rename, so no partial object is ever visible. */
export async function collectCasV1(options:CasCollectOptionsV1):Promise<CasCollectionV1>{
 const policy=requireCasPolicyV1(options.policy??CAS_POLICY_V1),now=options.now??Date.now,pid=options.pid??process.pid,signal=options.signal;
 const release=await acquire(options.root,policy,now(),pid);
 if(!release)return{status:"locked",partial:false,dryRun:options.dryRun===true,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:zero,byReason:{age:0,area:0,host:0}};
 let scanned=0,evicted=0,freed=0,busy=0,skipped=0,used=0,lastReport=-Infinity,partial=false;const byReason:Record<CasReasonV1,number>={age:0,area:0,host:0};
 const report=async(stage:CasProgressV1["stage"],force=false):Promise<void>=>{
  signal?.throwIfAborted();const at=now();
  if(!force&&at-lastReport<policy.progressMs)return;
  lastReport=at;await options.progress?.({stage,entries:scanned,usedBytes:used,freedBytes:freed});signal?.throwIfAborted();
 };
 try{
  const started=now();await sweepTemporary(options.root,policy,started);
  const bounds=[{id:STORE,capBytes:null,maxAgeMs:null},...(options.areas??[]).map(area=>{const row=policy.areas.find(candidate=>candidate.id===area.id);return{id:area.id,capBytes:row?.capBytes??null,maxAgeMs:row?.maxAgeMs??null};})];
  const planner=createCasPlannerV1({policy,areas:bounds,externalBytes:options.externalBytes??0,nowMs:started});
  let pins=await readCasPinsV1(options.root,started);
  try{
   for await(const entry of entries(options,policy,pins,started)){
    planner.add(entry.area,entry.access,entry.size,entry.protectedEntry);scanned++;used+=entry.size;
    if(scanned%policy.checkEvery===0)await report("scan");
   }
  }catch(error){if(signal?.aborted){partial=true;}else throw error;}
  const plan=partial?undefined:planner.finish();
  if(!plan)return{status:"collected",partial:true,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals:zero,byReason};
  const totals=plan.totals;
  if(totals.plannedBytes<=0){
   await compactAll(options,policy,started);
   return{status:"idle",partial:false,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason};
  }
  const evaluator=plan.evaluator();pins=await readCasPinsV1(options.root,now());let currentShard:string|undefined,view:CasShardViewV1|undefined,pinStamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;
  try{
   for await(const entry of entries(options,policy,pins,started)){
    if(entry.shard!==currentShard){currentShard=entry.shard;pins=await readCasPinsV1(options.root,now());view=entry.shard?await openCasShardViewV1(options.root,entry.shard):undefined;}
    if(entry.protectedEntry)continue;
    const reason=evaluator.decide(entry.area,entry.access,entry.size);
    if(reason===null)continue;
    if(options.dryRun){evicted++;freed+=entry.size;byReason[reason]++;continue;}
    await options.beforeEvict?.(entry.path);
    await view?.refresh();const stamp=(await stat(casPinDirectoryV1(options.root)).catch(()=>undefined))?.mtimeMs??0;if(stamp!==pinStamp){pinStamp=stamp;pins=await readCasPinsV1(options.root,now());}const fresh=entry.hash?Math.max((await lstat(entry.path).catch(()=>undefined))?.mtimeMs??-1,view?.latest(entry.hash)??0):(await lstat(entry.path).catch(()=>undefined))?.atimeMs??-1;
    if(fresh<0)continue;
    if(fresh>entry.access||fresh>=started||(entry.hash&&pins.has(entry.hash))){skipped++;continue;}
    try{await unlink(entry.path);evicted++;freed+=entry.size;used-=entry.size;byReason[reason]++;}
    catch(error){const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")continue;if(code==="EBUSY"||code==="EPERM"||code==="EACCES"){busy++;continue;}throw error;}
    if(evicted%policy.checkEvery===0)await report("evict");
   }
  }catch(error){if(signal?.aborted)partial=true;else throw error;}
  if(!partial&&!options.dryRun)await compactAll(options,policy,started);
  await options.progress?.({stage:"evict",entries:scanned,usedBytes:used,freedBytes:freed});
  return{status:"collected",partial,dryRun:options.dryRun===true,scanned,evicted,freedBytes:freed,busy,skipped,totals,byReason};
 }finally{await release();}
}

async function compactAll(options:CasCollectOptionsV1,policy:CasPolicyV1,nowMs:number):Promise<void>{
 if(options.dryRun)return;
 const objects=join(options.root,"v1","o");let shards:string[];try{shards=sorted(await readdir(objects));}catch{return;}
 for(const shard of shards){
  options.signal?.throwIfAborted();let names:string[];try{names=await readdir(join(objects,shard));}catch{continue;}
  const live=new Set(names.map(name=>shard+name));await compactCasShardV1(options.root,shard,live,await foldCasShardV1(options.root,shard),nowMs,policy);
 }
}

/** 📣️ The one-line report of a collection in the requested language. */
export function formatCasCollectionV1(result:CasCollectionV1,policy:CasPolicyV1,language:"en"|"de"):string{
 const mib=(bytes:number):number=>Math.round(bytes/1048576*10)/10,gib=(bytes:number):number=>Math.round(bytes/1073741824*100)/100;
 if(result.status==="locked")return casTextV1("locked",language);
 if(result.status==="idle")return casTextV1("idle",language,{usedGib:gib(result.totals.totalBytes),capGib:gib(policy.capBytes)});
 const text=casTextV1("done",language,{evicted:result.evicted,freedMib:mib(result.freedBytes),keptGib:gib(result.totals.keptBytes),protectedMib:mib(result.totals.protectedBytes),partial:result.partial?casTextV1("partial",language):""});
 return result.totals.shortfallBytes>0?text+" "+casTextV1("shortfall",language,{overMib:mib(result.totals.shortfallBytes)}):text;
}
