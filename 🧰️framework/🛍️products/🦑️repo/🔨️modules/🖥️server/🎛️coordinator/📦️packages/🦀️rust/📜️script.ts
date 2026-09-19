#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-coordinator-rs` router: `bun ./📜️script.ts build|test|run`. */
import { BundleScript, ScriptRouter, devToolingEnv, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const CRATE = "semio-framework-repo-coordinator";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", CRATE], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted([CRATE], this.repoRoot, rest);
  }
}

/** ▶️ Runs the coordinator, which reads `COMPOSE_SERVER_ADDR`/`COMPOSE_SERVER_DB`/`COMPOSE_SERVER_TOKEN`. */
class RunScript extends BundleScript {
  run(): void {
    process.exit(runCmdStatus("cargo", ["run", "-p", CRATE, "--bin", "semio-repo-coordinator"], { cwd: this.repoRoot, env: devToolingEnv() }));
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript);

await runBundleScriptMain(router, import.meta.url);
