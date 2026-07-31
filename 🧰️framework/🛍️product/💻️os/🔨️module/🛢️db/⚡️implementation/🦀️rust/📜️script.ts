#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `db` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["db"], import.meta.dir, rest);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
