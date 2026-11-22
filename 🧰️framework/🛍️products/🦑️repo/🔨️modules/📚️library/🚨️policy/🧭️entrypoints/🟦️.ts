import { fileURLToPath } from "node:url";
import { runPolicyExit } from "../🏃️runner/🟦️.ts";
import { createScriptProcessEnvelope, readScriptPolicy, receiveScriptProcessInvocation, runScriptMain, SCRIPT_PROCESS_INVOCATION_ENV, type ScriptInvocation, ScriptRouter, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { retryOnConcurrentEditV1 } from "../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🚪️When argv contains `policy`, runs this bundle's policy lint and exits. */
export async function dispatchPolicyArgv(segments: string[], scriptUrl: string): Promise<boolean> {
  if (segments[0] !== "policy") return false;
  setTimeout(async () => {
    try {
      await runPolicyExit(fileURLToPath(scriptUrl));
    } catch (e) {
      console.error(e);
      process.exit(1);
    }
  }, 0);
  return true;
}

/** ▶️Policy-only bundle entry when no other subcommands are registered. */
export async function runPolicyOnlyMain(scriptUrl: string): Promise<void> {
  const segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  console.error("usage: repo ./📜️script.ts policy");
  process.exit(1);
}

/** 📜️Workspace root `script.ts` entry (no policy dispatch). */
export async function runWorkspaceScriptMain(router: ScriptRouter, options:Readonly<{argv?:readonly string[];freshCapabilities?:()=>Promise<Readonly<Record<string,unknown>>>;retryFreshBuild?:boolean}>={}): Promise<void> {
  const argv = options.argv === undefined ? undefined : Object.freeze([...options.argv]);
  const run = (invocation: ScriptInvocation) => runScriptMain(router, { invocation, argv });
  if (process.env[SCRIPT_PROCESS_INVOCATION_ENV] !== undefined) return receiveScriptProcessInvocation(process.env, run);
  const policy = readScriptPolicy({ version: 1, owner: "workspace-root", maximumElapsedMilliseconds: 0 });
  const capabilities = await options.freshCapabilities?.() ?? {workspaceRoot:router.repoRoot};
  const birth = (dispatch = run) => withScriptProcessEnvelope(createScriptProcessEnvelope(policy, capabilities, Date.now()), dispatch);
  if (!options.retryFreshBuild) return birth();
  let signal: AbortSignal | undefined;
  return retryOnConcurrentEditV1("build", () => {
    signal = undefined;
    return birth(original => {
      signal = original.control.signal;
      return run(original);
    });
  }, { retryable: () => signal?.aborted === false });
}
