#!/usr/bin/env bun
/** 📜️ `@semio-tech/framework-typeset` — the one semio typesetting crate: cargo test and clippy gates. */
import { runCargoLint, runCargoTestBudgeted, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-typeset"], this.repoRoot, rest);
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-typeset --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["semio-framework-typeset"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runScriptMain(router, { defaultCommand: "test" });
