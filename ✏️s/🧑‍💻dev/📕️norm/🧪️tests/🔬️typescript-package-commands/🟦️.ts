import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCmd, runCargo } from "@semio-tech/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "norm-document-contract") {
    const { testNormDocumentContractOracle } = await import("../../../../🔌️plugins/📕️norm/📦️packages/🟦️typescript/🧪️testing/🪪️document-contract/🟦️.ts");
    await testNormDocumentContractOracle();
    const normRoot = join(context.repoRoot, "✏️s/🔌️plugins/📕️norm");
    const files = ["⚖️en1990", "⚡️din18599"].flatMap((artifact) => ["", "📸️snapshot", "🔺️diff"].map((facet) => join(normRoot, "🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema", facet, "🟦️.ts")));
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...files, join(normRoot, "🧪️tests/🪪️document-contract/🟦️.ts")], { cwd: context.repoRoot });
    return;
  }
if (segments[0] === "norm-results-window-ownership") {
    const configRoot = join(context.repoRoot, "✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config");
    const oracle = join(configRoot, "🧪️tests/🔬️window-ownership/🟦️.ts");
    const { testNormResultsWindowOwnershipOracle } = await import(oracle);
    testNormResultsWindowOwnershipOracle();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(configRoot, "🧬️schema/🟦️.ts"), join(configRoot, "🧬️schema/🧬️mutations/🟦️.ts"), oracle], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-norm-en1996", "--lib", "norm_results_window_ownership_", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
