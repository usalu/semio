import {answerKernelCheckV1} from "../📥️entry/✅️check/🟦️.ts";
import{runCapturedRepositoryKernelV1,type CapturedBuildCommandProofV1}from"../📥️entry/🟦️.ts";
import{withRepositoryCommandEnvironmentV1,withRepositoryCommandDiagnosticsV1}from"../📥️context/🟦️.ts";
import{readOwnedBuildInvocationV1}from"../../📥️caller/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{readRepositoryCargoLaunchV1,launchRepositoryCargoV1}from"../../../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts";
/** 🦀️ Executes Cargo under its captured implementation and original command custody. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{if(answerKernelCheckV1(args,proof))return;await runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,async(root,args,parent,phaseControl)=>{readOwnedBuildInvocationV1(root,parent,args);await withRepositoryCommandDiagnosticsV1(root,parent,phaseControl,phase=>withRepositoryCommandEnvironmentV1(root,args,parent,phase,async({resolved,invocation})=>{if(resolved.args[0]!=="cargo")throw Error("Cargo command kernel required");process.exitCode=await launchRepositoryCargoV1(invocation,createIssuedResourceControlV1(invocation,"cargo:build-entry",0),root,readRepositoryCargoLaunchV1(resolved.args.slice(1),process.env),process.cwd(),process.env,{maximumMilliseconds:0,output:"stream"});}));});}
