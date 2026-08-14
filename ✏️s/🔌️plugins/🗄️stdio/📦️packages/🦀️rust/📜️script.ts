#!/usr/bin/env bun
/** 🗄️ `@semio-tech/stdio-plugin` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-plugin-stdio"], this.repoRoot, rest);
  }
}

/** 📈️ Runs the criterion `Brep` kernel benchmark suite (`benches/🦀️brep_kernel.rs`) — moved here
 * from `semio-framework-3d` in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
 * ARTIFACTS wave G5, alongside the `Brep` kernel itself. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-s-plugin-stdio"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
