#!/usr/bin/env bun
/** 🌱️ Executes the shared authority's complete app and portable native laws. */
import { receiveScriptProcessInvocation } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("run", RunScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["run"] }));
