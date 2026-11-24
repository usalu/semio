import {spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {closeSync,linkSync,watch,type FSWatcher,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeSync} from "node:fs";
import {join} from "node:path";
import {filesystemBusyErrorV1,settleBusyFilesystemSyncV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {enterScriptProcessWaitV1} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
/** ⏳️ Cancellable pause that counts as a declared wait for the process watchdog: a process that sleeps here for a lane, a claim or a slot is waiting, not stalled (bounded by the watchdog's wait ceiling). */
export const sleepV1=async(milliseconds:number,signal?:AbortSignal):Promise<void>=>{const leave=enterScriptProcessWaitV1();try{await new Promise<void>((done,fail)=>{if(signal?.aborted){fail(signal.reason);return;}const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);fail(signal.reason);},{once:true});});}finally{leave();}};

/** 👂️ A waiter that sleeps on directory events instead of a poll: `wait` returns at once when a file was created or removed in one of the directories since the last `wait` (heartbeat touches are `change` events and ignored), else after `milliseconds` (a jittered fallback that reaps dead holders), and blocks without CPU in between. A directory that cannot be watched is covered by the fallback alone. Create it BEFORE inspecting the state, so no event between inspection and sleep is lost. */
export function watchChangesV1(directories:readonly string[]):Readonly<{wait(milliseconds:number,signal?:AbortSignal):Promise<void>;close():void}>{
 let pending=false,wake:(()=>void)|undefined;const watchers:FSWatcher[]=[];
 for(const directory of directories)try{const watcher=watch(directory,type=>{if(type!=="rename")return;pending=true;wake?.();});watcher.on("error",()=>undefined);watchers.push(watcher);}catch{}
 return Object.freeze({
  async wait(milliseconds,signal){
   if(pending){pending=false;return;}
   const leave=enterScriptProcessWaitV1();
   try{await new Promise<void>((done,fail)=>{
    if(signal?.aborted){fail(signal.reason);return;}
    const finish=()=>{clearTimeout(timer);signal?.removeEventListener("abort",abort);wake=undefined;pending=false;};
    const abort=()=>{finish();fail(signal!.reason);};
    const timer=setTimeout(()=>{finish();done();},Math.ceil(milliseconds*(0.8+Math.random()*0.4)));
    wake=()=>{finish();done();};signal?.addEventListener("abort",abort,{once:true});
   });}finally{leave();}
  },
  close(){for(const watcher of watchers)try{watcher.close();}catch{}},
 });
}

/** 🫀️ Whether a process id exists in the table, zombies included; a permission refusal still means it exists. */
export function processExistsV1(pid:number):boolean{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}}

/** 🧟️ Whether a process has exited but its parent has not reaped it yet: its id still answers `kill(pid,0)` although it can never release anything again. Linux reads the state of `/proc/<pid>/stat`, other POSIX hosts ask `ps -o stat=`, Windows has no zombies. */
export function processZombieV1(pid:number,world:Readonly<{platform?:string;state?:(pid:number)=>string|undefined}>={}):boolean{
 const platform=world.platform??process.platform;if(platform==="win32")return false;
 const state=world.state??(target=>{
  if(platform==="linux"){try{const text=readFileSync("/proc/"+String(target)+"/stat","utf8");return text.slice(text.lastIndexOf(")")+2).trim().charAt(0);}catch{return undefined;}}
  try{const run=spawnSync("ps",["-o","stat=","-p",String(target)],{encoding:"utf8",timeout:2000,windowsHide:true});return run.status===0?run.stdout.trim():undefined;}catch{return undefined;}
 });
 return(state(pid)??"").startsWith("Z");
}

/** 🫀️ Whether a process can still act: it exists and is not a zombie. */
export function processAliveV1(pid:number,world:Readonly<{platform?:string;state?:(pid:number)=>string|undefined}>={}):boolean{return processExistsV1(pid)&&!processZombieV1(pid,world);}

