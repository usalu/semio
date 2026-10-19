#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ trinity-rewriting Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runCargo } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "rewriting-document-contract") {
      const { testRewritingDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
      testRewritingDocumentContractOracle();
      return;
    }
if (segments[0] === "rewriting-map-ownership") {
      const { testRewritingMapOwnershipOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🗂️map-ownership/🟦️.ts");
      testRewritingMapOwnershipOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
      for (const filter of ["rewriting_map_ownership", "committed_diff"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", filter, "--", "--nocapture"], join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
      return;
    }
if (segments[0] === "rewriting-window-config") {
      const { testRewritingDocumentRetirementOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts");
      testRewritingDocumentRetirementOracle();
      const { testRewritingWindowConfigOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts");
      testRewritingWindowConfigOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "rewriting_window_config", "--", "--nocapture"], join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-trinity-rewriting", { packages: ["semio-s-artifact-trinity-rewriting", "semio-s-artifact-trinity-rewriting-editor", "semio-s-artifact-trinity-rewriting-inferences", "semio-s-artifact-trinity-rewriting-layout", "semio-s-artifact-trinity-rewriting-model", "semio-s-artifact-trinity-rewriting-mut-any", "semio-s-artifact-trinity-rewriting-operations", "semio-s-artifact-trinity-rewriting-schema", "semio-s-artifact-trinity-rewriting-tests-narrows-the-lhs-pattern-to-a-shaft-neighbour", "semio-s-artifact-trinity-rewriting-v1"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📋️contract/🟦️.ts", "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], commands: { "graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript, verify: OwnedVerifyScript } });
