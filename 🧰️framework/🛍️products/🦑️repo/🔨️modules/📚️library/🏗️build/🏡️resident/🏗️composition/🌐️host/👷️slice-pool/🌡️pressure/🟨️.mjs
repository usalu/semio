/** 🌡 Sustained memory throttling of the service cgroup: a load that crawls under `memory.high` never fails by itself, so the slice pool asks this watch whether the pressure has lasted long enough to cancel the load. The measure is the memory PSI `full avg10` where the host has PSI, else the `high` event rate of `memory.events`, else (no cgroup) the free memory of the host; the thresholds are the schema's `HostPolicy.memoryPressure`, and a burst shorter than `sustainedSeconds` never trips.
 * @see ../🟨️.mjs */
import {readFileSync} from "node:fs";
import {freemem} from "node:os";
import {join} from "node:path";
import {HOST_POLICY_V1} from "../../../../📨️protocol/🎚️policy/🟨️.mjs";

const MiB=1048576;

/** 📖 The measures of one sample: `full` (PSI full avg10, percent), `high` (cumulative `high` events) and `freeBytes`; each only where the host offers it. */
export function cgroupMeasuresV1({root="/sys/fs/cgroup",read=path=>readFileSync(path,"utf8"),own=()=>read("/proc/self/cgroup").split("\n").map(line=>/^0::(.*)$/u.exec(line)?.[1]).find(path=>path!==undefined)??"",free=freemem}={}){
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
