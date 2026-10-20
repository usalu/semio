#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
/** 🚪️ `capture-program` — runs the program of this script. */
class CaptureProgramScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `resolve-program` — runs the program of this script. */
class ResolveProgramScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("capture-program", CaptureProgramScript).register("resolve-program", ResolveProgramScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["capture-program"] }));
