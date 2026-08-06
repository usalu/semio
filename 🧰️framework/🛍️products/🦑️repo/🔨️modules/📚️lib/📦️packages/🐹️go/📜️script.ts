#!/usr/bin/env bun
/** 🧭️ `repo-go-lib` router: `bun ./📜️script.ts test`. */
import { dirname, join } from "node:path";
import { BundleScript, ScriptRouter, goLevelTestArgs, goCoverageArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

const ownerRoot = join(dirname(import.meta.dir), "..");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], { cwd: ownerRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
