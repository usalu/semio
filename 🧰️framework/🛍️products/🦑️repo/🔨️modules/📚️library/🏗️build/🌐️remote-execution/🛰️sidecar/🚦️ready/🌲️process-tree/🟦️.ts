import {spawn,spawnSync} from "node:child_process";
import {mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {scaledMillisecondsV1} from "../../../../🐢️speed/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The numbers of the process-tree rules.
 * @see ./🧬️schema/🔣️.json */
export const READY_TREE_POLICY_V1=schema.definitions.Policy.const;
const P=READY_TREE_POLICY_V1;

/** 🧾️ One row of the process table: the pid, the pid of its parent, the start time in milliseconds since the epoch and, where the table says it, the name of the program. */
export type ReadyProcessV1=Readonly<{pid:number;ppid:number;startMs:number;name?:string}>;
/** 🪪️ A process named by pid and start time (and, once seen in the table, by the name of its program, so a report can name it): a reused pid has another start time and is another process. */
export type ReadyKnownV1=Readonly<{pid:number;startMs:number;name?:string}>;

const near=(left:number,right:number,tolerance:number)=>Math.abs(left-right)<=tolerance;

/** 🧭️ Whether the process `known` named is still in the table as the same process, gone, or its pid belongs to another process now. A start time of 0 in the record means the start was not known and only the pid counts. */
export function readyIdentityV1(known:ReadyKnownV1,table:readonly ReadyProcessV1[],tolerance:number=P.toleranceMilliseconds):"same"|"gone"|"reused"{
 const row=table.find(each=>each.pid===known.pid);
 if(row===undefined)return"gone";
 return known.startMs===0||near(row.startMs,known.startMs,tolerance)?"same":"reused";
}

/** 🌲️ The processes to stop, leaf first (`spare` and everything below it never: a service that other agents use stays up): the root (if it is still the same process), everything below it in the table, and every process recorded in an earlier sample that is still there with its recorded start time (the orphans whose parent has already exited) with everything below those. A child counts as below its parent only when it did not start before the parent (a reused parent pid is not a parent). Never this process or its ancestors, never the system pids. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyKillPlanV1(input:Readonly<{table:readonly ReadyProcessV1[];root:ReadyKnownV1;known:readonly ReadyKnownV1[];self:number;tolerance?:number;spare?:ReadonlySet<number>}>):readonly number[]{
 const tolerance=input.tolerance??P.toleranceMilliseconds,byPid=new Map(input.table.map(row=>[row.pid,row])),children=new Map<number,ReadyProcessV1[]>();
 for(const row of input.table){const parent=byPid.get(row.ppid);if(parent!==undefined&&row.startMs>=parent.startMs-tolerance&&row.pid!==row.ppid){const list=children.get(row.ppid)??[];list.push(row);children.set(row.ppid,list);}}
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
 return[...depthOf].filter(([pid])=>pid>P.reservedPids&&!protectedPids.has(pid)&&!spared.has(pid)).sort((a,b)=>b[1]-a[1]||a[0]-b[0]).map(([pid])=>pid);
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
 return {...policy,stallMilliseconds:scaledMillisecondsV1(policy.stallMilliseconds,factor),stallStopMilliseconds:scaledMillisecondsV1(policy.stallStopMilliseconds,factor),holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),staleStageMilliseconds:scaledMillisecondsV1(policy.staleStageMilliseconds,factor)};
}

/** 🔁️ What follows the `consecutive`-th stop in a row of one silent stage: a restart after an exponential back-off (`stallRestartBackoffMilliseconds` doubling up to `stallRestartBackoffMaximumMilliseconds`), and a failed state once more than `stallRestartMaximum` stops followed each other, so a stage that waits for something that never comes ends with its reason instead of being stopped every ten minutes for ever. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readyStallRestartV1(input:Readonly<{consecutive:number;policy?:typeof P}>):Readonly<{action:"restart"|"fail";waitMilliseconds:number}>{
 const policy=input.policy??P,count=Math.max(1,Math.floor(input.consecutive));
 if(count>policy.stallRestartMaximum)return{action:"fail",waitMilliseconds:0};
 return{action:"restart",waitMilliseconds:Math.min(policy.stallRestartBackoffMaximumMilliseconds,policy.stallRestartBackoffMilliseconds*2**(count-1))};
}

/** ⏳️ What to do with a stage past its soft cap: it runs while inside the cap; between the soft and the hard cap (`hardCapMultiplier` times the soft cap) it goes on, `extended`, as long as a counter of its log changed within the stall limit; otherwise it is stopped, `stalled` when the counters stood still, `timeout` at the hard cap. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyDeadlineV1(input:Readonly<{elapsedMs:number;softMs:number;sinceProgressMs:number;policy?:typeof P}>):"run"|"extended"|"timeout"|"stalled"{
 const policy=input.policy??P;
 if(input.elapsedMs<=input.softMs)return"run";
 if(input.elapsedMs>input.softMs*policy.hardCapMultiplier)return"timeout";
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

/** 🐢️ The stall tracker of one stage run: `observe` takes the last lines of the log and answers whether any of them is new, that is differs after normalisation from every line seen earlier in this run; the run made progress exactly then. A line that only changed its seconds counter is not new, one whose counter moved (`1957 of 2927`) is. At most `stallSeenMaximum` distinct lines are kept; a full memory is forgotten at once and counts as progress. Empty lines are no progress. A pure function of the lines seen so far.
 * @see ./🧪️tests/🐍️.py */
export function readyStallTrackerV1(policy:typeof P=P):Readonly<{observe(lines:readonly string[]):boolean}>{
 const seen=new Set<string>();
 return Object.freeze({observe(lines){
  let fresh=false;
  for(const line of lines){
   const key=readyNormaliseLineV1(line,policy.normalisePatterns);
   if(key===""||seen.has(key))continue;
   if(seen.size>=policy.stallSeenMaximum)seen.clear();
   seen.add(key);fresh=true;
  }
  return fresh;
 }});
}

/** 🛑️ Whether a stage whose output showed nothing new for `sinceProgressMs` is stopped to be tried again: at `stallStopMilliseconds` and later. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function readyStalledV1(sinceProgressMs:number,policy:typeof P=P):boolean{return sinceProgressMs>=policy.stallStopMilliseconds;}

/** 🪤️ The tracker of a wait on a named holder (a log line matching `holderWaitPatterns`): `observe` takes the last lines of the log and the clock and answers for how long the stage has waited for a holder without a round that showed none; a round without a holder line starts the count again.
 * @see ./🧪️tests/🐍️.py */
export function readyHolderWaitTrackerV1(policy:typeof P=P):Readonly<{observe(lines:readonly string[],nowMs:number):number}>{
 const expressions=policy.holderWaitPatterns.map(pattern=>new RegExp(pattern,"u"));let since:number|null=null;
 return Object.freeze({observe(lines,nowMs){
  if(!lines.some(line=>expressions.some(expression=>expression.test(line)))){since=null;return 0;}
  since??=nowMs;return nowMs-since;
 }});
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

/** 🧬️ The first file below `codeRoots` of `root` (relative path, extensions `codeExtensions`, no directory of `codeSkipDirectories`, no link) that was modified after `sinceMs`, `null` when none was: the code a stage loaded at its start has changed. Reads metadata only and stops at the first hit. */
export function readyCodeChangedSinceV1(root:string,sinceMs:number,policy:typeof P=P):string|null{
 const skip=new Set(policy.codeSkipDirectories),stack=[...policy.codeRoots];
 while(stack.length>0){
  const relative=stack.pop()!;let entries;
  try{entries=readdirSync(join(root,relative),{withFileTypes:true});}catch{continue;}
  for(const entry of entries){
   if(entry.isSymbolicLink())continue;
   const next=join(relative,entry.name);
   if(entry.isDirectory()){if(!skip.has(entry.name))stack.push(next);continue;}
   if(!entry.isFile()||!policy.codeExtensions.some(extension=>entry.name.endsWith(extension)))continue;
   try{if(statSync(join(root,next)).mtimeMs>sinceMs)return next;}catch{}
  }
 }
 return null;
}

/** 📜️ The process table from the text of `ps -A -o pid=,ppid=,etime=,comm=` (Linux and macOS): the start time is the moment of the call less the elapsed time (`[[dd-]hh:]mm:ss`), good to a second; the name is the last part of `comm`. */
export function readyParseUnixTableV1(text:string,nowMs:number):readonly ReadyProcessV1[]{
 const rows:ReadyProcessV1[]=[];
 for(const line of text.split("\n")){
  const found=/^\s*(\d+)\s+(\d+)\s+(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)(?:\s+(\S.*?))?\s*$/u.exec(line);
  if(found===null)continue;
  const seconds=((Number(found[3]??0)*24+Number(found[4]??0))*60+Number(found[5]))*60+Number(found[6]),name=found[7]===undefined?undefined:found[7].split("/").at(-1)!;
  rows.push({pid:Number(found[1]),ppid:Number(found[2]),startMs:nowMs-seconds*1000,...name===undefined||name===""?{}:{name}});
 }
 return rows;
}

/** 📜️ The process table from the lines `pid,ppid,startMs,name` that the PowerShell command of {@link readyTableCommandV1} prints on Windows. */
export function readyParseWindowsTableV1(text:string):readonly ReadyProcessV1[]{
 const rows:ReadyProcessV1[]=[];
 for(const line of text.split(/\r?\n/u)){const found=/^\s*(\d+),(\d+),(\d+)(?:,(.*?))?\s*$/u.exec(line);if(found!==null)rows.push({pid:Number(found[1]),ppid:Number(found[2]),startMs:Number(found[3]),...found[4]===undefined||found[4]===""?{}:{name:found[4]}});}
 return rows;
}

/** ⏱️ The elapsed time of `ps -o etime=` (`[[dd-]hh:]mm:ss`) in milliseconds, `null` when the text is not that. */
export function readyElapsedMillisecondsV1(text:string):number|null{
 const found=/^\s*(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)\s*$/u.exec(text);
 return found===null?null:((((Number(found[1]??0)*24+Number(found[2]??0))*60+Number(found[3]))*60+Number(found[4]))*1000);
}

/** 🕰️ The start time of one process, synchronously (a lease check asks for one process only): `ps -o etime=` on Linux and macOS, the creation date of the Win32 process on Windows; `null` when the process is not there or the command fails. */
export function readyProcessStartSyncV1(pid:number,platform:NodeJS.Platform=process.platform,now:()=>number=Date.now):number|null{
 try{
  const id=Math.trunc(pid);
  if(!Number.isInteger(id)||id<1)return null;
  if(platform==="win32"){
   const run=spawnSync("powershell.exe",["-NoProfile","-NonInteractive","-Command",`([DateTimeOffset](Get-CimInstance Win32_Process -Filter 'ProcessId=${id}').CreationDate).ToUnixTimeMilliseconds()`],{encoding:"utf8",windowsHide:true,timeout:15000}),value=Number(run.stdout.trim());
   return Number.isFinite(value)&&value>0?value:null;
  }
  const run=spawnSync("ps",["-o","etime=","-p",String(id)],{encoding:"utf8",timeout:5000}),elapsed=readyElapsedMillisecondsV1(run.stdout);
  return elapsed===null?null:now()-elapsed;
 }catch{return null;}
}

/** 🖥️ The command that prints the process table on this platform. */
export function readyTableCommandV1(platform:NodeJS.Platform=process.platform):Readonly<{command:string;args:readonly string[]}>{
 return platform==="win32"
  ?{command:"powershell.exe",args:["-NoProfile","-NonInteractive","-Command","Get-CimInstance Win32_Process | ForEach-Object { '{0},{1},{2},{3}' -f $_.ProcessId,$_.ParentProcessId,([DateTimeOffset]$_.CreationDate).ToUnixTimeMilliseconds(),$_.Name }"]}
  :{command:"ps",args:["-A","-o","pid=,ppid=,etime=,comm="]};
}

/** 📜️ Reads the process table of this machine; an empty list when the command fails or takes longer than `timeoutMilliseconds`. */
export function readyReadTableV1(timeoutMilliseconds:number=20000,platform:NodeJS.Platform=process.platform,now:()=>number=Date.now):Promise<readonly ReadyProcessV1[]>{
 return new Promise(done=>{
  const {command,args}=readyTableCommandV1(platform);
  let out="",finished=false;
  const end=(table:readonly ReadyProcessV1[])=>{if(!finished){finished=true;clearTimeout(timer);done(table);}};
  let child:ReturnType<typeof spawn>;
  try{child=spawn(command,[...args],{windowsHide:true,stdio:["ignore","pipe","ignore"]});}catch{return done([]);}
  const timer=setTimeout(()=>{try{child.kill();}catch{}end([]);},timeoutMilliseconds);
  child.stdout?.on("data",chunk=>{out+=String(chunk);});
  child.on("error",()=>end([]));
  child.on("close",()=>end(platform==="win32"?readyParseWindowsTableV1(out):readyParseUnixTableV1(out,now())));
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
