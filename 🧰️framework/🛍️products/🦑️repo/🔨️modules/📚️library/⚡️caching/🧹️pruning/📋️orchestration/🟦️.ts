import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import { acquireResourceLease } from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { CACHE_POLICY } from "../../🔍️discovery/📂️source/🟦️.ts";
import {compactCargoIncrementalSessionsV1, deleteUnit, formatBytes, planCachePrune, type PrunePlan } from "../🟦️.ts";
import {cargoDirectories} from "../../🦀️cargo/🟦️.ts";
import { areaUnitRoot, pruneTestEvidence, scanCacheAreas } from "../🌐️workspace/🟦️.ts";

/** 📊️ Dry-run visibility into the shared cache root: per-area sizes, unit counts, and exactly what `cache-prune` would delete. */
export class CacheReportScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const controller = new AbortController(),
      cancel = (): void => controller.abort(new Error("Cache report cancelled"));
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      const json = args.includes("--json");
      const areas = scanCacheAreas(this.repoRoot, controller.signal, json ? undefined : (area, unit) => console.log(`[cache-report] scanned ${area}/${unit.path}`));
      const plan = planCachePrune(areas, Date.now(), CACHE_POLICY.storage.guardAgeMs);
      if (json) {
        console.log(JSON.stringify(plan));
        return;
      }
      for (const area of plan.areas) {
        const ageBytes = area.ageDeletions.reduce((sum, unit) => sum + unit.bytes, 0),
          budgetBytes = area.budgetDeletions.reduce((sum, unit) => sum + unit.bytes, 0);
        console.log(
          `[cache-report] ${area.name}: ${formatBytes(area.totalBytes)} across ${area.unitCount} units; would delete ${area.ageDeletions.length} unused units (${formatBytes(ageBytes)}) and ${area.budgetDeletions.length} over-budget units (${formatBytes(budgetBytes)}); retains ${formatBytes(area.retainedBytes)}${area.guardedOverBudgetBytes ? `; ${formatBytes(area.guardedOverBudgetBytes)} over budget but within the guard age` : ""}`,
        );
      }
      console.log(`[cache-report] tests: ${await pruneTestEvidence(this.repoRoot, true)}`);
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}

/** 🧹️ Serializes explicit cleanup and applies age/budget policy; the age guard is not an active-build lock. */
export class CachePruneScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const controller = new AbortController(),
      cancel = (): void => controller.abort(new Error("Cache prune cancelled"));
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    const dryRun = args.includes("--dry-run");
    try {
      if(args.includes("--stale-incremental-only")){
        const index=args.indexOf("--profile"),profile=args[index+1];if(index<0||!profile||profile.startsWith("-")||args.some((arg,position)=>!["--stale-incremental-only","--dry-run","--profile"].includes(arg)&&position!==index+1))throw Error("Incremental compaction requires exactly one explicit Cargo profile");
        const result=await compactCargoIncrementalSessionsV1({buildDirectory:cargoDirectories(this.repoRoot).build,leaseDirectory:repoCacheDirectory(this.repoRoot,"agents/resource-leases"),profile,signal:controller.signal,dryRun,onUnit:unit=>console.log(`[DEBUG] Incremental compaction ${dryRun?"planned":"removing"} ${unit.path} bytes=${unit.bytes}`)});
        console.log(`[DEBUG] Incremental compaction result=${JSON.stringify(result)}`);return;
      }
      const lease = await acquireResourceLease({
        directory: repoCacheDirectory(this.repoRoot, "agents/resource-leases"),
        resource: "cache-prune",
        mode: "exclusive",
        signal: controller.signal,
        onWait: (wait) => console.log(`[cache-prune] waiting for exclusive access elapsedMs=${wait.elapsedMs}`),
      });
      try {
        const areas = scanCacheAreas(this.repoRoot, controller.signal, (area, unit) => console.log(`[cache-prune] scanned ${area}/${unit.path}`));
        const plan: PrunePlan = planCachePrune(areas, Date.now(), CACHE_POLICY.storage.guardAgeMs);
        let deletedBytes = 0,
          deletedCount = 0;
        for (const area of plan.areas) {
          for (const unit of [...area.ageDeletions, ...area.budgetDeletions]) {
            controller.signal.throwIfAborted();
            console.log(`[cache-prune] ${dryRun ? "would delete" : "deleting"} ${area.name}/${unit.path} (${formatBytes(unit.bytes)})`);
            if (!dryRun) deleteUnit(areaUnitRoot(this.repoRoot, area.name, unit), unit);
            deletedBytes += unit.bytes;
            deletedCount++;
          }
          if (area.guardedOverBudgetBytes) console.log(`[cache-prune] ${area.name}: ${formatBytes(area.guardedOverBudgetBytes)} over budget but touched within the last ${Math.round(CACHE_POLICY.storage.guardAgeMs / 3600000)}h; left in place`);
        }
        console.log(`[cache-prune] ${dryRun ? "would delete" : "deleted"} ${deletedCount} units; ${formatBytes(deletedBytes)}`);
        controller.signal.throwIfAborted();
        console.log(`[cache-prune] tests: ${await pruneTestEvidence(this.repoRoot, dryRun)}`);
      } finally {
        lease.release();
      }
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}
