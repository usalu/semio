#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runStylingPythonGenerateScript, runStylingPythonTestScript } from "../../🧪️tests/🔬️python-package-commands/🟦️.ts";

/** 🎨️ Confirms that the shared styling generator dependency completed. */
export class StylingPythonGenerateScript extends BundleScript {
  async run(): Promise<void> {
    await runStylingPythonGenerateScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

/** 🧪️ Imports the canonical Python source and its wheel projection. */
export class StylingPythonTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runStylingPythonTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("generate", StylingPythonGenerateScript).register("test", StylingPythonTestScript)).run(process.argv.slice(2), original));
