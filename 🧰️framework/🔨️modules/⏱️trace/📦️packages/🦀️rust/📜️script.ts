#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-trace` task router: `bun ./📜️script.ts <test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-trace"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
