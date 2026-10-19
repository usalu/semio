
import {BundleScript, ScriptRouter} from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runArtifactRustPackageMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runAssemblyChecks } from "../../🧪️tests/📦️assembly/🟦️.ts";
import { runDeletionProofScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🗑️ Compiles the general assembly in a regular-file workspace with the concrete fleet absent. */
class DeletionProofScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDeletionProofScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("deletion-proof", DeletionProofScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-plugin-stdio", { twins: [{ name: "authored-assembly", run: runAssemblyChecks }], commands: router });
