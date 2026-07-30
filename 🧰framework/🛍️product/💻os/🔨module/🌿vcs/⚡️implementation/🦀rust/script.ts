#!/usr/bin/env bun
/** 🗄️ `@semio-tech/vcs-rs` router: `bun ./script.ts test`. Pure rlib — no wasm target (the
 * serialization/wasm seam moved to `store/rs`; see `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`). */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["vcs"], this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
