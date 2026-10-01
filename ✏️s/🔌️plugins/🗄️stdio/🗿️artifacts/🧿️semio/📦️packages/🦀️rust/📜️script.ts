#!/usr/bin/env bun
/** 📦️ semio Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { retainedExtrusionOracle } from "../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/📦️extrude-orientation/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { prepareCargoCapabilityLinksV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts";
import { runCargoCapabilityContributionChecks } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🧪️tests/🟦️.ts";

class CompositionScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 1 || segments[0] !== "prepare") throw new Error("Unknown Semio conversion composition command");
    console.log(`semio conversion composition: targets=${prepareCargoCapabilityLinksV1(this.repoRoot, resolve(this.root, "../.."), "📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔗️conversions/🔣️.json")}`);
  }
}

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "stdio-document-contract") {
      const { testSemioObjectDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      testSemioObjectDocumentContract();
      const { testSemioKitDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      testSemioKitDocumentContract();
      const { testSemioGeometryContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🧪️tests/🔬️unit/🟦️.ts");
      testSemioGeometryContract();
      const stdioSchemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${stdioSchemaRoot}/✉️base/🧬️schema/🧮️geometry/🟦️.ts`, ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts"].map((file) => `${stdioSchemaRoot}/📦️object/🧬️schema/${file}`), ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts", "🧪️tests/🪪️document/🟦️.ts"].map((file) => `${stdioSchemaRoot}/🧰️kit/🧬️schema/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        for (const filter of ["stdio_document_contract", "subsets::object::", "subsets::kit::"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-stdio-semio", "--lib", filter, "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-semio", { ...{ twins: [{ name: "retained-extrusion", run: retainedExtrusionOracle }, { name: "conversion-contributions", run: runCargoCapabilityContributionChecks }] }, commands: { verify: OwnedVerifyScript, composition: CompositionScript }, snapshotSqliteTestBudgetMs: 120000, snapshotSqliteTests: [
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🎬️video/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📐️cad/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",
] });
