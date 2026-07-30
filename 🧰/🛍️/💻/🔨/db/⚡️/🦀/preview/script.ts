#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `db_preview` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../🦑/🔨/lib/⚡️/🟦/📦.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["db_preview"], import.meta.dir, rest);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
