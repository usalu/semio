import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { artifactBudgets, measureArtifactRegistry } from "../../📦️artifacts/📇️registry/🟦️.ts";
import { inventory } from "../../📇️inventory/🧮️composition/🟦️.ts";
import { ticketOutput } from "../../🎫️output/🟦️.ts";

export class DiskScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const output = ticketOutput(this.repoRoot, args);
    const controller = new AbortController(),
      cancel = (): void => controller.abort(new Error("Storage accounting cancelled"));
    process.once("SIGINT", cancel);
    process.once("SIGTERM", cancel);
    try {
      const registry = inventory(this.repoRoot).artifactRegistry;
      const result = await measureArtifactRegistry(this.repoRoot, registry, { signal: controller.signal, onProgress: (progress) => console.log(`[nx-disk] measured ${progress.files} files; ${progress.path}`) });
      const budgets = Object.entries(artifactBudgets).map(([group, budget]) => {
        const allocated = result.entries.filter((entry) => entry.retention.budgetGroup === group).reduce<number | null>((total, entry) => (total === null || entry.allocated === null ? null : total + entry.allocated), 0);
        return { group, budget, allocated, overBudget: allocated === null ? null : allocated > budget };
      });
      writeFileSync(join(output, "disk.json"), JSON.stringify({ ...result, budgets, findings: registry.findings }, null, 2) + "\n");
      const allocation = (value: number | null): string => (value === null ? "allocation unavailable" : `${(value / 1024 ** 3).toFixed(2)} GiB allocated`);
      for (const entry of result.entries.filter((entry) => entry.present && entry.files > 0)) console.log(`[nx-disk] ${entry.owner}: ${allocation(entry.allocated)}; ${entry.path}`);
      console.log(`[nx-disk] ${result.complete ? "Complete" : "Incomplete"}: ${result.totals.files} unique files; ${allocation(result.totals.allocated)}`);
      if (!result.complete) throw new Error(`Storage accounting is incomplete: ${result.errors.length} read errors; ${registry.findings.length} ownership violations`);
    } finally {
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
  }
}
