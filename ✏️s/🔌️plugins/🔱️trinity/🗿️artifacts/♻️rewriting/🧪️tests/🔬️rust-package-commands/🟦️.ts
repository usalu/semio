import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "rewriting-document-contract") {
    const { testRewritingDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
    testRewritingDocumentContractOracle();
    return;
  }
if (segments[0] === "rewriting-map-ownership") {
    const { testRewritingMapOwnershipOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🗂️map-ownership/🟦️.ts");
    testRewritingMapOwnershipOracle();
    if (segments[1] === "oracle") return;
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    for (const filter of ["rewriting_map_ownership", "committed_diff"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", filter, "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
    return;
  }
if (segments[0] === "rewriting-window-config") {
    const { testRewritingDocumentRetirementOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts");
    testRewritingDocumentRetirementOracle();
    const { testRewritingWindowConfigOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts");
    testRewritingWindowConfigOracle();
    if (segments[1] === "oracle") return;
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "rewriting_window_config", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
