import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {scaledMillisecondsV1} from "../../../../🐢️speed/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {pinSpawnedProcessV1,processIdentityVerdictV1,readProcessStartV1}from"../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

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
 const verdict=processIdentityVerdictV1(known,table.find(each=>each.pid===known.pid)?.startMs??"gone",tolerance);
 return verdict==="same"||verdict==="reused"?verdict:"gone";
}

/** 📌️ The identity of the stage process spawned at `startedMs`: the recorded start of the pid in an earlier sample (`restored`, a start other than the spawn time), else the row of the table in the window a spawn can have created it (the clock is read before the system creates the process and a loaded Windows machine creates it late), else the spawn time itself, which then judges any row outside the window another process. The creation time is never taken from the clock before the spawn alone: that told a slow Windows spawn from a reused pid.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyPinStageRootV1(flight:Readonly<{pid:number;startedMs:number}>,table:readonly ReadyProcessV1[],restored?:ReadyKnownV1|undefined):ReadyKnownV1{
 if(restored!==undefined&&restored.pid===flight.pid&&restored.startMs!==flight.startedMs&&pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},restored.startMs)!==undefined)return{pid:restored.pid,startMs:restored.startMs};
 const pinned=pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},table.find(row=>row.pid===flight.pid)?.startMs);
 return pinned===undefined?{pid:flight.pid,startMs:flight.startedMs}:pinned;
}

/** 🧭️ Whether the process now at the pid of a stage in flight is that stage's process (`same`) or another process that reused the pid (`reused`), from its creation time `seenMs` (`null`: not readable, never a verdict of reuse) and the pinned start of the tree record: a pinned start is compared exactly, an unpinned one by the window a spawn can have created it in.
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyStageProcessVerdictV1(flight:Readonly<{pid:number;startedMs:number}>,restored:ReadyKnownV1|undefined,seenMs:number|null):"same"|"reused"{
 if(seenMs===null)return"same";
 const root=readyPinStageRootV1(flight,[],restored);
 if(root.startMs!==flight.startedMs)return processIdentityVerdictV1(root,seenMs)==="same"?"same":"reused";
 return pinSpawnedProcessV1({pid:flight.pid,spawnedMs:flight.startedMs},seenMs)!==undefined?"same":"reused";
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
 const row=status as{ready?:unknown;state?:unknown;updatedMs?:unknown;refreshing?:unknown};
 return(row.ready===false&&(row.state==="running"||row.state==="waiting"||row.state==="backoff")||row.ready===true&&row.state==="ready"&&row.refreshing===true)&&typeof row.updatedMs==="number"&&Number.isFinite(row.updatedMs)&&nowMs-row.updatedMs<=policy.foregroundFreshMilliseconds;
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
 return {...policy,speedFactor:Math.max(1,factor),stallMilliseconds:scaledMillisecondsV1(policy.stallMilliseconds,factor),stallStopMilliseconds:scaledMillisecondsV1(policy.stallStopMilliseconds,factor),holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),holderSilentMilliseconds:scaledMillisecondsV1(policy.holderSilentMilliseconds,factor),holderYieldMilliseconds:scaledMillisecondsV1(policy.holderYieldMilliseconds,factor),staleStageMilliseconds:scaledMillisecondsV1(policy.staleStageMilliseconds,factor),staleWaitingMilliseconds:scaledMillisecondsV1(policy.staleWaitingMilliseconds,factor)};
}

/** 🔁️ What follows the `consecutive`-th stop in a row of one silent stage: a restart after an exponential back-off (`stallRestartBackoffMilliseconds` doubling up to `stallRestartBackoffMaximumMilliseconds`), and a failed state once more than `stallRestartMaximum` stops followed each other, so a stage that waits for something that never comes ends with its reason instead of being stopped every ten minutes for ever. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readyStallRestartV1(input:Readonly<{consecutive:number;policy?:typeof P}>):Readonly<{action:"restart"|"fail";waitMilliseconds:number}>{
 const policy=input.policy??P,count=Math.max(1,Math.floor(input.consecutive));
 if(count>policy.stallRestartMaximum)return{action:"fail",waitMilliseconds:0};
 return{action:"restart",waitMilliseconds:Math.min(policy.stallRestartBackoffMaximumMilliseconds,policy.stallRestartBackoffMilliseconds*2**(count-1))};
}

/** ⏳️ What to do with a stage past its soft cap: it runs while inside the cap; between the soft and the hard cap (the configured stage multiplier times the soft cap) it goes on, `extended`, as long as a counter of its log changed within the stall limit, and past the hard cap it goes on the same way up to `progressCeilingMultiplier` times the soft cap (a stage that makes progress is never killed by a clock, only by a stall or the ceiling); otherwise it is stopped, `stalled` when the counters stood still before the hard cap, `timeout` when they stood still past it or at the ceiling. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyDeadlineV1(input:Readonly<{stage?:string;elapsedMs:number;softMs:number;sinceProgressMs:number;policy?:typeof P}>):"run"|"extended"|"timeout"|"stalled"{
 const policy=input.policy??P;
 if(input.elapsedMs<=input.softMs)return"run";
 const overrides:Readonly<Record<string,number>>=policy.stageHardCapMultipliers;
 const multiplier=input.stage!==undefined&&Object.hasOwn(overrides,input.stage)?overrides[input.stage]!:policy.hardCapMultiplier;
 if(input.elapsedMs>input.softMs*Math.max(multiplier,policy.progressCeilingMultiplier))return"timeout";
 if(input.elapsedMs>input.softMs*multiplier)return input.sinceProgressMs<=policy.stallMilliseconds?"extended":"timeout";
 return input.sinceProgressMs<=policy.stallMilliseconds?"extended":"stalled";
}

/** 🗓️ The wall-clock moment at which a stage is ended whatever else is alive: its start plus its soft cap times the larger of the hard-cap multiplier of the stage and the progress ceiling plus `absoluteDeadlineExtraIntervals` (the room the tools grace may add). The detached wrapper enforces it without the sidecar, so a dead sidecar never leaves a stage without a deadline. A pure function.
 * @see ./🧪️tests/🗓️absolute-deadline */
