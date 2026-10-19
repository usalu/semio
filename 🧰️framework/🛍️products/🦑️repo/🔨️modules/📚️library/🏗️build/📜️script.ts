#!/usr/bin/env bun
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { join, resolve, dirname } from "node:path";
import {createBuildBootstrapCapabilitiesV1} from "./📥️caller/📤️child/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "../🏃️process/🔌️providers/🟦️.ts";
import { readScriptPolicy,Script, ScriptRouter, findWorkspaceRoot, scriptInvocationBudget, scriptInvocationEnvironment,checkScriptInvocation,advanceScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { readBuildMode } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { buildBudgetMs } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

import bazelTestSchema from "./🔧️tools/🏗️bazel/🧪️tests/🧬️schema/📋️dispatch.json";
import {compileJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const bazelTestContract=compileJsonSchemaSubset(bazelTestSchema);

/** 🧪️ Selects closed independently bounded Bazel contracts without replacing their Original. */
export function bazelTestCommandsV1(arguments_:readonly string[]):readonly Readonly<{phase:string|null;pattern:string;maximumMilliseconds:number}>[]{
 const phase=arguments_[0]??null;if(arguments_.length>1||bazelTestContract.validate({phase}).length)throw Error("Expected test bazel [declared runtime phase]");const policy=bazelTestSchema.definitions.Policy.const,phases=phase===null?[null,...bazelTestSchema.definitions.Phase.enum]:[phase];return phases.map(phase=>({phase,pattern:phase===null?policy.sourcePattern:policy.runtimePrefix+phase+policy.runtimeSuffix,maximumMilliseconds:policy.maximumChildMilliseconds}));
}

/** 🏗️ Executes workspace build leaves without importing repository audits. */
export class BuildScript extends Script {
  async run(segments: string[]): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE), slice = segments[0];
    if (!slice) {
      console.log(`[build] Completed the ${mode} workspace build graph.`);
      return;
    }
    if (slice === "canvas") {
      if (segments.length !== 2 || segments[1] !== "fonts") throw Error("Expected build canvas fonts");
      const cwd = join(this.repoRoot, "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust");
      const { repositoryProcessOwnerContextV1 } = await import("../🏃️process/📋️context/🔣️policy/🟦️.ts");
      const { repositoryCargoArtifactBuildPolicyV1 } = await import("../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts");
      const { prepareCargoWorkspaceInvocation } = await import("../🗂️workspaces/🦀️cargo/🟦️.ts");
      const {repositoryCargoPreparationStorageV1}=await import("../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts");
      const { devToolingEnv } = await import("../🏃️process/🌿️environment/🟦️.ts");
      await prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(this.repoRoot), this.repoRoot, ["build", "--manifest-path", join(cwd, "Cargo.toml")], this.repoRoot, scriptInvocationEnvironment(this.invocation,process.env),scriptCompilerAdmissionControlV1(this.invocation,"cargo:font-preparation",buildBudgetMs()));
      const env = devToolingEnv({ SEMIO_PROCESS_OWNER_CONTEXT: JSON.stringify(repositoryProcessOwnerContextV1(cwd)), SEMIO_CARGO_ARTIFACT_POLICY: JSON.stringify(repositoryCargoArtifactBuildPolicyV1(cwd, this.repoRoot)) });
      await runOwnedCommand(process.execPath, [join(cwd, "📜️script.ts"), "build"], cwd, "canvas-font-assets:build", scriptInvocationBudget(this.invocation, buildBudgetMs()), { env, signal: this.invocation.control.signal });
      return;
    }
    if (slice === "storybook") {
      if(segments.length!==1)throw Error("Storybook build accepts only its declared intent");const {buildRepositoryStorybookV1}=await import("./📖️storybook/🏃️execution/🟦️.ts");await buildRepositoryStorybookV1(this.repoRoot,mode,this.invocation);return;
    }
    const targets: Readonly<Record<string, string>> = { assets: "@semio-tech/assets:build", "repo-cli": "@semio-tech/repo-client:build", "repo-server": "@semio-tech/repo-coordinator:build", "repo-vscode": "@semio-tech/repo-vscode:build-vsix" };
    const target = targets[slice];
    if (!target) throw Error(`Unknown workspace build selection: ${slice}`);
    await runOwnedBuildInvocationV1(this.repoRoot, ["run", target, `--configuration=${mode}`, ...(segments.length > 1 ? ["--", ...segments.slice(1)] : [])], this.invocation,createIssuedResourceControlV1(this.invocation,"build:command",0));
  }
}

