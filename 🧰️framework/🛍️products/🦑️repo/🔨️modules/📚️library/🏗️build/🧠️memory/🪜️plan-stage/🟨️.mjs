import{GRAPH_MEMORY_POLICY_V1,createMemoryGovernorV1,createMemoryPortsV1,hostMemoryMegabytesV1,selectGraphMemoryLanguagesV1}from"../🟨️.mjs";
import{graphCutConcurrencyV1}from"../✂️cut/🟨️.mjs";
import{availableParallelism}from"node:os";
import{LEAN_SLICE_POLICY_V1,leanSlicePlanV1}from"../🪶️lean/🟨️.mjs";
import{FOOTPRINT_POLICY_V1}from"../🦶️footprint/🟨️.mjs";
import{ownerTooHeavyOfV1,runInOwnerScopeV1,throwIfOwnerExceededV1}from"../🎯️owner-scope/🟨️.mjs";
import{createRequire}from"node:module";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const nativeRequire=createRequire(import.meta.url);

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 🪜️ The declared limits and stages of a graph plan: the schema file is the single source. */
export const PLAN_STAGE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));

/** 🗣️ The bilingual lines of the plan stages, read from the schema file. */
export const PLAN_STAGE_MESSAGES_V1=freeze(structuredClone(schema.definitions.Messages.const));
const P=PLAN_STAGE_POLICY_V1,MEGABYTE=1048576,megabytes=bytes=>Math.round(bytes/MEGABYTE);

/** 🌐️ The listeners of the stage events of this process (a slice worker forwards them to its service). */
const sinks=new Set();

/** 🛑️ The listeners of the stops of the plan guard of this process. */
const stopSinks=new Set();

/** 🛑️ Registers a listener that is told when the plan guard of this process stops a plan (`{stage,operation,usedMegabytes,budgetMegabytes}`), at the moment of the stop and before the heap detail is written; returns the function that removes it. */
export function onPlanStopV1(listener){stopSinks.add(listener);return()=>stopSinks.delete(listener);}

/** 📡️ Registers a listener of the stage events (`StageEvent` of the schema) of every plan of this process; returns the function that removes it. */
export function onPlanStageV1(listener){sinks.add(listener);return()=>sinks.delete(listener);}

/** 💬️ Renders one plan stage line as one line per language of the environment. */
export function renderPlanStageMessageV1(key,fields,environment){
 return selectGraphMemoryLanguagesV1(environment).map(language=>{
  const template=PLAN_STAGE_MESSAGES_V1[language][key];
  if(template===undefined)throw Error("Closed plan stage message required: "+key);
  return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
 });
}

/** 📏️ The managed bytes a plan may add to what its process holds when it starts: `hostShare` of the physical memory between the declared minimum and maximum, none on a machine that serves the fleet (it holds the whole graph and its plans are measured, not limited), none when the environment says `off`, and the megabytes the environment names when it names a number. */
export function planStageLimitBytesV1(environment=process.env,hostMegabytes=hostMemoryMegabytesV1(),policy=PLAN_STAGE_POLICY_V1){
 const text=environment[policy.limitEnvironment];
 if(text===policy.off)return Infinity;
 const named=Number(text);
 if(text!==undefined&&text!==""&&Number.isFinite(named)&&named>0)return Math.floor(named)*MEGABYTE;
 if(!(hostMegabytes>0)||hostMegabytes>=policy.serveHostMegabytes)return Infinity;
 const lean=Math.min(Math.floor(hostMegabytes*LEAN_SLICE_POLICY_V1.ceilingBasisPoints/10000),LEAN_SLICE_POLICY_V1.parentMegabytes+LEAN_SLICE_POLICY_V1.childMegabytes);
 return Math.min(policy.maximumMegabytes,Math.max(policy.minimumMegabytes,lean,Math.floor(hostMegabytes*policy.hostShare)))*MEGABYTE;
}

const LEAN_CEILING_BASIS_POINTS_V1=LEAN_SLICE_POLICY_V1.ceilingBasisPoints;
const farms=new Map();
const priorityOf=kind=>P.farmPriority[kind]??0;
const reservedMegabytes=kind=>{let total=0;for(const row of farms.values())if(priorityOf(row.kind)>=priorityOf(kind))total+=row.megabytes;return total;};

