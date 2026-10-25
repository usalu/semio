import type { ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readScriptPolicy } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { SCRIPT_PROCESS_INVOCATION_ENV, createScriptProcessEnvelope, receiveScriptProcessInvocation, withScriptProcessEnvelope } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain, type RunScriptMainOptions } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

export type RunRepoScriptMainOptions = Pick<RunScriptMainOptions, "defaultCommand"|"beforeDispatch"> & { readonly policyModule?: string };

/** 🗺️ Names the module that exports `policy`: the script itself, or the owner module its relative specifier resolves to. */
export function policyOwnerUrl(scriptUrl: string, policyModule?: string): string { return policyModule === undefined ? scriptUrl : new URL(policyModule, scriptUrl).href; }

/** 🦑️ Contributes repository policy commands to the neutral command entrypoint, receiving the caller's invocation or opening the root one. */
export async function runRepoScriptMain(router: ScriptRouter, scriptUrl: string, options: RunRepoScriptMainOptions = {}): Promise<void> {
  const run = (invocation: ScriptInvocation) => runScriptMain(router, { ...options, invocation, beforeDispatch: async (argv) => {
    if (argv[0] !== "policy") return options.beforeDispatch?.(argv,invocation)??false;
    const { dispatchPolicyArgv } = await import("../../../🚨️policy/🧭️entrypoints/🟦️.ts");
    return dispatchPolicyArgv([...argv], policyOwnerUrl(scriptUrl, options.policyModule));
  } });
  if (process.env[SCRIPT_PROCESS_INVOCATION_ENV] !== undefined) return receiveScriptProcessInvocation(process.env, run);
  const policy = readScriptPolicy({ version: 1, owner: "repo-script", maximumElapsedMilliseconds: 0 });
  return withScriptProcessEnvelope(createScriptProcessEnvelope(policy, {}, Date.now()), run);
}
