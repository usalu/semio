import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {scaledMillisecondsV1} from "../../../../🐢️speed/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The numbers of the process-tree rules.
 * @see ./🧬️schema/🔣️.json */
export const READY_TREE_POLICY_V1=schema.definitions.Policy.const;
const P=READY_TREE_POLICY_V1;

/** 🧾️ One row of the process table: the pid, the pid of its parent, the start time in milliseconds since the epoch and, where the table says it, the name of the program. */
export type ReadyProcessV1=Readonly<{pid:number;ppid:number;startMs:number;name?:string;windows?:Readonly<{birth:string;cpu?:Readonly<{kernel:number;user:number}>;workingSetBytes?:number}>}>;
/** 🪪️ A process named by pid and start time (and, once seen in the table, by the name of its program, so a report can name it): a reused pid has another start time and is another process. */
export type ReadyKnownV1=Readonly<{pid:number;startMs:number;name?:string}>;

const near=(left:number,right:number,tolerance:number)=>Math.abs(left-right)<=tolerance;

/** 🧭️ Whether the process `known` named is still in the table as the same process, gone, or its pid belongs to another process now. A start time of 0 in the record means the start was not known and only the pid counts. */
export function readyIdentityV1(known:ReadyKnownV1,table:readonly ReadyProcessV1[],tolerance:number=P.toleranceMilliseconds):"same"|"gone"|"reused"{
 const row=table.find(each=>each.pid===known.pid);
 if(row===undefined)return"gone";
 return known.startMs===0||near(row.startMs,known.startMs,tolerance)?"same":"reused";
}

/** 🪪️ Confirms a later birth for the same PID independently in both process observations. */
export function readyConfirmProcessReplacementV1(known:ReadyKnownV1,first:ReadyProcessV1,second:readonly ReadyProcessV1[],tolerance:number=P.toleranceMilliseconds):ReadyProcessV1|undefined{
 if(!Number.isFinite(tolerance)||tolerance<0||known.startMs<=0||first.pid!==known.pid||!Number.isFinite(first.startMs)||!(first.startMs-known.startMs>tolerance))return undefined;
 return second.find(row=>row.pid===known.pid&&row.startMs===first.startMs&&row.windows?.birth===first.windows?.birth);
}

/** 🏔️ Whether a worker is busy with its readiness: its `ready.json` says not ready in a state of work (running, waiting, backing off) and was written within `foregroundFreshMilliseconds`, so a loop that died never holds a graph server back for ever. The graph server of the machine neither serves nor refreshes meanwhile: readiness is the foreground, a whole-workspace read is not. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyForegroundBusyV1(status:unknown,nowMs:number,policy:typeof P=P):boolean{
 if(status===null||typeof status!=="object")return false;
 const row=status as{ready?:unknown;state?:unknown;updatedMs?:unknown};
 return row.ready===false&&(row.state==="running"||row.state==="waiting"||row.state==="backoff")&&typeof row.updatedMs==="number"&&Number.isFinite(row.updatedMs)&&nowMs-row.updatedMs<=policy.foregroundFreshMilliseconds;
}

/** 🌲️ The processes to stop, leaf first (`spare` and everything below it never, nor a shared system process of `sharedSystemProcesses` such as `diskimages-helper`, which serves the mounted disk images of every stage and every other agent of the machine: ending it with the first stage that attached an image dropped the writes in flight to the partitions of the worker, nor the launchers between a spared service and the root, which own it: a service that other agents use stays up): the root (if it is still the same process), everything below it in the table, and every process recorded in an earlier sample that is still there with its recorded start time (the orphans whose parent has already exited) with everything below those. A child counts as below its parent only when it did not start before the parent (a reused parent pid is not a parent). Never this process or its ancestors, never the system pids. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyKillPlanV1(input:Readonly<{table:readonly ReadyProcessV1[];root:ReadyKnownV1;known:readonly ReadyKnownV1[];self:number;tolerance?:number;spare?:ReadonlySet<number>}>):readonly number[]{
 const tolerance=input.tolerance??P.toleranceMilliseconds,byPid=new Map(input.table.map(row=>[row.pid,row])),children=new Map<number,ReadyProcessV1[]>(),parentOf=new Map<number,number>();
 for(const row of input.table){const parent=byPid.get(row.ppid);if(parent!==undefined&&row.startMs>=parent.startMs-tolerance&&row.pid!==row.ppid){const list=children.get(row.ppid)??[];list.push(row);children.set(row.ppid,list);parentOf.set(row.pid,row.ppid);}}
 const protectedPids=new Set<number>();
 for(let at:number|undefined=input.self,depth=0;at!==undefined&&depth<256&&!protectedPids.has(at);at=byPid.get(at)?.ppid,depth++)protectedPids.add(at);
 const depthOf=new Map<number,number>(),queue:{pid:number;depth:number}[]=[];
 const seeds=[input.root,...input.known].filter(known=>readyIdentityV1(known,input.table,tolerance)==="same");
 for(const seed of seeds)if(!depthOf.has(seed.pid)){depthOf.set(seed.pid,0);queue.push({pid:seed.pid,depth:0});}
 while(queue.length>0){
  const {pid,depth}=queue.shift()!;
  for(const child of children.get(pid)??[])if(!depthOf.has(child.pid)){depthOf.set(child.pid,depth+1);queue.push({pid:child.pid,depth:depth+1});}
 }
 const spared=new Set<number>();
 for(const pid of input.spare??[])if(depthOf.has(pid))for(const stack=[pid];stack.length>0;){const at=stack.pop()!;if(!spared.has(at)){spared.add(at);for(const child of children.get(at)??[])stack.push(child.pid);}}
 for(const pid of input.spare??[])if(depthOf.has(pid))for(let at=parentOf.get(pid),guard=0;at!==undefined&&at!==input.root.pid&&guard<256;at=parentOf.get(at),guard++)protectedPids.add(at);
 return[...depthOf].filter(([pid])=>pid>P.reservedPids&&!protectedPids.has(pid)&&!spared.has(pid)&&!P.sharedSystemProcesses.includes(byPid.get(pid)?.name??"")).sort((a,b)=>b[1]-a[1]||a[0]-b[0]).map(([pid])=>pid);
}

/** 🌲️ The processes of the tree of `root` now (root included), as `{pid, startMs}` rows for the record of the next sample. */
export function readyTreeMembersV1(table:readonly ReadyProcessV1[],root:ReadyKnownV1,known:readonly ReadyKnownV1[],tolerance:number=P.toleranceMilliseconds):readonly ReadyKnownV1[]{
 const plan=new Set(readyKillPlanV1({table,root,known,self:0,tolerance}));
 return table.filter(row=>plan.has(row.pid)).map(row=>row.name===undefined?{pid:row.pid,startMs:row.startMs}:{pid:row.pid,startMs:row.startMs,name:row.name});
}

