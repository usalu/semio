#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🗣️ Runs the neutral lexical and grammar ownership contract against authored source. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipScript);
await runScriptMain(router, { defaultCommand: "test-ownership" });
