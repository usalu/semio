import { BundleScript, getWorkspaceRoot, runCmd } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { readUiAxes } from "../📥️source/🟦️.ts";
import { uiAxesPreview, uiAxesTargets } from "../📋️plan/🟦️.ts";
import { publishUiAxes, staleUiAxesTargets } from "../📤️publication/🟦️.ts";

export class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const repoRoot = getWorkspaceRoot();
    process.stdout.write(`${JSON.stringify(uiAxesPreview(repoRoot, uiAxesTargets(repoRoot, readUiAxes(repoRoot))))}\n`);
  }
}

export class GenerateAxesScript extends BundleScript {
  run(): void {
    const repoRoot = getWorkspaceRoot();
    const axes = readUiAxes(repoRoot);
    publishUiAxes(uiAxesTargets(repoRoot, axes));
    console.log(`ui axes refreshed (${axes.locales.length} locales, ${axes.terminologies.length} terminologies)`);
  }
}

export class CheckAxesScript extends BundleScript {
  run(): void {
    const repoRoot = getWorkspaceRoot();
    const axes = readUiAxes(repoRoot);
    const stale = staleUiAxesTargets(uiAxesTargets(repoRoot, axes));
    if (stale.length > 0) throw new Error(`ui axes are stale: ${stale.join(", ")}`);
    runCmd("bun", ["test", "./🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts"], { cwd: repoRoot });
    console.log(`ui axes are fresh (${axes.locales.length} locales, ${axes.terminologies.length} terminologies).`);
  }
}
