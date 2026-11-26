import {processPidExistsV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts"
import{spawnSync}from"node:child_process";
import{existsSync,readdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";

export type OwnerPoolLeaseV1=Readonly<{version:1;pid:number;startTime?:string;beatMs:number;waves:readonly string[]}>;
export type OwnerPoolRunnerV1=(command:string,args:readonly string[])=>Readonly<{status:number|null;stdout:string}>;
export type OwnerPoolChildVerdictV1="holds"|"parent-gone"|"lease-gone"|"lease-other"|"lease-stale"|"wave-superseded";
export type OwnerPoolOrphanV1=Readonly<{pid:number;startTime:string;parentPid?:number;parentStart?:string}>;

const native:OwnerPoolRunnerV1=(command,args)=>{const result=spawnSync(command,[...args],{encoding:"utf8",windowsHide:true,timeout:15000,env:{...process.env,LC_ALL:"C"}});return{status:result.error?null:result.status,stdout:result.stdout??""};};

/** 🕰️ The start time of a process as its operating system prints it (`ps -o lstart=` on macOS and Linux, `Process.StartTime` of PowerShell on Windows), `undefined` when the process is gone or the tool is not there: a pid together with its start time names one process for ever, a pid alone names whoever got it last.
 * @see ../🧪️tests/🪦️reaper/🟦️.ts */
export function ownerPoolProcessStartV1(pid:number,platform:NodeJS.Platform=process.platform,run:OwnerPoolRunnerV1=native):string|undefined{
 if(!Number.isInteger(pid)||pid<1)return undefined;
 const result=platform==="win32"?run("powershell.exe",["-NoLogo","-NoProfile","-NonInteractive","-Command",`(Get-Process -Id ${pid} -ErrorAction Stop).StartTime.ToUniversalTime().ToString('o')`]):run("ps",["-o","lstart=","-p",String(pid)]),text=result.stdout.trim().replace(/\s+/gu," ");
 return result.status===0&&text!==""?text:undefined;
}

/** 🫀️ Whether a process exists (`EPERM` means it does, owned by somebody else); works on every platform. */
export function ownerPoolAliveV1(pid:number):boolean{return processPidExistsV1(pid);}

/** 📝️ Writes the lease of a pool process (a temporary file and a rename, so a child never reads half of it). */
export function writeOwnerPoolLeaseV1(file:string,lease:OwnerPoolLeaseV1):void{const temporary=`${file}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(lease));renameSync(temporary,file);}

/** 📖️ Reads a lease, `undefined` when there is none or it is not one. */
export function readOwnerPoolLeaseV1(file:string):OwnerPoolLeaseV1|undefined{
 try{const lease=JSON.parse(readFileSync(file,"utf8")) as OwnerPoolLeaseV1;return lease.version===1&&Number.isInteger(lease.pid)&&Number.isFinite(lease.beatMs)&&Array.isArray(lease.waves)?lease:undefined;}catch{return undefined;}
}

/** 🐕️ Whether a child of the pool still has a pool to work for: its parent process lives (and on POSIX is still its parent: a dead parent hands its children to pid 1), the lease names that parent, was beaten within `staleMilliseconds` and lists the wave of the child. Anything else and the child must end: nobody reads what it publishes and it holds memory and cores the machine's own work needs. A pure function.
 * @see ../🧪️tests/🪦️reaper/🟦️.ts */
export function ownerPoolChildVerdictV1(input:Readonly<{parentPid:number;currentParent:number;alive:(pid:number)=>boolean;lease:OwnerPoolLeaseV1|undefined;wave:string;now:number;staleMilliseconds:number;platform?:NodeJS.Platform}>):OwnerPoolChildVerdictV1{
 if(!input.alive(input.parentPid)||(input.platform??process.platform)!=="win32"&&input.currentParent!==input.parentPid)return"parent-gone";
 const {lease}=input;
 if(lease===undefined)return"lease-gone";
 if(lease.pid!==input.parentPid)return"lease-other";
 if(input.now-lease.beatMs>input.staleMilliseconds)return"lease-stale";
 return lease.waves.includes(input.wave)?"holds":"wave-superseded";
}

/** 🪤️ What a watchdog does with a verdict: `parent-gone` (the pool process is dead or no longer the parent) and `lease-gone` (the pool ended in order and removed its lease, which it never lacks while it lives: the file is replaced atomically) end the child after `graceMilliseconds`; every other verdict (a lease that was not beaten, names another pool or lacks the wave) is a pool that is starved or busy while its process lives, so the child ends only after `lostGraceMilliseconds` of it without a single `holds` in between. `since` is when the verdict began, `undefined` while the child holds. A pure function.
 * @see ../🧪️tests/🪦️reaper/🟦️.ts */
export function ownerPoolChildFateV1(input:Readonly<{verdict:OwnerPoolChildVerdictV1;since:number|undefined;now:number;graceMilliseconds:number;lostGraceMilliseconds:number}>):Readonly<{since:number|undefined;end:boolean}>{
 if(input.verdict==="holds")return{since:undefined,end:false};
 const since=input.since??input.now;
 return{since,end:input.now-since>=(input.verdict==="parent-gone"||input.verdict==="lease-gone"?input.graceMilliseconds:input.lostGraceMilliseconds)};
}

/** 🧾️ The children of earlier pools that the files of this checkout name: the records `child-<n>.json` of the wave directories and the rows of the registry that carry a pid and the start time of the process, as pairs of the process and its pool. */
export function ownerPoolOrphanRecordsV1(kept:string,registry:string):readonly OwnerPoolOrphanV1[]{
 const rows=new Map<number,OwnerPoolOrphanV1>(),take=(file:string)=>{try{const row=JSON.parse(readFileSync(file,"utf8")) as Partial<OwnerPoolOrphanV1>;if(Number.isInteger(row.pid)&&row.pid!>0&&typeof row.startTime==="string"&&row.startTime!=="")rows.set(row.pid!,row as OwnerPoolOrphanV1);}catch{}};
 try{for(const wave of readdirSync(kept))if(!wave.includes("."))try{for(const name of readdirSync(join(kept,wave)))if(/^child-\d+\.json$/u.test(name))take(join(kept,wave,name));}catch{}}catch{}
 try{for(const name of readdirSync(registry))if(/^\d+\.json$/u.test(name))take(join(registry,name));}catch{}
 return[...rows.values()];
}

/** 🪦️ Ends the children of pools that are gone before the new pool measures what the machine has: a record whose process lives, whose start time is the recorded one (a reused pid never matches) and whose pool is neither this process nor a living process of the recorded start time is an orphan and is ended (`end`, forcibly: its claims run out by themselves); the answer is the pids ended once they are gone (waits at most `waitMilliseconds`). A tool that cannot read a start time ends nobody. */
export async function reapOwnerPoolOrphansV1(input:Readonly<{kept:string;registry:string;own:ReadonlySet<number>;self:number;alive?:(pid:number)=>boolean;startOf?:(pid:number)=>string|undefined;end?:(pid:number)=>void;waitMilliseconds?:number;pause?:(milliseconds:number)=>Promise<void>}>):Promise<readonly number[]>{
 const alive=input.alive??ownerPoolAliveV1,startOf=input.startOf??(pid=>ownerPoolProcessStartV1(pid)),end=input.end??(pid=>{try{process.kill(pid,"SIGKILL");}catch{}}),pause=input.pause??(milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds)));
 const ended:number[]=[];
 for(const row of ownerPoolOrphanRecordsV1(input.kept,input.registry)){
  if(input.own.has(row.pid)||row.pid===input.self||!alive(row.pid))continue;
  if(startOf(row.pid)!==row.startTime)continue;
  if(row.parentPid!==undefined&&row.parentPid!==input.self&&alive(row.parentPid)&&(row.parentStart===undefined||startOf(row.parentPid)===row.parentStart))continue;
  end(row.pid);ended.push(row.pid);
 }
 const until=Date.now()+(input.waitMilliseconds??P.reapWaitMilliseconds);
 while(ended.some(alive)&&Date.now()<until)await pause(50);
 for(const pid of ended)if(!alive(pid))try{rmSync(join(input.registry,pid+".json"),{force:true});}catch{}
 return ended.filter(pid=>!alive(pid));
}

/** 🧹️ Whether any file of the registry or the wave records still names a living process that is none of `own`: the machine's other children, which `ownerPoolTargetV1` must count as processes of the pool (their load is not "its own work") and never as capacity the pool already uses. */
export function ownerPoolSurvivorsV1(kept:string,registry:string,own:ReadonlySet<number>,alive:(pid:number)=>boolean=ownerPoolAliveV1):number{
 return existsSync(registry)||existsSync(kept)?ownerPoolOrphanRecordsV1(kept,registry).filter(row=>!own.has(row.pid)&&alive(row.pid)).length:0;
}