export function readyAbsoluteDeadlineV1(input:Readonly<{stage:string;startedMs:number;softMs:number;policy?:typeof P}>):number{
 const policy=input.policy??P,overrides:Readonly<Record<string,number>>=policy.stageHardCapMultipliers,multiplier=Object.hasOwn(overrides,input.stage)?overrides[input.stage]!:policy.hardCapMultiplier;
 return input.startedMs+input.softMs*(Math.max(multiplier,policy.progressCeilingMultiplier)+policy.absoluteDeadlineExtraIntervals);
}

/** 🔢️ The progress of a stage as the last line of its log that carries a counter (the matching text, so the same counter is the same key and a counter that moved is another): `null` when no line does. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyProgressKeyV1(lines:readonly string[],patterns:readonly string[]=P.progressPatterns):string|null{
 const expressions=patterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--)for(const expression of expressions){const found=expression.exec(lines[index]!);if(found!==null)return found[0];}
 return null;
}

/** 🧽️ A line of a stage log with its elapsed-seconds counters, clocks and timestamps replaced by `#` (`normalisePatterns` of the schema), a carriage return dropped and a Windows path (drive letter, backslashes or doubled backslashes of JSON, any case) spelled `c:/users/x`, so a line that only counts seconds or only spells a path the Windows way is the same line every time; a counter such as `1957 of 2927` stays. A wait in the host capacity gate is one line whatever the gate reports (available megabytes, grants, queue, reason): a stage queued there computes nothing, so its changing facts are no progress, the stall rule ends a wait that never gets its grant, and the restart runs on the current code (a slice read queued for 7 h behind its own session's grants on old gate code looked alive forever). A pure function.
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

/** 🔥️ Whether a stage whose only sign of life is processor time has had nothing else for `cpuOnlyStallMilliseconds`: a loop that burns CPU and decides nothing is a stall, however busy the tree looks.
 * @see ./🧪️tests/🔥️cpu-only */
export function readyCpuOnlyStalledV1(cpuSince:number|undefined,nowMs:number,policy:typeof P=P):boolean{return cpuSince!==undefined&&nowMs-cpuSince>=policy.cpuOnlyStallMilliseconds;}

