/** 🧩️ Semantic plugin catalog refresh owner. */

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

import { filterProjectedPluginRegistry, readGeneratedCatalogProjection } from "../📖️catalog-view/🟦️.ts";

import { generatePluginRegistry, type PluginRegistryEntry } from "../🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { createConcurrencyLimiter, materializeConcurrencyLimit } from "../../🏗️build/🏃️execution/🟦️.ts";

import { resolveCatalogFilterPluginId } from "../../🏗️build/📋️plan/🟦️.ts";

import { buildPluginCargo, materializePlugin, publishShardWorker } from "../../🏗️build/📦️materialization/🟦️.ts";

import { syncBuiltPluginDescriptors } from "../../🏗️build/🛂️descriptor/🟦️.ts";



/** @emoji 🚰️ Builds a whole catalog of `orderedTargets`: the CARGO stage runs strictly serially, one
 * `cargo build` at a time, in `orderedTargets`' own order — never two overlapping, exactly as before
 * this packet, since parallel `cargo` is the repeatedly-machine-saturating failure mode
 * `📌️important.md` records. The MATERIALIZE stage for each target that finished its cargo build is
 * enqueued into a bounded pool (`materializeConcurrencyLimit()`, default 4) WITHOUT the cargo loop
 * waiting for it — so target N+1's `cargo build` runs concurrently with target N's (and N-1's, up to the
 * cap) jco/wasm-opt/file-emission pass, instead of the old fully-interleaved `buildPlugin` forcing every
 * cargo build to wait out the previous target's ENTIRE materialize pass first. This is the actual fix
 * for the serialized-materialize-stage finding: overlap, not just "run materialize in parallel with
 * itself". `publishShardWorker()` (identical content for every target) is written once at the end
 * rather than once per target. Injectable `cargoFn`/`materializeFn`/`publishShardWorkerFn` so this can
 * be exercised in tests without a real `cargo`/`jco`/`wasm-opt` toolchain or filesystem writes. */
async function buildPluginCatalog(
  orderedTargets: readonly PluginRegistryEntry[],
  cargoFn: (target: PluginRegistryEntry) => Promise<{ readonly artifact: string }> = buildPluginCargo,
  materializeFn: (target: PluginRegistryEntry, artifact: string) => Promise<void> = materializePlugin,
  concurrencyLimit: number = materializeConcurrencyLimit(),
  publishShardWorkerFn: () => void = publishShardWorker,
): Promise<{ readonly failedPluginIds: readonly string[] }> {
  const limiter = createConcurrencyLimiter(concurrencyLimit);
  const failed: string[] = [];
  const materializeTasks: Promise<void>[] = [];
  for (const target of orderedTargets) {
    let cargoResult: { readonly artifact: string };
    try {
      cargoResult = await cargoFn(target);
    } catch (error) {
      failed.push(target.pluginId);
      console.error(`plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
      continue;
    }
    const { artifact } = cargoResult;
    materializeTasks.push(
      limiter.run(async () => {
        try {
          await materializeFn(target, artifact);
        } catch (error) {
          failed.push(target.pluginId);
          console.error(`plugin materialize failed: ${target.pluginId}`, error);
        }
      }),
    );
  }
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
