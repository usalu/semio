#!/usr/bin/env bun
/** 🎬️ Tests scene payload projections against their shared neutral fixtures. */
import { resolve } from "node:path";
import { runCmd } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", [
      "test",
      resolve(import.meta.dir, "../../🧪️tests/🚚️text-editor-lanes/🟦️.test.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️table-lanes/🟦️.test.ts"), resolve(import.meta.dir, "../../🧪️tests/✂️text-splice/🟦️.test.ts"),
      resolve(import.meta.dir, "../../🧪️tests/🚚️world3d-scene-lanes/🟦️.test.ts"),
      ...segments,
    ], { cwd: this.repoRoot });
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
