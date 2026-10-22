#!/usr/bin/env repo
/** 📦️ note note Rust artifact package router. */
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
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-note-note", { packages: ["semio-s-artifact-note-note", "semio-s-artifact-note-note-delete-blocks", "semio-s-artifact-note-note-diff", "semio-s-artifact-note-note-insert-table-row", "semio-s-artifact-note-note-kinds-catalog", "semio-s-artifact-note-note-mut-family", "semio-s-artifact-note-note-mut-family-2", "semio-s-artifact-note-note-mut-family-3", "semio-s-artifact-note-note-mut-family-4", "semio-s-artifact-note-note-mutations", "semio-s-artifact-note-note-nudge-selection", "semio-s-artifact-note-note-schema", "semio-s-artifact-note-note-tests-hides-the-grid", "semio-s-artifact-note-note-unit-tests", "semio-s-artifact-note-note-v1"], commands: router });
