import { scriptCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, readBuildMode } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";

/** 📂️ Gives the native producer and its runner one profile-specific artifact directory. */
export function nativeRendererDirectory(rustPackageRoot: string, profile: string): string {
  return join(rustPackageRoot, "dist", `native-${readBuildMode(profile)}`);
}

/** 🧊️ Publishes only the selected renderer binary from Cargo's private compiler outputs. */
export async function buildNativeRenderer(rustPackageRoot: string, profile: string, workspace: string, control: CompilerAdmissionControl): Promise<void> {
  const mode = readBuildMode(profile), output = nativeRendererDirectory(rustPackageRoot, mode);
  if (process.env.SEMIO_BUILD_MODE !== undefined && readBuildMode(process.env.SEMIO_BUILD_MODE) !== mode) throw Error("Native renderer mode contradicts its original environment");
  await buildRepositoryCargoArtifacts(join(rustPackageRoot, "Cargo.toml"), ["-p", "semio-framework-os-renderer-wgpu", "--bin", "semio-wgpu-native", "--features", "native-bin", "--profile", buildModePolicy(mode).cargoProfile], workspace, { control, output, environment: buildModeEnvironment(mode) });
}

/** 🚀️ Selects the executable materialized by the native producer or an Nx cache restoration. */
export function nativeRendererBinary(rustPackageRoot: string, profile: string): string {
  const path = join(nativeRendererDirectory(rustPackageRoot, profile), `semio-wgpu-native${process.platform === "win32" ? ".exe" : ""}`);
  if (!existsSync(path)) throw new Error(`Missing Nx native renderer artifact: ${path}`);
  return path;
}

class BuildScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    if (args.length) throw new Error("Native renderer builds accept only the profile selected by Nx");
    await buildNativeRenderer(this.root, profile, this.repoRoot, scriptCompilerAdmissionControlV1(this.invocation, "renderer:build", 0));
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(resolve(import.meta.dir, "../../📦️packages/🦀️rust")).register("build", BuildScript)).run(process.argv.slice(2), original));
