#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** 💾️ Runs every resident library assertion through the shared native budget. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Resident native test accepts no arguments");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value-resident"], cwd: this.root, extraArgs: ["--lib"] }, readCargoTestPolicyV1(process.env));
  }
}

/** 🌐️ Checks both toolchain-declared guest targets without parallel Cargo work. */
class CheckWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Resident Wasm check accepts no arguments");
    for (const target of ["wasm32-wasip2", "wasm32-unknown-unknown"]) {
      await runBudgetedTestCommand("cargo", ["check", "--manifest-path", "Cargo.toml", "--lib", "--target", target], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
