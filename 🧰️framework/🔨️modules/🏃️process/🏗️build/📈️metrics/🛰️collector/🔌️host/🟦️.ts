import {readFileSync,readdirSync,statSync,statfsSync} from "node:fs";
import {availableParallelism,cpus,freemem,loadavg,totalmem} from "node:os";
import {join} from "node:path";
import {hostCapacityDirectoryV1} from "../../../🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import {HOST_WARDEN_POLICY_V1} from "../../../🌡️host-capacity/🛡️warden/🟦️.ts";
import {hostWardenDirectoryV1,readHostWardenStateV1} from "../../../🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";
import type {MetricsSourcesV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const P=schema.definitions.Policy.const;

type Cached<T>=Readonly<{stamp:number;value:T}>;
const cachedByStamp=<T>(path:string,read:()=>T):(()=>T|null)=>{
 let cache:Cached<T|null>|undefined;
 return ()=>{
  let stamp:number;try{stamp=statSync(path).mtimeMs;}catch{cache=undefined;return null;}
  if(cache===undefined||cache.stamp!==stamp){let value:T|null=null;try{value=read();}catch{}cache={stamp,value};}
  return cache.value;
 };
};

/** 🖥️ The sources every machine has, read without spawning a process: processor times, load, memory from the operating system; the warden state and the gate sample as files (re-parsed only when their modification time changed); free space of the volumes by `statfs`; the waiters of the fair queue by a directory listing; this process by `process.cpuUsage` and `process.memoryUsage`. `volumes` lists the paths whose free space matters (the worktree, its cache and the mounted cache images). Sources of the product layer (unit, offload, cache counters, pool, ready, journal) are added by the caller.
 * @see ../🧬️schema/🔣️.json */
export function hostMetricsSourcesV1(input:Readonly<{volumes:()=>readonly string[];directory?:string;platform?:NodeJS.Platform;now?:()=>number}>):MetricsSourcesV1{
 const directory=input.directory??hostCapacityDirectoryV1(),platform=input.platform??process.platform,now=input.now??Date.now;
 const wardenFile=join(hostWardenDirectoryV1(directory),HOST_WARDEN_POLICY_V1.stateFile),sampleFile=join(directory,"sample.json");
 const warden=cachedByStamp(wardenFile,()=>readHostWardenStateV1(directory,0)??null);
 const sample=cachedByStamp(sampleFile,()=>(JSON.parse(readFileSync(sampleFile,"utf8")) as {host?:Record<string,unknown>}).host??null);
 return {
  now,
  cpuTimes:()=>{let busy=0,total=0;for(const row of cpus()){const t=row.times;busy+=t.user+t.nice+t.sys+t.irq;total+=t.user+t.nice+t.sys+t.irq+t.idle;}return total>0?{busy,total}:null;},
  cores:()=>availableParallelism(),
  load:()=>platform==="win32"?null:loadavg()[0]!,
  memory:()=>({totalBytes:totalmem(),freeBytes:freemem()}),
  warden:()=>{
   const state=warden();if(state===null||state===undefined||now()-state.atMs>P.wardenFreshMilliseconds)return null;
   return {atMs:state.atMs,totalMegabytes:state.host.totalMegabytes,availableMegabytes:state.host.availableMegabytes,swapUsedMegabytes:state.host.swapUsedMegabytes,swapOutMegabytesPerSecond:state.host.swapOutMegabytesPerSecond,compressedMegabytes:state.host.compressedMegabytes,pressure:state.host.pressure,level:state.level,subjects:state.subjects,headroomMegabytes:state.headroomMegabytes,cancelled:state.counters.cancelled};
  },
  gate:()=>{
   const host=sample();if(host===null||host===undefined||typeof host.atMs!=="number"||now()-host.atMs>P.sampleFreshMilliseconds)return null;
   const whole=(value:unknown):number|null=>typeof value==="number"&&Number.isFinite(value)?value:null;
   return {totalMegabytes:whole(host.totalMegabytes)??0,availableMegabytes:whole(host.availableMegabytes)??0,swapUsedMegabytes:whole(host.swapUsedMegabytes),swapOutMegabytesPerSecond:whole(host.swapOutMegabytesPerSecond),compressedMegabytes:whole(host.compressedMegabytes),pressure:host.pressure==="warn"||host.pressure==="critical"?host.pressure:"normal",openFiles:whole(host.openFiles),attachedImages:whole(host.attachedImages)};
  },
  volumes:()=>{
   const rows:{freeBytes:number;totalBytes:number}[]=[];
   for(const path of input.volumes())try{const fact=statfsSync(path);rows.push({freeBytes:Number(fact.bavail)*Number(fact.bsize),totalBytes:Number(fact.blocks)*Number(fact.bsize)});}catch{}
   return rows;
  },
  queueDepth:()=>{try{return readdirSync(join(directory,"queue")).filter(name=>name.endsWith(".json")).length;}catch{return null;}},
  cpuMicroseconds:()=>{const usage=process.cpuUsage();return usage.user+usage.system;},
  self:()=>{const usage=process.cpuUsage();return {cpuMicroseconds:usage.user+usage.system,rssBytes:process.memoryUsage.rss()};},
 };
}

/** 💽️ The volumes that matter on a machine: the worktree, its `.cache` and every directory directly below `.cache` (the mounted cache images), and the temporary directory; unreadable ones are skipped by the reader. */
export function metricsVolumePathsV1(root:string,cache:string,temporary:string):readonly string[]{
 const paths=new Set<string>([root,cache,temporary]);
 try{for(const name of readdirSync(cache).slice(0,P.volumeChildrenMaximum))paths.add(join(cache,name));}catch{}
 return [...paths];
}
