#!/usr/bin/env bun
/** 🦀️ `semio-s-3d` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-3d"], this.repoRoot, rest);
  }
}

/** 📈️ Runs the criterion benchmark suite (`benches/🦀️kernel.rs`). */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-s-3d"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
