import{hostProfileV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import{registerLeaseWaitScaleV1}from"../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import{readHostTextV1,runHostCommandV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import{spawnSync}from"node:child_process";
import{readFileSync}from"node:fs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🐢️ The declared numbers of the machine speed factor: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const MACHINE_SPEED_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
const cpuProbeTimeoutMilliseconds=3000;

/** 🐢️ How many times slower than the reference machine a machine with `cores` cores and `totalMegabytes` MB is taken to be: 1 at or above the reference, 2.83 for 8 GB and 4 cores, never above `maximumFactor`; rounded up to two decimals so the same machine always gets the same factor.
 * @see ./🧪️tests/🟦️.ts */
export function machineSpeedFactorV1(machine,policy=MACHINE_SPEED_POLICY_V1){
 const cores=Math.max(1,Number(machine.cores)||1),total=Math.max(1,Number(machine.totalMegabytes)||policy.referenceMegabytes);
 const factor=Math.max(1,policy.referenceCores/cores)*Math.max(1,policy.referenceMegabytes/total)**policy.memoryExponent;
 return Math.min(policy.maximumFactor,Math.ceil(factor*100-1e-9)/100);
}

/** 🐢️ The speed factor of the machine this process runs on, a declared smaller machine (`SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES`, `SEMIO_HOST_CAPACITY_CORES`) counting as declared. */
export function localMachineSpeedFactorV1(environment=process.env,policy=MACHINE_SPEED_POLICY_V1){
 const host=hostProfileV1(environment);
 return machineSpeedFactorV1({cores:host.cores,totalMegabytes:host.totalMegabytes},policy);
}

/** 🐢️ An authored wall-clock limit lengthened for the machine: the same number on a reference machine. */
export const scaledMillisecondsV1=(milliseconds,factor)=>Math.ceil(milliseconds*Math.max(1,factor));

/** 🫀️ The processor time in milliseconds a live process has used so far, or undefined where the platform cannot say: a step that keeps using processor time makes progress, whatever it prints. Linux reads `/proc/<pid>/stat`, macOS `ps -o time=`; Windows answers undefined (the limit then stays a window without output). */
export function processCpuMillisecondsV1(pid,platform=process.platform){
 if(!Number.isSafeInteger(pid)||pid<=0)return undefined;
 try{
  if(platform==="linux"){
   return linuxCpuMillisecondsV1(readFileSync(`/proc/${pid}/stat`,"utf8"));
  }
  if(platform==="darwin"){
   const run=spawnSync("ps",["-o","time=","-p",String(pid)],{encoding:"utf8",timeout:cpuProbeTimeoutMilliseconds});
   return run.status===0?cpuTimeTextMillisecondsV1(run.stdout):undefined;
  }
 }catch{}
 return undefined;
}

/** 📡️ Reads one process's CPU time without blocking a parent that also owns child timers and IPC. */
export async function readProcessCpuMillisecondsV1(pid,{platform=process.platform,signal,run=runHostCommandV1,read=readHostTextV1}={}){
 if(signal?.aborted||!Number.isSafeInteger(pid)||pid<=0)return undefined;
 try{
  if(platform==="linux"){
   const text=await read(`/proc/${pid}/stat`);
   return signal?.aborted||text===null?undefined:linuxCpuMillisecondsV1(text);
  }
  if(platform==="darwin"){
   const timeout=AbortSignal.timeout(cpuProbeTimeoutMilliseconds),sampling=signal===undefined?timeout:AbortSignal.any([signal,timeout]);
   const text=await run("ps",["-o","time=","-p",String(pid)],sampling);
   return sampling.aborted?undefined:cpuTimeTextMillisecondsV1(text);
  }
 }catch{}
 return undefined;
}

/** 📏️ Converts the user and system ticks of a Linux proc stat row to milliseconds. */
function linuxCpuMillisecondsV1(text){
 const rest=text.slice(text.lastIndexOf(")")+2).split(" "),ticks=Number(rest[11])+Number(rest[12]);
 return Number.isFinite(ticks)?Math.round(ticks*10):undefined;
}

/** 🕰️ Reads the `[[d-]h:]m:s[.f]` time that `ps` prints into milliseconds, undefined for anything else. */
export function cpuTimeTextMillisecondsV1(text){
 const match=/^\s*(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)\s*$/u.exec(String(text));
 if(match===null)return undefined;
 const[,days,hours,minutes,seconds]=match;
 return Math.round((((Number(days??0)*24+Number(hours??0))*60+Number(minutes))*60+Number(seconds))*1000);
}

registerLeaseWaitScaleV1(()=>localMachineSpeedFactorV1());
