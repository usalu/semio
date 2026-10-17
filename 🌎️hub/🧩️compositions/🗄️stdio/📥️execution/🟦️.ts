/** 📥️ Execution: module exporting `runStdioOwnedCommandV1`. */
import{requireIssuedResourceControlInvocationV1,withRequestedIssuedResourceControlV1,type ScriptInvocation}from"../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"@semio-tech/process/leases";
import{captureOwnedCommandV1,writeOwnedCommandOutputV1}from"@semio-tech/process/owned-execution";
import{readNativeCallerAuthorityV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{withRepositoryCompilerStoreV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import{withCompilerWorkspaceWriteV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import{withRepositoryCargoInputsV1,repositoryCargoNetworkV1,repositoryCargoConsumptionV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import{prepareCargoWorkspaceInvocation,cargoInvocationManifestV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
import{repositoryCargoPreparationStorageV1}from"../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
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
  await withRepositoryCompilerStoreV1(workspace,authority.original.buildMode,phase,capturedEnvironment,async(store,actualControl)=>{
   const prepared=Object.freeze({...capturedEnvironment,...store.environment}),preparation={...repositoryCargoPreparationStorageV1(workspace),directory:requireNativeStoragePathV1(store.root,join(store.buildDirectory,"preparation"))};
   await prepareCargoWorkspaceInvocation(preparation,workspace,capturedArgs,cwd,prepared,actualControl);
   await withRepositoryCargoInputsV1({invocation,consumption:repositoryCargoConsumptionV1(capturedArgs,"artifact"),workspace,cwd,manifest:cargoInvocationManifestV1(capturedArgs,cwd),args:capturedArgs,network:repositoryCargoNetworkV1(invocation,workspace),control:actualControl,storage:store.storage,environment:prepared},async input=>{
    const selected=input.selections[0];if(input.selections.length!==1||!selected)throw Error("One exact Stdio Cargo selection required");await input.recheck();
    const tail=await withCompilerWorkspaceWriteV1(store.storage,join(workspace,".cache/b/l/m/data"),input.control,()=>captureOwnedCommandV1(command,selected.arguments,cwd,{control:input.control,env:input.environment,output}));if(tail)await writeOwnedCommandOutputV1(process.stdout,tail,input.control);await input.recheck();
   });
  });
 });
}
