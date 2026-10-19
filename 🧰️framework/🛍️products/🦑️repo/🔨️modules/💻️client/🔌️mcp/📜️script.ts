#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧭️ `repo-mcp` router: `bun ./📜️script.ts build|test`. The Go sources of this module are
 * taxonomy-named (`🖥️server/🐹️.go`, `🗄️repository/🐹️.go`, …), so both verbs go through the canonical
 * Go projection rather than calling `go` on the tree directly. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { goCoverageArgs, goLevelTestArgs, runCanonicalGoBuild, runCanonicalGoTests } from "../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { resolveMcpBin } from "../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const MODULE_DIR = import.meta.dir;

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const binary = resolveMcpBin(this.repoRoot);
    mkdirSync(dirname(binary), { recursive: true });
    await runCanonicalGoBuild(MODULE_DIR, ["-o", binary, "."], { control: scriptCompilerAdmissionControlV1(this.invocation, "go:build", 0),  env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs(),  });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(MODULE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, MODULE_DIR), ...rest], { env: goEnvironment(this.repoRoot) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
