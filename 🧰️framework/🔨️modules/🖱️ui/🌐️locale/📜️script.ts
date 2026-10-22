#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand } from "../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { isMainModuleV1 } from "../../⚙️runtime/🏁️main/🟦️.ts";

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

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript).register("check", CheckScript), { invocation: original }));

