import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";

/** 📂️ Gives the native producer and its runner one profile-specific artifact directory. */
export function nativeRendererDirectory(rustPackageRoot: string, profile: string): string {
  if (profile !== "dev" && profile !== "release") throw new Error("Select a native renderer profile: dev or release");
  return join(rustPackageRoot, "dist", `native-${profile}`);
}

/** 🧊️ Publishes only the selected renderer binary from Cargo's private compiler outputs. */
export async function buildNativeRenderer(rustPackageRoot: string, profile: string, workspace: string): Promise<void> {
  const output = nativeRendererDirectory(rustPackageRoot, profile);
  await buildCargoArtifacts(join(rustPackageRoot, "Cargo.toml"), ["-p", "semio-framework-os-renderer-wgpu", "--bin", "semio-wgpu-native", "--features", "native-bin", ...(profile === "release" ? ["--release"] : [])], workspace, { output });
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
    await buildNativeRenderer(this.root, profile, this.repoRoot);
  }
}

if (import.meta.main) await new ScriptRouter(resolve(import.meta.dir, "../../📦️packages/🦀️rust")).register("build", BuildScript).run(process.argv.slice(2));
