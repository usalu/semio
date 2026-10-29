import {repositoryCargoNetworkV1,withRepositoryCargoInputsV1} from "../📥️dependencies/🟦️.ts";
import {existsSync} from "node:fs";
import {withNativeRecheckWindowV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🟦️.ts";
import {withCompilerCapacityConsumptionV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts";
import {buildSchedulerV1,schedulerCargoSelectionV1,readSchedulerExecutablesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import {readNativeStorageReservationPolicyV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {checkScriptInvocation,createRequestedIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,scriptInvocationEnvironment,withIssuedResourceOperationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readBuildMode} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {ensureUnitCacheShimV1,unitCacheRootV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {withCompilerWorkspaceSharedWriteV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {withRepositoryCompilerStoreV1,type RepositoryCompilerStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {refuseProcessV1} from "../../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";

/** 🧰️ The two native helpers every Cargo build of the repository starts with, as they now exist on disk. */
export type RepositoryCargoHelpersV1=Readonly<{scheduler:string;wrapper:string}>;

/** 🔥️ Builds ahead of the first compile what `repo cargo` otherwise builds on demand, inside the same issued compiler store, capacity grant and storage custody: the memory-admitting rustc scheduler of the store (one build of its crate under the issued mode, kept by a receipt) and the compiler wrapper of the unit cache (a `/bin/sh` stand-in on POSIX, the digest-named native executable on Windows). Both are skipped when they already exist, so a second call costs a store open. The caller passes the `repo tools setup` invocation and its issued control; nothing here starts a nested `repo`.
 * @see ../🚪️launcher/🟦️.ts
 * @see ../../../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts */
export function prepareRepositoryCargoHelpersV1(invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,workspace:string,configuration:string,environment:Readonly<Record<string,string|undefined>>,maximumMilliseconds:number):Promise<RepositoryCargoHelpersV1>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);
 return withIssuedResourceOperationV1(phaseControl,()=>withNativeRecheckWindowV1(async()=>{
  checkScriptInvocation(invocation);
  const mode=readBuildMode(configuration),control=createRequestedIssuedResourceControlV1(phaseControl,maximumMilliseconds);
  return withRepositoryCompilerStoreV1(workspace,mode,control,scriptInvocationEnvironment(invocation,{...environment,SEMIO_BUILD_MODE:mode}),async(store,control)=>{
   const wrapper=ensureUnitCacheShimV1(unitCacheRootV1(store.root));
   const scheduler=await prepareRepositoryCargoSchedulerV1(invocation,control,workspace,store);
   if(!existsSync(wrapper))throw refuseProcessV1("failed","nativeHelpers.wrapper-absent",{path:wrapper});
   return Object.freeze({scheduler,wrapper});
  });
 }));
}

/** 🏗️ Fetches the original scheduler resolve before CPU admission and retains its configuration and registry custody through compilation. */
export function prepareRepositoryCargoSchedulerV1(invocation:ScriptInvocation,control:ResourceLeaseControl,workspace:string,store:RepositoryCompilerStoreV1):Promise<string>{
 requireIssuedResourceControlInvocationV1(control,invocation);
 const selection=schedulerCargoSelectionV1(false,readBuildMode(store.environment.SEMIO_BUILD_MODE!)),leaseDirectory=readNativeStorageReservationPolicyV1(store.storage,store.control).leaseDirectory;
 return withRepositoryCargoInputsV1({invocation,workspace,cwd:workspace,...selection,environment:store.environment,control,storage:store.storage,network:repositoryCargoNetworkV1(invocation,workspace),consumption:{kind:"compiler",producer:"artifact",operation:"build"}},input=>{const held=readSchedulerExecutablesV1(store.environment.CARGO_TARGET_DIR!,store.toolchain.digest,input.environment)?.get("semio-build-scheduler");if(held)return Promise.resolve(held);return withCompilerWorkspaceSharedWriteV1(store.storage,leaseDirectory,input.control,writer=>withCompilerCapacityConsumptionV1({directory:leaseDirectory,control:writer,environment:input.environment,request:{kind:"exact",parallelism:1}},async(_capacity,cpuControl)=>{
  const built=(await buildSchedulerV1({cwd:workspace,targetDirectory:store.environment.CARGO_TARGET_DIR!,buildDirectory:store.buildDirectory,environment:input.environment,arguments:input.selections[0]!.arguments,toolchain:store.toolchain.digest,control:cpuControl,storage:store.storage,sourceBoundary:store.sourceBoundary})).get("semio-build-scheduler");
  if(built===undefined||!existsSync(built))throw refuseProcessV1("failed","nativeHelpers.scheduler-absent",{path:built??""});return async()=>built;
 }));});
}
