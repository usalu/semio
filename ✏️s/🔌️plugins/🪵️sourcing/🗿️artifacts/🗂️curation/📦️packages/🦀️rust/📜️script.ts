#!/usr/bin/env bun
/** 📦️ sourcing curation Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import {runRepositoryCommand} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments.length===1&&segments[0]==="snapshot-sqlite-source"){
      const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
      await runRepositoryCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"curation-snapshot-sqlite-public-types");
      return;
    }
if (segments[0] === "curation-document-contract") {
  const schemaRoot = this.repoRoot + "/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
  const { testCurationDocumentContractOracle } = await import(schemaRoot + "/🧪️tests/🪪️document/🟦️.ts");
  testCurationDocumentContractOracle();
  const { runCmd } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts");
  const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
  runCmd("bun", [this.repoRoot + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", schemaRoot + "/🧪️tests/🪪️document/🟦️.ts"], { cwd: this.repoRoot });
  if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sourcing-curation", "--lib", "curation_document_contract", "--", "--nocapture"], this.repoRoot);
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-sourcing-curation", { commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"] });
