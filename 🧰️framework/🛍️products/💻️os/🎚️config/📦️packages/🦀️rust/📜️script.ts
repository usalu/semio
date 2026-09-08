#!/usr/bin/env bun
/** ⚙️ Validates the canonical OS configuration owner. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "-p", "semio-framework-os-config", ...segments], this.repoRoot);
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "-p", "semio-framework-os-config", ...segments], this.repoRoot);
  }
}

await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), import.meta.url, { defaultCommand: "test" });