/** 🔒️ Whether a lease is held: no lease is free; an old one has expired; a lease whose process is gone or is another process now (the pid was reused) is free at once; anything else is held. `process` is what the caller found out about the recorded pid, `processStartMs` the start time it found. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyLeaseVerdictV1(input:Readonly<{lease:Readonly<{pid:number;startMs?:number|null;wallMs:number}>|undefined|null;nowMs:number;staleMs:number;process:"alive"|"gone"|"unknown";processStartMs?:number|null;tolerance?:number}>):Readonly<{state:"free"|"held";reason:"none"|"expired"|"owner-gone"|"pid-reused"|"held"}>{
 const lease=input.lease;
 if(lease===undefined||lease===null)return{state:"free",reason:"none"};
 if(input.nowMs-lease.wallMs>input.staleMs)return{state:"free",reason:"expired"};
 if(input.process==="gone")return{state:"free",reason:"owner-gone"};
 if(input.process==="alive"&&lease.startMs!==undefined&&lease.startMs!==null&&lease.startMs>0&&input.processStartMs!==undefined&&input.processStartMs!==null&&!near(lease.startMs,input.processStartMs,input.tolerance??P.toleranceMilliseconds))return{state:"free",reason:"pid-reused"};
 return{state:"held",reason:"held"};
}

/** 🐢️ The tree policy for a machine `factor` times slower than the reference: the windows without progress (the stall window, the stall stop and the wait for a holder) are lengthened by it, so a small machine that waits more often is not stopped for waiting; the caps of other kinds stay as authored.
 * @see ./🧪️tests/🟦️.ts */
export function readyScaledTreePolicyV1(factor:number,policy:typeof P=P):typeof P{
 return {...policy,speedFactor:Math.max(1,factor),stallMilliseconds:scaledMillisecondsV1(policy.stallMilliseconds,factor),stallStopMilliseconds:scaledMillisecondsV1(policy.stallStopMilliseconds,factor),holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),holderSilentMilliseconds:scaledMillisecondsV1(policy.holderSilentMilliseconds,factor),holderYieldMilliseconds:scaledMillisecondsV1(policy.holderYieldMilliseconds,factor),staleStageMilliseconds:scaledMillisecondsV1(policy.staleStageMilliseconds,factor)};
}

/** 🔁️ What follows the `consecutive`-th stop in a row of one silent stage: a restart after an exponential back-off (`stallRestartBackoffMilliseconds` doubling up to `stallRestartBackoffMaximumMilliseconds`), and a failed state once more than `stallRestartMaximum` stops followed each other, so a stage that waits for something that never comes ends with its reason instead of being stopped every ten minutes for ever. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readyStallRestartV1(input:Readonly<{consecutive:number;policy?:typeof P}>):Readonly<{action:"restart"|"fail";waitMilliseconds:number}>{
 const policy=input.policy??P,count=Math.max(1,Math.floor(input.consecutive));
 if(count>policy.stallRestartMaximum)return{action:"fail",waitMilliseconds:0};
 return{action:"restart",waitMilliseconds:Math.min(policy.stallRestartBackoffMaximumMilliseconds,policy.stallRestartBackoffMilliseconds*2**(count-1))};
}

/** ⏳️ What to do with a stage past its soft cap: it runs while inside the cap; between the soft and the hard cap (the configured stage multiplier times the soft cap) it goes on, `extended`, as long as a counter of its log changed within the stall limit; otherwise it is stopped, `stalled` when the counters stood still, `timeout` at the hard cap. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyDeadlineV1(input:Readonly<{stage?:string;elapsedMs:number;softMs:number;sinceProgressMs:number;policy?:typeof P}>):"run"|"extended"|"timeout"|"stalled"{
 const policy=input.policy??P;
 if(input.elapsedMs<=input.softMs)return"run";
 const overrides:Readonly<Record<string,number>>=policy.stageHardCapMultipliers;
 const multiplier=input.stage!==undefined&&Object.hasOwn(overrides,input.stage)?overrides[input.stage]!:policy.hardCapMultiplier;
 if(input.elapsedMs>input.softMs*multiplier)return"timeout";
 return input.sinceProgressMs<=policy.stallMilliseconds?"extended":"stalled";
}

/** 🔢️ The progress of a stage as the last line of its log that carries a counter (the matching text, so the same counter is the same key and a counter that moved is another): `null` when no line does. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyProgressKeyV1(lines:readonly string[],patterns:readonly string[]=P.progressPatterns):string|null{
 const expressions=patterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--)for(const expression of expressions){const found=expression.exec(lines[index]!);if(found!==null)return found[0];}
 return null;
}

/** 🧽️ A line of a stage log with its elapsed-seconds counters, clocks and timestamps replaced by `#` (`normalisePatterns` of the schema), a carriage return dropped and a Windows path (drive letter, backslashes or doubled backslashes of JSON, any case) spelled `c:/users/x`, so a line that only counts seconds or only spells a path the Windows way is the same line every time; a counter such as `1957 of 2927` stays. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyNormaliseLineV1(line:string,patterns:readonly string[]=P.normalisePatterns):string{
 let text=line.replace(/\r/gu,"").trim().replace(/\b[A-Za-z]:[\\/][^\s"',;)\]]*/gu,path=>path.replace(/[\\/]+/gu,"/").toLowerCase());
 for(const pattern of patterns)text=text.replace(new RegExp(pattern,"gu"),"#");
 return text;
}