/** 🧹️ Removes one claim file and never throws: a Windows sharing violation (`EPERM`, `EBUSY`, `EACCES` while a reader, scanner or indexer holds the file) is retried on the churn table's pauses, and a file that stays busy is announced in every language and left to age out as stale; `false` then. */
export function removeClaimFileV1(path:string,world:Readonly<{platform?:string;sleepSync?:(milliseconds:number)=>void;report?:(text:string)=>void}>={}):boolean{
 return settleBusyFilesystemSyncV1(path,()=>rmSync(path,{force:true}),{report:text=>{try{(world.report??(line=>process.stderr.write(line+"\n")))(text);}catch{}},...world.platform?{platform:world.platform}:{},...world.sleepSync?{sleepSync:world.sleepSync}:{}}).settled;
}

type Holder=Readonly<{pid:number;token:string;atMs:number}>;
function holderOf(path:string):Holder|undefined{try{const row=JSON.parse(readFileSync(path,"utf8"));return typeof row?.pid==="number"?row:undefined;}catch{return undefined;}}
function ageOf(path:string,now:number):number|undefined{try{return now-statSync(path).mtimeMs;}catch{return undefined;}}

/** 🪪️ One held claim: a file created exclusively and kept fresh by its holder, released by removal. */
export type ClaimV1=Readonly<{path:string;waitedMs:number;release():void}>;

/** ⌛️ A claim is stale when its holder is gone or a zombie (exited, never reaped by its parent, so its id still answers and it would hold the claim and block every ticket behind it forever); a holder that is alive keeps it however old the file is, because a stalled event loop stops the heartbeat while the holder still works. The zombie check asks the platform only once the heartbeat is overdue, so a healthy holder costs nothing. A file without a readable holder ages out. */
function stale(path:string,staleMs:number,now:number):boolean{
 const age=ageOf(path,now);if(age===undefined)return false;
 const holder=holderOf(path);
 if(holder===undefined)return age>staleMs;
 return age>1000&&(!processExistsV1(holder.pid)||age>2*policy.lock.heartbeatMilliseconds&&processZombieV1(holder.pid));
}

function removeStale(path:string,staleMs:number):void{
 const grave=path+".dead-"+randomBytes(4).toString("hex");
 try{renameSync(path,grave);}catch{return;}
 if(!stale(grave,staleMs,Date.now())){try{linkSync(grave,path);}catch{}}
 removeClaimFileV1(grave);
}

/** 🔓️ Creates one claim file exclusively and returns its descriptor, `undefined` when the claim exists or stays busy: a Windows file that another process is deleting or renaming refuses the open with `EPERM`, `EACCES` or `EBUSY` instead of `EEXIST`, which is retried on the churn table's pauses and then counts as held, so the caller waits instead of failing. See https://learn.microsoft.com/windows/win32/fileio/deleting-an-open-file */
export function openClaimFileV1(path:string,world:Readonly<{platform?:string;sleepSync?:(milliseconds:number)=>void;open?:(path:string)=>number}>={}):number|undefined{
 const open=world.open??(target=>openSync(target,"wx"));
 const outcome=settleBusyFilesystemSyncV1(path,()=>open(path),{report:()=>undefined,...world.platform?{platform:world.platform}:{},...world.sleepSync?{sleepSync:world.sleepSync}:{}});
 if(outcome.settled)return outcome.value;
 const error=outcome.error as NodeJS.ErrnoException;
 if(error.code==="EEXIST"||filesystemBusyErrorV1(error,world.platform))return undefined;
 throw error;
}

function tryCreate(path:string,heartbeatMs:number):ClaimV1|undefined{
 const token=randomBytes(8).toString("hex");const fd=openClaimFileV1(path);if(fd===undefined)return undefined;
 try{writeSync(fd,JSON.stringify({pid:process.pid,token,atMs:Date.now()}));}finally{closeSync(fd);}
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(path,now,now);}catch{}},heartbeatMs);beat.unref?.();
 let released=false;
 return Object.freeze({path,waitedMs:0,release(){if(released)return;released=true;clearInterval(beat);if(holderOf(path)?.token===token)removeClaimFileV1(path);}});
}

