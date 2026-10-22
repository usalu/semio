
/** 📊️ Print native and shared chart SQLite verification through the repository execution policy. */
import { runArtifactRustPackageMain } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runNativeGrammarEmissionScript, runLogicalOwnerScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class NativeGrammarEmissionScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runNativeGrammarEmissionScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
class LogicalOwnerScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runLogicalOwnerScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dirname).register("logical-owner", LogicalOwnerScript).register("emit-grammar", NativeGrammarEmissionScript);
await runArtifactRustPackageMain(import.meta.dirname,"semio-framework-print",{testPackages:["semio-framework-print-tests"],commands: router,snapshotSqliteTests:["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
