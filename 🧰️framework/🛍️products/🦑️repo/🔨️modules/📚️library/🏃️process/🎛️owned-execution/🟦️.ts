import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import {repositoryCargoPreparationStorageV1, prepareCargoWorkspaceInvocation } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

import { runOwnedCommand, type OwnedCommandOptions } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🦀️ Executes caller commands under the repository's authored Cargo and build-budget policy. */
export async function runRepositoryCommand(command: string, args: string[], cwd: string, label: string, timeoutMs = buildBudgetMs(), options: OwnedCommandOptions = {}): Promise<void> {
  if (options.signal?.aborted) throw new Error(`${label} stopped: cancelled`);
  if (command === "cargo") prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(getWorkspaceRoot(options.env??process.env)),getWorkspaceRoot(options.env??process.env),args,cwd,options.env??process.env);
  return runOwnedCommand(command, args, cwd, label, timeoutMs, options);
}
