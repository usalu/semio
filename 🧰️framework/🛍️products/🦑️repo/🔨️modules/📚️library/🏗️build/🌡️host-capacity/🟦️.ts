import buildPolicy from "../🔣️policy.json" with {type:"json"};
import {adoptUpgradedHostPlanV1,attachHostCapacityProcessV1,currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,wholeHostProbeV1,hostClassBoundV1,hostProfileV1,createHostPhaseResizerV1,estimateHostPhasesV1,forecastHostQuietV1,formatHostCapacityV1,hostCapacityDirectoryV1,hostEstimateLineageV1,hostFloorMegabytesV1,hostRebuildSlotsV1,hostRequestPriorityV1,learnableHostRunV1,newHostCapacityIdV1,quietHostRequestV1,rampHostRequestV1,readHostEstimateStoreV1,recordHostForecastV1,recordHostPhasesV1,requestHostCapacityUpgradeV1,resolveHostCapacityLanguageV1,scaleHostRequestV1,sizeHostRequestV1,updateHostEstimateStoreV1,withHostCapacityV1,type HostActionKindV1,type HostRunV1,type HostCapacityHandleV1,type HostCapacityPolicyV1,type HostCapacityPortsV1,type HostRequestV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {HostUpgradeOutcomeV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {leasePriorityClassV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/📈️standing/⚖️priority/🟦️.ts";
import {availableParallelism} from "node:os";
import {agentSessionAddressV1,currentAgentSessionV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {currentWorkerHostV1,planWorkersV1,readWorkerPolicyV1,workerActionKindV1} from "../👷️workers/🟦️.ts";
import dashboardInstallation from "../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🔣️.json" with {type:"json"};
import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1,refuseCapacityV1} from "./🎛️control/🟦️.ts";
import {grantJobLimitV1,learnedRunAfterOutcomeV1} from "../🧱️job-limits/🎫️admission/🟦️.ts";
import type {JobLimitPolicyV1,JobOutcomeV1} from "../🧱️job-limits/🟦️.ts";
import {PHASELESS_COMMANDS_V1,PLANNED_COMMANDS_V1,PLANNER_PHASE_CLASSES_V1} from "./🎯️planned-commands/🟦️.ts";

export {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "./🎛️control/🟦️.ts";

/** 🅿️ Whether a grant should stay parked for the replacement process when its body failed: only while an outermost restart wrapper is open and the failure is one a fresh process cures; loaded when a body fails, never at startup. */
export async function parkGrantForRestartV1(error:unknown):Promise<boolean>{
 if(!(await import("../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts")).restartExpectedV1())return false;
 return(await import("../🧷️format-stamps/🟨️.mjs")).staleImplementationOfV1(error)!==undefined;
}


/** 🧭️ What an invocation asks the host for: the gated operation, the jobs it would like, the action kinds it will run and the classes its measured size is learned under, from general to specific. */
export type BuildCapacityAskV1=Readonly<{operation:string;preferredJobs:number;kinds:readonly HostActionKindV1[];targets:readonly string[];lineage:readonly string[]}>;

/** 🔎️ Reads the last value of one option before the argument separator. */
function option(args:readonly string[],name:string,short?:string):string|undefined{
 let last:string|undefined;
 for(let index=0;index<args.length&&args[index]!=="--";index++){
  const argument=args[index]!;
  if(argument===`--${name}`||short!==undefined&&argument===`-${short}`){const value=args[index+1];if(value!==undefined&&!value.startsWith("-")){last=value;index++;}}
  else if(argument.startsWith(`--${name}=`)||short!==undefined&&argument.startsWith(`-${short}=`))last=argument.slice(argument.indexOf("=")+1);
 }
 return last;
}

/** 🏷️ Classifies the targets an invocation names into action kinds by the worker pool's rules. */
function requestedTargets(args:readonly string[]):string[]{
 const named=args[0]==="run"?[args[1]?.split(":")[1]]:(option(args,"targets","t")??option(args,"target")??"").split(",");
 return named.filter((target):target is string=>typeof target==="string"&&target.length>0);
}
function requestedKinds(targets:readonly string[]):HostActionKindV1[]{
 const worker=readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers);
 return targets.map(target=>workerActionKindV1(worker,target,false));
}

/** 🪜️ Names the classes of one invocation: its operation, then its kinds, then the most specific thing it names (the target of `run`, the targets of the others, the cargo subcommand and package); a part never holds white space and never exceeds 120 characters. */
export function buildCapacityLineageV1(args:readonly string[],kinds:readonly HostActionKindV1[]):readonly string[]{
 const command=args[0]!,boundary=args.indexOf("--"),head=args.slice(1,boundary<0?undefined:boundary),tail=boundary<0?[]:args.slice(boundary+1),clean=(text:string)=>text.replace(/\s+/gu,"_").slice(0,120);
 const kind=[...new Set(kinds)].sort().join("+"),specific=command==="cargo"?[tail.find(argument=>!argument.startsWith("-"))??"",...(()=>{const index=tail.findIndex(argument=>argument==="-p"||argument==="--package");return index<0||tail[index+1]===undefined?[]:[tail[index+1]!];})()]:command==="run"?[head[0]??""]:[option(args,"targets","t")??option(args,"target")??head[0]??""];
 return hostEstimateLineageV1(command,...(command==="cargo"?[]:[clean(kind)]),...specific.map(clean));
}

/** 🎯️ Reads which operation, how many jobs and which kinds an already tool-stripped argument list asks for; answers undefined for light commands. */
export function buildCapacityAskV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>,policy:HostCapacityPolicyV1=BUILD_HOST_CAPACITY_POLICY_V1):BuildCapacityAskV1|undefined{
 const command=args[0]==="preflight"||args[0]==="tools"&&(args[1]==="exec"||args[1]==="setup")?"command":args[0];
 if(command===undefined||!policy.operations.some(row=>row.name===command))return undefined;
 const head=args.slice(0,args.indexOf("--")<0?undefined:args.indexOf("--"));
 if(head.some(argument=>argument==="--help"||argument==="-h"))return undefined;
 const requested=command==="cargo"?option(head,"parallelism"):option(head,"parallel"),fromEnvironment=command==="cargo"?undefined:environment.SEMIO_BUILD_PARALLEL,parsed=Number(requested??fromEnvironment);
 const preferred=Number.isSafeInteger(parsed)&&parsed>0?parsed:command==="cargo"?availableParallelism():planWorkersV1(readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers),currentWorkerHostV1()).localJobs;
 const targets=command==="cargo"?[]:requestedTargets(head),dry=command!=="cargo"&&head.some(argument=>argument==="--dry-run"||argument==="--graph"||argument.startsWith("--graph=")),kinds=dry?[]:requestedKinds(targets);
 return Object.freeze({operation:command,preferredJobs:dry?1:preferred,kinds:Object.freeze(kinds),targets:Object.freeze(targets),lineage:dry?buildCapacityLineageV1(args,kinds).map(name=>name+":dry"):buildCapacityLineageV1(args,kinds)});
}

/** 🎟️ How many graph computers the compute slots of a rebuild may run at once on this host: what the rebuild measurably took on earlier runs against the memory above the floor (the prior until it has learned), within the authored bounds; a store that cannot be read answers with the prior. */
export function buildRebuildSlotsV1(environment:Readonly<Record<string,string|undefined>>,ports:Readonly<{directory?:string;policy?:HostCapacityPolicyV1;now?:()=>number;totalMegabytes?:number}>={}):number{
 const policy=ports.policy??BUILD_HOST_CAPACITY_POLICY_V1,total=ports.totalMegabytes??hostProfileV1(environment).totalMegabytes;
 return hostRebuildSlotsV1(policy,readHostEstimateStoreV1(ports.directory??hostCapacityDirectoryV1(environment)),total,hostFloorMegabytesV1(policy,{totalMegabytes:total}),(ports.now??Date.now)());
}

/** ⏳️ Whether an invocation asks for its capacity later, from the work that needs it: `dependencies ensure` answers from the dependency receipt and requests a grant only for the real synchronisation, so a fresh receipt never queues. */
export function buildCapacityDeferredV1(args:readonly string[]):boolean{return args[0]==="dependencies"&&args[1]==="ensure";}

/** 🖥️ Whether an invocation only hands the terminal to the installed dashboard (`run`, `daemon`, `preferences`): it builds nothing, so it asks for no capacity and a busy host never delays the control plane; a stale or missing installation compiles the dashboard crate in this process. */
export async function nativeDashboardLaunchV1(args:readonly string[]):Promise<boolean>{
 if(args[0]!=="run"||!args[1]?.startsWith(dashboardInstallation.definitions.PackageOwner.const.project+":"))return false;
 return (await import("../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts")).dashboardInvocation(args)!==undefined;
}

/** ✍️ Rewrites the parallelism option of an invocation to the granted jobs so a tool can never exceed its grant. */
export function grantedBuildArgumentsV1(args:readonly string[],operation:string,jobs:number):string[]{
 const name=operation==="cargo"?"--parallelism":"--parallel",boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary),tail=boundary<0?[]:args.slice(boundary),kept:string[]=[];
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument===name){index++;continue;}if(argument.startsWith(name+"="))continue;kept.push(argument);}
 return [...kept,`${name}=${jobs}`,...tail];
}

