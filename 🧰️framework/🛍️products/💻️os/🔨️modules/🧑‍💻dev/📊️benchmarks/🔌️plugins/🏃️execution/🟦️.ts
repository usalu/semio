import { resolveTestLevel, atTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic benchmark execution owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const repoRoot = findWorkspaceRoot();

import { WGPU_SCRIPT_PATH } from "@semio-tech/framework-plugin/build/plan";

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
