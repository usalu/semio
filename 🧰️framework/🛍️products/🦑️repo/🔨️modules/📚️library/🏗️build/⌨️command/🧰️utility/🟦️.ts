import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join, resolve, dirname } from "node:path";
import { COMMAND_RUNTIME_LIMITS_V1 } from "../⚙️runtime/🧬️schema/🟦️.ts";
import{runCapturedRepositoryKernelV1,type CapturedBuildCommandProofV1}from"../📥️entry/🟦️.ts";
import {profilePhaseV1} from "../../⏱️profile/🟨️.mjs";
import {runOwnedBuildInvocationV1} from "../🏃️invocation/🟦️.ts";
import {writeResultLineV1,machineOutputV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {answerKernelCheckV1} from "../📥️entry/✅️check/🟦️.ts";
async function runUtilityV1(root:string,args:readonly string[],parent:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,parent);checkResourceLeaseControl(phaseControl);const{parseBuildInvocationV1}=await import("../../🧭️invocation/🟦️.ts"),selected=parseBuildInvocationV1(args,{SEMIO_BUILD_TOOL:(parent.capabilities as {tool?:string}).tool});
 if(selected.args[0]==="incremental"){const{runIncrementalV1}=await import("../../📏️incremental/⌨️command/🟦️.ts");await runIncrementalV1(root,selected.args,parent);return;}
 if(selected.args[0]==="conformance"){const{runConformanceCommandV1}=await import("../../📐️conformance/⌨️command/🟦️.ts");await runConformanceCommandV1(root,selected.args,parent);return;}
 if(selected.args[0]==="concurrency"){const{runConcurrencyV1}=await import("../../📏️incremental/🤝️concurrent/⌨️command/🟦️.ts");await runConcurrencyV1(root,selected.args,parent);return;}
 if(selected.args[0]==="bench"){const{runBenchV1}=await import("../../📏️incremental/🏎️bench/⌨️command/🟦️.ts");await runBenchV1(root,selected.args,parent);return;}
 if(selected.args[0]==="service"){const{runHostServiceCommandV1}=await import("../🌐️host/🟦️.ts");process.exitCode=await runHostServiceCommandV1(root,selected.args.slice(1),parent,phaseControl);return;}
 if(selected.args[0]==="units"){const{runUnitsCliV1}=await import("../../🧩️units/⌨️command/🟦️.ts");await runUnitsCliV1(root,selected.args,parent);return;}
 throw Error("Utility command kernel required");
}
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{if(answerKernelCheckV1(args,proof))return;await runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,runUtilityV1);}
