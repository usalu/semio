/** 🧩️ Semantic package test execution owner. */

import { buildBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsPlaygroundDefaultPort, frameworkOsLockedPrefsEnv, resolveTestLevel, atTestLevel, cargoProfileDir, selectComponentWasmProfile, semioBuildMode, semioShipEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";



//#endregion 🔖️HostHandleReachLint

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest[0] === "dev-contribution") return runVitest(this.root, ["../../🧪️tests/🧩️contribution/🟦️.ts", ...rest.slice(1)], "../../🧪️tests/🎚️config/🟦️.ts");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

export { TestScript };
