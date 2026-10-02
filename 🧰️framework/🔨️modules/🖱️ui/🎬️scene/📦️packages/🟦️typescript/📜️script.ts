#!/usr/bin/env bun
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { cmdBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
/** 🎬️ Tests scene payload projections against their shared neutral fixtures. */
import { resolve } from "node:path";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedCommand("bun", [
      "test",
      resolve(import.meta.dir, "../../🧪️tests/🚚️text-editor-lanes/🟦️.test.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️table-lanes/🟦️.test.ts"), resolve(import.meta.dir, "../../🧪️tests/✂️text-splice/🟦️.test.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️world3d-scene-lanes/🟦️.test.ts"),
      ...segments,
    ], this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
