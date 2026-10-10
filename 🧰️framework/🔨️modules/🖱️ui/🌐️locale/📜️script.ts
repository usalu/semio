#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";
import { cmdBudgetMs } from "../../🏃️process/⏱️budget/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "contract") throw new Error("Expected test contract");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🟦️.ts")], { cwd: this.root, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "types") throw new Error("Expected check types");
    await runOwnedCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], this.root, "ui-locale-contract-types", cmdBudgetMs(), { env: process.env });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), { invocation: original }));

