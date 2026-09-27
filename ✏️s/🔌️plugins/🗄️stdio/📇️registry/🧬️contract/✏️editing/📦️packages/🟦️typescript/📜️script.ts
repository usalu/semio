#!/usr/bin/env bun
/** ✏️ Stdio snapshot editing TypeScript package router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const root = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing");
    runCmd(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "esnext", "--module", "nodenext", "--moduleResolution", "nodenext", join(root, "🟦️.ts")], { cwd: this.repoRoot });
    runCmd(process.execPath, ["test", join(root, "🧪️tests/🔬️unit/🟦️.test.ts")]);
  }
}
await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), import.meta.url, { defaultCommand: "test" });
