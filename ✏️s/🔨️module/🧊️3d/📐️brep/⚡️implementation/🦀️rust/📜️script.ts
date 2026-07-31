#!/usr/bin/env bun
/** 🦀️ `kernel/3d/brep/rs` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["kernel_3d_brepkit"], this.root, rest);
  }
}

/** 📈️ Runs the criterion benchmark suite (`benches/🦀️kernel.rs`). */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "kernel_3d_brepkit"], { cwd: this.root, budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