/** 🚪️ Takes the claim only if it is free right now, replacing a dead or silent holder; never waits. */
export function tryClaimV1(directory:string,name:string):ClaimV1|undefined{
 mkdirSync(directory,{recursive:true});
 const path=join(directory,name+".lock");
 for(let attempt=0;attempt<2;attempt++){
  const claim=tryCreate(path,policy.lock.heartbeatMilliseconds);if(claim)return claim;
  if(!stale(path,policy.lock.staleMilliseconds,Date.now()))return undefined;
  removeStale(path,policy.lock.staleMilliseconds);
 }
 return undefined;
}

/** 👁️ Whether the claim `name` is held right now by a live, recently beating holder; reads only, never replaces a dead holder and never takes the claim. */
export function claimHeldV1(directory:string,name:string):boolean{
 const path=join(directory,name+".lock");
 return ageOf(path,Date.now())!==undefined&&!stale(path,policy.lock.staleMilliseconds,Date.now());
}

/** 🪪️ Who holds the claim `name` right now (pid and claim time), `undefined` when it is free or its holder is unreadable; reads only. */
export function claimHolderV1(directory:string,name:string):Readonly<{pid:number;atMs:number}>|undefined{
 const path=join(directory,name+".lock"),holder=holderOf(path);
 return holder===undefined||!claimHeldV1(directory,name)?undefined:Object.freeze({pid:holder.pid,atMs:holder.atMs});
}

/** 🔐️ Takes the per-unit claim named `name` inside `directory`. Waiting is bounded: `undefined` after the maximum wait means "compile anyway", so no wait chain can become a deadlock. A holder that died or stopped beating is replaced atomically. */
export async function acquireUnitClaimV1(directory:string,name:string,options:Readonly<{maximumWaitMs?:number;signal?:AbortSignal;onWait?:(waitedMs:number)=>void}>={}):Promise<ClaimV1|undefined>{
 mkdirSync(directory,{recursive:true});
 const path=join(directory,name+".lock"),started=Date.now(),limit=options.maximumWaitMs??policy.lock.maximumWaitMilliseconds,listener=watchChangesV1([directory]);
 let poll=policy.lock.pollMinimumMilliseconds,reported=false;
 try{for(;;){
  const claim=tryCreate(path,policy.lock.heartbeatMilliseconds);
  if(claim)return Object.freeze({...claim,waitedMs:Date.now()-started});
  if(stale(path,policy.lock.staleMilliseconds,Date.now())){removeStale(path,policy.lock.staleMilliseconds);continue;}
  const waited=Date.now()-started;
  if(waited>=limit)return undefined;
  if(!reported){reported=true;options.onWait?.(0);}
  await listener.wait(Math.min(poll,Math.max(1,limit-waited)),options.signal);
  poll=Math.min(policy.lock.pollMaximumMilliseconds,Math.ceil(poll*policy.lock.pollBackoffBasisPoints/10000));
 }}finally{listener.close();}
}

/** 🪶️ A light slot of a unit that only ships its inputs to a worker and waits for the answer: it holds no compile slot, so it is not charged against the local CPU and memory grant, and is bounded by its own count (`count` files `light-<n>.lock` beside the compile slots, the first free one wins, no ticket queue because a light slot is rarely contended). Waits while all are taken, so a unit never fails for want of one. */
export async function acquireLightSlotV1(root:string,count:number,options:Readonly<{signal?:AbortSignal;onWait?:(waitedMs:number)=>void}>={}):Promise<ClaimV1>{
 if(!Number.isSafeInteger(count)||count<1)throw Error("Positive light slot count required");
 const slots=join(root,policy.layout.slots);mkdirSync(slots,{recursive:true});
 const started=Date.now(),listener=watchChangesV1([slots]);let reported=false;
 try{
  for(;;){
   const now=Date.now();
   for(let index=0;index<count;index++){
    const path=join(slots,"light-"+String(index)+".lock");
    if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);
    const claim=tryCreate(path,policy.slot.heartbeatMilliseconds);
    if(claim)return Object.freeze({...claim,waitedMs:Date.now()-started});
   }
   if(!reported&&Date.now()-started>200){reported=true;options.onWait?.(Date.now()-started);}
   await listener.wait(policy.slot.fallbackMilliseconds,options.signal);
  }
 }finally{listener.close();}
}

