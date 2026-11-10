import {concurrentEditV1,retryOnConcurrentEditSyncV1,type ChurnOptionsV1} from "../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

import {spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {appendFileSync,existsSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {homedir,uptime} from "node:os";
import {isAbsolute,join} from "node:path";
import {beginAdmissionWaitV1,liveChildPidsV1,onScriptProcessWaitEdgeV1,scriptProcessWaitEpisodeV1,waitingDelayV1 as delay} from "../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../🔒️leases/🎛️control/🟦️.ts";
import {decideHostHandoverV1,describeHostBlockersV1,hostBlockersV1,hostBootAnchorV1,type HostBootAnchorV1,describeHostHandoverRefusalV1,describeHostReasonV1,evaluateHostCapacityV1,withUnmanagedHostV1,followHostGrantPlanV1,formatHostCapacityV1,hostCapacityErrorV1,isCurrentHostGrantV1,isHostCapacityV1,planHostCapacityV1,requireHostCapacityV1,resolveHostCapacityLanguageV1,softHostReasonV1,type HostCapacityPolicyV1,type HostEvaluationV1,type HostGrantV1,type HostGrowthSettleV1,type HostHandoverV1,type HostPlanV1,type HostReasonV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";
import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";
import {systemAncestryV1,WINDOWS_PROCESS_LISTING_V1} from "../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts";
import {hostDeclarationKeyV1} from "../🪞️profile/🟦️.ts";
import {HOST_PHASES_V1,hostPhaseSegmentsV1,type HostPhaseTransitionV1,type HostPhaseV1} from "../🪜️phases/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {absentV1,alive,currentSample,hostBootAtV1,hostCapacityDirectoryV1,modified,names,prepare,readGrants,readJson,readWaiters,remove,writeJson,type HostCapacityStatusV1} from "./📖️status/🟦️.ts";
export {hostBootAtV1,hostCapacityDirectoryV1,readHostCapacityStatusV1,type HostCapacityStatusV1} from "./📖️status/🟦️.ts";

const protocol=schema.definitions.Protocol.const,limits=schema.definitions.Limits.const;

/** 🆔️ Mints one unguessable request id. */
export function newHostCapacityIdV1():string{return randomBytes(16).toString("base64url");}

/** 🎟️ What an admitted operation holds and the environment its children inherit. */
export type HostCapacityHandleV1=Readonly<{id:string;jobs:number;cpu:number;reservedMegabytes:number;plan:HostPlanV1;inherited:boolean;adopted:boolean;bypassed:boolean;environment:Readonly<Record<string,string>>}>;

import type {HostCapacityOptionsV1,HostCapacityPortsV1,HostLedgerPrimitivesV1,HostProcessIdentityV1} from "./📋️contract/🟦️.ts";
export type {HostCapacityOptionsV1,HostCapacityPortsV1,HostLedgerPrimitivesV1,HostProcessIdentityV1,HostRunV1} from "./📋️contract/🟦️.ts";

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
 catch(error){if(["ENOENT","EEXIST"].includes((error as NodeJS.ErrnoException).code??""))return false;throw error;}
}
/** 🧷️ The ledger lock primitive for laws that need to pause inside it. */
export const takeHostLockV1=takeLock;
/** 🗝️ The fenced release of the ledger lock (`releaseLock`), for laws. */
export const releaseHostLockV1=releaseLock;
/** 🧿️ The ownership check of the ledger lock (`holdsLock`), for laws. */
export const holdsHostLockV1=holdsLock;


/** ⏳️ The progress line of a waiting request, naming what it stands behind (`hostBlockersV1`). */
function waitingLine(language:"en"|"de",request:HostRequestV1,host:HostSampleV1,evaluation:HostEvaluationV1,running:number,seconds:number,policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):string{
 const cap=Math.max(0,host.totalMegabytes-evaluation.floorMegabytes),position=evaluation.decisions.findIndex(row=>row.id===request.id),mine=evaluation.decisions[position]!,before=evaluation.decisions.slice(0,position),spare=evaluation.headroomMegabytes-before.reduce((sum,row)=>sum+row.reservedMegabytes,0);
 return formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:describeHostReasonV1(language,mine.reason as Exclude<HostReasonV1,"fits"|"reduced"|"override">),need:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,cap),spare,floor:evaluation.floorMegabytes,load:host.load===null?"n/a":host.load,running,ahead:position,behind:describeHostBlockersV1(language,hostBlockersV1(policy,ledger,waiters,evaluation,request.id,nowMs))});
}

