import type { ScriptRouter } from "../🟦️.ts";

export interface RunScriptMainOptions {
  readonly argv?: readonly string[];
  readonly defaultCommand?: string;
  readonly beforeDispatch?: (argv: readonly string[]) => boolean | Promise<boolean>;
}

/** 🚪️ Dispatches owned commands after an optional caller-owned contribution. */
export async function runScriptMain(router: ScriptRouter, options: RunScriptMainOptions = {}): Promise<void> {
  let argv = [...(options.argv ?? process.argv.slice(2))];
  if (await options.beforeDispatch?.([...argv])) return;
  if (options.defaultCommand && !argv.length) argv = [options.defaultCommand];
  if (!router.hasCommands()) throw Error(`usage: ${router.usage()}`);
  await router.run(argv);
}
