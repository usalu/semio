#!/usr/bin/env bun
/** ⚙️ Validates the canonical OS configuration owner. */
import { runCargo } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), { defaultCommand: "test" });
