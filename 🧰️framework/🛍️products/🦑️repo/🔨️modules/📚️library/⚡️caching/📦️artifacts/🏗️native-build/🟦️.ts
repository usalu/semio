import {profilePhaseV1,repoProfileV1} from "../../../🏗️build/⏱️profile/🟨️.mjs";
import {observeCargoPlanningPortV1} from "./⏱️observation/🟦️.ts";
import{nativeArtifactCargoConsumptionV1}from"../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../📋️native-orchestration/📥️invocation/🟦️.ts";
import {CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import {repositoryCargoCompilerInputsV1} from "../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import type {CargoDependencyNetworkV1} from "../../../🏃️process/🦀️cargo/📥️dependencies/📋️contract/🟦️.ts";
import { requestedCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, relative, resolve, dirname, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { cargoPreparationInvocationCurrentV1, cargoWorkspacePreparationInvocationV1 } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🗂️directories/🟦️.ts";
import { publishedCargoArtifactDirectoryV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📤️published/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { actualCargoCompilerPlanningPortV1, withCargoArtifactPlanV1, type CargoArtifactPlanV1, type CargoArtifactBuildPolicyV1, type CargoArtifactBuildOptionsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {recordStoreContentV1} from "../../../🏗️build/🧾️store-receipt/🟨️.mjs";
import {withRepositoryCompilerStoreProductV1,withRepositoryCompilerStoreV1,type RepositoryCompilerStoreV1} from "../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {focusFromCargoArgumentsV1} from "../../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {runtimeParseTomlV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 🏷️ The one version every crate in this workspace carries (`[workspace.package] version`). */
export function workspaceCargoVersion(repoRoot = getWorkspaceRoot()): string {
  const manifest = readFileSync(join(repoRoot, "Cargo.toml"), "utf8");
  const heading = "[workspace.package]";
  const start = manifest.indexOf(heading);
  if (start < 0) throw new Error("Cargo.toml declares no [workspace.package] section");
  const rest = manifest.slice(start + heading.length);
  const end = rest.indexOf("\n[");
  const version = /^\s*version\s*=\s*"([^"]+)"/m.exec(end < 0 ? rest : rest.slice(0, end));
  if (!version) throw new Error("Cargo.toml declares no [workspace.package] version");
  return version[1]!;
}

/** 🏗️ Authors explicit repository compiler storage for one source owner. */
export function repositoryCargoArtifactBuildPolicyV1(cwd: string, root = getWorkspaceRoot(), environment: Readonly<Record<string,string|undefined>> = process.env): CargoArtifactBuildPolicyV1 {
  return { version: 1, compilerConfiguration: join(root,".cargo/config.toml"), mode: readBuildMode(environment.SEMIO_BUILD_MODE), cwd: resolve(cwd), buildDirectory: cargoDirectories(root, environment).build, leaseDirectory: join(root, ".cache", "b", "l", "m", "data"), captureDirectory: environment.SEMIO_TEST_ARTIFACT_DIR ? resolve(environment.SEMIO_TEST_ARTIFACT_DIR) : resolve(cwd, "dist"), budgetMs: buildBudgetMs() };
}
/** 🛤️ The crates whose lane an artifact build takes: the `-p` packages of its arguments, else the package of its manifest (so the same product keeps getting the same warm lane), else the manifest path. */
export function cargoLaneCratesV1(args: readonly string[], manifest: string, root: string): string[] {
  const focus = focusFromCargoArgumentsV1(args);
  if (focus.length) return focus;
  let name: string | undefined;
  try { name = (runtimeParseTomlV1(readFileSync(manifest, "utf8")) as { package?: { name?: string } }).package?.name; } catch { name = undefined; }
  return [name ?? relative(root, manifest).split(sep).join("/")];
}
/** 🦀️ Composes selected repository preparation with the neutral compiler capture port. */
export async function withRepositoryCargoArtifactPlanV1<T>(manifest: string, args: string[], root: string, options: Omit<CargoArtifactBuildOptionsV1,"storage"|"sourceBoundary"|"maximumObservationBytes"|"withCargoInputs">&Readonly<{invocation:ScriptInvocation;producer:"artifact"|"descriptor-emitter";network:CargoDependencyNetworkV1}>, body:(plan:CargoArtifactPlanV1,control:ResourceLeaseControl)=>Promise<T>): Promise<T> {
  args=[...args];options=Object.freeze({...options,...(options.environment?{environment:Object.freeze({...options.environment})}:{})});requireIssuedResourceControlInvocationV1(options.control,options.invocation);const authority=readNativeCallerAuthorityV1(options.invocation.capabilities);if(resolve(root)!==authority.repositoryRoot||options.environment?.SEMIO_BUILD_MODE!==undefined&&options.environment.SEMIO_BUILD_MODE!==authority.original.buildMode)throw Error("Actual Cargo producer workspace and mode required");const consumption=nativeArtifactCargoConsumptionV1(options.producer,options.command??"build"),selectedEnvironment={...(options.environment??process.env),SEMIO_BUILD_MODE:authority.original.buildMode};const policy = repositoryCargoArtifactBuildPolicyV1(root, root, selectedEnvironment), control = requestedCompilerAdmissionControlV1(options.control, policy.budgetMs), path = resolve(root, manifest);
  const environment = selectedEnvironment;
  return profilePhaseV1("cargo.store.scope",()=>withRepositoryCompilerStoreV1(root,policy.mode,control,environment,async (store,control)=>profilePhaseV1("cargo.store.body",async()=>{
    let produced=false;
    const storage={...repositoryCargoPreparationStorageV1(root),directory:requireNativeStoragePathV1(store.root,join(store.buildDirectory,"preparation"))},preparedEnvironment={...store.environment,CARGO_NET_OFFLINE:String(options.network.offline)},request=cargoWorkspacePreparationInvocationV1(storage,root,[options.command??"build","--manifest-path",path,...args],root,preparedEnvironment);
    if(request&&!cargoPreparationInvocationCurrentV1(storage,root,request)){await store.storage.recheck();const output=await profilePhaseV1("cargo.prepare.workspace",()=>captureOwnedCommandV1(request.command,[...request.args],request.cwd,{control,env:request.environment,output:{maximumBytes:16777216,maximumLines:65536}}));if(output)process.stderr.write(output);await store.storage.recheck();}
    const compile=async(view:RepositoryCompilerStoreV1,control:ResourceLeaseControl)=>{const withCargoInputs=repositoryCargoCompilerInputsV1({invocation:options.invocation,consumption,workspace:root,network:options.network,control,storage:view.storage});const observed=repoProfileV1().enabled?observeCargoPlanningPortV1(options.planningPort??actualCargoCompilerPlanningPortV1()):undefined;try{return await profilePhaseV1("cargo.plan.scope",()=>withCargoArtifactPlanV1(path,args,{...policy,buildDirectory:view.buildDirectory,captureDirectory:view.captureDirectory},{...options,...(observed?{planningPort:observed.port}:{}),withCargoInputs,control,maximumObservationBytes:Math.min(CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1,authority.transport.maximumBytes),storage:view.storage,sourceBoundary:view.sourceBoundary,environment:{...view.environment,CARGO_NET_OFFLINE:String(options.network.offline)}},(plan,actual)=>body(Object.freeze({...plan,async produce(){await plan.produce();produced=true;}}),actual)));}finally{await observed?.finish();}};
    const result=await(options.observationOutput?compile(store,control):withRepositoryCompilerStoreProductV1(store,control,{manifest:path,command:options.command??"build",producer:options.producer,arguments:args,crates:cargoLaneCratesV1(args,path,root)},compile));
    if(produced&&!options.observationOutput)await recordStoreContentV1({workspaceRoot:root,environment:selectedEnvironment,paths:[options.output===undefined?publishedCargoArtifactDirectoryV1(store.storage.backing.root,path,policy.mode):resolve(dirname(path),options.output)]});
    return result;
  })));
}

/** 🏭️ Uses the same owned plan and publication path as planned native consumers. */
export async function buildRepositoryCargoArtifacts(manifest: string, args: string[], root: string, options: Omit<CargoArtifactBuildOptionsV1,"storage"|"sourceBoundary"|"maximumObservationBytes"|"withCargoInputs">&Readonly<{invocation:ScriptInvocation;producer:"artifact"|"descriptor-emitter";network:CargoDependencyNetworkV1}>): Promise<void> {
  return withRepositoryCargoArtifactPlanV1(manifest,args,root,options,plan=>plan.produce());
}
