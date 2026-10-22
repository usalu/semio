import{runCapturedRepositoryKernelV1,type CapturedBuildCommandProofV1}from"../📥️entry/🟦️.ts";
import{withRepositoryCommandEnvironmentV1,withRepositoryCommandDiagnosticsV1}from"../📥️context/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{advanceScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
/** 💾️ Executes cache operations through their own captured domain without admitting unrelated command drivers. */
export async function runCapturedBuildSourceV1(args:readonly string[],proof:CapturedBuildCommandProofV1):Promise<void>{await runCapturedRepositoryKernelV1(args,proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity,async(root,args,parent,phaseControl)=>withRepositoryCommandDiagnosticsV1(root,parent,phaseControl,phaseControl=>withRepositoryCommandEnvironmentV1(root,args,parent,phaseControl,async({selected,resolved,invocation})=>{const command=resolved.args[0];if(command!=="remote-cache"&&(command!=="cache"||(["status","gc","verify"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks"))))throw Error("Cache command kernel required");const{buildOperationRequiresV1}=await import("../../⚙️operations/🟦️.ts");if(buildOperationRequiresV1(command)==="dependencies"){const{ensureBuildEnvironmentV1}=await import("../../🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}

    if(command==="remote-cache"){
      const {runRemoteCacheCommandV1}=await import("../../🌐️remote-cache/⌨️command/🟦️.ts");
      const tools=async()=>await Promise.all((await import("../../🔌️backends/🟦️.ts")).buildBackendIdsV1().map(async id=>(await(await import("../../🔌️backends/🟦️.ts")).loadBuildBackendV1(id)).remoteCache()));
      const outcome=await runRemoteCacheCommandV1(resolved.args.slice(1),{root,environment:process.env,signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:"+stage,"running");},write:writeResultLineV1,tools});
      if(outcome.result!==undefined)console.log(JSON.stringify(outcome.result,null,2));
      if(outcome.exitCode!==0)process.exitCode=outcome.exitCode;
      return;
    }
const{withRepositoryCacheStoreV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts");await withRepositoryCacheStoreV1(root,"dependencies",parent,phaseControl,async(_dependencies,phaseControl)=>{

      const {runBuildCacheOperationV1}=await import("../../⚙️operations/🟦️.ts");
      const resolveCacheTool=async(tool:string)=>(await(await import("../../🔌️backends/🟦️.ts")).loadBuildBackendV1(tool)).cache();
      console.log(JSON.stringify(await runBuildCacheOperationV1(selected.tool,resolved.args.slice(1),{root,invocation,phaseControl},resolveCacheTool),null,2));
      return;
    
});})));}
