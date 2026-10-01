#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-providers-go` router: `bun ./📜️script.ts test`. */
import { goCoverageArgs, goLevelTestArgs, resolveTestLevel, runTestBudgeted } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const packageRoot = import.meta.dir;

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, packageRoot), ...rest], { cwd: packageRoot, env: { ...process.env, GOWORK: "off" } });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router);
