/** 🌡 Sustained memory throttling of the service cgroup: a load that crawls under `memory.high` never fails by itself, so the slice pool asks this watch whether the pressure has lasted long enough to cancel the load. The measure is the memory PSI `full avg10` where the host has PSI, else the `high` event rate of `memory.events`, else (no cgroup) the memory the host can still give out (free plus reclaimable pages: the free pages alone are a few megabytes on a macOS host that runs for days, so a plan was cancelled as starved while the host had gigabytes to give); the thresholds are the schema's `HostPolicy.memoryPressure`, and a burst shorter than `sustainedSeconds` never trips.
 * @see ../🟨️.mjs */
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {platform as osPlatform} from "node:os";
import {hostAvailableBytesV1} from "../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {join} from "node:path";
import {HOST_POLICY_V1} from "../../../../📨️protocol/🎚️policy/🟨️.mjs";

const MiB=1048576;

/** 📖 The measures of one sample: `full` (PSI full avg10, percent), `high` (cumulative `high` events) and `freeBytes`; each only where the host offers it. */
export function cgroupMeasuresV1({root="/sys/fs/cgroup",read=path=>readFileSync(path,"utf8"),own=()=>read("/proc/self/cgroup").split("\n").map(line=>/^0::(.*)$/u.exec(line)?.[1]).find(path=>path!==undefined)??"",free=hostAvailableBytesV1}={}){
 return()=>{
  const measures={};
  let directory;try{directory=join(root,own().trim());read(join(directory,"memory.events"));}catch{directory=undefined;}
  if(directory===undefined)return{freeBytes:free()};
  try{const match=/^full avg10=([0-9.]+)/mu.exec(read(join(directory,"memory.pressure")));if(match)measures.full=Number(match[1]);}catch{}
  try{const match=/^high ([0-9]+)/mu.exec(read(join(directory,"memory.events")));if(match)measures.high=Number(match[1]);}catch{}
  return measures;
 };
}

/** 🌡 A watch for one load: `sample()` reads the measures and answers `{tripped,kind,value,limit,seconds}` (`kind` is `psi`, `events` or `free`); tripped once the measure has stayed over its limit for `sustainedSeconds`. */
export function createPressureWatchV1({measures=cgroupMeasuresV1(),policy=HOST_POLICY_V1.memoryPressure,now=()=>performance.now()}={}){
 let over,previous;
 return{
  sample(){
   const at=now(),row=measures();let kind,value,limit,hot;
   if(row.full!==undefined){kind="psi";value=row.full;limit=policy.fullAvg10Percent;hot=value>=limit;}
   else if(row.high!==undefined){
    kind="events";limit=policy.throttleEventsPerSecond;
    value=previous===undefined||at<=previous.at?0:Math.round((row.high-previous.high)/((at-previous.at)/1000)*10)/10;previous={at,high:row.high};hot=value>=limit;
   }else if(row.freeBytes!==undefined){kind="free";value=Math.round(row.freeBytes/MiB);limit=policy.freeFloorMegabytes;hot=value<limit;}
   else return{tripped:false};
   if(!hot){over=undefined;return{tripped:false,kind,value,limit,seconds:0};}
   over??=at;const seconds=Math.round((at-over)/1000);
   return{tripped:at-over>=policy.sustainedSeconds*1000,kind,value,limit,seconds};
  },
 };
}

/**
 * 🌡 The raised-pressure signal of the host as `normal`, `warn` or `critical`, `undefined` where no source reads: macOS asks the kernel (`kern.memorystatus_vm_pressure_level`: 1 normal, 2 warn, 4 critical), Linux reads the memory PSI `full avg10` against the schema's `memoryPressure.fullAvg10Percent` (`warn` at or above it, `critical` at twice it).
 * @see https://developer.apple.com/documentation/dispatch/dispatch_source_memorypressure_flags_t
 */
export function hostPressureLevelV1({platform=osPlatform(),run=()=>{const result=spawnSync("sysctl",["-n","kern.memorystatus_vm_pressure_level"],{encoding:"utf8",timeout:2000});if(result.error!==undefined||result.status!==0)throw result.error??Error("sysctl exited with "+result.status);return result.stdout;},read=path=>readFileSync(path,"utf8"),policy=HOST_POLICY_V1.memoryPressure}={}){
 try{
  if(platform==="darwin"){const level=Number(run().trim());return level===4?"critical":level===2?"warn":level===1?"normal":undefined;}
  if(platform==="linux"){const match=/^full avg10=([0-9.]+)/mu.exec(read("/proc/pressure/memory"));if(match===null)return undefined;const value=Number(match[1]);return value>=policy.fullAvg10Percent*2?"critical":value>=policy.fullAvg10Percent?"warn":"normal";}
 }catch{}
 return undefined;
}
