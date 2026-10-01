#!/usr/bin/env bun
/** 🧭️ `repo-graphql-go` router: `bun ./📜️script.ts test`. */
import { goCoverageArgs, goLevelTestArgs, resolveTestLevel, runTestBudgeted } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    runTestBudgeted("go", ["test", "...", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router);
