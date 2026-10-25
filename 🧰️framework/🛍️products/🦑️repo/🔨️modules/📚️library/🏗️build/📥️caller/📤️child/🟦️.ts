import type { NativeBuildCallerCapabilitiesV1 } from "../📋️contract/🟦️.ts";
import callerSchema from"../../🧬️schema/📥️caller.json" with { type: "json" };
import {buildOperationHasNeutralBootstrapV1} from "../../⚙️operations/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {advanceScriptInvocation,scriptInvocationBudget,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {compilerAdmissionBudgetV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🛠️ Declares concrete resources for the selected build implementation. */
export async function createBuildBootstrapCapabilitiesV1(root:string,arguments_:readonly string[],environment:NodeJS.ProcessEnv) {
  const {parseBuildInvocationV1,resolveBuildInvocationV1,readBuildDependencyInvocationV1}=await import("../../🧭️invocation/🟦️.ts");
  const args=arguments_[0]==="repo"?arguments_.slice(1):arguments_,authored=environment.SEMIO_SCRIPT_CAPABILITIES===undefined?undefined:JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES),effective={...environment,...(environment.SEMIO_BUILD_MODE===undefined&&authored?.buildMode?{SEMIO_BUILD_MODE:authored.buildMode}:{})},selected=parseBuildInvocationV1(args,effective),resolved=resolveBuildInvocationV1(root,selected.args,effective);
  const {createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("../🟦️.ts");
  if(arguments_[0]==="repo"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts"),command=resolved.args[0];if(command!==undefined&&!buildInvocationRequestsHelpV1(resolved.args)){const known=((await import("../../⚙️operations/🔣️catalog.json",{with:{type:"json"}})).default.operations as readonly {name:string}[]).map(row=>row.name);if(!known.includes(command))throw Error(`Unknown repo command "${command}"; expected one of: ${known.join(", ")} / Unbekannter repo-Befehl "${command}"; erwartet wird einer von: ${known.join(", ")}`);}}
  if(resolved.args[0]==="preflight"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts");if(!buildInvocationRequestsHelpV1(resolved.args)){const {readBuildPreflightArgumentsV1}=await import("../../🩺️preflight/🟦️.ts");readBuildPreflightArgumentsV1(resolved.args);}}
  if(resolved.args[0]==="incremental"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts");if(!buildInvocationRequestsHelpV1(resolved.args)){const {incrementalArgumentsV1}=await import("../../📏️incremental/⌨️command/🟦️.ts");incrementalArgumentsV1(resolved.args.slice(1));}}
  if(resolved.args[0]==="conformance"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts");if(!buildInvocationRequestsHelpV1(resolved.args)){const {conformanceRequestV1}=await import("../../📐️conformance/⌨️command/🟦️.ts");conformanceRequestV1(resolved.args);}}
  if(resolved.args[0]==="concurrency"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts");if(!buildInvocationRequestsHelpV1(resolved.args)){const {concurrencyArgumentsV1}=await import("../../📏️incremental/🤝️concurrent/⌨️command/🟦️.ts");concurrencyArgumentsV1(resolved.args.slice(1));}}
  if(resolved.args[0]==="bench"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts");if(!buildInvocationRequestsHelpV1(resolved.args)){const {benchArgumentsV1}=await import("../../📏️incremental/🏎️bench/⌨️command/🟦️.ts");benchArgumentsV1(resolved.args.slice(1));}}
  const grant=authored===undefined?createNativeBuildCallerCapabilitiesV1(root,selected.args,selected.tool,{...effective,...resolved.env}):readBuildAuthoringCapabilitiesV1(root,selected.args,selected.tool,authored);
  if(effective.SEMIO_BUILD_MODE!==undefined&&effective.SEMIO_BUILD_MODE!==grant.buildMode)throw Error("Original authored build mode changed");
  const expanded=await expandBuildBootstrapCapabilitiesV1(root,selected.args,effective,grant);
  if(selected.args[0]!=="service"||selected.args[1]!=="executor")return expanded;
  const{createServiceGrantV1}=await import("../🛰️service/🟦️.ts"),holder={...expanded,service:createServiceGrantV1(root,effective)};
  if(validateJsonSchemaSubset(callerSchema,holder).length)throw Error("Closed build service capabilities required");return holder;
}

async function expandBuildBootstrapCapabilitiesV1(root:string,args:readonly string[],effective:NodeJS.ProcessEnv,grant:NativeBuildCallerCapabilitiesV1){
 const{readNativeRepoRecoveryPlanV1,nativeRepoRecoveryBuildDescendantsV1}=await import("../../🚀️bootstrap/⌨️cli/🟦️.ts");
 const{parseBuildInvocationV1,resolveBuildInvocationV1}=await import("../../🧭️invocation/🟦️.ts"),{nxOperationArgumentsV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts"),selected=parseBuildInvocationV1(args,{...effective,SEMIO_BUILD_TOOL:grant.tool});if(selected.tool!==grant.tool)throw Error("Original bootstrap build implementation changed");
  const {watchBuildChildArgumentsV1,maximumWatchBuildDescendantsV1}=await import("../../🧭️selection/👁️watch/🟦️.ts"),launches=[...grant.launches],pending=[selected.args],seen=new Set<string>();
  const add=(request:readonly string[])=>{if(!launches.some(owned=>JSON.stringify(owned)===JSON.stringify(request)))launches.push([...request]);};
  while(pending.length){
   const args=pending.shift()!,key=JSON.stringify(args);if(seen.has(key))continue;if(seen.size>=maximumWatchBuildDescendantsV1)throw Error("Bounded watch descendant declarations required");seen.add(key);
   const parsed=parseBuildInvocationV1(args,{...effective,SEMIO_BUILD_TOOL:selected.tool});if(parsed.tool!==selected.tool)throw Error("Watch callback cannot change the original build implementation");
   const invocation=resolveBuildInvocationV1(root,parsed.args,{...effective,SEMIO_BUILD_MODE:grant.buildMode}),command=invocation.args[0],tools=invocation.args[2]??selected.tool,nativeRecovery=readNativeRepoRecoveryPlanV1(root,invocation.args),main=nativeRecovery?undefined:await(await import("../../🔌️backends/🟦️.ts")).buildBackendBootstrapV1(invocation.args,selected.tool);
   add(["repo",...parsed.args]);
   for(const child of nativeRepoRecoveryBuildDescendantsV1(root,invocation.args))pending.push([...child]);
   if(command==="run"&&invocation.args[1]?.startsWith("@semio-tech/repo-dashboard-rs:")){const{dashboardBuildDescendantsV1}=await import("../../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts");for(const child of dashboardBuildDescendantsV1(invocation.args))pending.push(child);}
   if(main){const{resolveNxBootstrapLaunchesV1}=await import("../../../⚡️caching/🚀️bootstrap/📥️caller/🟦️.ts"),nx=resolveNxBootstrapLaunchesV1(root,[main.namespace,...main.arguments],{...effective,...invocation.env,SEMIO_BUILD_MODE:grant.buildMode});for(const launch of nx.launches)add([main.namespace,...launch]);}
   const boundary=invocation.args.indexOf("--"),callback=command==="watch"?watchBuildChildArgumentsV1(root,boundary<0?[]:invocation.args.slice(boundary+1)):undefined;
   if(callback)pending.push(callback);if(invocation.watch)pending.push(["run",invocation.watch,"--output=stream"]);
  }
  const capabilities={...grant,launches};if(validateJsonSchemaSubset(callerSchema,capabilities).length)throw Error("Closed build bootstrap capabilities required");return capabilities;
}

/** 📨️ Projects a declared neutral child beneath the same exact active issued phase. The child is its own root with its own declared request, so it never inherits the pinned graph of its parent: that graph admits only the parent's selection and the child plans (or reads from the sealed generation) exactly its own. */
export async function projectOwnedBuildChildEnvironmentV1(root:string,args:readonly string[],parent:ScriptInvocation,phaseControl:ResourceLeaseControl,changes?:import("../../🧭️selection/👁️watch/🟦️.ts").BuildWatchChangesV1):Promise<NodeJS.ProcessEnv>{
 requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);
 const labels=changes===undefined?{}:(await import("../../🧭️selection/👁️watch/🟦️.ts")).buildWatchChangesEnvironmentV1(changes);checkResourceLeaseControl(phaseControl);
 const{readOwnedBuildInvocationV1}=await import("../🟦️.ts");checkResourceLeaseControl(phaseControl);
 const{parseBuildInvocationV1}=await import("../../🧭️invocation/🟦️.ts");checkResourceLeaseControl(phaseControl);
 const{readIssuedScriptProcessEnvelopeV1,scriptProcessEnvironment}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");checkResourceLeaseControl(phaseControl);
 const original=readIssuedScriptProcessEnvelopeV1(parent),caller=readOwnedBuildInvocationV1(root,parent,args),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:caller.tool}).args,request=["repo",...selected],declared={...caller,arguments:request,selection:request,launches:[request]},environment={...process.env,SEMIO_BUILD_TOOL:caller.tool,SEMIO_BUILD_MODE:caller.buildMode,CARGO_NET_OFFLINE:String(caller.native.network.offline),CARGO_TARGET_DIR:caller.native.artifactDirectory,SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(declared)},capabilities=await expandBuildBootstrapCapabilitiesV1(root,selected,environment,declared);
 checkResourceLeaseControl(phaseControl);if(JSON.stringify((capabilities as any).native)!==JSON.stringify(caller.native))throw Error("Original neutral child native resources changed");
 return scriptProcessEnvironment({...original,capabilities},{...environment,...labels,SEMIO_BUILD_GRAPH_PATH:undefined,SEMIO_BUILD_GRAPH_DIGEST:undefined,SEMIO_BUILD_REUSE_GRAPH:undefined,SEMIO_SCRIPT_CAPABILITIES:undefined,SEMIO_SCRIPT_POLICY:JSON.stringify(original.policy)});
}

/** 🧷️ Launches a predeclared neutral descendant within its actual parent's finite phase and signal. */
export async function runOwnedBuildChildInvocationV1(root:string,args:readonly string[],parent:ScriptInvocation,phaseControl:ResourceLeaseControl,changes?:import("../../🧭️selection/👁️watch/🟦️.ts").BuildWatchChangesV1):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);
 const child=await projectOwnedBuildChildEnvironmentV1(root,args,parent,phaseControl,changes);checkResourceLeaseControl(phaseControl);
 const progress=async()=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(parent,"build:neutral-child","running");checkResourceLeaseControl(phaseControl);};await progress();
 const milliseconds=compilerAdmissionBudgetV1(phaseControl);
 await runOwnedCommand(process.execPath,["repo",...args],root,"build:neutral-child",scriptInvocationBudget(parent,milliseconds),{env:child,signal:phaseControl.signal,onProgress:progress});checkResourceLeaseControl(phaseControl);
}
