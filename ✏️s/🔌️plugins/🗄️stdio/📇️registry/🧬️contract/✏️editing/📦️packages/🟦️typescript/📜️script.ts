#!/usr/bin/env bun
/** ✏️ Stdio snapshot editing TypeScript package router. */
import { join } from "node:path";
import { runCmd } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const root = join(this.repoRoot, "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing");
    runCmd(process.execPath, ["x", "--no-install", "tsc", "--noEmit", "--strict", "--skipLibCheck", "--target", "esnext", "--module", "nodenext", "--moduleResolution", "nodenext", join(root, "🟦️.ts"), join(root, "🩹️patch/🟦️.ts")], { cwd: this.repoRoot });
    runCmd(process.execPath, ["test", join(root, "🧪️tests/🔬️unit/🟦️.test.ts"), join(root, "🩹️patch/🧪️tests/🟦️.test.ts")]);
  }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
