import type { ScriptInvocation } from "@semio-tech/process/routing";
import { resolveTestLevel } from "@semio-tech/process/test-budget";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import contract from "./🧪️tests/🧫️fixtures/🔮️ownership/🔣️.json";
import { resolve } from "node:path";

/** 🏃️ Runs the `NativeTestScript` command under the caller's roots and invocation. */
export async function runNativeTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { rest } = resolveTestLevel(segments);
  if (rest.length) throw new Error("The full Puzzle native cohort accepts no test or scenario filters.");
  for (const artifact of contract.artifacts) await runRepositoryTestCommand(process.execPath, ["repo", "run", `${artifact.project}:test`, "--skip-cache"], { cwd: resolve(context.root, "../../../.."), budgetMs: 3_600_000 });
}
