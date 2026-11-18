/** 🧹 Processes a slice worker thread started belong to the thread, which cannot be told to stop them once it is terminated, so the pool tracks them itself: pids the worker reports, processes carrying the load's token in their environment (children inherit it; POSIX), and on Windows the new children of the service while no other load runs. A victim and all its descendants are interrupted, then killed after a grace period, as a process group where it leads one (POSIX) or with the system's process-tree termination (Windows). The token also lets a restarted service find what a dead service left behind.
 * @see ../🟨️.mjs */
import {execFileSync} from "node:child_process";
import {readFileSync,readdirSync} from "node:fs";

export const SLICE_JOB_VARIABLE_V1="SEMIO_SLICE_JOB";
export const SLICE_GRACE_MILLISECONDS_V1=2000;

/** 🏷 A unique token for one load of the service `pid`. */
export const sliceJobTokenV1=(pid,index,random=Math.random().toString(36).slice(2,10))=>pid+"."+index+"."+random;

const SERVICE=/(?:^|[\s\0])service[\s\0]host[\s\0]run(?:[\s\0]|$)/u,PATTERN=new RegExp("(?:^|[\\0 ])"+SLICE_JOB_VARIABLE_V1+"=([0-9]+\\.[0-9]+\\.[a-z0-9]+)","u");

/** 📋 Every process of this user as `{pid,ppid,token?}` (the token only where the platform shows environments). */
export function listProcessesV1(platform=process.platform){
 const rows=[];
 if(platform==="linux"){
  let names;try{names=readdirSync("/proc");}catch{return rows;}
  for(const name of names){
   if(!/^[0-9]+$/u.test(name))continue;
   try{
    const stat=readFileSync("/proc/"+name+"/stat","utf8"),fields=stat.slice(stat.lastIndexOf(")")+2).split(" "),ppid=Number(fields[1]);if(fields[0]==="Z")continue;
    let token,service=false;try{token=PATTERN.exec(readFileSync("/proc/"+name+"/environ","utf8"))?.[1];}catch{}try{service=SERVICE.test(readFileSync("/proc/"+name+"/cmdline","utf8"));}catch{}
    rows.push({pid:Number(name),ppid,...token===undefined?{}:{token},...service?{service}:{}});
   }catch{}
  }
 }else if(platform==="darwin"){
  try{for(const line of execFileSync("ps",["-Eww","-A","-o","pid=,ppid=,command="],{encoding:"utf8",maxBuffer:64*1024*1024}).split("\n")){const[pid,ppid]=line.trim().split(/\s+/u).map(Number),token=PATTERN.exec(line)?.[1],service=SERVICE.test(line.replace(/ [A-Z][A-Z0-9_]*=.*$/u,""));if(Number.isInteger(pid)&&Number.isInteger(ppid))rows.push({pid,ppid,...token===undefined?{}:{token},...service?{service}:{}});}}catch{}
 }else if(platform==="win32"){
  try{for(const line of execFileSync("powershell.exe",["-NoProfile","-NonInteractive","-Command","Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId) $($_.ParentProcessId)\" }"],{encoding:"utf8",windowsHide:true,maxBuffer:64*1024*1024}).split(/\r?\n/u)){const[pid,ppid]=line.trim().split(/\s+/u).map(Number);if(Number.isInteger(pid)&&Number.isInteger(ppid))rows.push({pid,ppid});}}catch{}
 }
 return rows;
}

/** 📨 The system's way to stop one process and what it started: a signal to the process group where it leads one and to the process itself (POSIX), `taskkill /T` with `/F` for a kill (Windows). */
export function signalProcessTreeV1(pid,signal,platform=process.platform){
 if(platform==="win32"){try{execFileSync("taskkill",["/PID",String(pid),"/T",...signal==="SIGKILL"?["/F"]:[]],{stdio:"ignore",windowsHide:true});}catch{}return;}
 for(const target of[-pid,pid])try{process.kill(target,signal);}catch{}
}

/** 🔭 `roots` and everything below them in `table`. */
export function withDescendantsV1(table,roots){
 const children=new Map();for(const row of table){const list=children.get(row.ppid)??[];list.push(row.pid);children.set(row.ppid,list);}
 const found=new Set(),pending=[...roots];
 while(pending.length){const pid=pending.pop();if(found.has(pid))continue;found.add(pid);pending.push(...children.get(pid)??[]);}
 return found;
}

/** 🧹 A reaper over a process table; a graph service or its launcher (`repo service host run` in its command line), other than the one this pool belongs to, and everything below it is never a victim, even when it was started by a command a slice worker ran and so carries the token of that load and sits in its process tree: a service outlives the load that happened to start it. `list` and `send` are injectable (tests, platforms). */
export function createReaperV1({list=listProcessesV1,send=signalProcessTreeV1,graceMilliseconds=SLICE_GRACE_MILLISECONDS_V1,alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}}}={}){
 const victims=(table,scope)=>{
  const present=new Set(table.map(row=>row.pid)),roots=new Set([...scope.pids??[]].filter(pid=>present.has(pid)&&pid!==process.pid));
  for(const row of table)if(row.pid!==process.pid&&scope.match?.(row))roots.add(row.pid);
  const parents=new Map(table.map(row=>[row.pid,row.ppid])),mine=new Set();
  for(let pid=process.pid;pid>1&&!mine.has(pid);pid=parents.get(pid))mine.add(pid);
  const foreign=withDescendantsV1(table,table.filter(row=>row.service===true&&!mine.has(row.pid)).map(row=>row.pid));
  return[...withDescendantsV1(table,roots)].filter(pid=>pid!==process.pid&&!foreign.has(pid));
 };
 return{
  victims:scope=>victims(list(),scope),
  /** 🔪 Interrupts the victims of `scope`, kills those left after the grace period; resolves how many there were. */
  async reap(scope,grace=graceMilliseconds){
   const first=victims(list(),scope);if(first.length===0)return 0;
   for(const pid of first)send(pid,"SIGINT");
   const end=performance.now()+grace;let rest=first.filter(alive);
   while(rest.length&&performance.now()<end){await new Promise(resolve=>setTimeout(resolve,Math.min(50,Math.max(1,grace))));rest=victims(list(),scope);}
   for(const pid of rest)send(pid,"SIGKILL");
   return first.length;
  },
  /** 🧨 Kills the victims of `scope` at once (the last act of an exiting service). */
  killNow(scope){for(const pid of victims(list(),scope))send(pid,"SIGKILL");},
 };
}

/** 🪦 Accepts the rows whose token names a service that is no longer alive. */
export const deadServiceRowV1=(alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}})=>row=>row.token!==undefined&&!alive(Number(row.token.split(".")[0]));

/** 📣 For worker code: tells the pool about a child it just started. */
export const reportChildV1=(port,child)=>{if(child.pid!==undefined)port.postMessage({type:"child",pid:child.pid});return child;};
