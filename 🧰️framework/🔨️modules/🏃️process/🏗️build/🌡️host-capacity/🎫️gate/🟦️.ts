import{machinePathV1}from"../../🏠️machine-state/🟦️.ts";
import {concurrentEditV1,retryOnConcurrentEditSyncV1,type ChurnOptionsV1} from "../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

import {spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {appendFileSync,existsSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {homedir,uptime} from "node:os";
import {isAbsolute,join} from "node:path";
import {beginAdmissionWaitV1,liveChildPidsV1,onScriptProcessWaitEdgeV1,scriptProcessWaitEpisodeV1,waitingDelayV1 as delay} from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../🔒️leases/🎛️control/🟦️.ts";
import {decideHostHandoverV1,describeHostBlockersV1,hostBlockersV1,hostBootAnchorV1,hostUnderPressureV1,type HostBootAnchorV1,describeHostHandoverRefusalV1,describeHostReasonV1,evaluateHostCapacityV1,withUnmanagedHostV1,followHostGrantPlanV1,formatHostCapacityV1,hostCapacityErrorV1,isCurrentHostGrantV1,isHostCapacityV1,planHostCapacityV1,requireHostCapacityV1,resolveHostCapacityLanguageV1,softHostReasonV1,hostWaitDeadlineMillisecondsV1,hostRequestNeverFitsV1,type HostCapacityPolicyV1,type HostEvaluationV1,type HostGrantV1,type HostGrowthSettleV1,type HostHandoverV1,type HostPlanV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1,type HostDecisionV1,unsatisfiableHostRequestV1} from "../📏️decision/🟦️.ts";
import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";
import {systemAncestryV1,WINDOWS_PROCESS_LISTING_V1} from "../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts";
import {hostDeclarationKeyV1} from "../🪞️profile/🟦️.ts";
import {HOST_PHASES_V1,hostPhaseSegmentsV1,type HostPhaseTransitionV1,type HostPhaseV1} from "../🪜️phases/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {currentHostOrphanWatchV1} from "./👻️orphan/🟦️.ts";
import {absentV1,alive,currentSample,hostBootAtV1,hostCapacityDirectoryV1,modified,names,prepare,readGrants,readJson,readWaiters,remove,writeJson,type HostCapacityStatusV1} from "./📖️status/🟦️.ts";
export {hostBootAtV1,hostCapacityDirectoryV1,readHostCapacityStatusV1,type HostCapacityStatusV1} from "./📖️status/🟦️.ts";

const protocol=schema.definitions.Protocol.const,limits=schema.definitions.Limits.const;

/** 🆔️ Mints one unguessable request id. */
export function newHostCapacityIdV1():string{return randomBytes(16).toString("base64url");}

/** 🎟️ What an admitted operation holds and the environment its children inherit. */
export type HostCapacityHandleV1=Readonly<{id:string;jobs:number;cpu:number;reservedMegabytes:number;plan:HostPlanV1;inherited:boolean;adopted:boolean;bypassed:boolean;environment:Readonly<Record<string,string>>}>;

import type {HostCapacityOptionsV1,HostImageSweepV1,HostCapacityPortsV1,HostCapacityWaitV1,HostLedgerPrimitivesV1,HostProcessIdentityV1} from "./📋️contract/🟦️.ts";
export type {HostCapacityOptionsV1,HostCapacityPortsV1,HostCapacityWaitV1,HostLedgerPrimitivesV1,HostProcessIdentityV1,HostRunV1} from "./📋️contract/🟦️.ts";

/** 📥️ Reads the plan an enclosing grant carried into the environment, or undefined when the process runs under none. */
export function readHostCapacityPlanV1(environment:Readonly<Record<string,string|undefined>>):HostPlanV1|undefined{
 const text=environment[protocol.planEnvironment];
 if(text===undefined||text==="")return undefined;
 let value:unknown;try{value=JSON.parse(text);}catch{throw hostCapacityErrorV1("planRequired",{},environment);}
 return requireHostCapacityV1<HostPlanV1>("Plan",value);
}

/** 🔭️ Reads the plan an enclosing grant carried into the environment, followed to the grant row as it is now (`followHostGrantPlanV1`): a process started before its run grew or partly grew sees the jobs, cores and job memory the grant holds now; the carried plan when the row cannot be read, undefined when the process runs under no grant. */
export function readLiveHostCapacityPlanV1(environment:Readonly<Record<string,string|undefined>>):HostPlanV1|undefined{
 const plan=readHostCapacityPlanV1(environment);
 if(plan===undefined)return undefined;
 const row=readJson(grantPath(hostCapacityDirectoryV1(environment),plan.id));
 return isHostCapacityV1("Grant",row)?followHostGrantPlanV1(plan,row as HostGrantV1):plan;
}

/** 🧾️ Reads actual same-id ledger reservation DATA; the larger tool envelope never attests resident capacity. */
export function readLiveHostCapacityReservationV1(environment:Readonly<Record<string,string|undefined>>,policy:HostCapacityPolicyV1):Readonly<Pick<HostGrantV1,"id"|"jobs"|"cpu"|"reservedMegabytes">>|undefined{const plan=readHostCapacityPlanV1(environment);if(!plan)return undefined;const directory=hostCapacityDirectoryV1(environment),path=grantPath(directory,plan.id);const row=readJson(path);if(!isHostCapacityV1("Grant",row))return undefined;const grant=row as HostGrantV1,at=Date.now(),boot=hostBootAtV1(directory,at);if(!isCurrentHostGrantV1({grant,expectedId:plan.id,pidAlive:alive(grant.pid),nowMs:at,bootAtMs:boot,modifiedAtMs:modified(path)},policy))return undefined;return requireHostCapacityV1("Reservation",Object.freeze({id:grant.id,jobs:grant.jobs,cpu:grant.cpu,reservedMegabytes:grant.quiet?0:grant.reservedMegabytes}));}

const grantPath=(directory:string,id:string)=>join(directory,"grants",`${id}.json`);
const attachedPath=(directory:string,id:string)=>join(directory,"grants",`${id}.attached`);
const readAttached=(path:string):number[]=>{try{return readFileSync(path,"utf8").split("\n").map(Number).filter(pid=>Number.isSafeInteger(pid)&&pid>0);}catch(error){if(absentV1(error))return [];throw error;}};
/** 📎️ Lets a process attach a process it started outside its own tree (a persistent build server is not a descendant of the run that uses it) to the grant its environment carries, so the heartbeat counts the memory of the attached tree too and the learned peak of the operation includes it; false when the environment carries no grant. */
export function attachHostCapacityProcessV1(environment:Readonly<Record<string,string|undefined>>,pid:number):boolean{
 const id=environment[protocol.grantEnvironment];if(!id||!Number.isSafeInteger(pid)||pid<1)return false;
 const path=attachedPath(hostCapacityDirectoryV1(environment),id),pids=readAttached(path);
 if(!pids.includes(pid)){mkdirSync(join(path,".."),{recursive:true});const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,[...pids,pid].join("\n")+"\n");renameSync(temporary,path);}
 return true;
}
/** 🗒️ The file a run leaves beside its grant to say that work has to run now (see `requestHostCapacityUpgradeV1`). */
export const hostUpgradePathV1=(directory:string,id:string):string=>join(directory,"grants",`${id}${protocol.upgradeSuffix}`);
const phasesPath=(directory:string,id:string)=>join(directory,"grants",`${id}${protocol.phasesSuffix}`);
/** 🚏️ Lets any process of a run say which phase the run enters (assembly, plan, rebuild, execution): one line `<phase> <milliseconds>` appended to the grant's phase file, which the holder reads within a second; false when the environment carries no live grant, the switch of a nested invocation is off (an inherited grant belongs to the invocation that holds it; the holder itself when it re-enters the gate and the process it re-executes itself into, its direct child, continue that very invocation and keeps the switch on, so the plan of a captured run is learned) or the phase is unknown. */
export function markHostCapacityPhaseV1(environment:Readonly<Record<string,string|undefined>>,phase:HostPhaseV1,now:()=>number=Date.now):boolean{
 const id=environment[protocol.grantEnvironment];
 if(!id||environment[protocol.marksEnvironment]==="off"||!(HOST_PHASES_V1 as readonly string[]).includes(phase)||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return false;
 try{const directory=hostCapacityDirectoryV1(environment);if(!existsSync(grantPath(directory,id)))return false;appendFileSync(phasesPath(directory,id),`${phase} ${now()}\n`);return true;}catch{return false;}
}
const waitMarkerPath=(directory:string,id:string,pid:number)=>join(directory,"grants",`${id}${protocol.waitSuffix}${pid}`);
/** 🕰️ The start time of a process as `ps` prints it (`lstart`), the identity that tells a process from a later one that reuses its pid; undefined when the process is gone or the table cannot be read. Linux reads `/proc` without a spawn; the own start never changes and is read once, the start of another process is remembered `startCacheMilliseconds` so a poll never forks `ps` twice for one pid. */
const startCache=new Map<number,{start:string|undefined;at:number}>();
export function processStartV1(pid:number,now:()=>number=Date.now):string|undefined{
 const cached=startCache.get(pid),at=now();
 if(cached&&(pid===process.pid&&cached.start!==undefined||at-cached.at<protocol.startCacheMilliseconds))return cached.start;
 let start:string|undefined;
 try{
  if(process.platform==="linux")start=systemAncestryV1("linux",pid,1)[0]?.started;
  else{const result=spawnSync(process.platform==="win32"?"powershell.exe":"ps",process.platform==="win32"?["-NoLogo","-NoProfile","-Command",`(Get-Process -Id ${pid}).StartTime.ToUniversalTime().ToString('o')`]:["-o","lstart=","-p",String(pid)],{encoding:"utf8",timeout:limits.commandTimeoutMilliseconds,windowsHide:true,env:{...process.env,TZ:"UTC",LC_ALL:"C"}}),text=(result.stdout??"").trim();start=result.status===0&&text?text:undefined;}
 }catch{start=undefined;}
 if(startCache.size>=protocol.maximumWaitMarkers*4)startCache.clear();
 startCache.set(pid,{start,at});return start;
}
const sameProcess=(pid:number,start:string):boolean=>processStartV1(pid)===start;
type HostRebuildRequestV1=Readonly<{id:string;pid:number;start:string;askedAtMs:number;scope:string;holderPid:number;holderStart:string;holderBootAtMs:number;holderAdmittedAtMs:number}>;
type HostRebuildRefusalV1=Readonly<{request:HostRebuildRequestV1;minimum:number;total:number;floor:number;capacity:number}>;
const rebuildRequestsPath=(directory:string,id:string,scope:string):string=>join(directory,"grants",`${id}.rebuild.${scope}`);
const rebuildReceiptPath=(path:string):string=>path+".receipt";
const rebuildRequestMatches=(request:HostRebuildRequestV1,holder:HostGrantV1):boolean=>request.scope===holder.rebuildScope&&request.holderPid===holder.pid&&request.holderBootAtMs===holder.bootAtMs&&request.holderAdmittedAtMs===holder.admittedAtMs&&alive(holder.pid)&&sameProcess(holder.pid,request.holderStart);
function readRebuildRequest(path:string,holder:HostGrantV1):HostRebuildRequestV1|undefined{
 const row=readJson(path);if(!isHostCapacityV1("RebuildRequest",row))return undefined;
 const request=row as HostRebuildRequestV1;return rebuildRequestMatches(request,holder)&&alive(request.pid)&&sameProcess(request.pid,request.start)?request:undefined;
}

/** 🔎️ Checks the live issuer and holder births of an existing queued rebuild before an observer may pause its own watchdog. */
export function ownsQueuedHostRebuildV1(directory:string,grant:HostGrantV1,waiter:HostWaiterV1,owns:(pid:number,start:string)=>boolean):boolean{
 if(waiter.upgrade!==true||waiter.rebuild===undefined||waiter.rebuild!==grant.rebuildScope||waiter.pid!==grant.pid||waiter.request.id!==grant.id||grant.rebuildBookedAtMs!==undefined)return false;
 if(!ownsHostRebuildAskV1(directory,grant,waiter.arrivedAtMs,owns))return false;
 const current=readJson(queuePath(directory,waiter.arrivedAtMs,grant.id));
 return isHostCapacityV1("Waiter",current)&&JSON.stringify(current)===JSON.stringify(waiter);
}

/** 🪪️ Resolves the exact still-live rebuild request under the holder and issuer birth checks. */
export function ownsHostRebuildAskV1(directory:string,grant:HostGrantV1,askedAtMs:number,owns:(pid:number,start:string)=>boolean):boolean{
 if(grant.rebuildScope===undefined)return false;
 const folder=rebuildRequestsPath(directory,grant.id,grant.rebuildScope);
 let names:string[];try{names=readdirSync(folder);}catch(error){if(absentV1(error))return false;throw error;}
 for(const name of names.filter(name=>/^[A-Za-z0-9_-]{8,32}$/u.test(name)).slice(0,protocol.maximumWaitMarkers)){
  const path=join(folder,name),request=readRebuildRequest(path,grant);
  if(request?.id!==name||request.askedAtMs!==askedAtMs||isHostCapacityV1("RebuildRefusal",readJson(rebuildReceiptPath(path)))||!owns(request.pid,request.start)||!owns(request.holderPid,request.holderStart))continue;
  const current=readJson(grantPath(directory,grant.id));
  if(isHostCapacityV1("Grant",current)){
   const held=current as HostGrantV1;
   if(held.id===grant.id&&held.rebuildScope===grant.rebuildScope&&held.pid===grant.pid&&held.bootAtMs===grant.bootAtMs&&held.admittedAtMs===grant.admittedAtMs&&held.rebuildBookedAtMs===grant.rebuildBookedAtMs)return true;
  }
 }
 return false;
}

/** 🎟️ The megabytes the grant of this process holds for its booked rebuild right now (what the gate granted, which may be less than the rebuild asked for), so the farm of the graph build can be planned against it; undefined without a booked rebuild.
 * @see requestHostRebuildCapacityV1 */
export function readHostRebuildGrantMegabytesV1(environment:Readonly<Record<string,string|undefined>>):number|undefined{
 const id=environment[protocol.grantEnvironment];
 if(!id||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return undefined;
 const row=readJson(grantPath(hostCapacityDirectoryV1(environment),id));
 return isHostCapacityV1("Grant",row)&&(row as HostGrantV1).rebuildBookedAtMs!==undefined?(row as HostGrantV1).reservedMegabytes:undefined;
}

/** 🏗️ Waits for an exact funded rebuild scope and returns its release after graph custody closes. */
export async function requestHostRebuildCapacityV1(environment:Readonly<Record<string,string|undefined>>,control:ResourceLeaseControl,reuse?:()=>Promise<boolean>):Promise<(()=>void)|undefined>{
 const id=environment[protocol.grantEnvironment];
 if(!id||environment[protocol.marksEnvironment]==="off"||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return undefined;
 checkResourceLeaseControl(control);
 const directory=hostCapacityDirectoryV1(environment),row=grantPath(directory,id),first=readJson(row)as HostGrantV1|undefined;
 if(!isHostCapacityV1("Grant",first)||first?.rebuildScope===undefined)return undefined;
 const start=processStartV1(process.pid),holderStart=processStartV1(first.pid);if(start===undefined||holderStart===undefined)throw hostCapacityErrorV1("planRequired",{},environment);
 const request=Object.freeze({id:newHostCapacityIdV1(),pid:process.pid,start,askedAtMs:Date.now(),scope:first.rebuildScope,holderPid:first.pid,holderStart,holderBootAtMs:first.bootAtMs,holderAdmittedAtMs:first.admittedAtMs}),folder=rebuildRequestsPath(directory,id,request.scope),path=join(folder,request.id),receipt=rebuildReceiptPath(path),wire=JSON.stringify(request);
 const leave=beginAdmissionWaitV1();let retained=false;
 const release=()=>{process.removeListener("exit",release);try{remove(path);}finally{remove(receipt);}};
 try{
  mkdirSync(folder,{recursive:true});writeJson(path,requireHostCapacityV1<HostRebuildRequestV1>("RebuildRequest",request));
  for(;;){
   const budget=checkResourceLeaseControl(control),current=readJson(row)as HostGrantV1|undefined;
   if(!isHostCapacityV1("Grant",current)||current===undefined||!rebuildRequestMatches(request,current))throw hostCapacityErrorV1("planRequired",{},environment);
   const evidence=readJson(receipt);
   if(isHostCapacityV1("RebuildRefusal",evidence)){
    const{request:refused,minimum,total,floor,capacity}=evidence as HostRebuildRefusalV1;
    if(JSON.stringify(refused)===wire)throw hostCapacityErrorV1("memoryNeverFits",{operation:first.operation,minimum,total,floor,capacity,needed:minimum+floor},environment);
   }
   if(current.lentSinceMs===undefined&&current.rebuildBookedAtMs!==undefined&&JSON.stringify(evidence)===wire){retained=true;process.once("exit",release);return release;}
   if(reuse!==undefined&&await reuse()){checkResourceLeaseControl(control);return undefined;}
   try{await delay(Math.max(1,Math.min(protocol.upgradePollMilliseconds,budget??Infinity)),undefined,{signal:control.signal});}catch(error){throw control.signal.aborted?control.signal.reason:error;}
  }
 }finally{try{if(!retained)release();}finally{leave();}}
}
/** 🛌️ Publishes that THIS process waits (in a lease, claim, headroom or capacity queue) to the grant its environment carries: one marker file `<grant>.wait.<pid>` holding the start of the wait episode, removed `waitReleaseGapMilliseconds` after the wait ends so the polls of one queue do not flicker. The holder of the grant reads the markers to see that its whole process tree only waits and lends the reservation (see `holdHostGrantV1`): a run whose child sits at an inner gate no longer idles on memory other runs need. Returns the function that unbinds the process; false-y environment (no grant) binds nothing. */
export function bindHostCapacityWaitMarkerV1(environment:Readonly<Record<string,string|undefined>>=process.env,pid:number=process.pid):()=>void{
 const id=environment[protocol.grantEnvironment];
 if(!id||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return()=>undefined;
 let release:ReturnType<typeof setTimeout>|undefined,directory:string;try{directory=hostCapacityDirectoryV1(environment);}catch{return()=>undefined;}
 const path=waitMarkerPath(directory,id,pid);let written:number|undefined;
 const drop=()=>{written=undefined;try{unlinkSync(path);}catch{}};
 const off=onScriptProcessWaitEdgeV1((waiting,since)=>{
  if(release){clearTimeout(release);release=undefined;}
  if(waiting){
   if(written===since)return;
   try{const start=processStartV1(pid);if(start&&existsSync(grantPath(directory,id))){const temporary=`${path}.${randomBytes(4).toString("hex")}.tmp`;writeFileSync(temporary,JSON.stringify({since,start}));renameSync(temporary,path);written=since;}}catch{}
   return;
  }
  release=setTimeout(drop,protocol.waitReleaseGapMilliseconds);release.unref?.();
 });
 process.once("exit",drop);
 return()=>{off();if(release)clearTimeout(release);drop();process.removeListener("exit",drop);};
}
/** 👀️ The processes that currently wait under one grant and since when (start of their wait episode). A marker names the pid AND the start time of its process: the marker of a killed process (no exit handler ran) never counts a later process that reuses the pid. */
export function readHostCapacityWaitMarkersV1(directory:string,id:string,alive:(pid:number,start:string)=>boolean=sameProcess):ReadonlyMap<number,number>{
 const found=new Map<number,number>(),prefix=`${id}${protocol.waitSuffix}`;
 let names:string[]=[];try{names=readdirSync(join(directory,"grants")).filter(file=>file.startsWith(prefix)&&!file.endsWith(".tmp")).slice(0,protocol.maximumWaitMarkers);}catch{return found;}
 for(const name of names){
  try{
   const pid=Number(name.slice(prefix.length)),row=JSON.parse(readFileSync(join(directory,"grants",name),"utf8")) as {since?:unknown;start?:unknown};
   if(Number.isSafeInteger(pid)&&pid>0&&typeof row.since==="number"&&Number.isFinite(row.since)&&typeof row.start==="string"&&alive(pid,row.start))found.set(pid,row.since);
  }catch{}
 }
 return found;
}
/** 🌳️ The child lists of the live process table (`pid -> direct children`); undefined when the table cannot be read, so nothing is ever lent on a guess. */
export function processChildrenTableV1():ReadonlyMap<number,readonly number[]>|undefined{
 const table=new Map<number,number[]>(),add=(pid:number,parent:number)=>{if(Number.isSafeInteger(pid)&&Number.isSafeInteger(parent))(table.get(parent)??table.set(parent,[]).get(parent)!).push(pid);};
 try{
  if(process.platform==="linux"){
   for(const name of readdirSync("/proc"))if(/^\d+$/u.test(name))try{const stat=readFileSync(`/proc/${name}/stat`,"utf8"),rest=stat.slice(stat.lastIndexOf(")")+2).split(" ");add(Number(name),Number(rest[1]));}catch{}
   return table;
  }
  if(process.platform==="win32"){
   const result=spawnSync("powershell.exe",["-NoLogo","-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_LISTING_V1],{encoding:"utf8",timeout:10000,windowsHide:true,maxBuffer:33554432});if(result.status!==0)return undefined;
   const rows=JSON.parse(result.stdout) as unknown,list=Array.isArray(rows)?rows:[rows];for(const row of list as {pid?:number;ppid?:number}[])add(row.pid!,row.ppid!);
   return table;
  }
  const result=spawnSync("ps",["-axo","pid=,ppid="],{encoding:"utf8",timeout:limits.commandTimeoutMilliseconds,env:{...process.env,LC_ALL:"C"}});if(result.status!==0)return undefined;
  for(const line of result.stdout.split("\n")){const [pid,parent]=line.trim().split(/\s+/u).map(Number);add(pid!,parent!);}
  return table;
 }catch{return undefined;}
}
/** 🛌️ Whether the whole process tree below the grant holder only waits, and since when (0 when it does not). A process waits when it published a wait marker itself or when it has children and every one of them waits: the wrapper that merely awaits its waiting child declares no wait of its own and still counts. A holder without children is not decided here (its own wait episode covers it). */
export function hostTreeWaitsV1(children:ReadonlyMap<number,readonly number[]>|undefined,root:number,markers:ReadonlyMap<number,number>):number{
 if(children===undefined)return 0;const direct=children.get(root)??[];if(direct.length===0)return 0;
 const since=(pid:number,depth:number):number=>{
  if(depth>64)return 0;const own=markers.get(pid),kids=children.get(pid)??[];
  if(kids.length===0)return own??0;
  let earliest=own??Infinity;for(const kid of kids){const value=since(kid,depth+1);if(value===0)return 0;earliest=Math.min(earliest,value);}return earliest;
 };
 let earliest=Infinity;for(const kid of direct){const value=since(kid,0);if(value===0)return 0;earliest=Math.min(earliest,value);}return earliest;
}
const queuePath=(directory:string,arrivedAtMs:number,id:string)=>join(directory,"queue",`${String(arrivedAtMs).padStart(15,"0")}-${id}.json`);


/** 🔒️ Takes the machine-wide ledger lock (a directory with an owner file) once: `false` when another process holds it, when the lock was lost to a stale-lock breaker between creation and the owner write (the caller simply asks again), or after this call broke a lock whose owner is gone or too old; a stale verdict is acted on only while the owner text is still the one it was based on, so a breaker never removes the lock a newer taker just wrote. `afterCreate` lets a law pause between creation and the owner write. */
const lockOwners=new Map<string,string>(),lockKey=(lock:string,pid:number):string=>lock+"\n"+pid,ownerText=(lock:string):string|undefined=>{try{return readFileSync(join(lock,"owner"),"utf8");}catch{return undefined;}};
/** 🔏️ Whether the gate lock is still the one `pid` took in this process: the owner file holds exactly the text it wrote. A holder that stalled past `wait.lockStaleMilliseconds` (swap, a stopped process) may have been broken by a peer that then took the lock; such a holder must write nothing more. */
function holdsLock(directory:string,pid:number):boolean{const lock=join(directory,"lock"),mine=lockOwners.get(lockKey(lock,pid));return mine!==undefined&&ownerText(lock)===mine;}
/** 🧪️ Narrow removal and retry ports for the fenced lock-release laws. */
export type HostLockReleasePortsV1=Readonly<{remove?:(path:string)=>void;retry?:ChurnOptionsV1}>;
/** 🔂️ Retry only a known busy lock removal and recheck its exact owner before every attempt. */
function removeLock(lock:string,expected:string,ports:HostLockReleasePortsV1={}):void{
 retryOnConcurrentEditSyncV1("read",()=>{if((ownerText(lock)??"")!==expected)return;try{(ports.remove??(path=>rmSync(path,{recursive:true,force:true})))(lock);}catch(error){if((error as NodeJS.ErrnoException).code==="EBUSY")throw concurrentEditV1("busy",lock);throw error;}},ports.retry);
}
/** 🔓️ Gives the gate lock back only while it is still the one `pid` took (see `holdsLock`), so a holder whose lock was broken never removes the lock of the peer that holds it now. */
function releaseLock(directory:string,pid:number,ports:HostLockReleasePortsV1={}):void{const lock=join(directory,"lock"),key=lockKey(lock,pid),mine=lockOwners.get(key);try{if(mine!==undefined)removeLock(lock,mine,ports);}finally{lockOwners.delete(key);}}
/** 🔐️ Narrow creation and bounded retry ports for atomic lock acquisition laws. */
export type HostLockAcquirePortsV1=Readonly<{create?:(path:string)=>void;retry?:ChurnOptionsV1}>;
function takeLock(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number,afterCreate?:()=>void,ports:HostLockAcquirePortsV1={}):boolean{
 const lock=join(directory,"lock");
 try{retryOnConcurrentEditSyncV1("read",()=>{try{(ports.create??(path=>mkdirSync(path,{mode:0o700})))(lock);}catch(error){if(["EPERM","EBUSY"].includes((error as NodeJS.ErrnoException).code??""))throw concurrentEditV1("busy",lock);throw error;}},ports.retry);}
 catch(error){
  if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
  let text="";try{text=readFileSync(join(lock,"owner"),"utf8");}catch{}
  const owner=/^(\d+):(\d+)$/u.exec(text),since=owner?Number(owner[2]):modified(lock);
  if(owner&&!alive(Number(owner[1]))||now-since>policy.wait.lockStaleMilliseconds){let again="";try{again=readFileSync(join(lock,"owner"),"utf8");}catch{}if(again===text)removeLock(lock,text);}
  return false;
 }
 afterCreate?.();
 try{const text=`${pid}:${now}`;writeFileSync(join(lock,"owner"),text,{flag:"wx"});lockOwners.set(lockKey(lock,pid),text);return true;}
 catch(error){if(LOST_LOCK_OWNER_CODES.includes((error as NodeJS.ErrnoException).code??""))return false;throw error;}
}
/** 🏁️ A peer that broke the lock as stale removes the directory between our `mkdir` and the owner write: the open then fails with ENOENT, EEXIST, or (APFS, directory mid-removal) EINVAL, and every one of them means this attempt lost the lock. */
const LOST_LOCK_OWNER_CODES:readonly string[]=["ENOENT","EEXIST","EINVAL"];
/** 🧷️ The ledger lock primitive for laws that need to pause inside it. */
export const takeHostLockV1=takeLock;
/** 🗝️ The fenced release of the ledger lock (`releaseLock`), for laws. */
export const releaseHostLockV1=releaseLock;
/** 🧿️ The ownership check of the ledger lock (`holdsLock`), for laws. */
export const holdsHostLockV1=holdsLock;


/** 💿️ Detaches the images of dead test sandboxes below the workspace of the caller before an image count refuses a request, and counts what is ours, leaked and foreign. */
async function sweepLeakedImagesV1(environment:Readonly<Record<string,string|undefined>>):Promise<HostImageSweepV1>{
 const{sweepLeakedSandboxImagesV1}=await import("../../../🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts"),root=environment.SEMIO_WORKSPACE_ROOT;
 return sweepLeakedSandboxImagesV1({parents:root===undefined?[]:[machinePathV1(root,".cache","test-sandbox")]});
}

/** ⏳️ The progress line of a waiting request: the arithmetic of the room (available, floor, what is reserved but not yet resident, what was admitted ahead), how many megabytes it is short and who holds them (`hostBlockersV1`).
 * @see ../🧪️tests/🧾️blockers/🟦️.ts */
export function hostWaitingLineV1(language:"en"|"de",request:HostRequestV1,host:HostSampleV1,evaluation:HostEvaluationV1,running:number,seconds:number,policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number,capture?:(facts:HostCapacityWaitV1)=>void):string{
 const cap=Math.max(0,host.totalMegabytes-evaluation.floorMegabytes),position=evaluation.decisions.findIndex(row=>row.id===request.id),mine=evaluation.decisions[position]!,before=evaluation.decisions.slice(0,position),taken=before.reduce((sum,row)=>sum+row.reservedMegabytes,0),spare=evaluation.headroomMegabytes-taken,need=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,cap);
 const reason=mine.reason as HostCapacityWaitV1["reason"];
 capture?.(Object.freeze({reason,waitedSeconds:Math.max(0,seconds),active:running,queued:waiters.length,spareMegabytes:Math.max(0,Math.floor(spare)),floorMegabytes:evaluation.floorMegabytes,minimumMegabytes:request.minimumMegabytes??need}));
 return formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:describeHostReasonV1(language,reason),need,spare,available:host.availableMegabytes,held:evaluation.unrealizedMegabytes,taken,short:mine.reason==="memory"&&need>spare?formatHostCapacityV1(language,"waitingShort",{megabytes:need-spare}):"",floor:evaluation.floorMegabytes,load:host.load===null?"n/a":host.load,running,ahead:position,behind:describeHostBlockersV1(language,hostBlockersV1(policy,ledger,waiters,evaluation,request.id,nowMs))});
}

/** 🚪️ Admits one operation against the whole machine and runs its body under the grant: it waits in a fair machine-wide queue with progress lines and cancellation, reserves memory and cores while the body runs and always gives them back. A nested invocation runs its body under an inherited grant at once, with the plan the holder carried and its jobs and cores (for a quiet grant those of the full request, never the quiet row's single job: a forecast changes what a run reserves, never what it may use); it never grows a quiet grant itself, an executor of the run asks when work has to run (`requestHostCapacityUpgradeV1`).
 * An alternate admission may return a continuation on a waiting progress tick. The gate removes its waiter before calling that continuation. */
export async function withHostCapacityV1<T>(options:HostCapacityOptionsV1,body:(handle:HostCapacityHandleV1)=>Promise<T>,alternate?:(signal:AbortSignal)=>Promise<(()=>Promise<T>)|undefined>):Promise<T>{
 const environment=options.environment??process.env,policy=options.policy,request=requireHostCapacityV1<HostRequestV1>("Request",options.request),language=resolveHostCapacityLanguageV1(environment),write=options.write??(line=>process.stderr.write(line+"\n")),now=options.now??Date.now,pid=options.pid??process.pid;
 if(options.upgrade!==undefined)requireHostCapacityV1<HostRequestV1>("Request",options.upgrade);
 if(request.quiet===true&&(options.upgrade===undefined||options.upgrade.id!==request.id||options.upgrade.quiet===true))throw hostCapacityErrorV1("quietUpgradeRequired",{},environment);
 if(!policy.enabled||environment[protocol.switchEnvironment]==="off"){
  if(policy.enabled)write(formatHostCapacityV1(language,"bypassed",{switch:protocol.switchEnvironment}));
  const sizing=request.quiet===true?options.upgrade!:request,plan=planHostCapacityV1(policy,{totalMegabytes:1048576},sizing,sizing.preferredJobs);
  return body(Object.freeze({id:request.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:0,plan,inherited:false,adopted:false,bypassed:true,environment:Object.freeze({})}));
 }
 if(options.directory===undefined)void import("../🛡️warden/🔁️loop/🟦️.ts").then(module=>module.ensureHostWardenV1()).catch(()=>undefined);
 const inheritedId=environment[protocol.grantEnvironment];
 if(inheritedId!==undefined&&!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(inheritedId))throw hostCapacityErrorV1("inheritedIdentifier",{},environment);
 const directory=prepare(options.directory),inheritedPath=inheritedId?grantPath(directory,inheritedId):undefined,inherited=inheritedPath?readJson(inheritedPath):undefined,at=now();
 if(inheritedPath&&isCurrentHostGrantV1({grant:inherited,expectedId:inheritedId,pidAlive:isHostCapacityV1("Grant",inherited)&&alive((inherited as HostGrantV1).pid),nowMs:at,bootAtMs:hostBootAtV1(directory,at),modifiedAtMs:modified(inheritedPath)},policy)){
  const grant=inherited as HostGrantV1,carried=readHostCapacityPlanV1(environment),plan=carried?.id===grant.id?followHostGrantPlanV1(carried,grant):requireHostCapacityV1<HostPlanV1>("Plan",{version:1,id:grant.id,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:grant.reservedMegabytes,fixedMegabytes:0,floorMegabytes:0});
  return body(Object.freeze({id:grant.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:grant.reservedMegabytes,plan,inherited:true,adopted:false,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:grant.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:grant.pid===pid||grant.pid===(options.ppid??process.ppid)?"on":"off"})}));
 }
 const offered=environment[protocol.handoverEnvironment];
 if(offered!==undefined){const claimed=await(await import("./🤝️handover/🟦️.ts")).claimHostGrantHandoverV1(custody(),options,directory,offered,request,language,now,pid);if(claimed)return holdHostGrantV1(options,claimed,undefined,request,directory,pid,body);}
 if(names(directory,"queue",{retry:{signal:options.control.signal}}).length>=policy.wait.maximumWaiters
  &&readWaiters(directory,now(),policy,options.control.signal,false).length>=policy.wait.maximumWaiters)throw Error(schema.definitions.Messages.const[language].errors.queueFull);
 const arrivedAtMs=now(),waiterPath=queuePath(directory,arrivedAtMs,request.id),record:HostWaiterV1={request,arrivedAtMs,pid,...options.session===undefined?{}:{session:options.session.address,weight:options.session.weight}};
 writeJson(waiterPath,record);
 let grant:HostGrantV1|undefined,admittedHost:HostSampleV1|undefined,lastProgress=-Infinity,continuation:(()=>Promise<T>)|undefined,lastWait:HostDecisionV1["reason"]|undefined;
 let sweptAtMs=-Infinity,split:string|undefined;
 const endAdmissionWait=beginAdmissionWaitV1();
 try{
  while(!grant){
   checkResourceLeaseControl(options.control);
   let host=await currentSample(directory,policy,options.control,options,environment);
   if(host.attachedImages!==null&&host.attachedImages>=policy.attaches.maximum&&now()-sweptAtMs>=policy.wait.progressMilliseconds){
    sweptAtMs=now();
    try{
     const swept=await(options.sweepImages??(()=>sweepLeakedImagesV1(environment)))();
     split=(await import("../../../🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts")).describeImageSplitV1(language,swept);
     if(swept.detached>0){host=Object.freeze({...host,attachedImages:Math.max(0,host.attachedImages-swept.detached)});write("[capacity] "+(await import("../../../🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts")).describeImageSweepV1(language,swept.detached));}
    }catch{}
   }
   try{utimesSync(waiterPath,new Date(),new Date());}catch{}
   let progressAt:number|undefined,waitFacts:HostCapacityWaitV1|undefined;
   if(takeLock(directory,now(),policy,pid)){
    try{
     const at=now(),ledger=readGrants(directory,at,policy,undefined,options.control.signal),waiters=readWaiters(directory,at,policy,options.control.signal);
     if(!waiters.some(row=>row.request.id===request.id)){writeJson(waiterPath,record);waiters.push(record);}
     const evaluation=evaluateHostCapacityV1(policy,withUnmanagedHostV1(host,ledger),ledger,waiters,at),found=evaluation.decisions.find(row=>row.id===request.id)!,drained=Math.min(host.availableMegabytes+ledger.reduce((sum,row)=>sum+(row.lentSinceMs===undefined?row.observedMegabytes??row.reservedMegabytes:0),0),host.totalMegabytes)-evaluation.floorMegabytes,asked=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes)),lone=found.verdict==="wait"&&found.reason==="memory"&&evaluation.decisions.every(row=>row.verdict==="wait")&&evaluation.decisions[0]!.id===request.id&&unsatisfiableHostRequestV1(policy,{waitedMilliseconds:at-arrivedAtMs,roomMegabytes:evaluation.headroomMegabytes,drainedMegabytes:drained,needMegabytes:asked,admitted:0,replaced:false,quiet:request.quiet===true,elastic:request.elastic===true,pause:softHostReasonV1(policy,host)}),mine:HostDecisionV1=lone?Object.freeze({id:request.id,verdict:"admit",jobs:request.minimumJobs,reservedMegabytes:Math.min(asked,Math.floor(evaluation.headroomMegabytes)),reason:"reduced"}):found;
     if(mine.verdict==="wait"&&(mine.reason==="never"||mine.reason==="starved"||hostRequestNeverFitsV1(policy,host,request))){
      const total=host.totalMegabytes,floor=evaluation.floorMegabytes,minimum=request.minimumMegabytes??request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes;
      const held=mine.reason==="starved"&&ledger.some(row=>row.id!==request.id&&row.lentSinceMs===undefined);
      throw hostCapacityErrorV1(mine.reason!=="starved"?"memoryNeverFits":held?"memoryWaitedOut":"memoryStarved",{operation:request.operation,minimum,total,floor,capacity:Math.floor(Math.max(0,total-floor)*policy.memory.budgetBasisPoints/10000),needed:minimum+floor,spare:Math.max(0,evaluation.headroomMegabytes),seconds:Math.round((at-arrivedAtMs)/1000),...held?{holders:describeHostBlockersV1(language,hostBlockersV1(policy,ledger,waiters,evaluation,request.id,at))}:{}},environment);
     }
     const before=lastWait;if(mine.verdict==="wait")lastWait=mine.reason;
     if(mine.verdict==="admit"){
      if(!holdsLock(directory,pid))continue;
      grant=requireHostCapacityV1<HostGrantV1>("Grant",{id:request.id,operation:request.operation,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,admittedAtMs:at,observedMegabytes:null,pid,bootAtMs:hostBootAtV1(directory,at),...options.session===undefined?{}:{session:options.session.address},...request.label===undefined?{}:{label:request.label},...request.holder===undefined||mine.reason!=="lent"?{}:{holder:request.holder},...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},...request.rampSlot===undefined?{}:{rampSlot:request.rampSlot},...request.quiet===true?{quiet:true,arrivedAtMs}:{}});
      writeJson(grantPath(directory,request.id),grant);remove(waiterPath);admittedHost=host;
      const seconds=Math.round((at-arrivedAtMs)/1000);
      if(lone)write(formatHostCapacityV1(language,"unsatisfiable",{operation:request.operation,seconds,reserved:mine.reservedMegabytes,asked,available:Math.max(0,drained),floor:evaluation.floorMegabytes}));
      else if(mine.reason==="override")write(formatHostCapacityV1(language,"override",{operation:request.operation,seconds,reason:describeHostReasonV1(language,softHostReasonV1(policy,host)??"load")}));
      else if(mine.reason==="lent")write(formatHostCapacityV1(language,"lent",{operation:request.operation,holder:ledger.find(row=>row.id===request.holder)?.label??request.holder??"",jobs:mine.jobs,reserved:mine.reservedMegabytes}));
      else if(mine.reason==="shrunk"){
       const stopped=before!==undefined&&before!=="memory"&&before!=="queue-order"&&before!=="never"&&before!=="starved"&&at-arrivedAtMs>=hostWaitDeadlineMillisecondsV1(policy,request.priority),values={operation:request.operation,reserved:mine.reservedMegabytes,wanted:request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,spare:Math.max(0,evaluation.headroomMegabytes),floor:evaluation.floorMegabytes,minimum:request.minimumMegabytes??0};
       write(stopped?formatHostCapacityV1(language,"shrunkStopped",{...values,seconds:Math.round((at-arrivedAtMs)/1000),limit:Math.round(hostWaitDeadlineMillisecondsV1(policy,request.priority)/1000),reason:describeHostReasonV1(language,before as Exclude<HostDecisionV1["reason"],"fits"|"reduced"|"override"|"shrunk"|"lent">)}):formatHostCapacityV1(language,"shrunk",values));
      }
      else if(mine.reason==="reduced")write(formatHostCapacityV1(language,options.upgrade!==undefined&&options.upgrade.perJobMegabytes>0?"reducedGrows":"reduced",{operation:request.operation,jobs:mine.jobs,preferred:request.preferredJobs}));
      else write(formatHostCapacityV1(language,"admitted",{operation:request.operation,jobs:mine.jobs,reserved:mine.reservedMegabytes,available:host.availableMegabytes,floor:evaluation.floorMegabytes}));
     }else if(at-lastProgress>=policy.wait.progressMilliseconds){
      lastProgress=at;
      const limit=hostWaitDeadlineMillisecondsV1(policy,request.priority);
      write(hostWaitingLineV1(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000),policy,ledger,waiters,at,options.onWait===undefined?undefined:facts=>{waitFacts=facts;})+(split!==undefined&&mine.reason==="attaches"?"; "+split:"")+(at-arrivedAtMs>=limit?formatHostCapacityV1(language,"pastDeadline",{limit:Math.round(limit/1000),holders:describeHostBlockersV1(language,hostBlockersV1(policy,ledger,waiters,evaluation,request.id,at))}):""));progressAt=at;
     }
    }finally{releaseLock(directory,pid);}
   }
   if(grant)break;
   if(progressAt!==undefined){
    if(waitFacts!==undefined&&!options.control.signal.aborted)try{options.onWait?.(waitFacts);}catch{}
    await options.control.publish({resource:protocol.resource,mode:"shared",elapsedMs:progressAt-arrivedAtMs});
    checkResourceLeaseControl(options.control);
    await options.control.yieldContinuation();
    if(alternate!==undefined&&progressAt-arrivedAtMs>=policy.wait.progressMilliseconds){
     checkResourceLeaseControl(options.control);
     continuation=await alternate(options.control.signal);
     checkResourceLeaseControl(options.control);
     if(continuation!==undefined){remove(waiterPath);break;}
    }
   }
   const budget=checkResourceLeaseControl(options.control);
   try{await delay(Math.max(1,Math.min(policy.wait.pollMilliseconds*(0.75+Math.random()*0.5),budget??Infinity)),undefined,{signal:options.control.signal});}catch(error){throw options.control.signal.aborted?options.control.signal.reason:error;}
  }
 }catch(error){remove(waiterPath);throw error;}finally{endAdmissionWait();}
 return continuation===undefined?holdHostGrantV1(options,grant!,admittedHost!,request,directory,pid,body):continuation();
}

