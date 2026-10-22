#!/usr/bin/env repo
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "@semio-tech/framework/process/routing";
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dirname).register("run", RunScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["run"] }));
