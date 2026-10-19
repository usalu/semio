import { checkResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel, atTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic plugin catalog refresh owner. */

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import type { PluginBuildTargetV1 } from "../🔎️discovery/🟦️.ts";

const repoRoot = findWorkspaceRoot();

import { createConcurrencyLimiter, cargoConcurrencyLimit, materializeConcurrencyLimit } from "../../🏗️build/🏃️execution/🟦️.ts";



import type { ComponentDeliverableV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";



/** 🚰️ Builds a whole catalog of `orderedTargets`: cargo and materialize each run in bounded pools
 * sized by `cargoConcurrencyLimit()` / `materializeConcurrencyLimit()` (default `semioBuildParallel()`), with
 * materialize work enqueued as each target's cargo build finishes so both stages overlap across the catalog.
 * `publishShardWorker()` (identical content for every target) is written once at the end rather than once
 * per target. Injectable fakes support tests without a real toolchain. */
async function buildPluginCatalog(
  orderedTargets: readonly PluginBuildTargetV1[],
  control: CompilerAdmissionControl,
  cargoFn: (target: PluginBuildTargetV1) => Promise<{ readonly deliverable: ComponentDeliverableV1 }>,
  materializeFn: (target: PluginBuildTargetV1, built: { readonly deliverable: ComponentDeliverableV1 }) => Promise<void>,
  concurrencyLimit: number,
  publishShardWorkerFn: () => void,
  cargoLimit: number,
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
  const {runtimeCatalogProjectionRootV1}=await import("../📖️catalog-view/🟦️.ts");
  const root=runtimeCatalogProjectionRootV1();if(root===undefined)return;
  const {readFileSync}=await import("node:fs");
  const {parseCatalogPublicationV1}=await import("../📣️publication/🟦️.ts");
  const publication=parseCatalogPublicationV1(JSON.parse(readFileSync(join(root,"📣️publication.json"),"utf8")));
  if(resolve(repoRoot,publication.outputRoot)!==resolve(root))throw Error("Supplied catalog publication differs from its output root");
  const {renderCatalogFiles,publishCatalogFilesV1}=await import("../📽️projection/🟦️.ts");
  const result=renderCatalogFiles(repoRoot,undefined,"exclude",publication);
  publishCatalogFilesV1(root,result.files);
}


export { assertPluginCatalogComplete, buildPluginCatalog };
