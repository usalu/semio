#!/usr/bin/env bun
/** 🦀️ `semio-framework-3d` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-3d"], this.repoRoot, rest);
  }
}

/** 📈️ Runs the criterion benchmark suite (`benches/🦀️kernel.rs`). Benchmarks the pre-flip `Brep`
 * duplicate still kept here for `semio-framework-os-kernel`'s escape hatch (ticket 26/08/12/
 * DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave G5) — the stdio-side twin has its
 * own `bun ./📜️script.ts bench` in `semio-s-plugin-stdio`. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-framework-3d"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}


/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-3d --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-3d"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
