#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { cmdBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🎬️ Tests scene payload projections against their shared neutral fixtures. */
import { resolve } from "node:path";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runtimeExecutableV1 } from "../../../../⚙️runtime/🏷️identity/🟦️.ts";
import { runtimeTestArgumentsV1 } from "../../../../⚙️runtime/🚀️process/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedCommand(runtimeExecutableV1(), runtimeTestArgumentsV1([resolve(import.meta.dirname, "../../🧪️tests/🧩️block-list/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/🚚️text-editor-lanes/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/🚚️table-lanes/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/✂️text-splice/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/🚚️world3d-scene-lanes/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/📏️world3d-modelling/🟦️.ts"), resolve(import.meta.dirname, "../../🧪️tests/🚚️node-graph-scene-lanes/🟦️.ts"), ...segments]), this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
