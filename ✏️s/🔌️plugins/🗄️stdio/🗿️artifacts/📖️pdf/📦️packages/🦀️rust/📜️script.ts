#!/usr/bin/env bun
/** 📦️ Runs PDF artifact laws and its owned recursive retirement contract. */
import { resolve } from "node:path";
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🎛️ Preserves default artifact testing and owns the explicit retirement subcommand. */
class RecursiveRetirementTest extends BundleScript {
  async run(args: string[]): Promise<void> {
    const mode = args.shift();
    if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test recursive-retirement [source|native]");
    if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/♻️retirement/🟦️.ts")], this.repoRoot, "pdf:recursive-retirement:source", 15_000);
    if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-pdf", this.repoRoot, ["--lib", "pdf_recursive_retirement_"]);
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-pdf", { snapshotSqliteTests: ["../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🌱️value/🧬️octets/🧪️tests/🛫️output/🟦️.ts", "../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🧪️tests/🏭️producer/🟦️.ts"], testCommands: { "recursive-retirement": RecursiveRetirementTest } });
