#!/usr/bin/env bun
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
/** 🎠️ `@semio-tech/framework-kernel` (TS surface) router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "localized-label") {
      if (segments.length !== 1) throw Error("test localized-label accepts no arguments");
      await runVitestV1(readVitestPolicyV1(process.env, this.root), [], "../../🧪️tests/🏷️localized-label-fixture/🎚️config/🟦️.ts", process.env);
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env, this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
