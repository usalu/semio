/** 📥️ Execution: module exporting `runStdioOwnedCommandV1`. */
import{requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,type ScriptInvocation}from"@semio-tech/process/routing";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"@semio-tech/process/leases";
import{captureOwnedCommandV1,writeOwnedCommandOutputV1}from"@semio-tech/process/owned-execution";
import { readNativeCallerAuthorityV1, repositoryCargoPreparationStorageV1 } from "@semio-tech/repo-lib/artifacts";
import { withRepositoryCompilerStoreLaneV1, withRepositoryCompilerStoreV1, withCompilerWorkspaceSharedWriteV1 } from "@semio-tech/repo-lib/storage";
import { withRepositoryCargoInputsV1, repositoryCargoNetworkV1, repositoryCargoConsumptionV1, prepareCargoWorkspaceInvocation, cargoInvocationManifestV1, focusFromCargoArgumentsV1 } from "@semio-tech/repo-lib/cargo";
import{requireNativeStoragePathV1}from"@semio-tech/process/artifacts/native-build/storage/platform/contract";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{join}from"node:path";
import policy from"./🔣️policy.json";
import schema from"./🧬️schema/🔣️.json";
const output=Object.freeze({...policy});

/** 🗄️ Retains the actual caller phase and managed Cargo inputs through one bounded batch child. */
export async function runStdioOwnedCommandV1(invocation:ScriptInvocation,control:ResourceLeaseControl,command:string,args:readonly string[],cwd:string,environment:Readonly<Record<string,string|undefined>>):Promise<void>{
 requireIssuedResourceControlInvocationV1(control,invocation);checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,output).length)throw Error("Authored Stdio output policy required");
 const capturedArgs=Object.freeze([...args]),capturedEnvironment=Object.freeze({...environment}),authority=readNativeCallerAuthorityV1(invocation.capabilities),workspace=authority.repositoryRoot;
 await withRequestedIssuedResourceControlV1(control,0,async phase=>{
  if(command!=="cargo"){const tail=await captureOwnedCommandV1(command,capturedArgs,cwd,{control:phase,env:capturedEnvironment,output});if(tail)await writeOwnedCommandOutputV1(process.stdout,tail,phase);return;}
  await withRepositoryCompilerStoreV1(workspace,authority.original.buildMode,phase,capturedEnvironment,async(shared,sharedControl)=>{
   const preparation={...repositoryCargoPreparationStorageV1(workspace),directory:requireNativeStoragePathV1(shared.root,join(shared.buildDirectory,"preparation"))};
   await prepareCargoWorkspaceInvocation(preparation,workspace,capturedArgs,cwd,Object.freeze({...capturedEnvironment,...shared.environment}),sharedControl);
   await withRepositoryCompilerStoreLaneV1(shared,sharedControl,{crates:focusFromCargoArgumentsV1(capturedArgs),unitCache:["build","check","test","run","bench"].includes(capturedArgs[0]??"")},async(store,actualControl)=>{
    const prepared=Object.freeze({...capturedEnvironment,...store.environment});
    await withRepositoryCargoInputsV1({invocation,consumption:repositoryCargoConsumptionV1(capturedArgs,"artifact"),workspace,cwd,manifest:cargoInvocationManifestV1(capturedArgs,cwd),args:capturedArgs,network:repositoryCargoNetworkV1(invocation,workspace),control:actualControl,storage:store.storage,environment:prepared},async input=>{
     const selected=input.selections[0];if(input.selections.length!==1||!selected)throw Error("One exact Stdio Cargo selection required");await input.recheck();
     const tail=await withCompilerWorkspaceSharedWriteV1(store.storage,join(workspace,".cache/b/l/m/data"),input.control,()=>captureOwnedCommandV1(command,selected.arguments,cwd,{control:input.control,env:input.environment,output}));if(tail)await writeOwnedCommandOutputV1(process.stdout,tail,input.control);await input.recheck();
    });
   });
  }); });
}
