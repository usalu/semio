#!/usr/bin/env bun
/** 🗄️ `@semio-tech/stdio-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-plugin-stdio"], this.repoRoot, rest);
  }
}

/** 📈️ Runs the owned deterministic-iteration `Brep` kernel benchmark suite (`benches/brep_kernel.rs`) — moved here
 * from `semio-framework-3d` in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
 * ARTIFACTS wave G5, alongside the `Brep` kernel itself. */
class BenchScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["bench", "-p", "semio-s-plugin-stdio"], { cwd: this.repoRoot, budgetMs: buildBudgetMs() });
  }
}

/** 🧩 Builds the root plugin's optimized WASI-P2 component without making stdio a cdylib dependency. */
class BuildWasmReleaseScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["rustc", "-p", "semio-s-plugin-stdio", "--release", "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"], {
      cwd: this.repoRoot,
      budgetMs: buildBudgetMs(),
    });
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-stdio", join(this.root, "..", ".."), true));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript).register("build-wasm-release", BuildWasmReleaseScript).register("describe", DescribeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
