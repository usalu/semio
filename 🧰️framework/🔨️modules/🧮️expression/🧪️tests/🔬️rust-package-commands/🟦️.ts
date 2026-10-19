import type { ScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join, resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

const ORACLES = ["python"] as const;

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const cargoPhase=createIssuedResourceControlV1(context.invocation,"cargo:script-entry",0);
  if (segments[0] !== "oracle") {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(context.root, "Cargo.toml"), packages: ["semio-framework-expression"], cwd: context.root, extraArgs: rest }, readCargoTestPolicyV1(process.env), context.repoRoot, context.invocation,cargoPhase);
    return;
  }
  const { rest } = resolveTestLevel(segments.slice(1));
  const picked = rest.length ? rest : [...ORACLES];
  for (const name of picked) if (!(ORACLES as readonly string[]).includes(name)) throw new Error(`Expected test oracle ${ORACLES.join("|")}, got ${JSON.stringify(name)}`);
  const owner = resolve(context.root, "../..");
  const budgetMs = testLevelBudgetMs();
  if (picked.includes("python")) await runBudgetedTestCommand("uv", ["run", "--locked", "--no-sync", "python", "-m", "pytest", "-p", "no:cacheprovider", "-q", join(owner, "🧪️tests/📜️conformance/🐍️.py")], { cwd: context.repoRoot, budgetMs });
}
