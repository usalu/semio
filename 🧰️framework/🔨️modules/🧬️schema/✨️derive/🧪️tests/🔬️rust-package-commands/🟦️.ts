import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { executeCargoRequestV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { relative, resolve } from "node:path";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  await executeCargoRequestV1({version:1,kind:"tests",manifest:relative(context.repoRoot,resolve(context.root,"Cargo.toml")).replaceAll("\\","/"),cwd:relative(context.repoRoot,context.root).replaceAll("\\","/"),packages:["semio-framework-schema-derive"],arguments:segments},context.invocation);
}
