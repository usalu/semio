import type { ScriptRouter } from "../🟦️.ts";
import { advanceScriptInvocation, checkScriptInvocation, type ScriptInvocation } from "../📥️invocation/🟦️.ts";

export interface RunScriptMainOptions<Capabilities extends object = object> {
  readonly invocation: ScriptInvocation<Capabilities>;
  readonly argv?: readonly string[];
  readonly defaultCommand?: string;
  readonly beforeDispatch?: (argv: readonly string[], invocation: ScriptInvocation<Capabilities>) => boolean | Promise<boolean>;
}

/** 🚪️ Dispatches owned commands after an optional caller-owned contribution. */
export async function runScriptMain<Capabilities extends object>(router: ScriptRouter<Capabilities>, options: RunScriptMainOptions<Capabilities>): Promise<void> {
  checkScriptInvocation(options?.invocation);
  let argv = [...(options.argv ?? process.argv.slice(2))];
  if (options.beforeDispatch) {
    await advanceScriptInvocation(options.invocation, argv[0] ?? "", "contribution");
    const consumed = await options.beforeDispatch([...argv], options.invocation);
    checkScriptInvocation(options.invocation);
    if (consumed) return;
  }
  if (options.defaultCommand && !argv.length) argv = [options.defaultCommand];
  if (!router.hasCommands()) throw Error(`usage: ${router.usage()}`);
  await router.run(argv, options.invocation);
}