/** 🔼️ Says that work has to run now and returns when the run may start it: under a quiet forecast grant (a run expected to execute nothing, which holds only a small reservation) it grows the reservation into the full request in the fair queue, with progress lines and cancellation through the control (an abort signal or a lease control); under a full grant it only records that the run executed, and without a grant (the gate is off, a light command) it returns at once. The executors of a build tool call it before the first thing that is not a pure replay of earlier results (the planner, a graph rebuild, the birth of every Bazel action that really runs, whose width then follows the jobs the grant holds), in the holder's process or in any process that inherits the grant; work that no executor announces grows the grant once the run's tree outgrows the quiet reservation, without pausing it.  When the growth was partial (the free memory carries fewer jobs), the plan of the calling process shrinks to the granted jobs before it returns, so the build tool started afterwards plans with them.*/
export async function upgradeBuildCapacityV1(control?:AbortSignal|ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>=process.env,reuse?:()=>Promise<boolean>):Promise<HostUpgradeOutcomeV1>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return "absent";
 const waiting=control===undefined?createCapacityControlV1():undefined,wait:ResourceLeaseControl=waiting?.control??(control instanceof AbortSignal?Object.freeze({signal:control,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))}):control as ResourceLeaseControl);
 try{const outcome=await requestHostCapacityUpgradeV1(environment,wait,reuse===undefined?undefined:{reuse});if(outcome==="held"||outcome==="upgraded")adoptUpgradedHostPlanV1(environment as Record<string,string|undefined>);return outcome;}finally{waiting?.dispose();}
}


