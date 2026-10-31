import{linuxProcessDescendantsV1,linuxProcessCpuMillisecondsV1}from"./🐧️linux/🟦️.ts";
import{spawnSync}from"node:child_process";
import{setTimeout as pause}from"node:timers/promises";
import schema from"./🧬️schema/🔣️.json" with{type:"json"};
import budgetSchema from"../../../../⏱️budget/🧬️schema/🔣️.json" with{type:"json"};
const policy=Object.freeze({...schema.definitions.Policy.const}),notices=schema.definitions.Notice.const;
/** 🐕️ What the stall rule depends on. */
export type ScriptProcessStallStateV1=Readonly<{idleMilliseconds:number;liveChildren:number;treeIdleMilliseconds:number;waiting:number}>;
/** 📜️ The single pure rule: stalled once nothing progressed for the stall period and no child process is alive; tree-stalled once, with children alive, the whole descendant tree additionally used no CPU time for the idle-tree period (a build tool parked on a lock it will never get). @see ./🧬️schema/🔣️.json */
export function scriptProcessStallDecisionV1(state:ScriptProcessStallStateV1,stallMilliseconds:number=policy.stallMilliseconds,idleTreeMilliseconds:number=policy.idleTreeMilliseconds):(typeof schema.definitions.Decision.enum)[number]{
 if(state.idleMilliseconds<stallMilliseconds||state.waiting>0)return"healthy";
 if(state.liveChildren===0)return"stalled";
 return state.treeIdleMilliseconds>=idleTreeMilliseconds?"tree-stalled":"healthy";
}
let lastProgress=Date.now(),lastActivity=lastProgress,activityDemanded=false,outputObserved=false,ticket=0,episodeStart=0,lastWaitEnd=0;const waits=new Map<number,number>();
const naturalV1=(name:string):number|undefined=>{const value=Number(process.env[name]);return Number.isSafeInteger(value)&&value>=1000?value:undefined;};
/** ⏸️ Waits that are younger than the wait ceiling: a queue wait counts as waiting for that long, a wait that outlives the ceiling (a queue nobody serves any more) is stalled like anything else. */
export function activeScriptProcessWaitsV1(now:number=Date.now(),ceilingMilliseconds:number=naturalV1(policy.waitCeilingEnvironment)??policy.waitCeilingMilliseconds):number{let count=0;for(const since of waits.values())if(now-since<ceilingMilliseconds)count++;return count;}
/** 📝️ Records that this process made progress (output, published progress, settled accepted work). */
export function noteScriptProcessProgressV1():void{lastProgress=Date.now();}
/** ⌛️ Declares that this process waits in a lease, claim or capacity queue (or any other bounded external wait) until the returned function is called: a waiting process is never stalled. */
export function enterScriptProcessWaitV1():()=>void{const id=++ticket,now=Date.now();if(waits.size===0&&now-lastWaitEnd>policy.waitEpisodeGapMilliseconds)episodeStart=now;waits.set(id,now);lastProgress=now;lastActivity=now;return()=>{if(waits.delete(id)){lastProgress=Date.now();lastActivity=lastProgress;lastWaitEnd=lastProgress;}};}
/** ⚙️ How long this process has been in one continuous wait episode, i.e. inside declared waits that follow each other with gaps shorter than `waitEpisodeGapMilliseconds` (the polls of one queue); 0 when it is not waiting. A process that only waits can lend its capacity grant back. */
export function scriptProcessWaitEpisodeV1(now:number=Date.now()):number{return waits.size===0&&now-lastWaitEnd>policy.waitEpisodeGapMilliseconds?0:Math.max(1,now-episodeStart);}
/** 🌡️ Records that real work happened: child CPU time, child output, a finished observation step. Unlike progress this is never a heartbeat the process writes about itself, so an idle window it renews cannot be kept alive by a hung action. */
export function noteScriptProcessActivityV1(at:number=Date.now()):void{if(at>lastActivity)lastActivity=at;}
/** 💤️ The milliseconds since this process last did observable work; 0 while it sits in a declared wait. */
export function scriptProcessActivityIdleMillisecondsV1(now:number=Date.now()):number{return activeScriptProcessWaitsV1(now)>0?0:Math.max(0,now-lastActivity);}
/** 📣️ Declares that a deadline renewed by activity exists, so the sampler starts reading child CPU time. */
export function demandScriptProcessActivityV1():void{activityDemanded=true;}
/** 🧫️ One CPU sample of the live descendants: any change of their summed CPU time is work; a table or time that cannot be read counts as work (an unobservable child is never judged idle). Returns whether work was seen. */
export function createScriptProcessActivitySamplerV1(options:Readonly<{children?:()=>readonly number[]|undefined;cpu?:(pids:readonly number[])=>number|undefined;note?:()=>void;trace?:(children:number|undefined,cpuMilliseconds:number|undefined)=>void}>={}):()=>boolean{
 const children=options.children??liveChildPidsV1,cpu=options.cpu??treeCpuMillisecondsV1,note=options.note??noteScriptProcessActivityV1;let previous:number|undefined;
 return()=>{const pids=children();if(pids!==undefined&&pids.length===0){previous=undefined;return false;}const used=pids===undefined?undefined:cpu(pids);if(used===undefined||previous===undefined||used!==previous){previous=used;note();try{options.trace?.(pids?.length,used);}catch{}return true;}return false;};
}
/** 🔰️ Samples child CPU time at the declared cadence while a renewable deadline exists. */
export function startScriptProcessActivitySamplerV1(options:Readonly<{sample?:()=>boolean;sampleMilliseconds?:number;demanded?:()=>boolean}>={}):()=>void{
 const sample=options.sample??createScriptProcessActivitySamplerV1(),demanded=options.demanded??(()=>activityDemanded),timer=setInterval(()=>{if(demanded())sample();},options.sampleMilliseconds??budgetSchema.$defs.ProgressDeadlinePolicy.const.sampleMilliseconds);
 timer.unref();return()=>clearInterval(timer);
}
/** ⏰️ A timer wait that counts as waiting for the whole stall rule; the drop-in for `timers/promises` `setTimeout` in every queue and capacity poll. */
export async function waitingDelayV1<T>(milliseconds:number,value?:T,options?:Readonly<{signal?:AbortSignal;ref?:boolean}>):Promise<T|undefined>{const leave=enterScriptProcessWaitV1();try{return await pause(milliseconds,value,options as never);}finally{leave();}}
/** 📋️ Lists live descendants from bounded kernel metadata on Linux and a system process snapshot on macOS/Windows; an unreadable table answers `undefined` (callers then assume children are alive). */
export function liveChildPidsV1(pid:number=process.pid):readonly number[]|undefined{
 if(process.platform==="linux")return linuxProcessDescendantsV1(pid);
 try{
  const windows=process.platform==="win32",snapshot=windows?spawnSync("powershell.exe",["-NoLogo","-NoProfile","-NonInteractive","-Command","Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId) $($_.ParentProcessId)\" }"],{encoding:"utf8",windowsHide:true,timeout:10000}):spawnSync("ps",["-axo","pid=,ppid="],{encoding:"utf8",windowsHide:true,timeout:10000});
  if(snapshot.status!==0||typeof snapshot.stdout!=="string")return undefined;
  const parents=new Map<number,number>();for(const line of snapshot.stdout.split("\n")){const fields=line.trim().split(/\s+/u).map(Number);if(fields.length===2&&fields.every(Number.isSafeInteger))parents.set(fields[0]!,fields[1]!);}
  const found:number[]=[];for(const[child]of parents){if(child===process.pid||child===snapshot.pid)continue;for(let at=child,steps=0;steps<64;steps++){const parent=parents.get(at);if(parent===undefined)break;if(parent===pid){found.push(child);break;}at=parent;}}
  return found;
 }catch{return undefined;}
}
/** 🔢️ Counts live descendants; an unreadable table counts as one live child (fail safe). */
export function liveChildCountV1(pid:number=process.pid):number{return liveChildPidsV1(pid)?.length??1;}
/** 🌳️ Sums the CPU time (milliseconds) the given processes have used so far; `undefined` when it cannot be read (Windows, vanished pids). */
export function treeCpuMillisecondsV1(pids:readonly number[]):number|undefined{
 if(process.platform==="linux")return linuxProcessCpuMillisecondsV1(pids);
 if(process.platform==="win32"||!pids.length)return undefined;
 try{const snapshot=spawnSync("ps",["-o","cputime=","-p",pids.join(",")],{encoding:"utf8",timeout:10000});if(typeof snapshot.stdout!=="string")return undefined;let total=0;for(const line of snapshot.stdout.split("\n")){const text=line.trim();if(!text)continue;const match=/^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)$/u.exec(text);if(!match)return undefined;total+=((Number(match[1]??0)*24+Number(match[2]??0))*60+Number(match[3]))*60000+Number(match[4])*1000;}return Math.round(total);}catch{return undefined;}
}
/** 🖨️ Treats every byte written to stdout or stderr as progress, once per process; Bun's `console` writes to the file descriptors without passing `process.stdout.write`, so its methods are observed as well. */
export function observeScriptProcessOutputV1():void{
 if(outputObserved)return;outputObserved=true;
 for(const name of["log","info","warn","error","debug","trace"] as const){const write=console[name].bind(console) as(...args:unknown[])=>void;console[name]=(...args:unknown[])=>{lastProgress=Date.now();write(...args);};}
 for(const stream of[process.stdout,process.stderr]){const write=stream.write.bind(stream) as(...args:unknown[])=>boolean;(stream as unknown as{write:(...args:unknown[])=>boolean}).write=(...args:unknown[])=>{lastProgress=Date.now();return write(...args);};}
}
/** 🏭️ Renders one notice in every selected language. */
export function scriptProcessWatchdogNoticeV1(kind:keyof typeof notices,values:Readonly<Record<string,string|number>>,languages:readonly("en"|"de")[]):string{
 return languages.map(language=>Object.entries(values).reduce((text,[key,value])=>text.replaceAll("{"+key+"}",String(value)),notices[kind][language])).join("\n")+"\n";
}
/** ▶️ Starts the stall watchdog; `onStall` runs once with the idle time, the decision and the live child pids. */
export function startScriptProcessWatchdogV1(options:Readonly<{onStall:(idleMilliseconds:number,decision:"stalled"|"tree-stalled",children:readonly number[],treeIdleMilliseconds:number)=>void;now?:()=>number;children?:()=>readonly number[]|undefined;cpu?:(pids:readonly number[])=>number|undefined;stallMilliseconds?:number;idleTreeMilliseconds?:number;checkMilliseconds?:number}>):()=>void{
 const now=options.now??Date.now,children=options.children??liveChildPidsV1,cpu=options.cpu??treeCpuMillisecondsV1,natural=naturalV1,stall=options.stallMilliseconds??natural(policy.stallEnvironment)??policy.stallMilliseconds,idleTree=options.idleTreeMilliseconds??natural(policy.idleTreeEnvironment)??policy.idleTreeMilliseconds;let observed=now(),treeCpu:number|undefined,treeSince=observed;lastProgress=observed;
 const timer=setInterval(()=>{
  const clock=now(),idle=clock-Math.max(lastProgress,observed);
  const waiting=activeScriptProcessWaitsV1(clock);if(waiting>0){observed=clock;return;}
  if(idle<stall)return;
  const pids=children();
  if(pids!==undefined&&pids.length===0){clearInterval(timer);options.onStall(idle,"stalled",pids,0);return;}
  const used=pids===undefined?undefined:cpu(pids);
  if(used===undefined||treeCpu===undefined||used!==treeCpu){treeCpu=used;treeSince=clock;return;}
  const treeIdle=clock-treeSince;
  if(scriptProcessStallDecisionV1({idleMilliseconds:Math.round(idle),liveChildren:pids?.length??1,treeIdleMilliseconds:Math.round(treeIdle),waiting},stall,idleTree)==="tree-stalled"){clearInterval(timer);options.onStall(idle,"tree-stalled",pids??[],treeIdle);}
 },options.checkMilliseconds??policy.checkMilliseconds);
 timer.unref();return()=>clearInterval(timer);
}
/** 🆕️ Guarantees that a termination signal ends the process: the second signal, or the grace after the first, exits at once after a notice. */
export function createScriptTerminationGuardV1(options:Readonly<{exit:(code:number)=>void;notice:(text:string)=>void;languages:()=>readonly("en"|"de")[];graceMilliseconds?:number}>):(signal:string,code:number)=>void{
 const grace=options.graceMilliseconds??policy.terminationGraceMilliseconds;let armed=false;
 return(signal,code)=>{
  const end=():void=>{try{options.notice(scriptProcessWatchdogNoticeV1("terminating",{signal,seconds:Math.round(grace/1000)},options.languages()));}catch{}options.exit(code);};
  if(armed){end();return;}
  armed=true;try{options.notice(scriptProcessWatchdogNoticeV1("interrupted",{signal,seconds:Math.round(grace/1000)},options.languages()));}catch{}const timer=setTimeout(end,grace);timer.unref();
 };
}

let admissionWaitDepth=0,admissionWaitStarted=0,admissionWaitCompleted=0;
/** ⏳️ Opens one wait for machine admission and returns the call that closes it: overlapping waits count once. The time spent here is not part of any deadline, because a deadline starts when the work is admitted. */
export function beginAdmissionWaitV1():()=>void{
 if(admissionWaitDepth++===0)admissionWaitStarted=performance.now();
 let ended=false;
 return()=>{if(ended)return;ended=true;if(--admissionWaitDepth===0)admissionWaitCompleted+=performance.now()-admissionWaitStarted;};
}
/** 🔏️ How many milliseconds this process has spent waiting for machine admission so far, an open wait included. */
export function admissionWaitedMillisecondsV1():number{return admissionWaitCompleted+(admissionWaitDepth>0?performance.now()-admissionWaitStarted:0);}
/** 🧮️ The milliseconds left of a deadline whose clock stopped while the process waited for admission: `remaining` is what the wall clock says, `waitedSinceStart` what the ledger gained since the deadline began. */
export function creditedDeadlineRemainingV1(remainingMilliseconds:number,waitedSinceStartMilliseconds:number):number{return remainingMilliseconds+Math.max(0,waitedSinceStartMilliseconds);}

