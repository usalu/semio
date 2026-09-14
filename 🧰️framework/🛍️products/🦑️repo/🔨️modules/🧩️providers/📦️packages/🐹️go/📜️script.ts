#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-providers-go` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const packageRoot = import.meta.dir;

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, packageRoot), ...rest], { cwd: packageRoot, env: { ...process.env, GOWORK: "off" } });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
