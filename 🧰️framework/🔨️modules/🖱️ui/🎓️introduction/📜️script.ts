#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { isMainModuleV1 } from "../../⚙️runtime/🏁️main/🟦️.ts";

/** 🎓️ Runs the neutral introduction corpus and strict owned-interface checks. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Expected test");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🔬️unit/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original }));
