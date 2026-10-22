#!/usr/bin/env repo
/** 📦️ Block5d owning package, canonical Source consumers and snapshot proof. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-5d", {packages:["semio-s-artifact-block-5d","semio-s-artifact-block-5d-diff","semio-s-artifact-block-5d-editor","semio-s-artifact-block-5d-editor-tests","semio-s-artifact-block-5d-examples","semio-s-artifact-block-5d-inferences","semio-s-artifact-block-5d-io","semio-s-artifact-block-5d-model","semio-s-artifact-block-5d-mut-cameras","semio-s-artifact-block-5d-mut-grip-kinds","semio-s-artifact-block-5d-mut-grips","semio-s-artifact-block-5d-mut-meta","semio-s-artifact-block-5d-mut-part-kind","semio-s-artifact-block-5d-mut-representations","semio-s-artifact-block-5d-mutations","semio-s-artifact-block-5d-schema","semio-s-artifact-block-5d-viewer"],commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts","../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"]});