/** 🏋️ The footprint a farm child may reach while it runs alone on a host of `hostMegabytes`: the declared `aloneCapMegabytes`, lowered to what the plan ceiling of the slice guard (`ceilingBasisPoints` of the lean plan) leaves beside the parent footprint, so a heavy owner on a small host gets the whole plan the guard allows and is stopped by this cap before the guard would stop the whole tree; the declared number on every host that has room for it.
 * @see ./🧬️schema/🔣️.json */
export function farmAloneCapV1(hostMegabytes,policy=PLAN_STAGE_POLICY_V1){
 const declared=policy.farmChildFootprintMegabytes["project-farm"].aloneCapMegabytes;
 if(!(hostMegabytes>0))return declared;
 return Math.min(declared,Math.max(policy.farmChildFootprintMegabytes["project-farm"].capMegabytes,Math.floor(hostMegabytes*LEAN_SLICE_POLICY_V1.ceilingBasisPoints/10000)-policy.parentFootprintMegabytes));
}

/** 🪶️ The lean plan of the slice read on this machine (`../🪶️lean`): the plan limit of the machine, the ceiling the slice guard refuses at, the cores and what the parent holds already decide whether the read is lean and how many inference children the tree may hold. */
export function leanSliceV1({environment=process.env,hostMegabytes=hostMemoryMegabytesV1(),cores=availableParallelism(),residentMegabytes=0,policy=PLAN_STAGE_POLICY_V1}={}){
 const limit=planStageLimitBytesV1(environment,hostMegabytes,policy)/MEGABYTE;
 return leanSlicePlanV1({limitMegabytes:limit,ceilingMegabytes:hostMegabytes>0?hostMegabytes*LEAN_CEILING_BASIS_POINTS_V1/10000:Infinity,cores,residentMegabytes,environment});
}

/** 📐️ How many children of a farm whose footprint is bounded by `child` (`typicalMegabytes`, `capMegabytes`) fit in the hard share of the tree: the parent at its declared footprint, then one child at its cap beside the others at their typical size; 0 where not even one fits. @see ./🧬️schema/🔣️.json */
export function farmWidthV1({hostMegabytes,reservedMegabytes=0,policy=PLAN_STAGE_POLICY_V1,child,hardPercent=FOOTPRINT_POLICY_V1.hardPercent}){
 const free=hostMegabytes*policy.treeShare*hardPercent/100-policy.parentFootprintMegabytes-reservedMegabytes;
 return free>=child.capMegabytes?1+Math.floor((free-child.capMegabytes)/child.typicalMegabytes):0;
}

/** 🎟️ The physical memory a farm is planned against: the host's, lowered to the memory whose `treeShare` equals the rebuild reservation the gate granted (`grantEnvironment`, in megabytes), so a rebuild admitted with less than it asked for runs a narrower farm instead of the full one inside a smaller booking; the host's own when no grant names one. */
export function grantedHostMegabytesV1(hostMegabytes,environment=process.env,policy=PLAN_STAGE_POLICY_V1){
 const granted=Number(environment[policy.grantEnvironment]);
 return Number.isFinite(granted)&&granted>0&&hostMegabytes>0?Math.min(hostMegabytes,Math.floor(granted/policy.treeShare)):hostMegabytes;
}

/** 🌊️ The physical memory a farm is planned against when the machine has more free memory than the rebuild reservation: the granted memory of {@link grantedHostMegabytesV1} plus the free memory above the floor (`availableFloorMegabytes`) turned into the physical memory whose `treeShare` it is, never above the host. The children of the farm are admitted one by one by the host gate against the real free memory, so a shrunk reservation of the parent no longer narrows the farm to nothing while the machine is idle; without a reading of the free memory the granted memory stands.
 * @see ./🧬️schema/🔣️.json */
export function farmHostMegabytesV1(hostMegabytes,environment=process.env,availableMegabytes=undefined,policy=PLAN_STAGE_POLICY_V1){
 const granted=grantedHostMegabytesV1(hostMegabytes,environment,policy);
 if(granted>=hostMegabytes||!Number.isFinite(availableMegabytes))return granted;
 return Math.min(hostMegabytes,granted+Math.floor(Math.max(0,availableMegabytes-policy.availableFloorMegabytes)/policy.treeShare));
}

