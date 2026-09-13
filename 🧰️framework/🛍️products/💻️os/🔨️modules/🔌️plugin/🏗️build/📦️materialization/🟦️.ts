/** 🧩️ Semantic plugin build materialization owner. */

import { artifactFiles } from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";

import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

import { cargoTargetDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../📇️registry/🔎️discovery/🟦️.ts";

import {
  ensurePreview2ShimVendorAt,
  hostShimSource,
  PLUGIN_HOST_SHIM_FILE,
  PREVIEW2_VENDOR_RELATIVE,
  GUESTSLIM_FONT_RELATIVE,
  pluginComponentBridgeSource,
  rewriteJcoComponentAssetUrls,
  SHARD_WORKER_FILE,
  shardWorkerSource,
  rewriteJcoAsyncResultLifting,
  rewritePreview2ShimImportSource,
  rewritePreview2ShimImports,
  transpilePluginComponentAsync,
  type PluginWebMaterializeContext,
} from "../../🌐️browser-bundle/🏗️materialization/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { PLUGIN_WASM_TARGET, devStagingProfile, pluginCargoArgs, pluginOutRoot, pluginWasmProfile } from "../📋️plan/🟦️.ts";

import { publishBuiltExtension } from "../📥️installation/🟦️.ts";

import { assertPluginOutputChildren, describeBuiltPlugin, stagePluginDescriptor } from "../🛂️descriptor/🟦️.ts";



/** @emoji 🧵️ Publishes the single package-agnostic shard worker at `🔌️plugin-modules/🧵️shard/`, the
 * URL `ShardClient` pool members are constructed from (H2 design). Idempotent: rewritten on every
 * plugin build so a source change to `shardWorkerSource` always reaches the browser. */
function publishShardWorker(): void {
  const shardDir = join(pluginOutRoot, MODULE_SHARD_DIRECTORY);
  mkdirSync(shardDir, { recursive: true });
  writeFileSync(join(shardDir, SHARD_WORKER_FILE), shardWorkerSource());
}

//#endregion 🔖️Blake3

function preview2ShimVendorDir(): string {
  return join(pluginOutRoot, PREVIEW2_VENDOR_RELATIVE);
}

function pluginWebMaterializeContext(): PluginWebMaterializeContext {
  return { repoRoot, preview2VendorDir: preview2ShimVendorDir() };
}

function ensurePreview2ShimVendor(): void {
  ensurePreview2ShimVendorAt(preview2ShimVendorDir(), repoRoot);
}

/** @emoji 🫙 Ensures `🪞️vendor/🔤️guestslim-typst-fonts.bin` exists for plugin workers' typst text path. */
function ensureGuestSlimTypstFontsAsset(): void {
  const out = join(pluginOutRoot, GUESTSLIM_FONT_RELATIVE);
  if (existsSync(out) && statSync(out).size > 0) return;
  mkdirSync(dirname(out), { recursive: true });
  const status = runCmdStatus("cargo", ["run", "-p", "semio-framework-os-infinite", "--bin", "dump-guestslim-typst-fonts", "--features", "render", "--", out], { cwd: repoRoot, budgetMs: buildBudgetMs() });
  if (status !== 0 || !existsSync(out)) {
    throw new Error(`guestslim typst fonts asset missing and dump-guestslim-typst-fonts failed (expected ${out})`);
  }
}

/** @emoji 🔚️ Rewrites bare `@bytecodealliance/preview2-shim/*` imports in already-staged plugin JS. */
function rewriteExistingPluginShimImports(): void {
  if (!existsSync(pluginOutRoot)) return;
  const vendor = preview2ShimVendorDir();
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || !moduleIdForDirectoryName(entry.name)) continue;
    const pluginDir = join(pluginOutRoot, entry.name);
    for (const file of readdirSync(pluginDir)) {
      if (!file.endsWith(".js")) continue;
      rewritePreview2ShimImports(join(pluginDir, file), vendor);
    }
  }
}

async function readPackageName(cratePath: string): Promise<string> {
  const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
  const match = content.match(/^name = "([^"]+)"/m);
  if (!match) throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
  return match[1]!;
}

/** @emoji 🔑️ The `stageArtifacts` ownership key one component's staged module directory carries — byte
 * for byte the key `@semio-tech/framework-plugin-web`'s `materialize <profile> --manifest <Cargo.toml>`
 * writes, so the catalog builder and the per-crate Nx target own the SAME directory in the one staging
 * root instead of each claiming a tree of its own. */
function componentArtifactOwner(target: PluginRegistryEntry, profile: "dev" | "release"): string {
  return `${target.cratePath.split(/[\\/]/).join("/")}/Cargo.toml:browser:${profile}`;
}

