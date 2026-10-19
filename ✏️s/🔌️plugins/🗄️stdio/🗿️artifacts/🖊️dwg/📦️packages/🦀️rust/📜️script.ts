
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runControlledMetadataTest, runGrammarShapeTest, familyPackages } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🎛️ Preserves default artifact testing and owns the explicit metadata subcommand. */
class ControlledMetadataTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runControlledMetadataTest({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 🏗️ Validates zero-field record grammar with independent portable and native recognizers. */
class GrammarShapeTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runGrammarShapeTest({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const testRouter = new ScriptRouter(import.meta.dir).register("controlled-metadata", ControlledMetadataTest).register("grammar-shape", GrammarShapeTest);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-dwg", { packages: familyPackages, testCommands: testRouter, snapshotSqliteTests: [
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📦️public/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔢️numbers/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🫳️ownership/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🗄️schema/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📄️document/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔧️header/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🏷️xrecord/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/✏️drawing/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📦️objects/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖌️styles/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📃️layout/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧩️blocks/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📏️constraints/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📇️records/🟦️.ts",
  "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📐️entities/🟦️.ts"
] });