/** 🛑️ Whether a failure is a cancellation: the wait control was aborted, the error carries the cancelled code, or it is an abort. */
const cancelledV1=(error:unknown,control:ResourceLeaseControl):boolean=>control.signal.aborted||(error as {code?:unknown;name?:unknown}|null)?.code==="cancelled"||(error as {name?:unknown}|null)?.name==="AbortError";

const custody=():HostLedgerPrimitivesV1=>Object.freeze({protocol,takeLock,releaseLock,holdsLock,grantPath,readJson,writeJson,remove,alive,bootAt:hostBootAtV1,modified,delay,checkResourceLeaseControl});

/** 🔐️ Holds one admitted grant while its body runs: heartbeats what the process tree holds, lends the reservation while the process only waits in a queue, gives the reservation back when the body ends (or keeps it parked when the body failed in a way a restart cures and the caller says so). An adopted grant carries the plan its owner admitted. A run that marks its phases (`markHostCapacityPhaseV1`) is cut into phase segments by the readings it was observed at, and its reservation follows the `resize` port at each boundary; a grant in its `rebuild` phase counts in full and lends the unused part of it to the workers that name it for as long as a grant may live (`wait.staleMilliseconds`, not only for the learned time to peak: a graph refresh of many waves runs for hours), so the farm workers of a build always fit inside its grant and a second rebuild of that size is not admitted beside it unless it fits physically; the segments go to `learn`. A quiet grant whose tree holds more than its quiet reservation grows as if an executor had asked: work runs that no executor announced (the executors, which wait for the growth, are the planner, a graph rebuild and the birth of every Bazel action that really runs); the run goes on meanwhile and is learned as executed. */
async function holdHostGrantV1<T>(options:HostCapacityOptionsV1,held:HostGrantV1,admittedHost:HostSampleV1|undefined,request:HostRequestV1,directory:string,pid:number,body:(handle:HostCapacityHandleV1)=>Promise<T>):Promise<T>{
 const policy=options.policy,now=options.now??Date.now,environment=options.environment??process.env,path=grantPath(directory,held.id),adopted=admittedHost===undefined,carried=adopted?readHostCapacityPlanV1(environment):undefined;
 const plan=adopted?carried?.id===held.id?carried:requireHostCapacityV1<HostPlanV1>("Plan",{version:1,id:held.id,jobs:held.jobs,cpu:held.cpu,memoryMegabytes:held.reservedMegabytes,fixedMegabytes:0,floorMegabytes:0}):planHostCapacityV1(policy,admittedHost!,held.quiet===true&&options.upgrade!==undefined?options.upgrade:request,held.quiet===true&&options.upgrade!==undefined?options.upgrade.preferredJobs:held.jobs);
 const full=options.upgrade,quiet=held.quiet===true&&full!==undefined,growable=full!==undefined&&!adopted&&full.perJobMegabytes>0,upgradeFile=hostUpgradePathV1(directory,held.id);
 const rebuildScope=options.resize===undefined?undefined:newHostCapacityIdV1(),rebuildFolder=rebuildScope===undefined?undefined:rebuildRequestsPath(directory,held.id,rebuildScope);
 let growing=false,outgrown=false,settled:HostGrowthSettleV1|undefined,upgrading=false,rebuilding=false,stopped=false,attempted=-Infinity,holding=true,state:HostGrantV1=held,childless=false,checkedChildren=-Infinity,parked=false,pressured=false,peak=0,lentTotal=0,failure:unknown,failed=false,marks:(readonly [number,number,HostPhaseV1|undefined])[]=[],phase:HostPhaseV1|undefined,deferredPhase:HostPhaseV1|undefined,transitions:HostPhaseTransitionV1[]=[],markLines=0,markBytes=0;
 const startedAt=now();
 const cleanup=()=>{holding=false;try{unlinkSync(path);}catch{}try{unlinkSync(attachedPath(directory,held.id));}catch{}try{unlinkSync(phasesPath(directory,held.id));}catch{}try{unlinkSync(upgradeFile);}catch{}if(rebuildFolder!==undefined)try{rmSync(rebuildFolder,{recursive:true,force:true});}catch{}try{const prefix=`${held.id}${protocol.waitSuffix}`;for(const name of readdirSync(join(directory,"grants")))if(name.startsWith(prefix))unlinkSync(join(directory,"grants",name));}catch{}};
 process.once("exit",cleanup);
 const publish=(next:HostGrantV1):void=>{state=next;if(holding)writeJson(path,state);};
 if(rebuildScope!==undefined||held.rebuildScope!==undefined){const{rebuildScope:_,rebuildBookedAtMs:__,...original}=held;publish({...original,...rebuildScope===undefined?{}:{rebuildScope}});}
 const rebuildRequests=():HostRebuildRequestV1[]=>{
  if(rebuildFolder===undefined)return[];
  let files:string[];try{files=readdirSync(rebuildFolder);}catch(error){if(absentV1(error))return[];throw error;}
  return files.filter(name=>/^[A-Za-z0-9_-]{8,32}$/u.test(name)).slice(0,protocol.maximumWaitMarkers).flatMap(name=>{
   const requestPath=join(rebuildFolder,name),request=readRebuildRequest(requestPath,state);return request?.id===name&&!isHostCapacityV1("RebuildRefusal",readJson(rebuildReceiptPath(requestPath)))?[request]:[];
  }).sort((left,right)=>left.askedAtMs-right.askedAtMs||left.id.localeCompare(right.id));
 };
 const observeAll=async():Promise<number|null>=>{
  const attached=readAttached(attachedPath(directory,held.id)),observe=options.observe;
  if(observe===undefined){
   const graphPlans=readGrants(directory,now(),policy,undefined,options.control.signal,false)
    .filter(grant=>grant.id!==held.id&&grant.operation==="graph-plan"&&grant.observedMegabytes!==null);
   const excluded=new Set(graphPlans.flatMap(grant=>readAttached(attachedPath(directory,grant.id))));
   return observeTreeMegabytesV1(pid,undefined,undefined,undefined,undefined,undefined,undefined,{attached,excluded});
  }
  const own=await observe(pid);if(own===null)return null;
  let total=own;for(const worker of attached)total+=await observe(worker).catch(()=>null)??0;return total;
 };
 const activeAt=(at:number):number=>Math.max(0,at-startedAt-lentTotal-(state.lentSinceMs===undefined?0:at-state.lentSinceMs));
 const heartbeat=(tag:HostPhaseV1|undefined=phase)=>{try{const cached=(readJson(join(directory,"sample.json")) as {host?:unknown}|undefined)?.host;if(isHostCapacityV1("Host",cached)&&now()-(cached as HostSampleV1).atMs<=2*policy.wait.heartbeatMilliseconds&&hostUnderPressureV1(policy,cached as HostSampleV1))pressured=true;}catch{}void observeAll().then(observed=>{if(holding&&observed!==null){peak=Math.max(peak,observed);marks.push([activeAt(now()),observed,tag]);if(marks.length>protocol.maximumPeakMarks)marks=marks.filter((_,index)=>index%2===0||index===marks.length-1);publish({...state,observedMegabytes:observed});if(quiet&&options.quietUpgradeRequiresRequest!==true&&state.quiet===true&&observed>state.reservedMegabytes)outgrown=true;}}).catch(()=>undefined);};
 const enter=(next:HostPhaseV1):void=>{
  if(next!=="rebuild"&&state.rebuildBookedAtMs!==undefined&&rebuildRequests().length>0){deferredPhase=next;return;}
  deferredPhase=undefined;
  const at=now(),active=activeAt(at),left=phase;heartbeat(phase);transitions.push({phase:next,atMilliseconds:active});phase=next;
  if(next==="rebuild"){publish({...state,phase:next});return;}
  const {rebuildBookedAtMs,...base}=state;
  let resized:ReturnType<NonNullable<HostCapacityOptionsV1["resize"]>>;try{resized=state.quiet===true?undefined:options.resize?.(next,{jobs:state.jobs,reservedMegabytes:state.reservedMegabytes,activeMilliseconds:active});}catch{resized=undefined;}
  const leaving=state.quiet!==true&&(left==="rebuild"||rebuildBookedAtMs!==undefined)&&resized===undefined?{rampMilliseconds:policy.memory.rampMilliseconds,resumedAtMs:at}:{};
  publish({...base,phase:next,...resized===undefined?{}:{reservedMegabytes:resized.reservedMegabytes,rampMilliseconds:resized.rampMilliseconds,resumedAtMs:at},...leaving});
 };
 const readPhases=():void=>{
  const file=phasesPath(directory,held.id);let size=0;try{size=statSync(file).size;}catch{return;}
  if(size===markBytes||size>protocol.maximumPhaseMarkBytes)return;
  markBytes=size;
  const rows=readFileSync(file,"utf8").split("\n").filter(row=>row!=="");
  for(const row of rows.slice(markLines)){
   markLines++;const next=row.split(" ")[0] as HostPhaseV1;
   if((HOST_PHASES_V1 as readonly string[]).includes(next)&&(next!==phase||next!=="rebuild"&&state.rebuildBookedAtMs!==undefined)&&transitions.length<protocol.maximumPhaseTransitions)enter(next);
  }
 };
 const first=setTimeout(()=>heartbeat(),Math.min(limits.heartbeatFirstMilliseconds,policy.wait.heartbeatMilliseconds)),timer=setInterval(()=>heartbeat(),policy.wait.heartbeatMilliseconds);
 const watch=adopted?undefined:options.orphan??currentHostOrphanWatchV1(),orphaned=watch?setInterval(()=>{if(holding&&!stopped&&watch.lost())watch.stop();},policy.wait.heartbeatMilliseconds):undefined;orphaned?.unref();
 let tableAt=-Infinity,tableRows:ReturnType<typeof processChildrenTableV1>;const treeTable=(at:number)=>{if(at-tableAt>=policy.wait.heartbeatMilliseconds||at<tableAt){tableAt=at;tableRows=(options.tree??processChildrenTableV1)(pid);}return tableRows;};
 const lender=setInterval(()=>{
  try{
   readPhases();
   const at=now(),own=(options.waited??scriptProcessWaitEpisodeV1)(at),lent=state.lentSinceMs!==undefined;
   const marked=(options.markers??readHostCapacityWaitMarkersV1)(directory,held.id),treeSince=marked.size?hostTreeWaitsV1(treeTable(at),pid,marked):0,episode=Math.max(own,treeSince>0?Math.max(1,at-treeSince):0);
   if(state.rebuildBookedAtMs!==undefined)return;
   if(episode===0)checkedChildren=-Infinity;
   if(lent&&episode===0){const{lentSinceMs:since,...rest}=state;lentTotal+=at-since!;publish({...rest,resumedAtMs:at});return;}
   if(Math.min(episode,at-(state.resumedAtMs??state.admittedAtMs))<policy.wait.lendAfterMilliseconds&&!lent)return;
   if(at-checkedChildren>=policy.wait.heartbeatMilliseconds){checkedChildren=at;childless=((options.children??liveChildPidsV1)(pid)?.length??1)===0;}
   const treeWaits=treeSince>0;
   if(!lent&&(childless||treeWaits))publish({...state,lentSinceMs:at});
   else if(lent&&!childless&&!treeWaits){const{lentSinceMs:since,...rest}=state;lentTotal+=at-since!;publish({...rest,resumedAtMs:at});}
  }catch{}
 },policy.wait.pollMilliseconds);
 const watcher=quiet?setInterval(()=>{
  if(state.quiet!==true||upgrading||stopped||!outgrown&&!existsSync(upgradeFile)||now()-attempted<policy.wait.pollMilliseconds)return;
  attempted=now();upgrading=true;
  if(state.lentSinceMs!==undefined){const{lentSinceMs:since,...rest}=state;lentTotal+=attempted-since;publish({...rest,resumedAtMs:attempted});}
  void import("../🔼️upgrade/🟦️.ts").then(module=>module.upgradeHostGrantV1({ledger:Object.freeze({...custody(),readGrants,readWaiters,queuePath,sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,request:Object.freeze({...full!,minimumJobs:full!.preferredJobs}),control:options.control,state:()=>state,publish,requested:()=>outgrown||existsSync(upgradeFile),stopped:()=>stopped||!holding,now,pid,language:resolveHostCapacityLanguageV1(environment),write:options.write??(line=>process.stderr.write(line+"\n")),...options.session===undefined?{}:{session:options.session}})).catch(()=>undefined).finally(()=>{upgrading=false;});
 },protocol.upgradePollMilliseconds):undefined;
 const grower=growable?setInterval(()=>{
  if(growing||upgrading||rebuilding||stopped||!holding||state.quiet===true||state.lentSinceMs!==undefined||state.jobs>=full!.preferredJobs)return;
  growing=true;
  void import("../🔼️upgrade/🟦️.ts").then(module=>module.growHostGrantV1({ledger:Object.freeze({...custody(),readGrants,readWaiters,queuePath,sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,full:full!,state:()=>state,publish,now,pid,language:resolveHostCapacityLanguageV1(environment),write:options.write??(line=>process.stderr.write(line+"\n")),settled:()=>settled,settle:next=>{settled=next;}})).catch(()=>undefined).finally(()=>{growing=false;});
 },policy.wait.heartbeatMilliseconds):undefined;
 const rebuildWatcher=rebuildFolder===undefined?undefined:setInterval(()=>{
  if(rebuilding||growing||upgrading||stopped||!holding||state.quiet===true)return;
  try{
   readPhases();const pending=rebuildRequests(),ask=pending[0];
   if(state.rebuildBookedAtMs!==undefined){
    if(ask===undefined){enter(deferredPhase??(phase==="execution"?phase:"plan"));return;}
    for(const request of pending){const receipt=rebuildReceiptPath(join(rebuildFolder,request.id));if(!existsSync(receipt))writeJson(receipt,request);}
    return;
   }
   if(ask===undefined)return;
   let resized:ReturnType<NonNullable<HostCapacityOptionsV1["resize"]>>;try{resized=options.resize?.("rebuild",{jobs:state.jobs,reservedMegabytes:state.reservedMegabytes,activeMilliseconds:activeAt(now())});}catch{resized=undefined;}
   const requested=():boolean=>readRebuildRequest(join(rebuildFolder,ask.id),state)?.id===ask.id;
   const quote=Object.freeze({...request,id:state.id,minimumJobs:state.jobs,preferredJobs:state.jobs,fixedMegabytes:Math.max(state.reservedMegabytes,resized?.reservedMegabytes??0),perJobMegabytes:0,...resized?.minimumMegabytes===undefined?{}:{minimumMegabytes:resized.minimumMegabytes},cpuPerJob:Math.max(1,Math.ceil(state.cpu/state.jobs)),rampMilliseconds:Math.max(resized?.rampMilliseconds??state.rampMilliseconds??policy.memory.rampMilliseconds,policy.wait.staleMilliseconds),rampSlot:state.rampSlot??false});
   rebuilding=true;
   void import("../🔼️upgrade/🟦️.ts").then(module=>module.upgradeHostGrantV1({ledger:Object.freeze({...custody(),readGrants,readWaiters,queuePath,sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,request:quote,control:options.control,state:()=>state,publish:next=>{if(state.lentSinceMs!==undefined)lentTotal+=now()-state.lentSinceMs;publish(next);},requested,stopped:()=>stopped||!holding,now,pid,language:resolveHostCapacityLanguageV1(environment),write:options.write??(line=>process.stderr.write(line+"\n")),rebuildAskedAtMs:ask.askedAtMs,...options.session===undefined?{}:{session:options.session}})).then(result=>{
    if(typeof result!=="string"&&!stopped&&holding&&requested())writeJson(rebuildReceiptPath(join(rebuildFolder,ask.id)),requireHostCapacityV1<HostRebuildRefusalV1>("RebuildRefusal",{request:ask,...result}));
   }).catch(()=>undefined).finally(()=>{rebuilding=false;});
  }catch{}
 },protocol.upgradePollMilliseconds);
 first.unref();timer.unref();lender.unref();watcher?.unref();grower?.unref();rebuildWatcher?.unref();
 try{
  return await body(Object.freeze({id:held.id,jobs:quiet||growable?full!.preferredJobs:held.jobs,cpu:quiet||growable?full!.preferredJobs*full!.cpuPerJob:held.cpu,reservedMegabytes:held.reservedMegabytes,plan,inherited:false,adopted,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:held.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:"on"})}));
 }catch(error){
  failed=true;failure=error;
  try{parked=options.park!==undefined&&state.quiet!==true&&await options.park(error);}catch{parked=false;}
  throw error;
 }finally{
  stopped=true;clearTimeout(first);clearInterval(timer);clearInterval(lender);clearInterval(watcher);clearInterval(grower);clearInterval(rebuildWatcher);clearInterval(orphaned);process.removeListener("exit",cleanup);
  if(options.learn&&!adopted&&!parked&&peak>0){const ended=now(),active=activeAt(ended),near=marks.find(([,megabytes])=>megabytes*10000>=peak*protocol.peakNearBasisPoints);readPhases();const phases=hostPhaseSegmentsV1(marks,transitions,active);try{await options.learn(Object.freeze({jobs:quiet&&state.quiet===true?full!.preferredJobs:state.jobs,peakMegabytes:peak,activeMilliseconds:active,peakAtMilliseconds:Math.min(near?.[0]??active,active),endedAtMs:ended,...phases.length===0?{}:{phases},...pressured?{pressured:true}:{},...full===undefined?{}:{executed:state.upgradedAtMs!==undefined||outgrown||existsSync(upgradeFile)},status:!failed?"ok" as const:cancelledV1(failure,options.control)?"cancelled" as const:"failed" as const}));}catch{}}
  if(parked)try{await(await import("./🤝️handover/🟦️.ts")).parkHostGrantV1(custody(),{options,path,state,plan,pid,now,cleanup,language:resolveHostCapacityLanguageV1(environment)});holding=false;}catch{cleanup();}else cleanup();
 }
}

