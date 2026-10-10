#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🧬️ Checks canonical typed Pack error ownership without product infrastructure. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    const test = resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🪆️ Runs every canonical typed Pack refusal and error transport law. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const {rest} = resolveTestLevel(segments);
    await runCargoTestsV1({manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-pack-error"], cwd: this.root, extraArgs: rest}, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand: "test-ownership"}) }));
