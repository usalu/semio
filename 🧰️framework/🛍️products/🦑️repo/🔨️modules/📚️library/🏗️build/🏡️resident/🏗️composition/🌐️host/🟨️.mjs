/** 🏗 Composition root of `repo service host --foreground`: binds the graph loader to the per-host service and ties its lifetime to the process signals. */
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {mkdirSync,openSync} from "node:fs";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {graphServiceHeapBytesV1,GRAPH_MEMORY_POLICY_V1} from "../../../🧠️memory/🟨️.mjs";
import {placeServiceOutsideJobsV1,prepareServiceGroupV1,releaseOwnServiceGroupV1,runsInServiceGroupV1,serviceLaunchArgumentsV1,sweepEmptyServiceGroupsV1} from "../../../🧱️job-limits/🐧️cgroup/🟦️.ts";
import {appendServiceExitRowV1} from "../../🧾️exit-log/🟨️.mjs";

/** 🪵️ One service log line on stderr, which the detached service redirects to its log file. */
export function writeHostServiceLogV1(event,detail){process.stderr.write(new Date().toISOString()+" "+event+(detail===undefined?"":" "+detail)+"\n");}

/** ▶ Runs the host service until it closes; resolves the exit code (75 when the memory bound forced a restart, else 0). `loadGraph({root,signal})` returns the neutral build graph of one workspace. */
export async function runHostForegroundV1({implementation,loadGraph,loadSlice,identify,stale,capture,taskState,environment=process.env,platform=process.platform,policy,signal,log=()=>{},admission,workerMemory=()=>[],workerBusy=()=>0,cache}){
 const runtime=hostEndpointsV1(implementation,environment,platform);
 const service=createHostServiceV1({runtime:{...runtime},implementation,policy,log,ports:{planner:createGraphPlannerPortV1({loadGraph,loadSlice,identify,stale,capture,taskState}),...(admission?{admission}:{}),workers:{memory:workerMemory,active:workerBusy},cache:cache??{tick:async signal=>{const[{casEnforceTickV1},{casRootV1}]=await Promise.all([import("../../../🗄️cas/🚦️guard/🟦️.ts"),import("../../../🗄️cas/🗃️store/🟦️.ts")]);await casEnforceTickV1({root:casRootV1(environment,platform),signal});}}}});
 let stopping,recorded=false;
 const record=row=>{if(recorded)return;recorded=true;const usage=process.memoryUsage();appendServiceExitRowV1(runtime.directory,{event:"exit",at:Date.now(),pid:process.pid,rssMegabytes:Math.round(usage.rss/1048576),heapMegabytes:Math.round(usage.heapUsed/1048576),workers:workerMemory(),workersBusy:workerBusy(),...row});releaseOwnServiceGroupV1();};
 const crash=error=>{record({reason:"crash",error:error?.stack??String(error)});};
 process.once("uncaughtException",error=>{crash(error);process.exit(1);});process.once("unhandledRejection",crash);
 process.once("exit",code=>record({reason:"exit",code}));
 const stop=reason=>{record({reason:"stop:"+reason});stopping??=service.stop(reason);void stopping.catch(()=>{});};
 const terminate=()=>stop("SIGTERM"),interrupt=()=>stop("SIGINT"),abort=()=>stop("cancelled");
 process.once("SIGTERM",terminate);process.once("SIGINT",interrupt);signal?.addEventListener("abort",abort,{once:true});
 try{
  try{await service.start();}catch(error){if(error?.code==="running"){record({reason:"adopted",code:0});return 0;}throw error;}
  const reason=await service.closed,code=reason==="memory"?75:0;
  record({reason:String(reason),code});
  return code;
 }finally{
  signal?.removeEventListener("abort",abort);process.off("SIGTERM",terminate);process.off("SIGINT",interrupt);
  await (stopping??service.stop("settled"));
 }
}

/** 🧷️ What belongs to the client that happened to start the service and must not follow it: its capacity grant (released when the client ends) and the plan child of its own run. */
const CLIENT_BOUND_ENVIRONMENT_V1=Object.freeze(["SEMIO_HOST_CAPACITY_GRANT","SEMIO_HOST_CAPACITY_PLAN","SEMIO_HOST_CAPACITY_PHASE_MARKS","SEMIO_BUILD_GRAPH_PLAN_OUT","SEMIO_BUILD_GRAPH_YIELD_TICKET","SEMIO_BUILD_REUSE_GRAPH","SEMIO_BUILD_GRAPH_PATH","SEMIO_BUILD_GRAPH_DIGEST"]);

/** 🚀 Starts the service detached from this process: `command` is the argument vector that runs the foreground host (`repo service host --foreground`); output goes to the service log. */
export function spawnHostServiceV1({command,environment=process.env,cwd=process.cwd()}){
 return({runtime})=>{
  mkdirSync(runtime.directory,{recursive:true,mode:0o700});
  const env={...environment,BUN_JSC_forceRAMSize:String(graphServiceHeapBytesV1()),[HOST_POLICY_V1.memoryPurge.environmentVariable]:String(HOST_POLICY_V1.memoryPurge.delayMilliseconds)};
  for(const name of CLIENT_BOUND_ENVIRONMENT_V1)delete env[name];
  const linux=process.platform==="linux",name="graph-"+createHash("sha256").update(runtime.directory).digest("hex").slice(0,12),service={name,limitMegabytes:GRAPH_MEMORY_POLICY_V1.serviceLimitMegabytes,softRatioBasisPoints:9000};
  if(linux)sweepEmptyServiceGroupsV1("graph-",undefined);
  const group=linux?prepareServiceGroupV1(service):undefined,argv=group?serviceLaunchArgumentsV1(join(group,"cgroup.procs"),command):command;
  const log=openSync(runtime.log,"a",0o600),child=spawn(argv[0],argv.slice(1),{cwd,env,stdio:["ignore",log,log],detached:true,windowsHide:true});
  child.unref();
  if(child.pid===undefined)return;
  let placed=group!==undefined&&runsInServiceGroupV1(child.pid,group);
  for(let wait=0;group!==undefined&&!placed&&wait<20;wait++){Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,10);placed=runsInServiceGroupV1(child.pid,group);}
  if(group!==undefined&&!placed)placed=placeServiceOutsideJobsV1(child.pid,service);
  appendServiceExitRowV1(runtime.directory,{event:"start",at:Date.now(),pid:child.pid,...placed&&group?{cgroup:group,limitMegabytes:service.limitMegabytes}:{placed:false}});
 };
}
