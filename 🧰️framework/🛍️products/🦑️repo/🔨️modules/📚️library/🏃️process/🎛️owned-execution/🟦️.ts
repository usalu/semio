import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { prepareCargoWorkspaceInvocation } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🦀️ Executes caller commands under the repository's authored Cargo and build-budget policy. */
export async function runRepositoryCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: { stdout?: "inherit" | "ignore"; env?: Readonly<Record<string, string | undefined>>; signal?: AbortSignal; onLine?: (line: string) => void } = {}): Promise<void> {
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  if (command === "cargo") prepareCargoWorkspaceInvocation(getWorkspaceRoot(), args, cwd);
  return runOwnedCommand(command, args, cwd, label, timeoutMs, options);
}
