#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-raster` — the one semio offscreen-rasterization crate: cargo test and clippy gates. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoLint, runCargoTestBudgeted, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-raster"], this.repoRoot, rest);
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-raster --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-raster"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