const watchdogDigestV1=(text:string):string=>createHash("sha256").update(text).digest("hex");

/** 🐢️ The stall tracker of one stage run: `observe` takes the last lines of the log and answers whether any of them is new, that is differs after normalisation from every line seen earlier in this run; the run made progress exactly then. A line that only changed its seconds counter is not new, one whose counter moved (`1957 of 2927`) is. At most `stallSeenMaximum` distinct lines are kept; a full memory is forgotten at once and counts as progress. Empty lines are no progress. A pure function of the lines seen so far.
 * @see ./🧪️tests/🐍️.py */
export function readyStallTrackerV1(policy:typeof P=P,prior:readonly string[]=[]):Readonly<{observe(lines:readonly string[]):boolean;snapshot():readonly string[]}>{
 const seen=new Set(prior);
 return Object.freeze({snapshot:()=>[...seen],observe(lines){
  let fresh=false;
  for(const line of lines){
   const normalized=readyNormaliseLineV1(line,policy.normalisePatterns);
   if(normalized==="")continue;
   const key=watchdogDigestV1(normalized);
   if(seen.has(key))continue;
   if(seen.size>=policy.stallSeenMaximum)seen.clear();
   seen.add(key);fresh=true;
  }
  return fresh;
 }});
}

/** 🛑️ Whether a stage whose output showed nothing new for `sinceProgressMs` is stopped to be tried again: at `stallStopMilliseconds` and later. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyStalledV1(sinceProgressMs:number,policy:typeof P=P):boolean{return sinceProgressMs>=policy.stallStopMilliseconds;}

/** 🪜️ A step of the read as its progress line states it: the step in both languages (without the note that it passed its bound), the seconds it has run and the bound it declared. */
export type ReadyStepV1=Readonly<{text:string;seconds:number;bound:number}>;

