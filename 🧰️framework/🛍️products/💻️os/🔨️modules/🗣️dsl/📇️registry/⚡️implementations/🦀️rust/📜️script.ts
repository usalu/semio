#!/usr/bin/env bun
/** 📇️ `@semio-tech/dsl-registry-rs` router: `bun ./📜️script.ts test`. Fan-in `pack_cli::SchemaResolver`
 * — see `📦️lib.rs`'s module doc for the full design and current (W1) fan-in scope. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-os-kernel-dsl-registry"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
