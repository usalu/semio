/** 🧩️ Semantic verification execution owner. */

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

import { generatePluginRegistry, type PluginRegistryEntry } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { readPackageName } from "../../../🔌️plugin/🏗️build/📦️materialization/🟦️.ts";

import { runStudioE2eVerify } from "../🎬️studio/🟦️.ts";

import { CATALOG_SMOKE_DEFAULT_OUT_REL, catalogSmokeExitCode, runCatalogSmokeVerify } from "../🔬️catalog-smoke/🟦️.ts";

import { runCollabE2eVerify } from "../🤝️collaboration/🟦️.ts";

import { PluginCapabilityLintScript } from "../🧹️capability-policy/🟦️.ts";



//#endregion 🔖️CollabE2e

class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const port = process.env.S_OS_PORT ?? "6070";
    const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
    const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);
    if (segments[0] === "collab") {
      await runCollabE2eVerify();
      return;
    }
    if (segments[0] === "e2e") {
      await runStudioE2eVerify(studioUrl, timeoutMs);
      console.log(`s studio e2e verify passed (${studioUrl})`);
      return;
    }
    if (segments[0] === "catalog") {
      const outIndex = segments.indexOf("--out");
      const outDir = outIndex >= 0 && segments[outIndex + 1] ? resolve(segments[outIndex + 1]!) : join(repoRoot, CATALOG_SMOKE_DEFAULT_OUT_REL);
      const report = await runCatalogSmokeVerify(studioUrl, { outDir, timeoutMs, perProgramMs: Number(process.env.S_CATALOG_SMOKE_PROGRAM_MS ?? 30_000) });
      const exit = catalogSmokeExitCode(report);
      if (exit !== 0) throw new Error(`catalog smoke failed: ${report.boot.failure ?? "shell booted"}; ${report.totals.pass} rendered, ${report.totals.fail} did not, ${report.failedPlugins.length} plugin(s) in a failed install status`);
      console.log(`s catalog smoke passed (${report.totals.pass} programs, ${studioUrl})`);
      return;
    }
    for (const target of generatePluginRegistry(repoRoot)) {
      const packageName = await readPackageName(target.cratePath);
      if (runCmdStatus("cargo", ["test", "--lib", "-p", packageName], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`${packageName} tests failed`);
    }
    if (runBunxStatus(["vitest", "run"], join(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript")) !== 0) throw new Error("framework-renderer-react tests failed");
    await runStudioE2eVerify(studioUrl, timeoutMs);
    await new PluginCapabilityLintScript(this.root).run([]);
    console.log(`s studio verify passed (${studioUrl})`);
  }
}

export { VerifyScript };