/** 🧪️ Runs the independently bundled workspace build ownership contract. */
class TestScript extends Script {
  async run(args: string[]): Promise<void> {
    if(args[0]==="bazel"){
      const original=this.invocation,root=this.repoRoot,environment=Object.freeze({...process.env}),initial=createIssuedResourceControlV1(original,"build:bazel",0),commands=bazelTestCommandsV1(args.slice(1)),{repoTestArtifactEnvironment}=await import("../🏃️process/🌿️environment/🧪️test-output/🟦️.ts"),source=resolve(import.meta.dir,"🔧️tools/🏗️bazel/🧪️tests/🟦️.ts");
      for(const command of commands){
        const label="build:bazel:"+(command.phase??"source-parity");await advanceScriptInvocation(original,label,"running");
        await withRequestedIssuedResourceControlV1(initial,command.maximumMilliseconds,async phase=>{const progress=async()=>{checkResourceLeaseControl(phase);await advanceScriptInvocation(original,label,"running");checkResourceLeaseControl(phase);};checkResourceLeaseControl(phase);await runOwnedCommand(process.execPath,["test",source,"--test-name-pattern",command.pattern],root,label,compilerAdmissionBudgetV1(phase),{env:scriptResourceProcessEnvironmentV1(phase,{...environment,...repoTestArtifactEnvironment(root,"build-bazel"),SEMIO_WORKSPACE_ROOT:root,SEMIO_BAZEL_INTEGRATION:"1",SEMIO_BAZEL_FIXTURE_PHASE:command.phase??undefined}),signal:phase.signal,onProgress:progress});checkResourceLeaseControl(phase);});
        await advanceScriptInvocation(original,label,"complete");
      }
      return;
    }
    if (args.length === 1 && ["bazel-bootstrap", "runtime-build-producer"].includes(args[0]!)) {
      const { repoTestArtifactEnvironment } = await import("../🏃️process/🌿️environment/🧪️test-output/🟦️.ts");
      const source = args[0] === "runtime-build-producer" ? resolve(import.meta.dir, "../🔍️discovery/🕸️runtime/🧪️tests/🏗️build-producer/🟦️.ts") : resolve(import.meta.dir, "../⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🧪️tests/🟦️.ts");
      const env = { ...process.env, ...repoTestArtifactEnvironment(this.repoRoot, `build-${args[0]}`) };
      await runOwnedCommand(process.execPath, ["test", source], this.repoRoot, `build:${args[0]}:contract`, scriptInvocationBudget(this.invocation, 300_000), { env, signal: this.invocation.control.signal });
      return;
    }
    if (args.length) throw Error("Workspace build tests accept no arguments");
    await runOwnedCommand(process.execPath, ["test", resolve(import.meta.dir, "🧪️tests/🟦️.ts"), resolve(import.meta.dir,"🧪️tests/🕸️graph/🟦️.ts"),resolve(import.meta.dir,"👁️observation/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"🩺️preflight/🧪️tests/🟦️.ts"),resolve(import.meta.dir,"../⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts")], this.repoRoot, "workspace-build:contract", scriptInvocationBudget(this.invocation, 300_000), { signal: this.invocation.control.signal });
  }
}


/** 🧭️ Routes the repository's portable build commands to the selected implementation. */
export class RepoScript extends Script {
  async run(args:string[]):Promise<void> { await runOwnedBuildInvocationV1(this.repoRoot,args,this.invocation,createIssuedResourceControlV1(this.invocation,"repository:command",0)); }
}

/** 🌳️ Preserves caller cancellation and deadline while selecting configured build resources. */
export async function runOwnedBuildInvocationV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {
 requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);const{readOwnedBuildInvocationV1}=await import("./📥️caller/🟦️.ts");readOwnedBuildInvocationV1(root,parent,args);
 const{parseBuildInvocationV1,buildInvocationRequestsHelpV1}=await import("./🧭️invocation/🟦️.ts"),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:(parent.capabilities as {tool?:string}).tool});
 if(buildInvocationRequestsHelpV1(selected.args)){console.log("bun repo [--tool=nx|bazel] run <project:target[:configuration]> | run-many --targets=<targets> [--projects=<projects>] | affected --targets=<targets> | exec --projects=<projects> -- <command> | graph | show projects|project <name>|targets | watch --projects=<projects> -- <command> | reset | tools setup [nx|bazel|all] | preflight --scope <owner> | dependencies sync|lock --all|--scope <owner> | cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo subcommand> <args>");return;}
 if(selected.args[0]==="preflight"){const{runBuildPreflightV1}=await import("./🩺️preflight/🟦️.ts");await runBuildPreflightV1(root,selected.args,parent);return;}
 const {openRepositoryCachePartitionV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),control=phaseControl,leases=await openRepositoryCachePartitionV1(root,"leases",control);let failed=false;try{const {withRepositoryCacheStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");await withRepositoryCacheStoreV1(root,"diagnostics",parent,control,(diagnostics,diagnosticPhase)=>withRepositoryCacheStoreV1(root,"dependencies",parent,diagnosticPhase,async(_dependencies,phaseControl)=>{const previous=process.env.SEMIO_DIAGNOSTIC_DIRECTORY;process.env.SEMIO_DIAGNOSTIC_DIRECTORY=diagnostics.environment.SEMIO_DIAGNOSTIC_DIRECTORY;try{await runOwnedBuildInLeasesV1(root,args,parent,phaseControl);}finally{if(previous===undefined)delete process.env.SEMIO_DIAGNOSTIC_DIRECTORY;else process.env.SEMIO_DIAGNOSTIC_DIRECTORY=previous;}}));}catch(error){failed=true;throw error;}finally{try{await leases.close();}catch(error){if(!failed)throw error;}}
}

async function runOwnedBuildInLeasesV1(root:string,args:readonly string[],parent:import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void> {
  const {parseBuildInvocationV1,resolveBuildInvocationV1,nxBuildArgumentsV1,readBuildDependencyInvocationV1,readBuildGraphAncestorSelectionV1,loadBuildInvocationGraphV1}=await import("./🧭️invocation/🟦️.ts");
  const {scriptInvocationEnvironment,checkScriptInvocation}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
  checkScriptInvocation(parent);
  const{readIssuedScriptProcessEnvelopeV1}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{readOwnedBuildInvocationV1}=await import("./📥️caller/🟦️.ts"),capabilities=readOwnedBuildInvocationV1(root,parent,args),environment={...process.env,SEMIO_BUILD_TOOL:capabilities.tool,SEMIO_BUILD_MODE:capabilities.buildMode},selected=parseBuildInvocationV1(args,environment),resolved=resolveBuildInvocationV1(root,selected.args,environment),original=capabilities,ancestor=readBuildGraphAncestorSelectionV1(root,original),invocation=parent;
  readIssuedScriptProcessEnvelopeV1(parent);
  const patch:NodeJS.ProcessEnv={...resolved.env,SEMIO_BUILD_MODE:capabilities.buildMode,CARGO_NET_OFFLINE:String(capabilities.native.network.offline),SEMIO_BUILD_TOOL:selected.tool,SEMIO_WORKSPACE_ROOT:root,SEMIO_BUILD_INVOCATION_PID:process.env.SEMIO_BUILD_INVOCATION_PID??String(process.pid),SEMIO_BUILD_REUSE_GRAPH:process.env.SEMIO_BUILD_REUSE_GRAPH,SEMIO_BUILD_GRAPH_PATH:process.env.SEMIO_BUILD_GRAPH_PATH,SEMIO_BUILD_GRAPH_DIGEST:process.env.SEMIO_BUILD_GRAPH_DIGEST};
  const encoded=scriptInvocationEnvironment(invocation,{...process.env,...patch});
  patch.SEMIO_SCRIPT_PROCESS_INVOCATION=encoded.SEMIO_SCRIPT_PROCESS_INVOCATION;
  const previous=Object.fromEntries(Object.keys(patch).map(key=>[key,process.env[key]]));
  for(const[key,value]of Object.entries(patch))if(value===undefined)delete process.env[key];else process.env[key]=value;
  try {
    if(resolved.args[0]==="run"&&resolved.args[1]?.startsWith("@semio-tech/repo-dashboard-rs:")){
      const native=await import("../../🎛️dashboard/📦️installation/🟦️.ts"),mode=readBuildMode((capabilities as {buildMode?:unknown}).buildMode??resolved.env.SEMIO_BUILD_MODE??"prototype");
      const nativeArguments=native.dashboardInvocation(resolved.args);
      if(nativeArguments&&native.dashboardInstalled(root,mode)){
        const {readNativeCallerAuthorityV1}=await import("../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts");const code=await native.launchDashboard(root,nativeArguments,{invocation,workspace:root,mode,network:readNativeCallerAuthorityV1(invocation.capabilities).network,control:scriptCompilerAdmissionControlV1(invocation,"dashboard:launch",0)});
        if(code!==0)throw Error(`Dashboard invocation failed (${code})`);
        return;
      }
    }
    const command=resolved.args[0];
    if(["run","run-many","affected","exec","watch"].includes(command)){
      const {validateBuildTaskArgumentsV1}=await import("./🧭️selection/🟦️.ts");
      validateBuildTaskArgumentsV1(resolved.args);
    }
    if(command==="dependencies"){
      const {mode,selection}=readBuildDependencyInvocationV1(resolved.args),{SyncScript,RefreshLockScript}=await import("../⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts");
      await new (mode==="lock"?RefreshLockScript:SyncScript)(root,root,invocation).run(selection.kind==="all"?["--all"]:["--scope",selection.owner]);return;
    }
    if(command==="cargo"){
      const {readRepositoryCargoLaunchV1,launchRepositoryCargoV1}=await import("../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts");
      process.exitCode=await launchRepositoryCargoV1(invocation,createIssuedResourceControlV1(invocation,"cargo:build-entry",0),root,readRepositoryCargoLaunchV1(resolved.args.slice(1),process.env),process.cwd(),process.env,{maximumMilliseconds:0,output:"stream"});return;
    }
    if(!["tools","reset"].includes(command)){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/📜️script.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}
    const {withRepositoryCacheStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");
    await withRepositoryCacheStoreV1(root,"dependencies",parent,phaseControl,async(_dependencies,phaseControl)=>{
    if(command==="tools"){
      if(resolved.args[1]!=="setup"||resolved.args.length>3)throw Error("Expected repo tools setup [nx|bazel|all]");
      const tools=resolved.args[2]??selected.tool;
      if(!["nx","bazel","all"].includes(tools))throw Error(`Unknown build tool: ${tools}`);
      if(tools!=="nx"){const adapter=await import("./🔧️tools/🏗️bazel/📜️script.ts");const {issueBuildTaskEnvironmentV1}=await import("./📥️caller/🟦️.ts"),{withRepositoryToolingStoreV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts");await withRepositoryToolingStoreV1(root,invocation,phaseControl,(store,actualPhase)=>adapter.prepareBazel(root,store,actualPhase,event=>invocation.control.publish({owner:invocation.policy.owner,command:`build:tools:${event.phase}`,stage:"running"}),issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],process.env).CARGO_NET_OFFLINE==="true"));}
      if(tools!=="bazel"){const adapter=await import("../⚡️caching/🚀️bootstrap/📜️script.ts");await adapter.runOwnedNxInvocationV1(root,["--version"],invocation,phaseControl);}
      return;
    }
    if(command==="reset"){
      if(resolved.args.length!==1)throw Error("Expected repo reset without task arguments");
      if(selected.tool==="bazel"){const{resetBazelBuildStateV1}=await import("./🔧️tools/🏗️bazel/💾️storage/🗑️reset/🟦️.ts");await resetBazelBuildStateV1(root,invocation,phaseControl);}
      else{const{invalidateBuildGraphV1}=await import("./🕸️graph/🟦️.ts");await invalidateBuildGraphV1(root,invocation,phaseControl);const {runOwnedNxInvocationV1}=await import("../⚡️caching/🚀️bootstrap/📜️script.ts");await runOwnedNxInvocationV1(root,["reset"],invocation,phaseControl);}
      return;
    }
    if(["graph","show"].includes(command)){
      const {loadBuildGraphV1}=await import("./🕸️graph/🟦️.ts"),graph=await loadBuildGraphV1(root,invocation,phaseControl,{selection:{kind:"all"},reuse:process.env.SEMIO_BUILD_REUSE_GRAPH==="true"});
      const {buildOptionV1}=await import("./🧭️selection/🟦️.ts");
      let result:unknown=graph;
      for(const row of graph.unavailable??[])console.error(`[build] unavailable project ${row.project??row.root}: ${row.source} does not parse (${row.sha256.slice(0,12)})`);
      if(command==="show"){
        const subject=resolved.args[1];
        if(subject==="project"){const name=resolved.args[2];if(!name)throw Error("Build project name required");result=graph.projects[name];const withheld=graph.unavailable?.find(row=>row.project===name);if(!result)throw Error(withheld?`Build project ${name} is unavailable until ${withheld.source} parses: ${withheld.reason}`:`Unknown build project: ${name}`);}
        else if(subject==="projects")result=Object.keys(graph.projects).sort();
        else if(subject==="targets")result=[...new Set(Object.values(graph.projects).flatMap(project=>Object.keys(project.targets)))].sort();
        else throw Error("Expected repo show projects|project <name>|targets");
      }
      const output=buildOptionV1(resolved.args,"file");
      if(output){const {withRepositoryBuildReportV1}=await import("./📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,output,result,invocation,async artifact=>{console.log(artifact.path);});}
      else if(Array.isArray(result)&&!resolved.args.includes("--json"))console.log(result.join("\n"));else console.log(JSON.stringify(result,null,2));
      return;
    }
    const {prepareBuildToolStorageV1}=await import("./🔧️tools/💾️storage/🟦️.ts");await prepareBuildToolStorageV1(root,selected.tool,invocation,phaseControl,0);
    const baseline=await loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH==="true"),{withSelectedRustBuildGraphV1}=await import("./🕸️graph/🦀️rust/🟦️.ts");await withSelectedRustBuildGraphV1(root,baseline,resolved.args,invocation,phaseControl,async(prepared,rustPhase)=>{const graph=prepared.graph;for(const[key,value]of Object.entries(prepared.environment)){previous[key]=process.env[key];process.env[key]=value;}
    const {withRepositoryBuildGraphArtifactV1}=await import("../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts");
    await withRepositoryBuildGraphArtifactV1(root,graph,invocation,rustPhase,async(pinned,artifactPhase)=>{
    process.env.SEMIO_BUILD_GRAPH_PATH=pinned.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pinned.digest;process.env.SEMIO_BUILD_REUSE_GRAPH="true";
    if(command==="watch"){const selection=await import("./🧭️selection/🟦️.ts"),{readOwnedBuildInvocationV1}=await import("./📥️caller/🟦️.ts"),{runOwnedBuildChildInvocationV1}=await import("./📥️caller/📤️child/🟦️.ts");await selection.watchBuildInvocationV1(root,graph,resolved.args,invocation,artifactPhase,{child:{readOwnedBuildInvocationV1,runOwnedBuildChildInvocationV1}});return;}
    if(selected.tool==="nx"){
      const {runOwnedNxInvocationV1,publishBootstrapSources}=await import("../⚡️caching/🚀️bootstrap/📜️script.ts");
      publishBootstrapSources(root);
      const prior=process.exitCode;process.exitCode=undefined;
      try{await runOwnedNxInvocationV1(root,nxBuildArgumentsV1(resolved.args),invocation,artifactPhase);if(process.exitCode)throw Error(`Build invocation failed (${process.exitCode})`);}finally{process.exitCode=prior;}
      return;
    }
    const selection=await import("./🧭️selection/🟦️.ts");
    const request=await selection.selectBuildTasksV1(root,graph,resolved.args,invocation);
    process.env.SEMIO_BUILD_REUSE_GRAPH="true";
    const adapter=await import("./🔧️tools/🏗️bazel/📜️script.ts");
    const planning=resolved.args.slice(0,resolved.args.indexOf("--")<0?undefined:resolved.args.indexOf("--")).some(argument=>argument==="--dry-run"||argument==="--dry-run=true"||argument==="--graph"||argument.startsWith("--graph="));
    if(planning){
      const plan=adapter.planBazelTasksV1(graph,request),output=selection.buildOptionV1(resolved.args,"graph");
      if(output&&output!=="stdout"&&output!=="true"){
        const {withRepositoryBuildReportV1}=await import("./📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,output,plan,invocation,async artifact=>{console.log(artifact.path);});
      }else console.log(JSON.stringify(plan,null,2));
      return;
    }
    if(resolved.watch){
      const controller=new AbortController(),{withRequestedIssuedResourceControlV1}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{readOwnedBuildInvocationV1}=await import("./📥️caller/🟦️.ts"),{runOwnedBuildChildInvocationV1}=await import("./📥️caller/📤️child/🟦️.ts");
      await withRequestedIssuedResourceControlV1(artifactPhase,0,async childPhase=>{let watching:Promise<void>|undefined;
      try{
        await adapter.runBazelTasksV1(root,graph,request,invocation,childPhase,()=>{watching=selection.watchBuildInvocationV1(root,graph,["watch","--all","--include-global-files","--",process.execPath,join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"),"repo","run",resolved.watch!,"--output=stream"],invocation,childPhase,{initial:true,child:{readOwnedBuildInvocationV1,runOwnedBuildChildInvocationV1}});watching.catch(error=>controller.abort(error));});
      }finally{controller.abort();await watching?.catch(error=>{if(!controller.signal.aborted)throw error;});}},controller.signal);
      return;
    }
    await adapter.runBazelTasksV1(root,graph,request,invocation,artifactPhase);
    },additionalBytes=>prepareBuildToolStorageV1(root,selected.tool,invocation,rustPhase,additionalBytes));
    });
    });
  } finally {for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;}
}

/** 🏛️ Returns the single authored ordinary policy used before a fresh build CLI birth. */
export function readDefaultBuildCliPolicyV1(){return readScriptPolicy(Object.freeze({version:1,owner:"repository:build",maximumElapsedMilliseconds:0}));}

/** 🚪️ Admits the original authored CLI policy and resources before any backend or installed application launch. */
export async function runBuildCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void> {
  issueRepositoryScriptProvidersV1(environment);
  const {createScriptProcessEnvelope,readScriptProcessEnvelope,readScriptPolicy,withScriptProcessEnvelope,SCRIPT_PROCESS_INVOCATION_ENV}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
  const wire=environment[SCRIPT_PROCESS_INVOCATION_ENV];
  const incoming=wire===undefined?undefined:readScriptProcessEnvelope(JSON.parse(wire),Date.now());
  const policy=environment.SEMIO_SCRIPT_POLICY===undefined?incoming?.policy??readDefaultBuildCliPolicyV1():readScriptPolicy(JSON.parse(environment.SEMIO_SCRIPT_POLICY));
  if(incoming&&(incoming.policy.owner!==policy.owner||incoming.policy.version!==policy.version||incoming.policy.maximumElapsedMilliseconds!==policy.maximumElapsedMilliseconds))throw Error("Original build policy changed across handoff");
  const capabilities=incoming?.capabilities??await createBuildBootstrapCapabilitiesV1(root,args,environment),envelope=incoming??createScriptProcessEnvelope(policy,capabilities,Date.now());
  await withScriptProcessEnvelope(envelope,original=>runScriptMain(new ScriptRouter(root,root).register("repo",RepoScript).register("build",BuildScript).register("test",TestScript),{invocation:original,argv:[...args]}));
}

if(import.meta.main)await runBuildCliV1(findWorkspaceRoot(import.meta.dir),process.argv.slice(2));
