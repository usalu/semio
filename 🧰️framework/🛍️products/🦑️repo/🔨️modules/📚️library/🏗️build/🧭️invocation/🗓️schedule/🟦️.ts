import {createHash,randomUUID} from "node:crypto";
import {currentAgentSessionV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {openHostSchedulerStoreV1,scheduledEstimateV1} from "../../🗓️scheduler/📈️learn/🟦️.ts";
import {runScheduledV1,SCHEDULED_JOB_V1,type ScheduledContextV1,type ScheduledResultV1} from "../../🗓️scheduler/🎟️ticket/🟦️.ts";
import {schedulerTextV1,type JobRequestV1} from "../../🗓️scheduler/🟦️.ts";
import type {SchedulerStoreV1} from "../../🗓️scheduler/💾️store/🟦️.ts";

/** 🔑️ The dedupe key of a job: the same command over the same planned content, whatever agent asks. */
export function scheduledKeyV1(args:readonly string[],generationKey:string):string{
 const delimiter=args.indexOf("--"),head=delimiter<0?args:args.slice(0,delimiter);
 return head.slice(0,2).join(" ").slice(0,200)+"#"+createHash("sha256").update(JSON.stringify(args)).digest("hex").slice(0,12)+"@"+generationKey.slice(0,16);
}

/** 🗓️ Runs one heavy invocation through the host-wide scheduler: dedupe by key, fair order by agent, admission by the learned estimate, cancellation by signal; nested calls of a running job run inside it. */
export async function runScheduledInvocationV1(options:Readonly<{args:readonly string[];generationKey:string;workspace:string;environment:NodeJS.ProcessEnv;signal?:AbortSignal;write:(line:string)=>void;work:(context:ScheduledContextV1)=>Promise<void>;store?:SchedulerStoreV1;depth?:number}>):Promise<{exit:string;shared:boolean}>{
 const{args,environment,write}=options;
 if(environment[SCHEDULED_JOB_V1]!==undefined){await options.work({jobId:environment[SCHEDULED_JOB_V1]!,signal:options.signal??new AbortController().signal,progress:()=>undefined});return{exit:"ok",shared:false};}
 const language=resolveLanguageV1(["en","de"] as const,args,environment).language,session=currentAgentSessionV1({environment}),store=options.store??openHostSchedulerStoreV1(environment);
 const className=args[0]==="inputs"?"phase:plan":"phase:compile",estimate=await scheduledEstimateV1(className,512,environment);
 const request:JobRequestV1={id:randomUUID(),agent:session.agent.id,workspace:options.workspace,class:"compile",priority:"agent",depth:options.depth??0,key:scheduledKeyV1(args,options.generationKey),weight:Math.max(1,Math.round(session.budget.weight)),estimate};
 const outcome=await runScheduledV1(store,request,async(context):Promise<ScheduledResultV1>=>{
  const previous=environment[SCHEDULED_JOB_V1];environment[SCHEDULED_JOB_V1]=context.jobId;
  try{await options.work(context);return{exit:"ok"};}
  catch(error){if(context.signal.aborted)return{exit:"cancelled"};throw error;}
  finally{if(previous===undefined)delete environment[SCHEDULED_JOB_V1];else environment[SCHEDULED_JOB_V1]=previous;}
 },{...options.signal?{signal:options.signal}:{},onQueued:position=>write(schedulerTextV1(language,"queuedAt",{position}))});
 return{exit:outcome.exit,shared:outcome.shared};
}
