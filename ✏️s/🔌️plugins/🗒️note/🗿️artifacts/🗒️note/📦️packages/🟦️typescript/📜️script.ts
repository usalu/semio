#!/usr/bin/env bun
/** 🗒️ Note artifact TypeScript test infrastructure. */
import { join } from "node:path";
import { runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath, ["test", join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts"), join(subset, "🧬️schema/🧬️mutations/🧪️tests/🧪️drag-blocks/🟦️.ts")]);
  }
}
class SqliteTestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🧪️tests/🟦️.ts")]);
  }
}
class ContractTestScript extends BundleScript {
  async run(): Promise<void> {
    const { testNoteDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
    await testNoteDocumentContractOracle();
  }
}
class CheckScript extends BundleScript {
  run(): void {
    const schema = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
    runCmd(process.execPath, [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(schema, "🟦️.ts"), join(schema, "📸️snapshot/🟦️.ts"), join(schema, "🔺️diff/🟦️.ts"), join(schema, "📸️snapshot/🪶️sqlite/🟦️.ts"), join(schema, "📸️snapshot/🪶️sqlite/🧪️tests/🟦️.ts"), "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts"], { cwd: this.repoRoot });
  }
}
/** 🛂️ Validates artifact document behavior and its authored TypeScript surfaces. */
class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    await new ContractTestScript(this.root).run();
    new CheckScript(this.root).run();
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-snapshot-sqlite", SqliteTestScript).register("test-document-contract", ContractTestScript).register("check", CheckScript).register("canonical-architecture", CanonicalArchitectureScript);
await runScriptMain(router, { defaultCommand: "test" });
