#!/usr/bin/env repo
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain, createIssuedResourceControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
/** 🚪️ `capture-program` — runs the program of this script. */
class CaptureProgramScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram(createIssuedResourceControlV1(this.invocation,"cargo:preparation-program",0));
  }
}
/** 🧭️ `resolve-program` — runs the program of this script. */
class ResolveProgramScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram(createIssuedResourceControlV1(this.invocation,"cargo:preparation-program",0));
  }
}
const router = new ScriptRouter(import.meta.dirname).register("capture-program", CaptureProgramScript).register("resolve-program", ResolveProgramScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["capture-program"] }));
