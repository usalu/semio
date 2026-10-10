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
    if (segments.length !== 1 || !["schema", "ownership", "neutral-ownership"].includes(segments[0]!)) throw Error("Expected test schema or test ownership");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, segments[0] === "schema" ? "🧪️tests/🟦️.ts" : segments[0] === "ownership" ? "🧪️tests/📍️ownership/🟦️.ts" : "🧪️tests/📍️ownership/🧭️direction/🟦️.ts")], { cwd: this.root, budgetMs: testLevelBudgetMs(), env: process.env, throwOnFailure: true });
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "types") throw Error("Expected check types");
    await runOwnedCommand(process.execPath, ["x", "tsc", "--noEmit", "--incremental", "false", "-p", resolve(this.root, "tsconfig.json")], this.root, "compute-contract-types", cmdBudgetMs(), { env: process.env });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), { invocation: original }));

