/** 🧩️ Semantic plugin build installation owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

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

import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../🏪️store/📥️installation/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../📇️registry/📦️deployment/🟦️.ts";

import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { pluginOutRoot } from "../📋️plan/🟦️.ts";



const extensionOutRoot = defaultExtensionInstallRoot(repoRoot);

/** 🔎️Reports stale extension artifacts read-only, retaining bytes for the owning producer. */
function assertExtensionOutputsFresh(root: string = extensionOutRoot): void {
  if (!existsSync(root)) return;
  const currentHostShim = hostShimSource();
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const dir = join(root, entry.name);
    const retainedWorkerPath = join(dir, "🧵️plugin-worker.js");
    if (existsSync(retainedWorkerPath)) throw new Error(`Extension retained worker preserved: ${retainedWorkerPath}. Review this input before running @semio-tech/framework-os-dev:plugin for its owner.`);
    const shimPath = join(dir, PLUGIN_HOST_SHIM_FILE);
    if (existsSync(shimPath)) {
      if (readFileSync(shimPath, "utf8") !== currentHostShim) throw new Error(`Extension stale host shim preserved: ${shimPath}. Rebuild its owner through @semio-tech/framework-os-dev:plugin after reviewing the retained output.`);
    }
  }
}

/** @emoji 🧩️ Mirrors a just-built extension crate from `🔌️plugin-modules/` into the runtime `/🧩️extension-modules` install root so catalog loads resolve without a separate `.sxt` install step. */
function publishBuiltExtension(target: PluginRegistryEntry, builtOutDir: string): void {
  if (target.role !== "extension") return;
  if (!existsSync(builtOutDir)) return;
  mkdirSync(extensionOutRoot, { recursive: true });
  const outDir = join(extensionOutRoot, moduleDirectoryName(target.pluginId));
  const stagingDir = join(extensionOutRoot, `.staging-${target.pluginId}-${Date.now()}`);
  const retiredDir = join(extensionOutRoot, `.retired-${target.pluginId}-${Date.now()}`);
  cpSync(builtOutDir, stagingDir, { recursive: true });
  for (const file of readdirSync(stagingDir)) {
    if (!file.endsWith(".js")) continue;
    const filePath = join(stagingDir, file);
    const source = readFileSync(filePath, "utf8");
    const rewritten = rewritePreview2ShimImportSource(source, `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`);
    if (rewritten !== source) writeFileSync(filePath, rewritten);
  }
  const moduleUrl = `${MODULE_EXTENSION_ROUTE}/${moduleDirectoryName(target.pluginId)}/${MODULE_BRIDGE_FILE}`;
  const installedAt = Date.now();
  const record = {
    extensionId: target.pluginId,
    directoryName: moduleDirectoryName(target.pluginId),
    version: "0.0.0-dev",
    label: target.pluginId,
    extends: target.extends ?? "",
    moduleUrl,
    packageHash: `dev:${installedAt}`,
    installedAt,
  };
  writeFileSync(join(stagingDir, EXTENSION_INSTALL_META), `${JSON.stringify(record, null, 2)}\n`);
  if (existsSync(outDir)) renameSync(outDir, retiredDir);
  renameSync(stagingDir, outDir);
  if (existsSync(retiredDir)) rmSync(retiredDir, { recursive: true, force: true });
  writeFileSync(join(extensionOutRoot, EXTENSION_WATCH_MARKER), `${JSON.stringify({ kind: "installed", extensionId: target.pluginId, version: record.version, installedAt, emittedAt: Date.now() })}\n`);
  console.log(`published extension ${target.pluginId} -> ${moduleUrl}`);
}

/** @emoji 🧩️ Seeds `/🧩️extension-modules` from any extension crates already present under `🔌️plugin-modules/` (covers restart without rebuild). */
export function syncBuiltExtensionsToInstallRoot(entries: readonly PluginRegistryEntry[]): void {
  for (const target of entries) {
    if (target.role !== "extension") continue;
    const builtOutDir = join(pluginOutRoot, moduleDirectoryName(target.pluginId));
    if (!existsSync(join(builtOutDir, MODULE_BRIDGE_FILE))) continue;
    publishBuiltExtension(target, builtOutDir);
  }
}

export { assertExtensionOutputsFresh, extensionOutRoot, publishBuiltExtension };
