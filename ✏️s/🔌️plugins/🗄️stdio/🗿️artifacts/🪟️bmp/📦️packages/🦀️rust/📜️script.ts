#!/usr/bin/env bun
/** 📦️ bmp Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { runBmpPaintRegionChecks } from "../../🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧪️tests/🟦️.ts";
import { runBmpSourceHexFixtureChecks } from "../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔤️native-source-hex/🟦️.ts";

/** 🔮️ Runs the independent image-rs BMP decoder laws in its test-only crate. */
class OracleScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("test-oracle accepts no arguments");
    await runOwnedCommand("cargo", ["test", "--locked", "--manifest-path", resolve(this.root, "../../🔮️oracles/📦️packages/🦀️rust/Cargo.toml"), "--features", "oracles", "canonical_byte_authority", "--", "--nocapture"], this.repoRoot, "bmp:image-rs-oracle", 3_600_000, { env: process.env });
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-bmp", { testFeatures: ["component-app-assembly"], commands: { "test-oracle": OracleScript }, twins: [{ name: "snapshot-source-hex", run: runBmpSourceHexFixtureChecks }, { name: "paint-region", run: runBmpPaintRegionChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
