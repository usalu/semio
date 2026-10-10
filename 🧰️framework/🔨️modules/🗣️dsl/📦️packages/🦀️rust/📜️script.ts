#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🧫️ Checks the exact owned portable entrypoints without repository-specific tooling. */
async function checkPortableTypes(root: string, workspace: string, tests: string[]): Promise<void> {
  await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: workspace, budgetMs: 30000, throwOnFailure: true});
}

/** 🗣️ Runs every neutral selection, lexical, diagnostic and literal reference law. */
class ReferenceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-reference accepts no arguments");
    await checkPortableTypes(this.root, this.repoRoot, [join(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(this.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts")]);
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(this.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts")], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🚮️ Runs the full neutral reference corpus and its physically absent product projection. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    await checkPortableTypes(this.root, this.repoRoot, [join(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(this.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")]);
    await runBudgetedTestCommand(process.execPath, ["test", "--timeout", "30000", join(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), join(this.root, "../../📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🗣️ Executes the complete actual neutral lexical, grammar, literal and idiom cohorts. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-dsl"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test-ownership" }) }));
