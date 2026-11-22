import {remoteExecutionMatchPropertiesV1,remoteExecutionPlatformMatchesV1,type RemoteExecutionPropertyV1} from "../../🟦️.ts";
import type {RemoteExecutionLoadReportV1} from "../../🌡️load/🟦️.ts";

/** 📋️ A machine's row in the live load table: its last report, when it arrived, and the counters the scheduler keeps for it. */
export type RemoteExecutionLoadEntryV1={report:RemoteExecutionLoadReportV1;receivedAtMs:number;offloadedIn:number;offloadedOut:number;refused:number};

/** 🧮️ The live table of the fleet: the last load report of every machine, expired after `expireMs` of silence. Pure bookkeeping over an injected clock. */
export class RemoteExecutionLoadTableV1{
 private readonly entries=new Map<string,RemoteExecutionLoadEntryV1>();
 private readonly kept=new Map<string,Pick<RemoteExecutionLoadEntryV1,"offloadedIn"|"offloadedOut"|"refused">>();
 readonly expireMs:number; private readonly clock:()=>number; constructor(expireMs:number,clock:()=>number=Date.now){this.expireMs=expireMs;this.clock=clock;}

 /** 📥️ Takes a report; the scheduler's own counters of the machine survive it. */
 accept(report:RemoteExecutionLoadReportV1):void{
  const counters=this.entries.get(report.origin)??this.kept.get(report.origin)??{offloadedIn:0,offloadedOut:0,refused:0};
  this.entries.set(report.origin,{report,receivedAtMs:this.clock(),offloadedIn:counters.offloadedIn,offloadedOut:counters.offloadedOut,refused:counters.refused});
 }

 /** 🧹️ Drops machines that were silent for `expireMs`; their counters are kept for the day they return. */
 expire():readonly string[]{
  const now=this.clock(),gone:string[]=[];
  for(const [origin,entry] of this.entries)if(now-entry.receivedAtMs>this.expireMs){this.kept.set(origin,{offloadedIn:entry.offloadedIn,offloadedOut:entry.offloadedOut,refused:entry.refused});this.entries.delete(origin);gone.push(origin);}
  return gone;
 }

 get(origin:string):RemoteExecutionLoadEntryV1|undefined{this.expire();return this.entries.get(origin);}
 all():readonly RemoteExecutionLoadEntryV1[]{this.expire();return [...this.entries.values()].sort((left,right)=>left.report.origin<right.report.origin?-1:1);}
 count(origin:string,field:"offloadedIn"|"offloadedOut"|"refused"):void{
  const entry=this.entries.get(origin);
  if(entry){entry[field]++;return;}
  const kept=this.kept.get(origin)??{offloadedIn:0,offloadedOut:0,refused:0};kept[field]++;this.kept.set(origin,kept);
 }
 refuse(origin:string):void{
  const entry=this.entries.get(origin);
  if(entry)this.entries.set(origin,{...entry,report:{...entry.report,headroom:false}});
 }
}

/** 👷️ A registered worker as the placement sees it; `committedMegabytes` is the peak that the scheduler already placed on it and that its next load report may not show yet. */
export type PlacementWorkerV1=Readonly<{id:string;name:string;origin:string;properties:readonly RemoteExecutionPropertyV1[];concurrency:number;running:number;waiting:boolean;draining:boolean;refusedUntilMs:number;committedMegabytes?:number}>;

/** 🎬️ A queued action as the placement sees it: the properties it requires, the workers (by origin, else name) that already failed it, who asked for it and the megabytes it needs at its peak (0 when it declares none). */
export type PlacementOperationV1=Readonly<{id:string;platform:readonly RemoteExecutionPropertyV1[];tried:ReadonlySet<string>;requester:string|undefined;peakMegabytes?:number}>;

/** 🧭️ The identity of a worker in `tried` and in the requester check: its origin. */
export const placementKeyV1=(worker:Pick<PlacementWorkerV1,"origin"|"name">):string=>worker.origin===""?worker.name:worker.origin;

/** 🧬️ The properties an action requires: its own (control properties stripped) and, for what it leaves open, the platform key of its requester (operating system, architecture, toolchain build and target), so a unit never runs on another toolchain build by omission. */
export function placementRequiredV1(platform:readonly RemoteExecutionPropertyV1[],requester:RemoteExecutionLoadEntryV1|undefined):readonly RemoteExecutionPropertyV1[]{
 const own=remoteExecutionMatchPropertiesV1(platform),named=new Set(own.map(row=>row.name)),identity=requester?.report.platform;
 if(!identity)return own;
 const add=(name:string,value:string|null):RemoteExecutionPropertyV1[]=>value===null||named.has(name)?[]:[{name,value}];
 return [...own,...add("Arch",identity.arch),...add("OSFamily",identity.os),...add("semio-toolchain",identity.toolchain),...add("semio-target",identity.target)];
}

/** 🚦️ Whether a worker has headroom to be given one more action now: not draining, a free slot, not in its refusal cool-down, and (when its machine reports) neither critical pressure nor its own admission saying no. An action with a declared peak (`peakMegabytes`) also needs the machine's reported free memory, less what the scheduler already placed on it, to cover that peak, so a machine that has not reported never gets one. A machine that has not reported yet has headroom for every other action. */
export function placementHasHeadroomV1(worker:PlacementWorkerV1,entry:RemoteExecutionLoadEntryV1|undefined,nowMs:number,peakMegabytes:number=0):boolean{
 if(worker.draining||worker.running>=worker.concurrency||nowMs<worker.refusedUntilMs)return false;
 if(entry===undefined)return peakMegabytes<=0;
 return entry.report.pressure!=="critical"&&entry.report.headroom&&(peakMegabytes<=0||entry.report.freeMegabytes-(worker.committedMegabytes??0)>=peakMegabytes);
}

