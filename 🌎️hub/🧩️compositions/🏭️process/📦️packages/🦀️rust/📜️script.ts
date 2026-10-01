#!/usr/bin/env bun
/** 🏭️ `@semio-tech/process-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, resolveTestLevel, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-hub-process"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

registerPlaygroundSiteBuildCommands(router);

await runScriptMain(router, { defaultCommand: "test" });
