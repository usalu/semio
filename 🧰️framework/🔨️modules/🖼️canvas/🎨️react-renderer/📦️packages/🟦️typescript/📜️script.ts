#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🖼️ `@semio-tech/canvas-react-renderer` router: `bun ./📜️script.ts test`. */
import { readVitestPolicyV1, runVitestV1 } from "../../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env, this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
