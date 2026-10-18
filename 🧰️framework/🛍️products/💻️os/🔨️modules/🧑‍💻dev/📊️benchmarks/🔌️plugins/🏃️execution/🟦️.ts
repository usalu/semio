import { resolveTestLevel, atTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic benchmark execution owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { daemonBudgetOpts, runCmd, runCmdStatus, runNodeBinStatus, cargoProfileDir, semioBuildMode, semioReleaseEnv } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { describeDevPortOccupant, devServerUrl, isDevPortInUse } from "../../../../../../🦑️repo/🔨️modules/📚️library/🌐️network/🔌️ports/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { getRepoMetaDir } from "../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🧬️metadata/🟦️.ts";
import { loadFrameworkOsPlaygroundCatalog } from "../../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts";
import { wgpuDevPlayUrl } from "../../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🧊️wgpu-dev/🟦️.ts";
import { runBunxStatus } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🍞️bun/🟦️.ts";
import { runProbe } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🔍️probe/🟦️.ts";
import { runVitest } from "../../../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts";
import { spawnDaemon, type SpawnDaemonHandle } from "../../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/👹️daemon/🟦️.ts";
import { runViteBunxDev } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts";
import { frameworkOsLockedPrefsEnv } from "../../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { WGPU_SCRIPT_PATH } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { benchWebRows } from "../🌐️browser/🟦️.ts";

import { BENCH_BUDGETS, benchFlag, benchOutDir, benchRegistryRow } from "../📋️plan/🟦️.ts";

import { benchNativeRows } from "../🖥️host/🟦️.ts";

import { renderScaleFixtureArtifacts } from "@semio-tech/framework-os/testing/scale/projection";



//#endregion 🧪️BenchWebRows

class BenchPluginsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [renderer, ...segments] = args;
    if ((renderer!=="native"&&renderer!=="react"&&renderer!=="wgpu") || segments.some((segment) => segment === "--renderer" || segment.startsWith("--renderer="))) throw new Error("Select the renderer through the benchmark build target");
    const pluginCount = Number(benchFlag(segments, "count", "50"));
    const extensionsPerPlugin = Number(benchFlag(segments, "extensions", "50"));
    const shardCount = Number(benchFlag(segments, "shards", "8"));
    const outDir = benchOutDir(renderer);
    const outPath = benchFlag(segments, "out", join(outDir, `terra-v1b-bench-${renderer}.json`));

    const registryPath = join(outDir, "🔣️bench-registry.json");
    // 🔁️ Reuses `renderScaleFixtureArtifacts` verbatim (`//#region 🔖️ScaleFixture` above) — same
    // deterministic generator the committed `🤖️generated/📇️registry/🔣️.json` comes from, just scoped to
    // THIS run's `--count`/`--extensions` rather than whatever happens to be checked in.
    const { registryJson, registry } = renderScaleFixtureArtifacts(pluginCount, extensionsPerPlugin, 1);
    writeFileSync(registryPath, registryJson);

    const rows: Record<string, unknown>[] = [benchRegistryRow(registryPath, registry.recordCount)];

    if (renderer === "native") {
      rows.push(...benchNativeRows(repoRoot, outDir, registryPath, shardCount));
    } else if (renderer === "react" || renderer === "wgpu") {
      console.log(`bench: running web scale-bench (renderer=${renderer}, shards=${shardCount}) via headless Chromium — see 📊️bench-web-harness/🟦️.ts for real-vs-stub scope`);
      rows.push(...(await benchWebRows(BENCH_BUDGETS.slice(1), renderer, registry, shardCount)));
    } else {
      throw new Error(`bench plugins: unknown --renderer ${renderer} (expected native|react|wgpu)`);
    }

    const report = { renderer, pluginCount, extensionsPerPlugin, shardCount, seed: 1, generatedAt: new Date().toISOString(), budgets: rows };
    mkdirSync(dirname(outPath), { recursive: true });
    writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
    console.log(`bench: wrote report -> ${outPath}`);
    console.log(`bench summary: ${rows.map((r) => `${(r as { id: number }).id}:${(r as { status: string }).status}`).join(" ")}`);
  }
}

export { BenchPluginsScript };
