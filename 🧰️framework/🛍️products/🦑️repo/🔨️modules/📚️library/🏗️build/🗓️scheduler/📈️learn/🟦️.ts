import {recordHostPhasesV1,updateHostEstimateStoreV1,hostCapacityDirectoryV1,readHostEstimateStoreV1,hostClassBoundV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../🌡️host-capacity/🎛️control/🟦️.ts";
import {jobEstimateV1} from "../../🧱️job-limits/🟦️.ts";
import {schedulerLearnedPeaksV1,SCHEDULER_POLICY_V1} from "../🟦️.ts";
import {createFileSchedulerStoreV1,schedulerDirectoryV1,type SchedulerStoreV1} from "../💾️store/🟦️.ts";

/** 📈️ Opens the host-wide scheduler store whose finished jobs teach the learned memory estimates of their class (`schedulerLearnedPeaksV1`), so the next reservation and hard limit of that class follow what the jobs of all agents really peaked at. */
export function openHostSchedulerStoreV1(environment:NodeJS.ProcessEnv=process.env,now:()=>number=Date.now):SchedulerStoreV1{
 const directory=hostCapacityDirectoryV1(environment);
 return createFileSchedulerStoreV1(schedulerDirectoryV1(environment),SCHEDULER_POLICY_V1,{now,onEvents:(events,state)=>{
  const peaks=schedulerLearnedPeaksV1(events,state);if(peaks.length===0)return;
  void updateHostEstimateStoreV1(directory,BUILD_HOST_CAPACITY_POLICY_V1,store=>peaks.reduce((next,peak)=>recordHostPhasesV1(BUILD_HOST_CAPACITY_POLICY_V1,next,[peak.class],"scheduler",peak.peakMegabytes,{jobs:1,peakMegabytes:peak.peakMegabytes,activeMilliseconds:1000,peakAtMilliseconds:0,endedAtMs:now(),keepPeak:true}),store),{now}).catch(()=>undefined);
 }});
}

/** 📏️ The estimate of a job class: the learned peak of the host when the class has measured one, else the floor. */
export async function scheduledEstimateV1(className:string,floorMegabytes:number,environment:NodeJS.ProcessEnv,now:()=>number=Date.now){
 let learned:number|undefined;
 try{const bound=hostClassBoundV1(BUILD_HOST_CAPACITY_POLICY_V1,readHostEstimateStoreV1(hostCapacityDirectoryV1(environment)),[className],now());if(bound.source==="measured")learned=bound.megabytes;}catch{}
 return jobEstimateV1(className,floorMegabytes,learned);
}

/** 🏷️ The capacity class of the host graph service itself: it holds the fact index, so it learns its own peak and never shares a class with the clients that only execute. */
export const HOST_SERVICE_CLASS_V1="service:graph";

/** 🌐️ Records the peak resident size of the host service, bounded by the resident limit it enforces on itself, so one leak or spike cannot teach a reservation above what the service is allowed to hold. */
export async function recordHostServicePeakV1(environment:NodeJS.ProcessEnv,peakMegabytes:number,boundMegabytes:number,now:()=>number=Date.now):Promise<void>{
 const bounded=Math.max(1,Math.min(Math.round(peakMegabytes),Math.round(boundMegabytes)));
 await updateHostEstimateStoreV1(hostCapacityDirectoryV1(environment),BUILD_HOST_CAPACITY_POLICY_V1,store=>recordHostPhasesV1(BUILD_HOST_CAPACITY_POLICY_V1,store,[HOST_SERVICE_CLASS_V1],"service",bounded,{jobs:1,peakMegabytes:bounded,activeMilliseconds:1000,peakAtMilliseconds:0,endedAtMs:now(),keepPeak:true}),{now});
}
