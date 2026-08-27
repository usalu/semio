#!/usr/bin/env bun
/** 🖥️ Runs owned plugin-host checks and exact native test filters. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

//#region 🎯️Tasks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
//#endregion 🎯️Tasks