/** 🪜️ The step the newest step line of `lines` names (`stepLinePatterns`: the progress line of the slice worker whose phase is `<English> / <German>` and whose counter is the seconds the step has run of the seconds it is bounded by), `undefined` when no line is one. The note `beyond the expected N s` is cut from the text, so the same step is the same text before and after its bound. A pure function.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyStepOfV1(lines:readonly string[],policy:typeof P=P):ReadyStepV1|undefined{
 const expressions=policy.stepLinePatterns.map(pattern=>new RegExp(pattern,"u")),over=new RegExp(policy.stepOverBoundPattern,"gu");
 for(let index=lines.length-1;index>=0;index--)for(const expression of expressions){
  const found=expression.exec(lines[index]!.replace(/\r/gu,"").trim());
  if(found?.groups!==undefined)return{text:found.groups.step!.replace(over,""),seconds:Number(found.groups.done),bound:Number(found.groups.bound)};
 }
 return undefined;
}

/** 🪜️ The seconds a step may run before it is overdue: its declared bound, written for the reference machine and lengthened by the speed factor of this machine like every other limit (`scaledMillisecondsV1`), plus the window without progress that already ends a silent stage here (`stallStopMilliseconds`, scaled the same way). So a step is overdue exactly when it has been past what this machine is expected to need for as long as a stage may be silent; both parts follow the speed factor and nothing is a number of its own. A pure function.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyStepOverdueSecondsV1(boundSeconds:number,policy:typeof P=P):number{
 return Math.ceil((scaledMillisecondsV1(boundSeconds*1000,policy.speedFactor)+policy.stallStopMilliseconds)/1000);
}

export type ReadyStepWatchCheckpointV1=Readonly<{movedAt:number|null;identity:string|null;seen:readonly string[]}>;

/** 🪜️ The step watch of one stage run: `observe` takes the last lines of the log and the clock and answers the newest step, whether it is overdue and for how long nothing else moved. Something else moved when a line that is no step line is new after normalisation (a counter such as `taken 8 of 590`, an owner finished, a message) or when the step is another step than before; the seconds counter of the step itself, the processor time of the tree and a repeated heartbeat are not progress, because a wait that re-reads the same thing for hours shows all of them. Overdue is the step past `readyStepOverdueSecondsV1` and nothing else moved for `stallStopMilliseconds`. A pure function of the lines and clocks seen so far.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyStepWatchV1(policy:typeof P=P,prior?:ReadyStepWatchCheckpointV1):Readonly<{observe(lines:readonly string[],nowMs:number):Readonly<{step:ReadyStepV1|undefined;overdue:boolean;quietMs:number}>;snapshot():ReadyStepWatchCheckpointV1}>{
 const seen=new Set(prior?.seen),expressions=policy.stepLinePatterns.map(pattern=>new RegExp(pattern,"u"));let movedAt=prior?.movedAt??null,identity=prior?.identity??null;
 return Object.freeze({snapshot:()=>({movedAt,identity,seen:[...seen]}),observe(lines,nowMs){
  if(movedAt===null)movedAt=nowMs;
  for(const line of lines){
   if(expressions.some(expression=>expression.test(line.replace(/\r/gu,"").trim())))continue;
   const normalized=readyNormaliseLineV1(line,policy.normalisePatterns);
   if(normalized==="")continue;
   const key=watchdogDigestV1(normalized);
   if(seen.has(key))continue;
   if(seen.size>=policy.stallSeenMaximum)seen.clear();
   seen.add(key);movedAt=nowMs;
  }
  const step=readyStepOfV1(lines,policy),key=step===undefined?null:watchdogDigestV1(readyNormaliseLineV1(step.text,policy.normalisePatterns));
  if(key!==null&&key!==identity){if(identity!==null)movedAt=nowMs;identity=key;}
  const quietMs=nowMs-movedAt;
  return{step,overdue:step!==undefined&&step.seconds>=readyStepOverdueSecondsV1(step.bound,policy)&&quietMs>=policy.stallStopMilliseconds,quietMs};
 }});
}

/** 🪤️ The tracker of a wait on a named holder (a log line matching `holderWaitPatterns`): `observe` takes the last lines of the log and the clock and answers for how long the stage has waited for a holder without a round that showed none; a round without a holder line starts the count again.
 * @see ./🧪️tests/🐍️.py */
export function readyHolderWaitTrackerV1(policy:typeof P=P,prior:number|null=null):Readonly<{observe(lines:readonly string[],nowMs:number):number;snapshot():number|null}>{
 const expressions=policy.holderWaitPatterns.map(pattern=>new RegExp(pattern,"u"));let since=prior;
 return Object.freeze({snapshot:()=>since,observe(lines,nowMs){
  if(!lines.some(line=>expressions.some(expression=>expression.test(line)))){since=null;return 0;}
  since??=nowMs;return nowMs-since;
 }});
}

/** 🫱️ The holder a wait line of the dependency store names (English or German): who, since when, how long the waiter has waited, the bound it states and, when the waiter judged the holder silent, for how many seconds. */
export type ReadyHolderWaitV1=Readonly<{pid:number;command:string;since:string;silentSeconds:number|null;seconds:number;limitSeconds:number}>;

/** 🔎️ The holder the newest wait line of `lines` names, `undefined` when none does. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyHolderWaitOfV1(lines:readonly string[],policy:typeof P=P):ReadyHolderWaitV1|undefined{
 const expressions=policy.holderNamedPatterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--){
  const line=lines[index]!.replace(/\r$/u,"");
  for(const expression of expressions){
   const groups=expression.exec(line)?.groups;
   if(groups!==undefined)return Object.freeze({pid:Number(groups.pid),command:groups.command!,since:groups.since!,silentSeconds:groups.silent===undefined?null:Number(groups.silent),seconds:Number(groups.seconds),limitSeconds:Number(groups.limit)});
  }
 }
 return undefined;
}

/** 🪪️ What the lease registry of the worker says about one holder: its pid, the start it registered, the last time it renewed its entry and its command. */
export type ReadyHolderEntryV1=Readonly<{pid:number;startedAtMs:number;updatedAtMs?:number;command:string}>;
/** ⚖️ The judgement of a wait: `progressing` (the holder lives with the start it registered and renews its entry, the stated bound is not spent), `silent` (it lives but has not renewed its entry for `holderSilentMilliseconds`, so it may be blocked), `gone` (no such process, or the pid belongs to another process: the waiter reclaims the lease by itself), `bound` (the stated bound is spent), `unknown` (the process table could not be read). */
export type ReadyHolderVerdictV1=Readonly<{kind:"progressing"|"silent"|"gone"|"bound"|"unknown";silentMs:number}>;

