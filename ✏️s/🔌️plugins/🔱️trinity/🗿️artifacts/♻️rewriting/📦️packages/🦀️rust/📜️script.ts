#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ trinity-rewriting Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
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
      const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
      for (const filter of ["rewriting_map_ownership", "committed_diff"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", filter, "--", "--nocapture"], join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
      return;
    }
if (segments[0] === "rewriting-window-config") {
      const { testRewritingDocumentRetirementOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts");
      testRewritingDocumentRetirementOracle();
      const { testRewritingWindowConfigOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts");
      testRewritingWindowConfigOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--features", "component-app-assembly", "--lib", "rewriting_window_config", "--", "--nocapture"], join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust"));
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-trinity-rewriting", { snapshotSqliteTestFeatures: ["component-app-assembly"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/📋️contract/🟦️.ts", "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"], commands: { "graph-generate":GraphGenerateScript,"graph-wire-check":OwnerGraphWireCheckScript, verify: OwnedVerifyScript } });