/** 🚦️ Gives a run that was planned by the host service the capacity it executes with, once its plan is in and the scheduler lets it start: the run waited for planning and graph derivation under a small grant (so it never held execution memory while it only waited, and never starved the graph worker it waits for), here the grant grows into the full request in the fair queue and the plan of the environment follows the granted jobs, which the nested invocation turns into its arguments; a run under no grant or under a full grant waits for nothing. */
export async function admitPlannedBuildCapacityV1(control?:AbortSignal|ResourceLeaseControl,environment:NodeJS.ProcessEnv=process.env):Promise<void>{await upgradeBuildCapacityV1(control,environment);}

/** 🏊️ Asks the machine-wide gate for the worker processes of the graph inference farm beside the grant the invocation already holds (the request names that grant as its `holder`: the workers are admitted inside the unused part of its reservation while it ramps up, charged to it and not to the host, see `lendHostHoldersV1`), priced by what a worker holds (`megabytes` each, no fixed block of a run, any grant of one worker starts the farm; a request priced like a compile run needed 3584 MB for one worker and waited for ever on a host with less spare memory): an elastic request of the invocation's agent session (the first one of a farm is `essential`: admitted with one worker whatever the free memory says, the floor applies to the additional workers) (decided after every other request, it never makes another run wait and never becomes a barrier, because the farm infers in the parent without it), answered when the gate admits it, never given up on a timer; its grant observes nothing of its own, since its workers are descendants of the run whose grant already observes them. `granted` resolves with the jobs and the megabytes each of them may hold (undefined when the wait was cancelled); `release` cancels a wait or gives the grant back, and the grant is held until then. A gate that is off grants every preferred job. */
export function requestBuildFarmCapacityV1(input:HostCapacityPortsV1&Readonly<{preferredJobs:number;essential?:boolean;megabytes?:number;environment?:NodeJS.ProcessEnv;directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void;signal?:AbortSignal}>):Readonly<{granted:Promise<Readonly<{jobs:number;megabytes:number}>|undefined>;release:()=>void}>{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,environment:NodeJS.ProcessEnv={...(input.environment??process.env)},holder=environment["SEMIO_HOST_CAPACITY_GRANT"];
 delete environment["SEMIO_HOST_CAPACITY_GRANT"];delete environment["SEMIO_HOST_CAPACITY_PLAN"];
 const megabytes=Math.max(1,Math.ceil(input.megabytes??policy.kinds.script.megabytes)),request:HostRequestV1=Object.freeze({id:newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:Math.max(1,Math.min(input.preferredJobs,1024)),fixedMegabytes:0,perJobMegabytes:megabytes,cpuPerJob:1,elastic:true,...input.essential===true?{essential:true}:{},label:"graph-farm",...holder!==undefined&&/^[A-Za-z0-9_-]{8,32}$/u.test(holder)?{holder}:{}}),session=currentAgentSessionV1({environment});
 if(!policy.enabled||environment["SEMIO_HOST_CAPACITY"]==="off")return Object.freeze({granted:Promise.resolve(Object.freeze({jobs:input.preferredJobs,megabytes})),release:()=>{}});
 const controller=new AbortController(),control:ResourceLeaseControl=Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))});
 let release=()=>{},admitted=false;const held=new Promise<void>(done=>{release=done;});
 const cancel=()=>{input.signal?.removeEventListener("abort",cancel);if(!admitted)controller.abort(refuseCapacityV1("cancelled","farm-wait-cancelled"));release();};
 if(input.signal){if(input.signal.aborted)cancel();else input.signal.addEventListener("abort",cancel,{once:true});}
 const granted=new Promise<Readonly<{jobs:number;megabytes:number}>|undefined>(accept=>{
  withHostCapacityV1({observe:()=>Promise.resolve(0),...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>{admitted=true;accept(Object.freeze({jobs:handle.jobs,megabytes:Math.floor(handle.plan.memoryMegabytes/Math.max(1,handle.jobs))||megabytes}));await held;}).then(()=>accept(undefined),()=>accept(undefined));
 });
 return Object.freeze({granted,release:cancel});
}

