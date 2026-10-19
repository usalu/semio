import {readNativeCallerAuthorityV1} from "../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {join,resolve,basename,relative} from "node:path";
import {realpath} from "node:fs/promises";
import {createRequire} from "node:module";
import {requireIssuedResourceControlInvocationV1,scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "@semio-tech/process/leases";
import {withRepositoryNxStorageV1} from "../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import {withNxToolingUseV1,nxToolingUseStorageV1} from "@semio-tech/nx-bootstrap-tools/use";
import {withRepositoryCacheStoreV1} from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {withArtifactScratchV1,withArtifactDestinationTaskV1} from "../../📦️artifacts/💾️storage/🟦️.ts";
/** 🧷️ Holds the actual product sink and immutable toolkit namespace through every fixture child. */
export async function withInstalledNxFixtureV1<T>(workspace:string,selection:Readonly<{kind:"external";root:string}|{kind:"products"}>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(root:string,environment:Readonly<Record<string,string|undefined>>,output:string,actualPhase:ResourceLeaseControl,toolkitRoot:string)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 const toolkitRoot=await realpath(join(workspace,".nx/installation"));checkResourceLeaseControl(phaseControl);const identity=basename(toolkitRoot);
 return withRepositoryCacheStoreV1(workspace,"products",invocation,phaseControl,(store,productPhase)=>withArtifactScratchV1({workspace,invocation,storage:store.storage,control:store.control,phaseControl:productPhase,request:{version:1,mode:readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode,owner:relative(workspace,import.meta.dir).replaceAll("\\","/")}},destination=>withArtifactDestinationTaskV1(destination,invocation,sink=>withRepositoryNxStorageV1(workspace,invocation,sink.control,(bind,namespacePhase)=>withNxToolingUseV1(nxToolingUseStorageV1(workspace),{identity,toolkitRoot,invocation},namespacePhase,async(_witness,actualPhase)=>{
  const root=selection.kind==="external"?selection.root:sink.directory;if(resolve(root)!==root||root.length>256)throw Error("Canonical bounded Nx fixture workspace required");
  const implementation=createRequire(join(workspace,"package.json"))(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).nxBuildDependencyImplementationV1(),binding=await bind(identity,null,implementation,root);checkResourceLeaseControl(actualPhase);
  const environment=scriptInvocationEnvironment(invocation,{...binding.environment,SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:identity,NX_WORKSPACE_ROOT_PATH:root,SEMIO_BUILD_GRAPH_PATH:undefined,SEMIO_BUILD_GRAPH_DIGEST:undefined,SEMIO_BUILD_REUSE_GRAPH:undefined});
  const output=join(sink.directory,"out");if(output.length>256)throw Error("Bounded custody product output required");const value=await body(root,environment,output,actualPhase,toolkitRoot);checkResourceLeaseControl(actualPhase);return value;
 })))));
}

