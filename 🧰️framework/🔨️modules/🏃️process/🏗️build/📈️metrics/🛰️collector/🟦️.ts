import {linkSync,mkdirSync,readFileSync,realpathSync,renameSync,rmSync,statSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {isAbsolute,join,relative,resolve} from "node:path";
import {METRICS_CATALOGUE_V1,METRICS_FILES_V1,METRICS_RING_V1,METRICS_TIERS_V1,closeMetricsRingsV1,createCounterDeltasV1,openMetricsRingsV1,recordSampleV1,type MetricsRingsV1} from "../🟦️.ts";
import {ownProcessIdentityV1,processRecordAliveV1} from "../../../🔖️identity/🟦️.ts";
import {hostCapacityDirectoryV1} from "../../🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import {createMetricsPublisherV1} from "./📤️publication/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ⏱️ The cadences, bounds and file names of the collector, straight from the schema. @see ./🧬️schema/🔣️.json */
export const METRICS_COLLECTOR_POLICY_V1=schema.definitions.Policy.const;
/** 🗂️ The feeding collectors of the catalogue: name, cadence, catalogue source and the metric ids each feeds. @see ./🧬️schema/🔣️.json */
export const METRICS_COLLECTORS_V1=schema.definitions.Collectors.const as readonly Readonly<{name:string;cadence:"tick"|"slow";scope:"origin"|"fleet";source:string;ids:readonly string[]}>[];
/** 🕳️ The ids whose collector has no source at all: empty, every metric of the catalogue is fed (a machine that lacks a source still has a gap in the series, never a zero). @see ./🧬️schema/🔣️.json */
export const METRICS_UNSOURCED_V1:readonly string[]=schema.definitions.Unsourced.const;
const P=METRICS_COLLECTOR_POLICY_V1,ENUMS=schema.definitions.Enums.const,TEXTS=schema.definitions.Texts.const,KIND=new Map(METRICS_CATALOGUE_V1.map(metric=>[metric.id,metric.kind]));

const none=():Record<string,number>=>({});
export type MetricsCountsV1=Readonly<Record<string,number>>;
/** 🔌️ What the collectors read. Every reader answers `null` (or is absent) when its source does not exist on this machine, so a machine without a source has a gap, not a zero. Counter readers answer cumulative counts; the collector turns them into per-tick deltas. */
export interface MetricsSourcesV1{
 now?():number;
 cpuTimes?():Readonly<{busy:number;total:number}>|null;
 cores?():number|null;
 load?():number|null;
 memory?():Readonly<{totalBytes:number;freeBytes:number}>|null;
 warden?():Readonly<{atMs:number;totalMegabytes:number;availableMegabytes:number;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical";level:"calm"|"guard"|"danger";subjects:Readonly<{count:number;megabytes:number;cpuSeconds?:number;byKind:Readonly<Record<string,Readonly<{count:number;megabytes:number}>>>}>;headroomMegabytes:number|null;cancelled:number}>|null;
 gate?():Readonly<{totalMegabytes:number;availableMegabytes:number;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical";openFiles:number|null;attachedImages:number|null}>|null;
 volumes?():readonly Readonly<{freeBytes:number;totalBytes:number}>[]|null;
 queueDepth?():number|null;
 unitCounters?():Readonly<{localHit:number;remoteHit:number;miss:number;stored:number;skipped:number}>|null;
 cacheCounters?():Readonly<{localHit:number;localMiss:number;bytes?:number}>|null;
 remoteCache?():Readonly<{groups:Readonly<Record<string,Readonly<Record<string,number>>>>}>|null;
 remoteCacheStatus?():Readonly<{entries:number;bytes:number}>|null;
 offload?():Readonly<{out:number;refused:number;fellBack:number}>|null;
 remoteExecution?():Readonly<{running:number;queued:number;offloadedIn:number}>|null;
 pool?():Readonly<{units:number;claimed:number;published:number;failed:number;rssMegabytes:number}>|null;
 phase?():Readonly<{done:number;total:number}>|null;
 facts?():number|null;
 ready?():Readonly<{stageIndex:number;state:string;attempts:number}>|null;
 journal?():Readonly<{events:number;bytes:number}>|null;
 self?():Readonly<{cpuMicroseconds:number;rssBytes:number}>;
 cpuMicroseconds?():number;
 microseconds?():number;
}

const MEGABYTE=1048576;
const number=(value:unknown):number|undefined=>typeof value==="number"&&Number.isFinite(value)?value:undefined;
const bytes=(megabytes:number|null|undefined):number|undefined=>megabytes==null?undefined:megabytes*MEGABYTE;
const pick=(values:Record<string,number|undefined>):Record<string,number>=>{const out:Record<string,number>={};for(const id in values){const value=values[id];if(value!==undefined&&Number.isFinite(value))out[id]=value;}return out;};

/** 🧮️ Counts the collector keeps about itself between steps; the `self` collector reports them one step late (the step being measured cannot report its own duration). `stepCpuMicroseconds` is the process CPU time spent inside the collector's own steps since the last report (the work of the readiness process around it is not the collector's), `retainedBytes` what its rings, windows and counter maps hold, derived from the layout. */
interface SelfStateV1{previousCpu:number;previousAt:number;stepCpuMicroseconds:number;retainedBytes:number;tickMicroseconds:number;writeMicroseconds:number;publishMilliseconds:number;writeBytes:number;samples:number;dropped:number}

type CollectorV1=(sources:MetricsSourcesV1,state:CollectorStateV1)=>Record<string,number>;
interface CollectorStateV1{cpu?:Readonly<{busy:number;total:number}>;repo?:Readonly<{atMs:number;cpuSeconds:number}>;repoCpu?:number;self:SelfStateV1;rowBytes:number}

/** ⚙️ The CPU share of the repo process tree (percent of one core, 100 = one busy core) from two readings of the warden's cumulative CPU seconds of every process it classified (the warden adds the CPU time each classified process gained between two of its ticks, so a compiler that exited in between still counts): the rate over the span between the two warden states. A state that did not change keeps the last rate; a warden restart (the counter fell), a missing counter or a gap longer than `repoCpuMaximumGapMilliseconds` give no value (a gap in the series, never a zero). */
const repoCpuPercent=(warden:NonNullable<ReturnType<NonNullable<MetricsSourcesV1["warden"]>>>,state:CollectorStateV1):number|undefined=>{
 const cpu=warden.subjects.cpuSeconds;
 if(cpu===undefined)return undefined;
 const before=state.repo;
 if(before!==undefined&&warden.atMs===before.atMs)return state.repoCpu;
 state.repo={atMs:warden.atMs,cpuSeconds:cpu};
 const span=before===undefined?0:warden.atMs-before.atMs;
 state.repoCpu=before!==undefined&&span>0&&span<=P.repoCpuMaximumGapMilliseconds&&cpu>=before.cpuSeconds?100*(cpu-before.cpuSeconds)*1000/span:undefined;
 return state.repoCpu;
};

const COLLECTORS_V1:Readonly<Record<string,CollectorV1>>={
 cpu(sources,state){
  const now=sources.cpuTimes?.()??null,before=state.cpu;if(now!==null)state.cpu=now;
  const span=now!==null&&before!==undefined?now.total-before.total:0;
  return pick({"host.cpu.busy":span>0?Math.min(100,Math.max(0,100*(now!.busy-before!.busy)/span)):undefined,"host.cpu.load":number(sources.load?.()),"host.cpu.cores":number(sources.cores?.())});
 },
 memory(sources){
  const warden=sources.warden?.()??null,gate=warden===null?sources.gate?.()??null:null,os=sources.memory?.()??null,host=warden??gate;
  if(host!==null)return pick({"host.memory.total":host.totalMegabytes*MEGABYTE,"host.memory.available":host.availableMegabytes*MEGABYTE,"host.memory.compressed":bytes(host.compressedMegabytes),"host.memory.pressure":ENUMS.pressure[host.pressure],"host.swap.used":bytes(host.swapUsedMegabytes),"host.swap.rate":bytes(host.swapOutMegabytesPerSecond)});
  return pick({"host.memory.total":os?.totalBytes,"host.memory.available":os?.freeBytes});
 },
 warden(sources){
  const warden=sources.warden?.()??null;
  return warden===null?{}:pick({"warden.level":ENUMS.wardenLevel[warden.level],"warden.subjects.count":warden.subjects.count,"warden.subjects.memory":warden.subjects.megabytes*MEGABYTE,"warden.headroom":bytes(warden.headroomMegabytes),"warden.cancelled":warden.cancelled});
 },
 "process-tree"(sources,state){
  const warden=sources.warden?.()??null;
  if(warden===null)return none();
  const kinds=warden.subjects.byKind,running=Object.entries(kinds).filter(([name])=>name.endsWith("-unit")||name==="cargo"||name==="stage").reduce((sum,[,row])=>sum+row.count,0);
  return pick({"process.repo.count":warden.subjects.count,"process.repo.memory":warden.subjects.megabytes*MEGABYTE,"process.repo.cpu":repoCpuPercent(warden,state),"build.tasks.running":running});
 },
 self(sources,state){
  const own=sources.self?.()??{cpuMicroseconds:0,rssBytes:0},at=(sources.microseconds??(()=>performance.now()*1000))(),self=state.self,span=at-self.previousAt,measured=self.previousAt>0&&span>0;
  const whole=measured?100*(own.cpuMicroseconds-self.previousCpu)/span:undefined,collector=measured?100*self.stepCpuMicroseconds/span:undefined;
  self.previousCpu=own.cpuMicroseconds;self.previousAt=at;self.stepCpuMicroseconds=0;
  return pick({"metrics.self.cpu":collector===undefined?undefined:Math.max(0,collector),"metrics.self.memory":self.retainedBytes,"process.ready.cpu":whole===undefined?undefined:Math.max(0,whole),"process.ready.rss":own.rssBytes,"metrics.self.tick.duration":self.tickMicroseconds,"metrics.self.write.duration":self.writeMicroseconds,"metrics.self.write.bytes":self.writeBytes,"metrics.self.samples":self.samples,"metrics.self.dropped":self.dropped,"metrics.self.publish.duration":self.publishMilliseconds});
 },
 volumes(sources){
  const rows=sources.volumes?.()??[],worst=rows.length===0?undefined:rows.reduce((least,row)=>row.freeBytes<least.freeBytes?row:least),gate=sources.gate?.()??null;
  return pick({"host.disk.free":worst?.freeBytes,"host.disk.total":worst?.totalBytes,"host.files.open":number(gate?.openFiles),"host.images.attached":number(gate?.attachedImages)});
 },
 queue(sources){return pick({"build.tasks.queued":number(sources.queueDepth?.())});},
 "unit-counters"(sources){
  const units=sources.unitCounters?.()??null;
  return units===null?none():{"build.units.compiled":units.miss,"build.units.restored.local":units.localHit,"build.units.restored.fleet":units.remoteHit,"build.units.stored":units.stored,"build.units.skipped":units.skipped};
 },
 "local-cache"(sources){const cache=sources.cacheCounters?.()??null;return cache===null?{}:pick({"cache.local.hit":cache.localHit,"cache.local.miss":cache.localMiss,"cache.local.bytes":cache.bytes});},
 "remote-cache"(sources){
  const cache=sources.remoteCache?.()??null;if(cache===null)return {};
  const count=(group:string,name:string):number|undefined=>number(cache.groups[group]?.[name]);
  return pick({"cache.remote.ac.hit":count("ac","hits"),"cache.remote.ac.miss":count("ac","misses"),"cache.remote.ac.put":count("ac","puts"),"cache.remote.ac.conflict":count("ac","conflicts"),"cache.remote.cas.hit":count("cas","hits"),"cache.remote.cas.miss":count("cas","misses"),"cache.remote.cas.put":count("cas","puts"),"cache.remote.nx.hit":count("nx","hits"),"cache.remote.nx.miss":count("nx","misses"),"cache.remote.nx.put":count("nx","puts")});
 },
 "remote-cache-status"(sources){const status=sources.remoteCacheStatus?.()??null;return status===null?none():pick({"cache.remote.entries":status.entries,"cache.remote.bytes":status.bytes});},
 "offload-counters"(sources){const offload=sources.offload?.()??null;return offload===null?{}:pick({"exec.offload.out":offload.out,"exec.offload.fellback":offload.fellBack,"exec.refused":offload.refused});},
 "remote-execution"(sources){const row=sources.remoteExecution?.()??null;return row===null?none():{"exec.queue.running":row.running,"exec.queue.queued":row.queued,"exec.offload.in":row.offloadedIn};},
 "owner-pool"(sources){
  const pool=sources.pool?.()??null,phase=sources.phase?.()??null;
  return pick({"graph.read.owners":phase?.total??pool?.units,"graph.read.owners.done":phase?.done??pool?.published,"graph.pool.claimed":pool?.claimed,"graph.pool.published":pool?.published,"graph.pool.failed":pool?.failed,"graph.pool.memory":bytes(pool?.rssMegabytes)});
 },
 "graph-service"(sources){return pick({"graph.read.facts":number(sources.facts?.())});},
 ready(sources){
  const ready=sources.ready?.()??null;
  return ready===null?{"ready.state":0}:pick({"ready.stage":ready.stageIndex,"ready.state":(ENUMS.readyState as Readonly<Record<string,number>>)[ready.state]??0,"ready.attempts":ready.attempts});
 },
 journal(sources){const head=sources.journal?.()??null;return head===null?none():{"replica.journal.events":head.events,"replica.journal.bytes":head.bytes};},
};

/** 🧪️ Runs one collector by name (a pure function of its sources and the small state it keeps between steps); used by the collector loop and by the tests that feed every collector from injected sources. */
export function runMetricsCollectorV1(name:string,sources:MetricsSourcesV1,state:CollectorStateV1):Record<string,number>{
 const collector=COLLECTORS_V1[name];
 if(collector===undefined)throw new Error(`Unknown metrics collector ${name} / Unbekannter Metrik-Sammler ${name}`);
 return collector(sources,state);
}

/** 🆕️ A fresh collector state with the row size of the rings of `idCount` metrics (the bytes one second row writes). */
export function createMetricsCollectorStateV1(idCount:number=METRICS_CATALOGUE_V1.length):CollectorStateV1{
 const rowBytes=2*METRICS_RING_V1.stampBytes+idCount*METRICS_RING_V1.slotBytes+8;
 return {self:{previousCpu:0,previousAt:0,stepCpuMicroseconds:0,retainedBytes:METRICS_TIERS_V1.length*(rowBytes+5*8*idCount)+idCount*256,tickMicroseconds:0,writeMicroseconds:0,publishMilliseconds:0,writeBytes:0,samples:0,dropped:0},rowBytes};
}

const text=(language:"en"|"de",key:keyof typeof TEXTS.en,values:Readonly<Record<string,string>>):string=>TEXTS[language][key].replace(/\{(\w+)\}/gu,(_,name:string)=>values[name]??"");
const both=(key:keyof typeof TEXTS.en,values:Readonly<Record<string,string>>):string=>`${text("en",key,values)} / ${text("de",key,values)}`;

/** 🪪️ Who asks for the writer lock of a ring directory: the `role` it collects for (`origin`, or `fleet:<source>`) and the `checkout` it runs for. @see ./🧬️schema/🔣️.json */
export interface MetricsLockClaimV1{readonly role:string;readonly checkout?:string}
/** 🔒️ The answer to a claim: whether this process now writes, and, when not, why (`refused`: the claim may never hold this lock; `foreign`: another live process holds it, named in `detail`). */
export interface MetricsLockResultV1{readonly held:boolean;readonly refusal?:"refused"|"foreign";readonly detail?:string}
interface LockRecordV1{readonly version?:number;readonly pid?:number;readonly startMs?:number;readonly role?:string;readonly checkout?:string}

/** 🧪️ Whether a path lies inside the operating system's temporary directory (by name or by real path): the checkout of a test or a scratch member, never the worktree of a machine. */
export function metricsScratchPathV1(path:string|undefined):boolean{
 if(path===undefined||path==="")return false;
 const roots=new Set<string>([tmpdir()]);
 try{roots.add(realpathSync(tmpdir()));}catch{}
 return[...roots].some(root=>{const inside=relative(root,path);return inside!==""&&!inside.startsWith("..")&&!isAbsolute(inside);});
}

const insideV1=(path:string,root:string):boolean=>{const inside=relative(root,path);return inside===""||(!inside.startsWith("..")&&!isAbsolute(inside));};
const ROLE=new RegExp(P.lockRolePattern,"u");

/** 🛡️ Why a claim may not hold the lock of `directory`, or `undefined` when it may: the role must be a known one, and the machine-wide ring directory (the host capacity directory of this machine) belongs only to a real collector: not to a claim whose checkout is inside the temporary directory, not to a claim without a checkout for the machine role and not to a process under a test runner. A private directory is free for everyone.
 * @see ./🧬️schema/🔣️.json */
export function metricsLockRefusalV1(directory:string,claim:MetricsLockClaimV1,environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 if(!ROLE.test(claim.role))return `unknown role ${claim.role}`;
 let machine:string|undefined;
 try{machine=join(hostCapacityDirectoryV1(),METRICS_FILES_V1.directory);}catch{}
 if(machine===undefined||!insideV1(resolve(directory),machine))return undefined;
 if(metricsScratchPathV1(claim.checkout))return `checkout ${claim.checkout} is inside the temporary directory`;
 if(environment["NODE_ENV"]==="test")return "a test runner never writes the machine rings";
 if(claim.role==="origin"&&(claim.checkout===undefined||claim.checkout===""))return "no checkout named";
 return undefined;
}

const holderName=(holder:LockRecordV1):string=>`${holder.role??"unnamed collector"} pid ${holder.pid??"?"}${holder.checkout===undefined?"":` of ${holder.checkout}`}`;

/** 🔒️ Takes the writer lock of a ring directory for `claim` (`collector.lock`, written whole under a temporary name and linked into place, so a reader never sees half a record). The record names the process by its identity (pid and start time), its role and its checkout. A lock whose holder is gone (by identity, a reused pid is no holder) is taken over once; a live holder keeps it, and so does a record that cannot be read until it is `lockUnreadableMilliseconds` old. Answers whether this process now writes and, if not, why.
 * @see ./🧬️schema/🔣️.json */
export function acquireMetricsLockV1(directory:string,claim:MetricsLockClaimV1):MetricsLockResultV1{
 const refusal=metricsLockRefusalV1(directory,claim);
 if(refusal!==undefined)return {held:false,refusal:"refused",detail:refusal};
 const path=join(directory,P.lockFile),mine=ownProcessIdentityV1();
 mkdirSync(directory,{recursive:true});
 const record:LockRecordV1={version:1,pid:mine.pid,startMs:mine.startMs,role:claim.role,...claim.checkout===undefined?{}:{checkout:claim.checkout}};
 for(let attempt=0;attempt<2;attempt++){
  const temporary=`${path}.${mine.pid}.tmp`;
  try{
   try{writeFileSync(temporary,JSON.stringify(record));linkSync(temporary,path);return {held:true};}
   catch(error){
    if((error as NodeJS.ErrnoException).code!=="EEXIST"){
     try{writeFileSync(path,JSON.stringify(record),{flag:"wx"});return {held:true};}
     catch(second){if((second as NodeJS.ErrnoException).code!=="EEXIST")return {held:false,refusal:"refused",detail:(second as Error).message};}
    }
   }
   let holder:LockRecordV1;
   try{holder=JSON.parse(readFileSync(path,"utf8")) as LockRecordV1;if(typeof holder!=="object"||holder===null)throw new Error("not a record");}
   catch{
    try{if(Date.now()-statSync(path).mtimeMs<P.lockUnreadableMilliseconds)return {held:false,refusal:"foreign",detail:"unreadable record"};unlinkSync(path);}catch{}
    continue;
   }
   if(holder.pid===mine.pid&&holder.startMs===mine.startMs)return holder.role===claim.role?{held:true}:{held:false,refusal:"foreign",detail:holderName(holder)};
   if(processRecordAliveV1(holder.pid,holder.startMs))return {held:false,refusal:"foreign",detail:holderName(holder)};
   try{unlinkSync(path);}catch{}
  }finally{try{unlinkSync(temporary);}catch{}}
 }
 return {held:false,refusal:"foreign",detail:"lock contended"};
}

/** 🔓️ Gives the writer lock back when this process holds it (same identity and, when `claim` is given, the same role). */
export function releaseMetricsLockV1(directory:string,claim?:MetricsLockClaimV1):void{
 const path=join(directory,P.lockFile),mine=ownProcessIdentityV1();
 try{const holder=JSON.parse(readFileSync(path,"utf8")) as LockRecordV1;if(holder.pid===mine.pid&&holder.startMs===mine.startMs&&(claim===undefined||holder.role===claim.role))unlinkSync(path);}catch{}
}

export interface FleetMetricsOptionsV1{
 readonly directory:string;
 readonly sources:MetricsSourcesV1;
 readonly store?:Readonly<{root:string;origin:string}>;
 readonly fleet?:Readonly<{source:string;root?:string}>;
 readonly report?:(line:string)=>void;
 readonly ids?:readonly string[];
 readonly checkout?:string;
}
export interface FleetMetricsStepV1{readonly at:number;readonly owner:boolean;readonly accepted:number;readonly dropped:number;readonly unknown:number;readonly published:number;readonly failed:number}
export interface FleetMetricsCollectorV1{step(atMs?:number):FleetMetricsStepV1;close():void}

const writeBaseline=(directory:string,atMs:number,readings:ReadonlyMap<string,number>):void=>{
 const file=join(directory,P.baselineFile),temporary=`${file}.${process.pid}.tmp`;
 try{writeFileSync(temporary,JSON.stringify({version:1,atMs,readings:Object.fromEntries(readings)}));renameSync(temporary,file);}catch{try{rmSync(temporary,{force:true});}catch{}}
};
const readBaseline=(directory:string,atMs:number):Readonly<Record<string,number>>=>{
 try{
  const value=JSON.parse(readFileSync(join(directory,P.baselineFile),"utf8")) as {version?:number;atMs?:number;readings?:Record<string,number>};
  if(value.version!==1||typeof value.atMs!=="number"||atMs-value.atMs>P.baselineMaximumAgeMilliseconds||atMs<value.atMs||typeof value.readings!=="object"||value.readings===null)return {};
  return value.readings;
 }catch{return {};}
};

/** 📡️ The collector of one machine (or, with `fleet`, of one fleet-scope source such as the scheduler or the cache server: it runs only the fleet collectors of that source, keeps rings of only their metrics and publishes to `<store>/v1/fleet/metrics/<source>/`, so the fleet has one series written by the machine that runs the service): `step` reads the due collectors (the host gauges every tick, the counter and status files every `slowMilliseconds`), turns counter readings into deltas (a baseline survives a restart of the process, a counter that fell counts as restarted), writes one sample into the local rings and, every `publishMilliseconds`, appends the rows closed since the last look to the published ring files into the fleet store. One writer per directory (`collector.lock`); a second process steps idle until the holder is gone. Never throws: a collector that fails costs its own metrics for that step.
 * @see ./🧬️schema/🔣️.json */
export function createFleetMetricsCollectorV1(options:FleetMetricsOptionsV1):FleetMetricsCollectorV1{
 const sources=options.sources,now=sources.now??Date.now,microseconds=sources.microseconds??(()=>performance.now()*1000),report=options.report??(()=>{});
 const fleet=options.fleet,feeding=METRICS_COLLECTORS_V1.filter(collector=>fleet===undefined?collector.scope==="origin":collector.scope==="fleet"&&collector.source===fleet.source),fed=new Set(feeding.flatMap(collector=>collector.ids)),ids=options.ids??METRICS_CATALOGUE_V1.map(metric=>metric.id).filter(id=>fleet===undefined||fed.has(id));
 const state=createMetricsCollectorStateV1(ids.length),deltas=createCounterDeltasV1(),last=new Map<string,number>(),failed=new Set<string>();
 const target=fleet!==undefined?(fleet.root===undefined?undefined:join(fleet.root,"v1",P.fleetDirectory,METRICS_FILES_V1.directory,fleet.source)):options.store===undefined?undefined:join(options.store.root,"v1","origins",options.store.origin,METRICS_FILES_V1.directory);
 const publisher=target===undefined?undefined:createMetricsPublisherV1({from:options.directory,to:target,now,report});
 const claim:MetricsLockClaimV1={role:fleet===undefined?"origin":`fleet:${fleet.source}`,...options.checkout===undefined?{}:{checkout:options.checkout}};
 let handle:MetricsRingsV1|undefined,owner=false,slowAt=-Infinity,publishAt=-Infinity,retryAt=-Infinity,noted="",closed=false;
 const cpu=sources.cpuMicroseconds??(()=>0);
 const open=(atMs:number):boolean=>{
  if(owner)return true;
  if(atMs<retryAt)return false;
  try{
   const lock=acquireMetricsLockV1(options.directory,claim);
   if(!lock.held){
    const key=`${lock.refusal}:${lock.detail}`;
    if(key!==noted){noted=key;report(lock.refusal==="refused"?both("refused",{directory:options.directory,reason:lock.detail??""}):both("foreign",{directory:options.directory,holder:lock.detail??""}));}
    return false;
   }
   handle=openMetricsRingsV1({directory:options.directory,ids});owner=true;
   const saved=readBaseline(options.directory,atMs);for(const id in saved){if(KIND.get(id)==="counter"){deltas.delta(id,saved[id]!);last.set(id,saved[id]!);}}
   return true;
  }catch(error){
   retryAt=atMs+P.openRetryMilliseconds;const reason=(error as Error).message;
   if(reason!==noted){noted=reason;report(both("opening",{directory:options.directory,reason}));}
   return false;
  }
 };
 const empty=(at:number,published=0,failures=0):FleetMetricsStepV1=>({at,owner,accepted:0,dropped:0,unknown:0,published,failed:failures});
 return Object.freeze({
  step(atMs:number=now()):FleetMetricsStepV1{
   if(closed)return empty(atMs);
   const before=cpu();
   try{return work(atMs);}finally{state.self.stepCpuMicroseconds+=Math.max(0,cpu()-before);}
  },
  close():void{
   if(closed)return;closed=true;
   if(owner&&handle!==undefined){try{closeMetricsRingsV1(handle);}catch{}try{publisher?.publish(now());}catch{}releaseMetricsLockV1(options.directory,claim);}
  },
 });
 function work(atMs:number):FleetMetricsStepV1{
   const started=microseconds();
   if(!open(atMs))return empty(atMs);
   const slow=atMs-slowAt>=P.slowMilliseconds;if(slow)slowAt=atMs;
   if(slow&&!acquireMetricsLockV1(options.directory,claim).held){
    try{closeMetricsRingsV1(handle!);}catch{}
    handle=undefined;owner=false;retryAt=atMs+P.slowMilliseconds;
    return empty(atMs);
   }
   const values:Record<string,number>={};
   for(const collector of feeding){
    if(collector.cadence==="slow"&&!slow)continue;
    let readings:Record<string,number>;
    try{readings=runMetricsCollectorV1(collector.name,sources,state);failed.delete(collector.name);}
    catch(error){if(!failed.has(collector.name)){failed.add(collector.name);report(both("failed",{name:collector.name,reason:(error as Error).message}));}continue;}
    for(const id in readings){
     const value=readings[id]!;
     if(KIND.get(id)==="counter"){last.set(id,value);const delta=deltas.delta(id,value);if(delta!==null)values[id]=delta;}
     else values[id]=value;
    }
   }
   state.self.tickMicroseconds=Math.round(microseconds()-started);
   const wrote=microseconds(),record=recordSampleV1(handle!,atMs,values);
   state.self.writeMicroseconds=Math.round(microseconds()-wrote);
   if(record.accepted>0){state.self.samples+=1;state.self.writeBytes+=state.rowBytes;}
   state.self.dropped+=record.dropped+record.unknown;
   if(slow)writeBaseline(options.directory,atMs,new Map([...last].filter(([id])=>!id.startsWith("metrics.self."))));
   let published=0,failures=0;
   if(publisher!==undefined&&atMs-publishAt>=P.publishMilliseconds){
    publishAt=atMs;const began=microseconds(),outcome=publisher.publish(atMs);
    published=outcome.copied;failures=outcome.failed;state.self.publishMilliseconds=Math.round((microseconds()-began)/1000);
   }
   return {at:atMs,owner,accepted:record.accepted,dropped:record.dropped,unknown:record.unknown,published,failed:failures};
 }
}