/** 🏊️ How many worker processes of `workerMegabytes` a plan that holds `residentMegabytes` already (its own resident set by default) may start so that its whole process tree stays within `treeShare` of the physical memory (the slice guard judges the tree, not the managed bytes of the plan); Infinity where the plan has no limit (a serving machine, `off`), or the cap of the cut of the load (`SEMIO_GRAPH_CUT_CONCURRENCY`, see `../✂️cut`) where the work was cut smaller. A lean read (`../🪶️lean`) plans with the parent it is designed to keep (`parentMegabytes` of the lean policy, never less than what it holds) and the budget of its child, and always grants the first project child when one fits beside the parent within the ceiling. @see ./🧬️schema/🔣️.json */
export function planFarmWorkersV1({workerMegabytes,residentMegabytes=process.memoryUsage().rss/MEGABYTE,environment=process.env,hostMegabytes:physicalMegabytes=hostMemoryMegabytesV1(),policy=PLAN_STAGE_POLICY_V1,kind,counted=kind,availableMegabytes}){
 const hostMegabytes=farmHostMegabytesV1(physicalMegabytes,environment,availableMegabytes,policy);
 const cut=graphCutConcurrencyV1(environment);
 if(hostMegabytes===physicalMegabytes&&planStageLimitBytesV1(environment,hostMegabytes,policy)===Infinity)return cut;
 const child=kind===undefined?undefined:policy.farmChildFootprintMegabytes?.[kind];
 if(child!==undefined)return Math.min(cut,farmWidthV1({hostMegabytes,reservedMegabytes:counted===undefined?0:reservedMegabytes(counted),policy,child}));
 const lean=kind===undefined?undefined:leanSliceV1({environment,hostMegabytes,residentMegabytes,policy}),leaning=lean?.lean===true,parent=kind===undefined?residentMegabytes:Math.max(residentMegabytes,leaning?lean.parentMegabytes:policy.parentMegabytes),cost=kind===undefined?workerMegabytes:leaning&&kind==="project-farm"?lean.childMegabytes:Math.max(workerMegabytes,policy.farmWorkerPeakMegabytes[kind]??0);
 const room=Math.max(0,Math.floor((hostMegabytes*policy.treeShare-parent-(counted===undefined?0:reservedMegabytes(counted)))/cost));
 return Math.min(cut,room);
}

/**
 * 🧮️ The one memory plan of the process tree: every kind of child process a plan starts (`project-farm`, `walk-farm`) takes its workers from the same `treeShare` of the physical memory, after what the process holds itself and after what the farms of the same or a higher priority (`farmPriority`) already reserved, so the sum of all farms and the parent stays within the share that the slice guard judges. A farm that does not fit asks the lower priority farms to give their children back (`yield`) before it takes less. Returns the workers granted and `release()` (idempotent), which the farm calls when its children are gone; nothing is recorded where the plan has no limit.
 * @see ./🧬️schema/🔣️.json
 */
export function reserveFarmV1({kind,workerMegabytes,wanted=Infinity,residentMegabytes,environment=process.env,hostMegabytes,policy=PLAN_STAGE_POLICY_V1,yield:onYield}){
 const lean=leanSliceV1({environment,policy,residentMegabytes:residentMegabytes??process.memoryUsage().rss/MEGABYTE,...hostMegabytes===undefined?{}:{hostMegabytes}}),cost=policy.farmChildFootprintMegabytes?.[kind]?.typicalMegabytes??(lean.lean&&kind==="project-farm"?lean.childMegabytes:Math.max(workerMegabytes,policy.farmWorkerPeakMegabytes[kind]??0)),roomOf=counted=>planFarmWorkersV1({workerMegabytes,environment,policy,kind,counted,...residentMegabytes===undefined?{}:{residentMegabytes},...hostMegabytes===undefined?{}:{hostMegabytes}});
 let workers=Math.min(wanted,roomOf(kind));
 const giveBack=()=>{for(const[token,row]of[...farms])if(priorityOf(row.kind)<priorityOf(kind)){farms.delete(token);try{row.yield?.();}catch{}}};
 if(Math.min(wanted,roomOf("*"))<workers)giveBack();
 if(lean.lean&&kind==="project-farm"&&lean.fits&&wanted>0&&workers<1){giveBack();workers=Math.min(wanted,Math.max(1,roomOf(kind)),graphCutConcurrencyV1(environment));}
 const token=Symbol(kind);
 if(workers>0&&planStageLimitBytesV1(environment,hostMegabytes??hostMemoryMegabytesV1(),policy)!==Infinity)farms.set(token,{kind,megabytes:workers*cost,yield:onYield});
 return Object.freeze({workers,release(){farms.delete(token);}});
}

