#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `db_engine` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["db_engine"], import.meta.dir, rest);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
