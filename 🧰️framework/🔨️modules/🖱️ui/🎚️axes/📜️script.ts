import { writeGeneratedFileIfChanged } from "../../🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { cmdBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { BundleScript } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readUiAxes } from "./📥️source/🟦️.ts";
import { uiAxesPreview, uiAxesTargets } from "./📋️plan/🟦️.ts";
import { publishUiAxes, staleUiAxesTargets } from "./📤️publication/🟦️.ts";

/** 🔭️ Prints the planned UI axis projections without writing them. */
export class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const repoRoot = this.repoRoot;
    process.stdout.write(`${JSON.stringify(uiAxesPreview(repoRoot, uiAxesTargets(repoRoot, readUiAxes(repoRoot))))}\n`);
  }
}

/** 🏭️ Refreshes the UI axis projections. */
export class GenerateAxesScript extends BundleScript {
  run(): void {
    const repoRoot = this.repoRoot;
    const axes = readUiAxes(repoRoot);
    publishUiAxes(uiAxesTargets(repoRoot, axes), writeGeneratedFileIfChanged);
    console.log(`ui axes refreshed (${axes.locales.length} locales, ${axes.terminologies.length} terminologies)`);
  }
}

/** ✅️ Fails on stale UI axis projections, then runs the axes laws. */
export class CheckAxesScript extends BundleScript {
  async run(): Promise<void> {
    const repoRoot = this.repoRoot;
    const axes = readUiAxes(repoRoot);
    const stale = staleUiAxesTargets(uiAxesTargets(repoRoot, axes));
    if (stale.length > 0) throw new Error(`ui axes are stale: ${stale.join(", ")}`);
    await runBudgetedTestCommand(process.execPath, ["test", "./🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts"], { cwd: repoRoot, budgetMs: cmdBudgetMs(), env: process.env, throwOnFailure: true });
    console.log(`ui axes are fresh (${axes.locales.length} locales, ${axes.terminologies.length} terminologies).`);
  }
}
