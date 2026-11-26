import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {homedir,hostname} from "node:os";
import {join,resolve} from "node:path";
import type {LeaseHolderV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts";
import sessionSchema from "../../🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json" with {type:"json"};
import unitCacheSchema from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json" with {type:"json"};
import generationSchema from "../../🕸️graph/🕰️generations/🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {fleetTextV1} from "../🟦️.ts";
import type {HostWardenMirrorV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts";
import type {FleetBazelSlotFactV1,FleetCapacityWaiterFactV1,FleetCasFactV1,FleetOffloadFactV1,FleetFactsV1,FleetGrantFactV1,FleetHolderFactV1,FleetLanguageV1,FleetSealedEntryFactV1,FleetWorkerHostNameV1,FleetWorkerMappingV1,FleetWorkerMemoryV1,FleetWorkerTreeV1,FleetSourceV1,FleetStandingFactV1,FleetWaiterFactV1} from "../🟦️.ts";

/** 📥️ The readers of the fleet status: each one asks an existing status reader of the build (leases, capacity, Bazel slots, cache ledger, sealed store, graph generation pointer) and turns the answer into the facts the aggregation reads. A reader never takes a lease, never loads the graph and never writes; a reader that fails is reported in `sources` and leaves its section out.
 * @see ../🟦️.ts the aggregation, ../🧬️schema/🔣️.json the facts */
const policy=schema.definitions.Policy.const;
export const REPOSITORY_LEASE_DIRECTORY_FLEET_V1=".cache/b/l/m/data";
/** 🔌️ What a reader may use from its surroundings; tests replace it. */
export type FleetReaderContextV1=Readonly<{root:string;environment:NodeJS.ProcessEnv;language:FleetLanguageV1;now:()=>number}>;
export type FleetReaderV1=(context:FleetReaderContextV1)=>Promise<Partial<FleetFactsV1>>;
export type FleetReadersV1=Readonly<Record<FleetSourceV1["id"],FleetReaderV1>>;

const standingOf=(row:NonNullable<LeaseHolderV1["standing"]>):FleetStandingFactV1=>({state:row.state,...row.priority===undefined?{}:{priority:row.priority},...row.phase===undefined?{}:{phase:row.phase},...row.done===undefined?{}:{done:row.done},...row.total===undefined?{}:{total:row.total},...row.etaMinutes===undefined?{}:{etaMinutes:row.etaMinutes},...row.etaOverdue?{etaOverdue:true}:{},...row.implementationDigest===undefined?{}:{implementationDigest:row.implementationDigest},...row.implementationAgeSeconds===undefined?{}:{implementationAgeSeconds:row.implementationAgeSeconds},...row.parentPid===undefined?{}:{parentPid:row.parentPid},...row.chain===undefined?{}:{chain:[...row.chain]},...row.silentSeconds===undefined?{}:{silentSeconds:row.silentSeconds},...row.staleIds===undefined?{}:{staleIds:[...row.staleIds]}});

/** 🔒️ Leases: holders, waiters and cycles of the checkout's lease directory, with the history of the kinds of hold the waiters queue behind. */
export const readFleetLeasesV1:FleetReaderV1=async context=>{
 const {readLeaseReportV1}=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts");
 const {leaseRegistryRootV1,readLeaseHistoryV1}=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts");
 const {leaseHistoryKeyV1}=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts");
 const directory=resolve(machinePathV1(context.root,REPOSITORY_LEASE_DIRECTORY_FLEET_V1)),report=readLeaseReportV1(directory,context.environment,context.now(),0),registry=leaseRegistryRootV1(context.environment);
 const holders:FleetHolderFactV1[]=report.holders.map(row=>({pid:row.pid,resource:row.resource,mode:row.mode,class:row.class,heldSeconds:row.heldSeconds,...row.command===undefined?{}:{command:row.command},...row.session===undefined?{}:{session:row.session},...row.agent===undefined?{}:{agent:row.agent},...row.standing===undefined?{}:{standing:standingOf(row.standing)}}));
 const waiters:FleetWaiterFactV1[]=report.waiters.map(row=>({pid:row.pid,resource:row.resource,mode:row.mode,class:row.class,waitedSeconds:row.waitedSeconds,blockers:[...row.blockers],...row.position===undefined?{}:{position:row.position},...row.command===undefined?{}:{command:row.command},...row.session===undefined?{}:{session:row.session},...row.agent===undefined?{}:{agent:row.agent},...row.priority===undefined?{}:{priority:row.priority},historyKey:leaseHistoryKeyV1(row.resource,row.command??"")}));
 const history:Record<string,readonly number[]>={};
 for(const row of waiters){const key=row.historyKey!;if(!(key in history))history[key]=readLeaseHistoryV1(registry,directory,key);}
 return {leases:{holders,waiters,cycles:report.cycles.map(cycle=>cycle.members.join(" → "))},history};
};

type AgentSessionsV1=typeof import("../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🗄️store/🟦️.ts");
const agentOf=(sessions:AgentSessionsV1,environment:NodeJS.ProcessEnv,address:string|undefined,cache:Map<string,string|undefined>):string|undefined=>{
 if(address===undefined)return undefined;
 if(cache.has(address))return cache.get(address);
 const parsed=sessions.parseAgentSessionAddressV1(address),found=parsed===undefined?undefined:sessions.readAgentSessionV1(sessions.agentSessionStoreV1(sessions.agentSessionRegistryRootV1(environment),parsed.worktree),parsed.id)?.agent.id;
 cache.set(address,found);return found;
};

const agentField=(sessions:AgentSessionsV1,environment:NodeJS.ProcessEnv,address:string|undefined,cache:Map<string,string|undefined>):{agent?:string}=>{const agent=agentOf(sessions,environment,address,cache);return agent===undefined?{}:{agent};};

/** 🌡️ Capacity: the host sample, the live grants (reserved against really used), the queue in decision order and what earlier runs of each operation took. */
export const readFleetCapacityV1:FleetReaderV1=async context=>{
 const {describeHostReasonV1,hostCapacityDirectoryV1,readHostCapacityStatusV1,readHostEstimateStoreV1}=await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts");
 const {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1}=await import("../../🌡️host-capacity/🎛️control/🟦️.ts");
 const sessions=await import("../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🗄️store/🟦️.ts");
 const control=createCapacityControlV1();
 try{
  const directory=hostCapacityDirectoryV1(context.environment),policy=BUILD_HOST_CAPACITY_POLICY_V1,snapshot=await readHostCapacityStatusV1({directory,policy,control:control.control,now:context.now}),now=context.now(),agents=new Map<string,string|undefined>();
  const grants:FleetGrantFactV1[]=snapshot.grants.map(row=>({id:row.id,operation:row.operation,pid:row.pid,jobs:row.jobs,reservedMegabytes:row.reservedMegabytes,observedMegabytes:row.observedMegabytes,heldSeconds:Math.max(0,Math.floor((now-row.admittedAtMs)/1000)),...row.phase===undefined?{}:{phase:row.phase},...row.lentSinceMs===undefined?{}:{lent:true},...row.session===undefined?{}:{session:row.session},...agentField(sessions,context.environment,row.session,agents)}));
  const order=new Map(snapshot.evaluation.decisions.map((row,index)=>[row.id,index] as const)),reasons=new Map(snapshot.evaluation.decisions.map(row=>[row.id,row.reason] as const));
  const waiters:FleetCapacityWaiterFactV1[]=[...snapshot.waiters].sort((left,right)=>(order.get(left.request.id)??Infinity)-(order.get(right.request.id)??Infinity)||left.arrivedAtMs-right.arrivedAtMs).map((row,index)=>{
   const reason=reasons.get(row.request.id)??"queue-order",text=reason==="fits"||reason==="reduced"||reason==="override"||reason==="shrunk"||reason==="lent"?reason:describeHostReasonV1(context.language,reason);
   return {id:row.request.id,operation:row.request.operation,pid:row.pid,needMegabytes:row.request.fixedMegabytes+row.request.minimumJobs*row.request.perJobMegabytes,waitedSeconds:Math.max(0,Math.floor((now-row.arrivedAtMs)/1000)),reason,reasonText:text,position:index+1,...row.session===undefined?{}:{session:row.session},...agentField(sessions,context.environment,row.session,agents)};
  });
  const store=readHostEstimateStoreV1(directory),wanted=new Set([...grants.map(row=>row.operation),...waiters.map(row=>row.operation)]),active:Record<string,readonly number[]>={};
  for(const operation of wanted){const samples=store.classes[operation]?.samples;if(samples)active[operation]=samples.map(row=>row.activeMilliseconds).reverse();}
  const host=snapshot.host;
  return {host:{ageSeconds:Math.max(0,Math.floor((now-host.atMs)/1000)),platform:host.platform,cores:host.cores,totalMegabytes:host.totalMegabytes,availableMegabytes:host.availableMegabytes,pressure:host.pressure,swapUsedMegabytes:host.swapUsedMegabytes,load:host.load},capacity:{floorMegabytes:snapshot.evaluation.floorMegabytes,headroomMegabytes:snapshot.evaluation.headroomMegabytes,capacityMegabytes:Math.max(0,host.totalMegabytes-snapshot.evaluation.floorMegabytes),grants,waiters},activeMilliseconds:active};
 }finally{control.dispose();}
};

/** 🏗️ Bazel: every slot with an output base, the server process living in it and its memory, and what the slot remembers. */
export const readFleetBazelV1:FleetReaderV1=async context=>{
 const {observeTreeMegabytesV1}=await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts");
 const {repositoryCachePartitionRootDataV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts");
 const {bazelServerDirectoryV1,listBazelSlotsV1,readBazelServerPidSyncV1,readBazelSlotStateSyncV1}=await import("../../🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts");
 const server=bazelServerDirectoryV1(repositoryCachePartitionRootDataV1(context.root,"products")),slots:(Omit<FleetBazelSlotFactV1,"treeMegabytes">&{treeMegabytes:Promise<number|null>|null})[]=[];
 for(const slot of listBazelSlotsV1(server)){
  const pid=readBazelServerPidSyncV1(join(server,slot)),state=readBazelSlotStateSyncV1(server,slot);
  slots.push({slot,pid:pid??null,treeMegabytes:pid===undefined?null:observeTreeMegabytesV1(pid).catch(()=>null),...state===undefined?{}:{key:state.key,usedAt:state.usedAt,...state.session===undefined?{}:{session:state.session},...state.agent===undefined?{}:{agent:state.agent}}});
 }
 const trees=await Promise.all(slots.map(row=>row.treeMegabytes));
 return {bazel:{slotPrefix:sessionSchema.definitions.Policy.const.leases.slotPrefix,slots:slots.map((row,index)=>({...row,treeMegabytes:trees[index]!}))}};
};

/** 💾️ Storage: the images and areas of the cache ledger from statfs and the size ledger (no tree is walked) and the machine budget plan. */
export const readFleetStorageV1:FleetReaderV1=async context=>{
 const {cacheLedgerV1}=await import("../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts");
 const {machineHostFloorV1,machinePolicyV1,planMachineBudgetV1,resolveMachineTotalV1}=await import("../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts");
 const {cacheRunnerPortsV1,machineHostFreeBytesV1,machineHostTotalBytesV1,machineUsesV1,reportCacheLedgerV1}=await import("../../🧹️cache-gc/🏃️runner/🟦️.ts");
 const ledger=cacheLedgerV1(),ports=cacheRunnerPortsV1({signal:new AbortController().signal,progress:async()=>{},exclusive:async()=>undefined}),report=await reportCacheLedgerV1(context.root,ledger,ports),declared=machinePolicyV1();
 const fileText=(()=>{try{return readFileSync(join(context.root,...ledger.roots.repository.directory.split("/"),declared.file),"utf8");}catch{return undefined;}})(),total=resolveMachineTotalV1({environment:context.environment,...fileText===undefined?{}:{fileText}},declared);
 const plan=planMachineBudgetV1({totalBytes:total.bytes,hostFreeBytes:machineHostFreeBytesV1(context.root),hostTotalBytes:machineHostTotalBytesV1(context.root),uses:machineUsesV1(report)},declared);
 return {storage:{hostFreeBytes:machineHostFreeBytesV1(context.root),hostFreeFloorBytes:machineHostFloorV1(declared,machineHostTotalBytesV1(context.root)),images:report.partitions.map(row=>({id:row.id,kind:row.kind,state:row.state,usedBytes:row.usedBytes,capacityBytes:row.capacityBytes,budgetBytes:row.budgetBytes,source:row.source})),overAreas:report.areas.filter(row=>row.over).map(row=>row.id),machine:{totalBytes:plan.totalBytes,effectiveTotalBytes:plan.effectiveTotalBytes,usedBytes:plan.usedBytes,footprintBytes:plan.footprintBytes,overBytes:plan.overBytes,evictBytes:plan.evictBytes,blockedBytes:plan.blockedBytes,source:total.source}}};
};

const promotionOf=(path:string):FleetSealedEntryFactV1["last"]=>{
 try{const lines=readFileSync(path,"utf8").split("\n").filter(Boolean),row=JSON.parse(lines[lines.length-1]!) as {atMs?:unknown;added?:unknown;removed?:unknown;changed?:unknown};return Number.isSafeInteger(row.atMs)&&Number.isSafeInteger(row.added)&&Number.isSafeInteger(row.removed)&&Number.isSafeInteger(row.changed)?{atMs:row.atMs as number,added:row.added as number,removed:row.removed as number,changed:row.changed as number}:undefined;}catch{return undefined;}
};
/** 🏛️ Sealed implementation: the mode, and for every entry of the store its pointer, whether the live tree still equals the tree it was captured from (stat only, nothing is hashed unless a stamp moved), a blocked promotion and the last promotion. */
export const readFleetSealedV1:FleetReaderV1=async context=>{
 const {implementationModeV1,readSealedBlockedV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedPointerOfV1,sealedStoreDirectoryV1,sealedTreeDriftV1}=await import("../../🏛️sealed/🟦️.ts");
 const store=sealedStoreDirectoryV1(context.root),entries:FleetSealedEntryFactV1[]=[];
 let names:string[]=[];try{names=readdirSync(store,{withFileTypes:true}).filter(row=>row.isDirectory()).map(row=>row.name).sort();}catch{}
 for(const name of names){
  let pointer;try{pointer=sealedPointerOfV1(JSON.parse(readFileSync(join(store,name,"current.json"),"utf8")));}catch{pointer=undefined;}
  if(!pointer)continue;
  const entry=sealedEntryOfV1(context.root,pointer.entry),current=readSealedPointerV1(entry);if(!current)continue;
  const receipt=readSealedReceiptV1(entry,current.digest),drift=receipt?.tree?sealedTreeDriftV1(context.root,receipt.tree):undefined,blocked=readSealedBlockedV1(entry),last=promotionOf(join(entry.directory,"promotions.jsonl")),parts=current.entry.split("/");
  entries.push({entry:parts.slice(-3,-1).join("/")||current.entry,digest:current.digest,files:current.files,bytes:current.bytes,promotedAtMs:current.promotedAtMs,fresh:drift===undefined?null:drift.fresh,changed:drift?.changed.length??0,...blocked===undefined?{}:{blocked:{atMs:blocked.atMs,rows:blocked.rows.length,...blocked.rows[0]===undefined?{}:{first:`${blocked.rows[0].path.split("/").slice(-2).join("/")}:${blocked.rows[0].line}:${blocked.rows[0].column} ${blocked.rows[0].message}`.slice(0,160)}}},...last===undefined?{}:{last}});
 }
 return {sealed:{mode:implementationModeV1(context.environment),entries}};
};

const popcount=(base64:string):number=>{let count=0;for(const byte of Buffer.from(base64,"base64")){let value=byte;while(value){count+=value&1;value>>=1;}}return count;};
/** 🕰️ Generation: the published pointer and manifest of the graph store, read as plain files under their schema (the owner table of the manifest is counted, not walked), with how many owners the last verification covered. The graph itself is never loaded. */
export const readFleetGenerationV1:FleetReaderV1=async context=>{
 const {repositoryCachePartitionRootDataV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts");
 const defs=generationSchema.definitions,policy=defs.Policy.const as {pointer:string;manifests:string},directory=join(repositoryCachePartitionRootDataV1(context.root,"graph"),"generations");
 if(!existsSync(join(directory,policy.pointer)))return {};
 const pointer=JSON.parse(readFileSync(join(directory,policy.pointer),"utf8")) as {generation:number;manifest:string|null;verified?:{atMs:number;count:number;owners:string}};
 const bad=validateJsonSchemaSubset(defs.Pointer as never,pointer,generationSchema as never);if(bad.length)throw (await import("../../📖️texts/🟦️.ts")).refuseBuildV1("invalid","fleet.generation-pointer-invalid",{detail:JSON.stringify(bad[0])});
 if(pointer.manifest===null)return {};
 const path=join(directory,policy.manifests,pointer.manifest+".json"),manifest=JSON.parse(readFileSync(path,"utf8")) as {owners:Record<string,unknown>;implementation:string;built:{kind:"cold"|"delta"|"refresh";dirty:readonly string[]}};
 const shape=validateJsonSchemaSubset(defs.Manifest as never,{...manifest,owners:{}},generationSchema as never);if(shape.length)throw (await import("../../📖️texts/🟦️.ts")).refuseBuildV1("invalid","fleet.generation-manifest-invalid",{path,detail:JSON.stringify(shape[0])});
 return {generation:{number:pointer.generation,built:{kind:manifest.built.kind,dirty:manifest.built.dirty.length},owners:Object.keys(manifest.owners).length,implementation:manifest.implementation,publishedAtMs:Math.floor(statSync(path).mtimeMs),verified:pointer.verified===undefined?null:{atMs:pointer.verified.atMs,covered:popcount(pointer.verified.owners),count:pointer.verified.count}}};
};

/** 🗓️ Scheduler: the host-wide queue, read without taking its lock. */
export async function readFleetSchedulerV1(context:FleetReaderContextV1):Promise<Partial<FleetFactsV1>>{
 const [{fleetFactOfSnapshotV1,SCHEDULER_POLICY_V1},{createFileSchedulerStoreV1,schedulerDirectoryV1}]=await Promise.all([import("../../🗓️scheduler/🟦️.ts"),import("../../🗓️scheduler/💾️store/🟦️.ts")]);
 const store=createFileSchedulerStoreV1(schedulerDirectoryV1(context.environment),SCHEDULER_POLICY_V1);
 return{scheduler:fleetFactOfSnapshotV1(await store.snapshot())};
}

/** 🌍️ Fleet: the machines of the fleet with their load, pressure, queue, worker and offload counters from the scheduler of remote execution, and the counters and claims of the cache server; nothing while no scheduler is published. */
export async function readFleetLoadV1(context:FleetReaderContextV1):Promise<Partial<FleetFactsV1>>{
 const {readRemoteExecutionFleetFactsV1}=await import("../../🌐️remote-execution/🚢️status/🟦️.ts");
 const fleet=await readRemoteExecutionFleetFactsV1({root:context.root,environment:context.environment});
 return fleet===undefined?{}:{fleet};
}

/** 🏷️ The host name state of every given origin from the address sets the machines published in the fleet store: `duplicate` when another origin published the same label (or the label with macOS's conflict suffix), `named-as` when the label is the semantic name of another origin, `unknown` without a set or a usable label. Read-only; only the `hostLabel` of each published set is read, so a set from an agent that is not updated yet still counts; the same gate the agents run before they advertise.
 * @see ../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts */
export async function readFleetHostNamesV1(store:string,origins:readonly string[]):Promise<ReadonlyMap<string,FleetWorkerHostNameV1>>{
 const {fleetAdvertiseGateV1}=await import("../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts"),base=join(store,"v1","origins"),names=(()=>{try{return readdirSync(base).filter(name=>/^[a-z][a-z0-9-]{0,63}$/u.test(name)).slice(0,512);}catch{return [];}})(),labels=names.map(origin=>{try{const label=(JSON.parse(readFileSync(join(base,origin,"addresses.json"),"utf8")) as {hostLabel?:unknown}).hostLabel;return {origin,hostLabel:typeof label==="string"&&/^[a-z0-9][a-z0-9-]{0,62}$/u.test(label)?label:null};}catch{return {origin,hostLabel:null};}});
 return new Map(origins.map(origin=>{
  const label=labels.find(row=>row.origin===origin)?.hostLabel??null;
  if(label===null)return [origin,{state:"unknown",label:null,others:[]}];
  const gate=fleetAdvertiseGateV1({origin,hostLabel:label,others:labels});
  return [origin,gate.namedAs!==null?{state:"named-as",label,others:[gate.namedAs]}:gate.duplicates.length>0?{state:"duplicate",label,others:gate.duplicates}:{state:"ok",label,others:[]}];
 }) as [string,FleetWorkerHostNameV1][]);
}

/** 🩺️ Workers: every origin of the fleet store judged by the age of its readiness record and of the heartbeats of its machine, so a receipt that claims `running` with no live writer shows as offline; nothing without a fleet store. */
export async function readFleetWorkersV1(context:FleetReaderContextV1):Promise<Partial<FleetFactsV1>>{
 const [{readReadyWorkersV1,READY_OFFLINE_SECONDS_V1},{fleetStoreRootV1}]=await Promise.all([import("../../🌐️remote-execution/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts"),import("../../🌐️remote-cache/🛰️fleet/🟦️.ts")]);
 const store=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root);
 if(store===undefined||store==="")return {};
 const rows=readReadyWorkersV1({store,nowMs:context.now()});
 if(rows.length===0)return {};
 const trees=await readWorkerTreesV1(store,context),at=context.now(),hosts=await readFleetHostNamesV1(store,rows.map(row=>row.origin));
 return {workers:{offlineSeconds:READY_OFFLINE_SECONDS_V1,rows:rows.map(row=>{const mapping=readWorkerMappingV1(join(store,"v1","origins",row.origin,"slice-memory.json"),at,policy.readers.maximumMemoryRecordBytes,policy.readers.mappingFreshSeconds);return{...row,...trees?.has(row.origin)?{tree:trees.get(row.origin)!}:{},hostName:hosts.get(row.origin)!,...mapping===undefined?{}:{mapping}};})}};
}

/** 🔒️ The mapping grant queue a worker's graph read published in its `slice-memory.json` (`sources.owners.presence.mappingQueue`, the owners taken beside it): undefined for a missing, oversized, torn, stale or unrelated file, never throws. @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔒️mapping/🟦️.ts */
export function readWorkerMappingV1(file:string,nowMs:number,limit:number,freshSeconds:number):FleetWorkerMappingV1|undefined{
 try{
  const stat=statSync(file),age=Math.max(0,Math.floor((nowMs-stat.mtimeMs)/1000));
  if(stat.size>limit||age>freshSeconds)return undefined;
  const owners=(JSON.parse(readFileSync(file,"utf8")) as {sources?:{owners?:{taken?:unknown;presence?:{mappingQueue?:Record<string,any>}}}}).sources?.owners,queue=owners?.presence?.mappingQueue;
  if(queue===undefined||queue===null||typeof queue!=="object")return undefined;
  const waits=Array.isArray(queue.waiting)?queue.waiting as {step:"slot"|"grant";waitedMilliseconds:number}[]:[],longest=waits.reduce((best,row)=>row.waitedMilliseconds>best.waitedMilliseconds?row:best,{step:"grant" as const,waitedMilliseconds:0}),head=queue.head,shared=queue.granted?.shared??{},num=(value:unknown):number=>typeof value==="number"&&Number.isFinite(value)&&value>=0?value:0;
  return Object.freeze({taken:typeof owners?.taken==="number"?owners.taken:null,waiting:waits.length,step:waits.length===0?null:longest.step,longestWaitedSeconds:Math.floor(num(longest.waitedMilliseconds)/1000),holderKind:head?.holderKind==="shared"||head?.holderKind==="exclusive"?head.holderKind:null,holderPid:typeof head?.holderPid==="number"?head.holderPid:null,holderCommand:typeof head?.holderCommand==="string"?head.holderCommand.slice(0,120):null,holderSeconds:typeof head?.holderSeconds==="number"?head.holderSeconds:null,queued:typeof head?.queued==="number"?head.queued:null,sharedGrants:num(shared.count),sharedWaitedSeconds:Math.floor(num(shared.waitedMilliseconds)/1000),sharedLongestSeconds:Math.floor(num(shared.maximumMilliseconds)/1000),exclusiveHeldSeconds:Math.floor(num(queue.exclusiveHeld?.waitedMilliseconds)/1000),ageSeconds:age});
 }catch{return undefined;}
}

/** 🌳️ The paths whose state differs between two views: the union of the paths of the lines only one side shows (the first three characters of a line are its state), each counted once. */
export function treeDifferenceV1(onlyThere:readonly string[],onlyHere:readonly string[]):Readonly<{files:number;onlyThere:number;onlyHere:number}>{
 const pathOf=(line:string):string=>line.slice(3),there=new Set(onlyThere.map(pathOf)),here=new Set(onlyHere.map(pathOf)),all=new Set([...there,...here]);
 return Object.freeze({files:all.size,onlyThere:[...there].filter(path=>!here.has(path)).length,onlyHere:[...here].filter(path=>!there.has(path)).length});
}

/** 🌳️ How the tree of every worker differs from the tree of this machine, by the replica's own view comparison (`viewDiffV1`: the multiset difference of the sorted view lines each origin publishes with its digest) and not by a walk over the checkout: `null` for an origin whose view is missing or torn, no entry for an origin whose view equals the reference, `files` 0 for it as well; `undefined` when this machine is not an origin of the store or attests no view. */
export async function readWorkerTreesV1(store:string,context:FleetReaderContextV1):Promise<ReadonlyMap<string,FleetWorkerTreeV1|null>|undefined>{
 try{
  const [{viewDiffV1,readAttestationsV1,fleetReportV1},{remoteExecutionOriginV1}]=await Promise.all([import("../../../../🤝️shared-worktree/🩺️attest/🟦️.ts"),import("../../🌐️remote-execution/🌡️load/🟦️.ts")]);
  const reference=remoteExecutionOriginV1(context.root,context.environment),report=await viewDiffV1(store,reference);
  if(report.here===null)return undefined;
  const [attestations,fleet]=await Promise.all([readAttestationsV1(store),fleetReportV1(store,context.now())]),stale=attestations[reference]?.view?.stale===true,verdicts=new Map(fleet.rows.map(row=>[row.originId,row.verdict])),found=new Map<string,FleetWorkerTreeV1|null>();
  for(const [origin,attestation] of Object.entries(attestations))if(origin!==reference&&attestation.view!==null)found.set(origin,Object.freeze({reference,files:0,onlyThere:0,onlyHere:0,stale,verdict:verdicts.get(origin)??"missing"}));
  for(const row of report.differing)found.set(row.originId,Object.freeze({reference,...treeDifferenceV1(row.onlyThere,row.onlyHere),stale,verdict:verdicts.get(row.originId)??"missing"}));
  for(const origin of report.unavailable)found.set(origin,null);
  return found;
 }catch{return undefined;}
}

/** 🛡️ Turns a validated `memory.json` record (a warden state or the absent record) into the memory fact of one origin; null for no record. The last alarm is the newest of the state. */
export function workerMemoryOfV1(origin:string,mirror:HostWardenMirrorV1|null,nowMs:number):FleetWorkerMemoryV1|null{
 if(mirror===null)return null;
 const ageSeconds=Math.max(0,Math.floor((nowMs-mirror.atMs)/1000));
 if("absent" in mirror)return Object.freeze({origin,ageSeconds,absent:true,mode:null,level:null,hold:null,guardMegabytes:null,dangerMegabytes:null,availableMegabytes:null,totalMegabytes:null,swapUsedMegabytes:null,swapOutMegabytesPerSecond:null,compressedMegabytes:null,pressure:null,subjects:null,subjectMegabytes:null,suspended:null,counters:null,lastAlarm:null,overhead:null});
 const alarm=mirror.lastAlarms.reduce<(typeof mirror.lastAlarms)[number]|undefined>((newest,row)=>newest===undefined||row.atMs>=newest.atMs?row:newest,undefined);
 return Object.freeze({origin,ageSeconds,absent:false,mode:mirror.mode,level:mirror.level,hold:mirror.hold,guardMegabytes:mirror.guardMegabytes,dangerMegabytes:mirror.dangerMegabytes,availableMegabytes:mirror.host.availableMegabytes,totalMegabytes:mirror.host.totalMegabytes,swapUsedMegabytes:mirror.host.swapUsedMegabytes,swapOutMegabytesPerSecond:mirror.host.swapOutMegabytesPerSecond,compressedMegabytes:mirror.host.compressedMegabytes,pressure:mirror.host.pressure,subjects:mirror.subjects.count,subjectMegabytes:mirror.subjects.megabytes,suspended:mirror.suspended.length,counters:Object.freeze({suspended:mirror.counters.suspended,resumed:mirror.counters.resumed,cancelled:mirror.counters.cancelled}),lastAlarm:alarm===undefined?null:Object.freeze({atMs:alarm.atMs,action:alarm.action,kind:alarm.kind,pid:alarm.pid,owner:alarm.owner,megabytes:alarm.megabytes,text:Object.freeze({en:alarm.text.en,de:alarm.text.de})}),overhead:Object.freeze({tickMillisecondsP95:mirror.overhead.tickMillisecondsP95,cpuBasisPoints:mirror.overhead.cpuBasisPoints,residentMegabytes:mirror.overhead.residentMegabytes})});
}

/** 🛡️ Memory: the `memory.json` every worker mirrors to the group store (`<store>/v1/origins/<origin>/memory.json`) next to its readiness record, plus the state of the warden of this machine; no scheduler is asked. A missing, oversized, half-written or invalid file is null for that origin and never throws. */
export const readFleetMemoryV1:FleetReaderV1=async context=>{
 const [{fleetStoreRootV1},{hostCapacityDirectoryV1},{readHostWardenMirrorV1,readHostWardenStateV1}]=await Promise.all([import("../../🌐️remote-cache/🛰️fleet/🟦️.ts"),import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts"),import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")]);
 const store=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root),base=store===undefined||store===""?undefined:join(store,"v1","origins"),origins:Record<string,FleetWorkerMemoryV1|null>={},limit=policy.readers.maximumMemoryRecordBytes,at=context.now();
 let names:string[]=[];
 if(base!==undefined)try{names=readdirSync(base,{withFileTypes:true}).filter(row=>row.isDirectory()&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(row.name)).map(row=>row.name).sort();}catch{}
 for(const origin of names){
  const file=join(base!,origin,"memory.json");
  try{origins[origin]=statSync(file).size>limit?null:workerMemoryOfV1(origin,readHostWardenMirrorV1(file),at);}catch{origins[origin]=null;}
 }
 let state=null;
 try{state=readHostWardenStateV1(hostCapacityDirectoryV1(context.environment),at);}catch{}
 const name=context.environment.SEMIO_FLEET_ORIGIN||hostname(),local=state===null?null:workerMemoryOfV1(name,state,at);
 if(local!==null&&name in origins)origins[name]=local;
 return {memory:{local,origins}};
};

/** 🗄️ Host store: the cap in force with its watermarks and the last finished collection from the usage hint the collector leaves, and the leased and expired transient inputs (objects and bytes) from the census the last complete scan left. It reads no object tree and no lease ledger: only the hint and the census (bytes are null while no census exists). */
export const readFleetCasV1:FleetReaderV1=async context=>{
 const [{CAS_POLICY_V1},{casRootV1},{casEffectiveCapBytesV1,casReserveBytesV1,measureCasPartitionV1},{readCasUsageHintV1,readCasCensusV1}]=await Promise.all([import("../../🗄️cas/🧬️schema/🟦️.ts"),import("../../🗄️cas/🗃️store/🟦️.ts"),import("../../🗄️cas/🌡️partition/🟦️.ts"),import("../../🗄️cas/🧹️collector/🟦️.ts")]);
 const root=casRootV1(context.environment,process.platform,context.environment.HOME||context.environment.USERPROFILE||homedir()),hint=await readCasUsageHintV1(root),partition=measureCasPartitionV1(root),effective=partition===undefined?CAS_POLICY_V1.capBytes:casEffectiveCapBytesV1(CAS_POLICY_V1,partition,hint?.bytes);
 const census=await readCasCensusV1(root);
 const cas:FleetCasFactV1={root,...partition===undefined?{}:{reserve:{reserveBytes:casReserveBytesV1(CAS_POLICY_V1,partition),freeBytes:partition.freeBytes}},capBytes:CAS_POLICY_V1.capBytes,effectiveCapBytes:effective,highWatermarkBytes:Math.floor(effective*CAS_POLICY_V1.highWatermark),lowWatermarkBytes:Math.floor(effective*CAS_POLICY_V1.lowWatermark),incrementBytes:CAS_POLICY_V1.incrementBytes,leases:{leased:census?.classes.leased.objects??0,expired:census?.classes.expired.objects??0,leasedBytes:census?.classes.leased.bytes??null,expiredBytes:census?.classes.expired.bytes??null},lastCollection:hint===undefined?null:{atMs:hint.atMs,bytes:hint.bytes}};
 return {cas};
};

/** 🚀️ Offload: the compile slots and light slots the builds hold right now (a light slot is a unit running on the fleet), grouped by the parent process of each holder (the build), and the offload counters of this machine with the count of every decline or fall-back reason. */
export const readFleetOffloadV1:FleetReaderV1=async context=>{
 const [{readOffloadCountersV1,readOffloadReasonCountersV1},{repositoryCachePartitionRootDataV1},{claimHeldV1,claimHolderV1}]=await Promise.all([import("../../🌐️remote-execution/🚀️offload/📊️counters/🟦️.ts"),import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts"),import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts")]);
 const slots=join(context.environment.SEMIO_UNIT_CACHE_ROOT||join(repositoryCachePartitionRootDataV1(context.root,"rust"),"u"),unitCacheSchema.definitions.Policy.const.layout.slots),held:{pid:number;fleet:boolean}[]=[];
 let names:string[]=[];try{names=readdirSync(slots);}catch{}
 for(const name of names){const match=/^(light-)?\d+\.lock$/u.exec(name);if(match===null)continue;const claim=name.slice(0,-5),holder=claimHeldV1(slots,claim)?claimHolderV1(slots,claim):undefined;if(holder!==undefined)held.push({pid:holder.pid,fleet:match[1]!==undefined});}
 const parents=new Map<number,number>(),pids=[...new Set(held.map(row=>row.pid))].slice(0,policy.readers.maximumBuildOwners);
 if(pids.length>0&&process.platform!=="win32"){const listed=spawnSync("ps",["-o","pid=,ppid=","-p",pids.join(",")],{encoding:"utf8",timeout:2000});for(const line of String(listed.stdout??"").split("\n")){const [pid,parent]=line.trim().split(/\s+/u).map(Number);if(Number.isSafeInteger(pid)&&Number.isSafeInteger(parent)&&parent>0)parents.set(pid!,parent!);}}
 const builds=new Map<number,{pid:number;local:number;fleet:number}>();
 for(const row of held){const pid=parents.get(row.pid)??row.pid,build=builds.get(pid)??{pid,local:0,fleet:0};builds.set(pid,{...build,[row.fleet?"fleet":"local"]:build[row.fleet?"fleet":"local"]+1});}
 const cacheRoot=context.environment.SEMIO_UNIT_CACHE_ROOT||join(repositoryCachePartitionRootDataV1(context.root,"rust"),"u"),spool=await(async()=>{try{const {spoolBoundV1,spoolLengthV1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts");return{depth:spoolLengthV1(cacheRoot),bound:spoolBoundV1(cacheRoot)};}catch{return undefined;}})();
 const counters=readOffloadCountersV1(context.root),offload:FleetOffloadFactV1={local:held.filter(row=>!row.fleet).length,fleet:held.filter(row=>row.fleet).length,builds:[...builds.values()],out:counters.out,refused:counters.refused,fellBack:counters.fellBack,reasons:readOffloadReasonCountersV1(context.root),...spool===undefined?{}:{spool}};
 return {offload};
};

/** 🏭️ The readers of the real workspace. */
export const FLEET_READERS_V1:FleetReadersV1=Object.freeze({leases:readFleetLeasesV1,capacity:readFleetCapacityV1,bazel:readFleetBazelV1,storage:readFleetStorageV1,sealed:readFleetSealedV1,generation:readFleetGenerationV1,scheduler:readFleetSchedulerV1,fleet:readFleetLoadV1,workers:readFleetWorkersV1,memory:readFleetMemoryV1,cas:readFleetCasV1,offload:readFleetOffloadV1});

/** 🧺️ Runs every reader at once, times it and merges what it returned; a reader that throws is a failed source, never a failed command. */
export async function collectFleetFactsV1(context:FleetReaderContextV1,readers:FleetReadersV1=FLEET_READERS_V1,timeoutMilliseconds:number=policy.readers.timeoutMilliseconds):Promise<FleetFactsV1>{
 const ids=Object.keys(readers) as FleetSourceV1["id"][],sources:FleetSourceV1[]=[],parts:Partial<FleetFactsV1>[]=[];
 await Promise.all(ids.map(async id=>{
  const started=performance.now(),limit=timeoutMilliseconds;
  let timer:ReturnType<typeof setTimeout>|undefined;
  try{parts.push(await Promise.race([readers[id](context),new Promise<never>((_resolve,reject)=>{timer=setTimeout(()=>reject(Error(fleetTextV1(context.language,"error.timeout",{seconds:limit/1000}))),limit);})]));sources.push({id,ok:true,milliseconds:Math.round(performance.now()-started)});}
  catch(error){sources.push({id,ok:false,milliseconds:Math.round(performance.now()-started),error:String((error as Error).message??error).slice(0,200)});}
  finally{if(timer!==undefined)clearTimeout(timer);}
 }));
 const merged:Record<string,unknown>={};
 for(const part of parts)Object.assign(merged,part);
 return Object.freeze({...merged,version:1,atMs:context.now(),workspace:context.root,sources:ids.map(id=>sources.find(row=>row.id===id)!)}) as FleetFactsV1;
}
