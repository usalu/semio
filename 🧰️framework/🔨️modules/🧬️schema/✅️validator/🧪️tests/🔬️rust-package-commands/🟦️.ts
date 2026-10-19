import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";

/** 🏃️ Runs the `NeutralOwnerScript` command under the caller's roots and invocation. */
export async function runNeutralOwnerScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  const { runOwnedCommand } = await import("../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
  await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../🧪️tests/🧩️neutral-owner/🟦️.ts")], context.repoRoot, "schema-validator:neutral-owner", 120000, { env: process.env });
}
