#!/usr/bin/env bun
/** 🧠️ Neural engine native and language-neutral lifecycle validation. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

//#region 🧪️Validation
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-os-kernel-neural-engine", ...rest], this.repoRoot);
  }
}
class SourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧵️retirement/🧪️fixtures/📜️script.ts"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-source", SourceTestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
//#endregion 🧪️Validation
