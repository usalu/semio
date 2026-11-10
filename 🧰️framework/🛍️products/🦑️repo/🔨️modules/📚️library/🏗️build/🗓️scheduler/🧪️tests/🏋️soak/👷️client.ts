import {appendFileSync} from "node:fs";
import {join} from "node:path";
import {SCHEDULER_POLICY_V1,type JobRequestV1,type SchedulerPolicyV1} from "../../🟦️.ts";
import {createFileSchedulerStoreV1} from "../../💾️store/🟦️.ts";
import {runScheduledV1} from "../../🎟️ticket/🟦️.ts";

/** 👷️ One real client process of the soak: waits for the common start, runs equal unique jobs (fairness phase), then shared-key and cancelling jobs (dedupe and cancellation phase), journaling every submission, execution and result for the driver.
 * usage: bun 👷️client.ts <directory> <agent> <startAtMs> <uniqueRounds> <chaosRounds> */
const[directory,agent,startAt,unique,chaos]=process.argv.slice(2) as [string,string,string,string,string];
const policy={...SCHEDULER_POLICY_V1,maxConcurrent:3,maxPerAgent:1,pollMs:20,compactEveryEvents:64,maxFinished:32} as SchedulerPolicyV1;
const store=createFileSchedulerStoreV1(directory,policy,{host:()=>({budgetMegabytes:100000})}),journal=(name:string,row:unknown)=>appendFileSync(join(directory,`${name}.${agent}.jsonl`),JSON.stringify(row)+"\n");
const request=(ticket:string,key:string,depth:number):JobRequestV1=>({id:ticket,agent,workspace:"soak",class:"compile",priority:"agent",depth,key,estimate:{class:"phase:compile",floorMegabytes:0,limitMegabytes:512}});
const sleep=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
let seed=[...agent].reduce((sum,char)=>sum*31+char.charCodeAt(0),7);const random=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
async function one(ticket:string,key:string,depth:number,cancelAfter?:number):Promise<void>{
 journal("submitted",{ticket,key});
 const controller=new AbortController();if(cancelAfter!==undefined)setTimeout(()=>controller.abort(),cancelAfter);
 try{
  const outcome=await runScheduledV1(store,request(ticket,key,depth),async context=>{
   journal("executed",{ticket,key});
   await new Promise<void>(resolve=>{const timer=setTimeout(resolve,40+Math.floor(random()*30));context.signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});});
   return{exit:context.signal.aborted?"cancelled":"ok",observedPeakMegabytes:300};
  },{signal:controller.signal,sleep});
  journal("results",{ticket,key,exit:outcome.exit,shared:outcome.shared});
 }catch(error){journal("results",{ticket,key,error:String((error as Error).message)});}
}
await sleep(Math.max(0,Number(startAt)-Date.now()));
const started=Date.now();
for(let round=0;round<Number(unique);round++)await one(`${agent}-u${round}`,`unique:${agent}:${round}`,round%3);
journal("timing",{agent,elapsedMs:Date.now()-started});
for(let round=0;round<Number(chaos);round++){
 const shared=(round+agent.length)%2===0,cancel=random()<0.25?5+Math.floor(random()*80):undefined;
 await one(`${agent}-c${round}`,shared?`shared:${round%4}`:`own:${agent}:${round}`,round%5,cancel);
}
