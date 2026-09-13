/** 🧩️ Semantic plugin build watch owner. */

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

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../../📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { buildPlugin, buildPlugins } from "../🏃️execution/🟦️.ts";

import { resolvePluginBuildTargets } from "../📋️plan/🟦️.ts";

import { resolveCatalogFilterPluginId } from "../📋️plan/🟦️.ts";



//#endregion 🪶️PluginSizeMeasurement

/** @emoji 👀️ A plugin crate's edits alone don't cover every source that feeds its build: multi-crate
 * app families (e.g. `fem/plugin/rs` depending on `fem/2d/rs`/`fem/3d/rs`/`fem/core/rs`, or an
 * example fixture under `fem/2d/example`) live as SIBLING directories under the same top-level app
 * folder, not inside the plugin crate itself. Watching just `target.cratePath` misses them, so a
 * schema or fixture edit never triggers a hot-swap rebuild. Framework-hosted plugin crates
 * (`framework/...`) keep the narrow crate-only watch instead — widening to all of `framework/` would
 * watch the entire monorepo's shared core. Cargo's own `target/` output lives at the repo root and built
 * wasm lands in the one staging root under `🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/`, so
 * widening the watch root here cannot cause a rebuild to re-trigger itself. */
function pluginWatchRoot(target: PluginRegistryEntry): string {
  const segments = target.cratePath.split("/");
  const topLevel = segments[0];
  if (topLevel === "🧰️framework" || topLevel === "framework") return join(repoRoot, target.cratePath);
  // 🏛️ Post-restructure: sibling crate families live under `✏️s/🔌️plugins/<p>/...` (was `s/plugin/<p>/...`).
  // Widening to `✏️s/` would watch every plugin's tree on every crate's edit.
  if ((topLevel === "✏️s" || topLevel === "s") && (segments[1] === "🔌️plugins" || segments[1] === "plugins")) {
    return join(repoRoot, segments.slice(0, 3).join("/"));
  }
  return join(repoRoot, topLevel);
}

/** @emoji 👀️ Rebuilds each of `targets` on source change — one `fs.watch` per crate (see
 * `pluginWatchRoot`) feeding a single dirty-set queue that drains serially. Two crates edited in quick
 * succession (or one crate touched again before its own rebuild finishes) used to fire overlapping
 * `void buildPlugin(...)` calls that raced each other against the same `target/` cargo lock; the dirty
 * set collapses any number of change events for one crate into a single pending rebuild, and the drain
 * loop only ever runs one `buildPlugin` at a time. Shared by both the standalone `plugin watch` command
 * and `DevScript`'s streaming boot, which folds this in right after the initial build pass so plugin
 * edits keep hot-swapping the running shell for the rest of the dev session. */
function watchPluginRebuilds(targets: readonly PluginRegistryEntry[]): void {
  const byPluginId = new Map(targets.map((target) => [target.pluginId, target] as const));
  const dirty = new Set<string>();
  let draining = false;

  async function drain(): Promise<void> {
    if (draining) return;
    draining = true;
    try {
      while (dirty.size > 0) {
        const [pluginId] = dirty;
        dirty.delete(pluginId!);
        const target = byPluginId.get(pluginId!);
        if (!target) continue;
        try {
          await buildPlugin(target);
        } catch (error) {
          console.error("program watch rebuild failed", error);
        }
      }
    } finally {
      draining = false;
    }
  }

  for (const target of targets) {
    watch(pluginWatchRoot(target), { recursive: true }, () => {
      dirty.add(target.pluginId);
      void drain();
    });
  }
  console.log("watching plugin crates for hot-swap rebuilds");
}

class PluginWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
    const filterPluginId = resolveCatalogFilterPluginId(filterPlugin || undefined);
    const catalogEntries = filterProjectedPluginRegistry(readGeneratedCatalogProjection(), filterPluginId);
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
    watchPluginRebuilds(targets);
  }
}

export { PluginWatchScript, pluginWatchRoot, watchPluginRebuilds };
