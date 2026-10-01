#!/usr/bin/env bun
/** 🧭️ Repo SQLite schema package router: `bun ./📜️script.ts test`. */
import { resolveTestLevel, runVitest } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
