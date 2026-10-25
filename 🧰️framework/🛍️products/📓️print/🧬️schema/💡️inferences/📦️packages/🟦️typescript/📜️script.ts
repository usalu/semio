#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 📊️ `@semio-tech/print-viz-inference` router: `repo ./📜️script.ts build|test [level]`.
 * `test` runs the authored inference differential harness: every computation measured against the d3 package
 * that is its registered oracle. The check table and its d3 imports live in `../../🔬️probes/🟦️.ts`; the
 * inference implementations themselves have no runtime dependency on anything outside this repository.
 */
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript, runBuildScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🏗️ Type-checks the complete visualization inference surface. */
class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));


