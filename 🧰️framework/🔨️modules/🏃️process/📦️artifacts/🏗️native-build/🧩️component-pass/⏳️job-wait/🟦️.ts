import{churnProtocolV1}from"../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** ⏳️ What one job child is waiting for: the resource or path its last wait notice named. */
export type NativeBuildJobWaitV1=Readonly<{subject:string;sinceMs:number}>;
/** 🔭️ Tells real job progress from waiting: a wait notice (a churn retry or a lease activity that already waited) never counts as progress, so a job parked behind a lease or a retrying snapshot is seen as stalled once its notices are all the output it has. */
export type NativeBuildJobWatchV1=Readonly<{observe(line:string,nowMs:number):"progress"|"wait";waiting(nowMs:number):NativeBuildJobWaitV1|undefined;progressedAtMs():number}>;

const escape=(text:string):string=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
function retryPatternsV1():readonly RegExp[]{return Object.values(churnProtocolV1().texts).map(row=>new RegExp("^"+row.retry.split(/(\{[a-z]+\})/u).map(part=>part==="{path}"?"(?<path>.+?)":/^\{[a-z]+\}$/u.test(part)?".+?":escape(part)).join("")+"$","u"));}
const activityPrefix="[script] ";

/** 🎟️ Classifies one child output line: `{subject}` when it only announces a wait (a churn retry naming its path, or a lease activity with elapsed time naming its resource), `undefined` for real progress.
 * @see ../../../../../📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json */
export function nativeBuildJobWaitSubjectV1(line:string):string|undefined{
  for(const pattern of retryPatternsV1()){const path=pattern.exec(line)?.groups?.path;if(path!==undefined)return path;}
  if(line.startsWith(activityPrefix)){let value:unknown;try{value=JSON.parse(line.slice(activityPrefix.length));}catch{return undefined;}const activity=(value as{activity?:{resource?:unknown;elapsedMs?:unknown}}|null)?.activity;if(activity&&typeof activity.resource==="string"&&typeof activity.elapsedMs==="number"&&activity.elapsedMs>0)return activity.resource;}
  return undefined;
}

/** 👁️ Starts watching one job child from `startMs`. */
export function watchNativeBuildJobV1(startMs:number):NativeBuildJobWatchV1{
  let progress=startMs,wait:NativeBuildJobWaitV1|undefined;
  return Object.freeze({
    observe(line:string,nowMs:number){const subject=nativeBuildJobWaitSubjectV1(line);if(subject===undefined){progress=nowMs;wait=undefined;return"progress";}if(wait?.subject!==subject)wait=Object.freeze({subject,sinceMs:nowMs});return"wait";},
    waiting(nowMs:number){return wait&&nowMs>=wait.sinceMs?wait:undefined;},
    progressedAtMs:()=>progress
  });
}

/** 🛑️ The refusal naming the job and the resource it waited on when it made no progress for `stallMs`, `undefined` while it progresses or waits shorter. */
export function nativeBuildJobStallV1(job:string,watch:NativeBuildJobWatchV1,nowMs:number,stallMs:number):string|undefined{
  const idle=nowMs-watch.progressedAtMs(),wait=watch.waiting(nowMs);
  return wait&&idle>=stallMs?`Native build stalled: job:${job} made no progress for ${Math.round(idle/1000)} s and has only waited on ${wait.subject} since ${Math.round((nowMs-wait.sinceMs)/1000)} s`:undefined;
}


/** ⏲️ Detects a compiler idle timeout from the active driver observations. */
export function nativeBuildCompilerIdleStallV1(observation: Readonly<{
  children: number; planning: number; runningUnits: number; runningStages: number; idleMilliseconds: number; stallMilliseconds: number;
}>): boolean {
  return observation.children > observation.planning && observation.runningUnits === 0 && observation.runningStages === 0
    && observation.idleMilliseconds >= observation.stallMilliseconds;
}
