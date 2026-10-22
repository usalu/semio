#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "./../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../⚙️runtime/🏁️main/🟦️.ts";
/** 🚪️ `conformance` — runs the program of this script. */
class ConformanceScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `conformance-process` — runs the program of this script. */
class ConformanceProcessScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `conformance-command` — runs the program of this script. */
class ConformanceCommandScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `test` — runs the program of this script. */
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dirname).register("conformance", ConformanceScript).register("conformance-process", ConformanceProcessScript).register("conformance-command", ConformanceCommandScript).register("test", TestScript);
if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["conformance"] }));
