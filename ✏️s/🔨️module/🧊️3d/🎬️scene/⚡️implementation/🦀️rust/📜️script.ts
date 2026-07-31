#!/usr/bin/env bun
/** 🦀️ `kernel/3d/scene` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["kernel_3d_scene"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