//#endregion 🛂️DescriptorPublication
/** 🎯️ Serial component compilation against the ONE shared `cargoTargetDirectory` — descriptor extraction
 * runs after materialization. Fine-grain locking (`.cargo/config.toml`) makes concurrent invocations of
 * this function across agents/devs share every already-built unit, so no caller owns a private target dir. */
async function buildPluginCargo(target: PluginRegistryEntry): Promise<{ readonly target: PluginRegistryEntry; readonly artifact: string }> {
  const packageName = await readPackageName(target.cratePath);
  const profile = pluginWasmProfile();
  const cargoTargetRoot = cargoTargetDirectory(repoRoot);
  if (runCmdStatus("cargo", pluginCargoArgs(packageName, profile), { cwd: repoRoot, env: process.env, budgetMs: buildBudgetMs() }) !== 0) {
    throw new Error(`plugin build failed: ${target.pluginId}`);
  }
  const artifact = join(cargoTargetRoot, PLUGIN_WASM_TARGET, cargoProfileDir(profile), `${packageName.replace(/-/g, "_")}.wasm`);
  return { target, artifact };
}

/** @emoji 🎯️ One target's MATERIALIZE stage: jco transpile, `wasm-opt`, bridge/host-shim file
 * emission, extension publish, hot-swap marker — everything downstream of a finished cargo artifact
 * that touches neither the shared `target/` build-directory lock nor the global `~/.cargo`
 * package-cache lock, so it is safe to run several of these at once (see `buildPluginCatalog`). Does
 * NOT call `publishShardWorker()` — that write is identical content for every target in a catalog run,
 * so callers publish it once rather than redundantly per plugin (still "idempotent: rewritten on every
 * plugin build" per its own doc, just once per BUILD rather than once per PLUGIN). */
async function materializePlugin(target: PluginRegistryEntry, artifact: string): Promise<void> {
  const outDir = join(pluginOutRoot, moduleDirectoryName(target.pluginId));
  mkdirSync(pluginOutRoot, { recursive: true });
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  const componentBase = `${jsBase}_component`;
  assertPluginOutputChildren(outDir, componentBase);
  const temporary = mkdtempSync(`${outDir}.materialize-`);
  try {
    writeFileSync(join(temporary, "🟨️.js"), hostShimSource());
    // 🪶️ Transpile straight from cargo's own build output — plugin-modules never receives a copy of the
    // full component `.wasm` (see `emitRustArtifacts`'s doc comment). The browser only ever fetches
    // jco's extracted `${componentBase}.core.wasm`, so shipping the untranspiled component alongside it
    // was pure duplicate ~60MB-class weight per plugin; native `os run` now reads straight from `target/`.
    // 🚀️ T-P8: the ASYNC (non-blocking-spawn) transpile — see its doc — is what makes `buildPluginCatalog`'s
    // bounded-parallel materialize stage actually overlap in wall-clock time, not just in scheduling.
    await transpilePluginComponentAsync(artifact, temporary, componentBase, pluginWebMaterializeContext());
    await describeBuiltPlugin(target, artifact, join(temporary, `${componentBase}.js`));
    if (!stagePluginDescriptor(target, temporary)) throw new Error(`Missing fresh descriptor for ${target.pluginId}`);
    writeFileSync(join(temporary, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(componentBase, target.wasmOut));
    // 🧱️ Published through the SAME `stageArtifacts` ownership key `@semio-tech/framework-plugin-web`'s
    // `materialize-<profile>` uses, so this catalog builder and the per-crate Nx target are two producers of
    // ONE tree rather than two trees: either may replace a module directory the other staged, and neither
    // can leave a half-written module visible to a running dev server.
    await stageArtifacts(outDir, componentArtifactOwner(target, devStagingProfile()), artifactFiles(temporary));
  } finally { rmSync(temporary, { recursive: true, force: true }); }
  // 🧩️ Publish extension artifacts before the hot-swap marker: the browser reloads `/🧩️extension-modules/...`
  // from the SSE event, so the install root must already serve the new files.
  publishBuiltExtension(target, outDir);
  const hotSwapMarker = join(pluginOutRoot, MODULE_HOT_SWAP_FILE);
  writeFileSync(hotSwapMarker, `${JSON.stringify({ pluginId: target.pluginId, rebuiltAt: Date.now() })}\n`);
  console.log(`built program ${target.pluginId} (${PLUGIN_WASM_TARGET}, ${pluginWasmProfile()}) -> ${outDir}`);
}

export { buildPluginCargo, componentArtifactOwner, ensureGuestSlimTypstFontsAsset, ensurePreview2ShimVendor, materializePlugin, pluginWebMaterializeContext, preview2ShimVendorDir, publishShardWorker, readPackageName, rewriteExistingPluginShimImports };
