import {spawn,type ChildProcess} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {closeSync,existsSync,mkdirSync,openSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {readdir,readFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {processHolderAliveV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

/** 🧷️ One child the replica daemon supervises, as the topology declares it: a script below the worktree with its arguments, the machines it runs on (`member`: every machine, `server`: the group server only) and what it needs (`platform-peer`: at least one other machine of the same platform has attested). */
export type SidecarChildV1=Readonly<{name:string;script:string;arguments:readonly string[];roles:readonly ("member"|"server")[];requires:"none"|"platform-peer";enabled:boolean}>;

/** 🎚️ The `sidecars` setting of the topology: the master switch, how long a healthy daemon waits before starting children, the restart back-off (first delay, ceiling, and the run time after which the count of failures starts again), the grace a stopping child gets before it is killed, and the children. */
export type SidecarSettingsV1=Readonly<{enabled:boolean;startAfterSeconds:number;restart:Readonly<{initialMilliseconds:number;maximumMilliseconds:number;stableMilliseconds:number}>;graceSeconds:number;children:readonly SidecarChildV1[]}>;

/** 🩺️ What `health.json` reports about one child: `off` (not wanted here), `waiting` (wanted, not started yet), `running`, `backoff` (ended, restart scheduled), `stopped` (stopped with the daemon). */
export type SidecarStateV1=Readonly<{name:string;state:"off"|"waiting"|"running"|"backoff"|"stopped";pid:number|null;restarts:number;sinceMs:number;lastExit:string|null;nextStartMs:number|null}>;

/** 🛰️ The facts the daemon knows about its machine. */
export type SidecarFactsV1=Readonly<{worktree:string;storeRoot:string;originId:string;groupId:string;platform:"linux"|"macos"|"windows"}>;

/** 🔌️ The running supervisor: `tick` is called by the daemon on every round and never throws or blocks, `report` is what `health.json` carries, `stop` ends every child (SIGTERM, kill after the grace) and resolves when they are gone, `release` lets go of them without ending them so that the next daemon adopts them (the daemon restarts for new code or is ended by its watchdog). */
export type SidecarsV1=Readonly<{tick:(healthy:boolean)=>void;report:()=>readonly SidecarStateV1[];stop:()=>Promise<void>;release:()=>void}>;

/** 🪪️ What the launcher of a sidecar keeps fresh in `.cache/f/sidecars/<name>.json` (see `🧬️schema/🔣️.json`): who runs, since when, when it last wrote, and the hash of the entry script it runs. */
export type SidecarRecordV1=Readonly<{version:1;name:string;pid:number;startMs:number;startedMs:number;wallMs:number;entrySha256:string}>;

/** 🎫️ What the daemon keeps fresh in `.cache/f/sidecars/<name>.lease` while it owns the sidecar. */
export type SidecarLeaseV1=Readonly<{version:1;pid:number;wallMs:number}>;

/** ⏳️ The times of the adoption protocol in milliseconds: the daemon renews the lease every 5 s, the launcher writes its record every 2 s, a record counts as alive for 15 s and a launcher without a renewed lease ends itself after 2 h (a daemon that restarts within that time adopts the sidecar instead of starting another). */
export const sidecarProtocolV1=Object.freeze({renewMilliseconds:5_000,recordMilliseconds:2_000,aliveMilliseconds:15_000,leaseMilliseconds:7_200_000});

const validators=new Map<string,CompiledJsonSchemaSubset>();
const requireShape=(definition:"Record"|"Lease",value:unknown):void=>{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`Closed sidecar ${definition.toLowerCase()} required: ${errors[0]}`);
};

/** 📍️ The files of the adoption protocol of one sidecar below the worktree. */
export function sidecarFilesV1(worktree:string,name:string):Readonly<{record:string;lease:string}>{
 const directory=join(worktree,".cache","f","sidecars");
 return {record:join(directory,`${name}.json`),lease:join(directory,`${name}.lease`)};
}

/** #️⃣ The sha256 of an entry script, the identity of the code a launcher was started from; `undefined` when the file cannot be read. */
export function sidecarEntryHashV1(file:string):string|undefined{
 try{return createHash("sha256").update(readFileSync(file)).digest("hex");}catch{return undefined;}
}

function writeAtomicV1(file:string,text:string):void{
 mkdirSync(dirname(file),{recursive:true});
 const temporary=`${file}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`;
 try{writeFileSync(temporary,text);renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}
}

/** ✍️ Replaces the record of a sidecar (tmp + rename), validated against the schema. */
export function writeSidecarRecordV1(file:string,record:SidecarRecordV1):void{requireShape("Record",record);writeAtomicV1(file,JSON.stringify(record));}

/** 📖️ Reads a record; `undefined` when there is none or it is not closed. */
export function readSidecarRecordV1(file:string):SidecarRecordV1|undefined{
 try{const value=JSON.parse(readFileSync(file,"utf8")) as unknown;requireShape("Record",value);return value as SidecarRecordV1;}catch{return undefined;}
}

/** 🎫️ The lease file of one launcher: the base path of the record with the launcher's own pid (`<name>.<pid>.lease`), so a launcher that a newer one replaced never has its lease renewed and leaves by itself. */
export function sidecarLeaseOfV1(base:string,pid:number):string{return base.endsWith(".lease")?`${base.slice(0,-6)}.${pid}.lease`:`${base}.${pid}`;}

/** ✍️ Renews the lease of a sidecar for the process `pid`. */
export function writeSidecarLeaseV1(file:string,pid:number,now:number):void{const lease:SidecarLeaseV1={version:1,pid,wallMs:now};requireShape("Lease",lease);writeAtomicV1(file,JSON.stringify(lease));}

/** ⌛️ Whether the lease of a sidecar ran out: the file is missing or unreadable and `sinceMs` (the start of the asking launcher) is more than `expireMilliseconds` ago, or its last renewal is. A clock that moved back does not expire a lease. */
export function sidecarLeaseExpiredV1(file:string,now:number,expireMilliseconds:number,sinceMs:number):boolean{
 try{const value=JSON.parse(readFileSync(file,"utf8")) as unknown;requireShape("Lease",value);return now-(value as SidecarLeaseV1).wallMs>expireMilliseconds;}
 catch{return now-sinceMs>expireMilliseconds;}
}

/** 🫀️ Whether the launcher named by a record still lives (pid with creation time; a reused pid is another process). */
export function sidecarRecordLivesV1(record:Readonly<{pid:number;startMs:number}>):boolean{return processHolderAliveV1(record);}

/** ⏱️ The restart delay after `failures` consecutive failed runs: the first delay doubled per failure up to the ceiling. */
export function sidecarBackoffMillisecondsV1(restart:SidecarSettingsV1["restart"],failures:number):number{
 return Math.min(restart.maximumMilliseconds,restart.initialMilliseconds*2**Math.max(0,Math.min(failures-1,30)));
}

/** 🎯️ Which children a machine wants: enabled ones whose role matches the machine, that the `SEMIO_FLEET_SIDECARS` override names (`off` switches all off, a comma list keeps only those names), and (for `platform-peer`) only when `peers` other machines of its platform exist. */
export function wantedSidecarsV1(settings:SidecarSettingsV1,server:boolean,peers:number,override?:string):readonly SidecarChildV1[]{
 if(!settings.enabled||override==="off")return [];
 const names=override===undefined||override===""?undefined:new Set(override.split(",").map(name=>name.trim()));
 return settings.children.filter(child=>child.enabled&&child.roles.includes(server?"server":"member")&&(child.requires==="none"||peers>0)&&(names===undefined||names.has(child.name)));
}

/** 👥️ The number of other machines of the same platform that attested within ten minutes, read from the store (`origins/*\/attest.json`); a store that does not answer within `timeoutMilliseconds` counts as unknown (`undefined`). */
export async function countPlatformPeersV1(facts:Pick<SidecarFactsV1,"storeRoot"|"originId"|"platform">,nowMs:number,timeoutMilliseconds=5000):Promise<number|undefined>{
 const work=(async()=>{
  let count=0;
  for(const origin of await readdir(join(facts.storeRoot,"v1","origins"))){
   if(origin===facts.originId)continue;
   try{const value=JSON.parse(await readFile(join(facts.storeRoot,"v1","origins",origin,"attest.json"),"utf8")) as {platform?:unknown;wallMs?:unknown};if(value.platform===facts.platform&&typeof value.wallMs==="number"&&nowMs-value.wallMs<600_000)count++;}catch{}
  }
  return count;
 })();
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([work,new Promise<undefined>(done=>{timer=setTimeout(()=>{done(undefined);},timeoutMilliseconds);})]);}catch{return undefined;}finally{clearTimeout(timer);}
}

type Slot={spec:SidecarChildV1;state:SidecarStateV1["state"];proc?:ChildProcess;adopted?:ReturnType<typeof setInterval>;pid:number|null;startedMs:number;sinceMs:number;failures:number;restarts:number;lastExit:string|null;nextStartMs:number|null;ending?:Promise<void>;halt?:SidecarStateV1["state"]};

/** 🚀️ Starts the supervisor of the daemon. Children are crash-isolated from the daemon (own processes, their failures only schedule a restart with back-off and are visible in `report`), start only once `tick(true)` says the daemon is healthy and `startAfterSeconds` have passed, follow the wanted set as the number of platform peers changes, write their output to `.cache/f/sidecars/<name>.log` (rotated at 2 MiB, when a child starts) and run detached, owned by a lease file (`SEMIO_FLEET_SIDECAR_LEASE`) that the daemon renews every 5 s: a child outlives the daemon, so a daemon that restarts for whatever reason (code switch, killed by its watchdog, crash) *adopts* a child whose record (`SEMIO_FLEET_SIDECAR_RECORD`, written by its launcher) is alive and whose entry script is unchanged, instead of stopping and starting it; a child whose entry script changed is ended and started again, and a child whose lease nobody renews for 2 hours ends itself. `launch` replaces the process start in tests. */
export function createSidecarsV1(options:Readonly<{settings:SidecarSettingsV1;facts:SidecarFactsV1;environment?:Readonly<Record<string,string|undefined>>;now?:()=>number;peers?:()=>Promise<number|undefined>;launch?:(command:string,args:readonly string[],env:NodeJS.ProcessEnv,log:number)=>ChildProcess;peersMilliseconds?:number;protocol?:Partial<Record<keyof typeof sidecarProtocolV1,number>>}>):SidecarsV1{
 const now=options.now??Date.now,environment=options.environment??process.env,server=options.facts.originId===`${options.facts.groupId}-server`,bootedMs=now(),restart=options.settings.restart;
 const slots=options.settings.children.map((spec):Slot=>({spec,state:"off",pid:null,startedMs:0,sinceMs:bootedMs,failures:0,restarts:0,lastExit:null,nextStartMs:null}));
 const directory=join(options.facts.worktree,".cache","f","sidecars"),read=options.peers??(()=>countPlatformPeersV1(options.facts,now()));
 let peers:number|undefined,peersAt=-Infinity,reading=false,stopping=false,lastTick=-Infinity,zeroReads=0;
 const set=(slot:Slot,state:Slot["state"]):void=>{if(slot.state!==state){slot.state=state;slot.sinceMs=now();}};
 const refreshPeers=():void=>{
  if(reading||now()-peersAt<(options.peersMilliseconds??60_000))return;
  reading=true;peersAt=now();
  void read().then(count=>{if(count!==undefined){zeroReads=count===0?zeroReads+1:0;peers=count;}},()=>{}).finally(()=>{reading=false;});
 };
 const logFile=(name:string):number=>{
  mkdirSync(directory,{recursive:true});
  const file=join(directory,`${name}.log`);
  try{if(existsSync(file)&&statSync(file).size>2*1048576)renameSync(file,`${file}.1`);}catch{}
  return openSync(file,"a");
 };
 const launch=options.launch??((command,args,env,log)=>spawn(command,[...args],{env,stdio:["ignore",log,log],windowsHide:true,detached:true}));
 const protocol={...sidecarProtocolV1,...options.protocol},retiring=new Map<number,number>();
 let leasedAt=-Infinity;
 const entryOf=(slot:Slot):string=>join(options.facts.worktree,...slot.spec.script.split("/"));
 const settle=(slot:Slot,why:string):void=>{
  const ran=now()-slot.startedMs,ended=slot.pid;delete slot.proc;slot.pid=null;slot.lastExit=why;revoke(slot,ended);
  if(slot.adopted!==undefined){clearInterval(slot.adopted);delete slot.adopted;}
  if(stopping||slot.halt!==undefined){set(slot,slot.halt??"stopped");delete slot.halt;slot.nextStartMs=null;}
  else{slot.failures=ran>=restart.stableMilliseconds?1:slot.failures+1;slot.restarts++;slot.nextStartMs=now()+sidecarBackoffMillisecondsV1(restart,slot.failures);set(slot,"backoff");}
 };
 const renew=(slot:Slot):void=>{if(slot.pid===null)return;try{writeSidecarLeaseV1(sidecarLeaseOfV1(sidecarFilesV1(options.facts.worktree,slot.spec.name).lease,slot.pid),process.pid,Date.now());}catch{}};
 const revoke=(slot:Slot,pid:number|null):void=>{const base=sidecarFilesV1(options.facts.worktree,slot.spec.name).lease;try{rmSync(base,{force:true});if(pid!==null)rmSync(sidecarLeaseOfV1(base,pid),{force:true});}catch{}};
 const begin=(slot:Slot):void=>{
  let log=-1;
  try{
   log=logFile(slot.spec.name);
   const files=sidecarFilesV1(options.facts.worktree,slot.spec.name);
   rmSync(files.record,{force:true});revoke(slot,null);
   const env:NodeJS.ProcessEnv={...environment,SEMIO_FLEET_ROOT:options.facts.worktree,SEMIO_WORKSPACE_ROOT:options.facts.worktree,SEMIO_FLEET_ORIGIN:options.facts.originId,SEMIO_FLEET_GROUP:options.facts.groupId,SEMIO_FLEET_STORE_ROOT:options.facts.storeRoot,SEMIO_FLEET_SIDECAR:slot.spec.name,SEMIO_FLEET_SIDECAR_RECORD:files.record,SEMIO_FLEET_SIDECAR_LEASE:files.lease,SEMIO_FLEET_SIDECAR_LEASE_MS:String(protocol.leaseMilliseconds)};
   const proc=launch(process.execPath,[entryOf(slot),...slot.spec.arguments],env,log);
   slot.proc=proc;slot.pid=proc.pid??null;slot.startedMs=now();slot.nextStartMs=null;set(slot,"running");renew(slot);
   const done=new Promise<void>(finish=>{
    let settled=false;
    const end=(why:string):void=>{
     if(settled)return;settled=true;
     settle(slot,why);
     finish();
    };
    proc.on("error",error=>{end(`could not run: ${error.message}`);});
    proc.on("exit",(code,signal)=>{end(signal!==null?`signal ${signal}`:`exit ${code}`);});
   });
   slot.ending=done;
  }catch(error){
   slot.failures++;slot.restarts++;slot.lastExit=`could not start: ${(error as Error).message}`;slot.nextStartMs=now()+sidecarBackoffMillisecondsV1(restart,slot.failures);set(slot,"backoff");
  }finally{if(log>=0)try{closeSync(log);}catch{}}
 };
 const adopt=(slot:Slot,record:SidecarRecordV1):void=>{
  slot.pid=record.pid;slot.startedMs=record.startedMs;slot.nextStartMs=null;slot.lastExit=null;set(slot,"running");revoke(slot,null);renew(slot);
  slot.ending=new Promise<void>(finish=>{
   slot.adopted=setInterval(()=>{if(!sidecarRecordLivesV1(record)){settle(slot,"adopted process ended");finish();}},200);
   slot.adopted.unref?.();
  });
 };
 const claim=(slot:Slot,at:number):"adopted"|"waiting"|"free"=>{
  const record=readSidecarRecordV1(sidecarFilesV1(options.facts.worktree,slot.spec.name).record);
  if(record===undefined||record.name!==slot.spec.name||!sidecarRecordLivesV1(record))return "free";
  if(Date.now()-record.wallMs>protocol.aliveMilliseconds)return "free";
  if(record.entrySha256===sidecarEntryHashV1(entryOf(slot))){adopt(slot,record);return "adopted";}
  const first=retiring.get(record.pid);
  if(first===undefined){retiring.set(record.pid,at);try{process.kill(record.pid,"SIGTERM");}catch{}}
  else if(at-first>=options.settings.graceSeconds*1000)try{process.kill(record.pid,"SIGKILL");}catch{}
  slot.nextStartMs=at+250;
  return "waiting";
 };
 const end=(slot:Slot):Promise<void>=>{
  const proc=slot.proc,done=slot.ending;
  if(done===undefined)return Promise.resolve();
  const pid=proc?.pid??slot.pid;
  if(proc===undefined&&pid===null)return Promise.resolve();
  const send=(signal:NodeJS.Signals):void=>{try{if(proc!==undefined)proc.kill(signal);else process.kill(pid!,signal);}catch{}};
  send("SIGTERM");
  const timer=setTimeout(()=>{send("SIGKILL");},options.settings.graceSeconds*1000);
  return done.finally(()=>{clearTimeout(timer);});
 };
 return Object.freeze({
  tick:(healthy:boolean):void=>{
   try{
    const at=now();
    if(stopping||at-lastTick<200)return;
    lastTick=at;refreshPeers();
    const ready=healthy&&at-bootedMs>=options.settings.startAfterSeconds*1000,wanted=new Set(wantedSidecarsV1(options.settings,server,peers??0,environment.SEMIO_FLEET_SIDECARS).map(child=>child.name));
    const renewing=at-leasedAt>=protocol.renewMilliseconds;
    if(renewing)leasedAt=at;
    for(const slot of slots){
     const name=slot.spec.name,keep=wanted.has(name)||slot.spec.requires==="platform-peer"&&slot.state==="running"&&zeroReads<3&&wantedSidecarsV1(options.settings,server,1,environment.SEMIO_FLEET_SIDECARS).some(child=>child.name===name);
     if(slot.state==="running"){
      if(!keep){slot.halt="off";void end(slot);continue;}
      if(renewing)renew(slot);
      continue;
     }
     if(slot.state==="stopped")continue;
     if(!wanted.has(name)){if(slot.state!=="off"){set(slot,"off");slot.nextStartMs=null;}continue;}
     if(slot.nextStartMs!==null&&at<slot.nextStartMs)continue;
     if(slot.state==="off")set(slot,"waiting");
     if(claim(slot,at)!=="free")continue;
     if(!ready)continue;
     begin(slot);
    }
   }catch{}
  },
  report:():readonly SidecarStateV1[]=>slots.map(slot=>({name:slot.spec.name,state:slot.state,pid:slot.pid,restarts:slot.restarts,sinceMs:slot.sinceMs,lastExit:slot.lastExit,nextStartMs:slot.nextStartMs})),
  stop:async():Promise<void>=>{stopping=true;await Promise.all(slots.map(slot=>end(slot).catch(()=>{})));},
  release:():void=>{stopping=true;for(const slot of slots){if(slot.adopted!==undefined){clearInterval(slot.adopted);delete slot.adopted;}slot.proc?.unref();}},
 });
}
