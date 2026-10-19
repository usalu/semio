#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "./../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
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
const router = new ScriptRouter(import.meta.dir).register("conformance", ConformanceScript).register("conformance-process", ConformanceProcessScript).register("conformance-command", ConformanceCommandScript).register("test", TestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
