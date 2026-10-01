#!/usr/bin/env bun
/** note TypeScript package */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath, ["test", join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")]);
  }
}
class SqliteTestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", join(this.repoRoot, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🧪️tests/🟦️.ts")]);
  }
}
class ContractTestScript extends BundleScript {
  async run(): Promise<void> {
    const { testNoteDocumentContractOracle } = await import("../../🧪️tests/🪪️document-contract/🟦️.ts");
    await testNoteDocumentContractOracle();
  }
}
class CheckScript extends BundleScript {
  run(): void {
    const schema = "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
    runCmd(process.execPath, [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(schema, "🟦️.ts"), join(schema, "📸️snapshot/🟦️.ts"), join(schema, "🔺️diff/🟦️.ts"), join(schema, "📸️snapshot/🪶️sqlite/🟦️.ts"), join(schema, "📸️snapshot/🪶️sqlite/🧪️tests/🟦️.ts"), "✏️s/🔌️plugins/🗒️note/🧪️tests/🪪️document-contract/🟦️.ts"], { cwd: this.repoRoot });
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-snapshot-sqlite", SqliteTestScript).register("test-document-contract", ContractTestScript).register("check", CheckScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
