import {buildBudgetMs} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import {createScriptProcessEnvelope,readScriptPolicy,withScriptProcessEnvelope} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {readBuildMode} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../📥️caller/🟦️.ts";
import {parseBuildInvocationV1} from "../../🧭️invocation/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {dashboardInvocation,launchDashboard} from "../../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts";
import {issueRepositoryScriptProvidersV1} from "../../../🏃️process/🔌️providers/🟦️.ts";

/** 🎛️ Opens the installed dashboard without admitting the build implementation, the dependency store or a graph. A missing installation compiles the dashboard crate in this process. */
export async function runDashboardDoorV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):Promise<number>{
 const native=dashboardInvocation(args);
 if(native===undefined)throw Error("Native dashboard door target required");
 issueRepositoryScriptProvidersV1(environment);
 const selected=parseBuildInvocationV1(args,environment),capabilities=createNativeBuildCallerCapabilitiesV1(root,selected.args,selected.tool,{...environment,SEMIO_BUILD_MODE:environment.SEMIO_BUILD_MODE??"prototype"});
 const policy=readScriptPolicy(Object.freeze({version:1,owner:"repository:build",maximumElapsedMilliseconds:buildBudgetMs(environment)})),envelope=createScriptProcessEnvelope(policy,capabilities,Date.now());
 return withScriptProcessEnvelope(envelope,async invocation=>{
  const mode=readBuildMode(capabilities.buildMode);
  return launchDashboard(root,native,{invocation,workspace:root,mode,network:readNativeCallerAuthorityV1(invocation.capabilities).network,control:scriptCompilerAdmissionControlV1(invocation,"dashboard:launch",0)});
 });
}