/** 🔭️ The megabytes a process tree holds right now, for the slice pool to learn the real peak of a worker on hosts without a cgroup limit. */
export const observeHostTreeMegabytesV1=observeTreeMegabytesV1;

/** 🧿️ What the held grant of a graph plan observes: the tree of the attached slice worker, counted once. The grant is held by the service, which is the parent of the worker, so the tree of the service already holds the worker: only the attached worker is read and every other pid (the service itself) counts nothing, otherwise the worker would be booked twice (a 6.7 GB plan showed as 13.4 GB, parked every other command behind a phantom shortage).
 * @see ./🧪️tests/🧮️graph-plan/🟦️.ts */
export function graphPlanObserverV1(worker:()=>number|undefined,tree:(pid:number)=>Promise<number|null>):(pid:number)=>Promise<number|null>{
 return async pid=>pid===worker()?tree(pid):0;
}

/** 🧮️ Reserves the memory one graph plan will hold (the limit of its slice worker, which is never below the peak earlier plans of the host needed) in the machine-wide gate before the worker starts, and holds it until the worker is gone: builds that ask afterwards fit beside it or queue behind it in the fair queue instead of racing the worker for memory and cancelling it at its limit. The wait is never given up on a timer and ends with the signal; the held grant observes the worker tree that `attach` names, so the gate sees what the plan really holds; `release` is idempotent. A request with `minimumMegabytes` is admitted with what the machine has free when that is at least the minimum (the answer carries the granted `megabytes`), refused at once when the minimum can never fit and refused when nothing of ours runs and the minimum is not free after the override time; a request without a minimum that waits with nothing of ours able to free more is admitted with what the host has once it has waited the bypass time (see `unsatisfiableHostRequestV1`). A gate that is off reserves nothing. */
export function reserveGraphPlanCapacityV1(input:HostCapacityPortsV1&Readonly<{megabytes:number;minimumMegabytes?:number;environment?:NodeJS.ProcessEnv;directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void;signal?:AbortSignal}>):Promise<Readonly<{megabytes:number;attach:(pid:number)=>void;release:()=>void}>>{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,environment:NodeJS.ProcessEnv={...(input.environment??process.env)},none=Object.freeze({megabytes:Math.max(0,Math.ceil(input.megabytes)),attach:()=>undefined,release:()=>undefined});
 delete environment["SEMIO_HOST_CAPACITY_GRANT"];delete environment["SEMIO_HOST_CAPACITY_PLAN"];delete environment["SEMIO_HOST_CAPACITY_HANDOVER"];
 if(!policy.enabled||environment["SEMIO_HOST_CAPACITY"]==="off")return Promise.resolve(none);
 const request:HostRequestV1=Object.freeze({id:newHostCapacityIdV1(),operation:"graph-plan",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:Math.max(0,Math.ceil(input.megabytes)),perJobMegabytes:0,cpuPerJob:1,...input.minimumMegabytes===undefined?{}:{minimumMegabytes:Math.min(Math.max(0,Math.ceil(input.minimumMegabytes)),Math.max(0,Math.ceil(input.megabytes)))},label:"graph-plan"}),session=currentAgentSessionV1({environment}),controller=new AbortController();
 const control:ResourceLeaseControl=Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))});
 let worker:number|undefined,release=()=>{};const held=new Promise<void>(done=>{release=done;});
 const cancelWait=()=>controller.abort(input.signal?.reason??refuseCapacityV1("cancelled","graph-plan-wait-cancelled"));
 const leaveWait=()=>input.signal?.removeEventListener("abort",cancelWait);
 if(input.signal?.aborted)cancelWait();else input.signal?.addEventListener("abort",cancelWait,{once:true});
 return new Promise((accept,reject)=>{
  withHostCapacityV1({sample:previous=>readHostSampleV1(policy,wholeHostProbeV1(currentHostProbeV1()),previous,input.now,environment),...input,observe:graphPlanObserverV1(()=>worker,input.observe??observeTreeMegabytesV1),directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>{
   controller.signal.throwIfAborted();leaveWait();
   accept(Object.freeze({megabytes:handle.reservedMegabytes,attach:(pid:number)=>{worker=pid;attachHostCapacityProcessV1({...environment,SEMIO_HOST_CAPACITY_GRANT:handle.id},pid);},release}));
   await held;
  }).then(()=>undefined,error=>{leaveWait();reject(error);});
 });
}