/** ⚖️ Judges a wait on a named holder from the process table, its registry entry and the clock: a waiter behind a live holder that renews its entry is not stalled however long it waits within the bound it states; a holder that is gone or whose pid was reused is reclaimed by the waiter and is no stall either; only a silent or a bound-spent holder is named as the problem. The liveness comes from the process table (on Windows the CIM rows with the creation time, so a reused pid has another start). A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyHolderCustodyV1(input:Readonly<{wait:ReadyHolderWaitV1;entry:ReadyHolderEntryV1|undefined;table:readonly ReadyProcessV1[];nowMs:number;policy?:typeof P}>):ReadyHolderVerdictV1{
 const policy=input.policy??P,wait=input.wait;
 if(wait.seconds>=wait.limitSeconds)return Object.freeze({kind:"bound",silentMs:0});
 if(input.table.length===0)return Object.freeze({kind:"unknown",silentMs:0});
 const identity=input.entry===undefined?(input.table.some(row=>row.pid===wait.pid)?"same":"gone"):readyIdentityV1({pid:wait.pid,startMs:input.entry.startedAtMs},input.table,policy.toleranceMilliseconds);
 if(identity!=="same")return Object.freeze({kind:"gone",silentMs:0});
 const silentMs=input.entry?.updatedAtMs!==undefined&&Number.isFinite(input.entry.updatedAtMs)?Math.max(0,input.nowMs-input.entry.updatedAtMs):(wait.silentSeconds??0)*1000;
 return Object.freeze({kind:silentMs>policy.holderSilentMilliseconds?"silent":"progressing",silentMs});
}

/** 🧾️ What the detached runner of a graph read keeps in `read-record.json`: the kind of read, the runner, the read, when the runner started, and the end status once it ended. */
export type ReadyReadRecordV1=Readonly<{reason:"request"|"refresh";runner:number;read:number|null;startedMs:number;status:number|null;endedMs:number|null}>;

/** 🧘️ The processes to stop so that a foreground stage gets the dependency store from a background reader of the machine's own graph server: the runner of the running read recorded by the graph server (alive with its recorded start, not ended) and everything below it, only when `holder` is among them. Nothing otherwise: a holder of a person, another agent or another checkout is never a victim. A pure function.
 * @see ./🧪️tests/🤝️holder-custody/🐍️.py */
export function readyBackgroundReaderV1(input:Readonly<{record:ReadyReadRecordV1|undefined;holder:number;table:readonly ReadyProcessV1[];self:number;policy?:typeof P}>):Readonly<{root:ReadyKnownV1;plan:readonly number[];reason:"request"|"refresh"}>|undefined{
 const policy=input.policy??P,record=input.record;
 if(record===undefined||record.status!==null||record.endedMs!==null)return undefined;
 const row=input.table.find(each=>each.pid===record.runner);
 if(row===undefined||readyIdentityV1({pid:record.runner,startMs:record.startedMs},input.table,policy.toleranceMilliseconds)!=="same")return undefined;
 const root={pid:row.pid,startMs:row.startMs},plan=readyKillPlanV1({table:input.table,root,known:[],self:input.self,tolerance:policy.toleranceMilliseconds});
 return plan.includes(input.holder)&&!plan.includes(input.self)?Object.freeze({root,plan,reason:record.reason}):undefined;
}

/** 🔁️ Whether a stage that waits on a named holder is stopped to start again with the new code: it has waited `holderWaitRestartMilliseconds` and the code it runs changed since it started. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyHolderRestartV1(waitingMs:number,codeChanged:boolean,policy:typeof P=P):boolean{return codeChanged&&waitingMs>=policy.holderWaitRestartMilliseconds;}

/** 🕰️ Whether a stage that shows progress is stopped to start again with the new code: it has run for `staleStageMilliseconds` and the code it loaded at its start changed since, so a read that reports progress on code that is hours behind hands over (what it finished is stored and kept) and one that started a moment ago is not interrupted by every landing. A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export function readyStaleRestartV1(ageMs:number,codeChanged:boolean,policy:typeof P=P):boolean{return codeChanged&&ageMs>=policy.staleStageMilliseconds;}

/** 🧭️ The phase a stage run is in as its log shows it: reading or planning the project graph, executing planned tasks, or unknown. */
export type ReadyPhaseV1="reading"|"executing"|null;

/** 🧭️ The phase of a stage run as its log shows it: `executing` once a line says the plan is answered or a task runs (`executingPatterns`), `reading` while a line says the project graph is read or planned (`readingPatterns`); the last such line of `lines` decides (an executing line wins on one line), and `previous` stays when no line says either (a stage that never says is `null` for good). A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export function readyStagePhaseV1(lines:readonly string[],previous:ReadyPhaseV1,policy:typeof P=P):ReadyPhaseV1{
 const executing=policy.executingPatterns.map(pattern=>new RegExp(pattern,"u")),reading=policy.readingPatterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--){const line=lines[index]!;if(executing.some(expression=>expression.test(line)))return"executing";if(reading.some(expression=>expression.test(line)))return"reading";}
 return previous;
}

/** 🛑️ Whether the stale-code rule may stop a stage now: only while what it computes is not sealed, that is never in the `executing` phase (its tasks are planned, their hashes fix their inputs and what finishes is stored, the next stage run uses the new code), and then only after the stale age (`staleStageMilliseconds`, already scaled by the machine speed factor) in the phase; a stage that never reports a phase keeps the old rule. A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export function readyStaleStopV1(phase:ReadyPhaseV1,phaseAgeMs:number,policy:typeof P=P):boolean{return phase!=="executing"&&readyStaleRestartV1(phaseAgeMs,true,policy);}

/** 🧬️ The declared semantics of the code a stage loaded: the value of every `semanticsSources` pointer, keyed by file and pointer, `null` for a source that cannot be read now (a landing may be replacing it). A stage that loaded one semantics produces results that another process accepts exactly while the semantics are the same; an edit of any other file changes nothing a result says. Reads two small files, never a tree. A source is bumped when a result changes that another process accepts or reads (the head key a peer accepts from a sealed read, the persisted record format); it is NOT bumped for a change of how a read waits, what it counts as progress or what it says (R184 changed a wait bound and a progress sign: a bump would hand over every healthy read of the fleet and make peers refuse each other's sealed reads for a change that alters no result). A read that does not end itself on old code is ended from outside by the step rule (`readyStepWatchV1`, reason `step-overdue`). */
export function readySemanticsV1(root:string,policy:typeof P=P,fingerprint=(value:string)=>value):Readonly<Record<string,string|null>>{
 const rows:Record<string,string|null>={};
 for(const source of policy.semanticsSources){
  let value:string|null=null;
  try{let node:unknown=JSON.parse(readFileSync(join(root,source.file),"utf8"));for(const part of source.pointer.split("/").slice(1)){node=(node as Record<string,unknown>)?.[part.replaceAll("~1","/").replaceAll("~0","~")];}if(node!==undefined)value=fingerprint(JSON.stringify(node));}catch{value=null;}
  rows[source.file+"#"+source.pointer]=value;
 }
 return Object.freeze(rows);
}

