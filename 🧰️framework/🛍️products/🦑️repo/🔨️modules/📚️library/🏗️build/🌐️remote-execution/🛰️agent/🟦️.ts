import {platform as osPlatform} from "node:os";
import {join} from "node:path";
import type {HostCapacityPolicyV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {remoteExecutionPressureGateV1,admitRemoteExecutionActionV1,type RemoteExecutionHeldUnitV1,evaluateRemoteExecutionAdmissionV1,buildRemoteExecutionLoadReportV1,REMOTE_EXECUTION_ADMISSION_POLICY_V1,remoteExecutionActionMegabytesV1,readRemoteExecutionCountersV1,readRemoteExecutionHostViewV1,readRemoteExecutionToolchainV1,remoteExecutionOriginV1,remoteExecutionPlatformIdentityV1,runRemoteExecutionReporterV1,type RemoteExecutionAdmissionV1,type RemoteExecutionHostViewV1,type RemoteExecutionLoadReportV1,type RemoteExecutionReporterEventV1,type RemoteExecutionWorkerStateV1} from "../🌡️load/🟦️.ts";
import {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyV1} from "../🌡️load/🔎️why/🟦️.ts";
import {retireRemoteExecutionWorkV1,runRemoteExecutionWorkerV1,type RemoteExecutionThreadsAskV1,type RemoteExecutionUnitAskV1,type RemoteExecutionWorkerEventV1,type RemoteExecutionWorkerOptionsV1} from "../👷️worker/🟦️.ts";
import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,type RemoteExecutionPropertyV1} from "../🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../🌡️host-capacity/🎛️control/🟦️.ts";
import {remoteExecutionCodegenThreadsV1} from "../👷️worker/🧵️codegen/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../🚀️offload/📐️policy/🟦️.ts";
import {remoteExecutionDiskSlotsV1,remoteExecutionFreeBytesV1,type RemoteExecutionWorkerDiskV1} from "./💽️disk/🟦️.ts";

/** ⏱️ How often a worker whose work volume holds fewer slots than it offers collects the finished action sandboxes of dead workers before it refuses for disk (at most once a minute, so a volume that is full of live work is not swept in a loop). */
export const REMOTE_EXECUTION_COLLECT_INTERVAL_MILLISECONDS_V1=60000;

/** ⚙️ A build agent: the worker and the load reporter of one machine, one process. */
export type RemoteExecutionAgentOptionsV1=Readonly<Omit<RemoteExecutionWorkerOptionsV1,"admit"|"observe"|"origin"|"capacity"|"properties"|"platform">&{
 root:string;code?:string;environment?:Readonly<Record<string,string|undefined>>;origin?:string;extraProperties?:readonly RemoteExecutionPropertyV1[];loadIntervalSeconds:number;worker?:boolean;drain?:AbortSignal;onRunning?:(running:number)=>void;why?:RemoteExecutionWorkerWhyV1;retryWorkerSeconds?:number;disk?:RemoteExecutionWorkerDiskV1;collect?:()=>Promise<void>;freeBytes?:()=>number|undefined;hostFreeBytes?:()=>number|undefined;
 view?:()=>Promise<RemoteExecutionHostViewV1>;ready?:()=>boolean;policy?:HostCapacityPolicyV1;now?:()=>number;onReport?:(event:RemoteExecutionReporterEventV1)=>void;
}>;

/** 🚰️ Calls `done` once `drain` has fired and `busy()` reports no work in flight, checking every `pollMilliseconds`; without a drain signal nothing is ever called. Returns the function that stops watching. */
export function drainWhenIdleV1(drain:AbortSignal|undefined,busy:()=>number,done:()=>void,pollMilliseconds=500):()=>void{
 if(drain===undefined)return ()=>undefined;
 let timer:ReturnType<typeof setInterval>|undefined;
 const stop=():void=>{if(timer!==undefined){clearInterval(timer);timer=undefined;}drain.removeEventListener("abort",start);};
 const check=():void=>{if(busy()===0){stop();done();}};
 function start():void{if(timer!==undefined)return;timer=setInterval(check,pollMilliseconds);check();}
 if(drain.aborted)start();else drain.addEventListener("abort",start,{once:true});
 return stop;
}

