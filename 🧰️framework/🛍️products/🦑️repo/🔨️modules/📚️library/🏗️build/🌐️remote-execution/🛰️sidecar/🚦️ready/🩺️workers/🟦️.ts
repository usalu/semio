import {readFileSync,readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {readyWorkerLivenessV1,type ReadyLivenessV1} from "../💔️silence/🟦️.ts";

const P=schema.definitions.Policy.const;

/** 🩺️ How long a machine may show no sign of life before the fleet reports it offline, in seconds. */
export const READY_OFFLINE_SECONDS_V1=P.offlineMilliseconds/1000;

/** 🩺️ One worker as a reader on another machine sees it: who it is, what its record claims and what its liveness really is. */
export type ReadyWorkerRowV1=Readonly<{origin:string;platform:string|null;liveness:ReadyLivenessV1["liveness"];recorded:string|null;stage:string|null;stageIndex:number;stageCount:number;attempts:number;recordAgeSeconds:number|null;heartbeatAgeSeconds:number|null;refusal:Readonly<{code:string;en:string;de:string}>|null;recent:ReadyRecentRunsV1|null}>;

/** 📜️ How the runs of the stage a worker is in ended during the last `hours` hours, from the `ready-timings.json` the worker publishes: `ok`, `failed` (the stage failed or could not start), `stalled` (stopped because nothing new showed for the stall window), `timedOut`, and the `seconds` they ran in all. A worker that sits at attempt 17 shows here whether those were failures or stalls. */
export type ReadyRecentRunsV1=Readonly<{hours:number;ok:number;failed:number;stalled:number;timedOut:number;seconds:number}>;

/** 📜️ The window of the recent runs of a stage, in hours. */
export const READY_RECENT_HOURS_V1=12;
const timingsBytesV1=2_097_152;

/** 📜️ Counts how the runs of `stage` that ended within `hours` hours before `nowMs` ended; `null` when the stage is unknown, the file is missing, oversized or not of the shape, or no run of the stage ended in the window. A pure function of the parsed file.
 * @see ./🧪️tests/🟦️.ts */
export function readyRecentRunsV1(timings:unknown,stage:string|null,nowMs:number,hours:number=READY_RECENT_HOURS_V1):ReadyRecentRunsV1|null{
 if(stage===null||timings===null||typeof timings!=="object")return null;
 const runs=(timings as {runs?:unknown}).runs;
 if(!Array.isArray(runs))return null;
 const since=nowMs-hours*3_600_000,counts={ok:0,failed:0,stalled:0,timedOut:0,seconds:0};let seen=0;
 for(const run of runs as Record<string,unknown>[]){
  if(run===null||typeof run!=="object"||run.stage!==stage||!Number.isFinite(run.endedMs)||(run.endedMs as number)<since)continue;
  seen++;counts.seconds+=Number.isFinite(run.seconds)?Math.max(0,run.seconds as number):0;
  if(run.outcome==="ok")counts.ok++;else if(run.outcome==="stalled")counts.stalled++;else if(run.outcome==="stage-timeout")counts.timedOut++;else counts.failed++;
 }
 return seen===0?null:Object.freeze({hours,...counts,seconds:Math.round(counts.seconds)});
}

/** 🩺️ Reads every origin of the fleet store (`<store>/v1/origins/<origin>`): its `ready.json` and the newest modification time of the heartbeat files its replica writes every few seconds, and judges each with {@link readyWorkerLivenessV1}, so a machine that went away is `offline` however `running` its last record says it was. An unreadable store gives no rows; an origin without a record is `absent`.
 * @see ./🧪️tests/🟦️.ts */
export function readReadyWorkersV1(input:Readonly<{store:string;nowMs:number;policy?:typeof P}>):readonly ReadyWorkerRowV1[]{
 const policy=input.policy??P,base=join(input.store,"v1","origins");
 let names:string[]=[];
 try{names=readdirSync(base,{withFileTypes:true}).filter(row=>row.isDirectory()&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(row.name)).map(row=>row.name).sort();}catch{return[];}
 const recentOf=(directory:string,stage:string|null,nowMs:number):ReadyRecentRunsV1|null=>{try{const file=join(directory,"ready-timings.json");return statSync(file).size>timingsBytesV1?null:readyRecentRunsV1(JSON.parse(readFileSync(file,"utf8")),stage,nowMs);}catch{return null;}};
 return names.map(origin=>{
  const directory=join(base,origin);
  let status:unknown=null,heartbeat:number|null=null;
  try{status=JSON.parse(readFileSync(join(directory,policy.statusName),"utf8"));}catch{}
  for(const name of policy.heartbeatNames)try{heartbeat=Math.max(heartbeat??0,Math.floor(statSync(join(directory,name)).mtimeMs));}catch{}
  const verdict=readyWorkerLivenessV1({status,heartbeatMs:heartbeat,nowMs:input.nowMs,policy}),row=(status!==null&&typeof status==="object"?status:{}) as Record<string,unknown>;
  const text=(key:string):string|null=>typeof row[key]==="string"?row[key] as string:null,count=(key:string):number=>Number.isSafeInteger(row[key])&&(row[key] as number)>=0?row[key] as number:0,refusal=row.refusal as {code?:unknown;en?:unknown;de?:unknown}|null|undefined;
  return Object.freeze({origin,platform:text("platform"),liveness:verdict.liveness,recorded:verdict.liveness==="absent"?null:text("state"),stage:text("stage"),stageIndex:count("stageIndex"),stageCount:count("stageCount"),attempts:count("attempts"),recordAgeSeconds:verdict.ageMs===null?null:Math.floor(verdict.ageMs/1000),heartbeatAgeSeconds:verdict.heartbeatAgeMs===null?null:Math.floor(verdict.heartbeatAgeMs/1000),refusal:refusal!==null&&refusal!==undefined&&typeof refusal.code==="string"&&typeof refusal.en==="string"&&typeof refusal.de==="string"?Object.freeze({code:refusal.code,en:refusal.en,de:refusal.de}):null,recent:recentOf(directory,text("stage"),input.nowMs)});
 });
}
