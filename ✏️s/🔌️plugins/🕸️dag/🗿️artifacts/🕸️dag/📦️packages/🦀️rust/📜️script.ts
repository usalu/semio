#!/usr/bin/env repo
/** 📦️ dag dag Rust artifact package router. */
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
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-dag-dag", { packages: ["semio-s-artifact-dag-dag", "semio-s-artifact-dag-dag-config", "semio-s-artifact-dag-dag-dag", "semio-s-artifact-dag-dag-diff", "semio-s-artifact-dag-dag-document-behavior-tests", "semio-s-artifact-dag-dag-inferences", "semio-s-artifact-dag-dag-schema", "semio-s-artifact-dag-dag-tests", "semio-s-artifact-dag-dag-v1"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📋️contract/🟦️.ts"], commands: router });
