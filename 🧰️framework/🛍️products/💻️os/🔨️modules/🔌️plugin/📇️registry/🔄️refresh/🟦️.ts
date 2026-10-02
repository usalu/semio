import { resolveTestLevel, atTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 🧩️ Semantic plugin catalog refresh owner. */

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsPlaygroundDefaultPort, frameworkOsLockedPrefsEnv, cargoProfileDir, selectComponentWasmProfile, semioBuildMode, semioShipEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type DeployedRegistryEntryV1 } from "../🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { createConcurrencyLimiter, cargoConcurrencyLimit, materializeConcurrencyLimit } from "../../🏗️build/🏃️execution/🟦️.ts";

import { resolveCatalogFilterPluginId } from "../../🏗️build/📋️plan/🟦️.ts";

import { buildPluginCargo, materializePlugin, publishShardWorker } from "../../🏗️build/📦️materialization/🟦️.ts";

import { syncBuiltPluginDescriptors } from "../../🏗️build/🛂️descriptor/🟦️.ts";



/** 🚰️ Builds a whole catalog of `orderedTargets`: cargo and materialize each run in bounded pools
 * sized by `cargoConcurrencyLimit()` / `materializeConcurrencyLimit()` (default `semioNxParallel()`), with
 * materialize work enqueued as each target's cargo build finishes so both stages overlap across the catalog.
 * `publishShardWorker()` (identical content for every target) is written once at the end rather than once
 * per target. Injectable fakes support tests without a real toolchain. */
async function buildPluginCatalog(
  orderedTargets: readonly DeployedRegistryEntryV1[],
  cargoFn: (target: DeployedRegistryEntryV1) => Promise<{ readonly artifact: string }> = buildPluginCargo,
  materializeFn: (target: DeployedRegistryEntryV1, artifact: string) => Promise<void> = materializePlugin,
  concurrencyLimit: number = materializeConcurrencyLimit(),
  publishShardWorkerFn: () => void = publishShardWorker,
  cargoLimit: number = cargoConcurrencyLimit(),
): Promise<{ readonly failedPluginIds: readonly string[] }> {
  const cargoLimiter = createConcurrencyLimiter(cargoLimit);
  const materializeLimiter = createConcurrencyLimiter(concurrencyLimit);
  const failed: string[] = [];
  const materializeTasks: Promise<void>[] = [];
  await Promise.all(orderedTargets.map((target) => cargoLimiter.run(async () => {
    let cargoResult: { readonly artifact: string };
    try {
      cargoResult = await cargoFn(target);
    } catch (error) {
      failed.push(target.pluginId);
      console.error(`plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
      return;
    }
    const { artifact } = cargoResult;
    materializeTasks.push(
      materializeLimiter.run(async () => {
        try {
          await materializeFn(target, artifact);
        } catch (error) {
          failed.push(target.pluginId);
          console.error(`plugin materialize failed: ${target.pluginId}`, error);
        }
      }),
    );
  })));
  await Promise.all(materializeTasks);
  publishShardWorkerFn();
  return { failedPluginIds: failed };
}

/** 🛑 Rejects incomplete explicit builds after every target has been attempted. */
function assertPluginCatalogComplete(failedPluginIds: readonly string[]): void {
  if (failedPluginIds.length > 0) throw new Error(`plugin catalog build failed: ${failedPluginIds.join(", ")}`);
}

export async function ensurePluginRegistry(filterPlugin?: string): Promise<void> {
  const registryScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts");
  if (runCmdStatus("bun", [registryScript, "generate"], { cwd: repoRoot }) !== 0) throw new Error("plugin registry generation failed");
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  syncBuiltPluginDescriptors(filterProjectedPluginRegistry(readGeneratedCatalogProjection(), filterPluginId));
}

export { assertPluginCatalogComplete, buildPluginCatalog };
