import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { resolve } from "node:path";
import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `RecursiveRetirementTest` command under the caller's roots and invocation. */
export async function runRecursiveRetirementTest(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  const mode = args.shift();
  if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test recursive-retirement [source|native]");
  if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/♻️retirement/🟦️.ts")], context.repoRoot, "pdf:recursive-retirement:source", 15_000);
  if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-pdf", context.repoRoot, ["--lib", "pdf_recursive_retirement_"], context.invocation);
}
