/** 🧩️ Semantic activation freshness owner. */

import { ACTIVATION_RECEIPT_FILE, PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, developmentRuntimeRoot, nextActivationReceipt, playgroundSessionOutputPath, pluginModulesRoot, publishActivationReceipt, readActivationReceipt, resolveBootSourceContentHashes, resolveBootSourceContentHashesAsync, stagedModuleMtime, stagedModuleReportLines, stagedModuleVerdict, writeStagedSourceContentHash, type PreparedComponentFacts, type StagedModuleFacts, type StagedModuleVerdict } from "../🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { resolveCatalogFilterPluginId } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";




/** @emoji 🩺️ Reads one component's staged module directory into the pure facts
 * {@link preparedComponentVerdict} decides on — the ONE disk reader the preparation pass and the
 * activation receipt share, so "prepared" means the same thing in both. A crate that never compiled
 * leaves no directory, and an unreadable descriptor is a fact here, never a thrown error. */
export function stagedComponentFacts(moduleRoot: string, pluginId: string): PreparedComponentFacts {
  const directory = join(moduleRoot, moduleDirectoryName(pluginId));
  if (!existsSync(directory)) return { pluginId, directoryPresent: false, bridgePresent: false, artifactMarkerPresent: false };
  let descriptorPluginId: string | undefined;
  try { descriptorPluginId = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8")).manifest?.pluginId as string | undefined; } catch { descriptorPluginId = undefined; }
  return {
    pluginId,
    directoryPresent: true,
    descriptorPluginId,
    bridgePresent: existsSync(join(directory, MODULE_BRIDGE_FILE)),
    artifactMarkerPresent: existsSync(join(directory, ".nx-artifact.json")),
  };
}

/** @emoji 🔎️ Collects one variant's staged-module freshness facts out of the ONE staging root: the
 * activation receipt names what was activated, each component's owner tree supplies the newest source
 * mtime, and the extension install root supplies the published package hash. Read-only — it never builds,
 * never writes, and never blocks a serve; its whole job is that a served module which is behind its own
 * crate says so out loud instead of looking like "the rebuild did not take". */
export function collectStagedModuleFacts(options: {
  readonly moduleRoot: string;
  readonly installRoot: string;
  readonly receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string; readonly sourceContentSha256?: string }[] };
  readonly components: readonly PluginRegistryEntry[];
}): readonly StagedModuleFacts[] {
  const activated = new Map((options.receipt?.plugins ?? []).map((row) => [row.pluginId, row.artifactSha256]));
  const activatedSource = new Map((options.receipt?.plugins ?? []).map((row) => [row.pluginId, (row as { sourceContentSha256?: string }).sourceContentSha256]));
  return options.components.map((target): StagedModuleFacts => {
    const directoryName = moduleDirectoryName(target.pluginId);
    const installedMeta = join(options.installRoot, directoryName, EXTENSION_INSTALL_META);
    let installedPackageHash: string | undefined;
    if (existsSync(installedMeta)) {
      try { installedPackageHash = JSON.parse(readFileSync(installedMeta, "utf8")).packageHash as string; } catch { installedPackageHash = undefined; }
    }
    const moduleDirectory = join(options.moduleRoot, directoryName);
    const sourceRoot = join(repoRoot, target.cratePath, "..", "..");
    const receiptSource = activatedSource.get(target.pluginId);
    const stagedAtMs = stagedModuleMtime(moduleDirectory);
    const hashes = resolveBootSourceContentHashes({
      sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: receiptSource,
    });
    return {
      pluginId: target.pluginId,
      role: target.role === "extension" ? "extension" : "plugin",
      activationTracked: options.receipt !== undefined,
      stagedAtMs,
      newestSourceMs: hashes.newestSourceMs,
      newestSourcePath: hashes.newestSourcePath ? relative(repoRoot, hashes.newestSourcePath).split(/[\\/]/).join("/") : undefined,
      sourceContentSha256: hashes.sourceContentSha256,
      stagedSourceContentSha256: hashes.stagedSourceContentSha256,
      receiptArtifactSha256: activated.get(target.pluginId),
      installedPackageHash,
    };
  });
}