/** 🛑️ Whether a stage whose output showed nothing new for `sinceProgressMs` is stopped to be tried again: at `stallStopMilliseconds` and later. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyStalledV1(sinceProgressMs:number,policy:typeof P=P,exempt:boolean=false):boolean{return!exempt&&sinceProgressMs>=policy.stallStopMilliseconds;}

/** 🧊️ Whether the host warden holds a process of the stage tree suspended: a suspended process cannot show anything new, so the stall window does not run while the warden holds one (the warden bounds the hold itself: it resumes or cancels after `suspendMaximumSeconds`). A pure function.
 * @see ./🧪️tests/🧊️warden-hold */
export function readyWardenHoldsV1(mirror:Readonly<{absent?:true;suspended?:readonly Readonly<{pid:number}>[]}>,tree:ReadonlySet<number>):boolean{return mirror.absent!==true&&(mirror.suspended??[]).some(row=>tree.has(row.pid));}

/** 🧯️ The newest cancel record the host warden wrote for the stage process itself (`pid`, the wrapper that owns the stage tree), `undefined` when it cancelled none: only then is a SIGTERM of the stage the warden's and not a crash or a person's. A pure function.
 * @see ./🧪️tests/🧯️warden-cancel */
export function readyWardenCancelOfStageV1<Row extends Readonly<{pid:number;atMs:number}>>(cancels:readonly Row[],pid:number):Row|undefined{
 let newest:Row|undefined;
 for(const record of cancels)if(record.pid===pid&&(newest===undefined||record.atMs>newest.atMs))newest=record;
 return newest;
}

