import type { ScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { runBudgetedTestCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolveTypescriptCompilerV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

async function checkPortableTypes(root: string, workspace: string, tests: string[]): Promise<void> {
  await runBudgetedTestCommand(process.execPath, [resolveTypescriptCompilerV1(root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: workspace, budgetMs: 30000, throwOnFailure: true});
}

/** 🏃️ Runs the `ReferenceScript` command under the caller's roots and invocation. */
export async function runReferenceScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("test-reference accepts no arguments");
  await checkPortableTypes(context.root, context.repoRoot, [join(context.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(context.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts")]);
  await runBudgetedTestCommand(process.execPath, ["test", join(context.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(context.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts")], {cwd: context.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
}

/** ▶️ Runs the `OwnershipScript` command under the caller's roots and invocation. */
export async function runOwnershipScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("test-ownership accepts no arguments");
  await checkPortableTypes(context.root, context.repoRoot, [join(context.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(context.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(context.root, "../../🧪️tests/🚮️absence/🟦️.ts")]);
  await runBudgetedTestCommand(process.execPath, ["test", "--timeout", "30000", join(context.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(context.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(context.root, "../../🧪️tests/🚮️absence/🟦️.ts")], {cwd: context.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
}
