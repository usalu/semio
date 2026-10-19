import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { resolve } from "node:path";
import { runRepositoryCommand } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if(segments.length===1&&segments[0]==="snapshot-sqlite-source"){
    const snapshot=resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
    await runRepositoryCommand(context.invocation,process.execPath,[resolve(context.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],context.repoRoot,"curation-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
    return;
  }
if (segments[0] === "curation-document-contract") {
const schemaRoot = context.repoRoot + "/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const { testCurationDocumentContractOracle } = await import(schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts");
testCurationDocumentContractOracle();
const { runCmd } = await import("@semio-tech/framework/process/execution");
const { runCargo } = await import("@semio-tech/framework/process/execution");
runCmd("bun", [context.repoRoot + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts"], { cwd: context.repoRoot });
if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sourcing-curation", "--lib", "curation_document_contract", "--", "--nocapture"], context.repoRoot);
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
