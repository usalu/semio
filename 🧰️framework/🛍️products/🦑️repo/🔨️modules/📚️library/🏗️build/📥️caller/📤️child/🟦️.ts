import {checkScriptInvocation,advanceScriptInvocation,scriptInvocationBudget} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🛠️ Declares concrete resources for the selected build implementation. */
export async function createBuildBootstrapCapabilitiesV1(root:string,arguments_:readonly string[],environment:NodeJS.ProcessEnv):Promise<object> {
  const {parseBuildInvocationV1,resolveBuildInvocationV1,nxBuildArgumentsV1,readBuildDependencyInvocationV1}=await import("../../🧭️invocation/🟦️.ts");
  const args=arguments_[0]==="repo"?arguments_.slice(1):arguments_,authored=environment.SEMIO_SCRIPT_CAPABILITIES===undefined?undefined:JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES),effective={...environment,...(environment.SEMIO_BUILD_MODE===undefined&&authored?.buildMode?{SEMIO_BUILD_MODE:authored.buildMode}:{})},selected=parseBuildInvocationV1(args,effective),resolved=resolveBuildInvocationV1(root,selected.args,effective);
  const {createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("../🟦️.ts");
  const grant=authored===undefined?createNativeBuildCallerCapabilitiesV1(root,selected.args,selected.tool,{...effective,...resolved.env}):readBuildAuthoringCapabilitiesV1(root,selected.args,selected.tool,authored);
  if(effective.SEMIO_BUILD_MODE!==undefined&&effective.SEMIO_BUILD_MODE!==grant.buildMode)throw Error("Original authored build mode changed");
  const {watchBuildChildArgumentsV1,maximumWatchBuildDescendantsV1}=await import("../../🧭️selection/👁️watch/🟦️.ts"),launches=[...grant.launches],pending=[selected.args],seen=new Set<string>();
  const add=(request:readonly string[])=>{if(!launches.some(owned=>JSON.stringify(owned)===JSON.stringify(request)))launches.push([...request]);};
  while(pending.length){
   const args=pending.shift()!,key=JSON.stringify(args);if(seen.has(key))continue;if(seen.size>=maximumWatchBuildDescendantsV1)throw Error("Bounded watch descendant declarations required");seen.add(key);
   const parsed=parseBuildInvocationV1(args,{...effective,SEMIO_BUILD_TOOL:selected.tool});if(parsed.tool!==selected.tool)throw Error("Watch callback cannot change the original build implementation");
   const invocation=resolveBuildInvocationV1(root,parsed.args,{...effective,SEMIO_BUILD_MODE:grant.buildMode}),command=invocation.args[0],tools=invocation.args[2]??selected.tool,main=command==="tools"?(tools!=="bazel"?["--version"]:undefined):command==="reset"?(selected.tool==="nx"?["reset"]:undefined):selected.tool==="nx"&&!["dependencies","cargo","watch"].includes(command??"")?nxBuildArgumentsV1(invocation.args):undefined;
   add(["repo",...parsed.args]);
   if(main){const{createNxBootstrapCapabilitiesV1}=await import("../../../⚡️caching/🚀️bootstrap/📥️caller/🟦️.ts"),nx=createNxBootstrapCapabilitiesV1(root,["nx",...main],{...effective,...invocation.env,SEMIO_BUILD_MODE:grant.buildMode});for(const launch of nx.launches??[])add(launch);}
   const boundary=invocation.args.indexOf("--"),callback=command==="watch"?watchBuildChildArgumentsV1(root,boundary<0?[]:invocation.args.slice(boundary+1)):undefined;
   if(callback)pending.push(callback);if(invocation.watch)pending.push(["run",invocation.watch,"--output=stream"]);
  }
  return readBuildAuthoringCapabilitiesV1(root,selected.args,selected.tool,{...grant,launches});
}

/** 📨️ Projects one declared neutral child while preserving the actual parent's Original bounds. */
export async function projectOwnedBuildChildEnvironmentV1(root:string,args:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,changes?:import("../../🧭️selection/👁️watch/🟦️.ts").BuildWatchChangesV1):Promise<NodeJS.ProcessEnv>{
 const labels=changes===undefined?{}:(await import("../../🧭️selection/👁️watch/🟦️.ts")).buildWatchChangesEnvironmentV1(changes);
 const{readOwnedBuildInvocationV1}=await import("../🟦️.ts"),{parseBuildInvocationV1}=await import("../../🧭️invocation/🟦️.ts"),{readIssuedScriptProcessEnvelopeV1,scriptProcessEnvironment}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),original=readIssuedScriptProcessEnvelopeV1(parent),caller=readOwnedBuildInvocationV1(root,parent,args),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:caller.tool}).args,request=["repo",...selected],declared={...caller,arguments:request,selection:request,launches:[request]},environment={...process.env,SEMIO_BUILD_TOOL:caller.tool,SEMIO_BUILD_MODE:caller.buildMode,CARGO_NET_OFFLINE:String(caller.native.network.offline),CARGO_TARGET_DIR:caller.native.artifactDirectory,SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(declared)},capabilities=await createBuildBootstrapCapabilitiesV1(root,request,environment);
 checkScriptInvocation(parent);if(JSON.stringify((capabilities as any).native)!==JSON.stringify(caller.native))throw Error("Original neutral child native resources changed");
 return scriptProcessEnvironment({...original,capabilities},{...environment,...labels,...(changes===undefined?{}:{SEMIO_BUILD_GRAPH_PATH:undefined,SEMIO_BUILD_GRAPH_DIGEST:undefined,SEMIO_BUILD_REUSE_GRAPH:undefined}),SEMIO_SCRIPT_CAPABILITIES:undefined,SEMIO_SCRIPT_POLICY:JSON.stringify(original.policy)});
}

/** 🧷️ Launches one predeclared neutral descendant without replacing its actual parent's process scope. */
export async function runOwnedBuildChildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,changes?:import("../../🧭️selection/👁️watch/🟦️.ts").BuildWatchChangesV1,control?:import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts").ResourceLeaseControl):Promise<void>{
 if(control){const{requireIssuedResourceControlInvocationV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");requireIssuedResourceControlInvocationV1(control,parent);}
 const child=await projectOwnedBuildChildEnvironmentV1(root,args,parent,changes);checkScriptInvocation(parent);if(control){const{requireIssuedResourceControlInvocationV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");requireIssuedResourceControlInvocationV1(control,parent);}await advanceScriptInvocation(parent,"build:neutral-child","running");await runOwnedCommand(process.execPath,["repo",...args],root,"build:neutral-child",scriptInvocationBudget(parent,0),{env:child,signal:control?.signal??parent.control.signal,onProgress:()=>advanceScriptInvocation(parent,"build:neutral-child","running")});checkScriptInvocation(parent);
}

