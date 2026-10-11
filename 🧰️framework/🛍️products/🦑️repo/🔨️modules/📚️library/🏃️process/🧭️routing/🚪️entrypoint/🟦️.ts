import type { ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { readScriptPolicy } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, receiveScriptProcessInvocation, withScriptProcessEnvelope } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runScriptMain, type RunScriptMainOptions } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

export type RunRepoScriptMainOptions = Pick<RunScriptMainOptions, "defaultCommand">;

/** 🦑️ Contributes repository policy commands to the neutral command entrypoint, receiving the caller's invocation or opening the root one. */
export async function runRepoScriptMain(router: ScriptRouter, scriptUrl: string, options: RunRepoScriptMainOptions = {}): Promise<void> {
  const run = (invocation: ScriptInvocation) => runScriptMain(router, { ...options, invocation, beforeDispatch: async (argv) => {
    if (argv[0] !== "policy") return false;
    const { dispatchPolicyArgv } = await import("../../../🟦️.ts");
    return dispatchPolicyArgv([...argv], scriptUrl);
  } });
  if (process.env[SCRIPT_PROCESS_INVOCATION_ENV] !== undefined) return receiveScriptProcessInvocation(process.env, run);
  const policy = readScriptPolicy({ version: 1, owner: "repo-script", maximumElapsedMilliseconds: 0 });
  return withScriptProcessEnvelope(createScriptProcessEnvelope(policy, {}, Date.now()), run);
}
