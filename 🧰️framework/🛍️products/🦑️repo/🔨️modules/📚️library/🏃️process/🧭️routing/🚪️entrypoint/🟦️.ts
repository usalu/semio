import type { ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain, type RunScriptMainOptions } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

export type RunRepoScriptMainOptions = Pick<RunScriptMainOptions, "defaultCommand">;

/** 🦑️ Contributes repository policy commands to the neutral command entrypoint. */
export async function runRepoScriptMain(router: ScriptRouter, scriptUrl: string, options: RunRepoScriptMainOptions = {}): Promise<void> {
  await runScriptMain(router, { ...options, beforeDispatch: async (argv) => {
    if (argv[0] !== "policy") return false;
    const { dispatchPolicyArgv } = await import("../../../🟦️.ts");
    return dispatchPolicyArgv([...argv], scriptUrl);
  } });
}
