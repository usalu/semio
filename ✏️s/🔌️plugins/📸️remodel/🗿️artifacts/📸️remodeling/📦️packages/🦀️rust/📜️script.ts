#!/usr/bin/env repo
import { completeCargoPreparationObservationV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
/** 📦️ remodel remodeling Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { prepareCargoCapabilityLinksV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { runOwnedVerifyScript, familyRoot, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🔌️ Prepares source-owned optional Cargo capabilities. */
class CompositionScript extends BundleScript {
  run(args: string[]): void {
    if (args.length !== 1 || args[0] !== "prepare") throw new Error("composition prepare");
    console.log(`remodeling capabilities: targets=${prepareCargoCapabilityLinksV1(this.repoRoot, familyRoot, "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/📦️packages/🦀️rust/Cargo.toml", "🧩️composition/🔗️capabilities/🔣️.json")}`);
    completeCargoPreparationObservationV1();
  }
}

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("composition", CompositionScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-remodel-remodeling", { packages: familyPackages, commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️schema/🟦️.ts"] });


