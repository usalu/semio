import { checkResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel, atTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic plugin catalog refresh owner. */

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsPlaygroundDefaultPort, frameworkOsLockedPrefsEnv, cargoProfileDir, semioBuildMode, semioReleaseEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import type { PluginBuildTargetV1 } from "../🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { createConcurrencyLimiter, cargoConcurrencyLimit, materializeConcurrencyLimit } from "../../🏗️build/🏃️execution/🟦️.ts";

import { buildPluginCargo, materializePlugin, publishShardWorker } from "../../🏗️build/📦️materialization/🟦️.ts";

import type { ComponentDeliverableV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";



/** 🚰️ Builds a whole catalog of `orderedTargets`: cargo and materialize each run in bounded pools
 * sized by `cargoConcurrencyLimit()` / `materializeConcurrencyLimit()` (default `semioNxParallel()`), with
 * materialize work enqueued as each target's cargo build finishes so both stages overlap across the catalog.
 * `publishShardWorker()` (identical content for every target) is written once at the end rather than once
 * per target. Injectable fakes support tests without a real toolchain. */
async function buildPluginCatalog(
  orderedTargets: readonly PluginBuildTargetV1[],
  control: CompilerAdmissionControl,
  cargoFn: (target: PluginBuildTargetV1) => Promise<{ readonly deliverable: ComponentDeliverableV1 }> = target => buildPluginCargo(target, control),
  materializeFn: (target: PluginBuildTargetV1, built: { readonly deliverable: ComponentDeliverableV1 }) => Promise<void> = materializePlugin,
  concurrencyLimit: number = materializeConcurrencyLimit(),
  publishShardWorkerFn: () => void = publishShardWorker,
  cargoLimit: number = cargoConcurrencyLimit(),
): Promise<{ readonly failedPluginIds: readonly string[] }> {
  checkResourceLeaseControl(control);
  const cargoLimiter = createConcurrencyLimiter(cargoLimit);
  const materializeLimiter = createConcurrencyLimiter(concurrencyLimit);
  const failed: string[] = [];
  const materializeTasks: Promise<void>[] = [];
  await Promise.all(orderedTargets.map((target) => cargoLimiter.run(async () => {
    let cargoResult: { readonly deliverable: ComponentDeliverableV1 };
    try {
      checkResourceLeaseControl(control);
      cargoResult = await cargoFn(target);
      checkResourceLeaseControl(control);
    } catch (error) {
      checkResourceLeaseControl(control);
      failed.push(target.pluginId);
      console.error(`plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
      return;
    }
    materializeTasks.push(
      materializeLimiter.run(async () => {
        try {
          checkResourceLeaseControl(control);
          await materializeFn(target, cargoResult);
          checkResourceLeaseControl(control);
        } catch (error) {
          checkResourceLeaseControl(control);
          failed.push(target.pluginId);
          console.error(`plugin materialize failed: ${target.pluginId}`, error);
        }
      }),
    );
  })));
  await Promise.all(materializeTasks);
  checkResourceLeaseControl(control);
  publishShardWorkerFn();
  return { failedPluginIds: failed };
}

/** 🛑 Rejects incomplete explicit builds after every target has been attempted. */
function assertPluginCatalogComplete(failedPluginIds: readonly string[]): void {
  if (failedPluginIds.length > 0) throw new Error(`plugin catalog build failed: ${failedPluginIds.join(", ")}`);
}

/** 🗂️ Regenerates the plugin registry from the committed semantic descriptors before a dev build selects its targets. */
export async function ensurePluginRegistry(): Promise<void> {
  const registryScript = join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts");
  if (runCmdStatus("bun", [registryScript, "generate"], { cwd: repoRoot }) !== 0) throw new Error("plugin registry generation failed");
}

export { assertPluginCatalogComplete, buildPluginCatalog };
