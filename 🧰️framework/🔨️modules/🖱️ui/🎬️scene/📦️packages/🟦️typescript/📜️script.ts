#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🎬️ Tests scene payload projections against their shared neutral fixtures. */
import { resolve } from "node:path";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedCommand("bun", [
      "test",
      resolve(import.meta.dir, "../../🧪️tests/🧩️block-list/🟦️.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️text-editor-lanes/🟦️.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️table-lanes/🟦️.ts"), resolve(import.meta.dir, "../../🧪️tests/✂️text-splice/🟦️.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️world3d-scene-lanes/🟦️.ts"),
      resolve(import.meta.dir, "../../🧪️tests/📏️world3d-modelling/🟦️.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️node-graph-scene-lanes/🟦️.ts"),
      ...segments,
    ], this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
