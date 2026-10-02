#!/usr/bin/env bun
/** 🪪️ Runs neutral identity contracts. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/🎛️owned-execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
class TestScript extends BundleScript {
 async run(segments: string[]): Promise<void> {
  if (segments.length !== 1 || segments[0] !== "grapheme") throw Error("Expected test grapheme");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧩️grapheme/🧪️tests/🟦️.ts")], this.repoRoot, "identity-grapheme", TEST_LEVEL_BUDGET_MS.fundamental);
 }
}
if (import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript));
