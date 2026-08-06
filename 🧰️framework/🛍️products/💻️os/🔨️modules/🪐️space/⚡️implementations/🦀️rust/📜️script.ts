#!/usr/bin/env bun
/** 🪐️ `@semio-tech/space-rs` router: `bun ./📜️script.ts test`. Pure rlib — no wasm target (space/
 * collection are headless document technologies consumed by `os-core`/the space app, same shape as
 * `@semio-tech/vcs-rs`). */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["space"], this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
