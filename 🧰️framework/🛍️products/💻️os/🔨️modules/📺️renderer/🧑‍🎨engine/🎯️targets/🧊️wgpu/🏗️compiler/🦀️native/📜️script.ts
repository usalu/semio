import {verifyCapturedCargoArtifactsV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { repositoryCargoNetworkV1, type CargoDependencyNetworkV1, buildRepositoryCargoArtifacts, withRepositoryPublishedCargoExecutableV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { scriptCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, readBuildMode } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";

/** 🧊️ Publishes only the selected renderer binary from Cargo's private compiler outputs. */
export async function buildNativeRenderer(rustPackageRoot: string, profile: string, workspace: string, control: CompilerAdmissionControl,network:CargoDependencyNetworkV1,invocation:ScriptInvocation): Promise<void> {
  const mode = readBuildMode(profile);
  if (process.env.SEMIO_BUILD_MODE !== undefined && readBuildMode(process.env.SEMIO_BUILD_MODE) !== mode) throw new Error("Native renderer mode contradicts its original environment");
  await buildRepositoryCargoArtifacts(join(rustPackageRoot, "Cargo.toml"), ["-p", "semio-renderer-wgpu", "--bin", "semio-wgpu-native", "--features", "native-bin", "--profile", buildModePolicy(mode).cargoProfile], workspace, {producer:"artifact",invocation:invocation,consume:verifyCapturedCargoArtifactsV1, network,control, environment: buildModeEnvironment(mode) });
}

/** 🔒️ Holds the published renderer executable through the reader's native invocation. */
export function withNativeRendererBinaryV1<T>(rustPackageRoot: string, mode: "prototype" | "release", workspace: string, invocation: ScriptInvocation, control: ResourceLeaseControl, body: (path: string, control: ResourceLeaseControl) => Promise<T>): Promise<T> {
  const name = process.platform === "win32" ? "semio-wgpu-native.exe" : "semio-wgpu-native";
  return withRepositoryPublishedCargoExecutableV1(join(rustPackageRoot,"Cargo.toml"),name,workspace,mode,invocation,control,(artifact,phase)=>body(artifact.path,phase));
}

class BuildScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    if(args.length||(profile!=="prototype"&&profile!=="release"))throw new Error("Native renderer builds require the exact profile selected by the build tool");
    await buildNativeRenderer(this.root, profile, this.repoRoot, scriptCompilerAdmissionControlV1(this.invocation, "renderer:build", 0),repositoryCargoNetworkV1(this.invocation,this.repoRoot),this.invocation);
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(resolve(import.meta.dirname, "../../📦️packages/🦀️rust")).register("build", BuildScript)).run(process.argv.slice(2), original));
