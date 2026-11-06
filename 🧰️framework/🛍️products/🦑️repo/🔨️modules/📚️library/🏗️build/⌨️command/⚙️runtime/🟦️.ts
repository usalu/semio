import{maintainTaskStoresV1,runTaskGraphV1,ensureTaskDependenciesV1}from"../🏃️task/🤝️shared/🟦️.ts";
import {answerKernelCheckV1} from "../📥️entry/✅️check/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { join, resolve, dirname } from "node:path";
import { COMMAND_RUNTIME_LIMITS_V1 } from "./🧬️schema/🟦️.ts";
import { readScriptPolicy,Script, ScriptRouter, findWorkspaceRoot, scriptInvocationBudget, scriptInvocationEnvironment,checkScriptInvocation,advanceScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import{withRepositoryCommandEnvironmentV1,withRepositoryCommandDiagnosticsV1}from"../📥️context/🟦️.ts";
import{loadBuildInvocationGraphV1,loadBuildGraphInChildV1,buildGraphPlanInChildV1,writeBuildGraphPlanV1,BUILD_GRAPH_PLAN_OUT_V1}from"../../🧭️invocation/🕸️graph/🟦️.ts";
import{runCapturedRepositoryKernelV1,type CapturedBuildCommandProofV1}from"../📥️entry/🟦️.ts";
import {profileMarkV1} from "../../⏱️profile/🟨️.mjs";
import {markBuildCapacityPhaseV1} from "../../🌡️host-capacity/🚏️marks/🟦️.ts";
import {profilePhaseV1} from "../../⏱️profile/🟨️.mjs";
import { runtimeSleepV1, runtimeSpawnV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";
import {runOwnedBuildInvocationV1} from "../🏃️invocation/🟦️.ts";
import {writeResultLineV1,machineOutputV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
async function runAdmittedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {
 requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);const{readOwnedBuildInvocationV1}=await profilePhaseV1("invocation.load",()=>import("../../📥️caller/🟦️.ts"));profilePhaseV1("invocation.read",()=>readOwnedBuildInvocationV1(root,parent,args));
 const{parseBuildInvocationV1,resolveBuildInvocationV1,buildInvocationRequestsHelpV1}=await profilePhaseV1("invocation.parse-load",()=>import("../../🧭️invocation/🟦️.ts")),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:(parent.capabilities as {tool?:string}).tool});
 if(["preflight","dependencies","cache"].includes(selected.args[0]??""))throw Error("Runtime command kernel required");
 const dashboard=selected.args[0]==="run"&&selected.args[1]?.startsWith("@semio-tech/repo-dashboard-rs:")?(await import("../../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts")).dashboardInvocation(selected.args):undefined;
 if(buildInvocationRequestsHelpV1(selected.args)&&dashboard===undefined){const{buildHelpForArgumentsV1}=await import("../../⚙️operations/📖️help/🟦️.ts");writeResultLineV1(buildHelpForArgumentsV1(selected.args,process.env));return;}
 const{readNativeRepoRecoveryPlanV1}=await import("../../🚀️bootstrap/⌨️cli/🟦️.ts");
 if(profilePhaseV1("invocation.recovery-plan",()=>readNativeRepoRecoveryPlanV1(root,resolveBuildInvocationV1(root,selected.args,{...process.env,SEMIO_BUILD_MODE:(parent.capabilities as {buildMode:string}).buildMode}).args))){await runOwnedBuildInLeasesV1(root,args,parent,phaseControl);return;}
 await withRepositoryCommandDiagnosticsV1(root,parent,phaseControl,diagnosticPhase=>runOwnedBuildInLeasesV1(root,args,parent,diagnosticPhase));
}
async function runOwnedBuildInLeasesV1(root:string,args:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{await withRepositoryCommandEnvironmentV1(root,args,parent,phaseControl,async({capabilities,selected,resolved,original,ancestor,invocation,previous})=>{
    if(await profilePhaseV1("recovery.run",async()=>(await import("../../🚀️bootstrap/⌨️cli/🟦️.ts")).runNativeRepoRecoveryV1(root,resolved.args,invocation,phaseControl,(await import("../../🚀️bootstrap/⌨️cli/🔌️ports/🟦️.ts")).nativeRepoRecoveryPortsV1)))return;
    if(resolved.args[0]==="run"&&resolved.args[1]?.startsWith("@semio-tech/repo-dashboard-rs:")){
      const native=await import("../../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"),mode=readBuildMode((capabilities as {buildMode?:unknown}).buildMode??resolved.env.SEMIO_BUILD_MODE??"prototype");
      const nativeArguments=native.dashboardInvocation(resolved.args);
      if(nativeArguments!==undefined){
        const {readNativeCallerAuthorityV1}=await import("../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts");const code=await native.launchDashboard(root,nativeArguments,{invocation,workspace:root,mode,network:readNativeCallerAuthorityV1(invocation.capabilities).network,control:scriptCompilerAdmissionControlV1(invocation,"dashboard:launch",0)});
        if(code!==0)throw new Error(`Dashboard invocation failed (${code})`);
        return;
      }
    }
    const command=resolved.args[0];if(command===undefined)throw Error("Build command required");
    if(["run","run-many","affected","exec","watch"].includes(command)){
      const {validateBuildTaskArgumentsV1}=await import("../../🧭️selection/🟦️.ts");
      profilePhaseV1("selection.validate",()=>validateBuildTaskArgumentsV1(resolved.args));
    }
    if(command==="cargo"){
      const {readRepositoryCargoLaunchV1,launchRepositoryCargoV1}=await import("../../../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts");
      process.exitCode=await launchRepositoryCargoV1(invocation,createIssuedResourceControlV1(invocation,"cargo:build-entry",0),root,readRepositoryCargoLaunchV1(resolved.args.slice(1),process.env),process.cwd(),process.env,{maximumMilliseconds:0,output:"stream"});return;
    }
    await ensureTaskDependenciesV1(root,phaseControl,{capabilities,selected,resolved,original,ancestor,invocation,previous});
    if(command==="remote-execution"){
      const {runRemoteExecutionCommandV1}=await import("../../🌐️remote-execution/⌨️command/🟦️.ts");
      const entry=join(import.meta.dirname,"🌐️remote-execution","🏃️entry","🟦️.ts");
      const spawnWorker=(args:readonly string[],environment:Readonly<Record<string,string>>)=>{
        const child=runtimeSpawnV1([process.execPath,entry,...args],{cwd:root,env:{...process.env,...environment},stdin:"ignore",stdout:"inherit",stderr:"inherit"});
        return {exited:child.exited,stop:async()=>{child.kill("SIGTERM");await Promise.race([child.exited,runtimeSleepV1(COMMAND_RUNTIME_LIMITS_V1.serverStopGraceMilliseconds).then(()=>{child.kill("SIGKILL");})]);}};
      };
      const outcome=await runRemoteExecutionCommandV1(resolved.args.slice(1),{root,environment:process.env,signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:"+stage,"running");},write:writeResultLineV1,spawnWorker});
      if(outcome.result!==undefined)writeResultLineV1(JSON.stringify(outcome.result,null,2));
      if(outcome.exitCode!==0)process.exitCode=outcome.exitCode;
      return;
    }
    if(command==="tools"&&resolved.args[1]==="verify"){const{runToolsVerifyV1}=await import("../../🧰️tool-setup/⌨️command/🟦️.ts");process.exitCode=await runToolsVerifyV1(root,phaseControl,resolved.args.slice(2));return;}
    if(command==="tools"&&resolved.args.length===3&&resolved.args[1]==="setup"&&resolved.args[2]==="all")await(await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🟦️.ts")).setupRepositoryDependencyPublicationV1(invocation,phaseControl);
    const {withRepositoryCacheStoreV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
    profileMarkV1("dependencies.opening");await withRepositoryCacheStoreV1(root,"dependencies",parent,phaseControl,async(_dependencies,phaseControl)=>{profileMarkV1("dependencies.opened");
    if(command==="tools"){
      const {buildBackendIdsV1,buildBackendAlternationV1,loadBuildBackendV1,requireBuildBackendIdV1}=await import("../../🔌️backends/🟦️.ts");
      if(resolved.args[1]!=="setup"||(resolved.args.length>3&&resolved.args[2]!=="toolchains"))throw new Error(`Expected repo tools setup [${buildBackendAlternationV1()}|toolchains [tool...]|all] or repo tools verify [tool...]`);
      const tools=resolved.args[2]??selected.tool;
      if(!["toolchains","all"].includes(tools))requireBuildBackendIdV1(tools);
      if(tools==="toolchains"||tools==="all"){const{runToolchainsSetupV1,parseToolSetupSelectionV1}=await import("../../🧰️tool-setup/⌨️command/🟦️.ts"),names=tools==="toolchains"?parseToolSetupSelectionV1(resolved.args):[],{issueBuildTaskEnvironmentV1:issueToolsEnvironmentV1}=await import("../../📥️caller/🟦️.ts"),reports=await runToolchainsSetupV1(root,invocation,phaseControl,names,["bazel"],issueToolsEnvironmentV1(root,invocation,{kind:"noop"},[],process.env).CARGO_NET_OFFLINE==="true");if(reports.some(report=>report.status!=="ok"&&report.status!=="deferred"))process.exitCode=1;}
      for(const id of tools==="toolchains"?[]:tools==="all"?buildBackendIdsV1():[tools])await(await loadBuildBackendV1(id)).setup({root,invocation,phase:phaseControl,args:resolved.args,tool:id});
      return;
    }
    if(command==="reset"){
      if(resolved.args.length!==1)throw new Error("Expected repo reset without task arguments");
      await(await(await import("../../🔌️backends/🟦️.ts")).loadBuildBackendV1(selected.tool)).reset({root,invocation,phase:phaseControl,args:resolved.args,tool:selected.tool});
      return;
    }
    await maintainTaskStoresV1(root,invocation,phaseControl);
    if(["graph","show"].includes(command)){
      await markBuildCapacityPhaseV1("plan");
      const {loadBuildGraphV1}=await import("../../🕸️graph/🟦️.ts"),graph=await loadBuildGraphV1(root,invocation,phaseControl,{selection:{kind:"all"},reuse:process.env.SEMIO_BUILD_REUSE_GRAPH==="true",graph:{policy:selected.graphPolicy??"fresh",...(command==="show"&&resolved.args[1]==="project"&&resolved.args[2]?{closure:[resolved.args[2]]}:{}),...(command==="show"&&(resolved.args[1]==="projects"||resolved.args[1]==="targets")?{lean:true}:{})}});
      const {buildOptionV1}=await import("../../🧭️selection/🟦️.ts");
      let result:unknown=graph;
      const describeWithheld=async()=>{const{describeUnavailableProjectV1}=await import("../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs"),{resolveLanguageV1}=await import("../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"),{CATALOG_LANGUAGES_V1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts"),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language;return(row:Parameters<typeof describeUnavailableProjectV1>[0])=>describeUnavailableProjectV1(row,language);};
      if(graph.unavailable?.length){const describe=await describeWithheld();for(const row of graph.unavailable)console.error(describe(row));}
      if(command==="show"){
        const subject=resolved.args[1];
        if(subject==="project"){const name=resolved.args[2];if(!name)throw new Error("Build project name required");result=graph.projects[name];const withheld=graph.unavailable?.find(row=>row.project===name);if(!result)throw new Error(withheld?(await describeWithheld())(withheld):`Unknown build project: ${name}`);}
        else if(subject==="projects")result=Object.keys(graph.projects).sort();
        else if(subject==="targets")result=[...new Set(Object.values(graph.projects).flatMap(project=>Object.keys(project.targets)))].sort();
        else throw new Error("Expected repo show projects|project <name>|targets");
      }
      const output=buildOptionV1(resolved.args,"file");
      if(output){const {withRepositoryBuildReportV1}=await import("../../📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,output,result,invocation,async artifact=>{writeResultLineV1(artifact.path);});}
      else if(Array.isArray(result)&&!resolved.args.includes("--json"))writeResultLineV1(result.join("\n"));else{const {writeJsonStreamV1}=await import("../../🕸️graph/🕰️generations/🌊️json-stream/🟨️.mjs");await writeJsonStreamV1(result,text=>new Promise<void>((resolve,reject)=>{machineOutputV1.write(text,error=>error?reject(error):resolve());}),{signal:invocation.control.signal});machineOutputV1.write("\n");}
      return;
    }
    await runTaskGraphV1(root,args,parent,phaseControl,{capabilities,selected,resolved,original,ancestor,invocation,previous});

    });});}
/** ⚙️ Executes the existing graph and tool command body inside its captured runtime kernel. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{if(answerKernelCheckV1(args,proof))return;await runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,runAdmittedBuildInvocationV1);}
