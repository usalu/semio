#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-mcp-go` router: `bun ./📜️script.ts build|test|run`. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runCmd, runCmdStatus, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;
const ENTRY_DIR = join(PACKAGE_DIR, "🚀️bin");
const BINARY = process.platform === "win32" ? "semio-repo-mcp.exe" : "semio-repo-mcp";

/** 🗃️ The MCP binary is build output, so it lives in the marked repository cache and never in the tree. */
function binaryPath(repoRoot: string): string {
  const path = join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", BINARY);
  mkdirSync(dirname(path), { recursive: true });
  return path;
}

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  run(): void {
    runCmd("go", ["build", "-o", binaryPath(this.repoRoot), "."], { cwd: ENTRY_DIR, env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { cwd: PACKAGE_DIR, env: goEnvironment(this.repoRoot) });
  }
}

/** ▶️ Builds then execs the stdio server with the profile taken from `SEMIO_REPO_MCP_CLIENT`. */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    const binary = binaryPath(this.repoRoot);
    runCmd("go", ["build", "-o", binary, "."], { cwd: ENTRY_DIR, env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
    const profile = segments[0]?.trim();
    const environment = goEnvironment(this.repoRoot);
    if (profile) environment.SEMIO_REPO_MCP_CLIENT = profile;
    process.exit(runCmdStatus(binary, [], { cwd: this.repoRoot, env: environment }));
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript);

await runBundleScriptMain(router, import.meta.url);
