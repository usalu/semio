#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "@semio-tech/process/routing";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
/** 🚪️ `execute` — runs the program of this script. */
class ExecuteScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("execute", ExecuteScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["execute"] }));