export async function collectStagedModuleFactsAsync(options: {
  readonly moduleRoot: string;
  readonly installRoot: string;
  readonly receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string; readonly sourceContentSha256?: string }[] };
  readonly components: readonly PluginRegistryEntry[];
}): Promise<readonly StagedModuleFacts[]> {
  const activated = new Map((options.receipt?.plugins ?? []).map((row) => [row.pluginId, row.artifactSha256]));
  const activatedSource = new Map((options.receipt?.plugins ?? []).map((row) => [row.pluginId, (row as { sourceContentSha256?: string }).sourceContentSha256]));
  return Promise.all(options.components.map(async (target): Promise<StagedModuleFacts> => {
    const directoryName = moduleDirectoryName(target.pluginId);
    const installedMeta = join(options.installRoot, directoryName, EXTENSION_INSTALL_META);
    let installedPackageHash: string | undefined;
    if (existsSync(installedMeta)) {
      try { installedPackageHash = JSON.parse(readFileSync(installedMeta, "utf8")).packageHash as string; } catch { installedPackageHash = undefined; }
    }
    const moduleDirectory = join(options.moduleRoot, directoryName);
    const sourceRoot = join(repoRoot, target.cratePath, "..", "..");
    const receiptSource = activatedSource.get(target.pluginId);
    const stagedAtMs = stagedModuleMtime(moduleDirectory);
    const hashes = await resolveBootSourceContentHashesAsync({
      sourceRoot,
      moduleDirectory,
      receiptSourceContentSha256: receiptSource,
    });
    return {
      pluginId: target.pluginId,
      role: target.role === "extension" ? "extension" : "plugin",
      activationTracked: options.receipt !== undefined,
      stagedAtMs,
      newestSourceMs: hashes.newestSourceMs,
      newestSourcePath: hashes.newestSourcePath ? relative(repoRoot, hashes.newestSourcePath).split(/[\\/]/).join("/") : undefined,
      sourceContentSha256: hashes.sourceContentSha256,
      stagedSourceContentSha256: hashes.stagedSourceContentSha256,
      receiptArtifactSha256: activated.get(target.pluginId),
      installedPackageHash,
    };
  }));
}

/** @emoji 📣️ Prints one `[stale]` line per component whose served bytes are behind, each naming the exact
 * Nx target that fixes it. Called at serve start and again on every activation-receipt change, so a
 * restage that lands while the server runs retires its own warning. */
export function reportStagedModuleFreshness(variant: string, renderer: "react" | "wgpu", profile: "dev" | "release", facts: readonly StagedModuleFacts[]): readonly StagedModuleVerdict[] {
  const verdicts = facts.map(stagedModuleVerdict);
  const lines = stagedModuleReportLines(verdicts, `bun nx run @semio-tech/framework-os-dev:activate-${variant}-${renderer}-${profile}`);
  for (const line of lines) console.warn(line);
  if (lines.length === 0) console.log(`[fresh] ${facts.length} staged components match their sources and the activation receipt`);
  return verdicts;
}

/** @emoji 🔎️ Serve-start freshness pass over the one staging root — never throws: a dev server that
 * refuses to start over a stale module is worse than one that says which module is stale. */
async function reportServeStagedModuleFreshness(variant: string, renderer: "react" | "wgpu", profile: "dev" | "release", runtime: string, receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string }[] }): Promise<void> {
  try {
    const selected = new Set(filterProjectedPluginRegistry(readGeneratedCatalogProjection(), resolveCatalogFilterPluginId(variant)).map((entry) => entry.pluginId));
    const components = readGeneratedCatalogProjection().entries.filter((entry) => selected.has(entry.pluginId));
    reportStagedModuleFreshness(variant, renderer, profile, await collectStagedModuleFactsAsync({
      moduleRoot: pluginModulesRoot(profile),
      installRoot: join(runtime, "extensions"),
      receipt,
      components,
    }));
  } catch (error) {
    console.warn(`[stale] freshness check unavailable: ${String(error)}`);
  }
}

export { reportServeStagedModuleFreshness };
