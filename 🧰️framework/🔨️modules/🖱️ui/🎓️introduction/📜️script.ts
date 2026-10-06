#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🎓️ Runs the neutral introduction corpus and strict owned-interface checks. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("Expected test");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🔬️unit/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
  }
}

if (import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript));
