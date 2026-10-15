#!/usr/bin/env bun
/** 🏘️ Runs every cross-plugin composition law; plugins may not meet anywhere else. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Executes the composition laws with the repository's owned command runner. */
class CompositionTestScript extends BundleScript {
  async run(): Promise<void> {
    const tests = ["🔱️trinity-graph-child/🟦️.ts", "🌀️procedural-formats/🧪️laws/🟦️.ts", "🖨️raster-decoded-image/🟦️.ts"].map((file) => join(import.meta.dir, file));
    await runRepositoryCommand(this.invocation, process.execPath, ["test", ...tests], this.repoRoot, "plugin-composition", { version: 1, maximumElapsedMilliseconds: 0, output: null });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", CompositionTestScript);
await receiveScriptProcessInvocation(process.env, (original) => runScriptMain(router, { invocation: original }));
