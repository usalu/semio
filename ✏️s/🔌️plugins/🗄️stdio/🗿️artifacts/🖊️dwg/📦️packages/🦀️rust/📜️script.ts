#!/usr/bin/env bun
/** 📦️ Runs DWG artifact laws and its owned controlled metadata contract. */
import { resolve } from "node:path";
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🎛️ Preserves default artifact testing and owns the explicit metadata subcommand. */
class ControlledMetadataTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    const mode = args.shift();
    if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test controlled-metadata [source|native]");
    if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🎛️metadata/🟦️.ts")], this.repoRoot, "dwg:controlled-metadata:source", 15_000);
    if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-dwg", this.repoRoot, ["--lib", "dwg_controlled_metadata_"], this.invocation.control);
  }
}
/** 🏗️ Validates zero-field record grammar with independent portable and native recognizers. */
class GrammarShapeTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    const mode = args.shift();
    if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test grammar-shape [source|native]");
    if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🏗️grammar/🟦️.ts")], this.repoRoot, "dwg:grammar-shape:source", 15_000);
    if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-dwg", this.repoRoot, ["--lib", "dwg_grammar_shape_"], this.invocation.control);
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-dwg", { testCommands: { "controlled-metadata": ControlledMetadataTest, "grammar-shape": GrammarShapeTest }, snapshotSqliteTests: [
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
