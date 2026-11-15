import {readdirSync,readFileSync,rmSync,rmdirSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {classifySlotsV1,SLOT_GRACE_MILLISECONDS_V1,type SlotRowV1} from "../../../../../🏗️build/🪢️slots/🟦️.ts";
import {reclaimCasObjectsV1,type CasReclaimV1} from "../../../../../🏗️build/🗄️cas/🪦️reclaim/🟦️.ts";
import {casRootV1,createCasStoreV1,type CasStoreV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";

export {claimUnitV1} from "./🙋️claim/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout;

/** 🔌️ The crash points of one removal, for tests: before the first variant file goes, after it went and after the entry directory went. */
export type UnitSupersessionStepV1="before-remove"|"after-remove"|"after-directory";

/** 📊️ What one supersession pass saw and did (or, for a dry run, would do): entries scanned, entries with a slot, slots whose newest key is current, entries retained within the grace, entries removed (keys and files) and the objects reclaimed from the host store. */
export type UnitSupersessionV1=Readonly<{dryRun:boolean;scanned:number;slotted:number;current:number;retained:number;removedKeys:number;removedFiles:number;removedKeysList:readonly string[];reclaimed:CasReclaimV1}>;

type Row=SlotRowV1&{directory:string;newest:string;files:string[];refs:string[]};

const none:CasReclaimV1={entries:0,bytes:0,pinned:0,recent:0,shared:0,missing:0};
const yielding=()=>new Promise<void>(done=>setImmediate(done));
const refsOf=(entry:{outputs?:{sha256?:unknown}[];stdout?:unknown;stderr?:unknown}):string[]=>[...(entry.outputs??[]).map(row=>row.sha256),entry.stdout,entry.stderr].filter((sha):sha is string=>typeof sha==="string"&&/^[0-9a-f]{64}$/u.test(sha));

/** 🚶️ Reads every entry key of a unit cache root (`entries` and `runs`, `<shard>/<key>/<variants>`): its slot and time stored from the newest variant, the objects all variants reference. A key without a readable slot is unslotted and left to the older rules. */
export async function scanUnitSlotsV1(root:string,signal?:AbortSignal):Promise<Row[]>{
 const rows:Row[]=[];let done=0;
 for(const kind of[layout.entries,layout.runs]){
  const base=join(root,kind);let shards:string[];try{shards=readdirSync(base);}catch{continue;}
  for(const shard of shards){
   let keys:string[];try{keys=readdirSync(join(base,shard));}catch{continue;}
   for(const key of keys){
    signal?.throwIfAborted();if(++done%256===0)await yielding();
    const directory=join(base,shard,key);let files:string[];try{files=readdirSync(directory).filter(name=>name.endsWith(".json")).sort();}catch{continue;}
    if(files.length===0){try{if(Date.now()-statSync(directory).mtimeMs>60000)rmdirSync(directory);}catch{}continue;}
    let slot:string|null=null,storedAt=0,accessed=0,size=0;const refs=new Set<string>();
    for(const file of files){
     try{
      const text=readFileSync(join(directory,file),"utf8"),entry=JSON.parse(text) as {slot?:unknown;createdMs?:unknown;outputs?:{sha256?:unknown}[];stdout?:unknown;stderr?:unknown};
      for(const sha of refsOf(entry))refs.add(sha);size+=text.length;
      const created=typeof entry.createdMs==="number"?entry.createdMs:statSync(join(directory,file)).mtimeMs;
      if(created>=storedAt){storedAt=created;slot=typeof entry.slot==="string"&&/^[0-9a-f]{64}$/u.test(entry.slot)?entry.slot:null;}
      accessed=Math.max(accessed,statSync(join(directory,file)).mtimeMs);
     }catch{}
    }
    rows.push({key:`${kind}/${shard}/${key}`,slot,storedAt,accessed,size,directory,newest:files[files.length-1]!,files,refs:[...refs]});
   }
  }
 }
 return rows;
}

/** 🪢️ Drops the keys a newer key of the same slot superseded beyond the grace, so a worker's cache holds the current state and not its history; the objects only they referenced go back to the host store, which keeps whatever a live pin, a recent access or a workspace link protects. A key is re-read right before it goes (a store or claim since the scan keeps it), the variant files go first and the directory last (a crash leaves a readable cache), and a pass of a dry run only counts. */
export async function supersedeUnitsV1(root:string,options:Readonly<{nowMs?:number;graceMilliseconds?:number;dryRun?:boolean;host?:Pick<CasStoreV1,"root"|"policy"|"objectPath">;signal?:AbortSignal;onStep?:(point:UnitSupersessionStepV1)=>void}>={}):Promise<UnitSupersessionV1>{
 const nowMs=options.nowMs??Date.now(),grace=options.graceMilliseconds??SLOT_GRACE_MILLISECONDS_V1,rows=await scanUnitSlotsV1(root,options.signal),verdicts=classifySlotsV1(rows,{now:nowMs,graceMilliseconds:grace});
 let slotted=0,current=0,retained=0;const doomed:Row[]=[];
 for(const row of rows){
  const verdict=verdicts.get(row.key)!;
  if(row.slot!==null)slotted++;
  if(verdict.state==="current")current++;else if(verdict.state==="retained")retained++;else if(verdict.state==="expired")doomed.push(row);
 }
 const removed:Row[]=[];let removedFiles=0;
 for(const row of doomed){
  options.signal?.throwIfAborted();
  if(options.dryRun!==true){
   let names:string[];try{names=readdirSync(row.directory).filter(name=>name.endsWith(".json")).sort();}catch{continue;}
   if(names[names.length-1]!==row.newest||names.length!==row.files.length)continue;
   options.onStep?.("before-remove");
   for(const name of names){rmSync(join(row.directory,name),{force:true});removedFiles++;options.onStep?.("after-remove");}
   try{rmdirSync(row.directory);}catch{}
   options.onStep?.("after-directory");
  }else removedFiles+=row.files.length;
  removed.push(row);
 }
 let reclaimed=none;
 if(options.host&&removed.length>0){
  const gone=new Set(removed.map(row=>row.key)),live=new Set<string>();
  for(const row of rows)if(!gone.has(row.key))for(const sha of row.refs)live.add(sha);
  const candidates=new Set<string>();for(const row of removed)for(const sha of row.refs)if(!live.has(sha))candidates.add(sha);
  reclaimed=await reclaimCasObjectsV1(options.host,candidates,{dryRun:options.dryRun===true,...options.signal?{signal:options.signal}:{}});
 }
 return Object.freeze({dryRun:options.dryRun===true,scanned:rows.length,slotted,current,retained,removedKeys:removed.length,removedFiles,removedKeysList:Object.freeze(removed.map(row=>row.key)),reclaimed});
}

/** ⏱️ The minimum time between two supersession passes of one cache root: a pass reads every entry, the watching collector wakes far more often. */
export const UNIT_SUPERSESSION_INTERVAL_MILLISECONDS_V1=300000;

/** 🪢️ The pass the collectors run (command, maintenance, free-space pressure, the watching collector): at most once per interval, against the host store of this account, and never failing the collection. Answers the number of keys removed. */
export async function supersedeLocalUnitsV1(root:string,nowMs:number,signal?:AbortSignal,options:Readonly<{graceMilliseconds?:number;intervalMilliseconds?:number;host?:Pick<CasStoreV1,"root"|"policy"|"objectPath">}>={}):Promise<number>{
 const stamp=join(root,"supersede.stamp");
 try{
  let last=0;try{last=statSync(stamp).mtimeMs;}catch{}
  if(nowMs-last<(options.intervalMilliseconds??UNIT_SUPERSESSION_INTERVAL_MILLISECONDS_V1))return 0;
  writeFileSync(stamp,String(nowMs));utimesSync(stamp,nowMs/1000,nowMs/1000);
  const result=await supersedeUnitsV1(root,{nowMs,...options.graceMilliseconds===undefined?{}:{graceMilliseconds:options.graceMilliseconds},host:options.host??createCasStoreV1({root:casRootV1()}),...signal?{signal}:{}});
  return result.removedKeys;
 }catch{return 0;}
}
