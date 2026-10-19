#!/usr/bin/env bun
/** 📦️ sourcing curation Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-sourcing-curation", { packages: ["semio-s-artifact-sourcing-curation", "semio-s-artifact-sourcing-curation-domain", "semio-s-artifact-sourcing-curation-examples", "semio-s-artifact-sourcing-curation-inferences", "semio-s-artifact-sourcing-curation-operations", "semio-s-artifact-sourcing-curation-schema", "semio-s-artifact-sourcing-curation-tests"], commands: router, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