/** 🔢️ How many light slots (`acquireLightSlotV1`) live units hold right now: counted from the claim files, reads only; the number of units that are running on a worker for this machine. */
export function lightSlotsHeldV1(root:string,count:number):number{
 const slots=join(root,policy.layout.slots);let held=0;
 for(let index=0;index<count;index++)if(claimHeldV1(slots,"light-"+String(index)))held++;
 return held;
}

/** 🥇️ Takes a compile slot at once when one is free and nobody waits for it, else answers `undefined` without joining the queue: the local-first placement of a unit (a unit that finds a free slot compiles here, only a unit that would wait may go to the fleet). The oldest live ticket keeps its turn, so a free slot never overtakes a waiter; the claim itself is atomic (`tryCreate`), so concurrent callers never share one slot. */
export function tryAcquireSlotV1(root:string,count:number):ClaimV1|undefined{
 if(!Number.isSafeInteger(count)||count<1)return undefined;
 const slots=join(root,policy.layout.slots),tickets=join(root,policy.layout.tickets),now=Date.now();
 try{
  mkdirSync(slots,{recursive:true});
  for(const entry of readdirSync(tickets)){const path=join(tickets,entry);if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);else return undefined;}
 }catch{}
 for(let index=0;index<count;index++){
  const path=join(slots,String(index)+".lock");
  if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);
  const claim=tryCreate(path,policy.slot.heartbeatMilliseconds);
  if(claim)return Object.freeze({...claim,waitedMs:0});
 }
 return undefined;
}

/** 🎰️ Host-wide compile slots. Waiters take a numbered ticket and only the oldest live ticket may claim a free slot, so grants are first-in-first-out and no waiter starves; tickets and slots of dead processes are reaped. */
export async function acquireSlotV1(root:string,count:number,options:Readonly<{signal?:AbortSignal;onWait?:(waitedMs:number)=>void;since?:number}>={}):Promise<ClaimV1>{
 if(!Number.isSafeInteger(count)||count<1)throw Error("Positive compile slot count required");
 const slots=join(root,policy.layout.slots),tickets=join(root,policy.layout.tickets);mkdirSync(slots,{recursive:true});mkdirSync(tickets,{recursive:true});
 const started=Date.now(),name=String(Math.min(started,options.since??started)).padStart(15,"0")+"-"+randomBytes(4).toString("hex")+"-"+process.pid,ticket=join(tickets,name);
 const writeTicket=()=>{const fd=openSync(ticket,"w");try{writeSync(fd,JSON.stringify({pid:process.pid,token:name,atMs:started}));}finally{closeSync(fd);}};
 const listener=watchChangesV1([slots,tickets]);writeTicket();
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(ticket,now,now);}catch{}},policy.slot.heartbeatMilliseconds);beat.unref?.();
 let reported=false;
 try{
  for(;;){
   const now=Date.now(),live:string[]=[];
   for(const entry of readdirSync(tickets).sort()){const path=join(tickets,entry);if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);else live.push(entry);}
   if(live[0]===name){
    for(let index=0;index<count;index++){
     const path=join(slots,String(index)+".lock");
     if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);
     const claim=tryCreate(path,policy.slot.heartbeatMilliseconds);
     if(claim){clearInterval(beat);removeClaimFileV1(ticket);return Object.freeze({...claim,waitedMs:Date.now()-started});}
    }
   }
   if(!reported&&Date.now()-started>200){reported=true;options.onWait?.(Date.now()-started);}
   await listener.wait(policy.slot.fallbackMilliseconds,options.signal);
  }
 }catch(error){clearInterval(beat);removeClaimFileV1(ticket);throw error;}finally{listener.close();}
}
