#!/usr/bin/env bun
/** 🔋️ energy TypeScript package and authored-example verification. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const authority = resolve(this.repoRoot, "✏️s/🔌️plugins/🔋️energy/🧪️tests/🛠️toolchain-authority/🟦️.ts");
    if (segments.length) {
      if (segments.length !== 1 || segments[0] !== "toolchain-authority") throw new Error("Expected test toolchain-authority");
      runCmd(process.execPath, ["test", authority]);
      return;
    }
    runCmd(process.execPath, ["test", authority]);
    runCmd(process.execPath, ["test", resolve(this.repoRoot, "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts")]);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
