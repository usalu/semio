/** 🧩️ Semantic plugin build descriptor owner. */

import { ACTOR_COMPONENT_EXPORTS, assertActorComponentExports, finalizePluginDescriptor, PLUGIN_DESCRIPTOR_PROBE_SOURCE } from "../../🌐️browser-bundle/🛂️descriptor/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

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

import { pluginOutRoot } from "../📋️plan/🟦️.ts";



/** 🛡️Rejects unexpected output children without removing retained files before a build. */
function assertPluginOutputChildren(outDir: string, componentBase: string): void {
  if (!existsSync(outDir)) return;
  const files = new Set([PLUGIN_HOST_SHIM_FILE, MODULE_BRIDGE_FILE, `${componentBase}.js`, `${componentBase}.d.ts`, `${componentBase}.core.wasm`, "🔣️.json", "🛂️.descriptor.semio", ".nx-artifact.json"]);
  for (const entry of readdirSync(outDir, { withFileTypes: true })) {
    if (entry.isDirectory() && entry.name === "interfaces") continue;
    if (entry.isFile() && files.has(entry.name)) continue;
    throw new Error(`Unexpected plugin output preserved: ${join(outDir, entry.name)}. Review this retained input before running @semio-tech/framework-os-dev:plugin for its owner.`);
  }
}

/** @emoji 🛂️ Publishes the checked-in build-time descriptor beside the generated browser module.
 * `fetchDescriptorManifest()` deliberately reads this sibling before any actor is instantiated, so
 * leaving descriptors only at their owner roots makes every otherwise-valid module appear app-less
 * at runtime. Unmigrated crates remain honest: no source descriptor means no staged descriptor. */
function stagePluginDescriptor(target: PluginRegistryEntry, outDir: string, root: string = repoRoot): boolean {
  const ownerRoot = join(root, target.cratePath, "..", "..");
  const descriptorJson = join(ownerRoot, "🔣️.json");
  if (!existsSync(descriptorJson)) {
    rmSync(join(outDir, "🔣️.json"), { force: true });
    rmSync(join(outDir, "🛂️.descriptor.semio"), { force: true });
    return false;
  }
  copyFileSync(descriptorJson, join(outDir, "🔣️.json"));
  const descriptorPack = join(ownerRoot, "🛂️.descriptor.semio");
  if (existsSync(descriptorPack)) copyFileSync(descriptorPack, join(outDir, "🛂️.descriptor.semio"));
  else rmSync(join(outDir, "🛂️.descriptor.semio"), { force: true });
  return true;
}

/** @emoji 🔁️ Refreshes descriptor siblings for already-materialized modules on zero-build starts. */
function syncBuiltPluginDescriptors(entries: readonly PluginRegistryEntry[]): void {
  for (const target of entries) {
    const outDir = join(pluginOutRoot, moduleDirectoryName(target.pluginId));
    if (existsSync(outDir)) stagePluginDescriptor(target, outDir);
  }
}

//#region 🛂️DescriptorPublication
async function pluginFileDigest(path: string): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest("hex");
}

/** 🛂️ Describes the freshly materialized component with the same JSPI engine as web actors. */
async function describeBuiltPlugin(target: PluginRegistryEntry, artifact: string, componentModule: string): Promise<void> {
  const probe = runProbe("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", PLUGIN_DESCRIPTOR_PROBE_SOURCE, componentModule], { cwd: repoRoot, budgetMs: 60_000 });
  if (probe.status !== 0) throw new Error(`Plugin descriptor failed for ${target.pluginId}: ${probe.stderr}`);
  const base64 = probe.stdout.trim();
  if (!/^[A-Za-z0-9+/]+={0,2}$/.test(base64)) throw new Error(`Invalid descriptor response for ${target.pluginId}`);
  const [wasmHash, coreHash] = await Promise.all([pluginFileDigest(artifact), pluginFileDigest(componentModule.replace(/\.js$/, ".core.wasm"))]);
  const descriptor = finalizePluginDescriptor(Buffer.from(base64, "base64"), target.pluginId, wasmHash, coreHash);
  const ownerRoot = join(repoRoot, target.cratePath, "..", "..");
  mkdirSync(ownerRoot, { recursive: true });
  writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), descriptor.pack);
  writeFileSync(join(ownerRoot, "🔣️.json"), descriptor.json);
  console.log(`described ${target.pluginId} -> ${ownerRoot}`);
}

export { assertPluginOutputChildren, describeBuiltPlugin, pluginFileDigest, stagePluginDescriptor, syncBuiltPluginDescriptors };