/** 🗞️ The signals of the stage runner that its kill record `kills.log` lists since `sinceMs`: time, pid of the process that received the signal, the signal, the runner process that sent it and its stage, parsed from the English half of each line: the ready stage runner's `Process <pid> (<name>) received <signal> from the ready stage runner of this worker (runner process <killer>, stage <stage>, ...)` and the owned execution's `Process <pid> received <signal> from owned execution <label> (killing process <killer>): ...` (the label is the stage); lines of other forms are ignored. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export type ReadyKillRowV1=Readonly<{atMs:number;pid:number;signal:"SIGTERM"|"SIGKILL";killer:number;stage:string}>;
export function readyKillRowsV1(text:string,sinceMs:number):readonly ReadyKillRowV1[]{
 const rows:ReadyKillRowV1[]=[],expressions=[/^(\S+) Process (\d+) \(.*?\) received (SIGTERM|SIGKILL) from the ready stage runner of this worker \(runner process (\d+), stage ([^,]+),/u,/^(\S+) Process (\d+) received (SIGTERM|SIGKILL) from owned execution (.+?) \(killing process (\d+)\):/u];
 for(const line of text.split("\n").slice(-P.senderRecordLines)){
  const first=expressions[0]!.exec(line),second=first===null?expressions[1]!.exec(line):null,match=first??(second===null?null:[second[0],second[1],second[2],second[3],second[5],second[4]]as unknown as RegExpExecArray);
  if(match===null)continue;
  const atMs=Date.parse(match[1]!);
  if(Number.isFinite(atMs)&&atMs>=sinceMs)rows.push({atMs,pid:Number(match[2]),signal:match[3] as "SIGTERM"|"SIGKILL",killer:Number(match[4]),stage:match[5]!});
 }
 return rows;
}

/** 📟️ Who stopped a stage whose command ended by a signal, answered only from what this machine recorded before the signal was sent: `supervisor` (the stage record carries `stopBy`), `deadline` (the wrapper recorded its own deadline), `warden` (a cancel row of the host warden names the stage process), `runner` (the kill record of the stage runner names the stage process), and otherwise `outside-wrapper` (the wrapper itself received the signal) or `outside-child` (only the tree below it was signalled); `none` when the stage did not end by a signal (`ended` is `null`). A record counts only within `senderMatchMilliseconds` of the end, so an old row never explains a later end. The platform does not tell the pid of a sender, so an unrecorded sender is named by what it was not. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export type ReadyStopSenderV1=Readonly<{kind:"supervisor"|"deadline"|"warden"|"runner"|"outside-wrapper"|"outside-child"|"none";line:string}>;
export type ReadyStopSenderInputV1=Readonly<{ended:string|null;stagePid:number;endedMs:number;observer:number;stopBy?:Readonly<{supervisor:number;reason:string;atMs:number}>;received?:readonly Readonly<{signal:string;atMs:number}>[];sent?:readonly Readonly<{signal:string;reason:string;atMs:number}>[];kills:readonly ReadyKillRowV1[];warden?:Readonly<{atMs:number;kind:string;reason:string;megabytes:number}>}>;
export function readyStopSenderV1(input:ReadyStopSenderInputV1,policy:typeof P=P):ReadyStopSenderV1{
 if(input.ended===null)return{kind:"none",line:""};
 const texts=schema.definitions.SenderTexts.const,near=(atMs:number):boolean=>Math.abs(input.endedMs-atMs)<=policy.senderMatchMilliseconds,time=(atMs:number):string=>new Date(atMs).toISOString();
 const say=(key:keyof typeof texts.en,values:Readonly<Record<string,string|number>>):string=>(["en","de"] as const).map(language=>texts[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]??""))).join(" / ");
 const rowOf=input.kills.find(row=>row.pid===input.stagePid&&near(row.atMs)),others=input.kills.filter(row=>row!==rowOf).length;
 const deadline=input.sent?.find(row=>row.reason==="deadline"&&near(row.atMs));
 if(deadline!==undefined)return{kind:"deadline",line:say("deadline",{signal:deadline.signal,time:time(deadline.atMs)})};
 const by=input.stopBy;
 if(by!==undefined&&near(by.atMs))return{kind:"supervisor",line:say(by.supervisor===input.observer?"supervisorSelf":"supervisorOther",{supervisor:by.supervisor,time:time(by.atMs),reason:by.reason})};
 const warden=input.warden;
 if(warden!==undefined&&near(warden.atMs))return{kind:"warden",line:say("warden",{time:time(warden.atMs),kind:warden.kind,reason:warden.reason,megabytes:warden.megabytes})};
 if(rowOf!==undefined)return{kind:"runner",line:say("runner",{killer:rowOf.killer,signal:rowOf.signal,time:time(rowOf.atMs)})};
 const first=input.received?.[0];
 return first!==undefined?{kind:"outside-wrapper",line:say("outsideWrapper",{signal:first.signal,time:time(first.atMs),rows:others})}:{kind:"outside-child",line:say("outsideChild",{signal:input.ended,rows:others})};
}

/** 🪧️ Where the stall clock of a stage starts that a supervisor adopts without the watchdog history of the one that started it (no record, or one the current schema does not accept): at the adoption, never at the start of the stage. The adopting supervisor cannot know how long the stage was quiet, so a stage is never stopped for lack of history; with history the recorded clock and the time nobody watched are used instead. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export function readyAdoptionClockV1(input:Readonly<{startedMs:number;observedMs:number}>):number{return Math.max(input.startedMs,input.observedMs);}

/** 🔐️ Whether the read of the stage is queued for the storage mapping: its own fresh slice report (younger than `grantWaitFreshMilliseconds`) lists a waiter of the mapping queue (`presence.mappingQueue.waiting`) whose current wait is shorter than `grantWaitMaximumMilliseconds`. With the shared grant never queued behind long exclusive work a wait lasts seconds, so this is only a bounded safety: the stall window does not run while a short wait is pending, and a wait past the maximum is a wedge the stall rule ends. A pure function.
 * @see ./🧪️tests/🔐️grant-wait */
export function readyGrantWaitV1(report:unknown,nowMs:number,policy:typeof P=P):boolean{
 const row=report as {atMs?:number;sources?:{owners?:{presence?:{mappingQueue?:{waiting?:readonly {waitedMilliseconds?:number}[]}}}}}|null|undefined;
 if(typeof row?.atMs!=="number"||nowMs-row.atMs>policy.grantWaitFreshMilliseconds)return false;
 const waiting=row.sources?.owners?.presence?.mappingQueue?.waiting;
 return Array.isArray(waiting)&&waiting.some(wait=>typeof wait?.waitedMilliseconds==="number"&&wait.waitedMilliseconds<policy.grantWaitMaximumMilliseconds);
}

