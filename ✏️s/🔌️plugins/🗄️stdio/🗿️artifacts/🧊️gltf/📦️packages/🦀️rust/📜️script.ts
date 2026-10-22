#!/usr/bin/env repo
/** 📦️ gltf Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runDefinitionChecks } from "../../🧪️tests/📜️definition/🟦️.ts";
import { runBorrowedCarrierChecks } from "../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🧪️tests/🫳️borrowed-carriers/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runNativeSchemaScript, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧬️ Verifies the actual native codec graph under a caller-owned compiler cache. */
class NativeSchemaScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeSchemaScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("native-schema-check", NativeSchemaScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-gltf", { packages: familyPackages, commands: router, twins: [{ name: "gltf-definition", run: runDefinitionChecks }, { name: "gltf-borrowed-carriers", run: runBorrowedCarrierChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📜️schema/🟦️.ts", "../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
