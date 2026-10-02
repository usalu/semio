#!/usr/bin/env bun
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { buildBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs, resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ⏪️ `@semio-tech/framework-time-travel-rs` router: `bun ./📜️script.ts <test|check>` — TS conformance (ajv/xstate/fast-check) plus Rust fixture tests, and native plus `wasm32-wasip2` type checks. */
import { join } from "node:path";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const PACKAGE = "semio-framework-time-travel";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🧪️tests/🧪️conformance/🟦️.ts")], { cwd: this.repoRoot , budgetMs: testLevelBudgetMs()});
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [PACKAGE], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE], this.repoRoot, "tool:owner", buildBudgetMs(), {env: process.env});
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE, "--target", "wasm32-wasip2"], this.repoRoot, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript);

await runScriptMain(router, { defaultCommand: "test" });
