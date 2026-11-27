import{readFileSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{hostCapacityDirectoryV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import{METRICS_FILES_V1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
import{METRICS_COLLECTOR_POLICY_V1,createFleetMetricsCollectorV1,metricsScratchPathV1,type MetricsSourcesV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🟦️.ts";
import{hostMetricsSourcesV1,metricsVolumePathsV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🔌️host/🟦️.ts";
import{readBuildCountersV1}from"../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";
import{parseCounterLedgerV1,readEventCountsV1}from"../../../📈️counter-ledger/🟦️.ts";
import{casCensusPathV1,casCounterDirectoryV1,casRootV1}from"../../../🗄️cas/🗃️store/🟦️.ts";
import{NX_CACHE_POLICY_V1}from"../../../../⚡️caching/🚀️bootstrap/📊️nx-cache/🟦️.ts";
import{readOffloadCountersV1,readOffloadReasonCountersV1}from"../../🚀️offload/📊️counters/🟦️.ts";
import{readRemoteCacheClientGroupsV1}from"../../../🌐️remote-cache/🛰️fleet/🧮️client-counters/🟦️.ts";
import type{RemoteExecutionSidecarContextV1}from"../🧰️shared/🟦️.ts";

export{metricsScratchPathV1};

const P=METRICS_COLLECTOR_POLICY_V1,FRESH_MILLISECONDS=120000;
type Json=Record<string,unknown>;

const jsonByStamp=(path:string):(()=>Json|null)=>{
 let stamp=-1,value:Json|null=null;
 return()=>{
  let at:number;try{at=statSync(path).mtimeMs;}catch{stamp=-1;value=null;return null;}
  if(at!==stamp){stamp=at;try{const parsed=JSON.parse(readFileSync(path,"utf8")) as unknown;value=typeof parsed==="object"&&parsed!==null&&!Array.isArray(parsed)?parsed as Json:null;}catch{value=null;}}
  return value;
 };
};
const casCensusPath=(environment:Readonly<Record<string,string|undefined>>):string=>casCensusPathV1(casRootV1(environment));
const whole=(value:unknown):number|undefined=>typeof value==="number"&&Number.isFinite(value)&&value>=0?value:undefined;
const fresh=(at:unknown,now:number):boolean=>typeof at==="number"&&now-at<=FRESH_MILLISECONDS&&at<=now+FRESH_MILLISECONDS;

/** 🧭️ The sources of the product layer for one machine: the unit cache counter files, the offload counter files, the hit and miss counters and the census size of the host content store and of Nx (`nx-counters`; a store that has counted nothing yet has zero hits and misses, not a gap: its counter files appear with the first event, which the next step then counts), the persisted counter ledger of the remote cache server when this machine runs it (found through `fleet/cache.json` of the store), and the files this origin keeps in the fleet store (`pool.json`, `slice-memory.json` with the rows the graph service read, `ready.json`, `head.json`), each re-parsed only when its modification time changed. Whatever is not there is `null`, a gap in the series. The host sources (processor, memory, warden, disk) come from `hostMetricsSourcesV1`.
 * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🧬️schema/🔣️.json */
export function fleetMetricsSourcesV1(input:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;origin:string;now?:()=>number;directory?:string}>):MetricsSourcesV1{
 const now=input.now??Date.now,store=input.environment.SEMIO_FLEET_STORE_ROOT,own=store===undefined||store===""?undefined:join(store,"v1","origins",input.origin);
 const read=(name:string):(()=>Json|null)=>own===undefined?()=>null:jsonByStamp(join(own,name));
 const pool=read("pool.json"),slice=read("slice-memory.json"),ready=read("ready.json"),head=read("head.json"),census=jsonByStamp(casCensusPath(input.environment));
 const cache=machinePathV1(input.root,".cache"),nxDirectory=machinePathV1(input.root,".cache",...NX_CACHE_POLICY_V1.directory);
 return {
  ...hostMetricsSourcesV1({volumes:()=>metricsVolumePathsV1(input.root,cache,tmpdir()),...input.directory===undefined?{}:{directory:input.directory},now}),
  unitCounters:()=>readBuildCountersV1(input.root).unit,
  offload:()=>readOffloadCountersV1(input.root),
  cacheCounters:()=>{
   const counted=readEventCountsV1(casCounterDirectoryV1(casRootV1(input.environment)),["localHit","localMiss"]),nx=readEventCountsV1(nxDirectory,["nxLocalHit","nxExecuted"]);
   const size=whole(census()?.bytes);
   return {localHit:counted.counts.localHit!+nx.counts.nxLocalHit!,localMiss:counted.counts.localMiss!+nx.counts.nxExecuted!,...size===undefined?{}:{bytes:size}};
  },
  remoteCache:()=>({groups:readRemoteCacheClientGroupsV1(input.root)}),
  skipReasons:()=>readBuildCountersV1(input.root).skipReasons,
  offloadReasons:()=>readOffloadReasonCountersV1(input.root),
  pool:()=>{
   const row=pool();if(row===null||!fresh(row.updatedMs,now()))return null;
   return {units:whole(row.units)??0,claimed:whole(row.claimed)??0,published:whole(row.published)??0,failed:whole(row.failed)??0,rssMegabytes:whole(row.rssMegabytes)??0};
  },
  phase:()=>{
   const row=slice();if(row===null||!fresh(row.atMs,now()))return null;
   const phase=row.phase as Json|undefined,done=whole(phase?.done),total=whole(phase?.total);
   return done===undefined||total===undefined?null:{done,total};
  },
  ready:()=>{
   const row=ready();if(row===null)return null;
   return {stageIndex:whole(row.stageIndex)??0,state:typeof row.state==="string"?row.state:"absent",attempts:whole(row.attempts)??0};
  },
  journal:()=>{const row=head();return row===null?null:{events:whole(row.events)??0,bytes:whole(row.bytes)??0};},
  facts:()=>{
   const row=slice();if(row===null||!fresh(row.atMs,now()))return null;
   const rows=((row.sources as Json|undefined)?.caches as Json|undefined)?.rowStore as Json|undefined;
   return whole(rows?.rows)??null;
  },
 };
}

/** 📡️ Starts the metrics collector of this machine inside the readiness service (the per-machine agent that already runs on every worker and mirrors the warden state): a timer of `tickMilliseconds` (unreferenced, so it never keeps the process alive) steps the collector, which writes the local rings below the host capacity directory and publishes them into `origins/<origin>/metrics/` of the fleet store (`SEMIO_FLEET_STORE_ROOT`; no store, no publication). A readiness service of a scratch member (checkout inside the temporary directory, as the build contract tests start them) measures nothing: the ring directory belongs to the machine, and a test that took its writer lock held it for the whole run and silenced the real service (the group server published nothing). The returned function stops it and writes the last window; the abort of `context.signal` stops it too. Never throws: a machine whose directory cannot be resolved has no metrics, not a failed service.
 * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🟦️.ts */
export function startFleetMetricsV1(context:Pick<RemoteExecutionSidecarContextV1,"root"|"environment"|"signal"|"write">,origin:string):()=>void{
 try{
  if(metricsScratchPathV1(context.root)||metricsScratchPathV1(context.environment.SEMIO_FLEET_ROOT))return()=>{};
  const directory=join(hostCapacityDirectoryV1(context.environment),METRICS_FILES_V1.directory),store=context.environment.SEMIO_FLEET_STORE_ROOT;
  const collector=createFleetMetricsCollectorV1({directory,checkout:context.root,sources:fleetMetricsSourcesV1({root:context.root,environment:context.environment,origin}),report:context.write,...store===undefined||store===""?{}:{store:{root:store,origin}}});
  let stopped=false;
  const timer=setInterval(()=>{try{collector.step();}catch{}},P.tickMilliseconds),stop=():void=>{if(stopped)return;stopped=true;clearInterval(timer);try{collector.close();}catch{}};
  timer.unref?.();context.signal.addEventListener("abort",stop,{once:true});
  try{collector.step();}catch{}
  return stop;
 }catch{return()=>{};}
}

/** 🗓️ The fleet-scope sources of the scheduler (run by the machine that serves it): the remote executions it holds now (`counts`, read from the scheduler in the process) and the units offloaded into the machines of the fleet, the sum of the `offload.<origin>.offloadedIn` counts of its persisted counter ledger (`build-scheduler.counters.json`, cumulative over restarts and over every machine, so the sum only grows; re-parsed only when the file changed). `null` counts (a scheduler that is stopping) leave a gap.
 * @see ../../../📈️counter-ledger/🟦️.ts */
export function schedulerMetricsSourcesV1(input:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;counts:()=>Readonly<{running:number;queued:number}>|null;now?:()=>number}>):MetricsSourcesV1{
 const record=input.environment.SEMIO_FLEET_SIDECAR_RECORD,file=join(record===undefined||record===""?machinePathV1(input.root,".cache","f","sidecars"):dirname(record),"build-scheduler.counters.json");
 let stamp=-1,offloadedIn=0;
 const total=():number=>{
  try{
   const at=statSync(file).mtimeMs;
   if(at!==stamp){stamp=at;const document=parseCounterLedgerV1(readFileSync(file,"utf8"));offloadedIn=document===undefined?0:Object.entries(document.groups).reduce((sum,[group,row])=>group.startsWith("offload.")?sum+(whole(row.offloadedIn)??0):sum,0);}
  }catch{stamp=-1;}
  return offloadedIn;
 };
 return {...input.now===undefined?{}:{now:input.now},remoteExecution:()=>{const counts=input.counts();return counts===null?null:{running:counts.running,queued:counts.queued,offloadedIn:total()};}};
}

/** 🗄️ The fleet-scope source of the remote cache server (run by the machine that serves it): the entries and bytes its store holds now (`usage`, read from the store in the process: no walk, no file). */
export function cacheStatusMetricsSourcesV1(input:Readonly<{usage:()=>Readonly<{entries:number;bytes:number}>|null;now?:()=>number}>):MetricsSourcesV1{
 return {...input.now===undefined?{}:{now:input.now},remoteCacheStatus:()=>{const usage=input.usage();return usage===null?null:{entries:usage.entries,bytes:usage.bytes};}};
}

/** 🌐️ Starts the collector of one fleet-scope source (`remote-execution` for the scheduler, `remote-cache` for the cache server) inside the service that owns the data: one writer per source, rings below the host capacity directory (`metrics/fleet/<source>`), published into `fleet/metrics/<source>/` of the fleet store, so the fleet has one series per fact, never one per machine. The returned function stops it and writes the last window; the abort of `context.signal` stops it too. Never throws.
 * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🟦️.ts */
export function startFleetScopeMetricsV1(context:Pick<RemoteExecutionSidecarContextV1,"environment"|"signal"|"write">&Partial<Pick<RemoteExecutionSidecarContextV1,"root">>,input:Readonly<{source:"remote-execution"|"remote-cache";sources:MetricsSourcesV1}>):()=>void{
 try{
  const directory=join(hostCapacityDirectoryV1(context.environment),METRICS_FILES_V1.directory,P.fleetDirectory,input.source),store=context.environment.SEMIO_FLEET_STORE_ROOT;
  const collector=createFleetMetricsCollectorV1({directory,...context.root===undefined?{}:{checkout:context.root},sources:input.sources,report:context.write,fleet:{source:input.source,...store===undefined||store===""?{}:{root:store}}});
  let stopped=false;
  const timer=setInterval(()=>{try{collector.step();}catch{}},P.tickMilliseconds),stop=():void=>{if(stopped)return;stopped=true;clearInterval(timer);try{collector.close();}catch{}};
  timer.unref?.();context.signal.addEventListener("abort",stop,{once:true});
  try{collector.step();}catch{}
  return stop;
 }catch{return()=>{};}
}
