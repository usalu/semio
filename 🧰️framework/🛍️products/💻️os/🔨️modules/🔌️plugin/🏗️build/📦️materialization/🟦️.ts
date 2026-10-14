import {readCatalogBuildTargetsV1,runtimeCatalogBuildInventoryV1} from "../../📇️registry/📖️catalog-view/🟦️.ts";
import type { CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel, atTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

/** 🧩️ Semantic plugin build materialization owner. */




import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsLockedPrefsEnv, cargoProfileDir, semioBuildMode, semioReleaseEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import type { PluginBuildTargetV1 } from "../../📇️registry/🔎️discovery/🟦️.ts";

import {
  ensureGuestSlimTypstFontsAt,
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

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, parseModuleDirectories, moduleIdForDirectoryName, moduleRoutePath } from "../../📇️registry/📦️deployment/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { PLUGIN_WASM_TARGET, pluginOutRoot } from "../📋️plan/🟦️.ts";

import { publishBuiltExtension } from "../📥️installation/🟦️.ts";

import { discoverPluginBuildTargets } from "../../📇️registry/🔎️discovery/🟦️.ts";

import { materializeComponentModuleV1 } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";

import { ITERATE_COMPONENT_PROFILE, type ComponentDeliverableV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";

import { publishedDescriptorEmitter } from "../../🖨️describe/🏗️component-build/🟦️.ts";

import { componentStageTargetV1, stageComponentV1 } from "../../🖨️describe/🧩️component-stage/🟦️.ts";



/** 🧵️ Publishes the single package-agnostic shard worker at `🔌️plugin-modules/🧵️shard/`, the
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

/** 🫙 Ensures `🪞️vendor/🔤️guestslim-typst-fonts.bin` exists in the dev runner's module root: the shared
 * {@link ensureGuestSlimTypstFontsAt} from the staged seed, else the font tool builds it in place. */
function ensureGuestSlimTypstFontsAsset(): void {
  if (ensureGuestSlimTypstFontsAt(pluginOutRoot, repoRoot)) return;
  const out = join(pluginOutRoot, GUESTSLIM_FONT_RELATIVE);
  mkdirSync(dirname(out), { recursive: true });
  const status = runCmdStatus("cargo", ["run", "-p", "semio-framework-canvas-font-assets", "--bin", "pack-typst-font-assets", "--", out], { cwd: repoRoot, budgetMs: buildBudgetMs() });
  if (status !== 0 || !existsSync(out)) {
    throw new Error(`guestslim typst fonts asset missing and pack-typst-font-assets failed (expected ${out})`);
  }
}

/** 🔚️ Rewrites bare `@bytecodealliance/preview2-shim/*` imports in already-staged plugin JS. */
function rewriteExistingPluginShimImports(): void {
  if (!existsSync(pluginOutRoot)) return;
  const vendor = preview2ShimVendorDir();
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || !moduleIdForDirectoryName(entry.name, runtimeCatalogBuildInventoryV1().moduleDirectories)) continue;
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


/** 🎯️ One plugin's iterate stage: the `wasm-prototype` build of exactly this crate into its `dist/components-iterate`,
 * described locally (never committed) and bound by a `wasm-prototype` receipt. Every other plugin keeps its shipped module.
 * Fine-grain locking (`.cargo/config.toml`) lets concurrent agents and devs share every already-built unit. */
async function buildPluginCargo(target: PluginBuildTargetV1, control: CompilerAdmissionControl): Promise<{ readonly target: PluginBuildTargetV1; readonly artifact: string; readonly deliverable: ComponentDeliverableV1 }> {
  publishedDescriptorEmitter(repoRoot);
  const staged = await stageComponentV1(repoRoot, componentStageTargetV1(repoRoot, join(target.cratePath, "Cargo.toml")), ITERATE_COMPONENT_PROFILE, "local", control);
  return { target, artifact: staged.deliverable.wasm, deliverable: staged.deliverable };
}

/** 🎯️ One target's MATERIALIZE stage: the shared module materializer over the iterate deliverable (no `wasm-opt`), then
 * the extension publish and the hot-swap marker. Safe to run beside sibling targets: it touches neither the shared cargo
 * build directory nor the global package cache. */
async function materializePlugin(target: PluginBuildTargetV1, built: Readonly<{ deliverable: ComponentDeliverableV1 }>, control: CompilerAdmissionControl): Promise<void> {
  const { output } = await materializeComponentModuleV1(repoRoot, target, built.deliverable, control);
  publishBuiltExtension(target, output);
  const hotSwapMarker = join(pluginOutRoot, MODULE_HOT_SWAP_FILE);
  writeFileSync(hotSwapMarker, `${JSON.stringify({ pluginId: target.pluginId, rebuiltAt: Date.now() })}\n`);
  console.log(`iterated ${target.pluginId} (${PLUGIN_WASM_TARGET}, ${ITERATE_COMPONENT_PROFILE}) -> ${output}`);
}

/** 🔁️ `components iterate <plugin-id>`: the single-plugin browser loop — iterate build, local describe, unoptimized module. */
export async function iterateComponentV1(pluginId: string, control: CompilerAdmissionControl): Promise<void> {
  const target = readCatalogBuildTargetsV1().find((entry) => entry.pluginId === pluginId);
  if (!target) throw new Error(`components iterate: ${pluginId} names no discovered plugin crate with a deployment directory`);
  ensurePreview2ShimVendor();
  ensureGuestSlimTypstFontsAsset();
  publishShardWorker();
  await materializePlugin(target, await buildPluginCargo(target, control), control);
}

export { buildPluginCargo, ensureGuestSlimTypstFontsAsset, ensurePreview2ShimVendor, materializePlugin, pluginWebMaterializeContext, preview2ShimVendorDir, publishShardWorker, readPackageName, rewriteExistingPluginShimImports };
