#!/usr/bin/env repo
/** 📦️ txt Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOriginalTxtEncodingScript, runOriginalTxtReceivingScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🛫️ Runs original TXT output custody laws under the unchanged supplied invocation. */
class OriginalTxtEncodingScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalTxtEncodingScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 🫴️ Runs TXT semantic receiving under the original finite invocation. */
class OriginalTxtReceivingScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalTxtReceivingScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-original-encoding", OriginalTxtEncodingScript).register("test-original-receiving", OriginalTxtReceivingScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-txt", { packages: ["semio-s-artifact-stdio-txt", "semio-s-artifact-stdio-txt-examples", "semio-s-artifact-stdio-txt-inferences", "semio-s-artifact-stdio-txt-mutations", "semio-s-artifact-stdio-txt-schema", "semio-s-artifact-stdio-txt-tests"], commands: router, snapshotSqliteTests: ["../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts", "../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🗂️typed-file/🟦️.ts"] });
