#!/usr/bin/env bun
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
/** 🖥️ `@semio-tech/framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env,this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

/** 🌱️ Runs the original fold-index lazy entry neutral oracle. */
class IndexEntryTestScript extends BundleScript {
  async run():Promise<void>{await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🌱️entry/🟦️.ts");}
}

/** 🧺️ Runs the original generic set custody neutral oracle. */
class IndexSetTestScript extends BundleScript {
  async run():Promise<void>{await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🧺️set.ts");}
}

/** 🎟️ Exercises independently authored original index insertion references. */
class IndexInsertionTestScript extends BundleScript{async run():Promise<void>{const {testHistoryIndexInsertion}=await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🎟️insertion/🟦️.ts");testHistoryIndexInsertion();}}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-index-entry",IndexEntryTestScript).register("test-index-insertion",IndexInsertionTestScript).register("test-index-set",IndexSetTestScript);

await runScriptMain(router, { defaultCommand: "test" });
