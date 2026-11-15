import {createHash} from "node:crypto";
import {existsSync,statSync} from "node:fs";
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
import {GRAPH_WAVE_POLICY_V1,renderGraphWaveTextV1} from "../../../🕸️graph/📈️wave-row/🟨️.mjs";
import type {GraphWaveRowV1} from "../../../🕸️graph/📈️wave-row/🟨️.mjs";
import {ownerCostContextV1,ownerCostPathV1,readOwnerCostFileV1,recordOwnerCostFileV1,waveEvidenceV1} from "../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import {wantedTextV1} from "../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import {createGraphReadWatchV1,graphReadEndTextV1,runWantedServerV1,serveStateFileV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1} from "../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import {createGenerationUploaderV1,localGenerationUploadSourceV1,WAVE_UPLOAD_POLICY_V1} from "../../../🕸️graph/🕰️generations/📡️wave-upload/🟦️.ts";
import {journalCursorStoreV1} from "../../../🏡️resident/🕰️graph-hooks/📖️direct/🟦️.ts";
import {snapshotStoreV1} from "../../../🪪️graph-state/📍️cursor/🟦️.ts";
import {readRemoteExecutionHostViewV1,remoteExecutionOriginV1} from "../../🌡️load/🟦️.ts";
import {sleep,type RemoteExecutionSidecarContextV1} from "../🧰️shared/🟦️.ts";
import {awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,readGraphLeaseEntriesV1} from "./🔬️stall/🟦️.ts";
import {measureReadTreeV1,memoryOverrunV1} from "./🔬️stall/🌳️tree/🟦️.ts";

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

