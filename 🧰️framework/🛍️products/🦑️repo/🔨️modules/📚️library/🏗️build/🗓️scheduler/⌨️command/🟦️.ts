import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {jobBreachLearnedPeakV1,jobPolicyV1,renderJobOutcomeV1,runLimitedJobV1} from "../../🧱️job-limits/🟦️.ts";
import {selectJobLimiterV1} from "../../🧱️job-limits/🎚️select/🟦️.ts";
import {randomUUID} from "node:crypto";
import {currentAgentSessionV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {runScheduledV1,SCHEDULED_JOB_V1} from "../🎟️ticket/🟦️.ts";
import {formatSchedulerSnapshotV1,schedulerTextV1,type SchedulerLanguageV1} from "../🟦️.ts";
import type {SchedulerStoreV1} from "../💾️store/🟦️.ts";
import {openHostSchedulerStoreV1} from "../📈️learn/🟦️.ts";

/** 🗓️ `repo scheduler status [--json]`, `repo scheduler cancel <ticket>` and `repo scheduler run [--key=k] [--priority=p] [--depth=n] [--class=c] [--lane=l] [--peak-mb=n] -- <command...>`: read the host-wide queue, cancel one ticket (its runner then stops through the abort signal) or run any command through the queue (the generic replacement of per-ticket slot scripts).
 * @see ../🧬️schema/🔣️.json the texts */
export type SchedulerCommandPortsV1=Readonly<{store?:SchedulerStoreV1;limited?:typeof runLimitedJobV1}>;
const both=(key:string,values:Record<string,string|number>={}):string=>schedulerTextV1("en",key,values)+"\n"+schedulerTextV1("de",key,values);

const flag=(args:readonly string[],name:string):string|undefined=>args.find(argument=>argument.startsWith(`--${name}=`))?.slice(name.length+3);

/** ▶️ Runs one CLI invocation and returns its exit code. */
export async function runSchedulerCliV1(_root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1,ports:SchedulerCommandPortsV1={}):Promise<number>{
 const language:SchedulerLanguageV1=resolveLanguageV1(["en","de"] as const,args,environment).language,store=ports.store??openHostSchedulerStoreV1(environment);
 if(args[0]==="status"&&args.slice(1).every(argument=>argument==="--json"||argument.startsWith("--lang="))){
  const snapshot=await store.snapshot();
  if(args.includes("--json"))write(JSON.stringify(snapshot,null,2));else for(const line of formatSchedulerSnapshotV1(language,snapshot))write(line);
  return 0;
 }
 if(args[0]==="cancel"&&args[1]!==undefined&&!args[1].startsWith("-")){
  const ticket=args[1],found=await store.transact((state,at)=>state.ticketJob[ticket]===undefined?{events:[],result:false}:{events:[{type:"cancelled",at,ticket}],result:true});
  write(schedulerTextV1(language,found?"cancelled":"cancelUnknown",{ticket}));
  return found?0:1;
 }
 if(args[0]==="run"&&args.includes("--")){
  const split=args.indexOf("--"),command=args.slice(split+1),options=args.slice(1,split);
  if(command.length===0)throw Error(both("error.syntax"));
  const session=currentAgentSessionV1({environment}),peak=Number(flag(options,"peak-mb")??1024),key=flag(options,"key")??command.join(" "),controller=new AbortController(),forward=():void=>controller.abort();
  process.once("SIGINT",forward);process.once("SIGTERM",forward);
  const outcome=await runScheduledV1(store,{id:randomUUID(),agent:session.agent.id,workspace:session.worktree,class:(flag(options,"class")??"compile") as "compile",priority:(flag(options,"priority")??"agent") as "agent",depth:Number(flag(options,"depth")??0),key,...flag(options,"lane")===undefined?{}:{lane:flag(options,"lane")!},weight:Math.max(1,Math.round(session.budget.weight)),estimate:{class:flag(options,"estimate-class")??"phase:compile",floorMegabytes:0,limitMegabytes:peak}},async context=>{
   const policy=jobPolicyV1(peak),outcome=await(ports.limited??runLimitedJobV1)({job:key,policy,limiter:selectJobLimiterV1(policy).limiter,command:command[0]!,args:command.slice(1),signal:context.signal,progressMilliseconds:1000,environment:{...environment,[SCHEDULED_JOB_V1]:context.jobId}});
   const text=renderJobOutcomeV1(outcome,key,language);if(text!==undefined)write(text);
   return{exit:outcome.kind,observedPeakMegabytes:jobBreachLearnedPeakV1(outcome)};
  },{signal:controller.signal,onQueued:position=>write(schedulerTextV1(language,"queuedAt",{position}))});
  process.off("SIGINT",forward);process.off("SIGTERM",forward);
  return outcome.exit==="ok"?0:1;
 }
 throw Error(both("error.syntax"));
}