/** 🧬️ The first semantics source (its file as written in the policy) whose value is now another value than `before`, `null` when none is: the code a stage loaded at its start can no longer produce a result another process would accept. A source that cannot be read now or could not be read before is no change. A pure function of the two files. */
export function readySemanticsChangedV1(root:string,before:Readonly<Record<string,string|null>>,policy:typeof P=P,fingerprint?:(value:string)=>string):string|null{
 const now=readySemanticsV1(root,policy,fingerprint);
 for(const source of policy.semanticsSources){const key=source.file+"#"+source.pointer,was=before[key],is=now[key];if(was!==null&&was!==undefined&&is!==null&&is!==undefined&&was!==is)return source.file;}
 return null;
}

const posixProcessColumns="pid=,ppid=,lstart=,comm=";
const posixProcessEnvironment=()=>({...process.env,LC_ALL:"C",TZ:"UTC0"});

/** 📜️ Reads absolute process births from the C-locale UTC `ps` listing, at one-second precision. */
export function readyParseUnixTableV1(text:string):readonly ReadyProcessV1[]{
 const rows:ReadyProcessV1[]=[];
 for(const line of text.split("\n")){
  const found=/^\s*(\d+)\s+(\d+)\s+([A-Z][a-z]{2}) ([A-Z][a-z]{2}) +([0-9]{1,2}) (\d{2}:\d{2}:\d{2}) (\d{4})(?:\s+(\S.*?))?\s*$/u.exec(line);
  if(found===null)continue;
  const pid=Number(found[1]),ppid=Number(found[2]),start=`${found[3]}, ${found[5]!.padStart(2,"0")} ${found[4]} ${found[7]} ${found[6]} GMT`,date=new Date(start),startMs=date.getTime();
  if(!Number.isSafeInteger(pid)||!Number.isSafeInteger(ppid)||!Number.isFinite(startMs)||date.toUTCString()!==start)continue;
  const name=found[8]?.split("/").at(-1);rows.push({pid,ppid,startMs,...name?{name}:{}});
 }
 return rows;
}

const unsigned=(text:string|undefined):number|undefined=>{if(text===undefined||!/^\d+$/u.test(text))return undefined;const value=Number(text);return Number.isSafeInteger(value)?value:undefined;};

/** 📜️ Decodes the tab-separated CIM process table, including exact creation ticks, raw CPU ticks and working-set bytes. */
export function readyParseWindowsTableV1(text:string):readonly ReadyProcessV1[]{
 const rows:ReadyProcessV1[]=[];
 for(const line of text.split(/\r?\n/u)){
  const [p,pp,start,birth,k,u,working,name]=line.split("\t"),pid=unsigned(p),ppid=unsigned(pp),startMs=unsigned(start);
  if(pid===undefined||ppid===undefined||!startMs||pid>0xffffffff||ppid>0xffffffff||birth===undefined||!/^[1-9][0-9]{0,18}$/u.test(birth))continue;
  const kernel=unsigned(k),user=unsigned(u),workingSetBytes=unsigned(working);
  rows.push({pid,ppid,startMs,...name?{name}:{},windows:{birth,...kernel===undefined||user===undefined?{}:{cpu:{kernel,user}},...workingSetBytes?{workingSetBytes}:{}}});
 }
 return rows;
}

export type ReadyWindowsWatchdogSampleV1=Readonly<{rootBirth:string;cpu:readonly Readonly<{pid:number;birth:string;kernel:number;user:number}>[]}>;

