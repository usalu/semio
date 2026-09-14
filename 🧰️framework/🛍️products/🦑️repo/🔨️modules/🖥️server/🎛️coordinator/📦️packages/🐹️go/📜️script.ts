#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-coordinator-go` router: `bun ./📜️script.ts build|test|run`. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runCmd, runCmdStatus, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;
const BINARY_DIR = join(PACKAGE_DIR, "📦️main");
const BINARY = process.platform === "win32" ? "semio-repo-coordinator.exe" : "semio-repo-coordinator";

/** 🗃️ The coordinator binary is build output, so it lives in the marked repository cache and never in the tree. */
export function coordinatorBinaryPath(repoRoot: string): string {
  const path = join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", BINARY);
  mkdirSync(dirname(path), { recursive: true });
  return path;
}

/** 🐹️ Every Go invocation of this package shares the repository workspace file. */
export function coordinatorGoEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  run(): void {
    runCmd("go", ["build", "-o", coordinatorBinaryPath(this.repoRoot), "."], { cwd: BINARY_DIR, env: coordinatorGoEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { cwd: PACKAGE_DIR, env: coordinatorGoEnvironment(this.repoRoot) });
  }
}

/** ▶️ Builds then execs the coordinator, which reads `COMPOSE_SERVER_ADDR`/`COMPOSE_SERVER_DB`/`COMPOSE_SERVER_TOKEN`. */
class RunScript extends BundleScript {
  run(): void {
    const binary = coordinatorBinaryPath(this.repoRoot);
    runCmd("go", ["build", "-o", binary, "."], { cwd: BINARY_DIR, env: coordinatorGoEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
    process.exit(runCmdStatus(binary, [], { cwd: this.repoRoot, env: coordinatorGoEnvironment(this.repoRoot) }));
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript);

await runBundleScriptMain(router, import.meta.url);
