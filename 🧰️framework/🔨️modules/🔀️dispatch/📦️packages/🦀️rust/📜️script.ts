#!/usr/bin/env bun
/** ✨️ `@semio-tech/dispatch-macros-rs` router: `bun ./📜️script.ts test`. */
import { resolveTestLevel, runCargoTestBudgeted } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-dispatch-macros"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
