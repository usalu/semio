import {execFile} from "node:child_process";
import {readFileSync,readdirSync} from "node:fs";
import {JOB_LIMIT_RULES_V1,killJobTreeV1,type JobBreachV1,type JobLimitPolicyV1,type JobLimiterV1,type LimitHandleV1} from "../🟦️.ts";

/** 🧾️ One process of the process table: its id, its parent and the resident kilobytes it holds. */
export type ProcessRowV1=Readonly<{pid:number;ppid:number;kilobytes:number}>;

/** 🌳️ The megabytes the tree below a root process holds; pure over the table, so every platform reader shares it. */
export function treeMegabytesV1(rows:readonly ProcessRowV1[],root:number):number{
 const children=new Map<number,ProcessRowV1[]>(),byPid=new Map<number,ProcessRowV1>();
 for(const row of rows){byPid.set(row.pid,row);const list=children.get(row.ppid);if(list)list.push(row);else children.set(row.ppid,[row]);}
 let kilobytes=0;const seen=new Set<number>(),stack=[root];
 while(stack.length>0){const pid=stack.pop()!;if(seen.has(pid))continue;seen.add(pid);kilobytes+=byPid.get(pid)?.kilobytes??0;for(const child of children.get(pid)??[])stack.push(child.pid);}
 return Math.ceil(kilobytes/1024);
}

/** 🔎️ Parses `pid ppid kilobytes` lines (macOS `ps`, the Windows process query). */
export function parseProcessRowsV1(text:string):readonly ProcessRowV1[]{
 return text.split("\n").flatMap(line=>{const [pid,ppid,kilobytes]=line.trim().split(/\s+/u).map(Number);return Number.isInteger(pid)&&Number.isInteger(ppid)&&Number.isFinite(kilobytes)?[{pid:pid!,ppid:ppid!,kilobytes:kilobytes!}]:[];});
}

/** 🐧️ Reads the process table from `/proc`: the parent from `stat`, the resident kilobytes from `status`. */
export function readProcTableV1(root:string="/proc"):readonly ProcessRowV1[]{
 const rows:ProcessRowV1[]=[];
 for(const name of readdirSync(root)){
  if(!/^\d+$/u.test(name))continue;
  try{
   const stat=readFileSync(`${root}/${name}/stat`,"utf8"),after=stat.slice(stat.lastIndexOf(")")+2).split(" "),status=readFileSync(`${root}/${name}/status`,"utf8"),match=/^VmRSS:\s+(\d+)\s+kB/mu.exec(status);
   rows.push({pid:Number(name),ppid:Number(after[1]),kilobytes:match?Number(match[1]):0});
  }catch{}
 }
 return rows;
}

const run=(command:string,args:readonly string[]):Promise<string>=>new Promise(done=>execFile(command,[...args],{maxBuffer:64*1048576,windowsHide:true},(error,stdout)=>done(error?"":stdout)));

/** 🧭️ The platform reader of the process table: `/proc` on Linux, `ps` on macOS, a CIM query on Windows. */
export async function readProcessTableV1(platform:NodeJS.Platform=process.platform):Promise<readonly ProcessRowV1[]>{
 if(platform==="linux")return readProcTableV1();
 if(platform==="win32")return parseProcessRowsV1(await run("powershell.exe",["-NoProfile","-NonInteractive","-Command","Get-CimInstance Win32_Process|ForEach-Object{'{0} {1} {2}' -f $_.ProcessId,$_.ParentProcessId,[int64]($_.WorkingSetSize/1024)}"]));
 return parseProcessRowsV1(await run("ps",["-A","-o","pid=,ppid=,rss="]));
}

/** 🔌️ What the watchdog needs from the host. */
export type WatchdogPortsV1=Readonly<{table:()=>Promise<readonly ProcessRowV1[]>|readonly ProcessRowV1[];kill:(pid:number)=>void;pollMilliseconds?:number}>;

/** 🐕️ The cooperative fallback (macOS, Linux without a delegated cgroup, Windows without the helper): polls the resident memory of the job's process tree and kills the tree the moment it reaches the limit, then reports the breach; it cannot stop a spike faster than its poll interval, so the limit is enforced after the fact by at most one interval. */
export function createWatchdogLimiterV1(ports:WatchdogPortsV1={table:()=>readProcessTableV1(),kill:killJobTreeV1}):JobLimiterV1{
 return Object.freeze({enforcement:"watchdog" as const,async create(policy:JobLimitPolicyV1):Promise<LimitHandleV1>{
  const callbacks:((breach:JobBreachV1)=>void)[]=[];
  let root:number|undefined,peak=0,current=0,fired=false,busy=false,timer:ReturnType<typeof setInterval>|undefined,stopped=false;
  const sample=async():Promise<void>=>{
   if(root===undefined||busy||stopped)return;
   busy=true;
   try{
    current=treeMegabytesV1(await ports.table(),root);peak=Math.max(peak,current);
    if(current>=policy.memoryMegabytes&&!fired&&!stopped){fired=true;ports.kill(root);const breach={peakMegabytes:peak,limitMegabytes:policy.memoryMegabytes};for(const callback of callbacks)callback(breach);}
   }finally{busy=false;}
  };
  return Object.freeze({enforcement:"watchdog" as const,
   async attach(pid:number){root=pid;timer=setInterval(()=>{void sample();},ports.pollMilliseconds??JOB_LIMIT_RULES_V1.watchdogPollMilliseconds);timer.unref?.();await sample();},
   onBreach(callback:(breach:JobBreachV1)=>void){callbacks.push(callback);},
   async usage(){return{rssMegabytes:current,peakMegabytes:peak};},
   async release(){stopped=true;if(timer!==undefined)clearInterval(timer);return{peakMegabytes:peak,breached:fired};}});
 }});
}
