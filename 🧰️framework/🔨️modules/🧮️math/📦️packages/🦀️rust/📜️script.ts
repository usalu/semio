#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-math` — the one semio math crate: cargo test and clippy gates. */
import { runCargoLint, runCargoTestBudgeted, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-math"], this.repoRoot, rest);
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-math --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-math"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runScriptMain(router, { defaultCommand: "test" });
