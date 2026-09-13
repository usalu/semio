/** 🧩️ Semantic activation installation owner. */

import { artifactFiles } from "../../../🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";

import { stageArtifacts } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { createHash } from "node:crypto";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

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
} from "../../../🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";

import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";



/** 🔏️ Includes every completed module/support byte in its runtime activation identity. */
async function activationFilesDigest(files: ReadonlyMap<string, string>, signal: AbortSignal): Promise<string> {
  const hash = createHash("sha256");
  for (const [name, path] of [...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)) {
    signal.throwIfAborted();
    hash.update(JSON.stringify([name.replaceAll("\\", "/"), statSync(path).size]) + "\n");
    for await (const chunk of createReadStream(path, { signal })) hash.update(chunk);
  }
  return hash.digest("hex");
}

/** 🧩️ Installs source-owned extensions into this development variant's runtime namespace. */
async function publishActivatedExtension(target: PluginRegistryEntry, source: string, installRoot: string, artifactSha256: string, rebuiltAt: number, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  const output = join(installRoot, moduleDirectoryName(target.pluginId)), recordPath = join(output, EXTENSION_INSTALL_META);
  const files = artifactFiles(source);
  files.delete(".nx-artifact.json");
  if (existsSync(recordPath) && JSON.parse(readFileSync(recordPath, "utf8")).packageHash === artifactSha256 && [...files.keys()].every((name) => existsSync(join(output, name)))) return;
  mkdirSync(installRoot, { recursive: true });
  const temporary = mkdtempSync(join(installRoot, `.extension-${target.pluginId}-`));
  try {
    for (const [name, path] of files) if (name.endsWith(".js")) {
      signal.throwIfAborted();
      const rewritten = rewritePreview2ShimImportSource(readFileSync(path, "utf8"), `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`);
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      writeFileSync(destination, rewritten);
      files.set(name, destination);
    }
    const record = { extensionId: target.pluginId, directoryName: moduleDirectoryName(target.pluginId), version: "0.0.0-dev", label: target.pluginId, extends: target.extends ?? "", moduleUrl: `${MODULE_EXTENSION_ROUTE}/${moduleDirectoryName(target.pluginId)}/${MODULE_BRIDGE_FILE}`, packageHash: artifactSha256, installedAt: rebuiltAt };
    const metadata = join(temporary, EXTENSION_INSTALL_META);
    writeFileSync(metadata, JSON.stringify(record) + "\n");
    files.set(EXTENSION_INSTALL_META, metadata);
    await stageArtifacts(output, `development-extension:${target.pluginId}`, files, { signal });
    console.log(`Activated extension ${target.pluginId}`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

export { activationFilesDigest, publishActivatedExtension };
