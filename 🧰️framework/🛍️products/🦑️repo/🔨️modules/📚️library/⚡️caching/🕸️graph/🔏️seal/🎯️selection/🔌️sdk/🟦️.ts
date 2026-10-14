import {createRequire} from "node:module";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import type {FilesystemJournalControlV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📥️journal/🟦️.ts";
import {readBuildMode} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {SelectedGraphReusePortsV1} from "../📤️publisher/🟦️.ts";
import {SELECTED_GRAPH_HANDOFF_ENV} from "../📥️handoff/🟦️.ts";
import type {SelectedTaskSelectionV1} from "../📥️scope/🟦️.ts";

/** 🎯️ Only one explicit finite run can narrow its original task graph. */
export function selectedTaskSelectionV1(args:readonly string[],mode:"prototype"|"release"):SelectedTaskSelectionV1|null{
 if(args[0]!=="run"||!args[1]||args.some(argument=>/^--(?:graph|help)(?:=|$)/u.test(argument)))return null;
 const [project,target]=args[1].split(":");if(!project||!target||project==="workspace"||/[?*]/u.test(project+target))return null;
 return {project,target,configuration:mode,excludeTaskDependencies:args.includes("--excludeTaskDependencies")||args.includes("--excludeTaskDependencies=true")};
}

/** 🔌️ The actual pinned SDK and immutable owned bundle remain behind closed source ports. */
export async function selectedGraphSdkPortsV1(root:string,implementationFile:string,args:readonly string[],mode:"prototype"|"release",control:FilesystemJournalControlV1):Promise<SelectedGraphReusePortsV1>{
 const require=createRequire(implementationFile),{toolingIdentity,authorizers}=await require("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1(),{daemonClient}=require("nx/src/daemon/client/client"),native=require("nx/src/native"),selection=selectedTaskSelectionV1(args,mode),planner=authorizers.nativeSelectedTaskPlannerV1(implementationFile);
 return {toolingIdentity,environment:require("nx/src/daemon/client/daemon-environment").getDaemonEnv(),hashFile:path=>native.hashFile(path),scope:graph=>selection?authorizers.selectedTaskScopeV1(root,graph,selection,planner,control.signal):Promise.resolve(null),capture:async scope=>{control.signal.throwIfAborted();const response=await daemonClient.getSemioFilesystemJournalV1(scope,control),witness=authorizers.readFilesystemJournalRpcResponseV1(response,toolingIdentity);if(!witness)throw Error("Original live selected graph observer unavailable");return witness;}};
}

/** 🔐️ Every SDK phase receives the real incoming caller before rechecking the selected source handoff. */
export async function originalSelectedGraphHandoffHoldsV1(root:string,implementationFile:string):Promise<boolean>{
 if(process.env[SELECTED_GRAPH_HANDOFF_ENV]===undefined)return true;
 return receiveScriptProcessInvocation(process.env,async original=>{
  const requested=original.capabilities as {workspaceRoot?:unknown;arguments?:unknown;buildMode?:unknown};if(requested.workspaceRoot!==root||!Array.isArray(requested.arguments)||requested.arguments.some(value=>typeof value!=="string"))throw Error("Original selected graph caller refused");
  const control=scriptCompilerAdmissionControlV1(original,"nx:selected-source-custody",0),ports=await selectedGraphSdkPortsV1(root,implementationFile,requested.arguments,readBuildMode(requested.buildMode),control),{authorizers}=await createRequire(implementationFile)("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();
  return authorizers.selectedGraphHandoffHoldsV1(root,process.env.NX_WORKSPACE_DATA_DIRECTORY!,process.env[SELECTED_GRAPH_HANDOFF_ENV],ports,original.control.signal);
 });
}