/** 🚪️ Admits one operation against the whole machine and runs its body under the grant: it waits in a fair machine-wide queue with progress lines and cancellation, reserves memory and cores while the body runs and always gives them back. A nested invocation runs its body under an inherited grant at once, with the plan the holder carried and its jobs and cores (for a quiet grant those of the full request, never the quiet row's single job: a forecast changes what a run reserves, never what it may use); it never grows a quiet grant itself, an executor of the run asks when work has to run (`requestHostCapacityUpgradeV1`). */
export async function withHostCapacityV1<T>(options:HostCapacityOptionsV1,body:(handle:HostCapacityHandleV1)=>Promise<T>):Promise<T>{
 const environment=options.environment??process.env,policy=options.policy,request=requireHostCapacityV1<HostRequestV1>("Request",options.request),language=resolveHostCapacityLanguageV1(environment),write=options.write??(line=>process.stderr.write(line+"\n")),now=options.now??Date.now,pid=options.pid??process.pid;
 if(options.upgrade!==undefined)requireHostCapacityV1<HostRequestV1>("Request",options.upgrade);
 if(request.quiet===true&&(options.upgrade===undefined||options.upgrade.id!==request.id||options.upgrade.quiet===true))throw hostCapacityErrorV1("quietUpgradeRequired",{},environment);
 if(!policy.enabled||environment[protocol.switchEnvironment]==="off"){
  if(policy.enabled)write(formatHostCapacityV1(language,"bypassed",{switch:protocol.switchEnvironment}));
  const sizing=request.quiet===true?options.upgrade!:request,plan=planHostCapacityV1(policy,{totalMegabytes:1048576},sizing,sizing.preferredJobs);
  return body(Object.freeze({id:request.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:0,plan,inherited:false,adopted:false,bypassed:true,environment:Object.freeze({})}));
 }
 const inheritedId=environment[protocol.grantEnvironment];
 if(inheritedId!==undefined&&!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(inheritedId))throw hostCapacityErrorV1("inheritedIdentifier",{},environment);
 const directory=prepare(options.directory),inheritedPath=inheritedId?grantPath(directory,inheritedId):undefined,inherited=inheritedPath?readJson(inheritedPath):undefined,at=now();
 if(inheritedPath&&isCurrentHostGrantV1({grant:inherited,expectedId:inheritedId,pidAlive:isHostCapacityV1("Grant",inherited)&&alive((inherited as HostGrantV1).pid),nowMs:at,bootAtMs:hostBootAtV1(directory,at),modifiedAtMs:modified(inheritedPath)},policy)){
  const grant=inherited as HostGrantV1,carried=readHostCapacityPlanV1(environment),plan=carried?.id===grant.id?followHostGrantPlanV1(carried,grant):requireHostCapacityV1<HostPlanV1>("Plan",{version:1,id:grant.id,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:grant.reservedMegabytes,fixedMegabytes:0,floorMegabytes:0});
  return body(Object.freeze({id:grant.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:grant.reservedMegabytes,plan,inherited:true,adopted:false,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:grant.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:grant.pid===pid||grant.pid===(options.ppid??process.ppid)?"on":"off"})}));
 }
 const offered=environment[protocol.handoverEnvironment];
 if(offered!==undefined){const claimed=await(await import("./🤝️handover/🟦️.ts")).claimHostGrantHandoverV1(custody(),options,directory,offered,request,language,now,pid);if(claimed)return holdHostGrantV1(options,claimed,undefined,request,directory,pid,body);}
 if(names(directory,"queue").length>=policy.wait.maximumWaiters)throw Error(schema.definitions.Messages.const[language].errors.queueFull);
 const arrivedAtMs=now(),waiterPath=queuePath(directory,arrivedAtMs,request.id),record:HostWaiterV1={request,arrivedAtMs,pid,...options.session===undefined?{}:{session:options.session.address,weight:options.session.weight}};
 writeJson(waiterPath,record);
 let grant:HostGrantV1|undefined,admittedHost:HostSampleV1|undefined,lastProgress=-Infinity;
 const endAdmissionWait=beginAdmissionWaitV1();
 try{
  while(!grant){
   checkResourceLeaseControl(options.control);
   const host=await currentSample(directory,policy,options.control,options,environment);
   try{utimesSync(waiterPath,new Date(),new Date());}catch{}
   let progressAt:number|undefined;
   if(takeLock(directory,now(),policy,pid)){
    try{
     const at=now(),ledger=readGrants(directory,at,policy),waiters=readWaiters(directory,at,policy);
     if(!waiters.some(row=>row.request.id===request.id)){writeJson(waiterPath,record);waiters.push(record);}
     const evaluation=evaluateHostCapacityV1(policy,withUnmanagedHostV1(host,ledger),ledger,waiters,at),mine=evaluation.decisions.find(row=>row.id===request.id)!;
     if(mine.verdict==="admit"){
      if(!holdsLock(directory,pid))continue;
      grant=requireHostCapacityV1<HostGrantV1>("Grant",{id:request.id,operation:request.operation,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,admittedAtMs:at,observedMegabytes:null,pid,bootAtMs:hostBootAtV1(directory,at),...options.session===undefined?{}:{session:options.session.address},...request.label===undefined?{}:{label:request.label},...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},...request.rampSlot===undefined?{}:{rampSlot:request.rampSlot},...request.quiet===true?{quiet:true,arrivedAtMs}:{}});
      writeJson(grantPath(directory,request.id),grant);remove(waiterPath);admittedHost=host;
      const seconds=Math.round((at-arrivedAtMs)/1000);
      if(mine.reason==="override")write(formatHostCapacityV1(language,"override",{operation:request.operation,seconds,reason:describeHostReasonV1(language,softHostReasonV1(policy,host)??"load")}));
      else if(mine.reason==="reduced")write(formatHostCapacityV1(language,options.upgrade!==undefined&&options.upgrade.perJobMegabytes>0?"reducedGrows":"reduced",{operation:request.operation,jobs:mine.jobs,preferred:request.preferredJobs}));
      else write(formatHostCapacityV1(language,"admitted",{operation:request.operation,jobs:mine.jobs,reserved:mine.reservedMegabytes,available:host.availableMegabytes,floor:evaluation.floorMegabytes}));
     }else if(at-lastProgress>=policy.wait.progressMilliseconds){
      lastProgress=at;write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000),policy,ledger,waiters,at));progressAt=at;
     }
    }finally{releaseLock(directory,pid);}
   }
   if(grant)break;
   if(progressAt!==undefined){await options.control.publish({resource:protocol.resource,mode:"shared",elapsedMs:progressAt-arrivedAtMs});checkResourceLeaseControl(options.control);await options.control.yieldContinuation();}
   const budget=checkResourceLeaseControl(options.control);
   try{await delay(Math.max(1,Math.min(policy.wait.pollMilliseconds*(0.75+Math.random()*0.5),budget??Infinity)),undefined,{signal:options.control.signal});}catch(error){throw options.control.signal.aborted?options.control.signal.reason:error;}
  }
 }catch(error){remove(waiterPath);throw error;}finally{endAdmissionWait();}
 return holdHostGrantV1(options,grant!,admittedHost!,request,directory,pid,body);
}

