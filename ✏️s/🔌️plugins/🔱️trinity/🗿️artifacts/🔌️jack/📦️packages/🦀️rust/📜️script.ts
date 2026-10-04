#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ trinity-jack Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "jack-document-contract") {
      const { testJackDocumentContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
      testJackDocumentContract();
      return;
    }
if (segments[0] === "jack-query-ownership") {
      const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
      const { testResumableQueryOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🪜️resumable-query/🟦️.ts");
      testResumableQueryOracle();
      if (segments[1] === "oracle") return;
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--features", "component-app-assembly", "--lib", "query_ownership", "--", "--nocapture"], join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust"));
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-trinity-jack", { snapshotSqliteTestFeatures: ["component-app-assembly"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"], commands: { "graph-generate":GraphGenerateScript,"graph-wire-check":OwnerGraphWireCheckScript, verify: OwnedVerifyScript } });

