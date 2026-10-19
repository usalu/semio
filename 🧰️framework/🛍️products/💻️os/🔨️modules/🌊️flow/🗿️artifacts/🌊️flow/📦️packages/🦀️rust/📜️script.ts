
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🪶️ Runs the authored Flow snapshot ownership laws through the existing task budgets. */
class FlowSqliteTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0];
    if (segments.length > 1 || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Unknown Flow SQLite test mode");
    if (mode !== "source") await runArtifactRustTests("semio-framework-artifact-flow-flow-snapshot-laws", this.repoRoot, ["--lib", "sqlite_snapshot_", "--no-fail-fast"], this.invocation);
    if (mode !== "native") await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 60_000 });
  }
}
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-snapshot-sqlite", FlowSqliteTestScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-framework-artifact-flow-flow", { packages: ["semio-framework-artifact-flow-flow", "semio-framework-artifact-flow-flow-model", "semio-framework-artifact-flow-flow-document", "semio-framework-artifact-flow-flow-copy", "semio-framework-artifact-flow-flow-mutation-laws", "semio-framework-artifact-flow-flow-snapshot-laws"], commands: router });
