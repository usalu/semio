import {decideSubmitV1,schedulerTextV1,schedulerTicketStateV1,type JobRequestV1,type SchedulerEventV1,type SchedulerExitV1,type SchedulerProgressV1,type SchedulerStateV1,type SchedulerTicketStateV1} from "../🟦️.ts";
import type {SchedulerStoreV1} from "../💾️store/🟦️.ts";

/** 🎟️ Client side of the queue/ticket protocol: submit one request, wait with heartbeats, run the work when this ticket is the runner of the (possibly shared) job, or watch the shared run of another agent. Cancellation by signal or by `repo scheduler cancel` stops the work through its `AbortSignal`.
 * @see ../🧬️schema/🔣️.json events and exits */
export type ScheduledContextV1=Readonly<{jobId:string;signal:AbortSignal;progress:(progress:SchedulerProgressV1)=>void}>;
export type ScheduledResultV1=Readonly<{exit:"ok"|"failed"|"memory-exceeded"|"cancelled";observedPeakMegabytes?:number}>;
export type ScheduledOutcomeV1=Readonly<{exit:SchedulerExitV1;shared:boolean;jobId:string}>;
export type ScheduledOptionsV1=Readonly<{signal?:AbortSignal;onQueued?:(position:number)=>void;sleep?:(milliseconds:number)=>Promise<void>}>;
/** 🪢️ Set in the environment of a running job, so every nested `repo` call of its tasks runs inside the job instead of queueing behind it. */
export const SCHEDULED_JOB_V1="SEMIO_SCHEDULED_JOB";
const both=(key:string,values:Record<string,string|number>={}):string=>schedulerTextV1("en",key,values)+"\n"+schedulerTextV1("de",key,values);

/** ▶️ Runs `work` once for all agents that ask for the same `request.key`; resolves with the exit shared by every ticket of the job. */
export async function runScheduledV1(store:SchedulerStoreV1,request:JobRequestV1,work:(context:ScheduledContextV1)=>Promise<ScheduledResultV1>,options:ScheduledOptionsV1={}):Promise<ScheduledOutcomeV1>{
 const policy=store.policy,sleep=options.sleep??((milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds))),ticket=request.id;
 const submitted=await store.transact((state,at)=>{const decision=decideSubmitV1(policy,state,request,at);return{events:decision.events,result:decision.outcome};});
 if(submitted.kind==="busy")throw Error(both("error.busy",{queued:policy.maxQueued}));
 if(submitted.kind==="invalid")throw Error(both("error.invalid",{details:submitted.details??""}));
 if(submitted.kind==="duplicate")throw Error(both("error.duplicate",{ticket}));
 const jobId=submitted.jobId!;
 const look=(extra:(state:SchedulerStateV1,at:number)=>readonly SchedulerEventV1[]=()=>[]):Promise<SchedulerTicketStateV1>=>store.transact((state,at)=>({events:[{type:"heartbeat",at,ticket},...extra(state,at)],result:schedulerTicketStateV1(policy,state,ticket,at)}));
 let aborted=false;const cancel=async():Promise<void>=>{if(aborted)return;aborted=true;await store.transact((state,at)=>({events:[{type:"cancelled",at,ticket}],result:undefined}));};
 options.signal?.addEventListener("abort",()=>{void cancel();},{once:true});
 if(options.signal?.aborted)await cancel();
 let reported=0,ranHere=false;
 for(;;){
  const current=await look();
  if(current.status==="finished")return{exit:current.exit!,shared:!ranHere,jobId};
  if(current.status==="cancelled")return{exit:"cancelled",shared:false,jobId};
  if(current.status==="unknown")throw Error(both("error.lost"));
  if(current.status==="queued"){if(current.position!==undefined&&current.position!==reported){reported=current.position;options.onQueued?.(reported);}await sleep(policy.pollMs);continue;}
  if(current.role==="runner"){
   ranHere=true;const controller=new AbortController();if(current.stopping||aborted)controller.abort();
   let latest:SchedulerProgressV1|undefined,sent:SchedulerProgressV1|undefined,stop=false;
   const beat=(async()=>{while(!stop){await sleep(policy.pollMs);if(stop)break;const row=await look((_state,at)=>latest&&latest!==sent?[{type:"progressed",at,ticket,progress:(sent=latest)}]:[]);if(row.stopping)controller.abort();}})();
   let result:ScheduledResultV1;
   try{result=await work({jobId,signal:controller.signal,progress:progress=>{latest=progress;}});}
   catch(error){stop=true;await beat;await store.transact((state,at)=>({events:[{type:"completed",at,jobId,exit:"failed"}],result:undefined}));throw error;}
   stop=true;await beat;
   const exit=result.exit;
   await store.transact((state,at)=>({events:[{type:"completed",at,jobId,exit,...result.observedPeakMegabytes===undefined?{}:{observedPeakMegabytes:result.observedPeakMegabytes}}],result:undefined}));
   return{exit,shared:false,jobId};
  }
  await sleep(policy.pollMs);
 }
}
