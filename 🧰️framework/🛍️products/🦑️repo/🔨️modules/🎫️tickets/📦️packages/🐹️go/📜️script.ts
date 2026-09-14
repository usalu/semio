#!/usr/bin/env bun
/** 🧭️ `repo-tickets-go` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    runTestBudgeted("go", ["test", "./...", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
