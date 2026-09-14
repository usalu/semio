#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-metrics-go` router: `bun ./📜️script.ts build|test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runCmd, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  run(): void {
    runCmd("go", ["build", "./..."], { cwd: PACKAGE_DIR, env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { cwd: PACKAGE_DIR, env: goEnvironment(this.repoRoot) });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
