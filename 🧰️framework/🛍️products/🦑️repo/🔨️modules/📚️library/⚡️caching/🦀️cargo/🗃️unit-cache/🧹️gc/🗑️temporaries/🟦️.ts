import {lstatSync,type Stats,readFileSync,readdirSync,rmSync,rmdirSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {laneIdsV1,lanePathsV1} from "../../🧵️lane/🟦️.ts";
import {processAliveV1} from "../../🔒️lock/🟦️.ts";
import type {CargoGcControlV1} from "../📏️tree/🟦️.ts";
import {parseProcessIdentityV1} from "../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const,cache=UNIT_CACHE_POLICY_V1;
/** 📐️ The authored policy of the compiler temporaries collector.
 * @see ./🧬️schema/🔣️.json */
export const TEMPORARIES_POLICY_V1=policy;
/** 🫀️ One running holder of a lane: when it started, `null` when its record cannot be read (then nothing of the lane is known to be older than it). */
export type TemporariesHolderV1=Readonly<{startedMs:number|null}>;
/** 📄️ One file below a lane's compiler temporaries directory. */
export type TemporariesFileV1=Readonly<{path:string;bytes:number;mtimeMs:number}>;
/** 🧵️ One lane with its running holders and its temporary files. */
export type TemporariesLaneV1=Readonly<{id:string;holders:readonly TemporariesHolderV1[];files:readonly TemporariesFileV1[]}>;
/** 📋️ The measured state the pure planner decides on. */
export type TemporariesInputV1=Readonly<{nowMs:number;lanes:readonly TemporariesLaneV1[]}>;
/** 🗑️ One planned removal. */
export type TemporariesRemovalV1=Readonly<{lane:string;path:string;bytes:number}>;

/** 🕰️ The modification time below which a file of a lane is garbage: for a lane without holders everything older than the idle grace before `nowMs` (a compile that holds no marker keeps its fresh files), nothing when a holder is unknown, else the start of the oldest holder minus the grace. Pure. */
export function temporariesCutoffV1(holders:readonly TemporariesHolderV1[],nowMs:number,override:Partial<typeof policy>={}):number{
 if(holders.length===0)return nowMs-{...policy,...override}.idleMilliseconds;
 const starts=holders.map(row=>row.startedMs);if(starts.includes(null))return -Infinity;
 return Math.min(...starts as number[])-{...policy,...override}.graceMilliseconds;
}

/** 🧮️ Plans which temporary files go: the files of an idle lane older than the idle grace, only the files older than the oldest running holder minus the grace of a live lane, nothing of a lane with an unknown holder. Pure and deterministic. */
export function planTemporariesV1(input:TemporariesInputV1,override:Partial<typeof policy>={}):TemporariesRemovalV1[]{
 const byText=(left:string,right:string)=>left<right?-1:left>right?1:0;
 return[...input.lanes].sort((left,right)=>byText(left.id,right.id)).flatMap(lane=>{const cutoff=temporariesCutoffV1(lane.holders,input.nowMs,override);return[...lane.files].filter(file=>file.mtimeMs<cutoff).sort((left,right)=>byText(left.path,right.path)).map(file=>({lane:lane.id,path:file.path,bytes:file.bytes}));});
}

/** 🫀️ The running holders of a lane, read without changing anything: a marker of a dead process is no holder, an unreadable directory or marker is a holder of unknown start, a vanished marker was released. */
export function temporariesHoldersV1(live:string,alive:(holder:Readonly<{pid:number;startMs:number}>)=>boolean=processAliveV1):TemporariesHolderV1[]{
 const unknown:TemporariesHolderV1={startedMs:null};let names:string[];
 try{names=readdirSync(live);}catch(error){return(error as NodeJS.ErrnoException).code==="ENOENT"?[]:[unknown];}
 return names.flatMap(name=>{
  let row:{pid?:unknown;startedMs?:unknown};
  try{row=JSON.parse(readFileSync(join(live,name),"utf8"));}catch(error){return(error as NodeJS.ErrnoException).code==="ENOENT"?[]:[unknown];}
  const holder=parseProcessIdentityV1(row);
  if(holder===undefined)return[unknown];
  return alive(holder)?[{startedMs:Number.isSafeInteger(row.startedMs)?row.startedMs as number:null}]:[];
 });
}

const turn=()=>new Promise<void>(done=>setImmediate(done));
const allocated=(info:Stats):number=>Math.round((info.blocks??Math.ceil(info.size/512))*512/Math.max(1,info.nlink));

/** 🔭️ Reads every lane's holders and temporary files below a cache root, read-only, yielding between bounded batches of entries. */
export async function measureTemporariesV1(root:string,options:CargoGcControlV1&Readonly<{alive?:(holder:Readonly<{pid:number;startMs:number}>)=>boolean}>={}):Promise<TemporariesLaneV1[]>{
 const rows:TemporariesLaneV1[]=[],ids=laneIdsV1(root).filter(id=>!cache.pool.reserved.includes(id));let seen=0,done=0;
 for(const id of ids){
  options.signal?.throwIfAborted();
  const paths=lanePathsV1(root,id),files:TemporariesFileV1[]=[],stack=[paths.temporaries];
  while(stack.length){
   const directory=stack.pop()!;let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{continue;}
   for(const entry of entries){
    options.signal?.throwIfAborted();
    const path=join(directory,entry.name);
    if(entry.isDirectory())stack.push(path);
    else try{const info=lstatSync(path);files.push({path,bytes:allocated(info),mtimeMs:Math.floor(info.mtimeMs)});}catch{}
    if(++seen%policy.scanYieldEntries===0)await turn();
   }
  }
  rows.push({id,holders:temporariesHoldersV1(paths.live,options.alive),files});
  await options.progress?.(++done,ids.length,"scan",paths.temporaries);
 }
 return rows;
}

/** 📊️ What a collection did or, in a dry run, would do. */
export type TemporariesResultV1=Readonly<{dryRun:boolean;lanes:number;liveLanes:number;files:number;removedFiles:number;freedBytes:number}>;
/** 💬️ One localized line with its placeholders filled. */
export function temporariesTextV1(key:keyof typeof messages,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
const gib=(bytes:number)=>bytes>=1073741824?(bytes/1073741824).toFixed(1)+" GiB":(bytes/1048576).toFixed(0)+" MiB";
/** 🗣️ Where and in which language the collection reports; absent, it reports nothing. */
export type TemporariesReportV1=Readonly<{language:"en"|"de";write:(line:string)=>void}>;

/** 🧹️ Measures, plans and (unless `dryRun`) removes the compiler temporaries of every lane of the cache root `root`. Liveness is read again before every batch and every file is checked against it right before its removal, so a compiler that starts meanwhile keeps its files; a vanished file is fine. Removal stops as soon as `satisfied()` says the image has freed what was needed (while `needBytes()` is above zero). Progress and cancellation are served between batches. */
export async function collectTemporariesV1(root:string,options:CargoGcControlV1&Readonly<{dryRun:boolean;needBytes?:()=>number;satisfied?:()=>boolean;alive?:(holder:Readonly<{pid:number;startMs:number}>)=>boolean;report?:TemporariesReportV1;batchFiles?:number;now?:()=>number}>):Promise<TemporariesResultV1>{
 const report=options.report,say=(key:keyof typeof messages,variables:Readonly<Record<string,string|number>>={}):void=>{if(report)report.write(temporariesTextV1(key,report.language,variables));};
 const now=options.now??Date.now;
 say("scan",{root});
 const lanes=await measureTemporariesV1(root,options),plan=planTemporariesV1({nowMs:now(),lanes}),batch=options.batchFiles??policy.batchFiles,total=lanes.reduce((sum,lane)=>sum+lane.files.length,0),liveLanes=lanes.filter(lane=>lane.holders.length>0).length;
 const byLane=new Map<string,TemporariesRemovalV1[]>();for(const row of plan)byLane.set(row.lane,[...byLane.get(row.lane)??[],row]);
 let removedFiles=0,freedBytes=0,done=0;
 for(const lane of lanes){
  const rows=byLane.get(lane.id)??[];if(rows.length===0)continue;
  if(options.dryRun){removedFiles+=rows.length;freedBytes+=rows.reduce((sum,row)=>sum+row.bytes,0);say("would",{lane:lane.id,count:rows.length,bytes:gib(rows.reduce((sum,row)=>sum+row.bytes,0)),kept:lane.files.length-rows.length});continue;}
  const live=lanePathsV1(root,lane.id).live;let laneFiles=0,laneBytes=0,stopped=false;
  for(let from=0;from<rows.length&&!stopped;from+=batch){
   options.signal?.throwIfAborted();
   if(done>0&&(options.needBytes?.()??0)>0&&options.satisfied?.()){stopped=true;break;}
   const cutoff=temporariesCutoffV1(temporariesHoldersV1(live,options.alive),now());
   for(const row of rows.slice(from,from+batch)){
    try{if(lstatSync(row.path).mtimeMs>=cutoff)continue;}catch{continue;}
    try{rmSync(row.path,{force:true});laneFiles++;laneBytes+=row.bytes;}catch{}
   }
   done+=Math.min(batch,rows.length-from);await options.progress?.(done,plan.length,"remove",lane.id);await turn();
  }
  removedFiles+=laneFiles;freedBytes+=laneBytes;
  if(laneFiles>0){say("removed",{lane:lane.id,count:laneFiles,bytes:gib(laneBytes),kept:lane.files.length-laneFiles});pruneEmptyDirectories(lanePathsV1(root,lane.id).temporaries);}
 }
 say(options.dryRun?"summaryPlan":"summaryApply",{count:removedFiles,total,bytes:gib(freedBytes),lanes:lanes.length,live:liveLanes});
 return Object.freeze({dryRun:options.dryRun,lanes:lanes.length,liveLanes,files:total,removedFiles,freedBytes});
}

function pruneEmptyDirectories(directory:string,keep=true):boolean{
 let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return false;}
 let empty=true;for(const entry of entries){if(entry.isDirectory()&&pruneEmptyDirectories(join(directory,entry.name),false))continue;empty=false;}
 if(!empty||keep)return empty;
 try{rmdirSync(directory);return true;}catch{return false;}
}
