/** 🧩️ Semantic benchmark host owner. */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { buildBudgetMs, runCmdStatus } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { WGPU_SCRIPT_PATH } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";

const SCALE_COMPONENT_ARTIFACT = "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm";

function benchNativeRows(repoRoot: string, outDir: string, registryPath: string, shardCount: number): Record<string, unknown>[] {
  const wasmPath = join(repoRoot, SCALE_COMPONENT_ARTIFACT);
  if (!existsSync(wasmPath)) throw new Error(`bench: expected wasm artifact missing: ${wasmPath}`);
  const nativeReportPath = join(outDir, "🔣️bench-native-raw.json");
  console.log(`bench: running native scale-bench (shards=${shardCount})`);
  const status = runCmdStatus("bun", [WGPU_SCRIPT_PATH, "native", "dev", "--scale", registryPath, "--scale-wasm", wasmPath, "--shards", String(shardCount), "--report", nativeReportPath], { cwd: repoRoot, env: process.env, budgetMs: buildBudgetMs() });
  if (status !== 0) throw new Error("bench: native scale-bench run failed");
  return (JSON.parse(readFileSync(nativeReportPath, "utf8")) as { budgets: Record<string, unknown>[] }).budgets;
}

export { SCALE_COMPONENT_ARTIFACT, benchNativeRows };
