import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {leaseMedianV1} from "../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/⏱️eta/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🚢️ Fleet status: one read-only picture of a workspace that many agents share. This file is the pure aggregation (facts in, report out) and the two renderers; the readers that fill the facts live in `📥️readers`, the command in `⌨️command`. No lease, no graph, no file is touched here.
 * @see ./🧬️schema/🔣️.json the closed schema of facts, report, policy and the EN+DE texts, ./🧪️tests/🐍️.py the independent oracle */
export type FleetLanguageV1="en"|"de";
export type FleetModeV1="shared"|"exclusive";
export type FleetPriorityV1="interactive"|"agent"|"background";
export type FleetHolderStateV1="unknown"|"progressing"|"silent"|"stuck"|"stale";
export type FleetStandingFactV1=Readonly<{state:FleetHolderStateV1;priority?:FleetPriorityV1;phase?:string;done?:number;total?:number;etaMinutes?:number;etaOverdue?:boolean;implementationDigest?:string;implementationAgeSeconds?:number;parentPid?:number;chain?:readonly number[];silentSeconds?:number;staleIds?:readonly string[]}>;
export type FleetHolderFactV1=Readonly<{pid:number;resource:string;mode:FleetModeV1;class:string;heldSeconds:number;command?:string;session?:string;agent?:string;standing?:FleetStandingFactV1}>;
export type FleetWaiterFactV1=Readonly<{pid:number;resource:string;mode:FleetModeV1;class:string;waitedSeconds:number;blockers:readonly string[];position?:number;command?:string;session?:string;agent?:string;priority?:FleetPriorityV1;historyKey?:string}>;
export type FleetGrantFactV1=Readonly<{id:string;operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes:number|null;heldSeconds:number;phase?:string;lent?:boolean;session?:string;agent?:string}>;
export type FleetCapacityWaiterFactV1=Readonly<{id:string;operation:string;pid:number;needMegabytes:number;waitedSeconds:number;reason:string;reasonText:string;position:number;session?:string;agent?:string}>;
export type FleetSourceV1=Readonly<{id:"leases"|"capacity"|"bazel"|"storage"|"sealed"|"generation"|"scheduler"|"fleet"|"workers"|"memory"|"cas"|"offload";ok:boolean;milliseconds:number;error?:string}>;
export type FleetImageFactV1=Readonly<{id:string;kind:"partition"|"control";state:"mounted"|"detached"|"directory"|"absent";usedBytes:number|null;capacityBytes:number|null;budgetBytes:number|null;source:"statfs"|"recorded"|"walked"|"unknown"}>;
export type FleetMachineV1=Readonly<{totalBytes:number;effectiveTotalBytes:number;usedBytes:number;footprintBytes:number;overBytes:number;evictBytes:number;blockedBytes:number;source:"argument"|"environment"|"file"|"default"}>;
export type FleetSealedEntryFactV1=Readonly<{entry:string;digest:string;files:number;bytes:number;promotedAtMs:number;fresh:boolean|null;changed:number;blocked?:Readonly<{atMs:number;rows:number;first?:string}>;last?:Readonly<{atMs:number;added:number;removed:number;changed:number}>}>;
export type FleetBazelSlotFactV1=Readonly<{slot:string;pid:number|null;key?:string;usedAt?:number;session?:string;agent?:string;treeMegabytes:number|null}>;
/** 🧺️ Everything the aggregation may look at; a section a reader could not fill is absent and reported through `sources`. */
export type FleetFactsV1=Readonly<{
 version:1;atMs:number;workspace:string;
 host?:Readonly<{ageSeconds:number;platform:string;cores:number|null;totalMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";swapUsedMegabytes:number|null;load:number|null}>;
 leases?:Readonly<{holders:readonly FleetHolderFactV1[];waiters:readonly FleetWaiterFactV1[];cycles:readonly string[]}>;
 capacity?:Readonly<{floorMegabytes:number;headroomMegabytes:number;capacityMegabytes:number;grants:readonly FleetGrantFactV1[];waiters:readonly FleetCapacityWaiterFactV1[]}>;
 activeMilliseconds?:Readonly<Record<string,readonly number[]>>;
 history?:Readonly<Record<string,readonly number[]>>;
 bazel?:Readonly<{slotPrefix:string;slots:readonly FleetBazelSlotFactV1[]}>;
 storage?:Readonly<{hostFreeBytes:number|null;hostFreeFloorBytes:number;images:readonly FleetImageFactV1[];overAreas:readonly string[];machine:FleetMachineV1}>;
 sealed?:Readonly<{mode:"sealed"|"live"|"promote";entries:readonly FleetSealedEntryFactV1[]}>;
 generation?:Readonly<{number:number;built:Readonly<{kind:"cold"|"delta"|"refresh";dirty:number}>;owners:number;implementation:string;publishedAtMs:number|null;verified:Readonly<{atMs:number;covered:number;count:number}>|null}>;
 scheduler?:FleetSchedulerFactV1;
 fleet?:FleetLoadFactV1;
 workers?:FleetWorkersFactV1;
 memory?:FleetMemoryFactV1;
 cas?:FleetCasFactV1;
 offload?:FleetOffloadFactV1;
 sources:readonly FleetSourceV1[];
}>;
export type FleetSchedulerJobFactV1=Readonly<{key:string;class:string;priority:string;depth:number;agents:readonly string[];tickets:number;reserveMegabytes:number;waitedSeconds:number;position?:number;runSeconds?:number;progress?:Readonly<{phase:string;done:number;total:number}>;stopping?:boolean}>;
export type FleetSchedulerFactV1=Readonly<{budgetMegabytes:number;reservedMegabytes:number;running:readonly FleetSchedulerJobFactV1[];queued:readonly FleetSchedulerJobFactV1[];agents:readonly Readonly<{agent:string;running:number;queued:number;served:number;percent:number}>[]}>;
/** 🌍️ The machines of the fleet as the scheduler of remote execution sees them (load, pressure, queue, worker, offload and unit cache counters) and what the cache server counts. */
export type FleetLoadFactV1=Readonly<{scheduler:string;expireSeconds:number;machines:readonly Readonly<{origin:string;platform:string;pressure:"normal"|"warn"|"critical";load:number|null;cores:number;running:number;queued:number;headroom:boolean;ageSeconds:number;worker:"absent"|"idle"|"busy"|"refusing";why?:Readonly<{code:string;en:string;de:string}>;offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number;cacheSkipReasons?:Readonly<Record<string,number>>}>[];queued:number;executing:number;offloadRejected:number;refused:number;retried:number;cache?:Readonly<{hits:number;misses:number;puts:number}>;claims?:Readonly<{active:number;waiting:number;oldestSeconds:number}>;keys?:Readonly<{origins:number;units:number;diffs:readonly Readonly<{crate:string;slot:string;origins:readonly string[];differs:readonly string[];sameSource:boolean;values?:Readonly<Record<string,Readonly<Record<string,string>>>>}>[];byComponent:Readonly<Record<string,number>>}>;retention?:FleetRetentionFactV1}>;
/** 🗄️ The host content store as its collector records it: the cap in force and its watermarks, the leases of transient offload inputs (bytes are null above the stat bound) and the last finished collection; nothing is walked. */
export type FleetCasFactV1=Readonly<{root:string;capBytes:number;effectiveCapBytes:number;highWatermarkBytes:number;lowWatermarkBytes:number;incrementBytes:number;leases:Readonly<{leased:number;expired:number;leasedBytes:number|null;expiredBytes:number|null}>;lastCollection:Readonly<{atMs:number;bytes:number}>|null}>;
/** 🍂️ What the last staleness sweep of the fleet cache found, as the status endpoint of the cache server reports it. */
export type FleetRetentionFactV1=Readonly<{version:1;mode:"off"|"dry-run"|"enforce";sweptAt:number;entries:number;declared:number;live:number;grace:number;stale:Readonly<Record<string,number>>;unreached:number;retired:number;refused:Readonly<Record<string,number>>;examples:readonly string[]}>;
/** 🚀️ Units of the builds of this machine in compile slots (local) and light slots (on the fleet), and the offload counters with every decline or fall-back reason. */
export type FleetOffloadFactV1=Readonly<{local:number;fleet:number;builds:readonly Readonly<{pid:number;local:number;fleet:number}>[];out:number;refused:number;fellBack:number;reasons:Readonly<Record<string,number>>}>;
/** 🩺️ One worker of the fleet: what its readiness record claims (`recorded`) next to what is really known (`liveness`, from the age of the record and of the heartbeats of its machine). */
export type FleetWorkerFactV1=Readonly<{origin:string;platform:string|null;liveness:"absent"|"offline"|"silent"|"working"|"ready"|"idle";recorded:string|null;stage:string|null;stageIndex:number;stageCount:number;attempts:number;recordAgeSeconds:number|null;heartbeatAgeSeconds:number|null;refusal:Readonly<{code:string;en:string;de:string}>|null}>;
export type FleetWorkersFactV1=Readonly<{offlineSeconds:number;rows:readonly FleetWorkerFactV1[]}>;
/** 🛡️ What one host warden published as `memory.json` (read from the group store, never from the scheduler): host numbers, level and mode, interventions so far and the last alarm with its own EN and DE text; an absent record has every number null. */
export type FleetWorkerMemoryV1=Readonly<{origin:string;ageSeconds:number;absent:boolean;mode:"observe"|"enforce"|null;level:"calm"|"guard"|"danger"|null;hold:boolean|null;guardMegabytes:number|null;dangerMegabytes:number|null;availableMegabytes:number|null;totalMegabytes:number|null;swapUsedMegabytes:number|null;swapOutMegabytesPerSecond:number|null;compressedMegabytes:number|null;pressure:"normal"|"warn"|"critical"|null;subjects:number|null;subjectMegabytes:number|null;suspended:number|null;counters:Readonly<{suspended:number;resumed:number;cancelled:number}>|null;lastAlarm:Readonly<{atMs:number;action:"suspend"|"resume"|"cancel";kind:string;pid:number;owner:string;megabytes:number;text:Readonly<{en:string;de:string}>}>|null;overhead:Readonly<{tickMillisecondsP95:number;cpuBasisPoints:number;residentMegabytes:number}>|null}>;
/** 🗂️ The memory records by worker origin (null: nothing readable) and the record of the warden of this machine. */
export type FleetMemoryFactV1=Readonly<{local:FleetWorkerMemoryV1|null;origins:Readonly<Record<string,FleetWorkerMemoryV1|null>>}>;
export type FleetFindingV1=Readonly<{id:string;severity:"alert"|"warn"|"info";subject:string;values:Readonly<Record<string,string|number>>;text:string}>;
export type FleetReportV1=Readonly<{version:1;atMs:number;workspace:string;summary:Row;memory:Row;runs:readonly Row[];queues:Row;bazel:Row;storage:Row;sealed:Row;generation:Row|null;scheduler?:Row;fleet?:Row;workers?:Row;cas?:Row;retention?:Row|null;offload?:Row;findings:readonly FleetFindingV1[];sources:readonly FleetSourceV1[]}>;
type Row=Record<string,any>;

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const as Readonly<Record<string,Readonly<Record<FleetLanguageV1,string>>>>,fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{([A-Za-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
/** 🗣️ One text of the schema in the chosen language. */
export function fleetTextV1(language:FleetLanguageV1,key:string,values:Readonly<Record<string,string|number>>={}):string{const row=messages[key];if(!row)throw Error(`fleet-text-unknown:${key}`);return fill(row[language],values);}
/** ⏱️ A duration in the units of the schema: seconds below two minutes, minutes below two hours, hours with one decimal. */
export function fleetDurationV1(language:FleetLanguageV1,seconds:number):string{
 if(seconds<120)return fleetTextV1(language,"unit.s",{n:seconds});
 if(seconds<7200)return fleetTextV1(language,"unit.min",{n:Math.floor(seconds/60+0.5)});
 const tenths=Math.floor(seconds/360+0.5);return fleetTextV1(language,"unit.h",{n:`${Math.floor(tenths/10)}.${tenths%10}`});
}
const GIB=2**30,gib=(bytes:number):string=>{const tenths=Math.floor((bytes*10+GIB/2)/GIB);return `${Math.floor(tenths/10)}.${tenths%10}`;};
const shortResource=(resource:string):string=>{const limit=policy.runs.resourceCharacters;return resource.length>limit?"…"+resource.slice(-(limit-1)):resource;};
/** 🔤️ Orders strings by code point (the order every other language sorts by), not by UTF-16 unit, so an emoji path sorts the same everywhere. */
const compare=(left:string,right:string):number=>{if(left===right)return 0;const a=Array.from(left),b=Array.from(right);for(let index=0;index<Math.min(a.length,b.length);index++){const difference=a[index]!.codePointAt(0)!-b[index]!.codePointAt(0)!;if(difference!==0)return difference<0?-1:1;}return a.length<b.length?-1:a.length>b.length?1:0;};
const SEVERITY:Readonly<Record<string,number>>={alert:0,warn:1,info:2};
const startsWithAny=(text:string,prefixes:readonly string[]):boolean=>prefixes.some(prefix=>text.startsWith(prefix));
const medianSeconds=(durations:readonly number[]|undefined):number|undefined=>{const median=leaseMedianV1(durations??[]);return median===undefined?undefined:Math.round(median/1000);};

/** ⏳️ Waiting time of every lease waiter: what the holders still need (their own time left, unknown when they report none) plus the typical hold of every waiter ahead; a waiter nobody blocks waits no time. */
function leaseEstimates(facts:FleetFactsV1):Map<string,number|null>{
 const found=new Map<string,number|null>(),leases=facts.leases;if(!leases)return found;
 const byResource=new Map<string,FleetWaiterFactV1[]>();
 for(const waiter of leases.waiters)byResource.set(waiter.resource,[...byResource.get(waiter.resource)??[],waiter]);
 for(const [resource,waiters] of byResource){
  const holders=leases.holders.filter(row=>row.resource===resource),times=holders.map(row=>row.standing?.etaMinutes===undefined?null:row.standing.etaMinutes*60),remaining=times.length===0?0:times.some(time=>time===null)?null:Math.max(...times as number[]);
  const ordered=[...waiters].sort((left,right)=>(left.position??Infinity)-(right.position??Infinity)||right.waitedSeconds-left.waitedSeconds||left.pid-right.pid);
  let ahead:number|null=0;
  for(const waiter of ordered){
   found.set(`${waiter.pid}\0${resource}`,waiter.blockers.length===0?0:remaining===null||ahead===null?null:remaining+ahead);
   const service=waiter.historyKey===undefined?undefined:medianSeconds(facts.history?.[waiter.historyKey]);
   ahead=ahead===null||service===undefined?null:ahead+service;
  }
 }
 return found;
}

/** 🧠️ Time until each memory waiter is admitted, in queue order: the grants give their memory back after the typical duration of their operation less the time they ran (a grant of an operation without history never does), a waiter is admitted once the freed memory covers its need and every waiter ahead is admitted, and its own memory comes back after its operation's typical duration. */
function capacityEstimates(facts:FleetFactsV1):Map<string,number|null>{
 const found=new Map<string,number|null>(),capacity=facts.capacity;if(!capacity)return found;
 const pending:{time:number;megabytes:number}[]=[];
 const insert=(time:number,megabytes:number):void=>{let at=pending.length;while(at>0&&(pending[at-1]!.time>time||pending[at-1]!.time===time&&pending[at-1]!.megabytes>megabytes))at--;pending.splice(at,0,{time,megabytes});};
 for(const grant of capacity.grants){const typical=medianSeconds(facts.activeMilliseconds?.[grant.operation]);if(typical!==undefined)insert(Math.max(0,typical-grant.heldSeconds),grant.reservedMegabytes);}
 let free=capacity.headroomMegabytes,now=0,unknown=false;
 for(const waiter of [...capacity.waiters].sort((left,right)=>left.position-right.position||compare(left.id,right.id))){
  if(unknown){found.set(waiter.id,null);continue;}
  const need=capacity.capacityMegabytes>0?Math.min(waiter.needMegabytes,capacity.capacityMegabytes):waiter.needMegabytes;
  while(pending.length>0&&pending[0]!.time<=now)free+=pending.shift()!.megabytes;
  while(free<need&&pending.length>0){const next=pending.shift()!;now=Math.max(now,next.time);free+=next.megabytes;while(pending.length>0&&pending[0]!.time<=now)free+=pending.shift()!.megabytes;}
  if(free<need){unknown=true;found.set(waiter.id,null);continue;}
  found.set(waiter.id,now);free-=need;
  const typical=medianSeconds(facts.activeMilliseconds?.[waiter.operation]);if(typical!==undefined)insert(now+typical,need);
 }
 return found;
}

/** 🏃️ Joins every process the ledgers know into runs: an invocation belongs to the run of its outermost known ancestor (the parent it reports, else the last of its chain), so the build that holds the memory grant and the children that hold its leases are one run. */
function buildRuns(facts:FleetFactsV1,leaseWait:Map<string,number|null>,capacityWait:Map<string,number|null>,language:FleetLanguageV1):Row[]{
 const holders=facts.leases?.holders??[],waiters=facts.leases?.waiters??[],grants=facts.capacity?.grants??[],memoryWaiters=facts.capacity?.waiters??[];
 const known=new Set<number>([...holders,...waiters,...grants,...memoryWaiters].map(row=>row.pid)),parent=new Map<number,number>();
 for(const row of holders){const standing=row.standing;if(!standing||parent.has(row.pid))continue;const up=standing.parentPid??standing.chain?.[standing.chain.length-1];if(up!==undefined)parent.set(row.pid,up);}
 const rootOf=(pid:number):number=>{let at=pid;for(let step=0;step<policy.runs.parentWalk;step++){const up=parent.get(at);if(up===undefined||up===at||!known.has(up))break;at=up;}return at;};
 const groups=new Map<number,{pids:Set<number>;holders:FleetHolderFactV1[];waiters:FleetWaiterFactV1[];grants:FleetGrantFactV1[];memoryWaiters:FleetCapacityWaiterFactV1[]}>();
 const group=(pid:number)=>{const root=rootOf(pid);let row=groups.get(root);if(!row){row={pids:new Set(),holders:[],waiters:[],grants:[],memoryWaiters:[]};groups.set(root,row);}row.pids.add(pid);return row;};
 for(const row of holders)group(row.pid).holders.push(row);
 for(const row of waiters)group(row.pid).waiters.push(row);
 for(const row of grants)group(row.pid).grants.push(row);
 for(const row of memoryWaiters)group(row.pid).memoryWaiters.push(row);
 const standings=holders.filter(row=>row.standing?.implementationDigest!==undefined&&row.standing.implementationAgeSeconds!==undefined).map(row=>row.standing!).sort((left,right)=>left.implementationAgeSeconds!-right.implementationAgeSeconds!||compare(left.implementationDigest!,right.implementationDigest!)),newest=standings[0]?.implementationDigest;
 const runs:Row[]=[];
 for(const [root,found] of groups){
  const pids=[...found.pids].sort((left,right)=>left-right),own=(rows:readonly {pid:number}[])=>[...rows].sort((left,right)=>Number(right.pid===root)-Number(left.pid===root)||left.pid-right.pid);
  const withSession=[...own(found.holders),...own(found.waiters),...own(found.grants),...own(found.memoryWaiters)] as {command?:string;session?:string;agent?:string}[];
  const command=withSession.find(row=>row.command)?.command,session=withSession.find(row=>row.session)?.session,agent=withSession.find(row=>row.agent)?.agent;
  const grant=[...found.grants].sort((left,right)=>right.heldSeconds-left.heldSeconds||left.pid-right.pid)[0];
  const standing=[...found.holders].filter(row=>row.standing!==undefined).sort((left,right)=>Number(right.mode==="exclusive")-Number(left.mode==="exclusive")||right.heldSeconds-left.heldSeconds||left.pid-right.pid)[0],detail=[...found.holders].filter(row=>row.standing?.phase!==undefined).sort((left,right)=>Number(right.mode==="exclusive")-Number(left.mode==="exclusive")||right.heldSeconds-left.heldSeconds||left.pid-right.pid)[0]?.standing;
  const waits=[...found.waiters.map(row=>({kind:"lease" as const,pid:row.pid,waited:row.waitedSeconds,subject:shortResource(row.resource),estimate:leaseWait.get(`${row.pid}\0${row.resource}`)??null,priority:row.priority})),...found.memoryWaiters.map(row=>({kind:"capacity" as const,pid:row.pid,waited:row.waitedSeconds,subject:row.operation,estimate:capacityWait.get(row.id)??null,priority:undefined}))].sort((left,right)=>right.waited-left.waited||left.pid-right.pid||compare(left.kind,right.kind));
  const ages=[...found.holders.map(row=>row.heldSeconds),...found.waiters.map(row=>row.waitedSeconds),...found.grants.map(row=>row.heldSeconds),...found.memoryWaiters.map(row=>row.waitedSeconds)];
  const observedRows=found.grants.filter(row=>row.observedMegabytes!==null),admitted=found.grants.length===0?null:found.grants.reduce((sum,row)=>sum+row.reservedMegabytes,0),observed=observedRows.length===0?null:observedRows.reduce((sum,row)=>sum+row.observedMegabytes!,0);
  const run:Row={pid:root,pids,state:waits.length===0?"running":found.holders.length+found.grants.length===0?"waiting":"blocked",ageSeconds:ages.length===0?0:Math.max(...ages)};
  if(agent!==undefined)run.agent=agent;if(session!==undefined)run.session=session;
  if(command!==undefined)run.command=command.slice(0,policy.runs.commandCharacters);
  if(grant)run.operation=grant.operation;
  const phase=detail?.phase??grant?.phase;if(phase!==undefined)run.phase=phase;
  if(detail?.done!==undefined)run.done=detail.done;if(detail?.total!==undefined)run.total=detail.total;
  const chosen=standing?.standing??detail;
  if(chosen?.etaMinutes!==undefined)run.etaSeconds=chosen.etaMinutes*60;if(chosen?.etaOverdue)run.etaOverdue=true;
  const priority=chosen?.priority??waits.find(row=>row.priority!==undefined)?.priority;if(priority!==undefined)run.priority=priority;
  if(standing?.standing){const digest=standing.standing.implementationDigest;run.implementation={state:standing.standing.state,...digest===undefined?{}:{digest:digest.slice(0,policy.digestCharacters)},...standing.standing.implementationAgeSeconds===undefined?{}:{ageSeconds:standing.standing.implementationAgeSeconds},freshness:digest===undefined?"unknown":digest===newest?"current":"older"};}
  run.memory={admittedMegabytes:admitted,observedMegabytes:observed,deltaMegabytes:admitted===null||observed===null?null:observed-admitted,jobs:found.grants.reduce((sum,row)=>sum+row.jobs,0)};
  if(waits[0])run.waitingFor={kind:waits[0].kind,subject:waits[0].subject,waitedSeconds:waits[0].waited,estimatedWaitSeconds:waits[0].estimate};
  const exclusive=[...new Map([...found.holders].filter(row=>row.mode==="exclusive").sort((left,right)=>right.heldSeconds-left.heldSeconds||compare(left.resource,right.resource)).map(row=>[row.resource,shortResource(row.resource)] as const)).values()].slice(0,policy.runs.exclusiveShown);
  run.exclusive=exclusive;
  if(found.grants.some(row=>row.lent===true))run.lent=true;
  runs.push(run);
 }
 void language;
 const order:Readonly<Record<string,number>>={running:0,blocked:1,waiting:2};
 return runs.sort((left,right)=>order[left.state]!-order[right.state]!||right.ageSeconds-left.ageSeconds||left.pid-right.pid);
}

/** 🕸️ The graph custody as the leases show it: the exclusive builder, the computers that hold claims, the slots in use and everyone waiting on one of them. */
function buildGraph(facts:FleetFactsV1,leaseWait:Map<string,number|null>):Row{
 const holders=facts.leases?.holders??[],waiters=facts.leases?.waiters??[],graph=policy.graph;
 const builder=holders.filter(row=>row.mode==="exclusive"&&startsWithAny(row.resource,graph.builderResources)).sort((left,right)=>right.heldSeconds-left.heldSeconds||left.pid-right.pid)[0];
 const waiting=waiters.filter(row=>startsWithAny(row.resource,[...graph.builderResources,...graph.claimResources,...graph.slotResources])).sort((left,right)=>right.waitedSeconds-left.waitedSeconds||left.pid-right.pid);
 const head=waiting[0];
 return {
  builder:builder===undefined?null:{pid:builder.pid,...builder.standing?.phase===undefined?{}:{phase:builder.standing.phase},...builder.standing?.done===undefined?{}:{done:builder.standing.done},...builder.standing?.total===undefined?{}:{total:builder.standing.total},heldSeconds:builder.heldSeconds,state:builder.standing?.state??"unknown",etaSeconds:builder.standing?.etaMinutes===undefined?null:builder.standing.etaMinutes*60},
  computers:new Set(holders.filter(row=>startsWithAny(row.resource,graph.claimResources)).map(row=>row.pid)).size,
  slots:new Set(holders.filter(row=>startsWithAny(row.resource,graph.slotResources)).map(row=>`${row.pid}\0${row.resource}`)).size,
  waiting:new Set(waiting.map(row=>row.pid)).size,
  longestWaitSeconds:head?.waitedSeconds??0,
  estimatedWaitSeconds:head===undefined?null:leaseWait.get(`${head.pid}\0${head.resource}`)??null,
 };
}

const fleetSection=(fact:FleetLoadFactV1):Row=>({scheduler:fact.scheduler,machines:[...fact.machines].sort((left,right)=>compare(left.origin,right.origin)),queued:fact.queued,executing:fact.executing,offloadedIn:fact.machines.reduce((sum,row)=>sum+row.offloadedIn,0),offloadedOut:fact.machines.reduce((sum,row)=>sum+row.offloadedOut,0),refused:fact.refused,declined:fact.offloadRejected,retried:fact.retried,cacheHit:fact.machines.reduce((sum,row)=>sum+row.cacheHit,0),cacheMiss:fact.machines.reduce((sum,row)=>sum+row.cacheMiss,0),...fact.cache===undefined?{}:{cache:fact.cache},...fact.claims===undefined?{}:{claims:fact.claims},...fact.keys===undefined?{}:{keys:fact.keys}});

const casSection=(fact:FleetCasFactV1,atMs:number):Row=>{const used=fact.lastCollection?.bytes??null;return{root:fact.root,usedBytes:used,usedBasisPoints:used===null||fact.effectiveCapBytes===0?null:Math.floor(used*10000/fact.effectiveCapBytes),capBytes:fact.capBytes,effectiveCapBytes:fact.effectiveCapBytes,highWatermarkBytes:fact.highWatermarkBytes,lowWatermarkBytes:fact.lowWatermarkBytes,incrementBytes:fact.incrementBytes,leases:fact.leases,lastCollection:fact.lastCollection===null?null:{atMs:fact.lastCollection.atMs,ageSeconds:Math.max(0,Math.floor((atMs-fact.lastCollection.atMs)/1000)),bytes:fact.lastCollection.bytes}};};
const retentionSection=(fact:FleetRetentionFactV1,atMs:number):Row=>({mode:fact.mode,sweptAt:fact.sweptAt,ageSeconds:fact.sweptAt===0?null:Math.max(0,Math.floor((atMs-fact.sweptAt)/1000)),entries:fact.entries,judged:fact.declared,live:fact.live,grace:fact.grace,stale:Object.values(fact.stale).reduce((sum,count)=>sum+count,0),staleByReason:fact.stale,unreached:fact.unreached,undeclared:Math.max(0,fact.entries-fact.declared),retired:fact.retired,refused:fact.refused});
const offloadSection=(fact:FleetOffloadFactV1):Row=>({local:fact.local,fleet:fact.fleet,builds:[...fact.builds].sort((left,right)=>left.pid-right.pid),out:fact.out,refused:fact.refused,fellBack:fact.fellBack,deadline:fact.reasons.deadline??0,lost:fact.reasons["operation-lost"]??0,silent:fact.reasons.silent??0,reasons:fact.reasons});
const workersSection=(fact:FleetWorkersFactV1,memory:FleetMemoryFactV1|undefined):Row=>{const rows=[...fact.rows].sort((left,right)=>compare(left.origin,right.origin)).map(row=>({...row,memory:memory?.origins[row.origin]??null})),count=(state:string)=>rows.filter(row=>row.liveness===state).length;return{offlineSeconds:fact.offlineSeconds,counts:{ready:count("ready"),working:count("working"),idle:count("idle"),silent:count("silent"),offline:count("offline"),absent:count("absent")},rows};};

const natural=(left:string,right:string):number=>{const a=/^([a-z]*)(\d+)$/u.exec(left),b=/^([a-z]*)(\d+)$/u.exec(right);return a&&b?compare(a[1]!,b[1]!)||Number(a[2])-Number(b[2]):compare(left,right);};

const schedulerSection=(fact:FleetSchedulerFactV1):Row=>({budgetMegabytes:fact.budgetMegabytes,reservedMegabytes:fact.reservedMegabytes,running:fact.running.length,queued:fact.queued.length,waiting:fact.queued.length,longestWaitSeconds:Math.max(0,...fact.queued.map(row=>row.waitedSeconds)),sharedTickets:[...fact.running,...fact.queued].reduce((sum,row)=>sum+row.tickets-1,0),jobs:[...fact.running,...fact.queued].slice(0,policy.runs.maximumRows),agents:[...fact.agents]});

/** 🧮️ Aggregates the facts into the report: runs joined across the ledgers, the queues with their waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation, the last graph generation and the findings worth an agent's attention. A pure function of its arguments. */
export function summarizeFleetV1(facts:FleetFactsV1,language:FleetLanguageV1):FleetReportV1{
 const leaseWait=leaseEstimates(facts),capacityWait=capacityEstimates(facts),runs=buildRuns(facts,leaseWait,capacityWait,language),leases=facts.leases,capacity=facts.capacity,atMs=facts.atMs;
 const grants=capacity?.grants??[],observedGrants=grants.filter(row=>row.observedMegabytes!==null);
 const memory:Row={totalMegabytes:facts.host?.totalMegabytes??0,availableMegabytes:facts.host?.availableMegabytes??0,floorMegabytes:capacity?.floorMegabytes??0,spareMegabytes:capacity?.headroomMegabytes??0,pressure:facts.host?.pressure??"normal",swapUsedMegabytes:facts.host?.swapUsedMegabytes??null,load:facts.host?.load??null,sampleAgeSeconds:facts.host?.ageSeconds??null,cores:facts.host?.cores??null,admittedMegabytes:grants.reduce((sum,row)=>sum+row.reservedMegabytes,0),observedMegabytes:observedGrants.reduce((sum,row)=>sum+row.observedMegabytes!,0),unobservedGrants:grants.length-observedGrants.length,overReservedMegabytes:observedGrants.reduce((sum,row)=>sum+Math.max(0,row.reservedMegabytes-row.observedMegabytes!),0),underReservedMegabytes:observedGrants.reduce((sum,row)=>sum+Math.max(0,row.observedMegabytes!-row.reservedMegabytes),0),grants:grants.length,waiters:capacity?.waiters.length??0,...facts.memory===undefined?{}:{warden:facts.memory.local}};
 const byResource=new Map<string,{waiters:FleetWaiterFactV1[]}>();
 for(const waiter of leases?.waiters??[]){const row=byResource.get(waiter.resource)??{waiters:[]};row.waiters.push(waiter);byResource.set(waiter.resource,row);}
 const queues=[...byResource].map(([resource,row])=>({resource,longest:Math.max(...row.waiters.map(waiter=>waiter.waitedSeconds)),waiters:[...row.waiters].sort((left,right)=>(left.position??Infinity)-(right.position??Infinity)||right.waitedSeconds-left.waitedSeconds||left.pid-right.pid)})).sort((left,right)=>right.longest-left.longest||compare(left.resource,right.resource));
 const leaseQueues=queues.slice(0,policy.queues.maximumLeaseQueues).map(queue=>({resource:shortResource(queue.resource),class:queue.waiters[0]!.class,holders:(leases?.holders??[]).filter(row=>row.resource===queue.resource).sort((left,right)=>right.heldSeconds-left.heldSeconds||left.pid-right.pid).map(row=>({pid:row.pid,mode:row.mode,heldSeconds:row.heldSeconds,etaSeconds:row.standing?.etaMinutes===undefined?null:row.standing.etaMinutes*60,...row.standing===undefined?{}:{state:row.standing.state}})),waiters:queue.waiters.map(row=>({pid:row.pid,mode:row.mode,position:row.position??null,waitedSeconds:row.waitedSeconds,...row.priority===undefined?{}:{priority:row.priority},estimatedWaitSeconds:leaseWait.get(`${row.pid}\0${row.resource}`)??null}))}));
 const bazelSlots=facts.bazel?.slots??[],prefix=facts.bazel?.slotPrefix??"",slotHolders=new Map<string,number>();
 for(const row of leases?.holders??[])if(prefix!==""&&row.resource.startsWith(prefix)&&row.mode==="exclusive")slotHolders.set(row.resource.slice(prefix.length),row.pid);
 const servers=[...bazelSlots].sort((left,right)=>natural(left.slot,right.slot)).map(row=>({slot:row.slot,live:row.pid!==null,held:slotHolders.has(row.slot),pid:row.pid,holderPid:slotHolders.get(row.slot)??null,treeMegabytes:row.treeMegabytes,...row.key===undefined?{}:{key:row.key},...row.usedAt===undefined?{}:{usedAt:row.usedAt},...row.agent===undefined?{}:{agent:row.agent}}));
 const storage=facts.storage;
 const images=[...storage?.images??[]].sort((left,right)=>compare(left.id,right.id)).map(row=>({...row,usedBasisPoints:row.usedBytes===null||row.capacityBytes===null||row.capacityBytes===0?null:Math.floor(row.usedBytes*10000/row.capacityBytes)}));
 const sealedEntries=(facts.sealed?.entries??[]).map(row=>({entry:row.entry,digest:row.digest.slice(0,policy.digestCharacters),files:row.files,bytes:row.bytes,promotedAtMs:row.promotedAtMs,ageSeconds:Math.max(0,Math.floor((atMs-row.promotedAtMs)/1000)),fresh:row.fresh,changed:row.changed,...row.blocked===undefined?{}:{blocked:row.blocked},...row.last===undefined?{}:{last:row.last}})).sort((left,right)=>compare(left.entry,right.entry));
 const generation=facts.generation===undefined?null:{number:facts.generation.number,built:facts.generation.built,owners:facts.generation.owners,implementation:facts.generation.implementation.slice(0,policy.digestCharacters),publishedAtMs:facts.generation.publishedAtMs,ageSeconds:facts.generation.publishedAtMs===null?null:Math.max(0,Math.floor((atMs-facts.generation.publishedAtMs)/1000)),verified:facts.generation.verified===null?null:{atMs:facts.generation.verified.atMs,ageSeconds:Math.max(0,Math.floor((atMs-facts.generation.verified.atMs)/1000)),covered:facts.generation.verified.covered,count:facts.generation.verified.count,basisPoints:facts.generation.verified.count===0?0:Math.floor(facts.generation.verified.covered*10000/facts.generation.verified.count)}};
 const findings:FleetFindingV1[]=[],say=(id:string,severity:"alert"|"warn"|"info",subject:string,values:Record<string,string|number>)=>{findings.push({id,severity,subject,values,text:fleetTextV1(language,`finding.${id}`,values)});};
 for(const cycle of leases?.cycles??[])say("cycle","alert",cycle,{subject:cycle});
 const seen=new Set<string>();
 for(const row of [...leases?.holders??[]].sort((left,right)=>left.pid-right.pid||compare(left.resource,right.resource))){
  const state=row.standing?.state,key=`${row.pid}\0${state}`;
  if((state==="stuck"||state==="stale")&&!seen.has(key)){seen.add(key);
   if(state==="stuck")say("holder-stuck","alert",`pid ${row.pid}`,{pid:row.pid,resource:shortResource(row.resource),minutes:Math.floor(((row.standing?.silentSeconds??0)+30)/60)});
   else say("holder-stale","warn",`pid ${row.pid}`,{pid:row.pid,resource:shortResource(row.resource),ids:(row.standing?.staleIds??[]).join(", ")});}
 }
 if(facts.host?.pressure!==undefined&&facts.host.pressure!=="normal")say("memory-pressure",facts.host.pressure==="critical"?"alert":"warn","host",{pressure:facts.host.pressure});
 if(facts.host!==undefined&&facts.host.ageSeconds>policy.findings.sampleStaleSeconds)say("sample-stale","warn","host",{age:fleetDurationV1(language,facts.host.ageSeconds)});
 for(const row of [...grants].sort((left,right)=>left.pid-right.pid||compare(left.id,right.id)))if(row.observedMegabytes!==null&&row.reservedMegabytes>0&&row.observedMegabytes*10000>row.reservedMegabytes*policy.findings.underReservedBasisPoints)say("under-reserved","warn",`pid ${row.pid}`,{pid:row.pid,observed:row.observedMegabytes,reserved:row.reservedMegabytes});
 for(const row of [...capacity?.waiters??[]].sort((left,right)=>left.position-right.position))if(row.waitedSeconds>=policy.findings.capacityWaitSeconds)say("capacity-wait","warn",`pid ${row.pid}`,{pid:row.pid,waited:fleetDurationV1(language,row.waitedSeconds),operation:row.operation,need:row.needMegabytes});
 const waited=new Map<string,FleetWaiterFactV1>();
 for(const row of leases?.waiters??[]){const key=`${row.pid}\0${row.resource}`;if(row.waitedSeconds>=policy.findings.leaseWaitSeconds&&(waited.get(key)?.waitedSeconds??-1)<row.waitedSeconds)waited.set(key,row);}
 for(const row of [...waited.values()].sort((left,right)=>right.waitedSeconds-left.waitedSeconds||left.pid-right.pid))say("lease-wait","warn",`pid ${row.pid}`,{pid:row.pid,waited:fleetDurationV1(language,row.waitedSeconds),resource:shortResource(row.resource)});
 if(storage){
  if(storage.hostFreeBytes!==null&&storage.hostFreeBytes<storage.hostFreeFloorBytes)say("disk-low","alert","host",{free:gib(storage.hostFreeBytes),floor:gib(storage.hostFreeFloorBytes)});
  if(storage.machine.overBytes>0)say("disk-over-budget","warn","machine",{over:gib(storage.machine.overBytes)});
  for(const row of images)if(row.usedBasisPoints!==null&&row.usedBasisPoints>=policy.findings.imageFullBasisPoints)say("image-full","warn",row.id,{id:row.id,percent:Math.floor(row.usedBasisPoints/100)});
 }
 for(const row of sealedEntries){
  if(row.blocked)say("sealed-blocked","alert",row.entry,{entry:row.entry,first:row.blocked.first??`${row.blocked.rows}`});
  else if(row.fresh===false)say("sealed-drifted","info",row.entry,{entry:row.entry,changed:row.changed});
 }
 for(const machine of facts.fleet?.machines??[]){
  const reasons=machine.cacheSkipReasons??{},denied=reasons["upload-expired-denied"]??0,readOnly=reasons["read-only"]??0;
  if(denied>0)say("unit-cache-denied","alert",machine.origin,{origin:machine.origin,count:denied});
  if(readOnly>0)say("unit-cache-read-only","warn",machine.origin,{origin:machine.origin,count:readOnly});
 }
 for(const diff of facts.fleet?.keys?.diffs??[])if(diff.sameSource)say("unit-key-machine-dependent","warn",diff.crate,{crate:diff.crate,origins:diff.origins.join(","),components:diff.differs.join(",")});
 for(const worker of facts.workers?.rows??[]){
  const seen=Math.min(worker.recordAgeSeconds??Infinity,worker.heartbeatAgeSeconds??Infinity),values={origin:worker.origin,age:fleetDurationV1(language,Number.isFinite(seen)?seen:0),recorded:worker.recorded??"-",stage:worker.stage??"-"};
  if(worker.liveness==="offline")say("worker-offline","warn",worker.origin,values);
  else if(worker.liveness==="silent")say("worker-silent","warn",worker.origin,{...values,age:fleetDurationV1(language,worker.recordAgeSeconds??0)});
  else if(worker.liveness==="absent"&&(worker.heartbeatAgeSeconds===null||worker.heartbeatAgeSeconds>=facts.workers!.offlineSeconds))say("worker-absent","info",worker.origin,{origin:worker.origin});
 }
 const wardens=new Map<string,FleetWorkerMemoryV1>(),staleSeconds=policy.findings.workerMemoryStaleSeconds,local=facts.memory?.local??null;
 for(const [origin,row] of Object.entries(facts.memory?.origins??{}))if(row!==null)wardens.set(origin,row);
 if(local!==null&&!wardens.has(local.origin))wardens.set(local.origin,local);
 for(const [origin,row] of [...wardens].sort((left,right)=>compare(left[0],right[0]))){
  if(row.absent||row.ageSeconds>staleSeconds)continue;
  const level=fleetTextV1(language,`workers.level.${row.level}`);
  if(row.level==="guard"||row.level==="danger")say("worker-memory-tight",row.level==="danger"?"alert":"warn",origin,{origin,level,available:row.availableMegabytes!,guard:row.guardMegabytes!,danger:row.dangerMegabytes!,swap:row.swapUsedMegabytes??"?",pressure:row.pressure!});
  const counters=row.counters!;
  if(counters.suspended+counters.cancelled>0)say("worker-warden-intervened",counters.cancelled>0?"alert":"warn",origin,{origin,suspended:counters.suspended,resumed:counters.resumed,cancelled:counters.cancelled,alarm:row.lastAlarm===null?"-":row.lastAlarm.text[language]});
  if(row.mode==="observe")say("worker-warden-observing","info",origin,{origin,level});
  if(row.swapUsedMegabytes!==null&&row.totalMegabytes!==null&&row.swapUsedMegabytes*10000>row.totalMegabytes*policy.findings.workerSwapWarnBasisPoints)say("worker-swap-high","warn",origin,{origin,swap:row.swapUsedMegabytes,percent:Math.floor(row.swapUsedMegabytes*100/row.totalMegabytes),total:row.totalMegabytes});
  if(row.overhead!==null&&row.overhead.cpuBasisPoints>policy.findings.wardenOverheadBasisPoints)say("worker-warden-overhead","warn",origin,{origin,percent:Math.floor(row.overhead.cpuBasisPoints/100),tick:Math.floor(row.overhead.tickMillisecondsP95+0.5),resident:Math.floor(row.overhead.residentMegabytes+0.5)});
 }
 if(facts.memory!==undefined)for(const worker of facts.workers?.rows??[]){
  if(worker.liveness!=="working"&&worker.liveness!=="ready"&&worker.liveness!=="idle")continue;
  const row=wardens.get(worker.origin),reason=row===undefined?fleetTextV1(language,"workers.warden.missing"):row.absent?fleetTextV1(language,"workers.warden.absent"):row.ageSeconds>staleSeconds?fleetTextV1(language,"workers.warden.stale",{age:fleetDurationV1(language,row.ageSeconds)}):null;
  if(reason!==null)say("worker-warden-absent","warn",worker.origin,{origin:worker.origin,reason});
 }
 for(const source of facts.sources)if(!source.ok)say("source-failed","warn",source.id,{section:source.id,reason:source.error??"?"});
 findings.sort((left,right)=>SEVERITY[left.severity]!-SEVERITY[right.severity]!||compare(left.id,right.id)||compare(left.subject,right.subject));
 const kept=findings.slice(0,policy.findings.maximum);
 const count=(state:string)=>runs.filter(row=>row.state===state).length;
 return Object.freeze({
  version:1,atMs,workspace:facts.workspace,
  summary:{running:count("running"),blocked:count("blocked"),waiting:count("waiting"),alerts:findings.filter(row=>row.severity==="alert").length,warnings:findings.filter(row=>row.severity==="warn").length},
  memory,runs:runs.slice(0,policy.runs.maximumRows),
  queues:{graph:buildGraph(facts,leaseWait),capacity:{floorMegabytes:capacity?.floorMegabytes??0,spareMegabytes:capacity?.headroomMegabytes??0,grants:grants.length,waiters:[...capacity?.waiters??[]].sort((left,right)=>left.position-right.position).map(row=>({id:row.id,pid:row.pid,operation:row.operation,needMegabytes:row.needMegabytes,waitedSeconds:row.waitedSeconds,reason:row.reason,reasonText:row.reasonText,position:row.position,estimatedWaitSeconds:capacityWait.get(row.id)??null}))},leases:leaseQueues,leasesOmitted:Math.max(0,queues.length-leaseQueues.length)},
  bazel:{slots:servers.length,live:servers.filter(row=>row.live).length,held:servers.filter(row=>row.held).length,rssMegabytes:servers.reduce((sum,row)=>sum+(row.treeMegabytes??0),0),servers},
  storage:{hostFreeBytes:storage?.hostFreeBytes??null,hostFreeFloorBytes:storage?.hostFreeFloorBytes??0,machine:storage?.machine??{totalBytes:0,effectiveTotalBytes:0,usedBytes:0,footprintBytes:0,overBytes:0,evictBytes:0,blockedBytes:0,source:"default"},overAreas:[...storage?.overAreas??[]].sort(compare),images},
  sealed:{mode:facts.sealed?.mode??"live",drifted:sealedEntries.filter(row=>row.fresh===false).length,blocked:sealedEntries.filter(row=>row.blocked!==undefined).length,entries:sealedEntries},
  generation,...facts.scheduler===undefined?{}:{scheduler:schedulerSection(facts.scheduler)},...facts.fleet===undefined?{}:{fleet:fleetSection(facts.fleet)},...facts.workers===undefined?{}:{workers:workersSection(facts.workers,facts.memory)},...facts.cas===undefined?{}:{cas:casSection(facts.cas,atMs)},...facts.fleet===undefined?{}:{retention:facts.fleet.retention===undefined?null:retentionSection(facts.fleet.retention,atMs)},...facts.offload===undefined?{}:{offload:offloadSection(facts.offload)},findings:kept,sources:facts.sources,
 }) as FleetReportV1;
}

/** ✅️ Checks a report against its own closed schema; the empty list means valid. */
export function validateFleetReportV1(report:unknown):readonly string[]{
 return validateJsonSchemaSubset(schema.definitions.Report as never,report,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}
/** 🔍️ Checks facts against their closed schema; the empty list means valid. */
export function validateFleetFactsV1(facts:unknown):readonly string[]{
 return validateJsonSchemaSubset(schema.definitions.Facts as never,facts,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

const clock=(atMs:number):string=>{const date=new Date(atMs),pad=(value:number)=>String(value).padStart(2,"0");return `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;};
/** 🏷️ The name a command is known by: the first word without its directory, nothing for a hash. */
const commandName=(command:string|undefined):string=>{const word=command?.split(/\s+/u)[0]?.split("/").pop()??"";return /^[0-9a-f]{16,}$/u.test(word)||word===""?"-":word.slice(0,policy.runs.whatCharacters);};
const counter=(row:Row):string=>row.done===undefined?"":row.total===undefined?` ${row.done}`:` ${row.done}/${row.total}`;

/** 🖨️ Renders the report as compact text lines in the chosen language; `limit` bounds the run rows shown. */
export function formatFleetReportV1(language:FleetLanguageV1,report:FleetReportV1,limit:number=20):readonly string[]{
 const say=(key:string,values:Readonly<Record<string,string|number>>={})=>fleetTextV1(language,key,values),duration=(seconds:number)=>fleetDurationV1(language,seconds),lines:string[]=[],memory=report.memory,summary=report.summary;
 const estimate=(seconds:number|null|undefined):string=>seconds===null||seconds===undefined?"":say("estimate",{estimate:duration(seconds)});
 lines.push(say("header",{time:clock(report.atMs),running:summary.running,blocked:summary.blocked,waiting:summary.waiting,alerts:summary.alerts,warnings:summary.warnings}));
 lines.push(memory.sampleAgeSeconds===null?say("memoryNone",{admitted:memory.admittedMegabytes,observed:memory.observedMegabytes,grants:memory.grants}):say("memory",{age:duration(memory.sampleAgeSeconds),admitted:memory.admittedMegabytes,observed:memory.observedMegabytes,grants:memory.grants,available:memory.availableMegabytes,total:memory.totalMegabytes,spare:memory.spareMegabytes,floor:memory.floorMegabytes,pressure:memory.pressure,swap:memory.swapUsedMegabytes??"n/a",load:memory.load??"n/a"}));
 const warden=memory.warden as FleetWorkerMemoryV1|null|undefined;
 if(warden!==undefined&&warden!==null&&!warden.absent)lines.push(say("memory.warden",{origin:warden.origin,level:say(`workers.level.${warden.level}`),mode:say(`workers.mode.${warden.mode}`),available:warden.availableMegabytes!,swap:warden.swapUsedMegabytes??"?",swapOut:warden.swapOutMegabytesPerSecond??"?",compressed:warden.compressedMegabytes??"?",pressure:warden.pressure!,suspended:warden.counters!.suspended,resumed:warden.counters!.resumed,cancelled:warden.counters!.cancelled}));
 if(report.runs.length===0)lines.push(say("runsNone"));
 else{
  lines.push(say("runsHeader",{count:report.runs.length,shown:Math.min(limit,report.runs.length)}));
  for(const row of report.runs.slice(0,limit)){
   const run=row as Row,waitingFor=run.waitingFor,eta=run.etaSeconds!==undefined?say("eta",{eta:duration(run.etaSeconds)}):run.etaOverdue?say("etaOverdue"):"";
   lines.push(say("run",{pid:run.pid,state:say(`state.${run.state}`),who:(run.agent??run.session?.slice(run.session.indexOf(".")+1)??"-").slice(0,policy.runs.whoCharacters),what:run.operation??(commandName(run.command)!=="-"?commandName(run.command):run.exclusive?.[0]?.split(":")[0]?.slice(0,policy.runs.whatCharacters)??"-"),progress:run.phase===undefined?"":say("progress",{phase:run.phase,counter:counter(run)}),eta,age:duration(run.ageSeconds),priority:say(`priority.${run.priority??"unknown"}`),code:say(`code.${run.implementation?.freshness??"unknown"}`),memory:run.memory.admittedMegabytes===null?say("memoryPairNone"):say("memoryPair",{admitted:run.memory.admittedMegabytes,observed:run.memory.observedMegabytes??"?"}),waiting:waitingFor===undefined?"":say(`waiting.${waitingFor.kind}`,{waited:duration(waitingFor.waitedSeconds),subject:waitingFor.subject,estimate:estimate(waitingFor.estimatedWaitSeconds)})}));
  }
 }
 lines.push(say("queuesHeader"));
 const graph=report.queues.graph,capacity=report.queues.capacity,longest=graph.longestWaitSeconds>0?say("queue.longest",{waited:duration(graph.longestWaitSeconds)}):"";
 if(graph.builder)lines.push(say("queue.graph",{pid:graph.builder.pid,phase:graph.builder.phase??"-",counter:counter(graph.builder),held:duration(graph.builder.heldSeconds),eta:graph.builder.etaSeconds===null?"":say("eta",{eta:duration(graph.builder.etaSeconds)}),computers:graph.computers,slots:graph.slots,waiting:graph.waiting,longest,estimate:estimate(graph.estimatedWaitSeconds)}));
 else lines.push(say("queue.graphIdle",{computers:graph.computers,waiting:graph.waiting,longest}));
 lines.push(say("queue.capacity",{grants:capacity.grants,waiters:capacity.waiters.length,spare:capacity.spareMegabytes,floor:capacity.floorMegabytes}));
 for(const row of capacity.waiters.slice(0,policy.queues.maximumWaitersShown))lines.push(say("queue.capacityWaiter",{position:row.position,pid:row.pid,operation:row.operation,need:row.needMegabytes,waited:duration(row.waitedSeconds),reason:row.reasonText,estimate:estimate(row.estimatedWaitSeconds)}));
 for(const queue of report.queues.leases){
  lines.push(say("queue.lease",{resource:queue.resource,class:queue.class,holders:queue.holders.length===0?say("queue.leaseNoHolder"):queue.holders.map((holder:Row)=>say("queue.leaseHolder",{pid:holder.pid,mode:holder.mode,held:duration(holder.heldSeconds),eta:holder.etaSeconds===null?"":say("eta",{eta:duration(holder.etaSeconds)})})).join("; ")}));
  for(const waiter of queue.waiters.slice(0,policy.queues.maximumWaitersShown))lines.push(say("queue.leaseWaiter",{position:waiter.position??"?",pid:waiter.pid,mode:waiter.mode,waited:duration(waiter.waitedSeconds),estimate:estimate(waiter.estimatedWaitSeconds)}));
 }
 if(report.queues.leasesOmitted>0)lines.push(say("queue.leasesOmitted",{count:report.queues.leasesOmitted}));
 const scheduler=report.scheduler;
 if(scheduler!==undefined){
  lines.push(say("scheduler",{running:scheduler.running,queued:scheduler.queued,reserved:scheduler.reservedMegabytes,budget:scheduler.budgetMegabytes,shared:scheduler.sharedTickets,longest:scheduler.longestWaitSeconds>0?say("scheduler.longest",{waited:duration(scheduler.longestWaitSeconds)}):""}));
  const shown=policy.queues.maximumWaitersShown;
  for(const job of(scheduler.jobs as Row[]).filter(row=>row.position===undefined).slice(0,shown))lines.push(say("scheduler.run",{key:job.key,agents:job.agents.join(","),reserve:job.reserveMegabytes,run:duration(job.runSeconds??0),progress:job.progress?say("scheduler.progress",job.progress):"",stopping:job.stopping?say("scheduler.stopping"):""}));
  for(const job of(scheduler.jobs as Row[]).filter(row=>row.position!==undefined).slice(0,shown))lines.push(say("scheduler.queued",{position:job.position,key:job.key,agents:job.agents.join(","),reserve:job.reserveMegabytes,priority:job.priority,depth:job.depth,waited:duration(job.waitedSeconds)}));
  for(const agent of(scheduler.agents as Row[]).slice(0,shown))lines.push(say("scheduler.agent",agent));
 }
 const fleet=report.fleet;
 if(fleet!==undefined){
  lines.push(say("fleet",{machines:fleet.machines.length,queued:fleet.queued,executing:fleet.executing,offloadedIn:fleet.offloadedIn,offloadedOut:fleet.offloadedOut,refused:fleet.refused,declined:fleet.declined,retried:fleet.retried}));
  for(const machine of fleet.machines as Row[])lines.push(say("fleet.machine",{origin:machine.origin,platform:machine.platform,pressure:machine.pressure,load:machine.load===null?"?":machine.load,cores:machine.cores,running:machine.running,queued:machine.queued,worker:machine.worker,headroom:machine.headroom?"":say("fleet.noheadroom"),offloadedIn:machine.offloadedIn,offloadedOut:machine.offloadedOut,refused:machine.refused,cacheHit:machine.cacheHit,cacheMiss:machine.cacheMiss,cacheSkipped:machine.cacheSkipped??0,age:duration(machine.ageSeconds)}));
  for(const machine of fleet.machines as Row[]){const rows=Object.entries((machine.cacheSkipReasons??{}) as Record<string,number>).sort((left,right)=>right[1]-left[1]||compare(left[0],right[0]));if(rows.length>0)lines.push(say("fleet.skips",{origin:machine.origin,reasons:rows.map(([name,count])=>`${name} ${count}`).join(", ")}));}
  if(fleet.keys!==undefined){
   const diffs=fleet.keys.diffs as Row[],dependent=diffs.filter(row=>row.sameSource);
   lines.push(say("fleet.keys",{origins:fleet.keys.origins,units:fleet.keys.units,differing:diffs.length,dependent:dependent.length,components:Object.entries(fleet.keys.byComponent as Record<string,number>).sort((left,right)=>right[1]-left[1]||compare(left[0],right[0])).map(([name,count])=>`${name} ${count}`).join(", ")}));
   for(const row of diffs.slice(0,policy.findings.maximumKeyDiffsShown))lines.push(say("fleet.keydiff",{crate:row.crate,origins:row.origins.join(","),components:row.differs.join(","),source:row.sameSource?say("fleet.keysame"):say("fleet.keyedit")}));
   for(const row of diffs.slice(0,policy.findings.maximumKeyDiffsShown))for(const [component,texts] of Object.entries((row.values??{}) as Record<string,Record<string,string>>))lines.push(say("fleet.keyvalue",{component,texts:Object.entries(texts).map(([origin,text])=>`${origin} = ${text}`).join(" | ")}));
  }
  for(const machine of fleet.machines as Row[])if(machine.why!==undefined)lines.push(say("fleet.why",{origin:machine.origin,worker:machine.worker,reason:machine.why[language]}));
  if(fleet.machines.length>1)for(const machine of fleet.machines as Row[]){
   if(/-none$/u.test(machine.platform))lines.push(say("fleet.norustc",{origin:machine.origin,platform:machine.platform}));
   else if(!(fleet.machines as Row[]).some(other=>other.origin!==machine.origin&&other.platform===machine.platform))lines.push(say("fleet.apart",{origin:machine.origin,others:[...new Set((fleet.machines as Row[]).filter(other=>other.origin!==machine.origin).map(other=>other.platform))].sort().join(", ")}));
  }
  if(fleet.cache!==undefined)lines.push(say("fleet.cache",fleet.cache));
  if(fleet.claims!==undefined)lines.push(say("fleet.claims",{active:fleet.claims.active,waiting:fleet.claims.waiting,oldest:duration(fleet.claims.oldestSeconds)}));
 }
 const workers=report.workers;
 if(workers!==undefined){
  const counts=workers.counts as Record<string,number>;
  lines.push(say("workers",counts));
  const wardenText=(row:Row):string=>{const warden=row.memory as FleetWorkerMemoryV1|null;return warden===null?say("workers.memory.none"):warden.absent?say("workers.memory.nowarden"):warden.ageSeconds>policy.findings.workerMemoryStaleSeconds?say("workers.warden.stale",{age:duration(warden.ageSeconds)}):say("workers.memory",{available:warden.availableMegabytes!,swap:warden.swapUsedMegabytes??"?",compressed:warden.compressedMegabytes??"?",level:say(`workers.level.${warden.level}`),mode:say(`workers.mode.${warden.mode}`),suspended:warden.counters!.suspended,resumed:warden.counters!.resumed,cancelled:warden.counters!.cancelled});};
  for(const row of workers.rows as Row[])lines.push(say("workers.row",{memory:wardenText(row),origin:row.origin,platform:row.platform??"-",state:say(`workers.state.${row.liveness}`),stage:row.stage??"-",index:row.stageIndex,count:row.stageCount,attempts:row.attempts,record:row.recordAgeSeconds===null?"-":duration(row.recordAgeSeconds),machine:row.heartbeatAgeSeconds===null?"-":duration(row.heartbeatAgeSeconds),refusal:row.refusal===null?"":say("workers.refusal",{reason:row.refusal[language]})}));
 }
 const cas=report.cas;
 if(cas!==undefined){
  lines.push(say("cas",{used:cas.usedBytes===null?"n/a":gib(cas.usedBytes),effective:gib(cas.effectiveCapBytes),percent:cas.usedBasisPoints===null?"?":Math.floor(cas.usedBasisPoints/100),cap:gib(cas.capBytes),high:gib(cas.highWatermarkBytes),low:gib(cas.lowWatermarkBytes)}));
  lines.push(say("cas.leases",{leased:cas.leases.leased,leasedGib:cas.leases.leasedBytes===null?"?":gib(cas.leases.leasedBytes),expired:cas.leases.expired,expiredGib:cas.leases.expiredBytes===null?"?":gib(cas.leases.expiredBytes),collection:cas.lastCollection===null?say("cas.neverCollected"):say("cas.collected",{age:duration(cas.lastCollection.ageSeconds),gib:gib(cas.lastCollection.bytes)})}));
 }
 const retention=report.retention;
 if(retention===null)lines.push(say("retention.none"));
 else if(retention!==undefined){
  lines.push(say("retention",{mode:retention.mode,swept:retention.ageSeconds===null?say("retention.neverSwept"):say("retention.swept",{age:duration(retention.ageSeconds)}),entries:retention.entries,judged:retention.judged,stale:retention.stale,unreached:retention.unreached,undeclared:retention.undeclared,retired:retention.retired}));
  const reasons=(counts:Record<string,number>)=>Object.entries(counts).sort((left,right)=>right[1]-left[1]||compare(left[0],right[0])).map(([name,count])=>`${name} ${count}`).join(", ");
  if(retention.stale>0)lines.push(say("retention.stale",{reasons:reasons(retention.staleByReason)}));
  if(Object.keys(retention.refused).length>0)lines.push(say("retention.refused",{reasons:reasons(retention.refused)}));
 }
 const offload=report.offload;
 if(offload!==undefined){
  lines.push(say("offload",{local:offload.local,fleet:offload.fleet,out:offload.out,refused:offload.refused,fellBack:offload.fellBack,deadline:offload.deadline,lost:offload.lost,silent:offload.silent}));
  for(const build of(offload.builds as Row[]).slice(0,policy.queues.maximumWaitersShown))lines.push(say("offload.build",build));
 }
 const bazel=report.bazel;
 lines.push(say("bazel",{live:bazel.live,rss:bazel.rssMegabytes,held:bazel.held,slots:bazel.slots}));
 for(const server of bazel.servers.filter((row:Row)=>row.live))lines.push(say("bazelServer",{slot:server.slot,pid:server.pid,rss:server.treeMegabytes??"?",state:server.held?say("bazel.held",{holder:server.holderPid}):say("bazel.idle")}));
 const storage=report.storage;
 lines.push(say("storage",{free:storage.hostFreeBytes===null?"n/a":gib(storage.hostFreeBytes),floor:gib(storage.hostFreeFloorBytes),used:gib(storage.machine.usedBytes),effective:gib(storage.machine.effectiveTotalBytes),over:gib(storage.machine.overBytes),evict:gib(storage.machine.evictBytes),source:storage.machine.source}));
 for(const image of storage.images.filter((row:Row)=>row.usedBasisPoints!==null&&row.usedBasisPoints>=policy.findings.imageShownBasisPoints))lines.push(say("storageImage",{id:image.id,used:gib(image.usedBytes),capacity:gib(image.capacityBytes),percent:Math.floor(image.usedBasisPoints/100)}));
 const sealed=report.sealed;
 lines.push(say("sealed",{mode:sealed.mode,count:sealed.entries.length,drifted:sealed.drifted,blocked:sealed.blocked}));
 for(const entry of sealed.entries)lines.push(say("sealedEntry",{entry:entry.entry,digest:entry.digest,age:duration(entry.ageSeconds),files:entry.files,freshness:say(`fresh.${entry.fresh}`,{changed:entry.changed})}));
 const generation=report.generation;
 if(generation===null)lines.push(say("generationNone"));
 else lines.push(say("generation",{number:generation.number,kind:generation.built.kind,age:generation.ageSeconds===null?"?":duration(generation.ageSeconds),dirty:generation.built.dirty,covered:generation.verified?.covered??0,count:generation.verified?.count??generation.owners,percent:Math.floor((generation.verified?.basisPoints??0)/100),verified:generation.verified===null?"?":duration(generation.verified.ageSeconds)}));
 if(report.findings.length>0){lines.push(say("findingsHeader"));for(const finding of report.findings)lines.push(say("finding",{severity:say(`severity.${finding.severity}`),text:finding.text}));}
 const failed=report.sources.filter(row=>!row.ok);
 lines.push(failed.length===0?say("sources",{count:report.sources.length,milliseconds:report.sources.reduce((sum,row)=>sum+row.milliseconds,0)}):say("sourcesFailed",{failed:failed.length,count:report.sources.length,list:failed.map(row=>row.id).join(", ")}));
 return lines;
}