/** 🔇️ Whether the read of the stage went silent: its own slice report of this attempt (written at `atMs`, not before `startedMs`) is not the last one of a finished read (`reason` "end") and is older than `sliceSilentMilliseconds`. The slice worker writes it every 30 s from its event loop, so a report that old means a loop that no longer turns (a call blocked in the kernel, a stopped process, a wait that never settles), whatever the processor time of the tree or the progress line the host repeats from the last phase say; such a read is a stall when the stage log also showed no new line for the same window (the supervisor checks that beside it: a worker whose loop turns rarely but whose `taken` still moves is not one). A pure function.
 * @see ./🧪️tests/🔇️slice-silent */
export function readySliceSilentV1(report:unknown,nowMs:number,startedMs:number,policy:typeof P=P):boolean{
 const row=report as {atMs?:unknown;reason?:unknown}|null|undefined;
 return typeof row?.atMs==="number"&&row.atMs>=startedMs&&row.reason!=="end"&&nowMs-row.atMs>=policy.sliceSilentMilliseconds;
}

/** 🚶️ The progress signature of a fresh slice report of this attempt: the phase of the read with its done and total counts, the owners the ledger decided or runs, and the owners taken or finished; `null` for a report of an earlier attempt, a finished read, one without a clock or a malformed one. A change of the signature between two looks is progress of the read although the stage log repeats its last line (a planner of 440 owners that takes 15 minutes on a swapping 16 GB machine still counts `taken 559 of 616` in its own report); the processor time of the worker and the clock are never part of it, so a loop that spins without deciding anything is no progress. A pure function.
 * @see ./🧪️tests/🚶️slice-moved */
export function readySliceMovedSignatureV1(report:unknown,startedMs:number):string|null{
 const row=report as {atMs?:unknown;reason?:unknown;phase?:{name?:unknown;done?:unknown;total?:unknown};sources?:{owners?:{ledger?:{done?:unknown;running?:unknown;total?:unknown};finished?:unknown;taken?:unknown}}}|null|undefined;
 if(typeof row?.atMs!=="number"||row.atMs<startedMs||row.reason==="end")return null;
 const owners=row.sources?.owners,ledger=owners?.ledger;
 return JSON.stringify([row.phase?.name,row.phase?.done,row.phase?.total,ledger?.done,ledger?.running,ledger?.total,owners?.finished,owners?.taken]);
}

/** 🌱️ The graph counter tracker of one stage run: `observe` answers whether the newest counter of `graphCounterPatterns` (`taken 461 of 600`) is above the highest one seen, so a read whose text repeats after normalisation but whose server counter advanced is progress; a counter that repeats or falls back is none. A pure function of the lines seen so far.
 * @see ./🧪️tests/🌱️graph-progress */
export function readyGraphCounterTrackerV1(policy:typeof P=P,prior:number=-1):Readonly<{observe(lines:readonly string[]):boolean;snapshot():number}>{
 const expressions=policy.graphCounterPatterns.map(pattern=>new RegExp(pattern,"u"));let highest=prior;
 return Object.freeze({snapshot:()=>highest,observe(lines){
  let advanced=false;
  for(const line of lines)for(const expression of expressions){const done=expression.exec(line)?.groups?.done;if(done!==undefined&&Number(done)>highest){highest=Number(done);advanced=true;}}
  return advanced;
 }});
}

/** 🌱️ Whether the stage is in a read whose graph server is alive: the phase is `reading` and the read record names a runner that is not ended and still alive; such a read is bounded by the hard cap of its stage, not by the stall stop. A pure function.
 * @see ./🧪️tests/🌱️graph-progress */
export function readyReadingLiveV1(input:Readonly<{phase:ReadyPhaseV1;record:ReadyReadRecordV1|undefined;alive:(pid:number)=>boolean}>):boolean{return input.phase==="reading"&&input.record!==undefined&&input.record.endedMs===null&&input.alive(input.record.runner);}

