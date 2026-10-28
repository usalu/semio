import type { ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { requireIssuedResourceControlInvocationV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { resolveTestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { join } from "node:path";
import { runCanonicalGoBuild, runCanonicalGoTests } from "../../📚️library/📦️packages/🟦️typescript/🧪️testing/🐹️go-plan/🟦️.ts";
import { goCoverageArgs } from "../../📚️library/📦️packages/🟦️typescript/🧪️testing/📊️coverage/🟦️.ts";
import { goLevelTestArgs } from "../../📚️library/📦️packages/🟦️typescript/🧪️testing/🎚️levels/🟦️.ts";
import { resolveMcpBin } from "../../📚️library/📦️packages/🟦️typescript/🚨️policy/🖥️cli/🟦️.ts";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dirname, "");

const MODULE_DIR = SCRIPT_DIR;
function goEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

/** 🏃️ Runs the `BuildScript` command under the caller's roots and invocation. */
export async function runBuildScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation; control?: ResourceLeaseControl }>): Promise<void> {
  const control=context.control??scriptCompilerAdmissionControlV1(context.invocation,"go:build",0);
  requireIssuedResourceControlInvocationV1(control,context.invocation);
  const binary = resolveMcpBin(context.repoRoot);
  await runCanonicalGoBuild(MODULE_DIR, ["-o", binary, "."], { invocation:context.invocation, control, env: goEnvironment(context.repoRoot), budgetMs: buildBudgetMs() });
}

/** ▶️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { level, rest } = resolveTestLevel(segments);
  await runCanonicalGoTests(MODULE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(context.repoRoot, MODULE_DIR), ...rest], context.invocation, { env: goEnvironment(context.repoRoot) });
}