/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy: first admits it to the cache partition (`disk.low-space` refusal before anything is written, see `admitBuildDiskV1`), then waits fairly with progress lines that name what it stands behind (its request is labelled with what it builds, e.g. the target of `repo run`, without control characters), then hands the body the arguments rewritten to the granted jobs and sets the environment the children inherit; light commands pass untouched. A run of a forecast-aware tool is learned as one that executed work when an executor asked for it or when its peak exceeded the quiet size of its class, so the quiet class learns only what quiet runs hold and the next forecast follows the run's own last outcome. */
export async function withBuildCapacityV1<T>(args:readonly string[],environment:NodeJS.ProcessEnv,body:(granted:Readonly<{args:readonly string[];handle:HostCapacityHandleV1|undefined;limit:JobLimitPolicyV1|undefined;report:(outcome:JobOutcomeV1)=>void}>)=>Promise<T>,ports:HostCapacityPortsV1&Readonly<{offloadAdmission?:import("../🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts").OffloadAdmissionPortsV1;directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void;control?:ResourceLeaseControl;admitDeferred?:boolean;tool?:string;forecast?:boolean;exitCode?:()=>number|string|null|undefined;park?:(error:unknown)=>boolean|Promise<boolean>;strip?:(args:readonly string[],environment:NodeJS.ProcessEnv)=>readonly string[];plannedByService?:boolean;repositoryRoot?:string;disk?:import("./💽️disk/🟦️.ts").BuildDiskPortsV1}>={}):Promise<T>{
 const policy=ports.policy??BUILD_HOST_CAPACITY_POLICY_V1,parsed=ports.strip?undefined:(await import("../🧭️invocation/🟦️.ts")).parseBuildInvocationV1(args,environment),stripped=ports.strip?ports.strip(args,environment):parsed!.args,tool=ports.tool??parsed?.tool??environment.SEMIO_BUILD_TOOL,requested=buildCapacityAskV1(stripped,environment,policy);
 if(!requested||buildCapacityDeferredV1(stripped)&&ports.admitDeferred!==true||await nativeDashboardLaunchV1(stripped))return body({args,handle:undefined,limit:undefined,report:()=>undefined});
 if(ports.plannedByService===true&&requested.lineage[requested.lineage.length-1]!.endsWith(":dry"))return body({args,handle:undefined,limit:undefined,report:()=>undefined});
 let ask=requested,nativeImageRecovery=false;
 if(process.platform==="darwin"&&stripped[0]==="run"&&stripped[1]?.startsWith("repo-cache-reset:cache-")&&ports.repositoryRoot!==undefined){
  const root=ports.repositoryRoot,recovery=await import("../🚀️bootstrap/⌨️cli/🟦️.ts"),plan=recovery.readNativeRepoRecoveryPlanV1(root,stripped);
  nativeImageRecovery=recovery.admitsNativeImageDiskRecoveryV1(root,plan,environment);
  if(plan?.kind==="cache-image"&&(plan.target.operation==="reclaim"||plan.target.operation==="reclaim-wait"))ask={operation:"command",preferredJobs:1,kinds:[],targets:[],lineage:hostEstimateLineageV1("command",stripped[1])};
 }
 const sized=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets}),base:HostRequestV1|undefined=sized&&Object.freeze({...sized,label:(ask.lineage.length<2?"":ask.lineage[ask.lineage.length-1]!.slice(ask.lineage[ask.lineage.length-2]!.length+1)).replace(/\p{Cc}/gu,"").slice(0,120)||ask.operation});
 if(!base)return body({args,handle:undefined,limit:undefined,report:()=>undefined});
 if(stripped[0]!=="preflight"&&!nativeImageRecovery){
  const disk=await import("./💽️disk/🟦️.ts");
  await disk.admitBuildDiskV1(environment,{write:ports.write,signal:ports.control?.signal,expectedBytes:disk.expectedDiskGrowthBytesV1(environment,requested.operation),...ports.disk});
 }
 const fleetPolicy=ask.operation==="cargo"?await import("../🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts"):undefined;
 const mode=fleetPolicy?.extractOffloadFlagV1(stripped).mode,fleetEnvironment=mode===undefined?environment:{...environment,SEMIO_OFFLOAD:mode};
 const fleetAdmission=fleetPolicy!==undefined&&fleetPolicy.offloadModeV1(fleetEnvironment)!=="never"?(await import("../🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts")).offloadFanOutJobsV1:undefined;
 const runWithFleet=()=>{
  const language=resolveHostCapacityLanguageV1(environment),write=ports.write??(line=>process.stderr.write(line+"\n"));
  write(language==="de"?"[capacity] Cargo ermittelt lokale Compilerplätze. Die Flotte kann einzelne Einheiten übernehmen.":"[capacity] Cargo admits local compiler slots. The fleet can take individual units.");
  return body({args,handle:undefined,limit:undefined,report:()=>undefined});
 };
 if(fleetAdmission!==undefined&&await fleetAdmission(fleetEnvironment,ports.offloadAdmission)!==undefined)return runWithFleet();
 const directory=ports.directory??hostCapacityDirectoryV1(environment),session=currentAgentSessionV1({environment}),now=ports.now??Date.now,write=ports.write??(line=>process.stderr.write(line+"\n")),stored=readHostEstimateStoreV1(directory),client=ports.plannedByService===true,phaseless=client||PHASELESS_COMMANDS_V1.includes(ask.operation),learned=phaseless?Object.freeze({...stored,classes:Object.fromEntries(Object.entries(stored.classes).filter(([name])=>!PLANNER_PHASE_CLASSES_V1.includes(name)))}) as typeof stored:stored,estimate=estimateHostPhasesV1(policy,learned,ask.lineage,base,now());
 const total=hostProfileV1(environment).totalMegabytes,capacity=Math.max(0,total-hostFloorMegabytesV1(policy,{totalMegabytes:total})),scaled=scaleHostRequestV1(policy,{...base,priority:hostRequestPriorityV1(base.priority,leasePriorityClassV1(environment,{agentSession:session.agent.client!=="human",interactiveTerminal:session.agent.client==="human"}),estimate)},estimate.ratioBasisPoints,capacity,estimate.floorMegabytes),measured=hostClassBoundV1(policy,learned,ask.lineage,now()).source==="measured",request=rampHostRequestV1(base,scaled,estimate,measured),growth=estimate.growRatioBasisPoints<estimate.ratioBasisPoints?rampHostRequestV1(base,scaleHostRequestV1(policy,{...base,priority:request.priority},estimate.growRatioBasisPoints,capacity,estimate.floorMegabytes),estimate,measured):request;
 if(estimate.floorMegabytes>estimate.phases.execution!.megabytes)write(formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"phased",{operation:ask.operation,percent:Math.round(estimate.ratioBasisPoints/100),samples:estimate.samples,floor:estimate.floorMegabytes,phase:(estimate.phases.plan?.megabytes??0)>=(estimate.phases.assembly?.megabytes??0)?"plan":"assembly"}));
 else if(estimate.samples>0)write(formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"estimated",{operation:ask.operation,percent:Math.round(estimate.ratioBasisPoints/100),samples:estimate.samples}));
 const leaf=ask.lineage[ask.lineage.length-1]!,forecastable=ports.forecast!==false&&tool==="bazel"&&policy.operations.find(row=>row.name===ask.operation)?.forecast===true,forecast=forecastable||client?forecastHostQuietV1(policy,learned,leaf,now()):undefined,small=forecast!==undefined&&forecast.megabytes<growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes,quiet=small&&(client||forecast!.quiet),admission=quiet?quietHostRequestV1(growth,forecast!):request;
 if(quiet&&!client)write(forecast!.followUpMilliseconds===undefined?formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"quiet",{operation:ask.operation,samples:forecast!.samples,percent:Math.round(forecast!.shareBasisPoints/100),reserved:admission.fixedMegabytes,full:growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes}):formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"quietFollowUp",{operation:ask.operation,seconds:Math.round(forecast!.followUpMilliseconds/1000),reserved:admission.fixedMegabytes,full:growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes}));
 let breached:JobOutcomeV1|undefined;
 const learn=async(run:HostRunV1)=>{
  run=learnedRunAfterOutcomeV1(run,breached);
  const code=(ports.exitCode??(()=>process.exitCode))(),status=run.status==="ok"&&code!==undefined&&code!==null&&Number(code)!==0?"failed" as const:run.status,defaultMegabytes=Math.max(1,base.fixedMegabytes+run.jobs*base.perJobMegabytes);
  const forecasted=run.executed===undefined?undefined:{executed:run.executed||!client&&forecast!==undefined&&run.peakMegabytes>forecast.megabytes,peakMegabytes:run.peakMegabytes,activeMilliseconds:run.activeMilliseconds,peakAtMilliseconds:run.peakAtMilliseconds,endedAtMs:run.endedAtMs};
  if(forecasted!==undefined&&!forecasted.executed){if(status==="ok")await updateHostEstimateStoreV1(directory,policy,store=>recordHostForecastV1(policy,store,leaf,forecasted),{now});return;}
  const own=run.phases===undefined?run.peakMegabytes:Math.max(0,...run.phases.filter(row=>row.phase!=="rebuild").map(row=>row.peakMegabytes));
  if(!learnableHostRunV1(status,Math.max(1,Math.floor(own*10000/defaultMegabytes+0.5)),estimate))return;
  await updateHostEstimateStoreV1(directory,policy,store=>{const next=recordHostPhasesV1(policy,store,ask.lineage,ask.operation,defaultMegabytes,{...run,...phaseless&&run.phases?{phases:run.phases.filter(row=>!PLANNER_PHASE_CLASSES_V1.includes("phase:"+row.phase))}:{},keepPeak:true,ceilingMegabytes:capacity});return forecasted!==undefined&&status==="ok"?recordHostForecastV1(policy,next,leaf,forecasted):next;},{now});
 };
 const waiting=ports.control?undefined:createCapacityControlV1(),tasks=request.perJobMegabytes>0,deferring=quiet&&client;
 try{
  return await withHostCapacityV1({park:parkGrantForRestartV1,...ports,learn,resize:createHostPhaseResizerV1(policy,base,estimate,capacity),directory,policy,request:admission,...forecastable||quiet?{upgrade:growth}:{},...deferring?{quietUpgradeRequiresRequest:true}:{},control:ports.control??waiting!.control,environment,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>{
   waiting?.dispose();
   const previous=Object.fromEntries([...Object.keys(handle.environment),"SEMIO_BUILD_PARALLEL"].map(key=>[key,environment[key]]));
   Object.assign(environment,handle.environment,tasks?{SEMIO_BUILD_PARALLEL:String(handle.jobs)}:{});
   try{return await body({args:tasks&&!deferring?grantedBuildArgumentsV1(args,ask.operation,handle.jobs):args,handle,limit:grantJobLimitV1(handle),report:outcome=>{breached=outcome;}});}
   finally{for(const [key,value] of Object.entries(previous))if(value===undefined)delete environment[key];else environment[key]=value;}
  },fleetAdmission===undefined?undefined:async signal=>{
   const fan=await fleetAdmission(fleetEnvironment,{...ports.offloadAdmission,signal}).catch(()=>undefined);
   return fan===undefined?undefined:()=>{waiting?.dispose();return runWithFleet();};
  });
 }finally{waiting?.dispose();}
}
