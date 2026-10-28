import{writeSync}from"node:fs";
import{workerData}from"node:worker_threads";
import{fillTestOrphansMessageV1,testOrphansMessagesV1}from"../👶️child/🟦️.ts";
import{parentGoneV1}from"./🟦️.ts";

/** 👷️ The worker thread of the child guard: while the parent lives it only sleeps; the moment the parent is gone it names the reason (the heavy modules load only now) and ends this whole process tree, the root last, whatever the main thread is blocked in. */
const{parent,pid,interval}=workerData as Readonly<{parent:number;pid:number;interval:number}>;
let ending=false;
setInterval(async()=>{
 if(ending||!parentGoneV1(parent))return;ending=true;
 try{
  const{resolveHostCapacityLanguageV1}=await import("../../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts");
  writeSync(2,fillTestOrphansMessageV1(testOrphansMessagesV1[resolveHostCapacityLanguageV1(process.env)].parentGone,{parent,pid})+"\n");
 }catch{}
 try{const{terminateOwnedProcessTree}=await import("../../../../🪓️termination/🟦️.ts");terminateOwnedProcessTree(pid);}catch{}
 process.kill(pid,"SIGKILL");
},interval);
