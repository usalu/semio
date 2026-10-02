#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ⚙️ Builds and tests the `semio-framework-repo-codebase` crate (nx bridge for `repo/codebase/rs`). */
import { devToolingEnv, runRepositoryCargoTests, runCmd } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const CRATE = "semio-framework-repo-codebase";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", CRATE], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([CRATE], this.repoRoot, rest);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);
  await runScriptMain(router);
}
