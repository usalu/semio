import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,readdirSync,statSync} from "node:fs";
import {readdir} from "node:fs/promises";
import {homedir} from "node:os";
import {join} from "node:path";
import {resolveLanguageV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {remoteCacheFetchV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {fleetGroupViewV1} from "../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {forwardableRequestersV1} from "../../../🌐️remote-cache/🧭️peers/🟦️.ts";
import {graphFleetClientV1} from "../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../🪪️graph-state/🧶️nx/🟦️.ts";
import {graphPlatformV1} from "../../../🪪️graph-state/🟦️.ts";
import {ownerCostContextV1,ownerCostPathV1,readOwnerCostFileV1} from "../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import {wantedTextV1} from "../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import {runWantedServerV1,serveStateFileV1,wantedServerPolicyV1,type WantedInferV1} from "../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import {createGenerationUploaderV1,localGenerationUploadSourceV1,WAVE_UPLOAD_POLICY_V1} from "../../../🕸️graph/🕰️generations/📡️wave-upload/🟦️.ts";
import {journalCursorStoreV1} from "../../../🏡️resident/🕰️graph-hooks/📖️direct/🟦️.ts";
import {snapshotStoreV1} from "../../../🪪️graph-state/📍️cursor/🟦️.ts";
import {readRemoteExecutionHostViewV1,remoteExecutionOriginV1} from "../../🌡️load/🟦️.ts";
import {sleep,type RemoteExecutionSidecarContextV1} from "../🧰️shared/🟦️.ts";
import {awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,graphRefreshCustodyHeldV1,readGraphLeaseEntriesV1} from "./🔬️stall/🟦️.ts";
import {measureReadTreeV1,memoryOverrunV1} from "./🔬️stall/🌳️tree/🟦️.ts";
import {readyForegroundBusyOfStoreV1} from "../🚦️ready/🟦️.ts";

/** 🔌️ The environment the graph server and its reads run in: the one of the agent without the explicit remote cache overrides (`SEMIO_REMOTE_CACHE_ENDPOINT`, `_ACCESS`, `_INSTANCE`, `_TOKEN`, `SEMIO_FLEET_CACHE_ENDPOINT`), so the graph exchange uses the cache and instance the fleet announces, exactly as the workers do (their bootstrap unsets the same variables); an agent that carries the instance `semio` would otherwise look for requests in another instance than the one the workers write them to. */
export function graphExchangeEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const rest={...environment};
 for(const name of["SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_REMOTE_CACHE_TOKEN","SEMIO_FLEET_CACHE_ENDPOINT"])delete rest[name];
 return rest;
}

/** 🧰️ The `repo` executable a graph read is started with: the one named by `SEMIO_REPO_BIN`, else the installed one in the user's `.local/bin`, else the one on the path. */
export function graphServerCommandV1(environment:Readonly<Record<string,string|undefined>>):string{
 const named=environment.SEMIO_REPO_BIN;if(named!==undefined&&named!=="")return named;
 const installed=join(homedir(),".local","bin",process.platform==="win32"?"repo.exe":"repo");
 return existsSync(installed)?installed:"repo";
}

/** 🪶️ The headroom a machine offers the graph server: its available memory, or none at all when its physical memory is below `serveTotalMegabytes`. */
export function graphServerHeadroomV1<Pressure>(host:Readonly<{totalMegabytes:number;availableMegabytes:number;pressure:Pressure}>,serveTotalMegabytes:number):{freeMegabytes:number;pressure:Pressure}{
 return{freeMegabytes:host.totalMegabytes<serveTotalMegabytes?0:Math.max(0,Math.round(host.availableMegabytes)),pressure:host.pressure};
}

export{inferGraphReadV1,adoptGraphReadV1}from"./🔭️read/🟦️.ts";
import{adoptGraphReadV1, graphReadCapacityProbeV1, inferGraphReadV1, type GraphReadInputV1}from"./🔭️read/🟦️.ts";


/** 🕸️ The graph server of a machine: runs {@link runWantedServerV1} in its own supervised process (a graph read holds gigabytes and must never take the build agent down). A machine whose physical memory is below `serveTotalMegabytes` reports no headroom, so it neither serves requests nor refreshes: a full graph read does not fit beside its own work. Its reads are ordinary `repo show projects` commands, so inference runs under the fleet claims and the host capacity gate, and records and the generation reach the fleet cache through the paths every read already uses. A machine without a fleet cache waits and looks again. While the readiness loop of the machine is busy (its `ready.json` says not ready and running, waiting or backing off) it neither serves requests nor refreshes: readiness is the foreground, a background read holds the dependency store for hours, and a read still running when a stage queues behind it is stopped by the stage runner (`yield`). */
export async function runGraph(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const environment=graphExchangeEnvironmentV1({...context.environment,SEMIO_WORKSPACE_ROOT:context.root}),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment).language as "en"|"de",origin=remoteExecutionOriginV1(context.root,environment),policy=wantedServerPolicyV1;
 let settings=graphFleetSettingsV1(context.root,environment);
 while(settings===undefined&&!context.signal.aborted){context.write("graph-server: no fleet cache yet");await sleep(60_000,context.signal);settings=graphFleetSettingsV1(context.root,environment);}
 if(settings===undefined)return;
 const client=graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:context.root})}),store=environment.SEMIO_FLEET_STORE_ROOT,steerDirectory=machinePathV1(context.root,".cache","f","graph-server"),costPath=ownerCostPathV1(context.root);
 const readInput=(reason:"request"|"refresh",signal:AbortSignal|undefined):GraphReadInputV1=>({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,capacityWait:graphReadCapacityProbeV1(environment),say:context.write,onWave:row=>uploader?.wave(row),diagnose:async input=>describeGraphReadStallV1({rootPid:input.pid,entries:readGraphLeaseEntriesV1(environment),view:await readRemoteExecutionHostViewV1(environment).catch(()=>undefined),nowMs:Date.now(),silentMilliseconds:input.silentMilliseconds,lastOutput:input.lastOutput,language,tree:await measureReadTreeV1(input.pid).catch(()=>undefined)}),measure:async pid=>{const tree=await measureReadTreeV1(pid),view=await readRemoteExecutionHostViewV1(environment).catch(()=>undefined);return{footprintMegabytes:tree.footprintMegabytes??tree.rssMegabytes,grantMegabytes:view?.grants.find(row=>row.pid===pid)?.reservedMegabytes,pressure:view?.host.pressure??"normal"};},directory:steerDirectory});
 const infer=async(reason:"request"|"refresh",signal:AbortSignal|undefined,published:readonly string[]=[]):Promise<WantedInferV1>=>{
  if(published.length>0)context.write(wantedTextV1("continuing",language,{count:published.length}));
  await awaitGraphReadLeasesV1({blockers:()=>graphReadBlockersV1({entries:readGraphLeaseEntriesV1(environment),prefixes:policy.leaseResourcePrefixes,ownPid:process.pid,nowMs:Date.now()}),language,say:context.write,signal});
  if(signal?.aborted)return{ok:false,status:1,seconds:0,generation:null,detail:"stopped"};
  return inferGraphReadV1(readInput(reason,signal));
 };
 const adopt=(signal:AbortSignal|undefined)=>adoptGraphReadV1(readInput("refresh",signal));
 const serving=(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes;
 let uploader:ReturnType<typeof createGenerationUploaderV1>|undefined,tickTimer:ReturnType<typeof setInterval>|undefined,generationsRoot:string|undefined;
 if(serving){
  const{repositoryCacheBudgetV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),generations=join(repositoryCacheBudgetV1(context.root).partitions.graph.directory,"m","generations"),snapshots=snapshotStoreV1(context.root),cursorStore=journalCursorStoreV1(context.root,environment);
   generationsRoot=generations;
  uploader=createGenerationUploaderV1({language,say:context.write,statePath:join(steerDirectory,WAVE_UPLOAD_POLICY_V1.stateFile),
   source:localGenerationUploadSourceV1({workspace:context.root,root:generations,environment,cursors:{snapshotFor:async generation=>{const entry=cursorStore.get("g"+generation);if(!entry||entry.carry)return undefined;const bytes=await snapshots.bytes(entry.instance);return bytes?{instance:entry.instance,bytes}:undefined;},install:async()=>undefined}}),
   async may(){const host=(await readRemoteExecutionHostViewV1(environment)).host;return host.pressure==="normal"&&host.availableMegabytes>=WAVE_UPLOAD_POLICY_V1.minimumFreeMegabytes;}});
  tickTimer=setInterval(()=>uploader?.tick(),policy.roundMilliseconds);tickTimer.unref?.();
 }
 const work=context.work,adoptionStop=new AbortController();
 if(work!==undefined){const{repositoryCacheBudgetV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts");work.progressAtMs=()=>{const directory=join(repositoryCacheBudgetV1(context.root).partitions.graph.directory,"m","generations","s");try{let newest:number|null=null;for(const name of readdirSync(directory))if(name.endsWith(".progress.json"))newest=Math.max(newest??0,Math.floor(statSync(join(directory,name)).mtimeMs));return newest;}catch{return null;}};}
 const adoption=serving?undefined:import("./📥️adoption/🟦️.ts").then(({runHeadAdoptionLaneV1})=>runHeadAdoptionLaneV1({...context,signal:AbortSignal.any([context.signal,adoptionStop.signal,...context.drain?[context.drain]:[]])},{environment,store,origin,serveTotalMegabytes:policy.serveTotalMegabytes,steerDirectory})).catch(error=>{
  if(!adoptionStop.signal.aborted&&!context.signal.aborted)context.write("graph-head lane / Graphstand-Dienst "+String(error instanceof Error?error.message:error).slice(0,200));
 });
 try{await runWantedServerV1({client,owner:origin,...work===undefined?{}:{work:{restartPending:()=>work.restartPending,report:busy=>{const was=work.busy;work.busy=busy;if(was&&!busy)work.wake?.();}}},platform:graphPlatformV1(),language,steerDirectory,costs:()=>readOwnerCostFileV1(costPath,ownerCostContextV1().ceilingMegabytes),signal:context.signal,state:serveStateFileV1(join(steerDirectory,"serve-state.json")),say:context.write,sleep,infer,adopt,
  requesters:async()=>{
   if(store===undefined)return [];
   const view=await fleetGroupViewV1(context.root,environment),own=`${view?.id??environment.SEMIO_FLEET_GROUP??""}-`,peers=Object.keys(view?.peerStores??{}),names=(await readdir(join(store,"v1","origins"))).filter(name=>name!==origin&&(view===undefined||name.startsWith(own)||!peers.some(peer=>name.startsWith(`${peer}-`))));
   return view===undefined?names:[...names,...await forwardableRequestersV1({ownStore:store,peers:view.peerStores}).catch(()=>[])];
  },
  async headroom(){return graphServerHeadroomV1((await readRemoteExecutionHostViewV1(environment)).host,policy.serveTotalMegabytes);},
  async mayServe(){return(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes&&!readyForegroundBusyOfStoreV1(store,origin);},
   refreshDeferred:()=>generationsRoot!==undefined&&graphRefreshCustodyHeldV1({generationsRoot,leaseDirectory:machinePathV1(context.root,".cache/b/l/m/data"),environment,signal:context.signal}),
  async mayRefresh(){if(readyForegroundBusyOfStoreV1(store,origin))return false;const key=createHash("sha256").update(JSON.stringify(["graph-refresh",graphPlatformV1()])).digest("hex");return(await client.claim(key,origin,policy.refreshIntervalMilliseconds).catch(()=>({state:"fault"}))).state==="granted";},
  async quietMilliseconds(){try{return Math.max(0,Date.now()-statSync(machinePathV1(context.root,".cache","f","state.wal")).mtimeMs);}catch{return 0;}}});}
 finally{adoptionStop.abort();if(tickTimer!==undefined)clearInterval(tickTimer);uploader?.stop();await uploader?.idle();await adoption;}
}

/** 🛡️ Runs the graph server beside another role of the same process (the build agent, which every machine runs): a failure or a missing fleet cache only schedules another start after `pauseMilliseconds`, it never reaches the role it runs beside, and `SEMIO_GRAPH_SERVER=off` keeps the machine from serving. */
export async function superviseGraphServerV1(context:RemoteExecutionSidecarContextV1,run:(context:RemoteExecutionSidecarContextV1)=>Promise<void>=runGraph,pauseMilliseconds=30_000):Promise<void>{
 if(context.environment.SEMIO_GRAPH_SERVER==="off")return;
 while(!context.signal.aborted){
  try{await run(context);}catch(error){context.write(`graph-server stopped: ${(error as Error).message}`);}
  await sleep(pauseMilliseconds,context.signal);
 }
}
