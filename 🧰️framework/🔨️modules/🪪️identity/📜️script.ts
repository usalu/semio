#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🪪️ Runs neutral identity contracts. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { isMainModuleV1 } from "../⚙️runtime/🏁️main/🟦️.ts";
class TestScript extends BundleScript {
 async run(segments: string[]): Promise<void> {
  if (segments.length !== 1 || segments[0] !== "grapheme") throw new Error("Expected test grapheme");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧩️grapheme/🧪️tests/🟦️.ts")], this.repoRoot, "identity-grapheme", TEST_LEVEL_BUDGET_MS.fundamental);
 }
}
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original }));
