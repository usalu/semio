import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { join } from "node:path";
import { goCoverageArgs, goLevelTestArgs, runCanonicalGoBuild, runCanonicalGoTests } from "../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🐹️go");

const PACKAGE_DIR = SCRIPT_DIR;
function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

/** 🏃️ Runs the `BuildScript` command under the caller's roots and invocation. */
export async function runBuildScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  await runCanonicalGoBuild(PACKAGE_DIR, ["./..."], { control: scriptCompilerAdmissionControlV1(context.invocation, "go:build", 0),  env: goEnvironment(context.repoRoot), budgetMs: buildBudgetMs(),  });
}

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { level, rest } = resolveTestLevel(segments);
  await runCanonicalGoTests(PACKAGE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(context.repoRoot, PACKAGE_DIR), ...rest], { env: goEnvironment(context.repoRoot) });
}
