#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧠️ Neural engine native and language-neutral lifecycle validation. */
import { runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

//#region 🧪️Validation
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-os-kernel-neural-engine", ...rest], this.repoRoot);
  }
}
class SourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-source", SourceTestScript);
await runScriptMain(router, { defaultCommand: "test" });
//#endregion 🧪️Validation
