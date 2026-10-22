#!/usr/bin/env repo
/** 📦️ bmp Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { resolve } from "node:path";
import { runBmpPaintRegionChecks } from "../../🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🧪️tests/🟦️.ts";
import { runBmpSourceHexFixtureChecks } from "../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔤️native-source-hex/🟦️.ts";

/** 🔮️ Runs the independent image-rs BMP decoder laws in its test-only crate. */
class OracleScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("test-oracle accepts no arguments");
    await runOwnedCommand("cargo", ["test", "--locked", "--manifest-path", resolve(this.root, "../../🔮️oracles/📦️packages/🦀️rust/Cargo.toml"), "--features", "oracles", "canonical_byte_authority", "--", "--nocapture"], this.repoRoot, "bmp:image-rs-oracle", 3_600_000, { env: process.env });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-oracle", OracleScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-bmp", { packages: ["semio-s-artifact-stdio-bmp", "semio-s-artifact-stdio-bmp-editor", "semio-s-artifact-stdio-bmp-editor-2", "semio-s-artifact-stdio-bmp-inferences", "semio-s-artifact-stdio-bmp-mutation-regressions", "semio-s-artifact-stdio-bmp-mutations", "semio-s-artifact-stdio-bmp-schema", "semio-s-artifact-stdio-bmp-snapshot"], commands: router, twins: [{ name: "snapshot-source-hex", run: runBmpSourceHexFixtureChecks }, { name: "paint-region", run: runBmpPaintRegionChecks }], snapshotSqliteTests: ["../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
