#!/usr/bin/env bun
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargoTestBudgeted, runTestBudgeted } from "@semio-tech/repo-lib";

/** 💾️ Runs every resident library assertion through the shared native budget. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Resident native test accepts no arguments");
    await runCargoTestBudgeted(["semio-framework-value-resident"], this.repoRoot, ["--lib"]);
  }
}

/** 🌐️ Checks both toolchain-declared guest targets without parallel Cargo work. */
class CheckWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Resident Wasm check accepts no arguments");
    for (const target of ["wasm32-wasip2", "wasm32-unknown-unknown"]) {
      await runTestBudgeted("cargo", ["check", "-p", "semio-framework-value-resident", "--lib", "--target", target], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
