import {lstatSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {UNIT_POLICY_V1} from "../🧱️units/🟦️.ts";
import {allocatedCargoBytesV1,removeCargoGcTreeV1,type CargoGcControlV1} from "../📏️tree/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 📐️ The authored policy of the component run collector.
 * @see ./🧬️schema/🔣️.json */
export const COMPONENT_RUN_POLICY_V1=policy;
/** 🧩️ One run directory of the native component build (`components/runs/w…`): its allocated bytes, the last time anything in it changed, the components it materialized and whether a running build still holds it. */
type ComponentRunMetadataV1=Readonly<{path:string;lastUsedMs:number;components:readonly string[];held:boolean}>;
export type ComponentRunV1=ComponentRunMetadataV1&Readonly<{bytes:number}>;
/** 🗑️ One planned removal. */
export type ComponentRunRemovalV1=Readonly<{path:string;bytes:number}>;

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,byAge=(left:ComponentRunMetadataV1,right:ComponentRunMetadataV1)=>left.lastUsedMs-right.lastUsedMs||order(left.path,right.path);

function supersededRuns<T extends ComponentRunMetadataV1>(runs:readonly T[]):T[]{
 const ordered=[...runs].sort(byAge),settled=ordered.filter(run=>!run.held),latest=new Map<string,T>();
 for(const run of settled)for(const component of run.components)latest.set(component,run);
 const materialized=settled.filter(run=>run.components.length>0).at(-1);
 return settled.filter(run=>run.components.length>0?run.components.every(component=>latest.get(component)!==run):materialized!==undefined&&byAge(materialized,run)>0);
}

/** 🧮️ Plans which runs to remove, oldest first, until `needBytes` are freed: a run that no running build holds and that a newer run supersedes (a newer run materialized every component it did; a run that materialized nothing is superseded by the newest run that materialized anything). Only runs no build holds count as materialized, since a running build may still fail before it links; the newest such run of each component, every held run and every run newer than the newest materialized one always stay. Pure and deterministic. */
export function planComponentRunsV1(input:Readonly<{needBytes:number;runs:readonly ComponentRunV1[]}>):ComponentRunRemovalV1[]{
 const removed:ComponentRunRemovalV1[]=[];let freed=0;
 for(const run of supersededRuns(input.runs)){
  if(freed>=input.needBytes)break;
  removed.push({path:run.path,bytes:run.bytes});freed+=run.bytes;
 }
 return removed;
}

const names=(path:string):string[]=>{try{return readdirSync(path);}catch{return [];}};
const modified=(path:string):number=>{try{return lstatSync(path).mtimeMs;}catch{return 0;}};

/** 🕰️ The last time anything of a run changed: the run folder, its activity folders (logs, report, jobserver) and what they hold; never an access time, which measuring renews. */
export function componentRunRecencyV1(path:string,rules:Pick<typeof policy,"activityDirectories">=policy):number{
 let newest=modified(path);
 for(const name of rules.activityDirectories){const directory=join(path,name);newest=Math.max(newest,modified(directory),...names(directory).map(child=>modified(join(directory,child))));}
 return Math.floor(newest);
}

/** 🧩️ The components a run materialized: the folders of its `w` store, the linked copies counted as their component. */
export function componentRunComponentsV1(path:string,rules:Pick<typeof policy,"materializedDirectory"|"linkedSuffix">=policy):string[]{
 return [...new Set(names(join(path,rules.materializedDirectory)).map(name=>name.endsWith(rules.linkedSuffix)?name.slice(0,-rules.linkedSuffix.length):name))].sort();
}

/** 💬️ One localized line with its placeholders filled. */
export function componentRunTextV1(key:keyof typeof messages,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
const size=(bytes:number)=>bytes>=1073741824?(bytes/1073741824).toFixed(1)+" GiB":(bytes/1048576).toFixed(0)+" MiB";
const turn=()=>new Promise<void>(done=>setImmediate(done));

/** 🔭️ Reads run eligibility from shallow metadata before opening any run's content tree. */
async function censusRuns(store:string,nowMs:number,control:CargoGcControlV1):Promise<ComponentRunMetadataV1[]>{
 const root=join(store,policy.directory),rows:ComponentRunMetadataV1[]=[],entries=names(root).sort();
 for(const [index,name] of entries.entries()){
  control.signal?.throwIfAborted();
  const path=join(root,name);let directory=false;try{directory=lstatSync(path).isDirectory();}catch{}
  if(directory){
   const lastUsedMs=componentRunRecencyV1(path);
   rows.push({path,lastUsedMs,components:componentRunComponentsV1(path),held:nowMs-lastUsedMs<policy.heldMilliseconds});
  }
  if((index+1)%policy.yieldEvery===0||index+1===entries.length){
   await control.progress?.(index+1,entries.length,"scan",root);
   await turn();
  }
 }
 return rows;
}

/** 📊️ What a collection did or, in a dry run, would do. */
export type ComponentRunResultV1=Readonly<{dryRun:boolean;total:number;removed:readonly ComponentRunRemovalV1[];freedBytes:number;held:number;kept:number}>;

/** 🧹️ Measures superseded runs oldest first. Apply rechecks recency before deletion and uses live free space when supplied. */
export async function collectComponentRunsV1(store:string,options:CargoGcControlV1&Readonly<{dryRun:boolean;needBytes:()=>number;satisfied?:()=>boolean|undefined;nowMs?:number;report?:Readonly<{language:"en"|"de";write:(line:string)=>void}>}>):Promise<ComponentRunResultV1>{
 const nowMs=options.nowMs??Date.now(),report=options.report,say=(key:keyof typeof messages,variables:Readonly<Record<string,string|number>>={}):void=>{if(report)report.write(componentRunTextV1(key,report.language,variables));};
 say("scan",{root:join(store,policy.directory)});
 const runs=await censusRuns(store,nowMs,options),candidates=supersededRuns(runs),need=options.needBytes(),removed:ComponentRunRemovalV1[]=[];
 let freedBytes=0;
 const satisfied=()=>need<=0||((options.dryRun?undefined:options.satisfied?.())??(freedBytes>=need));
 for(const run of candidates){
  options.signal?.throwIfAborted();
  if(satisfied())break;
  const {path}=run,bytes=await allocatedCargoBytesV1(path,UNIT_POLICY_V1.maximumEntriesPerUnit,options);
  if(!options.dryRun){
   if(satisfied())break;
   const recency=componentRunRecencyV1(path);
   if(recency!==run.lastUsedMs||nowMs-recency<policy.heldMilliseconds){say("changed",{path});continue;}
   await removeCargoGcTreeV1(path,options);
  }
  removed.push({path,bytes});freedBytes+=bytes;
  if(removed.length<=policy.reportedRemovals)say(options.dryRun?"would":"removed",{path,bytes:size(bytes)});
  if(!options.dryRun)await options.progress?.(removed.length,candidates.length,"remove",path);
 }
 const held=runs.filter(run=>run.held).length;
 say(options.dryRun?"summaryPlan":"summaryApply",{count:removed.length,total:runs.length,bytes:size(freedBytes),held,kept:runs.length-removed.length-held});
 return Object.freeze({dryRun:options.dryRun,total:runs.length,removed:Object.freeze(removed),freedBytes,held,kept:runs.length-removed.length-held});
}
