#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";

/** 🌱️ Runs the schema-first neutral value ownership and actual products-absent compiler laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original, defaultCommand: "test" }));
