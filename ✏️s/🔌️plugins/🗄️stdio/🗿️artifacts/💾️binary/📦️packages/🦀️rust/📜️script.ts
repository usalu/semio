#!/usr/bin/env repo
/** 📦️ binary Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOriginalBinaryReceivingScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🫴️ Checks the original raw/hex field contract and independent schema/octet/authority witnesses. */
class OriginalBinaryReceivingScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOriginalBinaryReceivingScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-original-receiving", OriginalBinaryReceivingScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-binary", { packages: ["semio-s-artifact-stdio-binary", "semio-s-artifact-stdio-binary-io-registry", "semio-s-artifact-stdio-binary-mutations", "semio-s-artifact-stdio-binary-schema", "semio-s-artifact-stdio-binary-snapshot"], commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