/** 🪟️ Reuses the authorized roster and credits only monotonic processor time of the same Windows process birth. */
export function readyWindowsTreeSampleV1(input:Readonly<{table:readonly ReadyProcessV1[];root:number;members:readonly ReadyKnownV1[];previous?:ReadyWindowsWatchdogSampleV1;maximum:number;minimumCpuMilliseconds:number}>):Readonly<{checkpoint:ReadyWindowsWatchdogSampleV1;progress:boolean;megabytes:number|null;changed:boolean}>|undefined{
 const rows=new Map(input.table.map(row=>[row.pid,row])),root=rows.get(input.root),rootBirth=root?.windows?.birth;
 if(rootBirth===undefined||!input.members.some(member=>member.pid===input.root&&member.startMs===root!.startMs)||input.previous!==undefined&&input.previous.rootBirth!==rootBirth)return undefined;
 const previous=new Map(input.previous?.cpu.map(row=>[row.pid,row])),cpu:ReadyWindowsWatchdogSampleV1["cpu"][number][]=[],seen=new Set<number>();
 let bytes=0,complete=true,overflow=false;
 for(const member of input.members){
  if(seen.has(member.pid))continue;
  seen.add(member.pid);
  const row=rows.get(member.pid),metrics=row?.startMs===member.startMs?row.windows:undefined;
  if(metrics?.workingSetBytes===undefined)complete=false;
  else bytes+=metrics.workingSetBytes;
  if(metrics?.cpu!==undefined){
   if(cpu.length<input.maximum)cpu.push({pid:member.pid,birth:metrics.birth,...metrics.cpu});
   else overflow=true;
  }
 }
 if(overflow)cpu.length=0;
 let ticks=0,changed=input.previous===undefined||previous.size!==cpu.length;
 for(const row of cpu){
  const before=previous.get(row.pid);
  if(before===undefined||before.birth!==row.birth){changed=true;continue;}
  if(before.kernel===row.kernel&&before.user===row.user)continue;
  changed=true;
  if(row.kernel>=before.kernel&&row.user>=before.user)ticks+=row.kernel-before.kernel+row.user-before.user;
 }
 return{checkpoint:{rootBirth,cpu},progress:ticks>=input.minimumCpuMilliseconds*10000,megabytes:complete&&Number.isSafeInteger(bytes)?Math.round(bytes/1048576):null,changed};
}

/** 🕰️ Reads one lease owner's absolute process birth, or null when it could not be observed. */
export function readyProcessStartSyncV1(pid:number,platform:NodeJS.Platform=process.platform):number|null{
 try{
  const id=Math.trunc(pid);
  if(!Number.isInteger(id)||id<1)return null;
  if(platform==="win32"){
   const run=spawnSync("powershell.exe",["-NoProfile","-NonInteractive","-Command",`([DateTimeOffset](Get-CimInstance Win32_Process -Filter 'ProcessId=${id}').CreationDate).ToUnixTimeMilliseconds()`],{encoding:"utf8",windowsHide:true,timeout:15000}),value=Number(run.stdout.trim());
   return Number.isFinite(value)&&value>0?value:null;
  }
  const run=spawnSync("ps",["-p",String(id),"-o",posixProcessColumns],{encoding:"utf8",timeout:5000,env:posixProcessEnvironment()});
  return run.status===0?readyParseUnixTableV1(run.stdout).find(row=>row.pid===id)?.startMs??null:null;
 }catch{return null;}
}

/** 🖥️ The command that prints the process table on this platform. */
export function readyTableCommandV1(platform:NodeJS.Platform=process.platform):Readonly<{command:string;args:readonly string[]}>{
 return platform==="win32"
  ?{command:"powershell.exe",args:["-NoProfile","-NonInteractive","-Command","Get-CimInstance -ClassName Win32_Process -Property ProcessId,ParentProcessId,CreationDate,KernelModeTime,UserModeTime,WorkingSetSize,Name -ErrorAction Stop | ForEach-Object { if ($null -ne $_.CreationDate) { $born = [DateTimeOffset]$_.CreationDate; @($_.ProcessId,$_.ParentProcessId,$born.ToUnixTimeMilliseconds(),$born.UtcTicks,$_.KernelModeTime,$_.UserModeTime,$_.WorkingSetSize,$_.Name) -join [char]9 } }"]}
  :{command:"ps",args:["-A","-o",posixProcessColumns]};
}

