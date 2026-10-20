#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "@semio-tech/framework/process/routing";
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("run", RunScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["run"] }));
