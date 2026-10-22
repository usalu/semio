import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧩️ Semantic benchmark host owner. */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { runCmdStatus } from "@semio-tech/process/execution";
import { WGPU_SCRIPT_PATH } from "@semio-tech/framework-plugin/build/plan";
import { runtimeExecutableV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏷️identity/🟦️.ts";

const SCALE_COMPONENT_ARTIFACT = "🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📦️packages/🦀️rust/dist/component/semio_framework_os_scale_fixture.wasm";

function benchNativeRows(repoRoot: string, outDir: string, registryPath: string, shardCount: number): Record<string, unknown>[] {
  const wasmPath = join(repoRoot, SCALE_COMPONENT_ARTIFACT);
  if (!existsSync(wasmPath)) throw new Error(`bench: expected wasm artifact missing: ${wasmPath}`);
  const nativeReportPath = join(outDir, "🔣️bench-native-raw.json");
  console.log(`bench: running native scale-bench (shards=${shardCount})`);
  const status = runCmdStatus(runtimeExecutableV1(), [WGPU_SCRIPT_PATH, "native", "dev", "--scale", registryPath, "--scale-wasm", wasmPath, "--shards", String(shardCount), "--report", nativeReportPath], { cwd: repoRoot, env: process.env, budgetMs: buildBudgetMs() });
  if (status !== 0) throw new Error("bench: native scale-bench run failed");
  return (JSON.parse(readFileSync(nativeReportPath, "utf8")) as { budgets: Record<string, unknown>[] }).budgets;
}

export { SCALE_COMPONENT_ARTIFACT, benchNativeRows };
