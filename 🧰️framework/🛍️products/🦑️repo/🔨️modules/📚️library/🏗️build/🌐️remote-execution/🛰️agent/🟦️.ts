import {platform as osPlatform} from "node:os";
import type {HostCapacityPolicyV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {admitRemoteExecutionActionV1,buildRemoteExecutionLoadReportV1,readRemoteExecutionCountersV1,readRemoteExecutionHostViewV1,readRemoteExecutionToolchainV1,remoteExecutionOriginV1,remoteExecutionPlatformIdentityV1,runRemoteExecutionReporterV1,type RemoteExecutionAdmissionV1,type RemoteExecutionHostViewV1,type RemoteExecutionLoadReportV1,type RemoteExecutionReporterEventV1,type RemoteExecutionWorkerStateV1} from "../🌡️load/🟦️.ts";
import {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyV1} from "../🌡️load/🔎️why/🟦️.ts";
import {runRemoteExecutionWorkerV1,type RemoteExecutionWorkerEventV1,type RemoteExecutionWorkerOptionsV1} from "../👷️worker/🟦️.ts";
import {remoteExecutionHostPropertiesV1,type RemoteExecutionPropertyV1} from "../🟦️.ts";
import {remoteExecutionDiskSlotsV1,remoteExecutionFreeBytesV1,type RemoteExecutionWorkerDiskV1} from "./💽️disk/🟦️.ts";

/** ⚙️ A build agent: the worker and the load reporter of one machine, one process. */
export type RemoteExecutionAgentOptionsV1=Readonly<Omit<RemoteExecutionWorkerOptionsV1,"admit"|"observe"|"origin"|"capacity"|"properties"|"platform">&{
 root:string;environment?:Readonly<Record<string,string|undefined>>;origin?:string;extraProperties?:readonly RemoteExecutionPropertyV1[];loadIntervalSeconds:number;worker?:boolean;drain?:AbortSignal;why?:RemoteExecutionWorkerWhyV1;retryWorkerSeconds?:number;disk?:RemoteExecutionWorkerDiskV1;freeBytes?:()=>number|undefined;
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

/** 🛰️ Runs the build agent of this machine until its signal aborts: it reads the toolchain identity once, offers it (and the operating system and architecture) as worker properties, admits every remote action through the host capacity decision of this machine, and reports its load to the scheduler every `loadIntervalSeconds` as long as it runs, whether or not the worker could join. With `disk` the free space of the work directory is part of every admission: the worker runs as many actions at once as the space allows (none below the reserve) and says `disk-full` with the numbers while it is refusing for that reason, so a machine that runs short of disk neither leaves the scheduler nor needs a restart to serve again: the next admission, two seconds later, sees the space. A confinement the policy requires but the host lacks stops the worker (and the agent) with an error. With `ready` a machine lends only while it answers true (the toolchain behind its offer is installed): otherwise every admission refuses with `not-ready` and every report says `no-toolchain` without headroom, so the scheduler places nothing there. */
export async function runRemoteExecutionAgentV1(options:RemoteExecutionAgentOptionsV1):Promise<void>{
 const environment=options.environment??process.env,now=options.now??Date.now,origin=options.origin??remoteExecutionOriginV1(options.root,environment);
 const toolchain=await readRemoteExecutionToolchainV1(options.root,environment),identity=remoteExecutionPlatformIdentityV1(osPlatform(),process.arch,toolchain.toolchain,toolchain.target);
 let cached:{at:number;view:Promise<RemoteExecutionHostViewV1>}|undefined,state:RemoteExecutionWorkerStateV1={state:"absent",concurrency:options.concurrency,running:0,sandbox:null,confinement:null,network:null};
 let why:RemoteExecutionWorkerWhyV1|undefined=options.worker===false?options.why:remoteExecutionWorkerWhyV1("starting"),diskWhy:RemoteExecutionWorkerWhyV1|undefined;
 const view=():Promise<RemoteExecutionHostViewV1>=>{
  if(cached&&now()-cached.at<1000)return cached.view;
  const fresh=(options.view??(()=>readRemoteExecutionHostViewV1(environment,options.policy)))();
  cached={at:now(),view:fresh};fresh.catch(()=>{if(cached?.view===fresh)cached=undefined;});
  return fresh;
 };
 let draining=false;
 const admit=async(running:number):Promise<RemoteExecutionAdmissionV1>=>{
  if(draining)return {admit:false,reason:"draining"};
  if(options.ready!==undefined&&!options.ready())return {admit:false,reason:"not-ready"};
  const space=options.disk===undefined?undefined:remoteExecutionDiskSlotsV1({freeBytes:(options.freeBytes??(()=>remoteExecutionFreeBytesV1(options.directory)))(),concurrency:options.concurrency,disk:options.disk});
  diskWhy=space!==undefined&&running>=space.slots?remoteExecutionWorkerWhyV1("disk-full","",{free:((space.freeBytes??0)/2**30).toFixed(1),path:options.directory.split(/[\\/]/u).filter(Boolean).slice(-3).join("/"),slots:space.slots,concurrency:options.concurrency,oneAction:(space.oneActionBytes/2**30).toFixed(1)}):undefined;
  if(diskWhy!==undefined)return {admit:false,reason:"disk"};
  return admitRemoteExecutionActionV1({view:await view(),running,concurrency:options.concurrency,nowMs:now(),...options.policy?{policy:options.policy}:{}});
 };
 const report=async():Promise<RemoteExecutionLoadReportV1>=>{
  const built=buildRemoteExecutionLoadReportV1({origin,identity,view:await view(),worker:why!==undefined&&state.state==="absent"?{...state,why}:state,counters:readRemoteExecutionCountersV1(options.root),nowMs:now(),...options.policy?{policy:options.policy}:{}});
  if(options.ready!==undefined&&!options.ready())return {...built,headroom:false,worker:{...built.worker,...built.worker.state==="idle"||built.worker.state==="busy"?{state:"refusing" as const}:{},why:remoteExecutionWorkerWhyV1("no-toolchain","the compiler of the offer is gone or has no target")}};
  return diskWhy!==undefined&&state.state==="refusing"?{...built,headroom:false,worker:{...built.worker,state:"refusing",why:diskWhy}}:built;
 };
 const controller=new AbortController(),abort=():void=>{controller.abort();};
 options.signal.addEventListener("abort",abort,{once:true});
 if(options.signal.aborted)abort();
 const stopDraining=drainWhenIdleV1(options.drain,()=>{draining=true;return state.running;},()=>{controller.abort();});
 const capacity=await view().then(first=>({cores:Math.max(1,first.host.cores),memoryMegabytes:Math.max(1,first.host.totalMegabytes)}),()=>undefined);
 const properties=[...remoteExecutionHostPropertiesV1(osPlatform(),process.arch,toolchain),...options.extraProperties??[]];
 const reporter=runRemoteExecutionReporterV1({scheduler:options.scheduler,token:options.token,fetch:options.fetch,intervalSeconds:options.loadIntervalSeconds,signal:controller.signal,report,...options.onReport?{onEvent:options.onReport}:{}});
 const remember=(next:RemoteExecutionWorkerStateV1):void=>{state=next;if(next.state!=="absent")why=undefined;};
 const onEvent=(event:RemoteExecutionWorkerEventV1):void=>{
  if(event.stage==="registered")why=undefined;
  else if(event.stage==="reconnecting"){why=remoteExecutionWorkerWhyOfMessageV1(event.detail);state={...state,state:"absent",running:0};}
  options.onEvent?.(event);
 };
 const runWorker=async():Promise<void>=>{
  for(;;){
   try{await runRemoteExecutionWorkerV1({...options,origin,properties,signal:controller.signal,admit,observe:remember,onEvent,...capacity?{capacity}:{}});return;}
   catch(error){
    if(options.retryWorkerSeconds===undefined||controller.signal.aborted){controller.abort();throw error;}
    why=remoteExecutionWorkerWhyOfMessageV1((error as Error).message);state={...state,state:"absent",running:0};
    await new Promise<void>(done=>{const timer=setTimeout(done,options.retryWorkerSeconds!*1000);controller.signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
    if(controller.signal.aborted)return;
   }
  }
 };
 const worker=options.worker===false?Promise.resolve():runWorker();
 try{await Promise.all([reporter,worker]);}finally{stopDraining();options.signal.removeEventListener("abort",abort);controller.abort();}
}
