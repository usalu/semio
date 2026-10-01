#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-events-rs` router: `bun ./📜️script.ts build|test`. */
import { devToolingEnv, resolveTestLevel, runCargoTestBudgeted, runCmd } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-events"], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-repo-events"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runScriptMain(router);
