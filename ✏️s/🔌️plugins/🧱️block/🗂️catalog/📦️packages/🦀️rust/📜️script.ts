#!/usr/bin/env bun
/** 🗂️ Runs Block catalog native and portable ownership laws. */
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🚪️ `conformance` — runs the program of this script. */
class ConformanceScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("conformance", ConformanceScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
