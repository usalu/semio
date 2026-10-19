import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { previewStylingArtifacts } from "../../📽️projection/🟦️.ts";

/** 🏃️ Runs the `PreviewGeneratedScript` command under the caller's roots and invocation. */
export function runPreviewGeneratedScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  process.stdout.write(previewStylingArtifacts(context.repoRoot));
}
