#!/usr/bin/env repo
import { ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { completeCargoPreparationObservationV1 } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { retainedExtrusionOracle } from "../../🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/📦️extrude-orientation/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { prepareSemioConversionDefinitionV1 } from "../../🧩️composition/🔗️conversions/🟦️.ts";
import { runCargoCapabilityContributionChecks } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { runSemioConversionDefinitionChecks } from "../../🧩️composition/🔗️conversions/🧪️tests/🟦️.ts";
import { OwnedVerifyScript, TextReceivingSource, TextReceivingNative, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


class CompositionScript extends BundleScript {
run(segments: string[]): void {
if (segments.length === 1 && segments[0] === "check") { console.log(`semio conversion definition: checks=${runSemioConversionDefinitionChecks()}`); return; }
if (segments.length !== 1 || segments[0] !== "prepare") throw new Error("Unknown Semio conversion composition command");
console.log(`semio conversion composition: targets=${prepareSemioConversionDefinitionV1(this.repoRoot, resolve(this.root, "../.."))}`);
completeCargoPreparationObservationV1();
}
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("composition", CompositionScript).register("test-text-receiving-source", TextReceivingSource).register("test-text-receiving-native", TextReceivingNative);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-semio", { packages: familyPackages, ...{ twins: [{ name: "retained-extrusion", run: retainedExtrusionOracle }, { name: "conversion-contributions", run: runCargoCapabilityContributionChecks }] }, commands: router, snapshotSqliteTestBudgetMs: 120000, snapshotSqliteTests: [
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