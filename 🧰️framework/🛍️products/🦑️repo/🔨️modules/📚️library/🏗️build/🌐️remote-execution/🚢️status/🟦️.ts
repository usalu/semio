import {remoteExecutionPortsV1,remoteExecutionSchedulerEndpointV1,remoteExecutionWorkerApiV1} from "../🛰️sidecar/🧰️shared/🟦️.ts";
import {readRemoteExecutionPolicyV1,requireRemoteExecutionShapeV1} from "../🟦️.ts";

/** 🚢️ One machine of the fleet as `repo fleet status` shows it. */
export type RemoteExecutionFleetMachineV1=Readonly<{origin:string;platform:string;pressure:"normal"|"warn"|"critical";load:number|null;cores:number;running:number;queued:number;headroom:boolean;ageSeconds:number;worker:"absent"|"idle"|"busy"|"refusing";why?:Readonly<{code:string;en:string;de:string}>;offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number}>;

/** 🌍️ The fleet as the scheduler sees it, plus what the cache server counts: the facts `repo fleet status` aggregates. `cache` and `claims` are absent while the cache server does not answer or does not count them yet. */
export type RemoteExecutionFleetFactV1=Readonly<{scheduler:string;expireSeconds:number;machines:readonly RemoteExecutionFleetMachineV1[];queued:number;executing:number;offloadRejected:number;refused:number;retried:number;cache?:Readonly<{hits:number;misses:number;puts:number}>;claims?:Readonly<{active:number;waiting:number;oldestSeconds:number}>}>;

const whole=(value:unknown):number=>typeof value==="number"&&Number.isFinite(value)&&value>=0?Math.floor(value):0;

/** 📊️ Asks the scheduler of the fleet for its live table (`GET /v1/fleet/load`) and the cache server for its counters and claim table (`GET /status`), both with the fleet credential and a timeout; `undefined` when no scheduler is published, an error when the scheduler does not answer. */
export async function readRemoteExecutionFleetFactsV1(context:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;timeoutMilliseconds?:number}>,fetcher:typeof fetch=fetch):Promise<RemoteExecutionFleetFactV1|undefined>{
 const policy=readRemoteExecutionPolicyV1(),endpoint=remoteExecutionSchedulerEndpointV1(context);
 if(endpoint===undefined)return undefined;
 const {remoteCacheFetchV1,resolveRemoteCacheV1}=await import("../../🌐️remote-cache/🛰️fleet/🟦️.ts");
 const {fleetGroupViewV1}=await import("../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts");
 const environment={...context.environment,SEMIO_WORKSPACE_ROOT:context.root},resolution=resolveRemoteCacheV1(environment,{root:context.root}),workerToken=context.environment[policy.environment.workerToken];
 const scheduler=context.environment[policy.environment.scheduler]??remoteExecutionWorkerApiV1(endpoint,remoteExecutionPortsV1(context.environment,(await fleetGroupViewV1(context.root,context.environment))?.ports).worker),timeout=context.timeoutMilliseconds??3000;
 const signal=():AbortSignal=>AbortSignal.any([...context.signal?[context.signal]:[],AbortSignal.timeout(timeout)]),headers:Record<string,string>=workerToken?{authorization:`Bearer ${workerToken}`}:{},schedulerCall=remoteCacheFetchV1(environment,{root:context.root,base:fetcher,audience:"execution",roleOf:()=>"submit"}),cacheCall=remoteCacheFetchV1(environment,{root:context.root,base:fetcher});
 const response=await schedulerCall(`${scheduler.replace(/\/+$/u,"")}/v1/fleet/load`,{signal:signal(),headers});
 if(!response.ok){await response.body?.cancel();throw Error(`The scheduler answered ${response.status}`);}
 const table=await response.json() as {machines:readonly {origin:string;platform:{key:string};capacity:{cores:number};pressure:RemoteExecutionFleetMachineV1["pressure"];load:number|null;running:number;queued:number;headroom:boolean;ageSeconds:number;worker:{state:RemoteExecutionFleetMachineV1["worker"];why?:{code:string;en:string;de:string}};offloadedIn:number;offloadedOut:number;refused:number;counters:{unit:{localHit:number;remoteHit:number;miss:number}}}[];queued:number;executing:number;counters:{offloadRejected:number;refused:number;retried:number}};
 requireRemoteExecutionShapeV1("FleetLoad",table,"Closed fleet load required");
 let cache:RemoteExecutionFleetFactV1["cache"],claims:RemoteExecutionFleetFactV1["claims"];
 if(resolution!==undefined)try{
  const status=await cacheCall(`${resolution.contract.endpoint.replace(/\/+$/u,"")}/status`,{signal:signal()}),body=status.ok?await status.json() as {counters?:Record<string,{hits?:number;misses?:number;puts?:number}>;claims?:{active?:number;waiting?:number;oldestSeconds?:number}}:undefined;
  if(!status.ok)await status.body?.cancel();
  if(body?.counters!==undefined){const rows=Object.values(body.counters);cache={hits:rows.reduce((sum,row)=>sum+whole(row.hits),0),misses:rows.reduce((sum,row)=>sum+whole(row.misses),0),puts:rows.reduce((sum,row)=>sum+whole(row.puts),0)};}
  if(body?.claims!==undefined)claims={active:whole(body.claims.active),waiting:whole(body.claims.waiting),oldestSeconds:whole(body.claims.oldestSeconds)};
 }catch{}
 return {scheduler,expireSeconds:whole((table as unknown as {expireSeconds:number}).expireSeconds),machines:table.machines.map(row=>({origin:row.origin,platform:row.platform.key,pressure:row.pressure,load:row.load,cores:row.capacity.cores,running:row.running,queued:row.queued,headroom:row.headroom,ageSeconds:row.ageSeconds,worker:row.worker.state,...row.worker.why===undefined?{}:{why:row.worker.why},offloadedIn:row.offloadedIn,offloadedOut:row.offloadedOut,refused:row.refused,cacheHit:row.counters.unit.localHit+row.counters.unit.remoteHit,cacheMiss:row.counters.unit.miss,cacheSkipped:row.counters.unit.skipped??0})),queued:table.queued,executing:table.executing,offloadRejected:table.counters.offloadRejected,refused:table.counters.refused,retried:table.counters.retried,...cache===undefined?{}:{cache},...claims===undefined?{}:{claims}};
}
