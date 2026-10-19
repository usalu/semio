import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptInvocationEnvironment, scriptInvocationBudget } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.shift() !== "shape") throw new Error("Expected test shape");
  await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../🧪️tests/🧩️shape/🟦️.ts"), ...args], context.root, "schema-validator:shape", scriptInvocationBudget(context.invocation,15_000),{signal:context.invocation.control.signal,env:scriptInvocationEnvironment(context.invocation,process.env)});
}
