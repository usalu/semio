import {accessSync,constants,mkdirSync,readFileSync,readdirSync,rmdirSync,writeFileSync} from "node:fs";
import {dirname,join,posix} from "node:path";
import {JOB_LIMIT_RULES_V1,type JobBreachV1,type JobLimitPolicyV1,type JobLimiterV1,type LimitHandleV1} from "../🟦️.ts";

const MEGABYTE=1048576;

/** 🔌️ The file system the cgroup limiter touches, injected so tests run on a directory instead of the kernel. */
export type CgroupPortsV1=Readonly<{root:string;read:(path:string)=>string;write:(path:string,text:string)=>void;mkdir:(path:string)=>void;rmdir:(path:string)=>void;list:(path:string)=>readonly string[];writable:(path:string)=>boolean;own:()=>string;sleep:(milliseconds:number)=>Promise<void>;pid:number}>;

/** 🐧️ The real cgroup v2 file system. */
export function cgroupPortsV1(root:string="/sys/fs/cgroup"):CgroupPortsV1{
 return Object.freeze({root,read:path=>readFileSync(path,"utf8"),write:(path,text)=>writeFileSync(path,text),mkdir:path=>mkdirSync(path),rmdir:path=>rmdirSync(path),list:path=>readdirSync(path),writable:path=>{try{accessSync(path,constants.W_OK);return true;}catch{return false;}},own:()=>{const line=readFileSync("/proc/self/cgroup","utf8").split("\n").find(row=>row.startsWith("0::"));return line?.slice(3).trim()??"/";},sleep:milliseconds=>new Promise(done=>setTimeout(done,milliseconds)),pid:process.pid});
}

/** 🔎️ Parses a flat keyed cgroup file such as `memory.events`. */
export function parseCgroupKeysV1(text:string):Readonly<Record<string,number>>{
 return Object.freeze(Object.fromEntries(text.split("\n").filter(row=>row.trim()!=="").map(row=>{const [key,value]=row.trim().split(/\s+/u);return [key!,Number(value)];})));
}