/** 📏️ How loaded the machine of a worker is, lower is better: its local work (running and queued builds beside the remote actions it reports) plus the remote actions the scheduler knows about, per core, a raised pressure level and the load average. */
export function placementScoreV1(worker:PlacementWorkerV1,entry:RemoteExecutionLoadEntryV1|undefined):number{
 if(entry===undefined)return 1000+worker.running;
 const report=entry.report,cores=Math.max(1,report.capacity.cores),local=Math.max(0,report.running-report.worker.running)+report.queued;
 return (local+worker.running)/cores+(report.pressure==="warn"?1:0)+0.25*(report.load??0)/cores;
}

/** 🤝️ Whether the worker may take the action by platform, exclusion and requester (headroom is a separate question). */
export function placementCompatibleV1(operation:PlacementOperationV1,worker:PlacementWorkerV1,required:readonly RemoteExecutionPropertyV1[]):boolean{
 const key=placementKeyV1(worker);
 return !operation.tried.has(key)&&key!==operation.requester&&remoteExecutionPlatformMatchesV1(required,worker.properties);
}

/** 🎯️ One decision of a dispatch round. */
export type PlacementAssignmentV1=Readonly<{operation:string;worker:string}>;

/** 🗓️ Plans one dispatch round: the queue is served first come first served, each action goes to the least loaded waiting worker that is compatible with it (platform key including the toolchain build, not a worker that already failed it, not its requester) and has headroom; an action nobody can take stays queued without blocking the ones behind it; a waiting worker receives at most one action per round. Pure. */
export function planDispatchV1(queue:readonly PlacementOperationV1[],workers:readonly PlacementWorkerV1[],table:Pick<RemoteExecutionLoadTableV1,"get">,nowMs:number):readonly PlacementAssignmentV1[]{
 const free=new Map(workers.filter(worker=>worker.waiting).map(worker=>[worker.id,worker])),plan:PlacementAssignmentV1[]=[];
 for(const operation of queue){
  if(free.size===0)break;
  const required=placementRequiredV1(operation.platform,operation.requester===undefined?undefined:table.get(operation.requester));
  let best:PlacementWorkerV1|undefined,bestScore=Infinity;
  for(const worker of free.values()){
   if(!placementCompatibleV1(operation,worker,required)||!placementHasHeadroomV1(worker,table.get(worker.origin),nowMs,operation.peakMegabytes??0))continue;
   const score=placementScoreV1(worker,table.get(worker.origin));
   if(score<bestScore||score===bestScore&&best!==undefined&&(worker.running<best.running||worker.running===best.running&&worker.name<best.name)){best=worker;bestScore=score;}
  }
  if(best===undefined)continue;
  plan.push({operation:operation.id,worker:best.id});free.delete(best.id);
 }
 return plan;
}

/** 🔢️ How the registered workers stand for one action right now: all of them, those the platform key lets run it (never its requester, never one that failed it), those with headroom, and those that are both, the candidates, with the free action slots (`slots`) of the candidates, the width a build sizes its jobs to. The one counting behind every answer to "is another machine idle": the check at submission and the answer the build door asks for. */
export function placementIdleCountV1(operation:PlacementOperationV1,workers:readonly PlacementWorkerV1[],table:Pick<RemoteExecutionLoadTableV1,"get">,nowMs:number):Readonly<{workers:number;compatible:number;headroom:number;candidates:number;slots:number}>{
 const required=placementRequiredV1(operation.platform,operation.requester===undefined?undefined:table.get(operation.requester));
 let compatible=0,headroom=0,candidates=0,slots=0;
 for(const worker of workers){const fits=placementCompatibleV1(operation,worker,required),room=placementHasHeadroomV1(worker,table.get(worker.origin),nowMs,operation.peakMegabytes??0);if(fits)compatible++;if(room)headroom++;if(fits&&room){candidates++;slots+=Math.max(1,worker.concurrency-worker.running);}}
 return Object.freeze({workers:workers.length,compatible,headroom,candidates,slots});
}

/** 🔭️ Whether any registered worker other than the requester could take the action right now (waiting or not): the question `semio-offload=if-idle` asks at submission. */
export function placementIdleElsewhereV1(operation:PlacementOperationV1,workers:readonly PlacementWorkerV1[],table:Pick<RemoteExecutionLoadTableV1,"get">,nowMs:number):boolean{
 return placementIdleCountV1(operation,workers,table,nowMs).candidates>0;
}

/** 🔁️ Whether any registered worker could still take the action although it failed on `tried` (used to fail back to the requester at once instead of waiting for a worker that does not exist). */
export function placementAnyCandidateV1(operation:PlacementOperationV1,workers:readonly PlacementWorkerV1[],table:Pick<RemoteExecutionLoadTableV1,"get">):boolean{
 const required=placementRequiredV1(operation.platform,operation.requester===undefined?undefined:table.get(operation.requester));
 return workers.some(worker=>placementCompatibleV1(operation,worker,required));
}
