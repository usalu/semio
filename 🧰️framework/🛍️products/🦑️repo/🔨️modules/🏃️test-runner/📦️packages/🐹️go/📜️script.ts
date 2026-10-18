#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-test-runner-go` router: `bun ./📜️script.ts build|test`. */
import { join } from "node:path";
import { goCoverageArgs } from "../../../📚️library/🧪️testing/📊️coverage/🟦️.ts";
import { goLevelTestArgs } from "../../../📚️library/🧪️testing/🎚️levels/🟦️.ts";
import { runCanonicalGoBuild, runCanonicalGoTests } from "../../../📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;

function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runCanonicalGoBuild(PACKAGE_DIR, ["./..."], { control: scriptCompilerAdmissionControlV1(this.invocation, "go:build", 0),  env: goEnvironment(this.repoRoot), budgetMs: buildBudgetMs(),  });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(PACKAGE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { env: goEnvironment(this.repoRoot) });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
