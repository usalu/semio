import{runCapturedRepositoryKernelV1,type CapturedBuildCommandProofV1}from"../📥️entry/🟦️.ts";
import{readOwnedBuildInvocationV1}from"../../📥️caller/🟦️.ts";
import{parseBuildInvocationV1,buildInvocationRequestsHelpV1}from"../../🧭️invocation/🟦️.ts";
import{buildHelpForArgumentsV1}from"../../⚙️operations/📖️help/🟦️.ts";
import{runBuildPreflightV1}from"../../🩺️preflight/🟦️.ts";
/** 🩺️ Runs the existing host preflight through its admitted executable import closure. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{await runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,async(root,args,original)=>{const selected=parseBuildInvocationV1(args,process.env);if(buildInvocationRequestsHelpV1(selected.args)){console.log(buildHelpForArgumentsV1(selected.args,process.env));return;}readOwnedBuildInvocationV1(root,original,args);if(selected.args[0]!=="preflight")throw Error("Preflight command kernel required");await runBuildPreflightV1(root,selected.args,original);});}
