#!/usr/bin/env bun
/** 🎭️ `@semio-tech/framework-machine` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-machine"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
