import { cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { readUiAxes } from "../📥️source/🟦️.ts";
import { uiAxesPreview, uiAxesTargets } from "../📋️plan/🟦️.ts";
import { publishUiAxes, staleUiAxesTargets } from "../📤️publication/🟦️.ts";

export class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const repoRoot = this.repoRoot;
    process.stdout.write(`${JSON.stringify(uiAxesPreview(repoRoot, uiAxesTargets(repoRoot, readUiAxes(repoRoot))))}\n`);
  }
}

export class GenerateAxesScript extends BundleScript {
  run(): void {
    const repoRoot = this.repoRoot;
    const axes = readUiAxes(repoRoot);
    publishUiAxes(uiAxesTargets(repoRoot, axes));
    console.log(`ui axes refreshed (${axes.locales.length} locales, ${axes.terminologies.length} terminologies)`);
  }
}

export class CheckAxesScript extends BundleScript {
  async run(): Promise<void> {
    const repoRoot = this.repoRoot;
    const axes = readUiAxes(repoRoot);
    const stale = staleUiAxesTargets(uiAxesTargets(repoRoot, axes));
    if (stale.length > 0) throw new Error(`ui axes are stale: ${stale.join(", ")}`);
    await runOwnedCommand(process.execPath, ["test", "./🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts"], repoRoot, "ui-axes", cmdBudgetMs(), { env: process.env });
    console.log(`ui axes are fresh (${axes.locales.length} locales, ${axes.terminologies.length} terminologies).`);
  }
}