/** 📜️ Reads absolute process births independently of when the table command is scheduled; failed and timed-out observations return no rows.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyReadTableV1(timeoutMilliseconds:number=20000,platform:NodeJS.Platform=process.platform,run?:(command:string,args:readonly string[],timeoutMilliseconds:number)=>Promise<string|null>,signal?:AbortSignal):Promise<readonly ReadyProcessV1[]>{
 if(signal?.aborted)return Promise.resolve([]);
 const {command,args}=readyTableCommandV1(platform);
 const parse=(out:string|null):readonly ReadyProcessV1[]=>out===null?[]:platform==="win32"?readyParseWindowsTableV1(out):readyParseUnixTableV1(out);
 if(run!==undefined)return run(command,args,timeoutMilliseconds).then(text=>signal?.aborted?[]:parse(text),()=>[]);
 return new Promise(done=>{
  let out="",finished=false;
  const end=(table:readonly ReadyProcessV1[])=>{if(!finished){finished=true;clearTimeout(timer);signal?.removeEventListener("abort",stop);done(table);}};
  let child:ReturnType<typeof spawn>;
  try{child=spawn(command,[...args],{windowsHide:true,stdio:["ignore","pipe","ignore"],...platform==="win32"?{}:{env:posixProcessEnvironment()}});}catch{return done([]);}
  const stop=()=>{try{child.kill();}catch{}end([]);};
  const timer=setTimeout(stop,timeoutMilliseconds);
  signal?.addEventListener("abort",stop,{once:true});
  child.stdout?.on("data",chunk=>{out+=String(chunk);});
  child.on("error",()=>end([]));
  child.on("close",code=>end(code===0?parse(out):[]));
 });
}

/** 💾️ Writes the record of the processes seen below a stage (tmp + rename). */
export function readyWriteTreeV1(directory:string,stage:string,root:ReadyKnownV1,processes:readonly ReadyKnownV1[],nowMs:number):void{
 try{mkdirSync(directory,{recursive:true});const file=join(directory,`${stage}.${P.treeName}`),temporary=`${file}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify({version:1,stage,root,sampledMs:nowMs,processes}));renameSync(temporary,file);}catch{}
}

/** 💾️ Reads the record of the processes seen below a stage, `undefined` when there is none or it is damaged. */
export function readyReadTreeV1(directory:string,stage:string):Readonly<{root:ReadyKnownV1;processes:readonly ReadyKnownV1[]}>|undefined{
 try{const row=JSON.parse(readFileSync(join(directory,`${stage}.${P.treeName}`),"utf8")) as {version:number;root:ReadyKnownV1;processes:ReadyKnownV1[]};return row.version===1?{root:row.root,processes:row.processes}:undefined;}catch{return undefined;}
}

/** 🗑️ Forgets the record of a stage. */
export function readyForgetTreeV1(directory:string,stage:string):void{try{rmSync(join(directory,`${stage}.${P.treeName}`),{force:true});}catch{}}

/** 🪓️ The arguments of the one Windows command that ends a process and everything it started that the system still knows: `taskkill /PID <pid> /T /F`. */
export function readyTaskkillArgumentsV1(pid:number):readonly string[]{return["/PID",String(Math.trunc(pid)),"/T","/F"];}

/** 🔪️ Stops the tree of a stage on every platform: the plan from the process table now and the processes recorded earlier, leaf first, each with `signal`; on Windows `taskkill /F` per process of the plan, because a tree walk by the system loses every child of an exited parent. Returns the pids it signalled. */
export async function readyKillTreeV1(input:Readonly<{root:ReadyKnownV1;known:readonly ReadyKnownV1[];signal:"SIGTERM"|"SIGKILL";self?:number;table?:readonly ReadyProcessV1[];platform?:NodeJS.Platform;spare?:ReadonlySet<number>;record?:(row:ReadyProcessV1,signal:"SIGTERM"|"SIGKILL")=>void;kill?:(pid:number,signal:"SIGTERM"|"SIGKILL")=>void}>):Promise<readonly number[]>{
 const platform=input.platform??process.platform,table=input.table??await readyReadTableV1(20000,platform),plan=readyKillPlanV1({table,root:input.root,known:input.known,self:input.self??process.pid,...input.spare===undefined?{}:{spare:input.spare}});
 const kill=input.kill??((pid:number,signal:"SIGTERM"|"SIGKILL")=>{
  try{
   if(platform==="win32")spawn("taskkill",[...readyTaskkillArgumentsV1(pid)],{windowsHide:true,stdio:"ignore"}).on("error",()=>undefined);
   else process.kill(pid,signal);
  }catch{}
 });
 for(const pid of plan){const row=table.find(each=>each.pid===pid);if(row!==undefined)try{input.record?.(row,input.signal);}catch{}kill(pid,input.signal);}
 return plan;
}

/** ⏲️ The processor time of `ps -o time=` (`[[dd-]hh:]mm:ss[.cc]`) in milliseconds, `null` when the text is not that. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyCpuMillisecondsV1(text:string):number|null{
 const found=/^\s*(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:[.,]\d+)?)\s*$/u.exec(text);
 return found===null?null:Math.round((((Number(found[1]??0)*24+Number(found[2]??0))*60+Number(found[3]))*60+Number(found[4]!.replace(",","."))) *1000);
}

/** 🔥️ The processor time of every process of a `pid,ppid,rss,time` listing (`ps -A -o pid=,ppid=,rss=,time=`) that is `root`, below it or one of `extra`: pid to milliseconds. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyTreeCpuV1(rows:string,root:number,extra:readonly number[]=[]):ReadonlyMap<number,number>{
 const parent=new Map<number,number>(),cpu=new Map<number,number>(),wanted=new Set(extra);
 for(const line of rows.split("\n")){const found=/^\s*(\d+)\s+(\d+)\s+\d+\s+(\S+)\s*$/u.exec(line),spent=found===null?null:readyCpuMillisecondsV1(found[3]!);if(found!==null&&spent!==null){parent.set(Number(found[1]),Number(found[2]));cpu.set(Number(found[1]),spent);}}
 const tree=new Map<number,number>();
 for(const [pid,spent] of cpu){if(wanted.has(pid)){tree.set(pid,spent);continue;}for(let at:number|undefined=pid,depth=0;at!==undefined&&depth<64;at=parent.get(at),depth++)if(at===root){tree.set(pid,spent);break;}}
 return tree;
}

/** 💓️ Whether a stage is alive between two looks at its tree: its processes spent at least `livenessCpuMilliseconds` of processor time (a silent `rustc` does) or a process joined the tree (a build that starts children does). A pure function. Log output is not the only sign of life.
 * @see ./🧪️tests/🐍️.py */
export function readyLivenessV1(previous:ReadonlyMap<number,number>,current:ReadonlyMap<number,number>,policy:typeof P=P):boolean{
 let spent=0;
 for(const [pid,now] of current){const before=previous.get(pid);if(before===undefined){if(previous.size>0)return true;}else spent+=Math.max(0,now-before);}
 return spent>=policy.livenessCpuMilliseconds;
}
