
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ draw drawing Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript, runInspectorSelectionActionsScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🎛️ Executes actual bilingual selection projection and original command eligibility regressions. */
class InspectorSelectionActionsScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runInspectorSelectionActionsScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-inspector-selection-actions", InspectorSelectionActionsScript).register("graph-generate", GraphGenerateScript).register("preview-generated", GraphPreviewScript).register("graph-wire-check", OwnerGraphWireCheckScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-draw-drawing", { testPackages: ["semio-s-artifact-draw-drawing", "semio-s-artifact-draw-drawing-editor-commands-tests", "semio-s-artifact-draw-drawing-editor-interaction-tests", "semio-s-artifact-draw-drawing-editor-tests", "semio-s-artifact-draw-drawing-host-tests", "semio-s-artifact-draw-drawing-model-tests", "semio-s-artifact-draw-drawing-scene-tests", "semio-s-artifact-draw-drawing-structure-tests", "semio-s-artifact-draw-drawing-style-tests", "semio-s-artifact-draw-drawing-transform-tests", "semio-s-artifact-draw-drawing-editor-tests-2"], commands: router, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
