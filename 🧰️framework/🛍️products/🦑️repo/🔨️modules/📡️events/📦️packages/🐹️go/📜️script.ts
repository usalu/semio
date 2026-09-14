#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-events-go` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, goLevelTestArgs, goCoverageArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const moduleRoot = import.meta.dir;

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, moduleRoot), ...rest], { cwd: moduleRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