/** 📋️ What the farms of this process hold in the plan right now: one row per reservation. */
export const farmReservationsV1=()=>[...farms.values()].map(row=>({kind:row.kind,megabytes:row.megabytes}));

/**
 * 🪜️ Guards one plan (the assembly of a wave after its projects are inferred) as ONE owner of the memory governor, the same mechanism that stops one project: `run(work)` runs `work` inside the owner scope `plan`, `mark(stage)` names the stage that starts (and ends the one before it with its seconds and growth), `operation(name)` names the operation inside a stage and is a throw point for long operations, and a plan whose managed bytes grew past `limitBytes` (or a stage past its own budget `stageMegabytes`), confirmed by a full collection (garbage is no growth), is stopped at its next yield point, operation or stage boundary, `refuse({stage,operation,megabytes,limit})` building the typed error the caller throws. A sampler every `sampleMilliseconds` keeps the last samples (heap, external, array buffers, resident) and enforces the limits between yield points; when it stops a plan it writes ONE detail line measured at that moment (counters, fresh heap statistics, pending native requests, the recent samples), so the log says whether the growth is heap or buffers and when it began. Every stage start and end is one bilingual line to `report` (on a machine that has a limit, or with `SEMIO_GRAPH_PHASES=1`; a serving machine that has none stays quiet), so a plan that dies inside a stage names that stage in the log; `run` ends the last stage when the plan ends, whatever the outcome.
 * @param {{limitBytes?:number;ports?:{sample:()=>number;collect:()=>void};environment?:Record<string,string|undefined>;report?:(line:string)=>void;now?:()=>number;resident?:()=>number;usage?:()=>Record<string,number>;heap?:()=>Record<string,unknown>|undefined;resources?:()=>readonly string[]|undefined;stageMegabytes?:Record<string,number>;refuse?:(detail:{stage:string;operation:string;megabytes:number;limit:number})=>Error}} [options]
 */
