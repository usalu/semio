#!/usr/bin/env bun
/** 📦️ sourcing curation Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments.length===1&&segments[0]==="snapshot-sqlite-source"){
      const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
      await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"curation-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
      return;
    }
if (segments[0] === "curation-document-contract") {
  const schemaRoot = this.repoRoot + "/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
  const { testCurationDocumentContractOracle } = await import(schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts");
  testCurationDocumentContractOracle();
  const { runCmd } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
  const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
  runCmd("bun", [this.repoRoot + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts"], { cwd: this.repoRoot });
  if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sourcing-curation", "--lib", "curation_document_contract", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-sourcing-curation", { packages: ["semio-s-artifact-sourcing-curation", "semio-s-artifact-sourcing-curation-domain", "semio-s-artifact-sourcing-curation-examples", "semio-s-artifact-sourcing-curation-inferences", "semio-s-artifact-sourcing-curation-operations", "semio-s-artifact-sourcing-curation-schema", "semio-s-artifact-sourcing-curation-tests"], commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
