#!/usr/bin/env bun
/** 🔱️ Trinity TypeScript package. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts");
    const tests = ["🔌️jack", "♻️rewriting"].flatMap((artifact) => {
      const subset = join(artifacts, artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any");
      return [join(subset, "📚️examples/🎬️demo/🧪️tests/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts")];
    });
    runCmd(process.execPath, ["test", ...tests]);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
