import { fileURLToPath } from "node:url";
import { runPolicyExit } from "../🏃️runner/🟦️.ts";
import { createScriptProcessEnvelope, readScriptPolicy, receiveScriptProcessInvocation, runScriptMain, SCRIPT_PROCESS_INVOCATION_ENV, type ScriptInvocation, ScriptRouter, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

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

/** 🚪️Policy-only bundle entry when no other subcommands are registered. */
export async function runPolicyOnlyMain(scriptUrl: string): Promise<void> {
  const segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  console.error("usage: bun ./📜️script.ts policy");
  process.exit(1);
}

/** 🚪️Workspace root `script.ts` entry (no policy dispatch). */
export async function runWorkspaceScriptMain(router: ScriptRouter): Promise<void> {
  const run = (invocation: ScriptInvocation) => runScriptMain(router, { invocation });
  if (process.env[SCRIPT_PROCESS_INVOCATION_ENV] !== undefined) return receiveScriptProcessInvocation(process.env, run);
  const policy = readScriptPolicy({ version: 1, owner: "workspace-root", maximumElapsedMilliseconds: 0 });
  return withScriptProcessEnvelope(createScriptProcessEnvelope(policy, {}, Date.now()), run);
}

/**
 * 🧭️Nested subcommand dispatch inside a `Script.run` implementation.
 * `handlers` keys are the first argv segment; `defaultKey` runs when argv is empty.
 */
export function dispatchSubcommand(segments: string[], handlers: Record<string, (rest: string[]) => void | Promise<void>>, usage: string, defaultKey?: string): void | Promise<void> {
  const key = segments[0] ?? defaultKey;
  const handler = key ? handlers[key] : undefined;
  if (!handler) {
    console.error(`usage: ${usage}`);
    process.exit(1);
  }
  return handler(segments.slice(1));
}
