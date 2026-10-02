#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-search-go` router: `bun ./📜️script.ts test`. */
import { dirname, join } from "node:path";
import { goLevelTestArgs, goCoverageArgs, runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const moduleRoot = import.meta.dir;
const ownerRoot = join(dirname(dirname(moduleRoot)));

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand("go", ["test", "...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], { cwd: moduleRoot, env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") } });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router);