/** 🔭️ One graph read of the server (the wave that had exactly one project at its memory peak proves that project's cost, which `costPath` keeps for the planning of every reader): `repo show projects` as a background refresh (it gives way between waves, `SEMIO_GRAPH_BACKGROUND=1`, and neither asks the fleet nor waits for it), watched by its progress and not by a fixed budget: it runs for as long as it publishes waves, and is stopped with SIGINT (SIGKILL ten seconds later) only after `inferStallMilliseconds` without one or after `inferMaximumMilliseconds`; what is published stays, so the next read starts from it. The stderr is read as it arrives, the last line of a failed read and the generation of the newest wave are the answer. */
export async function inferGraphReadV1(input:Readonly<{command:string;cwd:string;environment:Readonly<Record<string,string|undefined>>;reason:"request"|"refresh";signal:AbortSignal|undefined;language:"en"|"de";steerDirectory?:string;costPath?:string;policy?:WantedServerPolicyV1;now?:()=>number;say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void;diagnose?:(input:Readonly<{pid:number;silentMilliseconds:number;lastOutput:string}>)=>Promise<readonly string[]>;measure?:(pid:number)=>Promise<Readonly<{footprintMegabytes:number|undefined;grantMegabytes:number|undefined;pressure:string}>|undefined>}>):Promise<WantedInferV1>{
 const policy=input.policy??wantedServerPolicyV1,now=input.now??Date.now,started=now(),watch=createGraphReadWatchV1(policy,now,row=>{try{input.onWave?.(row);}catch{}input.say?.(renderGraphWaveTextV1(row,input.language));const proof=waveEvidenceV1(row);if(proof!==undefined&&input.costPath!==undefined)recordOwnerCostFileV1(input.costPath,{name:proof.name,kind:"wave",megabytes:proof.megabytes,seconds:row.seconds,rows:0,stopped:false,atMs:now()},ownerCostContextV1().ceilingMegabytes);});
 const child=Bun.spawn([input.command,"show","projects"],{cwd:input.cwd,stdin:"ignore",stdout:"ignore",stderr:"pipe",env:{...input.environment,SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1",SEMIO_LEASE_PRIORITY:input.reason==="request"?"interactive":"background",SEMIO_GRAPH_WAVE_ROWS:"1",...input.steerDirectory===undefined?{}:{SEMIO_GRAPH_STEER_DIRECTORY:input.steerDirectory}} as Record<string,string>});
 const stop=()=>{try{child.kill("SIGINT");}catch{}setTimeout(()=>{try{child.kill("SIGKILL");}catch{}},10_000).unref();};
 let ended:"stalled"|"too-long"|"memory-overrun"|undefined,tail="",overrun:{footprintMegabytes:number|undefined;grantMegabytes:number|undefined}|undefined,sampling=false;
 let dumpedAt=-Infinity;
 const timer=setInterval(()=>{
  const verdict=watch.verdict();if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}
  if(input.measure!==undefined&&ended===undefined&&!sampling&&watch.silentMilliseconds()>=policy.memoryOverrunMilliseconds){
   sampling=true;void input.measure(child.pid).then(sample=>{if(sample!==undefined&&ended===undefined&&memoryOverrunV1({...sample,silentMilliseconds:watch.silentMilliseconds(),policy})){overrun=sample;ended="memory-overrun";stop();}}).catch(()=>undefined).finally(()=>{setTimeout(()=>{sampling=false;},policy.memorySampleMilliseconds).unref();});
  }
  if(input.diagnose!==undefined&&ended===undefined&&watch.silentMilliseconds()>=policy.stallDumpMilliseconds&&now()-dumpedAt>=policy.stallDumpRepeatMilliseconds){
   dumpedAt=now();void input.diagnose({pid:child.pid,silentMilliseconds:watch.silentMilliseconds(),lastOutput:tail}).then(lines=>{for(const line of lines)input.say?.(line);}).catch(()=>undefined);
  }
 },policy.watchMilliseconds);
 input.signal?.addEventListener("abort",stop,{once:true});
 try{
  const decoder=new TextDecoder(),pump=(async()=>{for await(const chunk of child.stderr){const text=decoder.decode(chunk,{stream:true});watch.feed(text);tail=(tail+text).slice(-4096);}})();
  const[,status]=await Promise.all([pump,child.exited]);
  return{ok:status===0&&ended===undefined,status,seconds:(now()-started)/1000,generation:watch.generation(),detail:ended==="memory-overrun"?wantedTextV1("memoryOverrun",input.language,{footprint:overrun?.footprintMegabytes??0,grant:Math.round(overrun?.grantMegabytes??0),minutes:(watch.silentMilliseconds()/60000).toFixed(1)}):ended!==undefined?graphReadEndTextV1(ended,input.language,policy):status===0?input.reason:tail.trim().split("\n").filter(line=>!line.includes(GRAPH_WAVE_POLICY_V1.marker)).at(-1)?.slice(0,200)??""};
 }finally{clearInterval(timer);input.signal?.removeEventListener("abort",stop);}
}


/** 🕸️ The graph server of a machine: runs {@link runWantedServerV1} in its own supervised process (a graph read holds gigabytes and must never take the build agent down). A machine whose physical memory is below `serveTotalMegabytes` reports no headroom, so it neither serves requests nor refreshes: a full graph read does not fit beside its own work. Its reads are ordinary `repo show projects` commands, so inference runs under the fleet claims and the host capacity gate, and records and the generation reach the fleet cache through the paths every read already uses. A machine without a fleet cache waits and looks again. */
export async function runGraph(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const environment=graphExchangeEnvironmentV1({...context.environment,SEMIO_WORKSPACE_ROOT:context.root}),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment).language as "en"|"de",origin=remoteExecutionOriginV1(context.root,environment),policy=wantedServerPolicyV1;
 let settings=graphFleetSettingsV1(context.root,environment);
 while(settings===undefined&&!context.signal.aborted){context.write("graph-server: no fleet cache yet");await sleep(60_000,context.signal);settings=graphFleetSettingsV1(context.root,environment);}
 if(settings===undefined)return;
 const client=graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:context.root})}),store=environment.SEMIO_FLEET_STORE_ROOT,steerDirectory=join(context.root,".cache","f","graph-server"),costPath=ownerCostPathV1(context.root);
 const infer=async(reason:"request"|"refresh",signal:AbortSignal|undefined,published:readonly string[]=[]):Promise<WantedInferV1>=>{
  if(published.length>0)context.write(wantedTextV1("continuing",language,{count:published.length}));
  await awaitGraphReadLeasesV1({blockers:()=>graphReadBlockersV1({entries:readGraphLeaseEntriesV1(environment),prefixes:policy.leaseResourcePrefixes,ownPid:process.pid,nowMs:Date.now()}),language,say:context.write,signal});
  if(signal?.aborted)return{ok:false,status:1,seconds:0,generation:null,detail:"stopped"};
  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,say:context.write,onWave:row=>uploader?.wave(row),diagnose:async input=>describeGraphReadStallV1({rootPid:input.pid,entries:readGraphLeaseEntriesV1(environment),view:await readRemoteExecutionHostViewV1(environment).catch(()=>undefined),nowMs:Date.now(),silentMilliseconds:input.silentMilliseconds,lastOutput:input.lastOutput,language,tree:await measureReadTreeV1(input.pid).catch(()=>undefined)}),measure:async pid=>{const tree=await measureReadTreeV1(pid),view=await readRemoteExecutionHostViewV1(environment).catch(()=>undefined);return{footprintMegabytes:tree.footprintMegabytes??tree.rssMegabytes,grantMegabytes:view?.grants.find(row=>row.pid===pid)?.reservedMegabytes,pressure:view?.host.pressure??"normal"};}});
 };
 const serving=(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes;
 let uploader:ReturnType<typeof createGenerationUploaderV1>|undefined,tickTimer:ReturnType<typeof setInterval>|undefined;
 if(serving){
  const{repositoryCacheBudgetV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),generations=join(repositoryCacheBudgetV1(context.root).partitions.graph.directory,"m","generations"),snapshots=snapshotStoreV1(context.root),cursorStore=journalCursorStoreV1(context.root,environment);
  uploader=createGenerationUploaderV1({language,say:context.write,statePath:join(steerDirectory,WAVE_UPLOAD_POLICY_V1.stateFile),
   source:localGenerationUploadSourceV1({workspace:context.root,root:generations,environment,cursors:{snapshotFor:async generation=>{const entry=cursorStore.get("g"+generation);if(!entry||entry.carry)return undefined;const bytes=await snapshots.bytes(entry.instance);return bytes?{instance:entry.instance,bytes}:undefined;},install:async()=>undefined}}),
   async may(){const host=(await readRemoteExecutionHostViewV1(environment)).host;return host.pressure==="normal"&&host.availableMegabytes>=WAVE_UPLOAD_POLICY_V1.minimumFreeMegabytes;}});
  tickTimer=setInterval(()=>uploader?.tick(),policy.roundMilliseconds);tickTimer.unref?.();
 }
 const work=context.work;
 try{await runWantedServerV1({client,owner:origin,...work===undefined?{}:{work:{restartPending:()=>work.restartPending,report:busy=>{const was=work.busy;work.busy=busy;if(was&&!busy)work.wake?.();}}},platform:graphPlatformV1(),language,steerDirectory,costs:()=>readOwnerCostFileV1(costPath,ownerCostContextV1().ceilingMegabytes),signal:context.signal,state:serveStateFileV1(join(steerDirectory,"serve-state.json")),say:context.write,sleep,infer,
  requesters:async()=>{
   if(store===undefined)return [];
   const view=await fleetGroupViewV1(context.root,environment),own=`${view?.id??environment.SEMIO_FLEET_GROUP??""}-`,peers=Object.keys(view?.peerStores??{}),names=(await readdir(join(store,"v1","origins"))).filter(name=>name!==origin&&(view===undefined||name.startsWith(own)||!peers.some(peer=>name.startsWith(`${peer}-`))));
   return view===undefined?names:[...names,...await forwardableRequestersV1({ownStore:store,peers:view.peerStores}).catch(()=>[])];
  },
  async headroom(){return graphServerHeadroomV1((await readRemoteExecutionHostViewV1(environment)).host,policy.serveTotalMegabytes);},
  async mayServe(){return(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes;},
  async mayRefresh(){const key=createHash("sha256").update(JSON.stringify(["graph-refresh",graphPlatformV1()])).digest("hex");return(await client.claim(key,origin,policy.refreshIntervalMilliseconds).catch(()=>({state:"fault"}))).state==="granted";},
  async quietMilliseconds(){try{return Math.max(0,Date.now()-statSync(join(context.root,".cache","f","state.wal")).mtimeMs);}catch{return 0;}}});}
 finally{if(tickTimer!==undefined)clearInterval(tickTimer);uploader?.stop();await uploader?.idle();}
}

/** 🛡️ Runs the graph server beside another role of the same process (the build agent, which every machine runs): a failure or a missing fleet cache only schedules another start after `pauseMilliseconds`, it never reaches the role it runs beside, and `SEMIO_GRAPH_SERVER=off` keeps the machine from serving. */
export async function superviseGraphServerV1(context:RemoteExecutionSidecarContextV1,run:(context:RemoteExecutionSidecarContextV1)=>Promise<void>=runGraph,pauseMilliseconds=30_000):Promise<void>{
 if(context.environment.SEMIO_GRAPH_SERVER==="off")return;
 while(!context.signal.aborted){
  try{await run(context);}catch(error){context.write(`graph-server stopped: ${(error as Error).message}`);}
  await sleep(pauseMilliseconds,context.signal);
 }
}