/** 🧭️ Finds the delegated cgroup the limiter may create children in: the highest ancestor of this process' cgroup that the user may write and that has the memory controller enabled for its children; creates `semio-jobs` there with the controllers enabled and removes empty leftovers of crashed runs. Answers undefined where no cgroup is delegated (containers, no systemd user manager), so the caller falls back to the watchdog. */
export function detectCgroupV2V1(ports:CgroupPortsV1=cgroupPortsV1()):string|undefined{
 try{if(!parseController(ports.read(join(ports.root,"cgroup.controllers"))).has("memory"))return undefined;}catch{return undefined;}
 const segments=ports.own().split("/").filter(Boolean),name=JOB_LIMIT_RULES_V1.cgroupDirectoryName;
 for(let depth=1;depth<=segments.length;depth++){
  const candidate=join(ports.root,...segments.slice(0,depth)),base=join(candidate,name);
  try{
   if(!ports.writable(candidate)||!parseController(ports.read(join(candidate,"cgroup.subtree_control"))).has("memory"))continue;
   try{ports.mkdir(base);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
   for(const controller of JOB_LIMIT_RULES_V1.controllers)try{ports.write(join(base,"cgroup.subtree_control"),`+${controller}`);}catch(error){if(controller==="memory")throw error;}
   for(const entry of ports.list(base))if(entry.startsWith("job-"))try{ports.rmdir(join(base,entry));}catch{}
   return base;
  }catch{continue;}
 }
 return undefined;
}
const parseController=(text:string):Set<string>=>new Set(text.trim().split(/\s+/u).filter(Boolean));

let counter=0;

/** 🐧️ The cgroup v2 limiter: one child cgroup per job with `memory.max`, `memory.swap.max=0`, `memory.high` at the soft ratio for services only (a throttle would delay the kill of a job by seconds), `memory.oom.group=1` (the kernel kills the whole job together, nothing else), `pids.max` and `cpu.weight`; the kernel enforces the limit, this code only observes `memory.events` (`oom_kill`) and `memory.peak`, so the overrun is reported as a typed failure of this job. */
export function createCgroupLimiterV1(base:string,ports:CgroupPortsV1=cgroupPortsV1()):JobLimiterV1{
 return Object.freeze({enforcement:"cgroup-v2" as const,async create(policy:JobLimitPolicyV1,job:string):Promise<LimitHandleV1>{
  const directory=join(base,`job-${ports.pid}-${++counter}`),file=(name:string)=>join(directory,name),bytes=policy.memoryMegabytes*MEGABYTE;
  ports.mkdir(directory);
  try{
   ports.write(file("memory.max"),String(bytes));
   ports.write(file("memory.swap.max"),"0");
  }catch(error){try{ports.rmdir(directory);}catch{}throw error;}
  const optional=(name:string,text:string)=>{try{ports.write(file(name),text);}catch{}};
  if(policy.kind==="service")optional("memory.high",String(Math.floor(bytes*policy.softRatioBasisPoints/10000)));
  optional("memory.oom.group","1");
  if(policy.pids>0)optional("pids.max",String(policy.pids));
  optional("cpu.weight",String(Math.max(1,Math.min(10000,policy.cpuShares))));
  const callbacks:((breach:JobBreachV1)=>void)[]=[];
  let peak=0,current=0,kills=0,fired=false,timer:ReturnType<typeof setInterval>|undefined,released=false;
  const number=(name:string):number=>{try{return Number(ports.read(file(name)).trim());}catch{return Number.NaN;}};
  const observe=():void=>{
   const held=number("memory.current"),top=number("memory.peak");
   if(Number.isFinite(held))current=held;
   peak=Math.max(peak,Number.isFinite(top)?top:0,current);
   try{kills=Math.max(kills,parseCgroupKeysV1(ports.read(file("memory.events"))).oom_kill??0);}catch{}
   if(kills>0&&!fired){fired=true;const breach={peakMegabytes:Math.ceil(Math.max(peak,bytes)/MEGABYTE),limitMegabytes:policy.memoryMegabytes};for(const callback of callbacks)callback(breach);}
  };
  timer=setInterval(observe,JOB_LIMIT_RULES_V1.cgroupPollMilliseconds);timer.unref?.();
  return Object.freeze({enforcement:"cgroup-v2" as const,
   async attach(pid:number){try{ports.write(file("cgroup.procs"),String(pid));}catch(error){if((error as NodeJS.ErrnoException).code!=="ESRCH")throw error;}},
   onBreach(callback:(breach:JobBreachV1)=>void){callbacks.push(callback);if(fired)callback({peakMegabytes:Math.ceil(peak/MEGABYTE),limitMegabytes:policy.memoryMegabytes});},
   async usage(){observe();return{rssMegabytes:Math.ceil(current/MEGABYTE),peakMegabytes:Math.ceil(peak/MEGABYTE)};},
   async release(){
    if(released)return{peakMegabytes:Math.ceil(peak/MEGABYTE),breached:fired};
    released=true;if(timer!==undefined)clearInterval(timer);
    observe();
    try{ports.write(file("cgroup.kill"),"1");}catch{}
    for(let attempt=0;attempt<JOB_LIMIT_RULES_V1.graceMilliseconds/10;attempt++){
     try{ports.rmdir(directory);break;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")break;await ports.sleep(10);}
    }
    return{peakMegabytes:Math.ceil(peak/MEGABYTE),breached:fired||kills>0};
   }});
 }});
}

/** 🛰️ Moves a long lived service out of the job that started it: a service spawned by a scheduled job stays in the job's cgroup (detaching changes the session, never the cgroup), so its resident memory would be charged to the job and die with it. The service gets its own leaf cgroup `semio-service-<name>` next to `semio-jobs` with a hard limit, `memory.high` at the soft ratio so it sheds caches before the kill, no swap and a whole-group kill. Answers false where no cgroup is delegated; never throws.
 * @see ./🟦️.ts */
export function prepareServiceGroupV1(service:Readonly<{name:string;limitMegabytes:number;softRatioBasisPoints:number}>,ports:CgroupPortsV1=cgroupPortsV1()):string|undefined{
 try{
  const base=detectCgroupV2V1(ports);if(base===undefined)return undefined;
  const directory=join(dirname(base),`semio-service-${service.name}`),bytes=service.limitMegabytes*MEGABYTE;
  try{ports.mkdir(directory);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
  ports.write(join(directory,"memory.max"),String(bytes));
  for(const[name,text]of[["memory.swap.max","0"],["memory.high",String(Math.floor(bytes*service.softRatioBasisPoints/10000))],["memory.oom.group","1"]])try{ports.write(join(directory,name),text);}catch{}
  return directory;
 }catch{return undefined;}
}

/** 🚀️ The argument vector that starts `command` already inside the cgroup behind `procsFile`: a shell writes its own pid there and then replaces itself by the command, so the service is born in its own group and never runs a moment inside the job of the client that started it. A refused write still starts the service (the caller checks the placement). */
export function serviceLaunchArgumentsV1(procsFile:string,command:readonly string[]):readonly string[]{
 return["/bin/sh","-c",'echo $$ > "$0" 2>/dev/null; exec "$@"',procsFile,...command];
}

/** 🔎️ Whether a process runs in the cgroup directory `directory` (read from `/proc/<pid>/cgroup`). */
export function runsInServiceGroupV1(pid:number,directory:string,ports:CgroupPortsV1=cgroupPortsV1()):boolean{
 try{const line=readFileSync(`/proc/${pid}/cgroup`,"utf8").split("\n").find(row=>row.startsWith("0::"));return line!==undefined&&join(ports.root,line.slice(3).trim())===directory;}catch{return false;}
}

export function placeServiceOutsideJobsV1(pid:number,service:Readonly<{name:string;limitMegabytes:number;softRatioBasisPoints:number}>,ports:CgroupPortsV1=cgroupPortsV1()):boolean{
 try{
  const directory=prepareServiceGroupV1(service,ports);if(directory===undefined)return false;
  ports.write(join(directory,"cgroup.procs"),String(pid));
  return true;
 }catch{return false;}
}

/** 📍️ The leaf cgroup directory `placeServiceOutsideJobsV1` gives a service, or undefined where cgroup v2 is not usable. */
export function serviceGroupDirectoryV1(name:string,ports:CgroupPortsV1=cgroupPortsV1()):string|undefined{
 try{const base=detectCgroupV2V1(ports);return base===undefined?undefined:join(dirname(base),`semio-service-${name}`);}catch{return undefined;}
}

/** 🧹️ Leaves the service leaf cgroup this process runs in and removes it once empty, so a service that ends does not leave a cgroup behind. Only a group named `semio-service-*` is touched. */
export function releaseOwnServiceGroupV1(ports:CgroupPortsV1=cgroupPortsV1()):boolean{
 try{
  const segments=ports.own().split("/").filter(Boolean),leaf=segments.at(-1);
  if(leaf===undefined||!leaf.startsWith("semio-service-"))return false;
  const directory=join(ports.root,...segments),parent=join(ports.root,...segments.slice(0,-1));
  ports.write(join(parent,"cgroup.procs"),String(ports.pid));
  try{ports.rmdir(directory);}catch{return false;}
  return true;
 }catch{return false;}
}

/** 🧽️ Removes the empty `semio-service-<prefix>*` leaf cgroups beside `keep` (a group that still holds a process refuses to go): the leak of services that ended without leaving their group. Returns the removed names. */
export function sweepEmptyServiceGroupsV1(prefix:string,keep:string|undefined,ports:CgroupPortsV1=cgroupPortsV1()):readonly string[]{
 const removed:string[]=[];
 try{
  const base=detectCgroupV2V1(ports);if(base===undefined)return removed;
  const parent=dirname(base);
  for(const entry of ports.list(parent)){
   if(!entry.startsWith(`semio-service-${prefix}`)||join(parent,entry)===keep)continue;
   try{if(ports.read(join(parent,entry,"cgroup.procs")).trim()!=="")continue;ports.rmdir(join(parent,entry));removed.push(entry);}catch{}
  }
 }catch{}
 return Object.freeze(removed);
}

/** ♻️ The inactive file cache one cgroup holds (`inactive_file` of `memory.stat`): memory the kernel reclaims under pressure, so it is not growth; 0 where the file is unreadable. */
function reclaimableBytesV1(ports:CgroupPortsV1,directory:string):number{
 try{const row=ports.read(join(directory,"memory.stat")).split("\n").find(line=>line.startsWith("inactive_file "));const bytes=Number(row?.slice(14));return Number.isFinite(bytes)&&bytes>0?bytes:0;}catch{return 0;}
}

/** 📏️ The memory this process may still grow into: the smallest `memory.max` of its own cgroup and every ancestor (`max` means unlimited) and the bytes the cgroup holds now (`memory.current` less the inactive file cache of `memory.stat`, which the kernel drops before it kills anything). Answers undefined where cgroup v2 is not readable. */
export function cgroupMemoryHeadroomV1(ports:CgroupPortsV1=cgroupPortsV1()):Readonly<{limitBytes:number;currentBytes:number}>|undefined{
 try{
  const segments=ports.own().split("/").filter(Boolean);let limit=Infinity,current:number|undefined;
  for(let depth=segments.length;depth>=0;depth--){
   const directory=join(ports.root,...segments.slice(0,depth));
   try{const text=ports.read(join(directory,"memory.max")).trim();if(text!=="max")limit=Math.min(limit,Number(text));}catch{}
   if(current===undefined)try{current=Number(ports.read(join(directory,"memory.current")).trim())-reclaimableBytesV1(ports,directory);}catch{}
  }
  return current===undefined||!Number.isFinite(current)?undefined:Object.freeze({limitBytes:limit,currentBytes:current});
 }catch{return undefined;}
}

export {posix as cgroupPathV1};
