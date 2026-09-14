#!/usr/bin/env bun
/** ⚙️ Builds and tests the `semio-framework-repo-contributors` crate (nx bridge for `repo/contributors/rs`). */
import { BundleScript, ScriptRouter, devToolingEnv, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const crate = "semio-framework-repo-contributors";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", crate], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([crate], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