/** 🛰️ Runs the build agent of this machine until its signal aborts: it reads the toolchain identity once, offers it (and the operating system and architecture) as worker properties, admits every remote action through the host capacity decision of this machine, and reports its load to the scheduler every `loadIntervalSeconds` as long as it runs, whether or not the worker could join. With `disk` the free space of the work directory and of the checkout (the host volume) is part of every admission: the worker runs as many actions at once as the space allows (none below the reserve) and says `disk-full` with the numbers while it is refusing for that reason, so a machine that runs short of disk neither leaves the scheduler nor needs a restart to serve again: the next admission, two seconds later, sees the space. A confinement the policy requires but the host lacks stops the worker (and the agent) with an error. With `ready` a machine lends only while it answers true (the toolchain behind its offer is installed): otherwise every admission refuses with `not-ready` and every report says `no-toolchain` without headroom, so the scheduler places nothing there. */
export async function runRemoteExecutionAgentV1(options:RemoteExecutionAgentOptionsV1):Promise<void>{
 const environment=options.environment??process.env,now=options.now??Date.now,origin=options.origin??remoteExecutionOriginV1(options.root,environment);
 const workerEnabled=options.worker!==false;
 const toolchain=await readRemoteExecutionToolchainV1(options.root,environment),identity=remoteExecutionPlatformIdentityV1(osPlatform(),process.arch,toolchain.toolchain,toolchain.target);
 let cached:{at:number;view:Promise<RemoteExecutionHostViewV1>}|undefined,state:RemoteExecutionWorkerStateV1={state:"absent",concurrency:options.concurrency,running:0,sandbox:null,confinement:null,network:null};
 let why:RemoteExecutionWorkerWhyV1|undefined=workerEnabled?remoteExecutionWorkerWhyV1("starting"):options.why,diskWhy:RemoteExecutionWorkerWhyV1|undefined;
 const view=():Promise<RemoteExecutionHostViewV1>=>{
  if(cached&&now()-cached.at<1000)return cached.view;
  const fresh=(options.view??(()=>readRemoteExecutionHostViewV1(environment,options.policy)))();
  cached={at:now(),view:fresh};fresh.catch(()=>{if(cached?.view===fresh)cached=undefined;});
  return fresh;
 };
 let draining=false,collectedAt=-Infinity;
 const collect=options.collect??(async()=>{await retireRemoteExecutionWorkV1(join(options.directory,"work"),{ownerlessSeconds:readRemoteExecutionPolicyV1().worker.ownerlessWorkSeconds});});
 let held:readonly RemoteExecutionHeldUnitV1[]=[];
 const unitsOf=(rows:readonly RemoteExecutionHeldUnitV1[]):readonly RemoteExecutionHeldUnitV1[]=>rows.map(row=>row.megabytes>0?row:{...row,megabytes:remoteExecutionActionMegabytesV1(options.policy)});
 const admit=async(running:number,unit:RemoteExecutionUnitAskV1):Promise<RemoteExecutionAdmissionV1>=>{
  held=unitsOf(unit.held);
  if(draining)return {admit:false,reason:"draining"};
  if(options.ready!==undefined&&!options.ready())return {admit:false,reason:"not-ready"};
  const measure=()=>options.disk===undefined?undefined:remoteExecutionDiskSlotsV1({freeBytes:(options.freeBytes??(()=>remoteExecutionFreeBytesV1(options.directory)))(),hostFreeBytes:(options.hostFreeBytes??(()=>remoteExecutionFreeBytesV1(options.root)))(),concurrency:options.concurrency,disk:options.disk});
  let space=measure();
  if(space!==undefined&&space.volume==="work"&&space.slots<options.concurrency&&now()-collectedAt>=REMOTE_EXECUTION_COLLECT_INTERVAL_MILLISECONDS_V1){
   collectedAt=now();
   try{await collect();space=measure();}catch{space=measure();}
  }
  diskWhy=space!==undefined&&running>=space.slots?remoteExecutionWorkerWhyV1("disk-full","",{free:((space.freeBytes??0)/2**30).toFixed(1),path:(space.volume==="host"?options.root:options.directory).split(/[\\/]/u).filter(Boolean).slice(-3).join("/"),slots:space.slots,concurrency:options.concurrency,oneAction:(space.oneActionBytes/2**30).toFixed(1)}):undefined;
  if(diskWhy!==undefined)return {admit:false,reason:"disk"};
  return admitRemoteExecutionActionV1({view:await view(),running,concurrency:options.concurrency,nowMs:now(),megabytes:unit.stage==="task"&&unit.megabytes>0?unit.megabytes:unit.stage==="poll"?REMOTE_EXECUTION_ADMISSION_POLICY_V1.smallestMegabytes:remoteExecutionActionMegabytesV1(options.policy),held,...options.policy?{policy:options.policy}:{}});
 };
 const threads=async(ask:RemoteExecutionThreadsAskV1):Promise<number>=>{
  const current=await view(),peak=ask.peakMegabytes>0?ask.peakMegabytes:remoteExecutionActionMegabytesV1(options.policy);
  const {spareMegabytes}=evaluateRemoteExecutionAdmissionV1({view:current,running:ask.held.length,concurrency:options.concurrency,nowMs:now(),megabytes:peak,held:unitsOf(ask.held),...options.policy?{policy:options.policy}:{}});
  const gate=remoteExecutionPressureGateV1(current.host,peak,(options.policy??BUILD_HOST_CAPACITY_POLICY_V1).swap.activeBasisPointsPerSecond);
  return remoteExecutionCodegenThreadsV1({cores:current.host.cores,load:current.host.load,others:ask.others,spareMegabytes,peakMegabytes:peak,pressured:gate==="pressure-warn"||gate==="pressure-critical"},readRemoteExecutionPolicyV1().worker.codegen);
 };
 const report=async():Promise<RemoteExecutionLoadReportV1>=>{
  const built=buildRemoteExecutionLoadReportV1({origin,identity,view:await view(),worker:why!==undefined&&state.state==="absent"?{...state,why}:state,counters:readRemoteExecutionCountersV1(options.root),...options.code===undefined?{}:{code:options.code},nowMs:now(),...held.length===state.running?{held}:{},...options.policy?{policy:options.policy}:{}});
  if(options.ready!==undefined&&!options.ready())return workerEnabled?{...built,headroom:false,worker:{...built.worker,...built.worker.state==="idle"||built.worker.state==="busy"?{state:"refusing" as const}:{},why:remoteExecutionWorkerWhyV1("no-toolchain","the compiler of the offer is gone or has no target")}}:{...built,headroom:false};
  return diskWhy!==undefined&&state.state==="refusing"?{...built,headroom:false,worker:{...built.worker,state:"refusing",why:diskWhy}}:built;
 };
 const controller=new AbortController(),abort=():void=>{controller.abort();};
 options.signal.addEventListener("abort",abort,{once:true});
 if(options.signal.aborted)abort();
 const stopDraining=drainWhenIdleV1(options.drain,()=>{draining=true;return state.running;},()=>{controller.abort();});
 const capacity=await view().then(first=>({cores:Math.max(1,first.host.cores),memoryMegabytes:Math.max(1,first.host.totalMegabytes)}),()=>undefined);
 const properties=[...remoteExecutionHostPropertiesV1(osPlatform(),process.arch,toolchain),{name:OFFLOAD_POLICY_V1.property.pin,value:origin},...options.extraProperties??[]];
 const reporter=runRemoteExecutionReporterV1({scheduler:options.scheduler,token:options.token,fetch:options.fetch,intervalSeconds:options.loadIntervalSeconds,signal:controller.signal,report,...options.onReport?{onEvent:options.onReport}:{}});
 const remember=(next:RemoteExecutionWorkerStateV1):void=>{state=next;options.onRunning?.(next.running);if(next.state!=="absent")why=undefined;};
 const onEvent=(event:RemoteExecutionWorkerEventV1):void=>{
  if(event.stage==="registered")why=undefined;
  else if(event.stage==="reconnecting"){why=remoteExecutionWorkerWhyOfMessageV1(event.detail);state={...state,state:"absent",running:0};}
  options.onEvent?.(event);
 };
 const runWorker=async():Promise<void>=>{
  for(;;){
   try{await runRemoteExecutionWorkerV1({threads,...options,origin,properties,signal:controller.signal,admit,observe:remember,onEvent,...capacity?{capacity}:{}});return;}
   catch(error){
    if(options.retryWorkerSeconds===undefined||controller.signal.aborted){controller.abort();throw error;}
    why=remoteExecutionWorkerWhyOfMessageV1((error as Error).message);state={...state,state:"absent",running:0};
    await new Promise<void>(done=>{const timer=setTimeout(done,options.retryWorkerSeconds!*1000);controller.signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
    if(controller.signal.aborted)return;
   }
  }
 };
 const worker=workerEnabled?runWorker():Promise.resolve();
 try{await Promise.all([reporter,worker]);}finally{stopDraining();options.signal.removeEventListener("abort",abort);controller.abort();}
}
