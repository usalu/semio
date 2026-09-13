/** 🧩️ Semantic activation freshness owner. */

import { ACTIVATION_RECEIPT_FILE, PLAYGROUND_SESSION_OUTPUT_ROOT_ENV, developmentRuntimeRoot, newestComponentSourceMtime, nextActivationReceipt, playgroundSessionOutputPath, pluginModulesRoot, publishActivationReceipt, readActivationReceipt, stagedModuleMtime, stagedModuleReportLines, stagedModuleVerdict, type StagedModuleFacts, type StagedModuleVerdict } from "../🟦️.ts";

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

import { defaultExtensionInstallRoot, EXTENSION_INSTALL_META, EXTENSION_WATCH_MARKER } from "../../../🔌️plugin/🏪️store/📥️installation/🟦️.ts";

import { MODULE_BRIDGE_FILE, MODULE_SHARD_DIRECTORY, MODULE_HOT_SWAP_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, moduleDirectoryName, moduleIdForDirectoryName, moduleRoutePath } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

import { getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { resolveCatalogFilterPluginId } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";




/** @emoji 🔎️ Collects one variant's staged-module freshness facts out of the ONE staging root: the
 * activation receipt names what was activated, each component's owner tree supplies the newest source
 * mtime, and the extension install root supplies the published package hash. Read-only — it never builds,
 * never writes, and never blocks a serve; its whole job is that a served module which is behind its own
 * crate says so out loud instead of looking like "the rebuild did not take". */
export function collectStagedModuleFacts(options: {
  readonly moduleRoot: string;
  readonly installRoot: string;
  readonly receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string }[] };
  readonly components: readonly PluginRegistryEntry[];
}): readonly StagedModuleFacts[] {
  const activated = new Map((options.receipt?.plugins ?? []).map((row) => [row.pluginId, row.artifactSha256]));
  return options.components.map((target): StagedModuleFacts => {
    const directoryName = moduleDirectoryName(target.pluginId);
    const newest = newestComponentSourceMtime(join(repoRoot, target.cratePath, "..", ".."));
    const installedMeta = join(options.installRoot, directoryName, EXTENSION_INSTALL_META);
    let installedPackageHash: string | undefined;
    if (existsSync(installedMeta)) {
      try { installedPackageHash = JSON.parse(readFileSync(installedMeta, "utf8")).packageHash as string; } catch { installedPackageHash = undefined; }
    }
    return {
      pluginId: target.pluginId,
      role: target.role === "extension" ? "extension" : "plugin",
      activationTracked: options.receipt !== undefined,
      stagedAtMs: stagedModuleMtime(join(options.moduleRoot, directoryName)),
      newestSourceMs: newest?.mtimeMs,
      newestSourcePath: newest ? relative(repoRoot, newest.path).split(/[\\/]/).join("/") : undefined,
      receiptArtifactSha256: activated.get(target.pluginId),
      installedPackageHash,
    };
  });
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
function reportServeStagedModuleFreshness(variant: string, renderer: "react" | "wgpu", profile: "dev" | "release", runtime: string, receipt?: { readonly plugins: readonly { readonly pluginId: string; readonly artifactSha256: string }[] }): void {
  try {
    const selected = new Set(filterProjectedPluginRegistry(readGeneratedCatalogProjection(), resolveCatalogFilterPluginId(variant)).map((entry) => entry.pluginId));
    const components = readGeneratedCatalogProjection().entries.filter((entry) => selected.has(entry.pluginId));
    reportStagedModuleFreshness(variant, renderer, profile, collectStagedModuleFacts({
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
