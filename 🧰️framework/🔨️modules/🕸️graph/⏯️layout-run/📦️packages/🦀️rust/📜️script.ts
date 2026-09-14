#!/usr/bin/env bun
/** ⏯️ `@semio-tech/framework-graph-layout-run-rs` router: `bun ./📜️script.ts <test|check>` — ajv fixture conformance plus Rust fixture, step-law and fdg-sim oracle tests, and native plus `wasm32-wasip2` type checks. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runTestBudgeted } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const PACKAGE = "semio-framework-graph-layout-run";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runTestBudgeted(process.execPath, ["test", join(this.root, "../../🧪️tests/🧩️conformance/🟦️.ts")], { cwd: this.repoRoot });
    await runCargoTestBudgeted([PACKAGE], this.repoRoot, rest);
  }
}

class CheckScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["check", "-p", PACKAGE], { cwd: this.repoRoot });
    runCmd("cargo", ["check", "-p", PACKAGE, "--target", "wasm32-wasip2"], { cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
