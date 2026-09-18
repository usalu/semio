#!/usr/bin/env bun
/** 🧭️ `repo-mcp` router: `bun ./📜️script.ts build|test`. The Go sources of this module are
 * taxonomy-named (`🖥️server/🐹️.go`, `🗄️repository/🐹️.go`, …), so both verbs go through the canonical
 * Go projection rather than calling `go` on the tree directly. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, goCoverageArgs, goLevelTestArgs, resolveMcpBin, resolveTestLevel, runBundleScriptMain, runCanonicalGoBuild, runCanonicalGoTests } from "../../📚️library/📦️packages/🟦️typescript/🟦️.ts";

const MODULE_DIR = import.meta.dir;

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  run(): void {
    const binary = resolveMcpBin(this.repoRoot);
    mkdirSync(dirname(binary), { recursive: true });
    runCanonicalGoBuild(MODULE_DIR, ["-o", binary, "."], { env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(MODULE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, MODULE_DIR), ...rest], { env: goEnvironment(this.repoRoot) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
