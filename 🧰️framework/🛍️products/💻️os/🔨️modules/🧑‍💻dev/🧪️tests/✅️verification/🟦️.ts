/** 🧩️ Semantic verification execution owner. */

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { buildBudgetMs, daemonBudgetOpts, describeDevPortOccupant, devServerUrl, getWorkspaceRoot, getRepoMetaDir, isDevPortInUse, loadFrameworkOsPlaygroundCatalog, wgpuDevPlayUrl, runCmd, runCmdStatus, runBunxStatus, runNodeBinStatus, runProbe, runVitest, spawnDaemon, type SpawnDaemonHandle, runViteBunxDev, frameworkOsPlaygroundDefaultPort, frameworkOsLockedPrefsEnv, resolveTestLevel, atTestLevel, cargoProfileDir, selectComponentWasmProfile, semioBuildMode, semioShipEnv } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { generatePluginRegistry, type DeployedRegistryEntryV1 } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { readPackageName } from "../../../🔌️plugin/🏗️build/📦️materialization/🟦️.ts";

import { runHomeE2eCli, runStudioE2eVerify } from "../🎬️studio/🟦️.ts";

import { CATALOG_SMOKE_DEFAULT_OUT_REL, catalogSmokeExitCode, runCatalogSmokeVerify } from "../🔬️catalog-smoke/🟦️.ts";


import { runProgramMatrixCli } from "../🧮️program-matrix/🟦️.ts";

import { runTwoHumanCli } from "../👥️two-human/🟦️.ts";

import { runToolRunMatrixCli } from "../⏯️tool-run-matrix/🟦️.ts";

import { runHubDocumentSweepCli } from "../🗂️hub-document-sweep/🟦️.ts";

import { runIoMatrixCli } from "../🚪️io-matrix/🟦️.ts";

import { runConnectionBudgetCli } from "../🔀️connection-budget/🟦️.ts";

import { runIdleBudgetCli } from "../💤️idle-budget/🟦️.ts";

import { runInteractionLatencyCli } from "../⏱️interaction-latency/🟦️.ts";

import { runBootBudgetCli } from "../🥾️boot-budget/🟦️.ts";

import { PluginCapabilityLintScript } from "../🧹️capability-policy/🟦️.ts";



//#endregion 🔖️CollabE2e

class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const port = process.env.S_OS_PORT ?? "6070";
    const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
    const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);
    if (segments[0] === "hub-sweep") {
      await runHubDocumentSweepCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🗂️hub-document-sweep"), segments.slice(1));
      return;
    }
    if (segments[0] === "io") {
      await runIoMatrixCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🚪️io-matrix"), segments.slice(1));
      return;
    }
    if (segments[0] === "home") {
      await runHomeE2eCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🏠️home-e2e"), segments.slice(1));
      return;
    }
    if (segments[0] === "two-human") {
      await runTwoHumanCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/👥️two-human"), segments.slice(1));
      return;
    }
    if (segments[0] === "latency") {
      await runInteractionLatencyCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/⏱️interaction-latency"), segments.slice(1));
      return;
    }
    if (segments[0] === "boot") {
      await runBootBudgetCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🥾️boot-budget"), segments.slice(1));
      return;
    }
    if (segments[0] === "idle") {
      await runIdleBudgetCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/💤️idle-budget"), segments.slice(1));
      return;
    }
    if (segments[0] === "connections") {
      await runConnectionBudgetCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🔀️connection-budget"), segments.slice(1));
      return;
    }
    if (segments[0] === "tool-run" || (segments[0] === "matrix" && segments[segments.indexOf("--column") + 1] === "tool-run" && segments.includes("--column"))) {
      const rest = segments.slice(1).filter((segment, index, all) => segment !== "--column" && all[index - 1] !== "--column");
      await runToolRunMatrixCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/⏯️tool-run-matrix"), rest);
      return;
    }
    if (segments[0] === "matrix") {
      await runProgramMatrixCli(repoRoot, join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🧮️program-matrix"), segments.slice(1));
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
    await new PluginCapabilityLintScript(this.root).run();
    console.log(`s studio verify passed (${studioUrl})`);
  }
}

export { VerifyScript };
