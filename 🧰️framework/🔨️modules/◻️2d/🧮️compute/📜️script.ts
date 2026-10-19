#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand } from "../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || !["schema", "ownership", "neutral-ownership"].includes(segments[0]!)) throw new Error("Expected test schema or test ownership");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, segments[0] === "schema" ? "🧪️tests/🟦️.ts" : segments[0] === "ownership" ? "🧪️tests/📍️ownership/🟦️.ts" : "🧪️tests/📍️ownership/🧭️direction/🟦️.ts")], { cwd: this.root, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "types") throw new Error("Expected check types");
    await runOwnedCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], this.root, "compute-contract-types", cmdBudgetMs(), { env: process.env });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), { invocation: original }));

