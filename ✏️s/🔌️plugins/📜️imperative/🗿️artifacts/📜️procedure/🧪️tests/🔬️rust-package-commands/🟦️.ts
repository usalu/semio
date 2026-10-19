import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "procedure-document-contract") {
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    const { testProcedureDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
    testProcedureDocumentContractOracle();
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), ...["--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)]], { cwd: context.repoRoot });
    if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-imperative-procedure", "--lib", "procedure_document_contract", "--", "--nocapture"], context.repoRoot);
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
