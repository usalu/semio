import type { ScriptInvocation } from "@semio-tech/process/routing";
import { resolveTestLevel } from "@semio-tech/process/test-budget";
import { runRepositoryTestCommand } from "@semio-tech/process/test-execution";
import contract from "./🧪️tests/🧫️fixtures/🔮️ownership/🔣️.json";
import { resolve } from "node:path";

/** 🏃️ Runs the `NativeTestScript` command under the caller's roots and invocation. */
export async function runNativeTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { rest } = resolveTestLevel(segments);
  if (rest.length) throw new Error("The complete Space document native gate accepts no cohort filters.");
  for (const project of contract.nativeProjects) {
    await runRepositoryTestCommand(process.execPath, ["repo", "run", `${project}:test`, "--skip-cache"], { cwd: resolve(context.root, "../../../.."), budgetMs: 3_600_000 });
  }
}
