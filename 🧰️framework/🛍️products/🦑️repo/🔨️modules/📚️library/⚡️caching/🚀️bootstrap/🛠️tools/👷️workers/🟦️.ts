import {WORKER_ACTION_KINDS_V1,type WorkerMappingV1,type WorkerPlanV1} from "../../../../📦️packages/🟦️typescript/🏗️build/👷️workers/🟦️.ts";

/** 🧰️ What the plan becomes in Nx: environment variables and a report of how exactly each contract field is honoured. */
export type NxWorkerSettingsV1=Readonly<{environment:Readonly<Record<string,string>>;mapping:readonly WorkerMappingV1[]}>;

/** 🔎️ Tells whether an Nx `executors.json` lets any executor run a batch of tasks in one process, the only form of task reuse Nx offers besides its graph daemon. */
export function nxBatchCapableV1(executors:unknown):boolean{
 const rows=(executors as {executors?:Record<string,unknown>}|undefined)?.executors??{};
 return Object.values(rows).some(row=>typeof row==="object"&&row!==null&&typeof (row as {batchImplementation?:unknown}).batchImplementation==="string");
}

/** 🗺️ Maps a resolved worker plan to Nx: the local job count is `NX_PARALLEL`; multiplexing is Nx batch mode (`NX_BATCH_MODE`) and only where an executor registers a batch implementation; Nx forks one process per task, so worker pools, instance bounds, in-process actions, memory budgets and worker sandboxing are reported as unsupported. */
export function nxWorkerSettingsV1(plan:WorkerPlanV1,options:Readonly<{batchCapable:boolean}>):NxWorkerSettingsV1{
 const environment:Record<string,string>={NX_PARALLEL:String(plan.localJobs)},multiplexed=plan.enabled&&WORKER_ACTION_KINDS_V1.some(kind=>plan.kinds[kind].workers&&plan.kinds[kind].multiplex);
 if(multiplexed&&options.batchCapable)environment.NX_BATCH_MODE="true";
 const none="Nx forks one process per task";
 const mapping:WorkerMappingV1[]=[
  {field:"protocol",support:"unsupported",setting:none},
  {field:"localJobs",support:"exact",setting:`NX_PARALLEL=${plan.localJobs}`},
  {field:"resetAfterRequests",support:"unsupported",setting:none},
  {field:"sandboxing",support:"unsupported",setting:"Nx has no task sandbox"},
  {field:"cancellation",support:"approximate",setting:"Nx terminates task process trees on interrupt"},
  {field:"memory.totalMegabytes",support:"unsupported",setting:"Nx has no memory budget; lower --parallel instead"},
  {field:"memory.perWorkerMegabytes",support:"unsupported",setting:"Nx has no memory budget; lower --parallel instead"}
 ];
 for(const kind of WORKER_ACTION_KINDS_V1){
  const field=(key:string)=>`kinds.${kind}.${key}`,row=plan.kinds[kind];
  mapping.push({field:field("workers"),support:"unsupported",setting:none},{field:field("maxInstances"),support:"unsupported",setting:"Nx bounds all tasks together by NX_PARALLEL"},{field:field("inProcess"),support:"unsupported",setting:none});
  mapping.push(row.workers&&row.multiplex?(options.batchCapable?{field:field("multiplex"),support:"approximate",setting:"NX_BATCH_MODE=true"}:{field:field("multiplex"),support:"unsupported",setting:"no executor registers a batchImplementation"}):{field:field("multiplex"),support:"unsupported",setting:none});
 }
 return Object.freeze({environment:Object.freeze(environment),mapping:Object.freeze(mapping)});
}
