#!/usr/bin/env bun
/** ⚙️ Builds and tests the `semio-framework-repo-identity` crate (nx bridge for `repo/identity/rs`). */
import { devToolingEnv, resolveTestLevel, runCargoTestBudgeted, runCmd } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const crate = "semio-framework-repo-identity";

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

await runScriptMain(router);
