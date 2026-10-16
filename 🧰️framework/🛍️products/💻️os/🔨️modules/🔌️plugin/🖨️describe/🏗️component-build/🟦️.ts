import {requireHostPlatformBuildArgumentsV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🎯️host-platform/🟦️.ts";
import {buildDescriptorEmitterV1} from "../🛠️emitter/🟦️.ts";
import {verifyCapturedCargoArtifactsV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { existsSync, lstatSync } from "node:fs";
import { join } from "node:path";
import { cargoTargetDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { devToolingEnv, resolveWorkspaceBin, runRepositoryCargoTests, runCmd } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { COMPONENT_TARGET, type ComponentProfileV1 } from "../🏷️component-receipt/🟦️.ts";
export const CRATE_NAME = "semio-framework-plugin-describe";
export const DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";
export const DESCRIPTOR_JSON_FILENAME = "🔣️.json";
export const FRESH_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;
export const FRESH_IO_CHUNK_BYTES = 64 * 1024;

export class DescriptorBuildScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    requireHostPlatformBuildArgumentsV1(segments,process.platform);
    await buildDescriptorEmitterV1(this.repoRoot,this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"descriptor:build",0));
  }
}

export class DescriptorTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([CRATE_NAME], this.repoRoot, this.invocation, rest);
  }
}

export function freshWasmArtifactSize(path: string, maximum: number, label: string): number {
  const info = lstatSync(path);
  if (!info.isFile() || info.isSymbolicLink() || !Number.isSafeInteger(info.size) || info.size < 1 || info.size > maximum)
    throw new Error(`${label} file bound: byteLength=${info.size} maximum=${maximum}`);
  return info.size;
}

export function cargoTargetRoot(repoRoot: string): string {
  return cargoTargetDirectory(repoRoot);
}

/** 🎯️ The uplifted artifact cargo writes for `packageName` under one component profile, resolved through
 * the same {@link cargoTargetRoot} every repository build publishes through. */
export function pluginWasmArtifactPath(repoRoot: string, packageName: string, profile: ComponentProfileV1, targetRoot = cargoTargetRoot(repoRoot)): string {
  return join(targetRoot, COMPONENT_TARGET, profile, `${packageName.replace(/-/g, "_")}.wasm`);
}

/** 🧬 Extracts the first core module from the exact component with jco's independent parser. */
export function extractPluginCore(repoRoot: string, component: string, outDir: string, baseName: string, budgetMs = buildBudgetMs()): string {
  const jco = resolveWorkspaceBin("@bytecodealliance/jco", repoRoot);
  if (!jco) throw new Error("missing @bytecodealliance/jco workspace binary; run bun install");
  runCmd("node", [jco, "transpile", component, "-o", outDir, "--name", baseName, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], {
    cwd: repoRoot,
    env: devToolingEnv(),
    budgetMs,
  });
  const core = join(outDir, `${baseName}.core.wasm`);
  if (!existsSync(core)) throw new Error(`jco did not extract ${core}`);
  return core;
}
