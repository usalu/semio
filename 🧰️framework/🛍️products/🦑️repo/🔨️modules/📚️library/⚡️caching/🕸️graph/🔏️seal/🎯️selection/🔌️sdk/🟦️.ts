import {readFilesystemJournalScopeV1,type FilesystemJournalScopeV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";
import {createRequire} from "node:module";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNxGraphNativeControlV1,withNxGraphNativeTaskV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
const sdkReadPolicy=Object.freeze({...schema.$defs.SdkReadPolicyV1.const});
import {readBuildMode} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {SelectedGraphReusePortsV1} from "../📤️publisher/🟦️.ts";
import {SELECTED_GRAPH_HANDOFF_ENV} from "../📥️handoff/🟦️.ts";
import type {SelectedGraphScopeV1} from "../🟦️.ts";
import type {SelectedTaskSelectionV1} from "../📥️scope/🟦️.ts";
import{captureSelectedTaskDataV1}from"../📥️scope/📋️data/🟦️.ts";

/** 🎯️ Only one explicit finite run can narrow its original task graph. */
export function selectedTaskSelectionV1(args:readonly string[],mode:"prototype"|"release"):SelectedTaskSelectionV1|null{
 if(args[0]!=="run"||!args[1]||args.some(argument=>/^--(?:graph|help)(?:=|$)/u.test(argument)))return null;
 const [project,target]=args[1].split(":");if(!project||!target||project==="workspace"||/[?*]/u.test(project+target))return null;
 return {project,target,configuration:mode,excludeTaskDependencies:args.includes("--excludeTaskDependencies")||args.includes("--excludeTaskDependencies=true")};
}

/** 📨️ Captures only journal path DATA before asynchronous SDK task admission. */
export function selectedGraphJournalScopeV1(scope:FilesystemJournalScopeV1):FilesystemJournalScopeV1{return readFilesystemJournalScopeV1({roots:scope.roots,files:scope.files});}

/** 🔌️ The actual pinned SDK and immutable owned bundle remain behind closed source ports. */
export async function selectedGraphSdkPortsV1(root:string,implementationFile:string,args:readonly string[],mode:"prototype"|"release",control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Promise<SelectedGraphReusePortsV1>{
 environment=Object.freeze({...environment});args=Object.freeze([...args]);requireNxGraphNativeControlV1(root,environment,control);let pending=0;
 const task=<T>(run:(actual:ResourceLeaseControl)=>Promise<T>):Promise<T>=>{requireNxGraphNativeControlV1(root,environment,control);if(pending>=sdkReadPolicy.maximumPendingOperations)throw Error("Bounded selected SDK task admission required");pending++;try{return withNxGraphNativeTaskV1(root,environment,control,run).finally(()=>{pending--;});}catch(error){pending--;throw error;}},require=createRequire(implementationFile),{toolingIdentity,authorizers}=await task(async()=>require("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1()),{daemonClient}=require("nx/src/daemon/client/client"),native=require("nx/src/native"),selection=selectedTaskSelectionV1(args,mode),planner=authorizers.nativeSelectedTaskPlannerV1(implementationFile);
 requireNxGraphNativeControlV1(root,environment,control);return Object.freeze({toolingIdentity,environment:require("nx/src/daemon/client/daemon-environment").getDaemonEnv(environment),hashFile:(path:string)=>{requireNxGraphNativeControlV1(root,environment,control);const value=native.hashFile(path);requireNxGraphNativeControlV1(root,environment,control);return value;},scope:(graph:unknown)=>{requireNxGraphNativeControlV1(root,environment,control);const captured=selection?captureSelectedTaskDataV1(graph):null;return task<SelectedGraphScopeV1|null>(actual=>selection?authorizers.selectedTaskScopeV1(root,captured,selection,planner,actual):Promise.resolve(null));},capture:(scope:Parameters<SelectedGraphReusePortsV1["capture"]>[0])=>{requireNxGraphNativeControlV1(root,environment,control);const requested=selectedGraphJournalScopeV1(scope);return task(async actual=>{const response=await daemonClient.getSemioFilesystemJournalV1(requested,actual),witness=authorizers.readFilesystemJournalRpcResponseV1(response,toolingIdentity);if(!witness)throw Error("Original live selected graph observer unavailable");return witness;});}});
}

/** 🔐️ Every SDK phase receives the real incoming caller before rechecking the selected source handoff. */
export async function originalSelectedGraphHandoffHoldsV1(root:string,implementationFile:string):Promise<boolean>{
 if(process.env[SELECTED_GRAPH_HANDOFF_ENV]===undefined)return true;
 return receiveScriptProcessInvocation(process.env,async original=>{
  const requested=original.capabilities as {workspaceRoot?:unknown;arguments?:unknown;buildMode?:unknown};if(requested.workspaceRoot!==root||!Array.isArray(requested.arguments)||requested.arguments.some(value=>typeof value!=="string"))throw Error("Original selected graph caller refused");
  const control=scriptCompilerAdmissionControlV1(original,"nx:selected-source-custody",0),ports=await selectedGraphSdkPortsV1(root,implementationFile,requested.arguments,readBuildMode(requested.buildMode),control,process.env),{authorizers}=await createRequire(implementationFile)("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();
  return authorizers.selectedGraphHandoffHoldsV1(root,process.env.NX_WORKSPACE_DATA_DIRECTORY!,process.env[SELECTED_GRAPH_HANDOFF_ENV],ports,original.control.signal);
 });
}
