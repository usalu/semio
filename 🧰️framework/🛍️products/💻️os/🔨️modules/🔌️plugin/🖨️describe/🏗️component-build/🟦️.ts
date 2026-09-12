import { existsSync, lstatSync } from "node:fs";
import { join } from "node:path";
import { cargoTargetDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
import { BundleScript, buildBudgetMs, devToolingEnv, resolveTestLevel, resolveWorkspaceBin, runCargoTestBudgeted, runCmd } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
export const CRATE_NAME = "semio-framework-plugin-describe";
export const DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";
export const DESCRIPTOR_JSON_FILENAME = "🔣️.json";
/** 🧱️ Admission bound for the two build artifacts the descriptor emitter reads — the raw
 * `wasm32-wasip2` component and jco's extracted core. {@link buildPluginComponent} builds the
 * UNOPTIMIZED `wasm-dev` profile, so this bounds a build-time input and is deliberately NOT the
 * runtime bound on a shipped component (`DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`, which
 * applies to the optimized artifact). The procedural plugin's dev component is ~80 MB. */
export const FRESH_COMPONENT_MAX_BYTES = 128 * 1024 * 1024;
export const FRESH_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;
export const FRESH_IO_CHUNK_BYTES = 64 * 1024;

export class DescriptorBuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildCargoArtifacts(join(import.meta.dir, "..", "📦️packages", "🦀️rust", "Cargo.toml"), ["--release", "--bin", CRATE_NAME], this.repoRoot);
  }
}

export class DescriptorTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([CRATE_NAME], this.repoRoot, rest);
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

/** @emoji 🛠️ Resolves the debug-profile binary path for the current platform, after ensuring it is built (cargo's incremental cache makes a no-op rebuild fast — never exec a possibly-stale binary). */
export function ensureBuiltBin(repoRoot: string, budgetMs = buildBudgetMs()): string {
  runCmd("cargo", ["build", "-p", CRATE_NAME], { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const binName = process.platform === "win32" ? `${CRATE_NAME}.exe` : CRATE_NAME;
  return join(cargoTargetRoot(repoRoot), "debug", binName);
}

/** @emoji 🎯️ WASI-development artifact path cargo just built for `packageName`, resolved through
 * the same {@link cargoTargetRoot} cargo used in {@link ensureBuiltBin}. */
export function pluginWasmArtifactPath(repoRoot: string, packageName: string, profile = "wasm-dev", targetRoot = cargoTargetRoot(repoRoot)): string {
  return join(targetRoot, "wasm32-wasip2", profile, `${packageName.replace(/-/g, "_")}.wasm`);
}

/** @emoji 🧩 Builds one exact plugin component and returns cargo's fresh output path. */
export function buildPluginComponent(repoRoot: string, packageName: string, rootCdylib = false, budgetMs = buildBudgetMs()): string {
  const buildArgs = rootCdylib ? ["rustc", "-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", "wasm-dev"] : ["build", "-p", packageName, "--target", "wasm32-wasip2", "--profile", "wasm-dev"];
  runCmd("cargo", buildArgs, { cwd: repoRoot, env: devToolingEnv(), budgetMs });
  const component = pluginWasmArtifactPath(repoRoot, packageName);
  if (!existsSync(component)) throw new Error(`cargo did not produce ${component}`);
  return component;
}

/** @emoji 🧬 Extracts the first core module from the exact component with jco's independent parser. */
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