/** 🔁️ The number of stops in a row that count against the restart budget: a stall after which the attempt had made progress starts the count again at one, only silent attempts add up. A pure function.
 * @see ./🧪️tests/🌱️graph-progress */
export function readyStallCountV1(prior:number,progressed:boolean):number{return progressed?1:prior+1;}

/** 🪜️ A step of the read as its progress line states it: the step in both languages (without the note that it passed its bound), the seconds it has run and the bound it declared. */
export type ReadyStepV1=Readonly<{text:string;seconds:number;bound:number}>;

/** 🪜️ The step the newest step line of `lines` names (`stepLinePatterns`: the progress line of the slice worker whose phase is `<English> / <German>` and whose counter is the seconds the step has run of the seconds it is bounded by), `undefined` when no line is one. The seconds and the bound are in the parenthesis, not in the step text, so the same step is the same text before and after its bound. A pure function.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyStepOfV1(lines:readonly string[],policy:typeof P=P):ReadyStepV1|undefined{
 const expressions=policy.stepLinePatterns.map(pattern=>new RegExp(pattern,"u"));
 for(let index=lines.length-1;index>=0;index--)for(const expression of expressions){
  const found=expression.exec(lines[index]!.replace(/\r/gu,"").trim());
  if(found?.groups!==undefined)return{text:found.groups.step!,seconds:Number(found.groups.done),bound:Number(found.groups.bound)};
 }
 return undefined;
}

/** 🪜️ The seconds a step may run before it is overdue: its declared bound, written for the reference machine and lengthened by the speed factor of this machine like every other limit (`scaledMillisecondsV1`), plus the window without progress that already ends a silent stage here (`stallStopMilliseconds`, scaled the same way). So a step is overdue exactly when it has been past what this machine is expected to need for as long as a stage may be silent; both parts follow the speed factor and nothing is a number of its own. A pure function.
 * @see ./🧪️tests/🪜️step-overdue */
export function readyStepOverdueSecondsV1(boundSeconds:number,policy:typeof P=P):number{
 return Math.ceil((scaledMillisecondsV1(boundSeconds*1000,policy.speedFactor)+policy.stallStopMilliseconds)/1000);
}

export type ReadyStepWatchCheckpointV1=Readonly<{movedAt:number|null;identity:string|null;seen:readonly string[]}>;

/** 🪜️ The step watch of one stage run: `observe` takes the last lines of the log and the clock and answers the newest step, whether it is overdue and for how long nothing else moved. Something else moved when a line that is no step line is new after normalisation (a counter such as `taken 8 of 590`, an owner finished, a message) or when the step is another step than before; the seconds counter of the step itself beyond its bound, the processor time of the tree and a repeated heartbeat are not progress (a step within its bound is alive by definition), because a wait that re-reads the same thing for hours shows all of them. Overdue is the step past `readyStepOverdueSecondsV1` and nothing else moved for `stallStopMilliseconds`. A pure function of the lines and clocks seen so far.
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
  if(step!==undefined&&step.seconds<=step.bound)movedAt=nowMs;
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

/** ⏳️ Whether a log line only says that the stage WAITS for the capacity gate or a named holder (`waitPatterns`): such a line changes its numbers with every look (`short by 790 MB`, `load 6.99`) and is therefore never progress. A pure function.
 * @see ./🧪️tests/🕰️stale-waiting */
export function readyWaitLineV1(line:string,policy:typeof P=P):boolean{return policy.waitPatterns.some(pattern=>new RegExp(pattern,"u").test(line))||readyCapacityWaitLineV1(line,policy);}

/** 🚦️ Whether a log line says that the read or the command WAITS for the capacity gate of the host (`capacityWaitPatterns`: the `[capacity] waiting` line of a command and the slice reservation, memory headroom and previous generation lines of the graph service, in both languages). A pure function.
 * @see ./🧪️tests/🚦️capacity-wait */
export function readyCapacityWaitLineV1(line:string,policy:typeof P=P):boolean{return policy.capacityWaitPatterns.some(pattern=>new RegExp(pattern,"u").test(line));}

