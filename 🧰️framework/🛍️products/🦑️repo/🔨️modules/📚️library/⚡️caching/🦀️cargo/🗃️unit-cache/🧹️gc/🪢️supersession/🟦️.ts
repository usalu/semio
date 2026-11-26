import {existsSync,appendFileSync,readdirSync,readFileSync,rmSync,rmdirSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {SLOT_GRACE_MILLISECONDS_V1,type DeclarationV1,type StalenessModeV1} from "../../../../../🏗️build/🪢️slots/🟦️.ts";
import {judgeStalenessV1,openTreeDigestsV1,stalenessReportV1,type StalenessVerdictV1} from "../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {reclaimCasObjectsV1,type CasReclaimV1} from "../../../../../🏗️build/🗄️cas/🪦️reclaim/🟦️.ts";
import {resetCasHeldCacheV1} from "../../../../../🏗️build/🗄️cas/🫂️holders/🟦️.ts";
import {casRootV1,createCasStoreV1,type CasStoreV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";

export {claimUnitV1} from "./🙋️claim/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout,staleness=UNIT_CACHE_POLICY_V1.gc.staleness;

/** 🔌️ The crash points of one removal, for tests: before the first variant file goes, after it went and after the entry directory went. */
export type UnitRetirementStepV1="before-remove"|"after-remove"|"after-directory";

/** 📊️ What one retirement pass saw and did (or, outside enforce mode, would do): entries scanned, entries that declared what they were built from, live ones, ones within the grace, stale ones by reason, unreached ones, entries removed (keys and files) and the objects reclaimed from the host store. */
export type UnitRetirementV1=Readonly<{mode:StalenessModeV1;scanned:number;declared:number;live:number;grace:number;stale:Readonly<Record<string,number>>;unreached:number;removedKeys:number;removedFiles:number;removedKeysList:readonly string[];wouldRemoveKeys:number;examples:readonly string[];reclaimed:CasReclaimV1}>;

type Row={key:string;storedAt:number;accessed:number;directory:string;newest:string;files:string[];refs:string[];declaration:DeclarationV1|undefined};

const none:CasReclaimV1={entries:0,bytes:0,pinned:0,held:0,recent:0,shared:0,missing:0,leased:0};
const yielding=()=>new Promise<void>(done=>setImmediate(done));
const refsOf=(entry:{outputs?:{sha256?:unknown}[];stdout?:unknown;stderr?:unknown}):string[]=>[...(entry.outputs??[]).map(row=>row.sha256),entry.stdout,entry.stderr].filter((sha):sha is string=>typeof sha==="string"&&/^[0-9a-f]{64}$/u.test(sha));

/** 🚶️ Reads every entry key of a unit cache root (`entries` and `runs`, `<shard>/<key>/<variants>`): when it was stored and what it declared it was built from (from its newest variant) and the objects all variants reference. A key whose newest variant declared nothing is judged by the older rules only. */
export async function scanUnitDeclarationsV1(root:string,signal?:AbortSignal):Promise<Row[]>{
 const rows:Row[]=[];let done=0;
 for(const kind of[layout.entries,layout.runs]){
  const base=join(root,kind);let shards:string[];try{shards=readdirSync(base);}catch{continue;}
  for(const shard of shards){
   let keys:string[];try{keys=readdirSync(join(base,shard));}catch{continue;}
   for(const key of keys){
    signal?.throwIfAborted();if(++done%256===0)await yielding();
    const directory=join(base,shard,key);let files:string[];try{files=readdirSync(directory).filter(name=>name.endsWith(".json")).sort();}catch{continue;}
    if(files.length===0){try{if(Date.now()-statSync(directory).mtimeMs>60000)rmdirSync(directory);}catch{}continue;}
    let declaration:DeclarationV1|undefined,storedAt=0,accessed=0;const refs=new Set<string>();
    for(const file of files){
     try{
      const entry=JSON.parse(readFileSync(join(directory,file),"utf8")) as {retention?:DeclarationV1;createdMs?:unknown;outputs?:{sha256?:unknown}[];stdout?:unknown;stderr?:unknown};
      for(const sha of refsOf(entry))refs.add(sha);
      const created=typeof entry.createdMs==="number"?entry.createdMs:statSync(join(directory,file)).mtimeMs;
      if(created>=storedAt){storedAt=created;declaration=entry.retention;}
      accessed=Math.max(accessed,statSync(join(directory,file)).mtimeMs);
     }catch{}
    }
    rows.push({key:`${kind}/${shard}/${key}`,storedAt,accessed,directory,newest:files[files.length-1]!,files,refs:[...refs],declaration});
   }
  }
 }
 return rows;
}

/** 🥀️ Judges every entry of a unit cache against the synchronized tree of this machine (the workspace, itself the replica of the coordinator's tree) and, in enforce mode, drops the keys that are stale (a recorded source differs, is gone, or a unit they consumed is stale) or unreached (a registry or standard library unit no live unit consumes) beyond the grace, so a worker's cache holds the builds of the current tree and not its history. The objects only they referenced go back to the host store, which keeps whatever a live pin, a recent access or a workspace link protects. A key is re-read right before it goes (a store since the scan keeps it), the variant files go first and the directory last (a crash leaves a readable cache); outside enforce mode a pass only reports. */
export async function retireStaleUnitsV1(root:string,options:Readonly<{workspace:string;mode?:StalenessModeV1;nowMs?:number;graceMilliseconds?:number;toolchains?:ReadonlySet<string>;host?:Pick<CasStoreV1,"root"|"policy"|"objectPath">;signal?:AbortSignal;onStep?:(point:UnitRetirementStepV1)=>void}>):Promise<UnitRetirementV1>{
 const mode=options.mode??staleness.mode as StalenessModeV1,nowMs=options.nowMs??Date.now(),grace=options.graceMilliseconds??SLOT_GRACE_MILLISECONDS_V1;
 if(mode==="off")return Object.freeze({mode,scanned:0,declared:0,live:0,grace:0,stale:{},unreached:0,removedKeys:0,removedFiles:0,removedKeysList:[],wouldRemoveKeys:0,examples:[],reclaimed:none});
 const rows=await scanUnitDeclarationsV1(root,options.signal),tree=await openTreeDigestsV1(options.workspace,join(root,"retention-digests.jsonl"));
 const verdicts=await judgeStalenessV1(rows.map(row=>({key:row.key,storedAt:row.storedAt,refs:row.refs,declaration:row.declaration})),{now:nowMs,graceMilliseconds:grace,digest:tree.digest,toolchains:options.toolchains,signal:options.signal});
 await tree.flush().catch(()=>undefined);
 const report=stalenessReportV1({mode,now:nowMs,verdicts,retired:0,refused:{},examples:staleness.examples}),doomed=rows.filter(row=>{const state=(verdicts.get(row.key) as StalenessVerdictV1).state;return state==="stale"||state==="unreached";});
 const removed:Row[]=[];let removedFiles=0;
 for(const row of doomed){
  options.signal?.throwIfAborted();
  if(mode==="enforce"){
   let names:string[];try{names=readdirSync(row.directory).filter(name=>name.endsWith(".json")).sort();}catch{continue;}
   if(names[names.length-1]!==row.newest||names.length!==row.files.length)continue;
   options.onStep?.("before-remove");
   for(const name of names){rmSync(join(row.directory,name),{force:true});removedFiles++;options.onStep?.("after-remove");}
   try{rmdirSync(row.directory);}catch{}
   options.onStep?.("after-directory");
   removed.push(row);
  }
 }
 let reclaimed=none;
 if(options.host&&removed.length>0){
  const gone=new Set(removed.map(row=>row.key)),live=new Set<string>();
  for(const row of rows)if(!gone.has(row.key))for(const sha of row.refs)live.add(sha);
  const candidates=new Set<string>();for(const row of removed)for(const sha of row.refs)if(!live.has(sha))candidates.add(sha);
  resetCasHeldCacheV1();
  reclaimed=await reclaimCasObjectsV1(options.host,candidates,{...options.signal?{signal:options.signal}:{}});
 }
 try{appendFileSync(join(root,"retention.jsonl"),JSON.stringify({...report,retired:removed.length})+"\n");}catch{}
 return Object.freeze({mode,scanned:rows.length,declared:report.declared,live:report.live,grace:report.grace,stale:report.stale,unreached:report.unreached,removedKeys:removed.length,removedFiles,removedKeysList:Object.freeze(removed.map(row=>row.key)),wouldRemoveKeys:doomed.length,examples:Object.freeze(report.examples),reclaimed});
}

/** ⏱️ The minimum time between two retirement passes of one cache root: a pass reads every entry, the watching collector wakes far more often. */
export const UNIT_RETIREMENT_INTERVAL_MILLISECONDS_V1=staleness.intervalMilliseconds;

/** 🌳️ The synchronized tree this cache root belongs to: the workspace the launcher names, else the nearest ancestor of the root that holds a `.git` entry; `undefined` when neither is known. */
export function unitWorkspaceOfV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const named=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT;if(named)return named;
 for(let at=dirname(root);at!==dirname(at);at=dirname(at))if(existsSync(join(at,".git")))return at;
 return undefined;
}

/** 🥀️ The pass the collectors run (command, maintenance, free-space pressure, the watching collector): at most once per interval, against the host store of this account, in the mode of the cache policy (`dry-run` by default: it only records) and never failing the collection. Answers the number of keys removed. */
export async function retireLocalStaleUnitsV1(root:string,nowMs:number,signal?:AbortSignal,options:Readonly<{graceMilliseconds?:number;intervalMilliseconds?:number;mode?:StalenessModeV1;workspace?:string;host?:Pick<CasStoreV1,"root"|"policy"|"objectPath">}>={}):Promise<number>{
 const stamp=join(root,"retire.stamp");
 try{
  const workspace=options.workspace??unitWorkspaceOfV1(root);if(workspace===undefined)return 0;
  let last=0;try{last=statSync(stamp).mtimeMs;}catch{}
  if(nowMs-last<(options.intervalMilliseconds??UNIT_RETIREMENT_INTERVAL_MILLISECONDS_V1))return 0;
  writeFileSync(stamp,String(nowMs));utimesSync(stamp,nowMs/1000,nowMs/1000);
  const result=await retireStaleUnitsV1(root,{workspace,nowMs,...options.mode===undefined?{}:{mode:options.mode},...options.graceMilliseconds===undefined?{}:{graceMilliseconds:options.graceMilliseconds},host:options.host??createCasStoreV1({root:casRootV1()}),...signal?{signal}:{}});
  return result.removedKeys;
 }catch{return 0;}
}