/** 🛑️ Whether a failure is a cancellation: the wait control was aborted, the error carries the cancelled code, or it is an abort. */
const cancelledV1=(error:unknown,control:ResourceLeaseControl):boolean=>control.signal.aborted||(error as {code?:unknown;name?:unknown}|null)?.code==="cancelled"||(error as {name?:unknown}|null)?.name==="AbortError";

const custody=():HostLedgerPrimitivesV1=>Object.freeze({protocol,takeLock,releaseLock,holdsLock,grantPath,readJson,writeJson,remove,alive,bootAt:hostBootAtV1,modified,delay,checkResourceLeaseControl});

/** 🔐️ Holds one admitted grant while its body runs: heartbeats what the process tree holds, lends the reservation while the process only waits in a queue, gives the reservation back when the body ends (or keeps it parked when the body failed in a way a restart cures and the caller says so). An adopted grant carries the plan its owner admitted. A run that marks its phases (`markHostCapacityPhaseV1`) is cut into phase segments by the readings it was observed at, and its reservation follows the `resize` port at each boundary; the segments go to `learn`. A quiet grant whose tree holds more than its quiet reservation grows as if an executor had asked: work runs that no executor announced (the executors, which wait for the growth, are the planner, a graph rebuild and the birth of every Bazel action that really runs); the run goes on meanwhile and is learned as executed. */
async function holdHostGrantV1<T>(options:HostCapacityOptionsV1,held:HostGrantV1,admittedHost:HostSampleV1|undefined,request:HostRequestV1,directory:string,pid:number,body:(handle:HostCapacityHandleV1)=>Promise<T>):Promise<T>{
 const policy=options.policy,now=options.now??Date.now,environment=options.environment??process.env,path=grantPath(directory,held.id),adopted=admittedHost===undefined,carried=adopted?readHostCapacityPlanV1(environment):undefined;
 const plan=adopted?carried?.id===held.id?carried:requireHostCapacityV1<HostPlanV1>("Plan",{version:1,id:held.id,jobs:held.jobs,cpu:held.cpu,memoryMegabytes:held.reservedMegabytes,fixedMegabytes:0,floorMegabytes:0}):planHostCapacityV1(policy,admittedHost!,held.quiet===true&&options.upgrade!==undefined?options.upgrade:request,held.quiet===true&&options.upgrade!==undefined?options.upgrade.preferredJobs:held.jobs);
 const full=options.upgrade,quiet=held.quiet===true&&full!==undefined,growable=full!==undefined&&!adopted&&full.perJobMegabytes>0,upgradeFile=hostUpgradePathV1(directory,held.id);
 let growing=false,outgrown=false,settled:HostGrowthSettleV1|undefined,upgrading=false,stopped=false,attempted=-Infinity,holding=true,state:HostGrantV1=held,childless=false,checkedChildren=-Infinity,parked=false,peak=0,lentTotal=0,failure:unknown,failed=false,marks:(readonly [number,number,HostPhaseV1|undefined])[]=[],phase:HostPhaseV1|undefined,transitions:HostPhaseTransitionV1[]=[],markLines=0,markBytes=0;
 const startedAt=now();
 const cleanup=()=>{holding=false;try{unlinkSync(path);}catch{}try{unlinkSync(attachedPath(directory,held.id));}catch{}try{unlinkSync(phasesPath(directory,held.id));}catch{}try{unlinkSync(upgradeFile);}catch{}try{const prefix=`${held.id}${protocol.waitSuffix}`;for(const name of readdirSync(join(directory,"grants")))if(name.startsWith(prefix))unlinkSync(join(directory,"grants",name));}catch{}};
 process.once("exit",cleanup);
 const publish=(next:HostGrantV1):void=>{state=next;if(holding)writeJson(path,state);};
 const observeAll=async():Promise<number|null>=>{const observe=options.observe??observeTreeMegabytesV1,own=await observe(pid);if(own===null)return null;let total=own;for(const attached of readAttached(attachedPath(directory,held.id)))total+=await observe(attached).catch(()=>null)??0;return total;};
 const activeAt=(at:number):number=>Math.max(0,at-startedAt-lentTotal-(state.lentSinceMs===undefined?0:at-state.lentSinceMs));
 const heartbeat=(tag:HostPhaseV1|undefined=phase)=>{void observeAll().then(observed=>{if(holding&&observed!==null){peak=Math.max(peak,observed);marks.push([activeAt(now()),observed,tag]);if(marks.length>protocol.maximumPeakMarks)marks=marks.filter((_,index)=>index%2===0||index===marks.length-1);publish({...state,observedMegabytes:observed});if(quiet&&state.quiet===true&&observed>state.reservedMegabytes)outgrown=true;}}).catch(()=>undefined);};
 const enter=(next:HostPhaseV1):void=>{
  const at=now(),active=activeAt(at);heartbeat(phase);transitions.push({phase:next,atMilliseconds:active});phase=next;
  let resized:ReturnType<NonNullable<HostCapacityOptionsV1["resize"]>>;try{resized=state.quiet===true?undefined:options.resize?.(next,{jobs:state.jobs,reservedMegabytes:state.reservedMegabytes,activeMilliseconds:active});}catch{resized=undefined;}
  publish({...state,phase:next,...resized===undefined?{}:{reservedMegabytes:resized.reservedMegabytes,rampMilliseconds:resized.rampMilliseconds,resumedAtMs:at}});
 };
 const readPhases=():void=>{
  const file=phasesPath(directory,held.id);let size=0;try{size=statSync(file).size;}catch{return;}
  if(size===markBytes||size>protocol.maximumPhaseMarkBytes)return;
  markBytes=size;
  const rows=readFileSync(file,"utf8").split("\n").filter(row=>row!=="");
  for(const row of rows.slice(markLines)){
   markLines++;const next=row.split(" ")[0] as HostPhaseV1;
   if((HOST_PHASES_V1 as readonly string[]).includes(next)&&next!==phase&&transitions.length<protocol.maximumPhaseTransitions)enter(next);
  }
 };
 const first=setTimeout(()=>heartbeat(),Math.min(limits.heartbeatFirstMilliseconds,policy.wait.heartbeatMilliseconds)),timer=setInterval(()=>heartbeat(),policy.wait.heartbeatMilliseconds);
 let tableAt=-Infinity,tableRows:ReturnType<typeof processChildrenTableV1>;const treeTable=(at:number)=>{if(at-tableAt>=policy.wait.heartbeatMilliseconds||at<tableAt){tableAt=at;tableRows=(options.tree??processChildrenTableV1)(pid);}return tableRows;};
 const lender=setInterval(()=>{
  try{
   readPhases();
   const at=now(),own=(options.waited??scriptProcessWaitEpisodeV1)(at),lent=state.lentSinceMs!==undefined;
   const marked=(options.markers??readHostCapacityWaitMarkersV1)(directory,held.id),treeSince=marked.size?hostTreeWaitsV1(treeTable(at),pid,marked):0,episode=Math.max(own,treeSince>0?Math.max(1,at-treeSince):0);
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
  if(growing||upgrading||stopped||!holding||state.quiet===true||state.lentSinceMs!==undefined||state.jobs>=full!.preferredJobs)return;
  growing=true;
  void import("../🔼️upgrade/🟦️.ts").then(module=>module.growHostGrantV1({ledger:Object.freeze({...custody(),readGrants,readWaiters,queuePath,sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,full:full!,state:()=>state,publish,now,pid,language:resolveHostCapacityLanguageV1(environment),write:options.write??(line=>process.stderr.write(line+"\n")),settled:()=>settled,settle:next=>{settled=next;}})).catch(()=>undefined).finally(()=>{growing=false;});
 },policy.wait.heartbeatMilliseconds):undefined;
 first.unref();timer.unref();lender.unref();watcher?.unref();grower?.unref();
 try{
  return await body(Object.freeze({id:held.id,jobs:quiet||growable?full!.preferredJobs:held.jobs,cpu:quiet||growable?full!.preferredJobs*full!.cpuPerJob:held.cpu,reservedMegabytes:held.reservedMegabytes,plan,inherited:false,adopted,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:held.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:"on"})}));
 }catch(error){
  failed=true;failure=error;
  try{parked=options.park!==undefined&&state.quiet!==true&&await options.park(error);}catch{parked=false;}
  throw error;
 }finally{
  stopped=true;clearTimeout(first);clearInterval(timer);clearInterval(lender);clearInterval(watcher);clearInterval(grower);process.removeListener("exit",cleanup);
  if(options.learn&&!adopted&&!parked&&peak>0){const ended=now(),active=activeAt(ended),near=marks.find(([,megabytes])=>megabytes*10000>=peak*protocol.peakNearBasisPoints);readPhases();const phases=hostPhaseSegmentsV1(marks,transitions,active);try{await options.learn(Object.freeze({jobs:quiet&&state.quiet===true?full!.preferredJobs:state.jobs,peakMegabytes:peak,activeMilliseconds:active,peakAtMilliseconds:Math.min(near?.[0]??active,active),endedAtMs:ended,...phases.length===0?{}:{phases},...full===undefined?{}:{executed:state.upgradedAtMs!==undefined||outgrown||existsSync(upgradeFile)},status:!failed?"ok" as const:cancelledV1(failure,options.control)?"cancelled" as const:"failed" as const}));}catch{}}
  if(parked)try{await(await import("./🤝️handover/🟦️.ts")).parkHostGrantV1(custody(),{options,path,state,plan,pid,now,language:resolveHostCapacityLanguageV1(environment)});holding=false;}catch{cleanup();}else cleanup();
 }
}

