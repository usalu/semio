
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ trinity-jack Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("graph-generate", GraphGenerateScript).register("preview-generated", GraphPreviewScript).register("graph-wire-check", OwnerGraphWireCheckScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-trinity-jack", { packages: ["semio-s-artifact-trinity-jack", "semio-s-artifact-trinity-jack-executor", "semio-s-artifact-trinity-jack-graph-manifest-tests", "semio-s-artifact-trinity-jack-inferences", "semio-s-artifact-trinity-jack-jack", "semio-s-artifact-trinity-jack-language-service", "semio-s-artifact-trinity-jack-model", "semio-s-artifact-trinity-jack-results-window", "semio-s-artifact-trinity-jack-schema", "semio-s-artifact-trinity-jack-transient", "semio-s-artifact-trinity-jack-window-config"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], commands: router });



