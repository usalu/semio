#!/usr/bin/env bun
/** 🧭 `repo-go-lib` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, goLevelTestArgs, goCoverageArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
