import{withCompilerWorkspaceWriteV1}from"../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {withRepositoryCargoInputsV1,type CargoDependencyNetworkV1} from "../../../🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { requestedCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, resolve, dirname } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { cargoWorkspacePreparationInvocationV1 } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { buildCargoArtifacts, type CargoArtifactBuildPolicyV1, type CargoArtifactBuildOptionsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {withRepositoryCompilerStoreV1} from "../../💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

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
/** 🦀️ Composes selected repository preparation with the neutral compiler capture port. */
export async function buildRepositoryCargoArtifacts(manifest: string, args: string[], root: string, options: Omit<CargoArtifactBuildOptionsV1,"storage"|"sourceBoundary">&Readonly<{network:CargoDependencyNetworkV1}>): Promise<void> {
  const policy = repositoryCargoArtifactBuildPolicyV1(root, root, options.environment ?? process.env), control = requestedCompilerAdmissionControlV1(options.control, policy.budgetMs), path = resolve(root, manifest);
  const environment = options.environment ?? process.env;
  await withRepositoryCompilerStoreV1(root,policy.mode,control,environment,async store=>{
    const storage={...repositoryCargoPreparationStorageV1(root),directory:requireNativeStoragePathV1(store.root,join(store.buildDirectory,"preparation"))},preparedEnvironment={...store.environment,CARGO_NET_OFFLINE:String(options.network.offline)},request=cargoWorkspacePreparationInvocationV1(storage,root,[options.command??"build","--manifest-path",path,...args],root,preparedEnvironment);
    if(request){await store.storage.recheck();const output=await captureOwnedCommandV1(request.command,[...request.args],request.cwd,{control,env:request.environment,output:{maximumBytes:16777216,maximumLines:65536}});if(output)process.stderr.write(output);await store.storage.recheck();}
    return withRepositoryCargoInputsV1({workspace:root,cwd:policy.cwd,manifest:path,args,network:options.network,control,storage:store.storage,environment:preparedEnvironment},environment=>withCompilerWorkspaceWriteV1(store.storage,join(root,".cache/b/l/m/data"),control,()=>buildCargoArtifacts(path,args,{...policy,buildDirectory:store.buildDirectory,captureDirectory:store.captureDirectory},{...options,control,storage:store.storage,sourceBoundary:store.sourceBoundary,environment})));
  });
}
