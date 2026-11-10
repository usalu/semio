import {createCgroupLimiterV1,detectCgroupV2V1,type CgroupPortsV1,cgroupPortsV1} from "../🐧️cgroup/🟦️.ts";
import {createJobObjectLimiterV1,ensureJobHelperV1,jobObjectProcessPortsV1} from "../🪟️job-object/🟦️.ts";
import {createWatchdogLimiterV1,type WatchdogPortsV1} from "../🐕️watchdog/🟦️.ts";
import type {JobLimitPolicyV1,JobLimiterV1} from "../🟦️.ts";

/** 🔌️ The platform facts the choice depends on, injected for tests. */
export type JobLimiterChoicePortsV1=Readonly<{platform?:NodeJS.Platform;cgroup?:CgroupPortsV1;watchdog?:WatchdogPortsV1;helper?:()=>string|undefined}>;

/** 🎚️ Chooses the strongest mechanism the platform offers: a delegated cgroup v2 on Linux, a job object on Windows, otherwise the cooperative watchdog; a hard policy that only gets the watchdog is reported through `fellBack`, so the caller can tell the user in their language. */
export function selectJobLimiterV1(policy:JobLimitPolicyV1,ports:JobLimiterChoicePortsV1={}):Readonly<{limiter:JobLimiterV1;fellBack:boolean}>{
 const platform=ports.platform??process.platform,watchdog=()=>createWatchdogLimiterV1(ports.watchdog);
 let limiter:JobLimiterV1|undefined;
 if(policy.enforcement==="hard"){
  if(platform==="linux"){const cgroup=ports.cgroup??cgroupPortsV1(),base=detectCgroupV2V1(cgroup);if(base!==undefined)limiter=createCgroupLimiterV1(base,cgroup);}
  else if(platform==="win32"){const helper=(ports.helper??ensureJobHelperV1)();if(helper!==undefined)limiter=createJobObjectLimiterV1(jobObjectProcessPortsV1(helper));}
 }
 return Object.freeze({limiter:limiter??watchdog(),fellBack:policy.enforcement==="hard"&&limiter===undefined});
}
