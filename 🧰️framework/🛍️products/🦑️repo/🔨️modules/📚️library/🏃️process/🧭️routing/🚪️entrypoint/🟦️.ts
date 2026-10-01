import type { ScriptRouter } from "../🟦️.ts";

export type RunBundleScriptMainOptions = {
  defaultCommand?: string;
};

/**
 * 🚪️Bundle `script.ts` entry: handles optional `policy`, then routes remaining argv through `router`.
 * Export `policy` / `policyFile` from the same file when policy lint applies.
 */
export async function runBundleScriptMain(router: ScriptRouter, scriptUrl: string, opts: RunBundleScriptMainOptions = {}): Promise<void> {
  let segments = process.argv.slice(2);
  if (segments[0] === "policy") {
    const { dispatchPolicyArgv } = await import("../../../🟦️.ts");
    if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  }
  if (opts.defaultCommand && segments.length === 0) {
    segments = [opts.defaultCommand];
  }
  if (!router.hasCommands()) {
    console.error(`usage: ${router.usage()}`);
    process.exit(1);
  }
  await router.run(segments);
}

