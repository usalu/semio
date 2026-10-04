#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🧩️ Runs strict schema, canonical ownership and mandatory JSON source-policy laws. */
class ReferenceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-reference accepts no arguments");
    const test = resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🚮️ Runs the complete reference corpus and the physical specialization absence law. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    const tests = [resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")];
    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", "--timeout", "30000", ...tests], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: {...process.env, SEMIO_TEST_ARTIFACT_DIR: readCargoTestPolicyV1(process.env).artifactDirectory}, throwOnFailure: true});
  }
}

/** 🧾️ Executes every original JSON primitive law and additive explicit-member law. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const {rest} = resolveTestLevel(segments);
    await runCargoTestsV1({manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-pack-json"], cwd: this.root, extraArgs: rest}, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await runScriptMain(router, {defaultCommand: "test-ownership"});
