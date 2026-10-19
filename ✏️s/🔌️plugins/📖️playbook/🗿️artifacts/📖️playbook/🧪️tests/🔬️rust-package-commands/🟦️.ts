import type { ScriptInvocation } from "@semio-tech/framework/process/routing";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "playbook-document-contract") {
    const { testPlaybookDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
    testPlaybookDocumentContractOracle();
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