/** 🚦️ Whether one of the last `capacityWaitLookbackLines` lines of the log is a capacity wait. A pure function.
 * @see ./🧪️tests/🚦️capacity-wait */
export function readyCapacityWaitingV1(lines:readonly string[],policy:typeof P=P):boolean{return lines.slice(-policy.capacityWaitLookbackLines).some(line=>readyCapacityWaitLineV1(line,policy));}

/** 🚦️ Whether a stage that waits in the capacity gate is alive and is exempt from the stall stop: the wait is current (a wait line is among the last lines), its log was written within `capacityWaitFreshMilliseconds` (a waiting planner says so again every few seconds, a wedged one does not) and it has not lasted `capacityWaitMaximumMilliseconds` (a gate that never answers ends in the stall stop, so no stage waits for ever). A queue wait is no silence: it shows nothing new and uses no processor time by design. A pure function.
 * @see ./🧪️tests/🚦️capacity-wait */
export function readyCapacityWaitLiveV1(input:Readonly<{sinceMs:number|null;logWrittenMs:number|null;nowMs:number;policy?:typeof P}>):boolean{
 const policy=input.policy??P;
 return input.sinceMs!==null&&input.logWrittenMs!==null&&input.nowMs-input.logWrittenMs<policy.capacityWaitFreshMilliseconds&&input.nowMs-input.sinceMs<policy.capacityWaitMaximumMilliseconds;
}

/** ⏱️ How long the wait line says the stage has been waiting (`[capacity] waiting 15549s for command: ...`), in milliseconds; `null` for any other line. A supervisor that adopts a stage hours into its wait learns the age of the wait from the line itself instead of starting its own clock, so a worker whose supervisor is restarted by every landing still reaches the stale-waiting verdict. A pure function.
 * @see ./🧪️tests/🕰️stale-waiting */
export function readyWaitedMillisecondsV1(line:string):number|null{const found=/^\[(?:capacity\] waiting|Kapazität\] wartet) (\d{1,9}) ?s\b/u.exec(line);return found===null?null:Number(found[1])*1000;}

/** 🕰️ Whether a stage that runs older code than the code now running is stopped to start again on the current one: its last line is a wait (`waiting`) and nothing but waits was new for `staleWaitingMilliseconds` (`quietMs`: no new line besides waits, no processor time, no new process, no counter), 15 minutes, three times the quiet window of a stale read. Code that is not older never stops by it, and a stage that makes progress has a young `quietMs`, so it is never stopped. A pure function.
 * @see ./🧪️tests/🕰️stale-waiting */
export function readyStaleWaitingV1(input:Readonly<{older:boolean;waiting:boolean;quietMs:number;policy?:typeof P}>):boolean{return input.older&&input.waiting&&input.quietMs>=(input.policy??P).staleWaitingMilliseconds;}

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

/** 🪢️ What a changed code declaration does to a running stage, decided in one place: `"holder"` when the stage has waited `holderWaitRestartMilliseconds` for a named holder (it makes no progress by definition), `"stale"` when it only reads or plans (never in the `executing` phase), has been in that phase for the stale age AND has shown no progress for `codeQuietMilliseconds`, `null` otherwise: a stage that is making progress is never stopped for new code, the new code applies to its next attempt, so a landing never costs an hour of reading. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readyCodeStopV1(input:Readonly<{holderWaitMs:number;phase:ReadyPhaseV1;phaseAgeMs:number;quietMs:number;policy?:typeof P}>):"holder"|"stale"|null{
 const policy=input.policy??P;
 if(readyHolderRestartV1(input.holderWaitMs,true,policy))return"holder";
 return readyStaleStopV1(input.phase,input.phaseAgeMs,policy)&&input.quietMs>=policy.codeQuietMilliseconds?"stale":null;
}

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

/** 🕰️ Reads one lease owner's absolute process birth, or null when it could not be observed: the shared process identity read (Windows `Win32_Process.CreationDate`, elsewhere `ps -o lstart=`).
 * @see ../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts */
export function readyProcessStartSyncV1(pid:number,platform:NodeJS.Platform=process.platform):number|null{
 const read=readProcessStartV1(pid,platform);
 return typeof read==="number"?read:null;
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
