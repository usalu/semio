#!/usr/bin/env bun
/** 🎬️ Tests scene payload projections against their shared neutral fixtures. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["test", resolve(import.meta.dir, "../../🧪️tests/🚚️text-editor-lanes/🟦️.test.ts"), resolve(import.meta.dir, "../../🧪️tests/🚚️table-lanes/🟦️.test.ts"), ...segments], { cwd: this.repoRoot });
  }
}

await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), import.meta.url, { defaultCommand: "test" });
