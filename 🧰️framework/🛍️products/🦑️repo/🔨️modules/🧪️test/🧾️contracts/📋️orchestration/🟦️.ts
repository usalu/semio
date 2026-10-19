import { selectCases } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import { runPhases } from "../../⚖️parity/📋️orchestration/🟦️.ts";
import { discoverTestCases, testLayoutBreaches, validateAllContracts,testCacheDir } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { type BreachRecord, formatBreachReport } from "@semio-tech/repo-lib/policy";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🔍️ Lists every discovered case as JSON — the same list Nx generates projects from. */
export class DiscoverScript extends Script {
  run(segments: string[]): void {
    const cases = discoverTestCases(this.repoRoot);
    if (segments.includes("--json")) {
      console.log(JSON.stringify(cases, null, 2));
      return;
    }
    for (const entry of cases) console.log(`${entry.projectName}\t${entry.caseDir}\t[${Object.keys(entry.adapters).join(",") || "no-adapter"}]`);
    console.log(`[discover] ${cases.length} test case(s)`);
  }
}

/** 🧾️ The contract phase — everything provable without executing a test. */
export class ContractScript extends Script {
  async run(segments: string[]): Promise<void> {
    const cases = selectCases(this.repoRoot, segments);
    const breaches: BreachRecord[] = [...validateAllContracts(this.repoRoot, cases), ...(await testLayoutBreaches(this.repoRoot))];
    const directory=testCacheDir(this.repoRoot,"reports"),cachePath=join(directory,"testing.json");
    mkdirSync(directory, { recursive: true });
    writeFileSync(cachePath, `${JSON.stringify(breaches, null, 2)}\n`);
    console.log(formatBreachReport(breaches, cachePath));
    if (breaches.length > 0) process.exitCode=1;
  }
}

/** ▶️ The default phase chain for a level: contract, then full parity. */
export class RunScript extends Script {
  async run(segments: string[]): Promise<void> {
    const cases = selectCases(this.repoRoot, segments);
    const breaches = [...validateAllContracts(this.repoRoot, cases), ...(await testLayoutBreaches(this.repoRoot))];
    if (breaches.length > 0) {
      console.error(formatBreachReport(breaches, "(not cached — contract phase failed inside run)"));
      process.exitCode=1;return;
    }
    process.exitCode=await runPhases(this.repoRoot, segments, ["oracle", "subject"],this.invocation);
  }
}
