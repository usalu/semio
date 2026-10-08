#!/usr/bin/env bun
import {completeCargoPreparationObservationV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";
/** 📦️ semio Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { retainedExtrusionOracle } from "../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/📦️extrude-orientation/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { prepareSemioConversionDefinitionV1 } from "../../🧩️composition/🔗️conversions/🟦️.ts";
import { runCargoCapabilityContributionChecks } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🧩️capabilities/🧪️tests/🟦️.ts";
import { runSemioConversionDefinitionChecks } from "../../🧩️composition/🔗️conversions/🧪️tests/🟦️.ts";

class CompositionScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length === 1 && segments[0] === "check") { console.log(`semio conversion definition: checks=${runSemioConversionDefinitionChecks()}`); return; }
    if (segments.length !== 1 || segments[0] !== "prepare") throw new Error("Unknown Semio conversion composition command");
    console.log(`semio conversion composition: targets=${prepareSemioConversionDefinitionV1(this.repoRoot, resolve(this.root, "../.."))}`);
    completeCargoPreparationObservationV1();
  }
}

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "stdio-document-contract") {
      const { testSemioObjectDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
      testSemioObjectDocumentContract();
      const { testSemioKitDocumentContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
      testSemioKitDocumentContract();
      const { testSemioGeometryContract } = await import("../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🧪️tests/🔬️unit/🟦️.ts");
      testSemioGeometryContract();
      const stdioSchemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${stdioSchemaRoot}/✉️base/🧬️schema/🧮️geometry/🟦️.ts`, ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts"].map((file) => `${stdioSchemaRoot}/📦️object/🧬️schema/${file}`), ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${stdioSchemaRoot}/🧰️kit/🧬️schema/${file}`)], { cwd: this.repoRoot });
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
"../../🧪️tests/🪶️sqlite/🛬️native/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔺️geometry/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📃️media/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔗️relationships/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
], snapshotSqliteTestGroups: [
[
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔺️geometry/🟦️.ts",
],
[
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📃️media/🟦️.ts",
],
[
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔗️relationships/🟦️.ts",
],
[
"../../🧪️tests/🪶️sqlite/🛬️native/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
],
[
"../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
],
[
"../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
],
[
"../../🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
],
] });
