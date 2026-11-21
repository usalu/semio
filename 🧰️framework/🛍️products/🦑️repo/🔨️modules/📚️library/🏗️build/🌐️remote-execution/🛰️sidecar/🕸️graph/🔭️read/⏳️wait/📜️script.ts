import{hostCapacityDirectoryV1,ownsHostRebuildAskV1,ownsQueuedHostRebuildV1,readHostCapacityStatusV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import{systemAncestryV1}from"../../../../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🧭️ancestry/🟦️.ts";
import{BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1}from"../../../../../🌡️host-capacity/🎛️control/🟦️.ts";
import type{GraphReadCapacityWaitV1}from"../🟦️.ts";

/** ⚖️ Checks the evaluator’s decision for the exact graph rebuild request. */
export function queuedGraphRebuildDecisionV1(id:string,decisions:Awaited<ReturnType<typeof readHostCapacityStatusV1>>["evaluation"]["decisions"]):boolean{
 return decisions.some(row=>row.id===id&&row.verdict==="wait"&&row.reason!=="never"&&row.reason!=="starved");
}

/** 🏗️ Observes only a live, birth-bound capacity wait for the detached reader's own rebuild. */
export async function probeGraphRebuildWaitV1(reader:number,runner:number,startedMs:number,environment:Readonly<Record<string,string|undefined>>=process.env,prior?:Readonly<Pick<GraphReadCapacityWaitV1,"id"|"scope"|"arrivedAtMs">>):Promise<GraphReadCapacityWaitV1|undefined>{
 if(!Number.isSafeInteger(reader)||reader<1||!Number.isSafeInteger(runner)||runner<1||!Number.isSafeInteger(startedMs)||startedMs<1)return undefined;
 const directory=hostCapacityDirectoryV1(environment),{control,dispose}=createCapacityControlV1(),policy=BUILD_HOST_CAPACITY_POLICY_V1;
 try{
  const status=await readHostCapacityStatusV1({directory,control,policy,environment}),current=Date.now();
  const ancestry=new Map<number,ReturnType<typeof systemAncestryV1>>();
  const owns=(pid:number,start:string):boolean=>{
   const lineage=ancestry.get(pid)??systemAncestryV1(process.platform,pid);
   ancestry.set(pid,lineage);
   const index=lineage.findIndex(row=>row.pid===reader),birth=lineage[0]?.started;
   return birth!==undefined&&(birth===start||process.platform==="win32"&&Date.parse(birth)===Date.parse(start))&&index>=0&&lineage[index+1]?.pid===runner;
  };
  if(prior!==undefined){
   const grant=status.grants.find(row=>row.id===prior.id&&row.rebuildScope===prior.scope&&row.operation==="show"&&row.admittedAtMs>=startedMs);
   const bookedAtMs=grant?.rebuildBookedAtMs;
   if(grant!==undefined&&bookedAtMs!==undefined&&bookedAtMs>=prior.arrivedAtMs&&bookedAtMs<=current&&ownsHostRebuildAskV1(directory,grant,prior.arrivedAtMs,owns))
    return {...prior,bookedAtMs,reservedMegabytes:grant.reservedMegabytes};
  }
  for(const waiter of status.waiters){
   if(waiter.arrivedAtMs<startedMs||waiter.arrivedAtMs>current||waiter.request.operation!=="show"||waiter.rebuild===undefined||!queuedGraphRebuildDecisionV1(waiter.request.id,status.evaluation.decisions))continue;
   const grant=status.grants.find(row=>row.id===waiter.request.id&&row.pid===waiter.pid&&row.operation==="show"&&row.phase==="plan"&&row.admittedAtMs>=startedMs);
   if(grant===undefined||waiter.request.fixedMegabytes<=grant.reservedMegabytes)continue;
   if(ownsQueuedHostRebuildV1(directory,grant,waiter,owns))return {id:grant.id,scope:waiter.rebuild,arrivedAtMs:waiter.arrivedAtMs,reservedMegabytes:grant.reservedMegabytes};
  }
  return undefined;
 }finally{dispose();}
}

if(import.meta.main){
 const [read,runner,start,id,scope,arrival]=process.argv.slice(2),arrivedAtMs=Number(arrival);
 const prior=id&&scope&&Number.isSafeInteger(arrivedAtMs)?{id,scope,arrivedAtMs}:undefined;
 try{const row=await probeGraphRebuildWaitV1(Number(read),Number(runner),Number(start),process.env,prior);if(row!==undefined)process.stdout.write(JSON.stringify(row));}catch{}
}
