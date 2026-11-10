import {spawn} from "node:child_process";
import {textV1,type LanguageCatalogV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import catalog from "./📖️texts/🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📏️ The authored constants of the limiter, read from the schema they are written first in. */
export const JOB_LIMIT_RULES_V1=schema.definitions.Rules.const;

/** 🧱️ What one job may hold; swap is always 0 so an overrun fails at the limit instead of thrashing the host. */
export type JobLimitPolicyV1=Readonly<{kind:"job"|"service";memoryMegabytes:number;swapMegabytes:0;cpuShares:number;pids:number;softRatioBasisPoints:number;enforcement:"hard"|"watchdog"}>;
/** 🛡️ How a limit is enforced. */
export type JobEnforcementV1="cgroup-v2"|"job-object"|"watchdog"|"fake";
/** 🎯️ The limit of one learned class: `max(1.25 x learned peak, floor)`, never below the minimum limit. */
export type JobEstimateV1=Readonly<{class:string;floorMegabytes:number;learnedPeakMegabytes?:number;limitMegabytes:number}>;
/** 💥️ The moment a job reached its limit. */
export type JobBreachV1=Readonly<{peakMegabytes:number;limitMegabytes:number}>;
/** 📈️ What a job holds right now and held at most. */
export type JobUsageV1=Readonly<{rssMegabytes:number;peakMegabytes:number}>;
/** 🏁️ What the limiter knows when a job is released. */
export type JobFinalV1=Readonly<{peakMegabytes:number;breached:boolean}>;
/** 🔒️ One limit around one job; created before the job starts, released after it ended. */
export interface LimitHandleV1{readonly enforcement:JobEnforcementV1;attach(pid:number):Promise<void>;onBreach(callback:(breach:JobBreachV1)=>void):void;usage():Promise<JobUsageV1>;release():Promise<JobFinalV1>}
/** 🏭️ The port of a platform mechanism: cgroup v2, job object, watchdog or a fake. */
export interface JobLimiterV1{readonly enforcement:JobEnforcementV1;create(policy:JobLimitPolicyV1,job:string):Promise<LimitHandleV1>}
/** 🚦️ How a limited job ended; `memory-exceeded` is the typed failure that replaces a SIGKILL of the kernel out-of-memory killer. */
export type JobOutcomeKindV1="ok"|"failed"|"memory-exceeded"|"cancelled";
/** 🧾️ The result of one limited job; the peak is recorded for every kind, killed jobs included. */
export type JobOutcomeV1=Readonly<{kind:JobOutcomeKindV1;enforcement:JobEnforcementV1;limitMegabytes:number;peakMegabytes:number;exitCode:number|null;signal:string|null;durationMilliseconds:number}>;

const round=(value:number):number=>Math.floor(value+0.5);

/** 🧮️ The limit of a class: 1.25 x the learned peak, at least its floor and the minimum limit; a class that learned nothing is limited by its floor. */
export function jobEstimateV1(className:string,floorMegabytes:number,learnedPeakMegabytes?:number):JobEstimateV1{
 const grown=learnedPeakMegabytes===undefined?0:round(learnedPeakMegabytes*JOB_LIMIT_RULES_V1.headroomBasisPoints/10000);
 return Object.freeze({class:className,floorMegabytes,...learnedPeakMegabytes===undefined?{}:{learnedPeakMegabytes},limitMegabytes:Math.max(grown,floorMegabytes,JOB_LIMIT_RULES_V1.minimumLimitMegabytes)});
}

/** 🧱️ The policy for a limit: hard where the platform enforces it, with the authored soft ratio (throttling only for services), no swap and one CPU share per started job. */
export function jobPolicyV1(limitMegabytes:number,options:Readonly<{kind?:"job"|"service";cpuShares?:number;pids?:number;enforcement?:"hard"|"watchdog"}>={}):JobLimitPolicyV1{
 return Object.freeze({kind:options.kind??"job",memoryMegabytes:Math.max(1,Math.floor(limitMegabytes)),swapMegabytes:0,cpuShares:options.cpuShares??100,pids:options.pids??0,softRatioBasisPoints:JOB_LIMIT_RULES_V1.softRatioBasisPoints,enforcement:options.enforcement??"hard"});
}

/** 🎫️ Admission and limit are one number: the memory a grant reserves is the memory its jobs may hold, so a job that overruns its reservation fails alone. */
export function jobPolicyForReservationV1(reservedMegabytes:number,jobs:number,options:Readonly<{kind?:"job"|"service";cpuShares?:number;pids?:number}>={}):JobLimitPolicyV1{
 return jobPolicyV1(Math.max(JOB_LIMIT_RULES_V1.minimumLimitMegabytes,Math.floor(reservedMegabytes/Math.max(1,jobs))),options);
}

/** 📈️ The peak a killed job teaches the estimate: it needed more than its limit, so the learned peak grows by the breach growth instead of staying at the limit it was stopped at. */
export function jobBreachLearnedPeakV1(outcome:Pick<JobOutcomeV1,"kind"|"limitMegabytes"|"peakMegabytes">):number{
 return outcome.kind==="memory-exceeded"?Math.max(outcome.peakMegabytes,round(outcome.limitMegabytes*JOB_LIMIT_RULES_V1.breachGrowthBasisPoints/10000)):outcome.peakMegabytes;
}

/** 🗣️ Renders the typed failure in one named language; no language is a default. */
export function renderJobOutcomeV1(outcome:JobOutcomeV1,job:string,language:"en"|"de"):string|undefined{
 return outcome.kind==="memory-exceeded"?textV1(catalog as LanguageCatalogV1,"memory-exceeded",language,{job,limit:outcome.limitMegabytes,peak:outcome.peakMegabytes,enforcement:outcome.enforcement}):undefined;
}

/** 🗣️ Renders the notice that a hard limit fell back to the watchdog. */
export function renderEnforcementFallbackV1(job:string,limitMegabytes:number,language:"en"|"de"):string{
 return textV1(catalog as LanguageCatalogV1,"enforcement-fallback",language,{job,limit:limitMegabytes});
}

/** 🏃️ What to run under a limit. */
export type LimitedJobV1=Readonly<{job:string;policy:JobLimitPolicyV1;limiter:JobLimiterV1;command:string;args:readonly string[];cwd?:string;environment?:Readonly<Record<string,string|undefined>>;stdio?:"inherit"|"ignore"|"pipe";signal?:AbortSignal;progress?:(usage:JobUsageV1&Readonly<{limitMegabytes:number}>)=>void;progressMilliseconds?:number;now?:()=>number;killTree?:(pid:number)=>void}>;

/** 🔪️ Kills a process tree: the process group on POSIX, the whole tree through `taskkill` on Windows. */
export function killJobTreeV1(pid:number):void{
 try{
  if(process.platform==="win32")spawn("taskkill",["/pid",String(pid),"/t","/f"],{stdio:"ignore",windowsHide:true}).on("error",()=>undefined);
  else process.kill(-pid,"SIGKILL");
 }catch{try{process.kill(pid,"SIGKILL");}catch{}}
}

/** 🏃️ Runs one job under a limit; asynchronous, cancellable through the signal, with progress; an overrun ends as a `memory-exceeded` outcome of this job alone and never throws, whatever the kernel did to the process. */
export async function runLimitedJobV1(job:LimitedJobV1):Promise<JobOutcomeV1>{
 const now=job.now??Date.now,kill=job.killTree??killJobTreeV1,started=now(),handle=await job.limiter.create(job.policy,job.job);
 let breach:JobBreachV1|undefined,cancelled=false,timer:ReturnType<typeof setInterval>|undefined,pid:number|undefined;
 const child=spawn(job.command,[...job.args],{cwd:job.cwd,env:job.environment as NodeJS.ProcessEnv|undefined,stdio:job.stdio??"inherit",detached:process.platform!=="win32",windowsHide:true});
 const ended=new Promise<Readonly<{code:number|null;signal:NodeJS.Signals|null}>>(done=>{child.once("error",()=>done({code:127,signal:null}));child.once("close",(code,signal)=>done({code,signal}));});
 pid=child.pid;
 handle.onBreach(found=>{if(breach===undefined){breach=found;if(pid!==undefined)kill(pid);}});
 const abort=()=>{cancelled=true;if(pid!==undefined)kill(pid);};
 if(job.signal?.aborted)abort();else job.signal?.addEventListener("abort",abort,{once:true});
 if(pid!==undefined)await handle.attach(pid).catch(()=>undefined);
 if(job.progress)timer=setInterval(()=>{handle.usage().then(usage=>job.progress!({...usage,limitMegabytes:job.policy.memoryMegabytes}),()=>undefined);},job.progressMilliseconds??250);
 const exit=await ended;
 if(timer!==undefined)clearInterval(timer);
 job.signal?.removeEventListener("abort",abort);
 if(pid!==undefined&&process.platform!=="win32")try{process.kill(-pid,"SIGKILL");}catch{}
 const final=await handle.release(),peak=Math.max(final.peakMegabytes,breach?.peakMegabytes??0),exceeded=breach!==undefined||final.breached;
 const kind:JobOutcomeKindV1=exceeded?"memory-exceeded":cancelled?"cancelled":exit.code===0?"ok":"failed";
 return Object.freeze({kind,enforcement:handle.enforcement,limitMegabytes:job.policy.memoryMegabytes,peakMegabytes:peak,exitCode:exit.code,signal:exit.signal,durationMilliseconds:now()-started});
}

/** 🎭️ A limiter that never enforces anything: it reports a scripted peak and breach, so every consumer is tested without a kernel; a breach is raised when the scripted peak reaches the limit. */
export function createFakeJobLimiterV1(script:Readonly<{peakMegabytes:number}>):JobLimiterV1{
 return Object.freeze({enforcement:"fake" as const,async create(policy){
  const callbacks:((breach:JobBreachV1)=>void)[]=[],over=script.peakMegabytes>=policy.memoryMegabytes;
  return Object.freeze({enforcement:"fake" as const,async attach(){if(over)queueMicrotask(()=>callbacks.forEach(callback=>callback({peakMegabytes:script.peakMegabytes,limitMegabytes:policy.memoryMegabytes})));},onBreach(callback){callbacks.push(callback);},async usage(){return{rssMegabytes:script.peakMegabytes,peakMegabytes:script.peakMegabytes};},async release(){return{peakMegabytes:script.peakMegabytes,breached:over};}});
 }});
}
