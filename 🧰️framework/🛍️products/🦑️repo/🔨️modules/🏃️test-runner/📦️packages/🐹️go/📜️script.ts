#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-test-runner-go` router: `bun ./📜️script.ts build|test`. */
import { join } from "node:path";
import { buildBudgetMs, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runCmd, runTestBudgeted } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  run(): void {
    runCmd("go", ["build", "..."], { cwd: PACKAGE_DIR, env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("go", ["test", "...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { cwd: PACKAGE_DIR, env: goEnvironment(this.repoRoot) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runScriptMain(router);
