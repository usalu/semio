/** 🧩️ Semantic plugin build watch owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { buildBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsPlaygroundDefaultPort, frameworkOsLockedPrefsEnv, resolveTestLevel, atTestLevel, cargoProfileDir, selectComponentWasmProfile, semioBuildMode, semioShipEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../../📇️registry/📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type DeployedRegistryEntryV1 } from "../../📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { buildPlugin, buildPlugins } from "../🏃️execution/🟦️.ts";

import { resolvePluginBuildTargets } from "../📋️plan/🟦️.ts";

import { resolveCatalogFilterPluginId } from "../📋️plan/🟦️.ts";



//#endregion 🪶️PluginSizeMeasurement

/** 👀️ Rebuilds each of `targets` on source change — admitted compilation and generator source inputs feeding a single dirty-set queue that drains serially. Two crates edited in quick
 * succession (or one crate touched again before its own rebuild finishes) used to fire overlapping
 * `void buildPlugin(...)` calls that raced each other against the same `target/` cargo lock; the dirty
 * set collapses any number of change events for one crate into a single pending rebuild, and the drain
 * loop only ever runs one `buildPlugin` at a time. Shared by both the standalone `plugin watch` command
 * and `DevScript`'s streaming boot, which folds this in right after the initial build pass so plugin
 * edits keep hot-swapping the running shell for the rest of the dev session. */
async function watchPluginRebuilds(targets: readonly DeployedRegistryEntryV1[]): Promise<void> {
  const { nativeSourceWatchPlanV1 } = await import("../../../../../🦑️repo/🔨️modules/📚️library/🟨️.mjs");
  const { startNativeSourceWatchV1, nativeSourceWatchSelectedV1 } = await import("./📋️plan/🟦️.ts");
  const plans = new Map<string, import("./📋️plan/🟦️.ts").NativeSourceWatchPlanV1>();
  let observation: { close(): Promise<void> } | undefined;
  const controller = new AbortController();
  const close = (): void => { controller.abort(); void observation?.close(); process.removeListener("SIGINT", close); process.removeListener("SIGTERM", close); };
  process.once("SIGINT", close); process.once("SIGTERM", close);
  const byPluginId = new Map(targets.map((target) => [target.pluginId, target] as const));
  const dirty = new Set<string>();
  let draining = false;

  async function drain(): Promise<void> {
    if (draining) return;
    draining = true;
    try {
      while (dirty.size > 0 && !controller.signal.aborted) {
        const [pluginId] = dirty;
        dirty.delete(pluginId!);
        const target = byPluginId.get(pluginId!);
        if (!target) continue;
        try {
          await buildPlugin(target);
          plans.set(target.pluginId, await nativeSourceWatchPlanV1(target.cratePath, repoRoot));
          await refresh();
        } catch (error) {
          console.error("program watch rebuild failed", error);
        }
      }
    } finally {
      draining = false;
    }
  }

  const refresh = async (): Promise<void> => {
    const inputs = [...plans.values()], prior = observation;
    const combined = { schema: "semio.framework.os.plugin.source-watch-plan/v1" as const, includes: [...new Set(inputs.flatMap(plan => plan.includes))], excludes: [], files: [...new Set(inputs.flatMap(plan => plan.files))] };
    observation = await startNativeSourceWatchV1(repoRoot, combined, path => {
      for (const [id, plan] of plans) if (nativeSourceWatchSelectedV1(plan, path)) dirty.add(id);
      void drain();
    }, { signal: controller.signal, selected: path => [...plans.values()].some(plan => nativeSourceWatchSelectedV1(plan, path)) });
    await prior?.close();
  };
  try {
    for (const target of targets) plans.set(target.pluginId, await nativeSourceWatchPlanV1(target.cratePath, repoRoot));
    if (plans.size) await refresh();
  } catch (error) { close(); await observation?.close(); throw error; }
  console.log("watching plugin crates for hot-swap rebuilds");
}

class PluginWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
    const filterPluginId = resolveCatalogFilterPluginId(filterPlugin || undefined);
    const catalogEntries = filterProjectedPluginRegistry(readGeneratedCatalogProjection(), filterPluginId);
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
    await watchPluginRebuilds(targets);
  }
}

export { PluginWatchScript, watchPluginRebuilds };
