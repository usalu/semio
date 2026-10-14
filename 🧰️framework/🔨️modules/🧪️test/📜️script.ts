#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🧪️ Runs the neutral scenario adapter and schema ownership contracts. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1 || segments[0] !== "adapter-ownership") throw new Error("Expected test adapter-ownership");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🔌️adapter/🧪️tests/🟦️.ts")], this.repoRoot, "adapter-ownership", TEST_LEVEL_BUDGET_MS.fundamental, { env: process.env });
  }
}
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original }));
