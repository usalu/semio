import {appendFileSync,existsSync,mkdirSync,readFileSync,renameSync,rmdirSync,statSync,writeFileSync} from "node:fs";
import {homedir,totalmem} from "node:os";
import {hostAvailableBytesV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {isAbsolute,join} from "node:path";
import {hostFloorMegabytesV1,readHostCapacityPolicyV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import buildPolicy from "../../🔣️policy.json" with {type:"json"};
import {applySchedulerEventV1,decideTickV1,emptySchedulerStateV1,schedulerSnapshotV1,schedulerTextV1,type SchedulerEventV1,type SchedulerHostV1,type SchedulerPolicyV1,type SchedulerSnapshotV1,type SchedulerStateV1} from "../🟦️.ts";

/** 💾 Single-writer event store of the scheduler on the local disk: an atomic `mkdir` lock, a snapshot plus an append-only NDJSON log with sequence numbers (a crash between snapshot and log truncation replays nothing twice), lock-free reads. The host daemon hosts the same core in memory later; clients only see `SchedulerStoreV1`.
 * @see ../🧬️schema/🔣️.json the closed event schema */
export type SchedulerTransactionV1<T>=Readonly<{events:readonly SchedulerEventV1[];result:T}>;
export type SchedulerStoreV1=Readonly<{
 policy:SchedulerPolicyV1;
 transact<T>(body:(state:SchedulerStateV1,at:number)=>SchedulerTransactionV1<T>):Promise<T>;
 snapshot():Promise<SchedulerSnapshotV1>;
 now():number;
}>;
export type SchedulerStorePortsV1=Readonly<{now?:()=>number;host?:()=>SchedulerHostV1|Promise<SchedulerHostV1>;onEvents?:(events:readonly SchedulerEventV1[],state:SchedulerStateV1)=>void;sleep?:(milliseconds:number)=>Promise<void>}>;

/** 🔐️ Whether a failed lock `mkdir` means another writer holds or is just releasing the lock: `EEXIST` everywhere, and on Windows `EPERM`/`EACCES` while the holder's `rmdir` is still delete-pending.
 * @see https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-removedirectoryw */
export function schedulerLockContendedV1(code:string|undefined,platform:NodeJS.Platform=process.platform):boolean{
 return code==="EEXIST"||platform==="win32"&&(code==="EPERM"||code==="EACCES");
}

/** 📍 Host-wide scheduler directory: the platform cache folder shared by every workspace and agent of the user. */
export function schedulerDirectoryV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment.SEMIO_SCHEDULER_DIRECTORY;if(override)return override;
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache");
 return join(base,"semio","scheduler");
}

/** 🌡️ Job budget of the host: the memory the host-capacity gate may give out at all (the memory above its free-memory floor, one rule for both gates), and the memory that is really available now.
 * @see ../../🌡️host-capacity/🟦️.ts */
export function defaultSchedulerHostV1():SchedulerHostV1{
 const totalMegabytes=Math.floor(totalmem()/2**20),available=hostAvailableBytesV1(),availableMegabytes=available===undefined?totalMegabytes:Math.floor(available/2**20);
 return{budgetMegabytes:Math.max(1,totalMegabytes-hostFloorMegabytesV1(readHostCapacityPolicyV1(buildPolicy.hostCapacity),{totalMegabytes})),availableMegabytes};
}

type Persisted={n:number;state:SchedulerStateV1};
const pause=(milliseconds:number):Promise<void>=>new Promise(resolve=>setTimeout(resolve,milliseconds));

/** 🏗️ Opens (creating when needed) the store in `directory`; `transact` runs due expiries, the body, then admission, and appends the events of all three under the lock. */
export function createFileSchedulerStoreV1(directory:string,policy:SchedulerPolicyV1,ports:SchedulerStorePortsV1={}):SchedulerStoreV1{
 const now=ports.now??Date.now,sleep=ports.sleep??pause,host=ports.host??defaultSchedulerHostV1,lock=join(directory,"lock"),snapshotFile=join(directory,"state.json"),logFile=join(directory,"log.ndjson");
 const readSnapshot=():Persisted=>existsSync(snapshotFile)?JSON.parse(readFileSync(snapshotFile,"utf8")) as Persisted:{n:0,state:emptySchedulerStateV1()};
 const readLog=():{n:number;event:SchedulerEventV1}[]=>{
  if(!existsSync(logFile))return[];
  const rows:{n:number;event:SchedulerEventV1}[]=[];
  for(const line of readFileSync(logFile,"utf8").split("\n")){if(!line)continue;try{const row=JSON.parse(line) as {n:number;e:SchedulerEventV1};rows.push({n:row.n,event:row.e});}catch{break;}}
  return rows;
 };
 const load=():{persisted:Persisted;rows:number}=>{
  for(;;){
   const first=readSnapshot(),rows=readLog().filter(row=>row.n>first.n);
   if(readSnapshot().n!==first.n)continue;
   const state=first.state;let n=first.n;for(const row of rows){applySchedulerEventV1(policy,state,row.event);n=row.n;}
   return{persisted:{n,state},rows:rows.length};
  }
 };
 const acquire=async():Promise<void>=>{
  const deadline=now()+policy.lockStaleMs*2;
  for(;;){
   try{mkdirSync(directory,{recursive:true});mkdirSync(lock);return;}catch(error){
    if(!schedulerLockContendedV1((error as NodeJS.ErrnoException).code))throw error;
    try{if(Date.now()-statSync(lock).mtimeMs>policy.lockStaleMs)rmdirSync(lock);}catch{}
    if(now()>deadline)throw Error(schedulerTextV1("en","error.lock",{seconds:Math.floor(policy.lockStaleMs*2/1000)})+"\n"+schedulerTextV1("de","error.lock",{seconds:Math.floor(policy.lockStaleMs*2/1000)}));
    await sleep(5+Math.floor(Math.random()*20));
   }
  }
 };
 const compact=(persisted:Persisted):void=>{const temporary=`${snapshotFile}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(persisted));renameSync(temporary,snapshotFile);writeFileSync(logFile,"");};
 return{
  policy,now,
  async transact<T>(body:(state:SchedulerStateV1,at:number)=>SchedulerTransactionV1<T>):Promise<T>{
   await acquire();
   try{
    const{persisted,rows}=load(),state=persisted.state,at=now(),budget=await host(),written:SchedulerEventV1[]=[];
    let n=persisted.n;
    const record=(events:readonly SchedulerEventV1[]):void=>{for(const event of events){applySchedulerEventV1(policy,state,event);written.push(event);}};
    record(decideTickV1(policy,state,at,budget));
    const outcome=body(state,at);record(outcome.events);record(decideTickV1(policy,state,at,budget));
    if(written.length>0){appendFileSync(logFile,written.map(event=>JSON.stringify({n:++n,e:event})).join("\n")+"\n");ports.onEvents?.(written,state);}
    if(rows+written.length>=policy.compactEveryEvents)compact({n,state});
    return outcome.result;
   }finally{try{rmdirSync(lock);}catch{}}
  },
  async snapshot():Promise<SchedulerSnapshotV1>{const at=now();return schedulerSnapshotV1(policy,load().persisted.state,at,await host());},
 };
}