export function createPlanStageGuardV1({limitBytes=planStageLimitBytesV1(),ports=createMemoryPortsV1(),environment=process.env,report=line=>{try{process.stderr.write(line+"\n");}catch{}},now=()=>performance.now(),resident=()=>process.memoryUsage().rss,usage=()=>process.memoryUsage(),heap=()=>{try{return nativeRequire("bun:jsc").heapStats();}catch{return undefined;}},resources=()=>process.getActiveResourcesInfo?.(),stageMegabytes=P.stageMegabytes,refuse}={}){
 let stage,operationName="-",startedAt=0,startedBytes=0,baseline=0,current,stoppedIn,stoppedDuring,told=false,ring=[];
 const say=(key,fields)=>{for(const line of renderPlanStageMessageV1(key,fields,environment))report(line);},tell=limitBytes<Infinity||environment.SEMIO_GRAPH_PHASES==="1",narrate=(key,fields)=>{if(tell)say(key,fields);};
 const emit=()=>{const event={type:"stage",stage,operation:operationName,managedMegabytes:megabytes(ports.sample()),residentMegabytes:megabytes(resident())};for(const sink of sinks)try{sink(event);}catch{}return event;};
 const close=()=>{
  if(stage===undefined)return;
  const bytes=ports.sample();
  narrate("finished",{stage,seconds:Math.round((now()-startedAt)/100)/10,managed:megabytes(bytes),delta:megabytes(bytes-startedBytes),resident:megabytes(resident())});
  stage=undefined;
 };
 const detail=()=>{
  try{
   const counters=usage(),statistics=heap(),pending={};
   for(const name of resources()??[])pending[name]=(pending[name]??0)+1;
   const row={stage:stoppedIn,operation:stoppedDuring,memory:Object.fromEntries(["rss","heapTotal","heapUsed","external","arrayBuffers"].map(name=>[name,megabytes(counters[name]??0)])),...statistics?{heap:{heapMb:Math.round(statistics.heapSize/MEGABYTE*10)/10,extraMb:Math.round(statistics.extraMemorySize/MEGABYTE*10)/10,objectCount:statistics.objectCount,objectTypes:Object.entries(statistics.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,8).map(([type,count])=>({type,count}))}}:{},resources:pending,recent:ring};
   for(const line of renderPlanStageMessageV1("detail",{json:JSON.stringify(row)},environment))report(line);
  }catch{}
 };
 const stopped=(grown,limit)=>{
  stoppedIn??=stage??"?";stoppedDuring??=operationName;
  if(told)return;told=true;
  say("stopped",{stage:stoppedIn,operation:stoppedDuring,used:megabytes(grown),budget:megabytes(limit)});
  for(const sink of stopSinks)try{sink({stage:stoppedIn,operation:stoppedDuring,usedMegabytes:megabytes(grown),budgetMegabytes:megabytes(limit)});}catch{}
  detail();
 };
 const over=()=>{
  const bytes=ports.sample(),plan=bytes-baseline,own=stage===undefined||stageMegabytes[stage]===undefined?undefined:stageMegabytes[stage]*MEGABYTE;
  if(plan>limitBytes)return[plan,limitBytes];
  if(own!==undefined&&bytes-startedBytes>own)return[bytes-startedBytes,own];
  return undefined;
 };
 const check=()=>{
  if(current===undefined||current.exceeded!==undefined||!(limitBytes<Infinity))return;
  if(over()===undefined)return;
  ports.collect();
  const hit=over();
  if(hit===undefined)return;
  current.exceed({takenBytes:hit[0],limitBytes:hit[1]});stopped(hit[0],hit[1]);
 };
 const sample=()=>{
  const counters=usage();
  ring.push({atMs:Math.round(now()),heapUsed:megabytes(counters.heapUsed??0),external:megabytes(counters.external??0),arrayBuffers:megabytes(counters.arrayBuffers??0),rss:megabytes(counters.rss??0),operation:stage===undefined?"-":stage+"/"+operationName});
  if(ring.length>P.recentSamples)ring.shift();
  check();
 };
 return Object.freeze({
  mark(name){
   throwIfOwnerExceededV1();check();throwIfOwnerExceededV1();close();
   stage=name;operationName=name;startedAt=now();startedBytes=ports.sample();
   const event=emit();
   narrate("started",{stage:name,managed:event.managedMegabytes,resident:event.residentMegabytes});
  },
  operation(name){
   throwIfOwnerExceededV1();check();throwIfOwnerExceededV1();
   if(name!==operationName){operationName=name;emit();}
  },
  async run(work){
   baseline=ports.sample();told=false;stoppedIn=undefined;stoppedDuring=undefined;ring=[];
   const governor=createMemoryGovernorV1({workers:1,headroom:()=>Number.MAX_SAFE_INTEGER,measure:ports.sample,collect:ports.collect,ownerLimitBytes:limitBytes,onStop:({takenBytes,limitBytes:limit})=>stopped(takenBytes,limit),report:()=>undefined,environment}),timer=setInterval(sample,P.sampleMilliseconds);
   timer.unref?.();
   try{
    return await governor.run(({scope})=>{current=scope;return runInOwnerScopeV1(scope,work);},{kind:P.owner,key:P.owner});
   }catch(error){
    const found=ownerTooHeavyOfV1(error);
    if(found===undefined||found.name!==P.owner||refuse===undefined)throw error;
    throw refuse({stage:stoppedIn??stage??"?",operation:stoppedDuring??operationName,megabytes:megabytes(found.takenBytes),limit:megabytes(found.limitBytes)});
   }finally{clearInterval(timer);close();governor.dispose();current=undefined;}
  },
 });
}
