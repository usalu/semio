import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {homedir,hostname} from "node:os";
import {delimiter,join} from "node:path";
import {evaluateHostCapacityV1,requireHostCapacityV1,withUnmanagedHostV1,type HostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {hostCapacityDirectoryV1,readHostCapacityStatusV1,type HostCapacityStatusV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import {readHostAsksV1,type HostAskV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../🌡️host-capacity/🎛️control/🟦️.ts";
import {remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyV1} from "./🔎️why/🟦️.ts";
import {readReadyStageClaimV1,withReadyStageV1,type ReadyStageClaimV1} from "./🪑️stage-claim/🟦️.ts";
import admissionSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import {requireRemoteExecutionShapeV1,type RemoteExecutionConfinementV1,type RemoteExecutionNetworkV1,type RemoteExecutionRefusalV1,type RemoteExecutionSandboxV1} from "../🟦️.ts";

/** 🧬️ The platform key of a machine: operating system, architecture and the build id of its Rust toolchain; units are shared and offloaded only between machines with an equal key. */
export type RemoteExecutionPlatformIdentityV1=Readonly<{key:string;os:string;arch:string;toolchain:string|null;target:string|null}>;

/** 📈️ The counters of the rustc wrapper and the unit cache of one machine. */
export type RemoteExecutionMachineCountersV1=Readonly<{unit:Readonly<{localHit:number;remoteHit:number;miss:number;stored:number;skipped?:number}>;skipReasons?:Readonly<Record<string,number>>;keySamples?:Readonly<{blob:string;count:number;atMs:number}>;offload:Readonly<{out:number;refused:number;fellBack:number}>}>;

/** 👷️ What the build worker of a machine does right now. */
export type RemoteExecutionWorkerStateV1=Readonly<{state:"absent"|"idle"|"busy"|"refusing";concurrency:number;running:number;sandbox:RemoteExecutionSandboxV1|null;confinement:RemoteExecutionConfinementV1|null;network:RemoteExecutionNetworkV1|null;why?:RemoteExecutionWorkerWhyV1}>;

export {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyCodeV1,type RemoteExecutionWorkerWhyV1} from "./🔎️why/🟦️.ts";

/** 📡️ What every machine sends to the scheduler every few seconds (`LoadReport` of the schema). */
export type RemoteExecutionLoadReportV1=Readonly<{version:1;origin:string;platform:RemoteExecutionPlatformIdentityV1;capacity:Readonly<{cores:number;memoryMegabytes:number}>;running:number;queued:number;pressure:"normal"|"warn"|"critical";freeMegabytes:number;load:number|null;headroom:boolean;wallMs:number;worker:RemoteExecutionWorkerStateV1;counters:RemoteExecutionMachineCountersV1}>;

/** ⚖️ The verdict of the admission of one more remote action. */
export type RemoteExecutionAdmissionV1=Readonly<{admit:true}>|Readonly<{admit:false;reason:RemoteExecutionRefusalV1}>;

/** 🔭️ What the admission looks at: the host sample, the local ledger and queue of the host capacity gate. */
export type RemoteExecutionHostViewV1=Pick<HostCapacityStatusV1,"host"|"grants"|"waiters">&Readonly<{asks?:readonly HostAskV1[];stage?:ReadyStageClaimV1}>;

const sha256=(data:string):string=>createHash("sha256").update(data).digest("hex");

/** 🔑️ The toolchain build id: the first 16 hex digits of the sha256 of the exact `rustc -vV` text (the text the unit cache keys on). */
export function remoteExecutionToolchainIdV1(versionText:string):string{return sha256(versionText).slice(0,16);}

/** 🎯️ The host triple of a `rustc -vV` text, `null` when it has none. */
export function remoteExecutionTargetOfV1(versionText:string):string|null{return /^host:\s*(\S+)/mu.exec(versionText)?.[1]??null;}

/** 🔖️ Builds the platform key of a machine. */
export function remoteExecutionPlatformIdentityV1(platform:string,arch:string,toolchain:string|null,target:string|null):RemoteExecutionPlatformIdentityV1{
 const os=platform==="darwin"?"macos":platform==="win32"?"windows":platform,cpu=arch==="x64"?"amd64":arch;
 return Object.freeze({key:`${os}-${cpu}-${toolchain??"none"}`,os,arch:cpu,toolchain,target});
}

let probed:Promise<Readonly<{toolchain:string|null;target:string|null}>>|undefined;

/** 🦀️ Reads the toolchain build id and host triple of this machine from `rustc -vV` run in `cwd` (so the pinned toolchain of the repository answers; `~/.cargo/bin` joins the search path because a service manager starts with a bare one), or from the text the launcher states in `SEMIO_UNIT_CACHE_TOOLCHAIN`; `null` for both when the compiler does not answer within `timeoutMilliseconds`. Memoised for the life of the process. */
export function readRemoteExecutionToolchainV1(cwd:string,environment:Readonly<Record<string,string|undefined>>=process.env,timeoutMilliseconds:number=20_000):Promise<Readonly<{toolchain:string|null;target:string|null}>>{
 const stated=environment.SEMIO_UNIT_CACHE_TOOLCHAIN;
 if(stated)return Promise.resolve({toolchain:remoteExecutionToolchainIdV1(stated),target:remoteExecutionTargetOfV1(stated)});
 return probed??=new Promise(done=>{
  let out="",finished=false;
  const finish=(value:Readonly<{toolchain:string|null;target:string|null}>):void=>{if(finished)return;finished=true;clearTimeout(timer);done(value);};
  let proc;
  try{proc=spawn("rustc",["-vV"],{cwd,env:{...environment,PATH:[environment.PATH,join(homedir(),".cargo","bin")].filter(Boolean).join(delimiter)} as NodeJS.ProcessEnv,stdio:["ignore","pipe","ignore"],windowsHide:true});}catch{finish({toolchain:null,target:null});return;}
  const timer=setTimeout(()=>{proc.kill("SIGKILL");finish({toolchain:null,target:null});},timeoutMilliseconds);
  proc.stdout.on("data",chunk=>{out+=String(chunk);});
  proc.on("error",()=>{finish({toolchain:null,target:null});});
  proc.on("close",code=>{finish(code===0&&out.includes("host:")?{toolchain:remoteExecutionToolchainIdV1(out),target:remoteExecutionTargetOfV1(out)}:{toolchain:null,target:null});});
 });
}

const ZERO:RemoteExecutionMachineCountersV1=Object.freeze({unit:Object.freeze({localHit:0,remoteHit:0,miss:0,stored:0}),offload:Object.freeze({out:0,refused:0,fellBack:0})});

/** 🧮️ Reads the counters of the wrapper from `.cache/f/build-counters.json` below the worktree (zeros when the file is absent or not of the closed shape). */
export function readRemoteExecutionCountersV1(root:string):RemoteExecutionMachineCountersV1{
 try{
  const value=JSON.parse(readFileSync(machinePathV1(root,".cache","f","build-counters.json"),"utf8")) as {unit?:unknown;skipReasons?:unknown;keySamples?:unknown;offload?:unknown};
  const counters={unit:value.unit,...value.skipReasons===undefined?{}:{skipReasons:value.skipReasons},...value.keySamples===undefined?{}:{keySamples:value.keySamples},offload:value.offload};
  requireRemoteExecutionShapeV1("MachineCounters",counters,"Closed build counters required");
  return counters as RemoteExecutionMachineCountersV1;
 }catch{return ZERO;}
}

/** 🆔️ The origin id of this machine: the explicit one, else `machine.json` of the replica, else the host name. */
export function remoteExecutionOriginV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const explicit=environment.SEMIO_FLEET_ORIGIN;
 if(explicit&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(explicit))return explicit;
 try{const id=(JSON.parse(readFileSync(machinePathV1(root,".cache","f","machine.json"),"utf8")) as {originId?:string}).originId;if(id&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(id))return id;}catch{}
 const name=hostname().replace(/[^A-Za-z0-9._-]/gu,"-").replace(/^[^A-Za-z0-9]+/u,"").slice(0,64);
 return name===""?"machine":name;
}

/** 💾️ The megabytes the admission reserves for one remote action: the host capacity policy's build kind. */
export const remoteExecutionActionMegabytesV1=(policy:HostCapacityPolicyV1=BUILD_HOST_CAPACITY_POLICY_V1):number=>policy.kinds.build.megabytes;

/** 📐️ The numbers of the admission of one unit.
 * @see ./🧬️schema/🔣️.json */
export const REMOTE_EXECUTION_ADMISSION_POLICY_V1=admissionSchema.definitions.Policy.const;

/** 🌡️ What the memory pressure of the host means for a unit of `megabytes`: `pressure-critical` refuses every unit; raised pressure (`warn`) alone refuses only a unit above `warnMaximumMegabytes` or while swap is actually growing (the swap-out rate at or above the policy's active share of the memory per second), else the unit is judged as on a calm host (`relaxed`); `normal` otherwise.
 * @see ../🧪️tests/🏋️unit-admission/🐍️.py */
export function remoteExecutionPressureGateV1(host:Pick<HostSampleV1,"pressure"|"totalMegabytes"|"swapOutMegabytesPerSecond">,megabytes:number,swapActiveBasisPointsPerSecond:number,rules:typeof REMOTE_EXECUTION_ADMISSION_POLICY_V1=REMOTE_EXECUTION_ADMISSION_POLICY_V1):"normal"|"relaxed"|"pressure-warn"|"pressure-critical"{
 if(host.pressure==="critical")return "pressure-critical";
 if(host.pressure==="normal")return "normal";
 const rate=host.swapOutMegabytesPerSecond;
 return megabytes>rules.warnMaximumMegabytes||rate!==null&&rate*10000>=host.totalMegabytes*swapActiveBasisPointsPerSecond?"pressure-warn":"relaxed";
}

/** 🧱️ One remote unit this machine runs, as its admission charges it: the peak it was admitted with (`megabytes`) and what its process group holds now (`usedMegabytes`, 0 before it started or where nothing measures it). */
export type RemoteExecutionHeldUnitV1=Readonly<{megabytes:number;usedMegabytes:number}>;

/** 🎚️ Decides whether this machine takes one more remote unit of `megabytes` (its predicted peak; without one the host capacity policy's build kind) and what a unit may still take: it asks the single admission decision of the host (`evaluateHostCapacityV1`) as a request of the lent class (served after every request of this machine, admitted only while none of them waits) for one build job of that size beside the grants of its local builds, the readiness stage (its measured use plus margin, nothing while it waits) and the running remote units (`held`; without it one build kind each, taken as already held in the sample). Each running unit is charged once: what it holds now is already in the sample, the part of its peak it has not grown into yet (`megabytes - usedMegabytes`) is kept free for it, so two units admitted a second apart never both count on the memory the first is about to take. A worker with all its slots busy refuses with `slots`; beyond that only measured memory decides how many units it lends, also while its readiness stage runs: the stage keeps its own reserve (the claim in the view), and a fixed count of one unit left a worker whose stage reads for hours with memory to spare idle for every final of the fleet. Pressure goes through {@link remoteExecutionPressureGateV1}: a stage that runs no longer refuses a small unit under `warn`. `spareMegabytes` is what the decision leaves above the floor. Pure.
 * @see ../🧪️tests/🏋️unit-admission/🐍️.py */
export function evaluateRemoteExecutionAdmissionV1(input:Readonly<{view:RemoteExecutionHostViewV1;running:number;concurrency:number;nowMs:number;policy?:HostCapacityPolicyV1;megabytes?:number;held?:readonly RemoteExecutionHeldUnitV1[]}>):Readonly<{admission:RemoteExecutionAdmissionV1;spareMegabytes:number}>{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,megabytes=input.megabytes??remoteExecutionActionMegabytesV1(policy),gate=remoteExecutionPressureGateV1(input.view.host,megabytes,policy.swap.activeBasisPointsPerSecond);
 const held=input.held??Array.from({length:input.running},()=>({megabytes:remoteExecutionActionMegabytesV1(policy),usedMegabytes:remoteExecutionActionMegabytesV1(policy)}));
 const remote:HostGrantV1[]=held.map((row,index)=>({id:`remote-${index}`,operation:"remote-execution",jobs:1,cpu:1,reservedMegabytes:row.megabytes,admittedAtMs:input.nowMs,observedMegabytes:row.usedMegabytes,pid:process.pid,bootAtMs:0})),host:HostSampleV1=withUnmanagedHostV1(gate==="relaxed"?{...input.view.host,pressure:"normal"}:input.view.host,[...input.view.grants,...remote]);
 const waiter:HostWaiterV1={request:{id:"remote-candidate",operation:"remote-execution",priority:"lent",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:input.nowMs,pid:process.pid};
 const evaluation=evaluateHostCapacityV1(policy,host,[...input.view.grants,...remote],[...input.view.waiters,waiter],input.nowMs),decision=evaluation.decisions.find(row=>row.id==="remote-candidate"),spareMegabytes=Math.max(0,Math.round(evaluation.headroomMegabytes));
 if(input.running>=input.concurrency)return {admission:{admit:false,reason:"slots"},spareMegabytes};
 if(gate==="pressure-warn"||gate==="pressure-critical")return {admission:{admit:false,reason:gate},spareMegabytes};
 if(decision?.verdict==="admit")return {admission:{admit:true},spareMegabytes};
 return {admission:{admit:false,reason:(decision?.reason??"memory") as RemoteExecutionRefusalV1},spareMegabytes};
}

/** ⚖️ The verdict of {@link evaluateRemoteExecutionAdmissionV1} alone. */
export const admitRemoteExecutionActionV1=(input:Parameters<typeof evaluateRemoteExecutionAdmissionV1>[0]):RemoteExecutionAdmissionV1=>evaluateRemoteExecutionAdmissionV1(input).admission;

/** 🔬️ Reads the host capacity gate of this machine without joining its queue: the one source of truth for pressure, free memory and load. `SEMIO_REMOTE_EXECUTION_HOST_VIEW` names a JSON file `{host,grants,waiters}` (a `Host` sample of the host capacity schema) that stands in for the machine, the way the host capacity gate itself accepts a declared host: tests and simulations drive pressure and load through it, and the file is read again on every call. */
export async function readRemoteExecutionHostViewV1(environment:Readonly<Record<string,string|undefined>>=process.env,policy:HostCapacityPolicyV1=BUILD_HOST_CAPACITY_POLICY_V1):Promise<RemoteExecutionHostViewV1>{
 const simulated=environment.SEMIO_REMOTE_EXECUTION_HOST_VIEW;
 if(simulated!==undefined&&simulated!==""){
  const value=JSON.parse(readFileSync(simulated,"utf8")) as RemoteExecutionHostViewV1;
  requireHostCapacityV1<HostSampleV1>("Host",value.host);
  return withReadyStageV1({host:value.host,grants:value.grants??[],waiters:value.waiters??[],asks:value.asks??[]},readReadyStageClaimV1(hostCapacityDirectoryV1(environment),Date.now()),Date.now());
 }
 const controller=new AbortController(),control={signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))};
 const status=await readHostCapacityStatusV1({directory:hostCapacityDirectoryV1(environment),policy,control,environment});
 return withReadyStageV1({host:status.host,grants:status.grants,waiters:status.waiters,asks:readHostAsksV1(hostCapacityDirectoryV1(environment))},readReadyStageClaimV1(hostCapacityDirectoryV1(environment),Date.now()),Date.now());
}

/** 📻️ Builds the load report of this machine from the host view: the pressure level is the sample's, `running` counts the local grants and the remote actions, `queued` the local waiters, `freeMegabytes` is what a unit may still take above the machine's floor (the memory the scheduler places a unit's predicted peak against), and `headroom` is this machine's own admission verdict for the smallest unit (slots are the scheduler's business, so a full worker still has headroom; whether a larger unit fits is the scheduler's comparison with `freeMegabytes`). The running remote units count here by what they hold now only: the part of their peak they have not grown into yet is charged by the scheduler (`placementChargeMegabytesV1`), so reporting it too would charge every growing unit twice. */
export function buildRemoteExecutionLoadReportV1(input:Readonly<{origin:string;identity:RemoteExecutionPlatformIdentityV1;view:RemoteExecutionHostViewV1;worker:RemoteExecutionWorkerStateV1;counters:RemoteExecutionMachineCountersV1;code?:string;nowMs:number;policy?:HostCapacityPolicyV1;held?:readonly RemoteExecutionHeldUnitV1[]}>):RemoteExecutionLoadReportV1{
 const host=input.view.host,held=(input.held??Array.from({length:input.worker.running},()=>({megabytes:0,usedMegabytes:0}))).map(row=>({megabytes:row.usedMegabytes,usedMegabytes:row.usedMegabytes}));
 const {admission,spareMegabytes}=evaluateRemoteExecutionAdmissionV1({view:input.view,running:input.worker.running,concurrency:Number.MAX_SAFE_INTEGER,nowMs:input.nowMs,megabytes:REMOTE_EXECUTION_ADMISSION_POLICY_V1.smallestMegabytes,held,...input.policy?{policy:input.policy}:{}});
 const report:RemoteExecutionLoadReportV1={version:1,origin:input.origin,platform:input.identity,capacity:{cores:Math.max(1,host.cores),memoryMegabytes:Math.max(1,host.totalMegabytes)},running:input.view.grants.length+input.worker.running,queued:input.view.waiters.length,pressure:host.pressure,freeMegabytes:spareMegabytes,load:host.load===null?null:Math.max(0,host.load),headroom:admission.admit,wallMs:input.nowMs,worker:input.worker.state==="absent"||admission.admit?input.worker:{...input.worker,state:"refusing",why:remoteExecutionWorkerWhyV1("no-headroom",admission.reason)},counters:input.counters,...input.code===undefined?{}:{code:input.code}};
 requireRemoteExecutionShapeV1("LoadReport",report,"Closed load report required");
 return report;
}

/** 📣️ Something the reporter did, for logs. */
export type RemoteExecutionReporterEventV1=Readonly<{stage:"sent"|"failed";detail:string}>;

/** 📶️ Sends the load report of this machine to the scheduler every `intervalSeconds` until the signal aborts. One report per round, a round never overlaps the next, every send has a timeout, and a failing scheduler only produces an event (a build never depends on it). */
export async function runRemoteExecutionReporterV1(options:Readonly<{scheduler:string;token?:string|undefined;fetch?:typeof fetch|undefined;intervalSeconds:number;signal:AbortSignal;report:()=>Promise<RemoteExecutionLoadReportV1>;onEvent?:(event:RemoteExecutionReporterEventV1)=>void;timeoutMilliseconds?:number}>):Promise<void>{
 const url=options.scheduler.replace(/\/+$/u,"")+"/v1/fleet/load";
 let lastFailure="";
 while(!options.signal.aborted){
  try{
   const report=await options.report();
   const response=await(options.fetch??fetch)(url,{method:"POST",signal:AbortSignal.any([options.signal,AbortSignal.timeout(options.timeoutMilliseconds??5000)]),headers:{"content-type":"application/json",...options.token?{authorization:`Bearer ${options.token}`}:{}},body:JSON.stringify(report)});
   await response.body?.cancel();
   if(!response.ok)throw Error(`Scheduler answered ${response.status}`);
   if(lastFailure!==""){lastFailure="";options.onEvent?.({stage:"sent",detail:"reachable again"});}
  }catch(error){
   if(options.signal.aborted)break;
   const detail=(error as Error).message;
   if(detail!==lastFailure){lastFailure=detail;options.onEvent?.({stage:"failed",detail});}
  }
  await new Promise<void>(done=>{const timer=setTimeout(done,options.intervalSeconds*1000);options.signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
 }
}
