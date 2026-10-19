import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "dag-document-contract") {
    const schemaRoot = `${context.repoRoot}/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
    const { testDagDocumentContractOracle } = await import(`${schemaRoot}/🧪️tests/🪪️document/🟦️.ts`);
    testDagDocumentContractOracle();
    const { runCmd } = await import("@semio-tech/framework/process/execution");
    runCmd("bun", [`${context.repoRoot}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-dag-dag", "--lib", ...(segments[2] === "all" ? [] : ["dag_document_contract_"]), "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
